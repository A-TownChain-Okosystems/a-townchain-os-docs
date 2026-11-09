//! Durable append-only block storage with crash recovery.
use std::{
    collections::BTreeMap,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    sync::RwLock,
};

use super::{
    mempool::{Account, Transaction, TxType},
    Block,
};

const MAGIC: &[u8] = b"ATCB1";
const VALIDATOR_MAGIC: &[u8] = b"ATCV2";
const LEGACY_VALIDATOR_MAGIC: &[u8] = b"ATCV1";
const FINALITY_MAGIC: &[u8] = b"ATCF1";
const SLASH_MAGIC: &[u8] = b"ATCS1";
const ISSUANCE_MAGIC: &[u8] = b"ATCI1";

type ValidatorSnapshot = (u64, BTreeMap<String, (u64, [u8; 32])>);
type SlashingRecord = (u64, String, [u8; 32], u64);

fn put(out: &mut Vec<u8>, b: &[u8]) {
    out.extend_from_slice(&(b.len() as u32).to_be_bytes());
    out.extend_from_slice(b);
}

fn get<'a>(b: &'a [u8], p: &mut usize) -> Result<&'a [u8], String> {
    if *p + 4 > b.len() {
        return Err("truncated length".into());
    }
    let n = u32::from_be_bytes(b[*p..*p + 4].try_into().unwrap()) as usize;
    *p += 4;
    if *p + n > b.len() {
        return Err("truncated field".into());
    }
    let r = &b[*p..*p + n];
    *p += n;
    Ok(r)
}

fn fixed<const N: usize>(b: &[u8], p: &mut usize) -> Result<[u8; N], String> {
    if *p + N > b.len() {
        return Err("truncated fixed field".into());
    }
    let r = b[*p..*p + N].try_into().unwrap();
    *p += N;
    Ok(r)
}

fn tx_encode(t: &Transaction, o: &mut Vec<u8>) {
    o.extend_from_slice(&t.chain_id.to_be_bytes());
    o.push(t.tx_type as u8);
    put(o, t.sender_did.as_bytes());
    match &t.recipient_did {
        Some(v) => {
            o.push(1);
            put(o, v.as_bytes());
        }
        None => o.push(0),
    }
    o.extend_from_slice(&t.amount.to_be_bytes());
    o.extend_from_slice(&t.gas_price.to_be_bytes());
    o.extend_from_slice(&t.gas_limit.to_be_bytes());
    o.extend_from_slice(&t.nonce.to_be_bytes());
    o.extend_from_slice(&t.timestamp.to_be_bytes());
    put(o, &t.payload);
    o.extend_from_slice(&t.signature);
    o.extend_from_slice(&t.public_key);
    o.extend_from_slice(&t.poh_hash);
}

fn tx_decode(b: &[u8], p: &mut usize) -> Result<Transaction, String> {
    let chain_id = u64::from_be_bytes(fixed::<8>(b, p)?);
    let ty = {
        let v = fixed::<1>(b, p)?[0];
        match v {
            0 => TxType::Transfer,
            1 => TxType::Stake,
            2 => TxType::Unstake,
            3 => TxType::Contract,
            4 => TxType::Validator,
            _ => return Err("invalid tx type".into()),
        }
    };
    let sender = String::from_utf8(get(b, p)?.to_vec()).map_err(|_| "invalid sender")?;
    let has = fixed::<1>(b, p)?[0];
    let recipient = if has == 1 {
        Some(String::from_utf8(get(b, p)?.to_vec()).map_err(|_| "invalid recipient")?)
    } else if has == 0 {
        None
    } else {
        return Err("invalid recipient flag".into());
    };
    let amount = u128::from_be_bytes(fixed::<16>(b, p)?);
    let gas_price = u64::from_be_bytes(fixed::<8>(b, p)?);
    let gas_limit = u64::from_be_bytes(fixed::<8>(b, p)?);
    let nonce = u64::from_be_bytes(fixed::<8>(b, p)?);
    let timestamp = u64::from_be_bytes(fixed::<8>(b, p)?);
    let payload = get(b, p)?.to_vec();
    let signature = fixed::<64>(b, p)?;
    let public_key = fixed::<32>(b, p)?;
    let poh = fixed::<32>(b, p)?;
    Ok(Transaction::new_with_chain_id(
        chain_id, ty, sender, recipient, amount, gas_price, gas_limit, nonce, timestamp, payload,
        signature, public_key, poh,
    ))
}

fn block_encode(b: &Block) -> Vec<u8> {
    let mut o = Vec::from(MAGIC);
    o.extend_from_slice(&b.height.to_be_bytes());
    o.extend_from_slice(&b.parent_hash);
    put(&mut o, b.proposer.as_bytes());
    o.extend_from_slice(&b.timestamp.to_be_bytes());
    o.extend_from_slice(&(b.transactions.len() as u32).to_be_bytes());
    for t in &b.transactions {
        let mut x = Vec::new();
        tx_encode(t, &mut x);
        put(&mut o, &x);
    }
    o.extend_from_slice(&b.tx_root);
    o.extend_from_slice(&b.state_root);
    o.extend_from_slice(&b.receipt_root);
    o.extend_from_slice(&b.signature);
    o.extend_from_slice(&b.id);
    o
}

fn block_decode(b: &[u8]) -> Result<Block, String> {
    if !b.starts_with(MAGIC) {
        return Err("invalid storage magic".into());
    }
    let mut p = MAGIC.len();
    let h = u64::from_be_bytes(fixed::<8>(b, &mut p)?);
    let parent = fixed::<32>(b, &mut p)?;
    let proposer = String::from_utf8(get(b, &mut p)?.to_vec()).map_err(|_| "invalid proposer")?;
    let ts = u64::from_be_bytes(fixed::<8>(b, &mut p)?);
    if p + 4 > b.len() {
        return Err("truncated tx count".into());
    }
    let n = u32::from_be_bytes(b[p..p + 4].try_into().unwrap()) as usize;
    p += 4;
    let mut txs = Vec::with_capacity(n);
    for _ in 0..n {
        let raw = get(b, &mut p)?;
        let mut q = 0usize;
        txs.push(tx_decode(raw, &mut q)?);
        if q != raw.len() {
            return Err("trailing tx bytes".into());
        }
    }
    let tx_root = fixed::<32>(b, &mut p)?;
    let state = fixed::<32>(b, &mut p)?;
    let receipt = fixed::<32>(b, &mut p)?;
    let sig = fixed::<64>(b, &mut p)?;
    let stored_id = fixed::<32>(b, &mut p)?;
    if p != b.len() {
        return Err("trailing storage bytes".into());
    }
    let block = Block::new(h, parent, proposer, ts, txs, state, receipt, sig);
    if block.tx_root != tx_root {
        return Err("transaction root mismatch".into());
    }
    if block.id != stored_id {
        return Err("block commitment mismatch".into());
    }
    Ok(block)
}

pub type DaoStateSnapshot = (BTreeMap<String, Account>, Vec<u8>);

pub struct ChainStorage {
    blocks: RwLock<BTreeMap<u64, Block>>,
    state_roots: RwLock<BTreeMap<u64, [u8; 32]>>,
    journal: Option<PathBuf>,
    state_journal: Option<PathBuf>,
    validator_journal: Option<PathBuf>,
    finality_journal: Option<PathBuf>,
    slashing_journal: Option<PathBuf>,
    issuance_journal: Option<PathBuf>,
}

impl Default for ChainStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl ChainStorage {
    pub fn new() -> Self {
        Self {
            blocks: RwLock::new(BTreeMap::new()),
            state_roots: RwLock::new(BTreeMap::new()),
            journal: None,
            state_journal: None,
            validator_journal: None,
            finality_journal: None,
            slashing_journal: None,
            issuance_journal: None,
        }
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let p = path.as_ref().to_path_buf();
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let state_p = p.with_extension("state");
        let validator_p = p.with_extension("validators");
        let finality_p = p.with_extension("finality");
        let slashing_p = p.with_extension("slashing");
        let issuance_p = p.with_extension("issuance");
        let s = Self {
            blocks: RwLock::new(BTreeMap::new()),
            state_roots: RwLock::new(BTreeMap::new()),
            journal: Some(p),
            state_journal: Some(state_p),
            validator_journal: Some(validator_p),
            finality_journal: Some(finality_p),
            slashing_journal: Some(slashing_p),
            issuance_journal: Some(issuance_p),
        };
        s.recover()?;
        // Eager journal validation: fail closed on any durable record set that
        // would violate invariants at restart (height regressions, torn
        // records, snapshots referencing missing blocks).
        s.recover_validator_snapshots()?;
        s.recover_finalized_inner(false)?;
        s.recover_issuance_inner(false)?;
        s.recover_slashing()?;
        Ok(s)
    }

    fn recover(&self) -> Result<(), String> {
        let Some(p) = &self.journal else {
            return Ok(());
        };
        if !p.exists() {
            return Ok(());
        }
        let f = File::open(p).map_err(|e| e.to_string())?;
        let mut expected_height = 0u64;
        let mut previous_id = None;
        for (line_no, line) in BufReader::new(f).lines().enumerate() {
            let l = line.map_err(|e| e.to_string())?;
            if l.trim().is_empty() {
                continue;
            }
            let bytes = hex::decode(l.trim())
                .map_err(|e| format!("journal line {}: invalid hex: {e}", line_no + 1))?;
            let block =
                block_decode(&bytes).map_err(|e| format!("journal line {}: {e}", line_no + 1))?;
            if block.height != expected_height {
                return Err(format!(
                    "journal line {}: non-sequential block height: expected {}, got {}",
                    line_no + 1,
                    expected_height,
                    block.height
                ));
            }
            if let Some(parent) = previous_id {
                if block.parent_hash != parent {
                    return Err(format!(
                        "journal line {}: canonical parent mismatch at height {}",
                        line_no + 1,
                        block.height
                    ));
                }
            } else if block.height != 0 || block.parent_hash != [0; 32] {
                return Err(format!(
                    "journal line {}: invalid genesis boundary",
                    line_no + 1
                ));
            }
            if let Some(existing) = self.blocks.read().unwrap().get(&block.height) {
                if existing.id != block.id {
                    return Err(format!(
                        "journal line {}: conflicting block at canonical height {}",
                        line_no + 1,
                        block.height
                    ));
                }
                continue;
            }
            previous_id = Some(block.id);
            self.state_roots
                .write()
                .unwrap()
                .insert(block.height, block.state_root);
            self.blocks.write().unwrap().insert(block.height, block);
            expected_height = expected_height.saturating_add(1);
        }
        Ok(())
    }

    fn append_block_record(&self, b: &Block) -> Result<(), String> {
        if let Some(p) = &self.journal {
            let line = format!("{}\n", hex::encode(block_encode(b)));
            let mut f = OpenOptions::new()
                .create(true)
                .append(true)
                .open(p)
                .map_err(|e| e.to_string())?;
            f.write_all(line.as_bytes()).map_err(|e| e.to_string())?;
            f.sync_data().map_err(|e| e.to_string())?;
        }
        self.state_roots
            .write()
            .unwrap()
            .insert(b.height, b.state_root);
        self.blocks.write().unwrap().insert(b.height, b.clone());
        Ok(())
    }

    pub fn commit(&self, b: Block) -> Result<(), String> {
        if let Some(existing) = self.blocks.read().unwrap().get(&b.height) {
            if existing.id == b.id {
                return Ok(());
            }
            return Err("conflicting block at canonical height".into());
        }
        if b.height > 0 {
            let parent_id = self
                .blocks
                .read()
                .unwrap()
                .get(&b.height.saturating_sub(1))
                .map(|parent| parent.id)
                .ok_or("cannot commit block without canonical parent")?;
            if b.parent_hash != parent_id {
                return Err("canonical parent mismatch".into());
            }
        } else if b.parent_hash != [0; 32] {
            return Err("invalid genesis parent".into());
        }
        self.append_block_record(&b)
    }

    fn append_state_snapshot(
        &self,
        height: u64,
        state: &BTreeMap<String, Account>,
        dao: &[u8],
    ) -> Result<(), String> {
        if let Some(p) = &self.state_journal {
            let mut o = Vec::new();
            o.extend_from_slice(&height.to_be_bytes());
            o.extend_from_slice(&(state.len() as u32).to_be_bytes());
            for (k, v) in state {
                put(&mut o, k.as_bytes());
                o.extend_from_slice(&v.balance.to_be_bytes());
                o.extend_from_slice(&v.staked.to_be_bytes());
                o.extend_from_slice(&v.nonce.to_be_bytes());
            }
            put(&mut o, dao);
            let line = format!("{}\n", hex::encode(o));
            let mut f = OpenOptions::new()
                .create(true)
                .append(true)
                .open(p)
                .map_err(|e| e.to_string())?;
            f.write_all(line.as_bytes()).map_err(|e| e.to_string())?;
            f.sync_data().map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub fn commit_state(
        &self,
        height: u64,
        state: &BTreeMap<String, Account>,
    ) -> Result<(), String> {
        self.commit_state_with_dao(height, state, &[])
    }

    pub fn commit_state_with_dao(
        &self,
        height: u64,
        state: &BTreeMap<String, Account>,
        dao: &[u8],
    ) -> Result<(), String> {
        self.block(height)
            .ok_or("state snapshot references missing block")?;
        self.append_state_snapshot(height, state, dao)
    }

    /// Commit the block-associated state as one ordered durable transaction.
    ///
    /// State and issuance are synced before the canonical block record. The block
    /// journal is therefore the durable commit point: after restart, a block that
    /// exists in the canonical journal necessarily follows durable state/issuance
    /// records, while a crash before the block write leaves those records above the
    /// canonical tip and recovery rejects them.
    pub fn commit_block_state_issuance_with_validator_snapshot(
        &self,
        block: Block,
        state: &BTreeMap<String, Account>,
        dao: &[u8],
        issued_base_units: u128,
        snapshot: &crate::consensus::ActivationSnapshot,
    ) -> Result<(), String> {
        let (activation_height, validators, validator_keys) = snapshot;
        if validators.len() != validator_keys.len()
            || validators
                .keys()
                .any(|address| !validator_keys.contains_key(address))
        {
            return Err("validator snapshot is incomplete".into());
        }
        if *activation_height > block.height {
            return Err("validator snapshot activates after synchronized block".into());
        }
        if let Some(existing) = self.blocks.read().unwrap().get(&block.height) {
            if existing.id == block.id {
                return Ok(());
            }
            return Err("conflicting block at canonical height".into());
        }
        if block.height > 0 {
            let parent_id = self
                .blocks
                .read()
                .unwrap()
                .get(&block.height.saturating_sub(1))
                .map(|parent| parent.id)
                .ok_or("cannot commit block without canonical parent")?;
            if block.parent_hash != parent_id {
                return Err("canonical parent mismatch".into());
            }
        } else if block.parent_hash != [0; 32] {
            return Err("invalid genesis parent".into());
        }
        if issued_base_units > crate::economics::MAX_SUPPLY {
            return Err("issued supply cap exceeded".into());
        }
        self.append_state_snapshot(block.height, state, dao)?;
        self.append_issuance_record(block.height, issued_base_units)?;
        self.commit_validators(*activation_height, validators, validator_keys)?;
        self.append_block_record(&block)
    }

    pub fn commit_block_state_issuance(
        &self,
        block: Block,
        state: &BTreeMap<String, Account>,
        dao: &[u8],
        issued_base_units: u128,
    ) -> Result<(), String> {
        if issued_base_units > crate::economics::MAX_SUPPLY {
            return Err("issued supply cap exceeded".into());
        }
        if let Some(existing) = self.blocks.read().unwrap().get(&block.height) {
            if existing.id == block.id {
                return Ok(());
            }
            return Err("conflicting block at canonical height".into());
        }
        if block.height > 0 {
            let parent_id = self
                .blocks
                .read()
                .unwrap()
                .get(&block.height.saturating_sub(1))
                .map(|parent| parent.id)
                .ok_or("cannot commit block without canonical parent")?;
            if block.parent_hash != parent_id {
                return Err("canonical parent mismatch".into());
            }
        } else if block.parent_hash != [0; 32] {
            return Err("invalid genesis parent".into());
        }

        self.append_state_snapshot(block.height, state, dao)?;
        self.append_issuance_record(block.height, issued_base_units)?;

        // This is intentionally the final durable write for the block commit.
        self.append_block_record(&block)
    }

    pub fn recover_state(&self) -> Result<Option<BTreeMap<String, Account>>, String> {
        Ok(self.recover_state_with_dao_at_height()?.map(|(_, x)| x.0))
    }

    /// Backward-compatible state recovery entry point for existing callers.
    pub fn recover_state_with_dao(&self) -> Result<Option<BTreeMap<String, Account>>, String> {
        self.recover_state()
    }

    /// Recover the latest durable state snapshot together with the exact
    /// canonical height it belongs to. The height is intentionally exposed
    /// so startup can reject cross-journal combinations that were never one
    /// committed block state.
    pub fn recover_state_with_dao_at_height(
        &self,
    ) -> Result<Option<(u64, DaoStateSnapshot)>, String> {
        let Some(p) = &self.state_journal else {
            return Ok(None);
        };
        if !p.exists() {
            return Ok(None);
        }
        let f = File::open(p).map_err(|e| e.to_string())?;
        let mut latest = None;
        let mut previous_height = None;
        for line in BufReader::new(f).lines() {
            let raw = line.map_err(|e| e.to_string())?;
            if raw.trim().is_empty() {
                continue;
            }
            let b = hex::decode(raw.trim()).map_err(|e| e.to_string())?;
            let mut q = 0usize;
            let h = u64::from_be_bytes(fixed::<8>(&b, &mut q)?);
            let n = u32::from_be_bytes(fixed::<4>(&b, &mut q)?) as usize;
            let mut map = BTreeMap::new();
            for _ in 0..n {
                let k = String::from_utf8(get(&b, &mut q)?.to_vec())
                    .map_err(|_| "invalid state key")?;
                let balance = u128::from_be_bytes(fixed::<16>(&b, &mut q)?);
                let staked = u128::from_be_bytes(fixed::<16>(&b, &mut q)?);
                let nonce = u64::from_be_bytes(fixed::<8>(&b, &mut q)?);
                map.insert(
                    k,
                    Account {
                        balance,
                        staked,
                        nonce,
                    },
                );
            }
            let dao = if q < b.len() {
                get(&b, &mut q)?.to_vec()
            } else {
                Vec::new()
            };
            if q != b.len() {
                return Err("trailing state bytes".into());
            }
            // A state record is only committed once its canonical block record
            // exists. This also detects a crash after state/issuance writes but
            // before the block journal reached its durable commit point.
            self.block(h)
                .ok_or("state snapshot references missing block")?;
            if let Some(prev) = previous_height {
                if h < prev {
                    return Err("state journal height regression".into());
                }
                if h == prev {
                    return Err("conflicting state snapshot at same height".into());
                }
            }
            previous_height = Some(h);
            latest = Some((h, (map, dao)));
        }
        Ok(latest)
    }

    /// Persist the complete active validator set as a deterministic snapshot.
    /// Persist the complete validator identity set. Every active validator must
    /// have a registered Ed25519 public key before the snapshot is durable.
    pub fn commit_validators(
        &self,
        height: u64,
        validators: &BTreeMap<String, u64>,
        validator_keys: &BTreeMap<String, [u8; 32]>,
    ) -> Result<(), String> {
        if validators.len() != validator_keys.len()
            || validators
                .keys()
                .any(|address| !validator_keys.contains_key(address))
        {
            return Err(
                "validator snapshot is incomplete: every validator needs a public key".into(),
            );
        }
        let Some(p) = &self.validator_journal else {
            return Ok(());
        };
        let mut o = Vec::from(VALIDATOR_MAGIC);
        o.extend_from_slice(&height.to_be_bytes());
        o.extend_from_slice(&(validators.len() as u32).to_be_bytes());
        for (address, stake) in validators {
            let public_key = validator_keys
                .get(address)
                .ok_or("validator snapshot is missing a public key")?;
            put(&mut o, address.as_bytes());
            o.extend_from_slice(&stake.to_be_bytes());
            o.extend_from_slice(public_key);
        }
        let line = format!("{}\n", hex::encode(o));
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(p)
            .map_err(|e| e.to_string())?;
        f.write_all(line.as_bytes()).map_err(|e| e.to_string())?;
        f.sync_data().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn recover_validators(&self) -> Result<Option<ValidatorSnapshot>, String> {
        let Some(p) = &self.validator_journal else {
            return Ok(None);
        };
        if !p.exists() {
            return Ok(None);
        }
        let f = File::open(p).map_err(|e| e.to_string())?;
        let mut latest = None;
        for (line_no, line) in BufReader::new(f).lines().enumerate() {
            let raw = line.map_err(|e| e.to_string())?;
            if raw.trim().is_empty() {
                continue;
            }
            let b = hex::decode(raw.trim())
                .map_err(|e| format!("validator journal line {}: invalid hex: {e}", line_no + 1))?;
            if b.starts_with(LEGACY_VALIDATOR_MAGIC) {
                return Err(format!(
                    "validator journal line {}: legacy validator snapshot has no public keys;                      migration is required before restart",
                    line_no + 1
                ));
            }
            if !b.starts_with(VALIDATOR_MAGIC) {
                return Err(format!(
                    "validator journal line {}: invalid magic",
                    line_no + 1
                ));
            }
            let mut q = VALIDATOR_MAGIC.len();
            let h = u64::from_be_bytes(fixed::<8>(&b, &mut q)?);
            let n = u32::from_be_bytes(fixed::<4>(&b, &mut q)?) as usize;
            let mut validators = BTreeMap::new();
            for _ in 0..n {
                let address = String::from_utf8(get(&b, &mut q)?.to_vec())
                    .map_err(|_| "invalid validator address")?;
                let stake = u64::from_be_bytes(fixed::<8>(&b, &mut q)?);
                let public_key = fixed::<32>(&b, &mut q)?;
                ed25519_dalek::VerifyingKey::from_bytes(&public_key)
                    .map_err(|_| "invalid validator public key")?;
                if address.is_empty() || stake == 0 {
                    return Err("invalid validator record".into());
                }
                if validators.insert(address, (stake, public_key)).is_some() {
                    return Err("duplicate validator record".into());
                }
            }
            if q != b.len() {
                return Err("trailing validator bytes".into());
            }
            latest = Some((h, validators));
        }
        Ok(latest)
    }

    /// Recover every durable validator snapshot, preserving historical
    /// height/epoch boundaries rather than only the latest mutable set.
    pub fn recover_validator_snapshots(
        &self,
    ) -> Result<BTreeMap<u64, crate::consensus::SealedValidatorSnapshot>, String> {
        let Some(p) = &self.validator_journal else {
            return Ok(BTreeMap::new());
        };
        if !p.exists() {
            return Ok(BTreeMap::new());
        }
        let f = File::open(p).map_err(|e| e.to_string())?;
        let mut snapshots: BTreeMap<u64, crate::consensus::SealedValidatorSnapshot> =
            BTreeMap::new();
        for (line_no, line) in BufReader::new(f).lines().enumerate() {
            let raw = line.map_err(|e| e.to_string())?;
            if raw.trim().is_empty() {
                continue;
            }
            let b = hex::decode(raw.trim())
                .map_err(|e| format!("validator journal line {}: invalid hex: {e}", line_no + 1))?;
            if b.starts_with(LEGACY_VALIDATOR_MAGIC) {
                return Err(format!("validator journal line {}: legacy validator snapshot has no public keys; migration is required before restart", line_no + 1));
            }
            if !b.starts_with(VALIDATOR_MAGIC) {
                return Err(format!(
                    "validator journal line {}: invalid magic",
                    line_no + 1
                ));
            }
            let mut q = VALIDATOR_MAGIC.len();
            let h = u64::from_be_bytes(fixed::<8>(&b, &mut q)?);
            let n = u32::from_be_bytes(fixed::<4>(&b, &mut q)?) as usize;
            let mut validators = BTreeMap::new();
            let mut keys = BTreeMap::new();
            for _ in 0..n {
                let address = String::from_utf8(get(&b, &mut q)?.to_vec())
                    .map_err(|_| "invalid validator address")?;
                let stake = u64::from_be_bytes(fixed::<8>(&b, &mut q)?);
                let public_key = fixed::<32>(&b, &mut q)?;
                ed25519_dalek::VerifyingKey::from_bytes(&public_key)
                    .map_err(|_| "invalid validator public key")?;
                if address.is_empty() || stake == 0 {
                    return Err("invalid validator record".into());
                }
                if validators.insert(address.clone(), stake).is_some() {
                    return Err("duplicate validator record".into());
                }
                keys.insert(address, public_key);
            }
            if q != b.len() {
                return Err("trailing validator bytes".into());
            }
            // Multiple validator mutations can intentionally target the same
            // next activation height before that block is committed. The journal is
            // append-only, so the latest complete snapshot at that height is the
            // durable revision. Revisions at an older height are never allowed.
            if let Some((&latest_height, _)) = snapshots.last_key_value() {
                if h < latest_height {
                    return Err("validator snapshot height regressed".into());
                }
            }
            snapshots.insert(h, (validators, keys));
        }
        Ok(snapshots)
    }

    pub fn validator_snapshot_round_trip_for_restart(
        &self,
        height: u64,
        validators: &BTreeMap<String, u64>,
        validator_keys: &BTreeMap<String, [u8; 32]>,
    ) -> Result<Option<ValidatorSnapshot>, String> {
        self.commit_validators(height, validators, validator_keys)?;
        self.recover_validators()
    }

    pub fn commit_finalized(&self, height: u64, block: [u8; 32]) -> Result<(), String> {
        let canonical = self
            .block(height)
            .ok_or("finality marker references missing block")?;
        if canonical.id != block {
            return Err("finality marker does not match canonical block".into());
        }
        if let Some((previous_height, previous_block)) = self.recover_finalized()? {
            if height < previous_height {
                return Err("finalized height regression".into());
            }
            if height == previous_height {
                if block == previous_block {
                    return Ok(());
                }
                return Err("conflicting finalized block at same height".into());
            }
        }
        let Some(p) = &self.finality_journal else {
            return Ok(());
        };
        let mut o = Vec::from(FINALITY_MAGIC);
        o.extend_from_slice(&height.to_be_bytes());
        o.extend_from_slice(&block);
        let line = format!("{}\n", hex::encode(o));
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(p)
            .map_err(|e| e.to_string())?;
        f.write_all(line.as_bytes()).map_err(|e| e.to_string())?;
        f.sync_data().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn recover_finalized(&self) -> Result<Option<(u64, [u8; 32])>, String> {
        self.recover_finalized_inner(true)
    }

    fn recover_finalized_inner(
        &self,
        require_canonical: bool,
    ) -> Result<Option<(u64, [u8; 32])>, String> {
        let Some(p) = &self.finality_journal else {
            return Ok(None);
        };
        if !p.exists() {
            return Ok(None);
        }
        let f = File::open(p).map_err(|e| e.to_string())?;
        let mut latest = None;
        for (line_no, line) in BufReader::new(f).lines().enumerate() {
            let raw = line.map_err(|e| e.to_string())?;
            if raw.trim().is_empty() {
                continue;
            }
            let b = hex::decode(raw.trim())
                .map_err(|e| format!("finality journal line {}: invalid hex: {e}", line_no + 1))?;
            if !b.starts_with(FINALITY_MAGIC) {
                return Err(format!(
                    "finality journal line {}: invalid magic",
                    line_no + 1
                ));
            }
            let mut q = FINALITY_MAGIC.len();
            let h = u64::from_be_bytes(
                fixed::<8>(&b, &mut q)
                    .map_err(|e| format!("finality journal line {}: {e}", line_no + 1))?,
            );
            let id = fixed::<32>(&b, &mut q)
                .map_err(|e| format!("finality journal line {}: {e}", line_no + 1))?;
            if q != b.len() {
                return Err("trailing finality bytes".into());
            }
            if let Some((prev, previous_id)) = latest {
                if h < prev {
                    return Err("finalized height regression".into());
                }
                if h == prev && id != previous_id {
                    return Err("conflicting finalized block at same height".into());
                }
            }
            if require_canonical {
                let canonical = self
                    .block(h)
                    .ok_or("finality marker references missing block")?;
                if canonical.id != id {
                    return Err("finality marker does not match canonical block".into());
                }
            }
            latest = Some((h, id));
        }
        Ok(latest)
    }

    pub fn commit_slashing(
        &self,
        height: u64,
        validator: &str,
        evidence_id: [u8; 32],
        penalty: u64,
    ) -> Result<(), String> {
        let Some(p) = &self.slashing_journal else {
            return Ok(());
        };
        let mut o = Vec::from(SLASH_MAGIC);
        o.extend_from_slice(&height.to_be_bytes());
        put(&mut o, validator.as_bytes());
        o.extend_from_slice(&evidence_id);
        o.extend_from_slice(&penalty.to_be_bytes());
        let line = format!("{}\n", hex::encode(o));
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(p)
            .map_err(|e| e.to_string())?;
        f.write_all(line.as_bytes()).map_err(|e| e.to_string())?;
        f.sync_data().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn recover_slashing(&self) -> Result<Vec<SlashingRecord>, String> {
        let Some(p) = &self.slashing_journal else {
            return Ok(Vec::new());
        };
        if !p.exists() {
            return Ok(Vec::new());
        }
        let f = File::open(p).map_err(|e| e.to_string())?;
        let mut out = Vec::new();
        for (line_no, line) in BufReader::new(f).lines().enumerate() {
            let raw = line.map_err(|e| e.to_string())?;
            if raw.trim().is_empty() {
                continue;
            }
            let b = hex::decode(raw.trim())
                .map_err(|e| format!("slashing journal line {}: invalid hex: {e}", line_no + 1))?;
            if !b.starts_with(SLASH_MAGIC) {
                return Err(format!(
                    "slashing journal line {}: invalid magic",
                    line_no + 1
                ));
            }
            let mut q = SLASH_MAGIC.len();
            let h = u64::from_be_bytes(fixed::<8>(&b, &mut q)?);
            let validator = String::from_utf8(get(&b, &mut q)?.to_vec())
                .map_err(|_| "invalid slashing validator")?;
            let evidence_id = fixed::<32>(&b, &mut q)?;
            let penalty = u64::from_be_bytes(fixed::<8>(&b, &mut q)?);
            if validator.is_empty() || penalty == 0 || q != b.len() {
                return Err("invalid slashing record".into());
            }
            out.push((h, validator, evidence_id, penalty));
        }
        Ok(out)
    }

    fn append_issuance_record(&self, height: u64, issued_base_units: u128) -> Result<(), String> {
        let Some(p) = &self.issuance_journal else {
            return Ok(());
        };
        if issued_base_units > crate::economics::MAX_SUPPLY {
            return Err("issued supply cap exceeded".into());
        }
        let mut o = Vec::from(ISSUANCE_MAGIC);
        o.extend_from_slice(&height.to_be_bytes());
        o.extend_from_slice(&issued_base_units.to_be_bytes());
        let line = format!("{}\n", hex::encode(o));
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(p)
            .map_err(|e| e.to_string())?;
        f.write_all(line.as_bytes()).map_err(|e| e.to_string())?;
        f.sync_data().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn commit_issuance(&self, height: u64, issued_base_units: u128) -> Result<(), String> {
        self.block(height)
            .ok_or("issuance record references missing block")?;
        self.append_issuance_record(height, issued_base_units)
    }

    pub fn recover_issuance(&self) -> Result<Option<(u64, u128)>, String> {
        self.recover_issuance_inner(true)
    }

    fn recover_issuance_inner(
        &self,
        require_canonical: bool,
    ) -> Result<Option<(u64, u128)>, String> {
        let Some(p) = &self.issuance_journal else {
            return Ok(None);
        };
        if !p.exists() {
            return Ok(None);
        }
        let f = File::open(p).map_err(|e| e.to_string())?;
        let mut latest = None;
        let mut previous_height = None;
        for line in BufReader::new(f).lines() {
            let raw = line.map_err(|e| e.to_string())?;
            if raw.trim().is_empty() {
                continue;
            }
            let b = hex::decode(raw.trim()).map_err(|e| e.to_string())?;
            if !b.starts_with(ISSUANCE_MAGIC) {
                return Err("invalid issuance magic".into());
            }
            let mut q = ISSUANCE_MAGIC.len();
            let h = u64::from_be_bytes(
                fixed::<8>(&b, &mut q).map_err(|_| "invalid issuance record".to_string())?,
            );
            let issued = u128::from_be_bytes(
                fixed::<16>(&b, &mut q).map_err(|_| "invalid issuance record".to_string())?,
            );
            if issued > crate::economics::MAX_SUPPLY || q != b.len() {
                return Err("invalid issuance record".into());
            }
            if let Some(prev) = previous_height {
                if h < prev {
                    return Err("issuance height regression".into());
                }
                if h == prev {
                    return Err("conflicting issuance record at same height".into());
                }
            }
            if require_canonical && self.block(h).is_none() {
                return Err("issuance record references missing block".into());
            }
            previous_height = Some(h);
            latest = Some((h, issued));
        }
        Ok(latest)
    }

    pub fn find_block_by_id(&self, id: [u8; 32]) -> Option<Block> {
        self.blocks
            .read()
            .unwrap()
            .values()
            .find(|b| b.id == id)
            .cloned()
    }

    pub fn block(&self, h: u64) -> Option<Block> {
        self.blocks.read().unwrap().get(&h).cloned()
    }

    pub fn state_root(&self, h: u64) -> Option<[u8; 32]> {
        self.state_roots.read().unwrap().get(&h).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    fn test_nonce() -> u64 {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        NEXT.fetch_add(1, Ordering::Relaxed)
    }

    #[test]
    fn validator_snapshot_persists_public_keys_across_restart() {
        let path = std::env::temp_dir().join(format!(
            "atc-validator-snapshot-{}-{}",
            std::process::id(),
            test_nonce()
        ));
        let journal = path.with_extension("journal");

        let key = ed25519_dalek::SigningKey::from_bytes(&[7u8; 32])
            .verifying_key()
            .to_bytes();
        let mut validators = BTreeMap::new();
        validators.insert("validator-a".to_string(), 100u64);
        let mut keys = BTreeMap::new();
        keys.insert("validator-a".to_string(), key);

        {
            let storage = ChainStorage::open(&journal).unwrap();
            storage.commit_validators(42, &validators, &keys).unwrap();
        }

        {
            let storage = ChainStorage::open(&journal).unwrap();
            let (height, recovered) = storage.recover_validators().unwrap().unwrap();
            assert_eq!(height, 42);
            assert_eq!(recovered.get("validator-a"), Some(&(100, key)));
        }

        let _ = std::fs::remove_file(journal.with_extension("validators"));
        let _ = std::fs::remove_file(&journal);
        let _ = std::fs::remove_file(journal.with_extension("state"));
        let _ = std::fs::remove_file(journal.with_extension("finality"));
        let _ = std::fs::remove_file(journal.with_extension("slashing"));
        let _ = std::fs::remove_file(journal.with_extension("issuance"));
    }

    #[test]
    fn validator_snapshot_history_survives_restart_without_mixing_heights() {
        let path = std::env::temp_dir().join(format!(
            "atc-validator-history-{}-{}",
            std::process::id(),
            test_nonce()
        ));
        let key0 = ed25519_dalek::SigningKey::from_bytes(&[41u8; 32])
            .verifying_key()
            .to_bytes();
        let key1 = ed25519_dalek::SigningKey::from_bytes(&[42u8; 32])
            .verifying_key()
            .to_bytes();
        let mut v0 = BTreeMap::new();
        v0.insert("alice".to_string(), 100u64);
        let mut k0 = BTreeMap::new();
        k0.insert("alice".to_string(), key0);
        let mut v1 = BTreeMap::new();
        v1.insert("alice".to_string(), 60u64);
        v1.insert("bob".to_string(), 40u64);
        let mut k1 = BTreeMap::new();
        k1.insert("alice".to_string(), key1);
        k1.insert(
            "bob".to_string(),
            ed25519_dalek::SigningKey::from_bytes(&[43u8; 32])
                .verifying_key()
                .to_bytes(),
        );

        {
            let storage = ChainStorage::open(&path).unwrap();
            storage.commit_validators(0, &v0, &k0).unwrap();
            storage.commit_validators(10, &v1, &k1).unwrap();
        }
        let recovered = ChainStorage::open(&path)
            .unwrap()
            .recover_validator_snapshots()
            .unwrap();
        assert_eq!(recovered.get(&0).unwrap().1.get("alice"), Some(&key0));
        assert_eq!(recovered.get(&10).unwrap().1.get("alice"), Some(&key1));
        assert_eq!(recovered.get(&0).unwrap().0.get("bob"), None);
        assert_eq!(recovered.get(&10).unwrap().0.get("bob"), Some(&40));

        for suffix in [
            "",
            ".state",
            ".validators",
            ".finality",
            ".slashing",
            ".issuance",
        ] {
            let target = if suffix.is_empty() {
                path.clone()
            } else {
                std::path::PathBuf::from(format!("{}{}", path.display(), suffix))
            };
            let _ = std::fs::remove_file(target);
        }
    }

    #[test]
    fn validator_snapshot_same_activation_height_uses_latest_revision() {
        let path = std::env::temp_dir().join(format!(
            "atc-validator-revision-{}-{}",
            std::process::id(),
            test_nonce()
        ));
        let key_a = ed25519_dalek::SigningKey::from_bytes(&[51u8; 32])
            .verifying_key()
            .to_bytes();
        let key_b = ed25519_dalek::SigningKey::from_bytes(&[52u8; 32])
            .verifying_key()
            .to_bytes();

        let mut first = BTreeMap::new();
        first.insert("alice".to_string(), 100u64);
        let mut first_keys = BTreeMap::new();
        first_keys.insert("alice".to_string(), key_a);

        let mut second = first.clone();
        second.insert("bob".to_string(), 50u64);
        let mut second_keys = first_keys.clone();
        second_keys.insert("bob".to_string(), key_b);

        {
            let storage = ChainStorage::open(&path).unwrap();
            storage.commit_validators(10, &first, &first_keys).unwrap();
            storage
                .commit_validators(10, &second, &second_keys)
                .unwrap();
        }

        let storage = ChainStorage::open(&path).unwrap();
        let recovered = storage.recover_validator_snapshots().unwrap();
        let (validators, keys) = recovered.get(&10).unwrap();
        assert_eq!(validators.get("alice"), Some(&100));
        assert_eq!(validators.get("bob"), Some(&50));
        assert_eq!(keys.get("bob"), Some(&key_b));

        for suffix in [
            "",
            ".state",
            ".validators",
            ".finality",
            ".slashing",
            ".issuance",
        ] {
            let target = if suffix.is_empty() {
                path.clone()
            } else {
                std::path::PathBuf::from(format!("{}{}", path.display(), suffix))
            };
            let _ = std::fs::remove_file(target);
        }
    }

    #[test]
    fn validator_snapshot_history_rejects_height_regression() {
        let path = std::env::temp_dir().join(format!(
            "atc-validator-regression-{}-{}",
            std::process::id(),
            test_nonce()
        ));
        let key = ed25519_dalek::SigningKey::from_bytes(&[53u8; 32])
            .verifying_key()
            .to_bytes();
        let mut validators = BTreeMap::new();
        validators.insert("alice".to_string(), 100u64);
        let mut keys = BTreeMap::new();
        keys.insert("alice".to_string(), key);

        {
            let storage = ChainStorage::open(&path).unwrap();
            storage.commit_validators(10, &validators, &keys).unwrap();
            storage.commit_validators(9, &validators, &keys).unwrap();
        }

        let err = match ChainStorage::open(&path) {
            Ok(_) => panic!("validator height regression must be rejected"),
            Err(err) => err,
        };
        assert!(err.contains("validator snapshot height regressed"));

        for suffix in [
            "",
            ".state",
            ".validators",
            ".finality",
            ".slashing",
            ".issuance",
        ] {
            let target = if suffix.is_empty() {
                path.clone()
            } else {
                std::path::PathBuf::from(format!("{}{}", path.display(), suffix))
            };
            let _ = std::fs::remove_file(target);
        }
    }

    #[test]
    fn incomplete_validator_snapshot_is_rejected() {
        let storage = ChainStorage::new();
        let mut validators = BTreeMap::new();
        validators.insert("validator-a".to_string(), 100u64);
        let keys = BTreeMap::new();
        let err = storage
            .commit_validators(1, &validators, &keys)
            .unwrap_err();
        assert!(err.contains("every validator needs a public key"));
    }

    #[test]
    fn finality_journal_is_idempotent_and_rejects_conflicting_height() {
        let path = std::env::temp_dir().join(format!(
            "atc-finality-idempotence-{}-{}",
            std::process::id(),
            test_nonce()
        ));
        let storage = ChainStorage::open(&path).unwrap();
        let genesis = Block::new(
            0,
            [0; 32],
            "v".into(),
            1,
            Vec::new(),
            [1; 32],
            [2; 32],
            [0; 64],
        );
        let block = Block::new(
            1,
            genesis.id,
            "v".into(),
            2,
            Vec::new(),
            [1; 32],
            [2; 32],
            [0; 64],
        );
        storage.commit(genesis).unwrap();
        storage.commit(block.clone()).unwrap();
        storage.commit_finalized(1, block.id).unwrap();
        storage.commit_finalized(1, block.id).unwrap();
        assert!(storage.commit_finalized(1, [8u8; 32]).is_err());
        assert_eq!(storage.recover_finalized().unwrap(), Some((1, block.id)));

        let raw = std::fs::read_to_string(path.with_extension("finality")).unwrap();
        assert_eq!(raw.lines().count(), 1);

        for suffix in [
            "",
            ".state",
            ".validators",
            ".finality",
            ".slashing",
            ".issuance",
        ] {
            let target = if suffix.is_empty() {
                path.clone()
            } else {
                std::path::PathBuf::from(format!("{}{}", path.display(), suffix))
            };
            let _ = std::fs::remove_file(target);
        }
    }

    #[test]
    fn canonical_commit_rejects_conflicting_height_and_parent() {
        let s = ChainStorage::new();
        let genesis = Block::new(
            0,
            [0; 32],
            "v".into(),
            1,
            Vec::new(),
            [1; 32],
            [2; 32],
            [0; 64],
        );
        s.commit(genesis.clone()).unwrap();

        let h1 = Block::new(
            1,
            genesis.id,
            "v".into(),
            2,
            Vec::new(),
            [3; 32],
            [4; 32],
            [0; 64],
        );
        s.commit(h1.clone()).unwrap();
        assert!(s.commit(h1.clone()).is_ok());
        let conflicting = Block::new(
            1,
            genesis.id,
            "v".into(),
            3,
            Vec::new(),
            [5; 32],
            [6; 32],
            [0; 64],
        );
        assert!(s.commit(conflicting).is_err());
        let wrong_parent = Block::new(
            2,
            [9; 32],
            "v".into(),
            4,
            Vec::new(),
            [7; 32],
            [8; 32],
            [0; 64],
        );
        assert!(s.commit(wrong_parent).is_err());
    }

    #[test]
    fn recovery_rejects_conflicting_canonical_height_in_journal() {
        let path = std::env::temp_dir().join(format!(
            "atc-storage-conflicting-height-{}-{}",
            std::process::id(),
            test_nonce()
        ));
        let genesis = Block::new(
            0,
            [0; 32],
            "v".into(),
            1,
            Vec::new(),
            [1; 32],
            [2; 32],
            [0; 64],
        );
        let first = Block::new(
            1,
            genesis.id,
            "v".into(),
            2,
            Vec::new(),
            [3; 32],
            [4; 32],
            [0; 64],
        );
        let conflicting = Block::new(
            1,
            genesis.id,
            "v".into(),
            3,
            Vec::new(),
            [5; 32],
            [6; 32],
            [0; 64],
        );
        let raw = format!(
            "{}\n{}\n{}\n",
            hex::encode(block_encode(&genesis)),
            hex::encode(block_encode(&first)),
            hex::encode(block_encode(&conflicting)),
        );
        std::fs::write(&path, raw).unwrap();

        let err = match ChainStorage::open(&path) {
            Ok(_) => panic!("conflicting journal must be rejected"),
            Err(err) => err,
        };
        assert!(err.contains("non-sequential block height"));

        for suffix in [
            "",
            ".state",
            ".validators",
            ".finality",
            ".slashing",
            ".issuance",
        ] {
            let target = if suffix.is_empty() {
                path.clone()
            } else {
                std::path::PathBuf::from(format!("{}{}", path.display(), suffix))
            };
            let _ = std::fs::remove_file(target);
        }
    }

    #[test]
    fn recovery_rejects_state_or_issuance_beyond_canonical_tip() {
        let path = std::env::temp_dir().join(format!(
            "atc-storage-state-boundary-{}-{}",
            std::process::id(),
            test_nonce()
        ));
        let genesis = Block::new(
            0,
            [0; 32],
            "v".into(),
            1,
            Vec::new(),
            [1; 32],
            [2; 32],
            [0; 64],
        );
        std::fs::write(&path, format!("{}\n", hex::encode(block_encode(&genesis)))).unwrap();

        let state_path = path.with_extension("state");
        let mut state_record = Vec::new();
        state_record.extend_from_slice(&1u64.to_be_bytes());
        state_record.extend_from_slice(&0u32.to_be_bytes());
        put(&mut state_record, &[]);
        std::fs::write(&state_path, format!("{}\n", hex::encode(state_record))).unwrap();
        let recovered = ChainStorage::open(&path).unwrap();
        assert!(recovered.recover_state_with_dao().is_err());

        let issuance_path = path.with_extension("issuance");
        let mut issuance_record = Vec::new();
        issuance_record.extend_from_slice(ISSUANCE_MAGIC);
        issuance_record.extend_from_slice(&1u64.to_be_bytes());
        issuance_record.extend_from_slice(&0u128.to_be_bytes());
        std::fs::write(
            &issuance_path,
            format!("{}\n", hex::encode(issuance_record)),
        )
        .unwrap();
        assert!(ChainStorage::open(&path)
            .unwrap()
            .recover_issuance()
            .is_err());

        for suffix in [
            "",
            ".state",
            ".validators",
            ".finality",
            ".slashing",
            ".issuance",
        ] {
            let target = if suffix.is_empty() {
                path.clone()
            } else {
                std::path::PathBuf::from(format!("{}{}", path.display(), suffix))
            };
            let _ = std::fs::remove_file(target);
        }
    }

    #[test]
    fn recovery_rejects_torn_canonical_block_record() {
        let path = std::env::temp_dir().join(format!(
            "atc-storage-torn-block-{}-{}",
            std::process::id(),
            test_nonce()
        ));
        let genesis = Block::new(
            0,
            [0; 32],
            "v".into(),
            1,
            Vec::new(),
            [1; 32],
            [2; 32],
            [0; 64],
        );
        let encoded = hex::encode(block_encode(&genesis));
        std::fs::write(
            &path,
            format!("{}\n{}", &encoded[..encoded.len() - 2], encoded),
        )
        .unwrap();
        let err = match ChainStorage::open(&path) {
            Ok(_) => panic!("torn canonical block journal must be rejected"),
            Err(err) => err,
        };
        assert!(err.contains("journal line 1"));

        for suffix in [
            "",
            ".state",
            ".validators",
            ".finality",
            ".slashing",
            ".issuance",
        ] {
            let target = if suffix.is_empty() {
                path.clone()
            } else {
                std::path::PathBuf::from(format!("{}{}", path.display(), suffix))
            };
            let _ = std::fs::remove_file(target);
        }
    }

    #[test]
    fn recovery_rejects_state_written_before_block_commit_point() {
        let path = std::env::temp_dir().join(format!(
            "atc-storage-precommit-state-{}-{}",
            std::process::id(),
            test_nonce()
        ));
        let mut state_record = Vec::new();
        state_record.extend_from_slice(&1u64.to_be_bytes());
        state_record.extend_from_slice(&0u32.to_be_bytes());
        put(&mut state_record, &[]);
        std::fs::write(
            path.with_extension("state"),
            format!("{}\n", hex::encode(state_record)),
        )
        .unwrap();

        let mut storage = ChainStorage::new();
        // The same recovery boundary used by open_storage() must fail closed:
        // state cannot become durable merely because its journal line is valid.
        storage.state_journal = Some(path.with_extension("state"));
        let err = storage.recover_state_with_dao().unwrap_err();
        assert!(err.contains("missing block"));

        for suffix in [
            "",
            ".state",
            ".validators",
            ".finality",
            ".slashing",
            ".issuance",
        ] {
            let target = if suffix.is_empty() {
                path.clone()
            } else {
                std::path::PathBuf::from(format!("{}{}", path.display(), suffix))
            };
            let _ = std::fs::remove_file(target);
        }
    }

    #[test]
    fn recovery_rejects_torn_finality_record() {
        let path = std::env::temp_dir().join(format!(
            "atc-storage-torn-finality-{}-{}",
            std::process::id(),
            test_nonce()
        ));
        let genesis = Block::new(
            0,
            [0; 32],
            "v".into(),
            1,
            Vec::new(),
            [1; 32],
            [2; 32],
            [0; 64],
        );
        let storage = ChainStorage::open(&path).unwrap();
        storage.commit(genesis.clone()).unwrap();
        let mut record = Vec::from(FINALITY_MAGIC);
        record.extend_from_slice(&0u64.to_be_bytes());
        record.extend_from_slice(&genesis.id);
        let encoded = hex::encode(record);
        std::fs::write(
            path.with_extension("finality"),
            format!("{}\n", &encoded[..encoded.len() - 4]),
        )
        .unwrap();
        let err = match ChainStorage::open(&path) {
            Ok(_) => panic!("torn finality journal must be rejected"),
            Err(err) => err,
        };
        assert!(err.contains("finality journal line 1"));

        for suffix in [
            "",
            ".state",
            ".validators",
            ".finality",
            ".slashing",
            ".issuance",
        ] {
            let target = if suffix.is_empty() {
                path.clone()
            } else {
                std::path::PathBuf::from(format!("{}{}", path.display(), suffix))
            };
            let _ = std::fs::remove_file(target);
        }
    }

    #[test]
    fn recovery_rejects_torn_issuance_record() {
        let path = std::env::temp_dir().join(format!(
            "atc-storage-torn-issuance-{}-{}",
            std::process::id(),
            test_nonce()
        ));
        let storage = ChainStorage::open(&path).unwrap();
        let genesis = Block::new(
            0,
            [0; 32],
            "v".into(),
            1,
            Vec::new(),
            [1; 32],
            [2; 32],
            [0; 64],
        );
        storage.commit(genesis).unwrap();
        let mut record = Vec::from(ISSUANCE_MAGIC);
        record.extend_from_slice(&0u64.to_be_bytes());
        record.extend_from_slice(&0u128.to_be_bytes());
        let encoded = hex::encode(record);
        std::fs::write(
            path.with_extension("issuance"),
            format!("{}\n", &encoded[..encoded.len() - 2]),
        )
        .unwrap();
        let err = match ChainStorage::open(&path) {
            Ok(_) => panic!("torn issuance journal must be rejected"),
            Err(err) => err,
        };
        assert!(err.contains("invalid issuance") || err.contains("range end index"));

        for suffix in [
            "",
            ".state",
            ".validators",
            ".finality",
            ".slashing",
            ".issuance",
        ] {
            let target = if suffix.is_empty() {
                path.clone()
            } else {
                std::path::PathBuf::from(format!("{}{}", path.display(), suffix))
            };
            let _ = std::fs::remove_file(target);
        }
    }

    #[test]
    fn round_trip() {
        let s = ChainStorage::new();
        let b = Block::new(
            0,
            [0; 32],
            "v".into(),
            1,
            Vec::new(),
            [1; 32],
            [2; 32],
            [0; 64],
        );
        s.commit(b.clone()).unwrap();
        assert_eq!(s.block(0), Some(b));
    }
}
