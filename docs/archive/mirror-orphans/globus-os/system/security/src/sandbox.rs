//! Deny-by-default process/runtime sandbox policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SandboxResource {
    Memory,
    Cpu,
    Filesystem,
    Network,
    Device,
    Ipc,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceQuota {
    pub memory_bytes: u64,
    pub cpu_millis: u32,
    pub open_files: u32,
    pub network_connections: u32,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SandboxPolicy {
    pub allowed: u16,
    pub quota: ResourceQuota,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandboxError {
    ResourceDenied,
    QuotaExceeded,
}
impl SandboxPolicy {
    pub const fn deny_all(q: ResourceQuota) -> Self {
        Self {
            allowed: 0,
            quota: q,
        }
    }
    pub fn allow(mut self, r: SandboxResource) -> Self {
        self.allowed |= 1u16 << (r as u8);
        self
    }
    pub fn check(&self, r: SandboxResource) -> Result<(), SandboxError> {
        if self.allowed & (1u16 << (r as u8)) != 0 {
            Ok(())
        } else {
            Err(SandboxError::ResourceDenied)
        }
    }
    pub fn check_memory(&self, n: u64) -> Result<(), SandboxError> {
        if n <= self.quota.memory_bytes {
            Ok(())
        } else {
            Err(SandboxError::QuotaExceeded)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deny_by_default() {
        let p = SandboxPolicy::deny_all(ResourceQuota {
            memory_bytes: 1024,
            cpu_millis: 100,
            open_files: 8,
            network_connections: 1,
        });
        assert_eq!(
            p.check(SandboxResource::Network),
            Err(SandboxError::ResourceDenied)
        );
        assert!(
            p.allow(SandboxResource::Network)
                .check(SandboxResource::Network)
                .is_ok()
        );
    }
}
