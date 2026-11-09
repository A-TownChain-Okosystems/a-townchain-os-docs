//! Deterministic validator-weighted finality boundary with signed-vote validation.

/// Validator registry: validator id -> voting weight.
pub type ValidatorWeights = BTreeMap<String, u64>;
/// Canonical Ed25519 identity binding per validator id.
pub type ValidatorKeys = BTreeMap<String, [u8; 32]>;
/// Sealed snapshot: weights plus their identity binding.
pub type SealedValidatorSnapshot = (ValidatorWeights, ValidatorKeys);
/// Snapshot with the height at which the set became active.
pub type ActivationSnapshot = (u64, ValidatorWeights, ValidatorKeys);
use crate::security::simple_hash;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::atomic::AtomicBool,
    sync::Mutex,
};

pub const EPOCH_LENGTH_BLOCKS: u64 = crate::economics::HALVING_INTERVAL_BLOCKS;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SlashingEvidence {
    pub validator: String,
    pub height: u64,
    pub block_a: [u8; 32],
    pub block_b: [u8; 32],
    pub approve_a: bool,
    pub approve_b: bool,
    pub public_key: [u8; 32],
    pub signature_a: [u8; 64],
    pub signature_b: [u8; 64],
    pub reason: String,
}

impl SlashingEvidence {
    pub fn id(&self) -> [u8; 32] {
        let mut b = Vec::from(b"ATC-SLASH-V1".as_slice());
        b.extend_from_slice(&(self.validator.len() as u32).to_be_bytes());
        b.extend_from_slice(self.validator.as_bytes());
        b.extend_from_slice(&self.height.to_be_bytes());
        b.extend_from_slice(&self.block_a);
        b.extend_from_slice(&self.block_b);
        b.extend_from_slice(&(self.reason.len() as u32).to_be_bytes());
        b.extend_from_slice(self.reason.as_bytes());
        simple_hash(&b)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Vote {
    pub block: [u8; 32],
    pub voter: String,
    pub approve: bool,
    pub signature: [u8; 64],
    pub public_key: [u8; 32],
}

pub fn vote_signing_bytes(chain: u64, v: &Vote) -> Vec<u8> {
    let mut b = Vec::new();
    b.extend_from_slice(b"ATC-VOTE-V1");
    b.extend_from_slice(&chain.to_be_bytes());
    b.extend_from_slice(&v.block);
    b.push(v.approve as u8);
    b.extend_from_slice(&(v.voter.len() as u32).to_be_bytes());
    b.extend_from_slice(v.voter.as_bytes());
    b
}

/// A vote recorded together with the height at which it was cast. Finality
/// is height-scoped: only votes cast at the block's height contribute.
pub struct RecordedVote {
    pub vote: Vote,
    pub height: u64,
}

pub struct ConsensusEngine {
    pub chain_id: u64,
    pub proposer: String,
    height: Mutex<u64>,
    finalized: Mutex<Option<(u64, [u8; 32])>>,
    slashed: Mutex<BTreeMap<String, u64>>,
    votes: Mutex<BTreeMap<[u8; 32], Vec<RecordedVote>>>,
    validators: Mutex<BTreeMap<String, u64>>,
    validator_keys: Mutex<BTreeMap<String, [u8; 32]>>,
    /// Immutable validator-set snapshots keyed by the height at which the
    /// set became active. Consensus verification never falls back to the
    /// mutable current registry for historical blocks.
    validator_snapshots: Mutex<BTreeMap<u64, SealedValidatorSnapshot>>,
    had_complete_bootstrap: AtomicBool,
}

impl ConsensusEngine {
    pub fn new(chain_id: u64, proposer: String) -> Self {
        Self {
            chain_id,
            proposer,
            height: Mutex::new(0),
            finalized: Mutex::new(None),
            slashed: Mutex::new(BTreeMap::new()),
            votes: Mutex::new(BTreeMap::new()),
            validators: Mutex::new(BTreeMap::new()),
            validator_keys: Mutex::new(BTreeMap::new()),
            validator_snapshots: Mutex::new(BTreeMap::new()),
            had_complete_bootstrap: AtomicBool::new(false),
        }
    }

    fn capture_validator_snapshot(&self, height: u64) -> Result<(), String> {
        let validators = self
            .validators
            .lock()
            .map_err(|_| "validator lock poisoned".to_string())?
            .clone();
        let keys = self
            .validator_keys
            .lock()
            .map_err(|_| "validator key lock poisoned".to_string())?
            .clone();
        if validators.len() != keys.len() || validators.keys().any(|id| !keys.contains_key(id)) {
            return Err(
                "cannot snapshot validator registry without complete public-key bindings".into(),
            );
        }
        self.validator_snapshots
            .lock()
            .map_err(|_| "validator snapshot lock poisoned".to_string())?
            .entry(height)
            .or_insert((validators, keys));
        Ok(())
    }

    pub fn restore_validator_snapshot(
        &self,
        height: u64,
        validators: BTreeMap<String, u64>,
        keys: BTreeMap<String, [u8; 32]>,
    ) -> Result<(), String> {
        if validators.len() != keys.len() || validators.keys().any(|id| !keys.contains_key(id)) {
            return Err("validator snapshot has incomplete public-key bindings".into());
        }
        self.validator_snapshots
            .lock()
            .map_err(|_| "validator snapshot lock poisoned".to_string())?
            .insert(height, (validators, keys));
        Ok(())
    }

    /// A validator set is available to authenticate `height` when an exact
    /// snapshot exists at that height or the latest activation at or below
    /// it can serve as the effective set (bootstrap/pending fallback).
    pub fn has_validator_snapshot(&self, height: u64) -> bool {
        match self.validator_snapshots.lock() {
            Ok(snapshots) => {
                snapshots.contains_key(&height) || snapshots.range(..=height).next_back().is_some()
            }
            Err(_) => false,
        }
    }

    /// Deterministic commitment to the validator set that authenticates a block height.
    /// The lookup intentionally uses the latest activation at or before the target height,
    /// matching consensus verification semantics while remaining independent of mutable state.
    pub fn validator_snapshot_commitment(&self, height: u64) -> Option<[u8; 32]> {
        let (validators, keys) = self.effective_validator_snapshot_for_height(height)?;
        let mut bytes = Vec::from(b"ATC-VALIDATOR-SET-V1".as_slice());
        for (address, stake) in &validators {
            bytes.extend_from_slice(&(address.len() as u32).to_be_bytes());
            bytes.extend_from_slice(address.as_bytes());
            bytes.extend_from_slice(&stake.to_be_bytes());
            bytes.extend_from_slice(keys.get(address)?);
        }
        Some(simple_hash(&bytes))
    }

    pub fn validator_snapshot_commitment_from(
        validators: &BTreeMap<String, u64>,
        keys: &BTreeMap<String, [u8; 32]>,
    ) -> Option<[u8; 32]> {
        if validators.len() != keys.len()
            || validators.keys().any(|address| !keys.contains_key(address))
        {
            return None;
        }
        let mut bytes = Vec::from(b"ATC-VALIDATOR-SET-V1".as_slice());
        for (address, stake) in validators {
            if address.is_empty()
                || *stake == 0
                || ed25519_dalek::VerifyingKey::from_bytes(keys.get(address)?).is_err()
            {
                return None;
            }
            bytes.extend_from_slice(&(address.len() as u32).to_be_bytes());
            bytes.extend_from_slice(address.as_bytes());
            bytes.extend_from_slice(&stake.to_be_bytes());
            bytes.extend_from_slice(keys.get(address)?);
        }
        Some(simple_hash(&bytes))
    }

    /// Exact-height snapshot lookup. Consensus verification (votes, slashing)
    /// must authenticate against the snapshot captured for that exact height;
    /// falling back to a later or mutable set would rewrite history.
    pub fn validator_snapshot_for_height(&self, height: u64) -> Option<SealedValidatorSnapshot> {
        self.validator_snapshots.lock().ok()?.get(&height).cloned()
    }

    /// Effective snapshot for block production and import: the exact snapshot
    /// at the target height, else the latest activation at or below it, else
    /// the mutable registry (bootstrap fallback).
    pub fn effective_validator_snapshot_for_height(
        &self,
        height: u64,
    ) -> Option<SealedValidatorSnapshot> {
        if let Some(snapshot) = self.validator_snapshot_for_height(height) {
            return Some(snapshot);
        }
        let snapshots = self.validator_snapshots.lock().ok()?;
        if let Some((_, snapshot)) = snapshots.range(..=height).next_back() {
            return Some(snapshot.clone());
        }
        drop(snapshots);
        let validators = self.validators.lock().ok()?.clone();
        if validators.is_empty() {
            return None;
        }
        let keys = self.validator_keys.lock().ok()?.clone();
        Some((validators, keys))
    }

    pub fn validator_snapshot_with_activation_for_height(
        &self,
        height: u64,
    ) -> Option<ActivationSnapshot> {
        let snapshots = self.validator_snapshots.lock().ok()?;
        if let Some((activation, (validators, keys))) = snapshots.range(..=height).next_back() {
            return Some((*activation, validators.clone(), keys.clone()));
        }
        drop(snapshots);
        let validators = self.validators.lock().ok()?.clone();
        if validators.is_empty() {
            return None;
        }
        let keys = self.validator_keys.lock().ok()?.clone();
        Some((height, validators, keys))
    }

    pub fn validator_snapshot_heights(&self) -> Vec<u64> {
        self.validator_snapshots
            .lock()
            .ok()
            .map(|s| s.keys().copied().collect())
            .unwrap_or_default()
    }

    pub fn register_validator(&self, address: String, stake: u64) -> Result<(), String> {
        if address.is_empty() || stake == 0 {
            return Err("validator address and stake are required".into());
        }
        self.validators
            .lock()
            .map_err(|_| "validator lock poisoned")?
            .insert(address, stake);
        Ok(())
    }

    pub fn unregister_validator(&self, address: &str) {
        self.validators.lock().unwrap().remove(address);
        self.validator_keys.lock().unwrap().remove(address);
    }

    pub fn register_validator_key(
        &self,
        address: &str,
        public_key: [u8; 32],
    ) -> Result<(), String> {
        if self.validator_stake(address) == 0 {
            return Err("validator must be registered before its signing key".into());
        }
        VerifyingKey::from_bytes(&public_key)
            .map_err(|_| "invalid validator public key".to_string())?;
        self.validator_keys
            .lock()
            .map_err(|_| "validator key lock poisoned".to_string())?
            .insert(address.to_owned(), public_key);
        let complete = {
            let validators = self
                .validators
                .lock()
                .map_err(|_| "validator lock poisoned".to_string())?;
            let keys = self
                .validator_keys
                .lock()
                .map_err(|_| "validator key lock poisoned".to_string())?;
            !validators.is_empty()
                && validators.len() == keys.len()
                && validators.keys().all(|id| keys.contains_key(id))
        };
        // Bootstrap coalescence: the first registry completion captures the
        // whole bootstrap set as one snapshot at the current height. Later
        // completions (key rotation, restart replay) never auto-capture: the
        // node persists next-height activations explicitly, and explicit
        // restores are authoritative and never overwritten.
        let mut bootstrap_captured = false;
        if complete
            && !self
                .had_complete_bootstrap
                .swap(true, std::sync::atomic::Ordering::AcqRel)
            && self
                .validator_snapshots
                .lock()
                .map(|s| s.is_empty())
                .unwrap_or(false)
        {
            bootstrap_captured = true;
            self.capture_validator_snapshot(self.height())?;
        }
        let _ = bootstrap_captured;
        Ok(())
    }

    pub fn validator_public_key(&self, address: &str) -> Option<[u8; 32]> {
        self.validator_keys.lock().ok()?.get(address).copied()
    }

    pub fn mutable_validator_state(
        &self,
    ) -> Result<(ValidatorWeights, ValidatorKeys, BTreeMap<String, u64>), String> {
        Ok((
            self.validators
                .lock()
                .map_err(|_| "validator lock poisoned".to_string())?
                .clone(),
            self.validator_keys
                .lock()
                .map_err(|_| "validator key lock poisoned".to_string())?
                .clone(),
            self.slashed
                .lock()
                .map_err(|_| "slashed lock poisoned".to_string())?
                .clone(),
        ))
    }

    pub fn restore_mutable_validator_state(
        &self,
        validators: BTreeMap<String, u64>,
        keys: BTreeMap<String, [u8; 32]>,
        slashed: BTreeMap<String, u64>,
    ) -> Result<(), String> {
        if validators.len() != keys.len()
            || validators.keys().any(|address| !keys.contains_key(address))
        {
            return Err("cannot restore incomplete validator identity state".into());
        }
        *self
            .validators
            .lock()
            .map_err(|_| "validator lock poisoned".to_string())? = validators;
        *self
            .validator_keys
            .lock()
            .map_err(|_| "validator key lock poisoned".to_string())? = keys;
        *self
            .slashed
            .lock()
            .map_err(|_| "slashed lock poisoned".to_string())? = slashed;
        Ok(())
    }

    pub fn validator_stake(&self, address: &str) -> u64 {
        self.validators
            .lock()
            .unwrap()
            .get(address)
            .copied()
            .unwrap_or(0)
    }

    pub fn epoch(height: u64) -> u64 {
        height / EPOCH_LENGTH_BLOCKS
    }

    pub fn is_epoch_boundary(height: u64) -> bool {
        height > 0 && height.is_multiple_of(EPOCH_LENGTH_BLOCKS)
    }

    pub fn slashed_stake(&self, address: &str) -> u64 {
        self.slashed
            .lock()
            .unwrap()
            .get(address)
            .copied()
            .unwrap_or(0)
    }

    pub fn slash(&self, evidence: SlashingEvidence, penalty: u64) -> Result<u64, String> {
        if evidence.block_a == evidence.block_b || evidence.validator.is_empty() || penalty == 0 {
            return Err("invalid slashing evidence".into());
        }
        // Evidence for the current tip height authenticates against the live
        // registry (the tip's votes are cast now, under the current keys).
        // Historical evidence must match the sealed snapshot at its height.
        let historical_keys = if evidence.height == self.height() {
            let keys = self
                .validator_keys
                .lock()
                .map_err(|_| "validator key lock poisoned".to_string())?;
            if keys.is_empty() {
                return Err(
                    "validator snapshot is unavailable for slashing evidence height".into(),
                );
            }
            keys.clone()
        } else {
            self.validator_snapshot_for_height(evidence.height)
                .ok_or("validator snapshot is unavailable for slashing evidence height")?
                .1
        };
        let expected_key = historical_keys
            .get(&evidence.validator)
            .copied()
            .ok_or("validator had no signing key at evidence height")?;
        if evidence.public_key != expected_key {
            return Err(
                "slashing evidence public key does not match validator identity at evidence height"
                    .into(),
            );
        }
        if evidence.block_a == evidence.block_b {
            return Err("slashing evidence blocks must conflict".into());
        }
        let verify_evidence =
            |block: [u8; 32], approve: bool, sig: [u8; 64]| -> Result<(), String> {
                let key = VerifyingKey::from_bytes(&expected_key)
                    .map_err(|_| "invalid validator public key".to_string())?;
                let mut bytes = Vec::new();
                bytes.extend_from_slice(b"ATC-SLASH-V1");
                bytes.extend_from_slice(&self.chain_id.to_be_bytes());
                bytes.extend_from_slice(&evidence.height.to_be_bytes());
                bytes.extend_from_slice(&block);
                bytes.push(approve as u8);
                bytes.extend_from_slice(&(evidence.validator.len() as u32).to_be_bytes());
                bytes.extend_from_slice(evidence.validator.as_bytes());
                key.verify(&bytes, &Signature::from_bytes(&sig))
                    .map_err(|_| "invalid slashing signature".to_string())
            };
        verify_evidence(evidence.block_a, evidence.approve_a, evidence.signature_a)?;
        verify_evidence(evidence.block_b, evidence.approve_b, evidence.signature_b)?;

        let mut validators = self
            .validators
            .lock()
            .map_err(|_| "validator lock poisoned")?;
        let current = validators
            .get(&evidence.validator)
            .copied()
            .ok_or("validator is not active")?;
        let applied = penalty.min(current);
        let remaining = current - applied;
        if remaining == 0 {
            validators.remove(&evidence.validator);
        } else {
            validators.insert(evidence.validator.clone(), remaining);
        }
        self.slashed
            .lock()
            .unwrap()
            .entry(evidence.validator)
            .and_modify(|x| *x = x.saturating_add(applied))
            .or_insert(applied);
        Ok(applied)
    }

    pub fn validators_snapshot(&self) -> BTreeMap<String, u64> {
        self.validators.lock().unwrap().clone()
    }

    /// Return the validator registry together with its canonical Ed25519
    /// identity binding for durable snapshots.
    pub fn validator_snapshot_with_keys(&self) -> Result<SealedValidatorSnapshot, String> {
        let validators = self
            .validators
            .lock()
            .map_err(|_| "validator lock poisoned".to_string())?
            .clone();
        let keys = self
            .validator_keys
            .lock()
            .map_err(|_| "validator key lock poisoned".to_string())?
            .clone();
        if validators.len() != keys.len()
            || validators.keys().any(|address| !keys.contains_key(address))
        {
            return Err("validator registry contains an identity without a public key".into());
        }
        Ok((validators, keys))
    }

    pub fn total_validator_stake(&self) -> u64 {
        self.validators
            .lock()
            .unwrap()
            .values()
            .copied()
            .fold(0, u64::saturating_add)
    }

    pub fn propose_id(&self, h: u64, parent: [u8; 32], state: [u8; 32], tx: [u8; 32]) -> [u8; 32] {
        let mut b = Vec::new();
        b.extend_from_slice(&self.chain_id.to_be_bytes());
        b.extend_from_slice(&h.to_be_bytes());
        b.extend_from_slice(&parent);
        b.extend_from_slice(&state);
        b.extend_from_slice(&tx);
        b.extend_from_slice(self.proposer.as_bytes());
        simple_hash(&b)
    }

    pub fn verify_vote_signature(&self, v: &Vote) -> Result<(), String> {
        let pk = VerifyingKey::from_bytes(&v.public_key)
            .map_err(|_| "invalid vote public key".to_string())?;
        pk.verify(
            &vote_signing_bytes(self.chain_id, v),
            &Signature::from_bytes(&v.signature),
        )
        .map_err(|_| "invalid vote signature".to_string())
    }

    pub fn vote_at_height(&self, v: Vote, height: u64) -> Result<(), String> {
        self.verify_vote_signature(&v)?;

        match self.validator_snapshot_for_height(height) {
            Some((validators, keys)) => {
                if !validators.contains_key(&v.voter) {
                    return Err("voter is not an active validator at vote height".into());
                }
                let expected_key = keys
                    .get(&v.voter)
                    .copied()
                    .ok_or("validator signing key is not registered at vote height")?;
                if expected_key != v.public_key {
                    return Err("vote public key does not match validator identity".into());
                }
            }
            None => {
                // Effective (fallback) authentication: heights without a
                // captured snapshot authenticate against the mutable registry.
                // Any failure is reported as a missing snapshot so callers
                // cannot distinguish registry contents from absence.
                let validators = self
                    .validators
                    .lock()
                    .map_err(|_| "validator lock poisoned".to_string())?;
                let keys = self
                    .validator_keys
                    .lock()
                    .map_err(|_| "validator key lock poisoned".to_string())?;
                let authenticated = validators.contains_key(&v.voter)
                    && keys.get(&v.voter).copied() == Some(v.public_key);
                if !authenticated {
                    return Err("validator snapshot is unavailable for vote height".into());
                }
            }
        }
        let mut all = self
            .votes
            .lock()
            .map_err(|_| "vote lock poisoned".to_string())?;
        let list = all.entry(v.block).or_default();
        if let Some(existing) = list
            .iter()
            .find(|x| x.vote.voter == v.voter && x.height == height)
        {
            // A re-submitted identical vote (e.g. a broadcast echoed back to
            // its origin, or an operator replay) is idempotent. A conflicting
            // re-vote by the same validator stays a protocol violation.
            if existing.vote.approve == v.approve && existing.vote.signature == v.signature {
                return Ok(());
            }
            return Err("conflicting duplicate voter".into());
        }
        list.push(RecordedVote { vote: v, height });
        Ok(())
    }

    pub fn vote(&self, v: Vote) -> Result<(), String> {
        self.vote_at_height(v, self.height())
    }

    /// Height-scoped count-based finality. The block height must resolve to a
    /// validator snapshot before votes can contribute to finality.
    pub fn finality_at_height(&self, id: &[u8; 32], height: u64, quorum: usize) -> bool {
        if quorum == 0 || self.validator_snapshot_for_height(height).is_none() {
            return false;
        }
        self.votes
            .lock()
            .unwrap()
            .get(id)
            .map(|v| {
                v.iter()
                    .filter(|x| x.vote.approve && x.height == height)
                    .map(|x| x.vote.voter.as_str())
                    .collect::<BTreeSet<_>>()
                    .len()
                    >= quorum
            })
            .unwrap_or(false)
    }

    pub fn finality(&self, id: &[u8; 32], quorum: usize) -> bool {
        self.finality_at_height(id, self.height(), quorum)
    }

    /// Canonical L1 finality: at least two thirds of registered validator stake.
    pub fn weighted_finality_at_height(&self, id: &[u8; 32], height: u64) -> bool {
        let validators = match self.validator_snapshot_for_height(height) {
            Some((validators, _keys)) => validators,
            None => match self.validators.lock() {
                Ok(validators) => validators.clone(),
                Err(_) => return false,
            },
        };
        let total = validators.values().copied().fold(0u64, u64::saturating_add);
        if total == 0 {
            return false;
        }
        let votes = self.votes.lock().unwrap();
        let Some(list) = votes.get(id) else {
            return false;
        };
        let mut seen = BTreeSet::new();
        let approved = list
            .iter()
            .filter(|v| v.vote.approve && v.height == height && seen.insert(v.vote.voter.as_str()))
            .filter_map(|v| validators.get(&v.vote.voter).copied())
            .fold(0u64, u64::saturating_add);
        (approved as u128) * 3 > (total as u128) * 2
    }

    /// True when a byte-identical vote is already recorded at this height.
    pub fn has_identical_vote(&self, vote: &Vote, height: u64) -> bool {
        let Ok(votes) = self.votes.lock() else {
            return false;
        };
        votes
            .get(&vote.block)
            .map(|list| {
                list.iter().any(|x| {
                    x.height == height
                        && x.vote.voter == vote.voter
                        && x.vote.approve == vote.approve
                        && x.vote.signature == vote.signature
                        && x.vote.public_key == vote.public_key
                })
            })
            .unwrap_or(false)
    }

    pub fn weighted_finality(&self, id: &[u8; 32]) -> bool {
        self.weighted_finality_at_height(id, self.height())
    }

    pub fn mark_finalized(&self, height: u64, block: [u8; 32]) -> Result<(), String> {
        if height > self.height() {
            return Err("cannot finalize above current consensus height".into());
        }
        let mut finalized = self
            .finalized
            .lock()
            .map_err(|_| "finality lock poisoned".to_string())?;
        if let Some((current, current_id)) = *finalized {
            if height < current || (height == current && block != current_id) {
                return Err("finalized height regression or conflicting block".into());
            }
        }
        *finalized = Some((height, block));
        Ok(())
    }

    pub fn finalized(&self) -> Option<(u64, [u8; 32])> {
        *self.finalized.lock().unwrap()
    }

    pub fn set_height(&self, h: u64) {
        *self.height.lock().unwrap() = h
    }

    pub fn height(&self) -> u64 {
        *self.height.lock().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};

    fn signed_vote(
        engine: &ConsensusEngine,
        key: &SigningKey,
        voter: &str,
        block: [u8; 32],
        approve: bool,
    ) -> Vote {
        let mut vote = Vote {
            block,
            voter: voter.into(),
            approve,
            signature: [0; 64],
            public_key: key.verifying_key().to_bytes(),
        };
        let sig = key.sign(&vote_signing_bytes(engine.chain_id, &vote));
        vote.signature = sig.to_bytes();
        vote
    }

    #[test]
    fn historical_validator_snapshot_is_immutable_for_vote_verification() {
        let engine = ConsensusEngine::new(658467, "proposer".into());
        let old_key = SigningKey::from_bytes(&[21u8; 32]);
        let new_key = SigningKey::from_bytes(&[22u8; 32]);

        engine.register_validator("alice".into(), 100).unwrap();
        engine
            .register_validator_key("alice", old_key.verifying_key().to_bytes())
            .unwrap();
        engine.set_height(1);

        // A later mutable registry update must not rewrite the historical set
        // that authenticated height 0.
        engine
            .register_validator_key("alice", new_key.verifying_key().to_bytes())
            .unwrap();
        assert_eq!(
            engine
                .validator_snapshot_for_height(0)
                .unwrap()
                .1
                .get("alice"),
            Some(&old_key.verifying_key().to_bytes())
        );
        assert!(engine.validator_snapshot_for_height(1).is_none());

        let mut historical = Vote {
            block: [41u8; 32],
            voter: "alice".into(),
            approve: true,
            signature: [0; 64],
            public_key: old_key.verifying_key().to_bytes(),
        };
        historical.signature = old_key
            .sign(&vote_signing_bytes(engine.chain_id, &historical))
            .to_bytes();
        assert!(engine.vote_at_height(historical, 0).is_ok());

        let mut current = Vote {
            block: [42u8; 32],
            voter: "alice".into(),
            approve: true,
            signature: [0; 64],
            public_key: new_key.verifying_key().to_bytes(),
        };
        current.signature = new_key
            .sign(&vote_signing_bytes(engine.chain_id, &current))
            .to_bytes();
        assert!(engine.vote_at_height(current, 1).is_ok());
    }

    #[test]
    fn old_key_is_rejected_after_rotation_until_next_height_snapshot() {
        let engine = ConsensusEngine::new(658467, "proposer".into());
        let old_key = SigningKey::from_bytes(&[31u8; 32]);
        let new_key = SigningKey::from_bytes(&[32u8; 32]);
        engine.register_validator("alice".into(), 100).unwrap();
        engine
            .register_validator_key("alice", old_key.verifying_key().to_bytes())
            .unwrap();
        engine.set_height(1);
        engine
            .register_validator_key("alice", new_key.verifying_key().to_bytes())
            .unwrap();

        let old_vote = signed_vote(&engine, &old_key, "alice", [51u8; 32], true);
        assert_eq!(
            engine.vote_at_height(old_vote, 1).unwrap_err(),
            "validator snapshot is unavailable for vote height"
        );

        engine
            .restore_validator_snapshot(
                1,
                engine.validators_snapshot(),
                [("alice".to_string(), new_key.verifying_key().to_bytes())]
                    .into_iter()
                    .collect(),
            )
            .unwrap();

        let old_vote = signed_vote(&engine, &old_key, "alice", [52u8; 32], true);
        assert_eq!(
            engine.vote_at_height(old_vote, 1).unwrap_err(),
            "vote public key does not match validator identity"
        );
        let new_vote = signed_vote(&engine, &new_key, "alice", [53u8; 32], true);
        assert!(engine.vote_at_height(new_vote, 1).is_ok());
    }

    #[test]
    fn vote_for_height_cannot_use_future_validator_snapshot() {
        let engine = ConsensusEngine::new(658467, "proposer".into());
        let old_key = SigningKey::from_bytes(&[33u8; 32]);
        let new_key = SigningKey::from_bytes(&[34u8; 32]);
        engine.register_validator("alice".into(), 100).unwrap();
        engine
            .register_validator_key("alice", old_key.verifying_key().to_bytes())
            .unwrap();
        engine
            .restore_validator_snapshot(
                0,
                engine.validators_snapshot(),
                [("alice".to_string(), old_key.verifying_key().to_bytes())]
                    .into_iter()
                    .collect(),
            )
            .unwrap();
        engine
            .register_validator_key("alice", new_key.verifying_key().to_bytes())
            .unwrap();
        engine
            .restore_validator_snapshot(
                2,
                engine.validators_snapshot(),
                [("alice".to_string(), new_key.verifying_key().to_bytes())]
                    .into_iter()
                    .collect(),
            )
            .unwrap();

        let vote = signed_vote(&engine, &old_key, "alice", [54u8; 32], true);
        assert!(engine.vote_at_height(vote.clone(), 0).is_ok());
        let wrong_height = signed_vote(&engine, &old_key, "alice", [55u8; 32], true);
        assert!(engine.vote_at_height(wrong_height, 2).is_err());
    }

    #[test]
    fn historical_finality_does_not_use_future_validator_weights() {
        let engine = ConsensusEngine::new(658467, "proposer".into());
        let a = SigningKey::from_bytes(&[35u8; 32]);
        let b = SigningKey::from_bytes(&[36u8; 32]);
        engine.register_validator("a".into(), 60).unwrap();
        engine.register_validator("b".into(), 40).unwrap();
        engine
            .register_validator_key("a", a.verifying_key().to_bytes())
            .unwrap();
        engine
            .register_validator_key("b", b.verifying_key().to_bytes())
            .unwrap();
        let keys0 = [
            (String::from("a"), a.verifying_key().to_bytes()),
            (String::from("b"), b.verifying_key().to_bytes()),
        ]
        .into_iter()
        .collect();
        engine
            .restore_validator_snapshot(0, engine.validators_snapshot(), keys0)
            .unwrap();
        engine
            .restore_validator_snapshot(
                1,
                [("a".to_string(), 100u64)].into_iter().collect(),
                [("a".to_string(), a.verifying_key().to_bytes())]
                    .into_iter()
                    .collect(),
            )
            .unwrap();

        let block0 = [56u8; 32];
        engine
            .vote_at_height(signed_vote(&engine, &a, "a", block0, true), 0)
            .unwrap();
        engine
            .vote_at_height(signed_vote(&engine, &b, "b", block0, true), 0)
            .unwrap();
        assert!(engine.weighted_finality_at_height(&block0, 0));
        assert!(!engine.weighted_finality_at_height(&block0, 1));
    }

    #[test]
    fn conflicting_votes_across_heights_do_not_cross_authentication_boundaries() {
        let engine = ConsensusEngine::new(658467, "proposer".into());
        let key = SigningKey::from_bytes(&[37u8; 32]);
        engine.register_validator("alice".into(), 100).unwrap();
        engine
            .register_validator_key("alice", key.verifying_key().to_bytes())
            .unwrap();
        engine
            .restore_validator_snapshot(
                0,
                engine.validators_snapshot(),
                [("alice".to_string(), key.verifying_key().to_bytes())]
                    .into_iter()
                    .collect(),
            )
            .unwrap();
        engine
            .restore_validator_snapshot(
                1,
                engine.validators_snapshot(),
                [("alice".to_string(), key.verifying_key().to_bytes())]
                    .into_iter()
                    .collect(),
            )
            .unwrap();

        let a = signed_vote(&engine, &key, "alice", [57u8; 32], true);
        let b = signed_vote(&engine, &key, "alice", [58u8; 32], false);
        assert!(engine.vote_at_height(a, 0).is_ok());
        assert!(engine.vote_at_height(b, 1).is_ok());
        assert!(engine.weighted_finality_at_height(&[57u8; 32], 0));
        assert!(!engine.weighted_finality_at_height(&[58u8; 32], 1));
    }

    #[test]
    fn weighted_finality_requires_two_thirds_stake() {
        let engine = ConsensusEngine::new(658467, "proposer".into());
        let a = SigningKey::from_bytes(&[1u8; 32]);
        let b = SigningKey::from_bytes(&[2u8; 32]);
        engine.register_validator("a".into(), 40).unwrap();
        engine.register_validator("b".into(), 35).unwrap();
        engine.register_validator("c".into(), 25).unwrap();
        engine
            .register_validator_key("a", a.verifying_key().to_bytes())
            .unwrap();
        engine
            .register_validator_key("b", b.verifying_key().to_bytes())
            .unwrap();
        let block = [9u8; 32];
        engine
            .vote(signed_vote(&engine, &a, "a", block, true))
            .unwrap();
        engine
            .vote(signed_vote(&engine, &b, "b", block, true))
            .unwrap();
        assert!(engine.weighted_finality(&block));
        let other = [8u8; 32];
        engine
            .vote(signed_vote(&engine, &a, "a", other, true))
            .unwrap();
        assert!(!engine.weighted_finality(&other));
    }

    #[test]
    fn epoch_is_height_deterministic() {
        assert_eq!(ConsensusEngine::epoch(0), 0);
        assert_eq!(ConsensusEngine::epoch(EPOCH_LENGTH_BLOCKS - 1), 0);
        assert_eq!(ConsensusEngine::epoch(EPOCH_LENGTH_BLOCKS), 1);
        assert!(ConsensusEngine::is_epoch_boundary(EPOCH_LENGTH_BLOCKS));
    }

    #[test]
    fn slashing_reduces_voting_weight_and_is_idempotent_by_state() {
        let engine = ConsensusEngine::new(658467, "proposer".into());
        engine.register_validator("a".into(), 100).unwrap();
        let key = SigningKey::from_bytes(&[3u8; 32]);
        let sign_evidence = |block: [u8; 32]| {
            let mut bytes = Vec::new();
            bytes.extend_from_slice(b"ATC-SLASH-V1");
            bytes.extend_from_slice(&engine.chain_id.to_be_bytes());
            bytes.extend_from_slice(&1u64.to_be_bytes());
            bytes.extend_from_slice(&block);
            bytes.push(1);
            bytes.extend_from_slice(&(1u32).to_be_bytes());
            bytes.extend_from_slice(b"a");
            key.sign(&bytes).to_bytes()
        };
        engine
            .register_validator_key("a", key.verifying_key().to_bytes())
            .unwrap();
        engine
            .restore_validator_snapshot(
                1,
                engine.validators_snapshot(),
                [("a".to_string(), key.verifying_key().to_bytes())]
                    .into_iter()
                    .collect(),
            )
            .unwrap();
        let evidence = SlashingEvidence {
            validator: "a".into(),
            height: 1,
            block_a: [1; 32],
            block_b: [2; 32],
            approve_a: true,
            approve_b: true,
            public_key: key.verifying_key().to_bytes(),
            signature_a: sign_evidence([1; 32]),
            signature_b: sign_evidence([2; 32]),
            reason: "double-sign".into(),
        };
        assert_eq!(engine.slash(evidence.clone(), 40).unwrap(), 40);
        assert_eq!(engine.validator_stake("a"), 60);
        assert_eq!(engine.slashed_stake("a"), 40);
        assert_eq!(engine.slash(evidence, 10).unwrap(), 10);
        assert_eq!(engine.validator_stake("a"), 50);
    }

    #[test]
    fn slashing_uses_validator_key_at_evidence_height_after_rotation() {
        let engine = ConsensusEngine::new(658467, "proposer".into());
        let old_key = SigningKey::from_bytes(&[81u8; 32]);
        let new_key = SigningKey::from_bytes(&[82u8; 32]);

        engine.register_validator("alice".into(), 100).unwrap();
        engine
            .register_validator_key("alice", old_key.verifying_key().to_bytes())
            .unwrap();
        engine
            .restore_validator_snapshot(
                1,
                engine.validators_snapshot(),
                [("alice".to_string(), old_key.verifying_key().to_bytes())]
                    .into_iter()
                    .collect(),
            )
            .unwrap();

        engine
            .register_validator_key("alice", new_key.verifying_key().to_bytes())
            .unwrap();
        engine
            .restore_validator_snapshot(
                2,
                engine.validators_snapshot(),
                [("alice".to_string(), new_key.verifying_key().to_bytes())]
                    .into_iter()
                    .collect(),
            )
            .unwrap();

        let sign = |key: &SigningKey, block: [u8; 32]| {
            let mut bytes = Vec::new();
            bytes.extend_from_slice(b"ATC-SLASH-V1");
            bytes.extend_from_slice(&engine.chain_id.to_be_bytes());
            bytes.extend_from_slice(&1u64.to_be_bytes());
            bytes.extend_from_slice(&block);
            bytes.push(1);
            bytes.extend_from_slice(&(5u32).to_be_bytes());
            bytes.extend_from_slice(b"alice");
            key.sign(&bytes).to_bytes()
        };
        let evidence = SlashingEvidence {
            validator: "alice".into(),
            height: 1,
            block_a: [91; 32],
            block_b: [92; 32],
            approve_a: true,
            approve_b: true,
            public_key: old_key.verifying_key().to_bytes(),
            signature_a: sign(&old_key, [91; 32]),
            signature_b: sign(&old_key, [92; 32]),
            reason: "double-sign".into(),
        };
        assert_eq!(engine.slash(evidence, 10).unwrap(), 10);

        let bad = SlashingEvidence {
            validator: "alice".into(),
            height: 1,
            block_a: [93; 32],
            block_b: [94; 32],
            approve_a: true,
            approve_b: true,
            public_key: new_key.verifying_key().to_bytes(),
            signature_a: sign(&new_key, [93; 32]),
            signature_b: sign(&new_key, [94; 32]),
            reason: "double-sign".into(),
        };
        assert_eq!(
            engine.slash(bad, 10).unwrap_err(),
            "slashing evidence public key does not match validator identity at evidence height"
        );
    }

    #[test]
    fn unregistered_votes_are_rejected() {
        let engine = ConsensusEngine::new(658467, "proposer".into());
        let key = SigningKey::from_bytes(&[7u8; 32]);
        let vote = signed_vote(&engine, &key, "unknown", [1u8; 32], true);
        assert_eq!(
            engine.vote(vote).unwrap_err(),
            "validator snapshot is unavailable for vote height"
        );
    }
}

#[cfg(test)]
mod key_binding_regression {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};

    #[test]
    fn mismatched_vote_key_is_rejected() {
        let engine = ConsensusEngine::new(658467, "proposer".into());
        let registered = SigningKey::from_bytes(&[11u8; 32]);
        let attacker = SigningKey::from_bytes(&[12u8; 32]);
        engine.register_validator("alice".into(), 100).unwrap();
        engine
            .register_validator_key("alice", registered.verifying_key().to_bytes())
            .unwrap();
        let mut vote = Vote {
            block: [1; 32],
            voter: "alice".into(),
            approve: true,
            signature: [0; 64],
            public_key: attacker.verifying_key().to_bytes(),
        };
        vote.signature = attacker
            .sign(&vote_signing_bytes(engine.chain_id, &vote))
            .to_bytes();
        assert_eq!(
            engine.vote(vote).unwrap_err(),
            "vote public key does not match validator identity"
        );
    }
}
