//! Capability-oriented resource quotas.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceQuota {
    pub cpu_millis: u32,
    pub memory_bytes: u64,
    pub handles: u32,
    pub io_bytes_per_sec: u64,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceUsage {
    pub cpu_millis: u32,
    pub memory_bytes: u64,
    pub handles: u32,
    pub io_bytes_per_sec: u64,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceError {
    Exceeded,
}
pub fn check(q: ResourceQuota, u: ResourceUsage) -> Result<(), ResourceError> {
    if u.cpu_millis <= q.cpu_millis
        && u.memory_bytes <= q.memory_bytes
        && u.handles <= q.handles
        && u.io_bytes_per_sec <= q.io_bytes_per_sec
    {
        Ok(())
    } else {
        Err(ResourceError::Exceeded)
    }
}

#[derive(Debug, Default)]
pub struct ResourceManager;
impl ResourceManager {
    pub const fn new() -> Self {
        Self
    }
    pub fn check(&self, q: ResourceQuota, u: ResourceUsage) -> Result<(), ResourceError> {
        check(q, u)
    }
}
