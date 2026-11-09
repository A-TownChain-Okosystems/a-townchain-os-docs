//! Userspace process lifecycle manager for GlobusOS.
//! Kernel execution, address spaces, capabilities and CPU scheduling remain
//! owned by ShivaCore; this crate manages process identity and lifecycle state.

use globus_process::{ProcessId, ProcessState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessAction {
    Start,
    Block,
    Resume,
    Terminate(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProcessRecord {
    pub id: ProcessId,
    pub parent: Option<ProcessId>,
    pub state: ProcessState,
    pub exit_code: Option<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessManagerError {
    DuplicateProcess,
    UnknownProcess,
    InvalidTransition,
    InvalidParent,
}

#[derive(Debug, Default)]
pub struct ProcessManager {
    next_pid: u64,
    processes: Vec<ProcessRecord>,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            next_pid: 1,
            processes: Vec::new(),
        }
    }

    pub fn spawn(&mut self, parent: Option<ProcessId>) -> Result<ProcessId, ProcessManagerError> {
        if let Some(pid) = parent
            && !self
                .processes
                .iter()
                .any(|p| p.id == pid && p.state != ProcessState::Exited)
        {
            return Err(ProcessManagerError::InvalidParent);
        }
        let id = self.allocate_pid();
        self.processes.push(ProcessRecord {
            id,
            parent,
            state: ProcessState::Created,
            exit_code: None,
        });
        Ok(id)
    }

    pub fn apply(
        &mut self,
        id: ProcessId,
        action: ProcessAction,
    ) -> Result<(), ProcessManagerError> {
        let p = self
            .processes
            .iter_mut()
            .find(|p| p.id == id)
            .ok_or(ProcessManagerError::UnknownProcess)?;
        match (p.state, action) {
            (ProcessState::Created, ProcessAction::Start) => p.state = ProcessState::Ready,
            (ProcessState::Ready | ProcessState::Running, ProcessAction::Block) => {
                p.state = ProcessState::Blocked
            }
            (ProcessState::Blocked, ProcessAction::Resume) => p.state = ProcessState::Ready,
            (
                ProcessState::Created
                | ProcessState::Ready
                | ProcessState::Running
                | ProcessState::Blocked,
                ProcessAction::Terminate(code),
            ) => {
                p.state = ProcessState::Exited;
                p.exit_code = Some(code);
            }
            _ => return Err(ProcessManagerError::InvalidTransition),
        }
        Ok(())
    }

    pub fn reap(&mut self, id: ProcessId) -> Result<ProcessRecord, ProcessManagerError> {
        let index = self
            .processes
            .iter()
            .position(|p| p.id == id)
            .ok_or(ProcessManagerError::UnknownProcess)?;
        if self.processes[index].state != ProcessState::Exited {
            return Err(ProcessManagerError::InvalidTransition);
        }
        Ok(self.processes.remove(index))
    }

    pub fn get(&self, id: ProcessId) -> Option<ProcessRecord> {
        self.processes.iter().copied().find(|p| p.id == id)
    }
    pub fn processes(&self) -> &[ProcessRecord] {
        &self.processes
    }

    fn allocate_pid(&mut self) -> ProcessId {
        loop {
            let id = ProcessId(self.next_pid);
            self.next_pid = self.next_pid.saturating_add(1).max(1);
            if !self.processes.iter().any(|p| p.id == id) {
                return id;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parent_and_reaping_are_enforced() {
        let mut m = ProcessManager::new();
        let root = m.spawn(None).unwrap();
        assert_eq!(
            m.spawn(Some(ProcessId(99))),
            Err(ProcessManagerError::InvalidParent)
        );
        m.apply(root, ProcessAction::Start).unwrap();
        let child = m.spawn(Some(root)).unwrap();
        m.apply(child, ProcessAction::Terminate(7)).unwrap();
        assert_eq!(m.reap(child).unwrap().exit_code, Some(7));
        assert!(m.get(child).is_none());
    }
}
