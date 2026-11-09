//! Deterministic recovery policy. Hardware reset/reboot remains below userspace.

use globus_system_manager::{SystemAction, SystemManager, SystemState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryReason {
    Watchdog,
    FailedUpdate,
    BootFailure,
    Manual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryError {
    InvalidState,
    Transition,
}

#[derive(Debug, Default)]
pub struct RecoveryManager;

impl RecoveryManager {
    pub fn enter(system: &mut SystemManager, _reason: RecoveryReason) -> Result<(), RecoveryError> {
        if !matches!(system.state(), SystemState::Running | SystemState::Degraded) {
            return Err(RecoveryError::InvalidState);
        }
        system
            .apply(SystemAction::EnterRecovery)
            .map(|_| ())
            .map_err(|_| RecoveryError::Transition)
    }
    pub fn resume(system: &mut SystemManager) -> Result<(), RecoveryError> {
        if system.state() != SystemState::Recovery {
            return Err(RecoveryError::InvalidState);
        }
        system
            .apply(SystemAction::Start)
            .map(|_| ())
            .map_err(|_| RecoveryError::Transition)
    }
}
