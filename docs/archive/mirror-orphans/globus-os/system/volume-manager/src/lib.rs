//! Deterministic volume inventory and lifecycle policy.
//!
//! This layer tracks logical volumes and their intended state. It does not
//! perform filesystem mounting or raw block I/O.

use std::collections::BTreeMap;

/// Logical volume state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VolumeState {
    Available,
    Active,
    Failed,
}

/// Logical volume descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Volume {
    pub id: u64,
    pub label: String,
    pub filesystem: String,
    pub readonly: bool,
}

/// Volume manager error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VolumeError {
    DuplicateId,
    EmptyLabel,
    EmptyFilesystem,
    Missing,
    InvalidTransition,
}

/// Deterministic volume manager.
#[derive(Debug, Default)]
pub struct VolumeManager {
    volumes: BTreeMap<u64, (Volume, VolumeState)>,
}

impl VolumeManager {
    /// Creates an empty volume manager.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a logical volume.
    pub fn register(&mut self, volume: Volume) -> Result<(), VolumeError> {
        if volume.label.is_empty() {
            return Err(VolumeError::EmptyLabel);
        }
        if volume.filesystem.is_empty() {
            return Err(VolumeError::EmptyFilesystem);
        }
        if self.volumes.contains_key(&volume.id) {
            return Err(VolumeError::DuplicateId);
        }
        self.volumes
            .insert(volume.id, (volume, VolumeState::Available));
        Ok(())
    }

    /// Changes the lifecycle state using explicit allowed transitions.
    pub fn set_state(&mut self, id: u64, state: VolumeState) -> Result<(), VolumeError> {
        let (_, current) = self.volumes.get_mut(&id).ok_or(VolumeError::Missing)?;
        let allowed = matches!(
            (*current, state),
            (VolumeState::Available, VolumeState::Active)
                | (VolumeState::Available, VolumeState::Failed)
                | (VolumeState::Active, VolumeState::Available)
                | (VolumeState::Active, VolumeState::Failed)
                | (VolumeState::Failed, VolumeState::Available)
        );
        if !allowed {
            return Err(VolumeError::InvalidTransition);
        }
        *current = state;
        Ok(())
    }

    /// Returns a volume and its current state.
    pub fn get(&self, id: u64) -> Option<(&Volume, VolumeState)> {
        self.volumes
            .get(&id)
            .map(|(volume, state)| (volume, *state))
    }

    /// Returns volumes in deterministic ID order.
    pub fn volumes(&self) -> impl Iterator<Item = (&Volume, VolumeState)> {
        self.volumes
            .values()
            .map(|(volume, state)| (volume, *state))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn volume(id: u64) -> Volume {
        Volume {
            id,
            label: format!("data-{id}"),
            filesystem: "atcfs".into(),
            readonly: false,
        }
    }

    #[test]
    fn lifecycle_is_explicit() {
        let mut manager = VolumeManager::new();
        manager.register(volume(2)).unwrap();
        manager.register(volume(1)).unwrap();
        assert_eq!(
            manager.volumes().map(|(v, _)| v.id).collect::<Vec<_>>(),
            vec![1, 2]
        );
        assert_eq!(manager.get(1).unwrap().1, VolumeState::Available);
        manager.set_state(1, VolumeState::Active).unwrap();
        assert_eq!(manager.get(1).unwrap().1, VolumeState::Active);
        assert!(manager.set_state(1, VolumeState::Active).is_err());
    }
}
