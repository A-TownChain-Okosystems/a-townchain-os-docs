// A-TownChain Ecosystem — canonical monetary-policy integration boundary
//
// The monetary-policy authority lives exclusively in
// components/atc-algorithm/src/economics.rs. This file is only an adapter
// surface for the A-TownChain component and MUST NOT define a second schedule.

use atc_algorithm::economics::{
    MonetaryPolicy, ATC_BASE_UNITS, FINAL_EMISSION_BLOCK as CANONICAL_FINAL_EMISSION_BLOCK,
    HALVING_INTERVAL_BLOCKS, MAX_HALVINGS, MAX_SUPPLY, INITIAL_SUBSIDY,
};

pub const MAX_SUPPLY_ATC: u64 = (MAX_SUPPLY / ATC_BASE_UNITS) as u64;
pub const HALVING_COUNT: u32 = MAX_HALVINGS;
pub const INITIAL_SUBSIDY_ATC: u64 = (INITIAL_SUBSIDY / ATC_BASE_UNITS) as u64;
pub const FINAL_EMISSION_BLOCK: u64 = CANONICAL_FINAL_EMISSION_BLOCK;
pub const POST_HALVING_BLOCK: u64 = HALVING_INTERVAL_BLOCKS * MAX_HALVINGS as u64;

pub const fn halving_epoch(height: u64) -> u32 {
    MonetaryPolicy::epoch(height)
}

pub const fn raw_subsidy_atc(height: u64) -> u128 {
    MonetaryPolicy::raw_subsidy(height)
}
