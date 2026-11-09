pub use atc_blockchain::chain_identity::NUMERIC_CHAIN_ID;
use atc_blockchain::{Block, Node};
use std::{path::Path, sync::Arc};

pub struct Runtime {
    pub node: Arc<Node>,
}

impl Runtime {
    pub fn devnet(proposer: impl Into<String>) -> Result<Self, String> {
        let node = Arc::new(Node::new(NUMERIC_CHAIN_ID, proposer.into()));
        node.state.genesis_credit("alice", 1_000_000)?;
        node.create_genesis(0)?;
        Ok(Self { node })
    }

    /// Open the canonical durable node state and reconstruct the chain/state from disk.
    ///
    /// This is deliberately separate from `devnet`: opening an existing path must never
    /// silently create a second genesis or discard previously committed state.
    pub fn open_storage<P: AsRef<Path>>(
        proposer: impl Into<String>,
        path: P,
    ) -> Result<Self, String> {
        Ok(Self {
            node: Arc::new(Node::open_storage(NUMERIC_CHAIN_ID, proposer.into(), path)?),
        })
    }

    pub fn submit(
        &self,
        tx: atc_blockchain::mempool::Transaction,
        now: u64,
    ) -> Result<[u8; 32], atc_blockchain::mempool::MempoolError> {
        self.node.submit(tx, now)
    }

    pub fn produce(&self, timestamp: u64, max: usize) -> Result<Block, String> {
        self.node.produce(timestamp, max)
    }

    pub fn finalize(&self, block: &Block) -> Result<bool, String> {
        self.node.finalize(block, 1)
    }
}
