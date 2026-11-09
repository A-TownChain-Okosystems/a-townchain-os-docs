//! Real block-device/VFS boundary. Backends implement sector I/O; VFS never invents disk state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockGeometry {
    pub block_size: u32,
    pub block_count: u64,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockError {
    InvalidGeometry,
    OutOfRange,
    Buffer,
    ReadOnly,
}
pub trait BlockDevice {
    fn geometry(&self) -> BlockGeometry;
    fn read_block(&self, index: u64, out: &mut [u8]) -> Result<(), BlockError>;
    fn write_block(&mut self, index: u64, data: &[u8]) -> Result<(), BlockError>;
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Partition {
    pub first_block: u64,
    pub block_count: u64,
}
impl Partition {
    pub fn contains(&self, index: u64) -> bool {
        index >= self.first_block && index - self.first_block < self.block_count
    }
}
pub fn validate_geometry(g: BlockGeometry) -> Result<(), BlockError> {
    if g.block_size == 0 || g.block_count == 0 {
        Err(BlockError::InvalidGeometry)
    } else {
        Ok(())
    }
}
