//! Package installation policy and deterministic package registry.
//! Signature verification remains delegated to globus-package.

use globus_package::{Package, Verification, verify_metadata};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageState {
    Registered,
    Installed,
    Removed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageRecord {
    pub package: Package,
    pub state: PackageState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageManagerError {
    Duplicate,
    Untrusted,
    Missing,
    InvalidTransition,
}

#[derive(Debug, Default)]
pub struct PackageManager {
    packages: BTreeMap<String, PackageRecord>,
}

impl PackageManager {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn register(&mut self, package: Package) -> Result<(), PackageManagerError> {
        if verify_metadata(&package) != Verification::Trusted {
            return Err(PackageManagerError::Untrusted);
        }
        if self.packages.contains_key(&package.name) {
            return Err(PackageManagerError::Duplicate);
        }
        self.packages.insert(
            package.name.clone(),
            PackageRecord {
                package,
                state: PackageState::Registered,
            },
        );
        Ok(())
    }
    pub fn install(&mut self, name: &str) -> Result<(), PackageManagerError> {
        let r = self
            .packages
            .get_mut(name)
            .ok_or(PackageManagerError::Missing)?;
        if r.state != PackageState::Registered {
            return Err(PackageManagerError::InvalidTransition);
        }
        r.state = PackageState::Installed;
        Ok(())
    }
    pub fn remove(&mut self, name: &str) -> Result<(), PackageManagerError> {
        let r = self
            .packages
            .get_mut(name)
            .ok_or(PackageManagerError::Missing)?;
        if r.state != PackageState::Installed {
            return Err(PackageManagerError::InvalidTransition);
        }
        r.state = PackageState::Removed;
        Ok(())
    }
    pub fn get(&self, name: &str) -> Option<&PackageRecord> {
        self.packages.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn p() -> Package {
        Package {
            name: "demo".into(),
            version: "1.0".into(),
            digest: "sha256:x".into(),
            signature: "sig".into(),
        }
    }
    #[test]
    fn lifecycle() {
        let mut m = PackageManager::new();
        m.register(p()).unwrap();
        m.install("demo").unwrap();
        m.remove("demo").unwrap();
        assert_eq!(m.get("demo").unwrap().state, PackageState::Removed);
    }
    #[test]
    fn rejects_untrusted() {
        let mut m = PackageManager::new();
        let mut x = p();
        x.signature.clear();
        assert_eq!(m.register(x), Err(PackageManagerError::Untrusted));
    }
}
