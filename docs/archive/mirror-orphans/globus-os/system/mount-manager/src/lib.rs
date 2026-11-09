//! Deterministic mount lifecycle policy for GlobusOS.
//!
//! The manager owns mount policy and namespace state. Filesystem drivers and
//! block-device access remain below this layer.

use globus_vfs::{Mount, MountTable};

/// Mount lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MountState {
    Mounted,
    Unmounted,
}

/// Mount manager error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MountError {
    InvalidMountpoint,
    AlreadyMounted,
    NotMounted,
}

/// Deterministic mount manager.
#[derive(Debug, Default)]
pub struct MountManager {
    table: MountTable,
    states: std::collections::BTreeMap<String, MountState>,
}

impl MountManager {
    /// Creates an empty manager.
    pub fn new() -> Self {
        Self::default()
    }

    /// Mounts a filesystem at a normalized namespace path.
    pub fn mount(&mut self, mount: Mount) -> Result<(), MountError> {
        if mount.mountpoint.is_empty() || !mount.mountpoint.starts_with('/') {
            return Err(MountError::InvalidMountpoint);
        }
        if self.states.get(&mount.mountpoint) == Some(&MountState::Mounted) {
            return Err(MountError::AlreadyMounted);
        }
        if !self.table.add(mount.clone()) {
            return Err(MountError::AlreadyMounted);
        }
        self.states.insert(mount.mountpoint, MountState::Mounted);
        Ok(())
    }

    /// Marks an existing mount as unmounted.
    ///
    /// The VFS table is immutable for this operation; callers must rebuild or
    /// replace the namespace table before reusing the path.
    pub fn unmount(&mut self, mountpoint: &str) -> Result<(), MountError> {
        if self.states.get(mountpoint) != Some(&MountState::Mounted) {
            return Err(MountError::NotMounted);
        }
        self.states
            .insert(mountpoint.to_owned(), MountState::Unmounted);
        Ok(())
    }

    /// Resolves a path only when its mount is currently active.
    pub fn resolve(&self, path: &str) -> Option<&Mount> {
        let mount = self.table.resolve(path)?;
        if self.states.get(&mount.mountpoint) == Some(&MountState::Mounted) {
            Some(mount)
        } else {
            None
        }
    }

    /// Returns the current namespace table.
    pub fn mounts(&self) -> &[Mount] {
        self.table.mounts()
    }

    /// Returns the lifecycle state of a mountpoint.
    pub fn state(&self, mountpoint: &str) -> Option<MountState> {
        self.states.get(mountpoint).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mounts_and_resolves_active_namespace() {
        let mut manager = MountManager::new();
        manager
            .mount(Mount {
                mountpoint: "/home".into(),
                filesystem: "data".into(),
                readonly: false,
            })
            .unwrap();
        assert_eq!(manager.resolve("/home/user").unwrap().filesystem, "data");
        assert_eq!(manager.state("/home"), Some(MountState::Mounted));
    }

    #[test]
    fn unmount_disables_resolution() {
        let mut manager = MountManager::new();
        manager
            .mount(Mount {
                mountpoint: "/system".into(),
                filesystem: "system".into(),
                readonly: true,
            })
            .unwrap();
        manager.unmount("/system").unwrap();
        assert!(manager.resolve("/system/bin").is_none());
        assert_eq!(manager.state("/system"), Some(MountState::Unmounted));
    }
}
