//! Stable application ABI metadata and capability handles.
pub const ABI_VERSION: u32 = 1;
pub const ABI_MAJOR: u16 = 1;
pub const ABI_MINOR: u16 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ApiClass {
    Process = 1,
    Memory = 2,
    Ipc = 3,
    File = 4,
    Network = 5,
    Graphics = 6,
    Input = 7,
    Identity = 8,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AbiHeader {
    pub version: u32,
    pub class: ApiClass,
    pub size: u16,
}
impl AbiHeader {
    pub const fn new(class: ApiClass, size: u16) -> Self {
        Self {
            version: ABI_VERSION,
            class,
            size,
        }
    }
}
pub fn compatible(version: u32) -> bool {
    version == ABI_VERSION
}
