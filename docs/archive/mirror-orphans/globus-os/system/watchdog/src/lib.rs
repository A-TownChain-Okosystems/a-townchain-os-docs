//! Bounded health supervision for GlobusOS control-plane services.
//!
//! The watchdog records heartbeats and emits deterministic recovery actions.
//! It does not restart kernel tasks or bypass capability boundaries.

use globus_event_bus::{EventBus, EventError};
use globus_service_manager::{HealthReport, ServiceError, ServiceManager};
use std::collections::BTreeMap;

/// Event kind emitted when a recovery action is required.
pub const WATCHDOG_EVENT_KIND: u16 = 0x7001;

/// Recovery action selected by the watchdog.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryAction {
    /// Ask the service supervisor to restart a failed service.
    RestartService,
    /// Escalate a critical service failure to system recovery.
    EnterRecovery,
}

/// Watchdog policy for one monitored service.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WatchTarget {
    pub service: String,
    pub timeout_ticks: u64,
    pub max_missed_heartbeats: u32,
    pub critical: bool,
}

/// Watchdog error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WatchdogError {
    DuplicateTarget(String),
    InvalidTarget(String),
    UnknownTarget(String),
    Service(ServiceError),
    Event(EventError),
}

impl From<ServiceError> for WatchdogError {
    fn from(value: ServiceError) -> Self {
        Self::Service(value)
    }
}

impl From<EventError> for WatchdogError {
    fn from(value: EventError) -> Self {
        Self::Event(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TargetState {
    last_heartbeat: u64,
    missed: u32,
    unhealthy: bool,
}

/// Deterministic, bounded watchdog.
#[derive(Debug, Default)]
pub struct Watchdog {
    targets: BTreeMap<String, (WatchTarget, TargetState)>,
}

impl Watchdog {
    /// Creates an empty watchdog.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a monitored service.
    pub fn register(&mut self, target: WatchTarget) -> Result<(), WatchdogError> {
        if target.service.is_empty()
            || target.timeout_ticks == 0
            || target.max_missed_heartbeats == 0
        {
            return Err(WatchdogError::InvalidTarget(target.service));
        }
        if self.targets.contains_key(&target.service) {
            return Err(WatchdogError::DuplicateTarget(target.service));
        }
        self.targets.insert(
            target.service.clone(),
            (
                target,
                TargetState {
                    last_heartbeat: 0,
                    missed: 0,
                    unhealthy: false,
                },
            ),
        );
        Ok(())
    }

    /// Records a heartbeat at a monotonically increasing logical tick.
    pub fn heartbeat(&mut self, service: &str, tick: u64) -> Result<(), WatchdogError> {
        let (_, state) = self
            .targets
            .get_mut(service)
            .ok_or_else(|| WatchdogError::UnknownTarget(service.into()))?;
        if tick < state.last_heartbeat {
            return Err(WatchdogError::InvalidTarget(service.into()));
        }
        state.last_heartbeat = tick;
        state.missed = 0;
        state.unhealthy = false;
        Ok(())
    }

    /// Evaluates all targets and returns one action per newly unhealthy service.
    ///
    /// The service manager remains authoritative for restart budgets.
    pub fn poll(
        &mut self,
        tick: u64,
        services: &mut ServiceManager,
        events: &mut EventBus,
    ) -> Result<Vec<(String, RecoveryAction)>, WatchdogError> {
        let mut actions = Vec::new();
        let names: Vec<String> = self.targets.keys().cloned().collect();
        for name in names {
            let (target, state) = self.targets.get_mut(&name).expect("target exists");
            let elapsed = tick.saturating_sub(state.last_heartbeat);
            let missed = elapsed / target.timeout_ticks;
            state.missed = missed.min(u32::MAX as u64) as u32;
            if state.missed < target.max_missed_heartbeats || state.unhealthy {
                continue;
            }
            state.unhealthy = true;

            let action = if target.critical {
                RecoveryAction::EnterRecovery
            } else {
                RecoveryAction::RestartService
            };
            let report = HealthReport {
                healthy: false,
                consecutive_failures: state.missed,
            };
            let restart_allowed = services.record_health(&name, report)?;
            if matches!(action, RecoveryAction::RestartService) && !restart_allowed {
                continue;
            }
            let value = match action {
                RecoveryAction::RestartService => 1,
                RecoveryAction::EnterRecovery => 2,
            };
            events.publish(WATCHDOG_EVENT_KIND, value)?;
            actions.push((name, action));
        }
        Ok(actions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use globus_service_manager::ServiceSpec;

    fn service(name: &str, critical: bool) -> ServiceSpec {
        ServiceSpec {
            name: name.into(),
            dependencies: Vec::new(),
            critical,
            restart_limit: 2,
        }
    }

    #[test]
    fn emits_bounded_restart_action() {
        let mut services = ServiceManager::new();
        services.register(service("network", false)).unwrap();
        let mut watchdog = Watchdog::new();
        watchdog
            .register(WatchTarget {
                service: "network".into(),
                timeout_ticks: 10,
                max_missed_heartbeats: 2,
                critical: false,
            })
            .unwrap();
        let mut events = EventBus::new(4).unwrap();
        assert!(
            watchdog
                .poll(19, &mut services, &mut events)
                .unwrap()
                .is_empty()
        );
        let actions = watchdog.poll(20, &mut services, &mut events).unwrap();
        assert_eq!(
            actions,
            vec![(String::from("network"), RecoveryAction::RestartService)]
        );
        assert!(
            watchdog
                .poll(40, &mut services, &mut events)
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn critical_service_escalates_to_recovery() {
        let mut services = ServiceManager::new();
        services.register(service("security", true)).unwrap();
        let mut watchdog = Watchdog::new();
        watchdog
            .register(WatchTarget {
                service: "security".into(),
                timeout_ticks: 5,
                max_missed_heartbeats: 1,
                critical: true,
            })
            .unwrap();
        let mut events = EventBus::new(2).unwrap();
        let actions = watchdog.poll(5, &mut services, &mut events).unwrap();
        assert_eq!(
            actions,
            vec![(String::from("security"), RecoveryAction::EnterRecovery)]
        );
    }

    #[test]
    fn heartbeat_clears_failure_latch() {
        let mut services = ServiceManager::new();
        services.register(service("network", false)).unwrap();
        let mut watchdog = Watchdog::new();
        watchdog
            .register(WatchTarget {
                service: "network".into(),
                timeout_ticks: 10,
                max_missed_heartbeats: 1,
                critical: false,
            })
            .unwrap();
        let mut events = EventBus::new(4).unwrap();
        assert_eq!(
            watchdog.poll(10, &mut services, &mut events).unwrap().len(),
            1
        );
        watchdog.heartbeat("network", 11).unwrap();
        assert!(
            watchdog
                .poll(20, &mut services, &mut events)
                .unwrap()
                .is_empty()
        );
    }
}
