//! GlobusOS system control plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemState {
    Booting,
    Starting,
    Running,
    Degraded,
    Recovery,
    ShuttingDown,
    Stopped,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemAction {
    Start,
    EnterDegraded,
    EnterRecovery,
    Shutdown,
    Reboot,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemError {
    InvalidTransition,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemManager {
    state: SystemState,
}
impl Default for SystemManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemManager {
    pub const fn new() -> Self {
        Self {
            state: SystemState::Booting,
        }
    }
    pub const fn state(&self) -> SystemState {
        self.state
    }
    pub fn apply(&mut self, a: SystemAction) -> Result<SystemState, SystemError> {
        let n = match (a, self.state) {
            (SystemAction::Start, SystemState::Booting | SystemState::Starting) => {
                SystemState::Running
            }
            (SystemAction::Start, SystemState::Recovery) => SystemState::Starting,
            (SystemAction::EnterDegraded, SystemState::Running) => SystemState::Degraded,
            (
                SystemAction::EnterRecovery,
                SystemState::Booting
                | SystemState::Starting
                | SystemState::Running
                | SystemState::Degraded,
            ) => SystemState::Recovery,
            (
                SystemAction::Shutdown,
                SystemState::Running | SystemState::Degraded | SystemState::Recovery,
            ) => SystemState::ShuttingDown,
            (
                SystemAction::Reboot,
                SystemState::Running | SystemState::Degraded | SystemState::Recovery,
            ) => SystemState::Booting,
            _ => return Err(SystemError::InvalidTransition),
        };
        self.state = n;
        Ok(n)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lifecycle() {
        let mut m = SystemManager::new();
        assert_eq!(m.apply(SystemAction::Start), Ok(SystemState::Running));
        assert_eq!(
            m.apply(SystemAction::EnterRecovery),
            Ok(SystemState::Recovery)
        );
    }
}
