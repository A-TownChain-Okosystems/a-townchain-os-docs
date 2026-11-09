//! Finality-gated indexer sink. Only Node::finalize may invoke this interface.
use atc_blockchain::{Block, IndexerSink};
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
pub struct MemoryIndexer {
    blocks: Arc<Mutex<Vec<Block>>>,
}

impl MemoryIndexer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn blocks(&self) -> Vec<Block> {
        self.blocks.lock().unwrap().clone()
    }

    pub fn contains(&self, id: [u8; 32]) -> bool {
        self.blocks.lock().unwrap().iter().any(|b| b.id == id)
    }
}

impl IndexerSink for MemoryIndexer {
    fn ingest_finalized(&self, block: &Block) -> Result<(), String> {
        let mut b = self
            .blocks
            .lock()
            .map_err(|_| "indexer lock poisoned".to_string())?;
        if b.iter().any(|x| x.id == block.id) {
            return Ok(());
        }
        b.push(block.clone());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_block() {
        let i = MemoryIndexer::new();
        assert!(i.blocks().is_empty());
    }
}
