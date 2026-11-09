//! L1 adapter to the single canonical A-TownChain monetary policy source.
//!
//! The consensus monetary schedule is defined exactly once in
//! atc-algorithm::economics. This module exposes the values needed by the
//! blockchain kernel without defining a second schedule.

pub use atc_algorithm::economics::{
    MonetaryPolicy, ATC_BASE_UNITS, FINAL_EMISSION_BLOCK, HALVING_INTERVAL_BLOCKS, INITIAL_SUBSIDY,
    MAX_HALVINGS, MAX_SUPPLY, TARGET_BLOCK_TIME_SECS,
};

pub const NATIVE_COIN_SYMBOL: &str = "ATC";
pub const MAX_ATC_SUPPLY: u64 = 360_000_000;
pub const BLOCK_TIME_SECONDS: u64 = TARGET_BLOCK_TIME_SECS;
pub const HALVING_EVENTS: u64 = MAX_HALVINGS as u64;
pub const HALVING_INTERVAL: u64 = HALVING_INTERVAL_BLOCKS;
pub const INITIAL_BLOCK_REWARD_ATC: u64 = 500;

/// Canonical block subsidy in base units.
pub const fn block_reward_base_units(height: u64, issued_before_block: u128) -> u128 {
    MonetaryPolicy::subsidy(height, issued_before_block)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kernel_uses_canonical_monetary_policy() {
        assert_eq!(MAX_SUPPLY, 360_000_000 * ATC_BASE_UNITS);
        assert_eq!(BLOCK_TIME_SECONDS, 360);
        assert_eq!(HALVING_INTERVAL, 360_000);
        assert_eq!(HALVING_EVENTS, 36);
        assert_eq!(INITIAL_BLOCK_REWARD_ATC, 500);
        assert_eq!(FINAL_EMISSION_BLOCK, 12_959_999);
    }

    #[test]
    fn reward_boundaries_match_canonical_policy() {
        assert_eq!(block_reward_base_units(0, 0), 500 * ATC_BASE_UNITS);
        assert_eq!(block_reward_base_units(360_000, 0), 250 * ATC_BASE_UNITS);
        assert_eq!(block_reward_base_units(12_960_000, 0), 0);
    }
}
