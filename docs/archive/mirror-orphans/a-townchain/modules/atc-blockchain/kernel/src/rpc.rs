//! Stable node API boundary.
use super::{mempool::Transaction, Block, Node};
pub struct Rpc<'a> {
    pub node: &'a Node,
}
impl<'a> Rpc<'a> {
    pub fn chain_height(&self) -> u64 {
        self.node.chain.height()
    }
    pub fn submit_transaction(&self, tx: Transaction, now: u64) -> Result<[u8; 32], String> {
        self.node.submit(tx, now).map_err(|e| format!("{e:?}"))
    }
    pub fn produce_block(&self, now: u64, max: usize) -> Result<Block, String> {
        self.node.produce(now, max)
    }
    pub fn get_block(&self, height: u64) -> Option<Block> {
        self.node.storage.block(height)
    }
    pub fn get_state_root(&self, height: u64) -> Option<[u8; 32]> {
        self.node.storage.state_root(height)
    }
}
