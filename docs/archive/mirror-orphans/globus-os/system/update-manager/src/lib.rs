//! Fail-closed update transaction state machine.
//! Payload verification/execution is delegated to the update subsystem.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Update {
    pub id: String,
    pub target_version: String,
    pub digest: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateState {
    Proposed,
    Verified,
    Staged,
    Applied,
    RolledBack,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateAction {
    Verify,
    Stage,
    Apply,
    Rollback,
    Fail,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateError {
    EmptyId,
    EmptyVersion,
    EmptyDigest,
    InvalidTransition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateRecord {
    pub update: Update,
    pub state: UpdateState,
}

#[derive(Debug, Default)]
pub struct UpdateManager {
    records: Vec<UpdateRecord>,
}

impl UpdateManager {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn propose(&mut self, update: Update) -> Result<usize, UpdateError> {
        if update.id.is_empty() {
            return Err(UpdateError::EmptyId);
        }
        if update.target_version.is_empty() {
            return Err(UpdateError::EmptyVersion);
        }
        if update.digest.is_empty() {
            return Err(UpdateError::EmptyDigest);
        }
        let i = self.records.len();
        self.records.push(UpdateRecord {
            update,
            state: UpdateState::Proposed,
        });
        Ok(i)
    }
    pub fn apply(&mut self, index: usize, action: UpdateAction) -> Result<(), UpdateError> {
        let r = self
            .records
            .get_mut(index)
            .ok_or(UpdateError::InvalidTransition)?;
        match (r.state, action) {
            (UpdateState::Proposed, UpdateAction::Verify) => r.state = UpdateState::Verified,
            (UpdateState::Verified, UpdateAction::Stage) => r.state = UpdateState::Staged,
            (UpdateState::Staged, UpdateAction::Apply) => r.state = UpdateState::Applied,
            (UpdateState::Applied, UpdateAction::Rollback) => r.state = UpdateState::RolledBack,
            (
                UpdateState::Proposed | UpdateState::Verified | UpdateState::Staged,
                UpdateAction::Fail,
            ) => r.state = UpdateState::Failed,
            _ => return Err(UpdateError::InvalidTransition),
        }
        Ok(())
    }
    pub fn get(&self, index: usize) -> Option<&UpdateRecord> {
        self.records.get(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fail_closed_transaction() {
        let mut m = UpdateManager::new();
        let i = m
            .propose(Update {
                id: "u1".into(),
                target_version: "1.1".into(),
                digest: "sha256:x".into(),
            })
            .unwrap();
        assert_eq!(
            m.apply(i, UpdateAction::Apply),
            Err(UpdateError::InvalidTransition)
        );
        m.apply(i, UpdateAction::Verify).unwrap();
        m.apply(i, UpdateAction::Stage).unwrap();
        m.apply(i, UpdateAction::Apply).unwrap();
        assert_eq!(m.get(i).unwrap().state, UpdateState::Applied);
    }
}
