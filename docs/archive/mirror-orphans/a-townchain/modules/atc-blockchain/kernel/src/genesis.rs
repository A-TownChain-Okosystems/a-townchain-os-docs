//! Reproducible canonical chain genesis configuration.
use crate::economics::{
    BLOCK_TIME_SECONDS, HALVING_EVENTS, HALVING_INTERVAL_BLOCKS, MAX_ATC_SUPPLY,
};

pub const PROTOCOL_VERSION: &str = "1.0.0";
pub const NETWORK_ID: &str = "a-townchain-mainnet";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Genesis {
    pub chain_id: u64,
    pub timestamp: u64,
    pub protocol_version: String,
    pub network_id: String,
    pub block_time_seconds: u64,
    pub halving_interval_blocks: u64,
    pub halving_events: u64,
    pub max_supply: u64,
    pub initial_allocations: Vec<(String, u64)>,
}

impl Genesis {
    pub fn new(chain_id: u64, timestamp: u64, allocations: Vec<(String, u64)>) -> Self {
        let mut a = allocations;
        a.sort();
        a.dedup_by(|x, y| x.0 == y.0);
        Self {
            chain_id,
            timestamp,
            protocol_version: PROTOCOL_VERSION.into(),
            network_id: NETWORK_ID.into(),
            block_time_seconds: BLOCK_TIME_SECONDS,
            halving_interval_blocks: HALVING_INTERVAL_BLOCKS,
            halving_events: HALVING_EVENTS,
            max_supply: MAX_ATC_SUPPLY,
            initial_allocations: a,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.chain_id == 0 {
            return Err("chain id must be non-zero".into());
        }
        if self.protocol_version != PROTOCOL_VERSION {
            return Err("unsupported protocol version".into());
        }
        if self.network_id != NETWORK_ID {
            return Err("unsupported network id".into());
        }
        if self.block_time_seconds != BLOCK_TIME_SECONDS {
            return Err("invalid block time".into());
        }
        if self.halving_interval_blocks != HALVING_INTERVAL_BLOCKS {
            return Err("invalid halving interval".into());
        }
        if self.halving_events != HALVING_EVENTS {
            return Err("invalid halving event count".into());
        }
        if self.max_supply != MAX_ATC_SUPPLY {
            return Err("invalid maximum supply".into());
        }
        if self.initial_allocations.iter().any(|(_, v)| *v == 0) {
            return Err("zero allocation".into());
        }
        let total = self
            .initial_allocations
            .iter()
            .try_fold(0u64, |acc, (_, v)| acc.checked_add(*v))
            .ok_or("genesis allocation overflow".to_string())?;
        if total > self.max_supply {
            return Err("genesis allocations exceed maximum supply".into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_genesis_accepts_valid_configuration() {
        let g = Genesis::new(658467, 1, vec![(String::from("genesis"), 100)]);
        assert_eq!(g.validate(), Ok(()));
    }

    #[test]
    fn canonical_genesis_rejects_supply_overflow() {
        let g = Genesis::new(
            658467,
            1,
            vec![(String::from("a"), MAX_ATC_SUPPLY), (String::from("b"), 1)],
        );
        assert!(g.validate().is_err());
    }
}
