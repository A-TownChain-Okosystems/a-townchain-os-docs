//! Deny-by-default application lifecycle and capability metadata.
//!
//! The application manager owns userspace application registration and lifecycle
//! policy. ShivaCore remains responsible for process execution, address spaces,
//! capabilities, isolation, and CPU scheduling.

use std::collections::BTreeMap;

use globus_process::ProcessId;
use globus_process_manager::{ProcessAction, ProcessManager, ProcessManagerError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ApplicationId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationSpec {
    pub id: ApplicationId,
    pub name: String,
    pub executable: String,
    pub capabilities: Vec<String>,
    pub auto_restart: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationState {
    Installed,
    Ready,
    Running,
    Suspended,
    Stopped,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationAction {
    Prepare,
    Start,
    Suspend,
    Resume,
    Stop,
    Fail,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplicationError {
    DuplicateId,
    EmptyName,
    EmptyExecutable,
    UnknownApplication,
    InvalidTransition,
    Process(ProcessManagerError),
}

impl From<ProcessManagerError> for ApplicationError {
    fn from(value: ProcessManagerError) -> Self {
        Self::Process(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationRecord {
    pub spec: ApplicationSpec,
    pub state: ApplicationState,
    pub process: Option<ProcessId>,
}

#[derive(Debug, Default)]
pub struct ApplicationManager {
    applications: BTreeMap<ApplicationId, ApplicationRecord>,
}

impl ApplicationManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, spec: ApplicationSpec) -> Result<(), ApplicationError> {
        if spec.name.is_empty() {
            return Err(ApplicationError::EmptyName);
        }
        if spec.executable.is_empty() {
            return Err(ApplicationError::EmptyExecutable);
        }
        if self.applications.contains_key(&spec.id) {
            return Err(ApplicationError::DuplicateId);
        }
        self.applications.insert(
            spec.id,
            ApplicationRecord {
                spec,
                state: ApplicationState::Installed,
                process: None,
            },
        );
        Ok(())
    }

    pub fn apply(
        &mut self,
        id: ApplicationId,
        action: ApplicationAction,
        processes: &mut ProcessManager,
    ) -> Result<(), ApplicationError> {
        let record = self
            .applications
            .get_mut(&id)
            .ok_or(ApplicationError::UnknownApplication)?;

        match (record.state, action) {
            (ApplicationState::Installed, ApplicationAction::Prepare) => {
                record.state = ApplicationState::Ready;
            }
            (ApplicationState::Ready | ApplicationState::Stopped, ApplicationAction::Start) => {
                let pid = processes.spawn(None)?;
                processes.apply(pid, ProcessAction::Start)?;
                record.process = Some(pid);
                record.state = ApplicationState::Running;
            }
            (ApplicationState::Running, ApplicationAction::Suspend) => {
                let pid = record.process.ok_or(ApplicationError::InvalidTransition)?;
                processes.apply(pid, ProcessAction::Block)?;
                record.state = ApplicationState::Suspended;
            }
            (ApplicationState::Suspended, ApplicationAction::Resume) => {
                let pid = record.process.ok_or(ApplicationError::InvalidTransition)?;
                processes.apply(pid, ProcessAction::Resume)?;
                record.state = ApplicationState::Running;
            }
            (ApplicationState::Running | ApplicationState::Suspended, ApplicationAction::Stop) => {
                if let Some(pid) = record.process.take() {
                    processes.apply(pid, ProcessAction::Terminate(0))?;
                    let _ = processes.reap(pid)?;
                }
                record.state = ApplicationState::Stopped;
            }
            (
                ApplicationState::Installed
                | ApplicationState::Ready
                | ApplicationState::Running
                | ApplicationState::Suspended
                | ApplicationState::Stopped,
                ApplicationAction::Fail,
            ) => {
                record.state = ApplicationState::Failed;
            }
            _ => return Err(ApplicationError::InvalidTransition),
        }
        Ok(())
    }

    pub fn get(&self, id: ApplicationId) -> Option<&ApplicationRecord> {
        self.applications.get(&id)
    }

    pub fn applications(&self) -> impl Iterator<Item = &ApplicationRecord> {
        self.applications.values()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use globus_process::ProcessState;

    fn spec(id: u64) -> ApplicationSpec {
        ApplicationSpec {
            id: ApplicationId(id),
            name: format!("app-{id}"),
            executable: format!("pkg://app-{id}"),
            capabilities: Vec::new(),
            auto_restart: false,
        }
    }

    #[test]
    fn lifecycle_is_explicit_and_process_backed() {
        let mut apps = ApplicationManager::new();
        let mut processes = ProcessManager::new();
        apps.register(spec(1)).unwrap();
        apps.apply(ApplicationId(1), ApplicationAction::Prepare, &mut processes)
            .unwrap();
        apps.apply(ApplicationId(1), ApplicationAction::Start, &mut processes)
            .unwrap();
        assert_eq!(
            apps.get(ApplicationId(1)).unwrap().state,
            ApplicationState::Running
        );
        let pid = apps.get(ApplicationId(1)).unwrap().process.unwrap();
        assert_eq!(processes.get(pid).unwrap().state, ProcessState::Ready);
        apps.apply(ApplicationId(1), ApplicationAction::Suspend, &mut processes)
            .unwrap();
        assert_eq!(processes.get(pid).unwrap().state, ProcessState::Blocked);
        apps.apply(ApplicationId(1), ApplicationAction::Resume, &mut processes)
            .unwrap();
        apps.apply(ApplicationId(1), ApplicationAction::Stop, &mut processes)
            .unwrap();
        assert_eq!(
            apps.get(ApplicationId(1)).unwrap().state,
            ApplicationState::Stopped
        );
        assert!(processes.get(pid).is_none());
    }

    #[test]
    fn duplicate_and_empty_metadata_are_rejected() {
        let mut apps = ApplicationManager::new();
        assert_eq!(
            apps.register(ApplicationSpec {
                id: ApplicationId(1),
                name: String::new(),
                executable: "x".into(),
                capabilities: vec![],
                auto_restart: false
            }),
            Err(ApplicationError::EmptyName)
        );
        apps.register(spec(1)).unwrap();
        assert_eq!(apps.register(spec(1)), Err(ApplicationError::DuplicateId));
    }

    #[test]
    fn applications_are_deterministically_ordered() {
        let mut apps = ApplicationManager::new();
        apps.register(spec(20)).unwrap();
        apps.register(spec(3)).unwrap();
        let ids: Vec<_> = apps.applications().map(|a| a.spec.id).collect();
        assert_eq!(ids, vec![ApplicationId(3), ApplicationId(20)]);
    }
}
