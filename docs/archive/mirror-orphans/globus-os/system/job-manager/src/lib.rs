//! Deterministic lifecycle manager for finite user-space jobs.

use globus_process::ProcessId;
use globus_process_manager::{ProcessAction, ProcessManager, ProcessManagerError};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct JobId(pub u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobState {
    Queued,
    Running,
    Completed,
    Failed,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobError {
    Duplicate,
    Missing,
    InvalidTransition,
    Process(ProcessManagerError),
}
#[derive(Debug)]
pub struct Job {
    pub id: JobId,
    pub process: ProcessId,
    pub state: JobState,
}
#[derive(Debug, Default)]
pub struct JobManager {
    next_id: u64,
    jobs: BTreeMap<JobId, Job>,
}

impl JobManager {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            ..Self::default()
        }
    }

    pub fn submit(
        &mut self,
        pm: &mut ProcessManager,
        parent: Option<ProcessId>,
        _priority: u8,
    ) -> Result<JobId, JobError> {
        let process = pm.spawn(parent).map_err(JobError::Process)?;
        let id = JobId(self.next_id);
        self.next_id = self.next_id.saturating_add(1);
        self.jobs.insert(
            id,
            Job {
                id,
                process,
                state: JobState::Queued,
            },
        );
        Ok(id)
    }

    pub fn start(&mut self, pm: &mut ProcessManager, id: JobId) -> Result<(), JobError> {
        let j = self.jobs.get_mut(&id).ok_or(JobError::Missing)?;
        if j.state != JobState::Queued {
            return Err(JobError::InvalidTransition);
        }
        pm.apply(j.process, ProcessAction::Start)
            .map_err(JobError::Process)?;
        j.state = JobState::Running;
        Ok(())
    }

    pub fn finish(&mut self, id: JobId, success: bool) -> Result<(), JobError> {
        let j = self.jobs.get_mut(&id).ok_or(JobError::Missing)?;
        if j.state != JobState::Running {
            return Err(JobError::InvalidTransition);
        }
        j.state = if success {
            JobState::Completed
        } else {
            JobState::Failed
        };
        Ok(())
    }

    pub fn get(&self, id: JobId) -> Option<&Job> {
        self.jobs.get(&id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn job_lifecycle() {
        let mut pm = ProcessManager::new();
        let mut jm = JobManager::new();
        let id = jm.submit(&mut pm, None, 1).unwrap();
        jm.start(&mut pm, id).unwrap();
        jm.finish(id, true).unwrap();
        assert_eq!(jm.get(id).unwrap().state, JobState::Completed);
    }
}
