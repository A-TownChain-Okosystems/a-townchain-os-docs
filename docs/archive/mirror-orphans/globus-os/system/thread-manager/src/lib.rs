//! Userspace thread lifecycle metadata for GlobusOS.
//! Actual CPU context switching and affinity enforcement belong to ShivaCore.

use globus_process::{ProcessId, ThreadId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadState {
    Created,
    Ready,
    Running,
    Blocked,
    Exited,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThreadRecord {
    pub id: ThreadId,
    pub process: ProcessId,
    pub state: ThreadState,
    pub priority: u8,
    pub cpu_affinity: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadError {
    DuplicateThread,
    UnknownThread,
    InvalidTransition,
}

#[derive(Debug, Default)]
pub struct ThreadManager {
    next_tid: u64,
    threads: Vec<ThreadRecord>,
}

impl ThreadManager {
    pub fn new() -> Self {
        Self {
            next_tid: 1,
            threads: Vec::new(),
        }
    }
    pub fn create(
        &mut self,
        process: ProcessId,
        priority: u8,
        cpu_affinity: Option<u32>,
    ) -> Result<ThreadId, ThreadError> {
        let id = self.allocate_tid();
        self.threads.push(ThreadRecord {
            id,
            process,
            state: ThreadState::Created,
            priority,
            cpu_affinity,
        });
        Ok(id)
    }
    pub fn set_state(&mut self, id: ThreadId, state: ThreadState) -> Result<(), ThreadError> {
        let t = self
            .threads
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or(ThreadError::UnknownThread)?;
        let valid = matches!(
            (t.state, state),
            (ThreadState::Created, ThreadState::Ready)
                | (
                    ThreadState::Ready,
                    ThreadState::Running | ThreadState::Blocked | ThreadState::Exited
                )
                | (
                    ThreadState::Running,
                    ThreadState::Ready | ThreadState::Blocked | ThreadState::Exited
                )
                | (
                    ThreadState::Blocked,
                    ThreadState::Ready | ThreadState::Exited
                )
                | (ThreadState::Exited, ThreadState::Exited)
        );
        if !valid {
            return Err(ThreadError::InvalidTransition);
        }
        t.state = state;
        Ok(())
    }
    pub fn get(&self, id: ThreadId) -> Option<ThreadRecord> {
        self.threads.iter().copied().find(|t| t.id == id)
    }
    pub fn threads(&self) -> &[ThreadRecord] {
        &self.threads
    }
    fn allocate_tid(&mut self) -> ThreadId {
        loop {
            let id = ThreadId(self.next_tid);
            self.next_tid = self.next_tid.saturating_add(1).max(1);
            if !self.threads.iter().any(|t| t.id == id) {
                return id;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn thread_lifecycle_and_affinity() {
        let mut m = ThreadManager::new();
        let t = m.create(ProcessId(1), 10, Some(2)).unwrap();
        assert_eq!(
            m.set_state(t, ThreadState::Running),
            Err(ThreadError::InvalidTransition)
        );
        m.set_state(t, ThreadState::Ready).unwrap();
        m.set_state(t, ThreadState::Running).unwrap();
        assert_eq!(m.get(t).unwrap().cpu_affinity, Some(2));
    }
}
