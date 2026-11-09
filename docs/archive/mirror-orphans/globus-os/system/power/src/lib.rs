//! Userspace power-management policy.
//!
//! Hardware operations remain outside this crate. This service validates and
//! authorizes power intents before a privileged platform adapter executes them.

/// Requested system power transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerAction {
    Suspend,
    Hibernate,
    Reboot,
    Shutdown,
}

/// Current userspace power state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerState {
    Running,
    Suspending,
    Hibernating,
    Rebooting,
    ShuttingDown,
}

impl PowerState {
    /// Returns whether the state is terminal for the current boot session.
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Rebooting | Self::ShuttingDown)
    }
}

/// Capability required by a caller to request a power transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerCapability {
    ManagePower,
}

/// Power-management error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerError {
    Unauthorized,
    InvalidTransition,
    InvalidBatteryTelemetry,
}

/// Battery telemetry supplied by a trusted hardware adapter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BatteryStatus {
    pub present: bool,
    pub charging: bool,
    pub percentage: u8,
    pub temperature_celsius: i16,
}

impl BatteryStatus {
    /// Validates hardware-reported battery telemetry.
    pub const fn validate(self) -> Result<Self, PowerError> {
        if self.percentage > 100 || self.temperature_celsius < -80 || self.temperature_celsius > 150
        {
            return Err(PowerError::InvalidBatteryTelemetry);
        }
        Ok(self)
    }
}

/// Userspace power policy state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PowerManager {
    state: PowerState,
    thermal_limit_celsius: i16,
}

impl Default for PowerManager {
    fn default() -> Self {
        Self {
            state: PowerState::Running,
            thermal_limit_celsius: 95,
        }
    }
}

impl PowerManager {
    /// Creates the default userspace power manager.
    pub const fn new() -> Self {
        Self {
            state: PowerState::Running,
            thermal_limit_celsius: 95,
        }
    }

    /// Returns the current state.
    pub const fn state(self) -> PowerState {
        self.state
    }

    /// Applies a requested action after capability and state validation.
    pub fn request(
        &mut self,
        capability: Option<PowerCapability>,
        action: PowerAction,
    ) -> Result<PowerState, PowerError> {
        if capability != Some(PowerCapability::ManagePower) {
            return Err(PowerError::Unauthorized);
        }
        if self.state.is_terminal() {
            return Err(PowerError::InvalidTransition);
        }
        self.state = match action {
            PowerAction::Suspend => PowerState::Suspending,
            PowerAction::Hibernate => PowerState::Hibernating,
            PowerAction::Reboot => PowerState::Rebooting,
            PowerAction::Shutdown => PowerState::ShuttingDown,
        };
        Ok(self.state)
    }

    /// Rejects telemetry above the configured thermal shutdown threshold.
    pub fn thermal_shutdown_required(&self, battery: BatteryStatus) -> Result<bool, PowerError> {
        let battery = battery.validate()?;
        Ok(battery.temperature_celsius >= self.thermal_limit_celsius)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power_actions_require_explicit_capability() {
        let mut manager = PowerManager::new();
        assert_eq!(
            manager.request(None, PowerAction::Shutdown),
            Err(PowerError::Unauthorized)
        );
        assert_eq!(manager.state(), PowerState::Running);
    }

    #[test]
    fn terminal_state_cannot_be_reentered() {
        let mut manager = PowerManager::new();
        assert_eq!(
            manager.request(Some(PowerCapability::ManagePower), PowerAction::Reboot),
            Ok(PowerState::Rebooting)
        );
        assert_eq!(
            manager.request(Some(PowerCapability::ManagePower), PowerAction::Shutdown),
            Err(PowerError::InvalidTransition)
        );
    }

    #[test]
    fn invalid_battery_telemetry_fails_closed() {
        let manager = PowerManager::new();
        let telemetry = BatteryStatus {
            present: true,
            charging: false,
            percentage: 101,
            temperature_celsius: 40,
        };
        assert_eq!(
            manager.thermal_shutdown_required(telemetry),
            Err(PowerError::InvalidBatteryTelemetry)
        );
    }

    #[test]
    fn thermal_limit_is_enforced() {
        let manager = PowerManager::new();
        let telemetry = BatteryStatus {
            present: true,
            charging: false,
            percentage: 50,
            temperature_celsius: 95,
        };
        assert_eq!(manager.thermal_shutdown_required(telemetry), Ok(true));
    }
}
