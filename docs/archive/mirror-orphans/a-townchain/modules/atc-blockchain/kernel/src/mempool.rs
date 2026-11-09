//! Transactions, mempool and deterministic state transition.
use crate::security::simple_hash;
use std::{collections::BTreeMap, sync::Mutex};

mod signature_serde {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &[u8; 64], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; 64], D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes = Vec::<u8>::deserialize(deserializer)?;
        bytes.try_into().map_err(|_| {
            serde::de::Error::custom("transaction signature must contain exactly 64 bytes")
        })
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TxType {
    Transfer = 0,
    Stake = 1,
    Unstake = 2,
    Contract = 3,
    Validator = 4,
}
impl TxType {
    pub fn base_gas(self) -> u64 {
        match self {
            Self::Transfer => 1000,
            Self::Stake => 1200,
            Self::Unstake => 1200,
            Self::Contract => 5000,
            Self::Validator => 2500,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Transaction {
    pub id: [u8; 32],
    pub chain_id: u64,
    pub tx_type: TxType,
    pub sender_did: String,
    pub recipient_did: Option<String>,
    pub amount: u128,
    pub gas_price: u64,
    pub gas_limit: u64,
    pub nonce: u64,
    pub timestamp: u64,
    pub payload: Vec<u8>,
    #[serde(with = "signature_serde")]
    pub signature: [u8; 64],
    pub public_key: [u8; 32],
    pub poh_hash: [u8; 32],
}
impl Transaction {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        t: TxType,
        s: String,
        r: Option<String>,
        a: u128,
        gp: u64,
        gl: u64,
        n: u64,
        ts: u64,
        p: Vec<u8>,
        sig: [u8; 64],
        poh: [u8; 32],
    ) -> Self {
        Self::new_with_chain_id(0, t, s, r, a, gp, gl, n, ts, p, sig, [0; 32], poh)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn new_signed(
        t: TxType,
        s: String,
        r: Option<String>,
        a: u128,
        gp: u64,
        gl: u64,
        n: u64,
        ts: u64,
        p: Vec<u8>,
        sig: [u8; 64],
        public_key: [u8; 32],
        poh: [u8; 32],
    ) -> Self {
        Self::new_with_chain_id(0, t, s, r, a, gp, gl, n, ts, p, sig, public_key, poh)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn new_with_chain_id(
        chain_id: u64,
        t: TxType,
        s: String,
        r: Option<String>,
        a: u128,
        gp: u64,
        gl: u64,
        n: u64,
        ts: u64,
        p: Vec<u8>,
        sig: [u8; 64],
        public_key: [u8; 32],
        poh: [u8; 32],
    ) -> Self {
        let mut b = Vec::new();
        b.extend_from_slice(b"ATC-TX-ID-V2");
        b.extend_from_slice(&chain_id.to_be_bytes());
        b.push(t as u8);
        b.extend_from_slice(&(s.len() as u32).to_be_bytes());
        b.extend_from_slice(s.as_bytes());
        match &r {
            Some(x) => {
                b.push(1);
                b.extend_from_slice(&(x.len() as u32).to_be_bytes());
                b.extend_from_slice(x.as_bytes())
            }
            None => b.push(0),
        }
        b.extend_from_slice(&a.to_be_bytes());
        b.extend_from_slice(&gp.to_be_bytes());
        b.extend_from_slice(&gl.to_be_bytes());
        b.extend_from_slice(&n.to_be_bytes());
        b.extend_from_slice(&ts.to_be_bytes());
        b.extend_from_slice(&(p.len() as u32).to_be_bytes());
        b.extend_from_slice(&p);
        b.extend_from_slice(&poh);
        Self {
            id: simple_hash(&b),
            chain_id,
            tx_type: t,
            sender_did: s,
            recipient_did: r,
            amount: a,
            gas_price: gp,
            gas_limit: gl,
            nonce: n,
            timestamp: ts,
            payload: p,
            signature: sig,
            public_key,
            poh_hash: poh,
        }
    }
    pub fn gas_cost(&self) -> u64 {
        self.tx_type.base_gas() + self.payload.len() as u64 * 10
    }
    pub fn max_fee(&self) -> u128 {
        u128::from(self.gas_limit).saturating_mul(u128::from(self.gas_price))
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxStatus {
    Pending,
    Validated,
    InBlock,
    Rejected,
    Expired,
    InvalidSignature,
}
#[derive(Clone, Debug)]
pub struct PoolEntry {
    pub tx: Transaction,
    pub status: TxStatus,
    pub added_at: u64,
    pub priority: u128,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MempoolError {
    PoolFull,
    DuplicateTx,
    TxNotFound,
    GasLimitTooLow,
    GasPriceTooLow,
    NoRecipient,
    InvalidNonce { expected: u64, got: u64 },
    InsufficientBalance,
    InsufficientStake,
    Expired,
    InvalidSignature,
    WrongChain,
    InvalidValidatorTransition(String),
}

impl std::fmt::Display for MempoolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PoolFull => write!(f, "mempool is full"),
            Self::DuplicateTx => write!(f, "duplicate transaction"),
            Self::TxNotFound => write!(f, "transaction not found"),
            Self::GasLimitTooLow => write!(f, "gas limit too low"),
            Self::GasPriceTooLow => write!(f, "gas price too low"),
            Self::NoRecipient => write!(f, "recipient required"),
            Self::InvalidNonce { expected, got } => {
                write!(f, "invalid nonce: expected {expected}, got {got}")
            }
            Self::InsufficientBalance => write!(f, "insufficient balance"),
            Self::InsufficientStake => write!(f, "insufficient stake"),
            Self::Expired => write!(f, "transaction expired"),
            Self::InvalidSignature => write!(f, "invalid signature"),
            Self::WrongChain => write!(f, "wrong chain"),
            Self::InvalidValidatorTransition(e) => write!(f, "invalid validator transition: {e}"),
        }
    }
}

impl std::error::Error for MempoolError {}

pub struct MemoryPool {
    entries: Mutex<BTreeMap<[u8; 32], PoolEntry>>,
    max_size: usize,
    max_age: u64,
}
impl MemoryPool {
    pub fn new(m: usize, a: u64) -> Self {
        Self {
            entries: Mutex::new(BTreeMap::new()),
            max_size: m,
            max_age: a,
        }
    }
    pub fn add(&self, tx: Transaction, now: u64) -> Result<(), MempoolError> {
        let mut e = self.entries.lock().unwrap();
        if e.contains_key(&tx.id) {
            return Err(MempoolError::DuplicateTx);
        }
        if e.len() >= self.max_size {
            return Err(MempoolError::PoolFull);
        }
        let id = tx.id;
        e.insert(
            id,
            PoolEntry {
                priority: u128::from(tx.gas_price).saturating_mul(u128::from(tx.gas_limit)),
                tx,
                status: TxStatus::Pending,
                added_at: now,
            },
        );
        Ok(())
    }
    pub fn remove(&self, id: &[u8; 32]) {
        self.entries.lock().unwrap().remove(id);
    }
    pub fn validate_tx(&self, id: &[u8; 32], now: u64) -> Result<(), MempoolError> {
        let mut e = self.entries.lock().unwrap();
        let x = e.get_mut(id).ok_or(MempoolError::TxNotFound)?;
        if now.saturating_sub(x.added_at) > self.max_age {
            x.status = TxStatus::Expired;
            return Err(MempoolError::Expired);
        }
        if x.tx.gas_limit < x.tx.gas_cost() {
            x.status = TxStatus::Rejected;
            return Err(MempoolError::GasLimitTooLow);
        }
        if x.tx.gas_price == 0 {
            x.status = TxStatus::Rejected;
            return Err(MempoolError::GasPriceTooLow);
        }
        if x.tx.tx_type == TxType::Transfer && x.tx.amount > 0 && x.tx.recipient_did.is_none() {
            x.status = TxStatus::Rejected;
            return Err(MempoolError::NoRecipient);
        }
        x.status = TxStatus::Validated;
        Ok(())
    }
    pub fn get_pending_batch(&self, n: usize) -> Vec<Transaction> {
        let e = self.entries.lock().unwrap();
        let mut v: Vec<_> = e
            .values()
            .filter(|x| x.status == TxStatus::Validated)
            .cloned()
            .collect();
        v.sort_by(|a, b| b.priority.cmp(&a.priority).then(a.tx.id.cmp(&b.tx.id)));
        v.into_iter().take(n).map(|x| x.tx).collect()
    }
    pub fn mark_in_block(&self, id: &[u8; 32]) {
        if let Some(x) = self.entries.lock().unwrap().get_mut(id) {
            x.status = TxStatus::InBlock
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Account {
    pub balance: u128,
    pub staked: u128,
    pub nonce: u64,
}
pub struct StateDb {
    accounts: Mutex<BTreeMap<String, Account>>,
    dao: Mutex<crate::dao_state::DaoState>,
    genesis_sealed: Mutex<bool>,
    issued_base_units: Mutex<u128>,
    validator_state: Mutex<crate::validator_state::ValidatorState>,
}
impl Default for StateDb {
    fn default() -> Self {
        Self::new()
    }
}

impl StateDb {
    pub fn new() -> Self {
        Self {
            accounts: Mutex::new(BTreeMap::new()),
            dao: Mutex::new(
                crate::dao_state::DaoState::new(1, 5000).expect("valid default DAO config"),
            ),
            genesis_sealed: Mutex::new(false),
            issued_base_units: Mutex::new(0),
            validator_state: Mutex::new(crate::validator_state::ValidatorState::new()),
        }
    }
    pub fn genesis_credit(&self, id: &str, n: u64) -> Result<(), String> {
        let n = u128::from(n)
            .checked_mul(crate::economics::ATC_BASE_UNITS)
            .ok_or("genesis allocation overflow".to_string())?;
        if *self.genesis_sealed.lock().unwrap() {
            return Err("genesis allocation is sealed".into());
        }
        let mut a = self.accounts.lock().unwrap();
        let current = a
            .values()
            .try_fold(0u128, |acc, x| {
                acc.checked_add(x.balance)
                    .and_then(|v| v.checked_add(x.staked))
            })
            .ok_or("supply overflow".to_string())?;
        let new_supply = current
            .checked_add(n)
            .ok_or("supply overflow".to_string())?;
        if new_supply > crate::economics::MAX_SUPPLY {
            return Err(format!(
                "ATC supply cap exceeded in base units: {new_supply} > {}",
                crate::economics::MAX_SUPPLY
            ));
        }
        let x = a.entry(id.into()).or_insert(Account {
            balance: 0,
            staked: 0,
            nonce: 0,
        });
        *self.issued_base_units.lock().unwrap() = new_supply;
        x.balance = x
            .balance
            .checked_add(n)
            .ok_or("balance overflow".to_string())?;
        Ok(())
    }
    pub fn restore_issued_base_units(&self, issued: u128) -> Result<(), String> {
        if issued > crate::economics::MAX_SUPPLY {
            return Err("issued supply cap exceeded".into());
        }
        *self.issued_base_units.lock().unwrap() = issued;
        Ok(())
    }

    pub fn issued_base_units(&self) -> u128 {
        *self.issued_base_units.lock().unwrap()
    }

    pub fn apply_block_reward(&self, height: u64, recipient: &str) -> Result<u128, String> {
        let reward = crate::economics::block_reward_base_units(height, self.issued_base_units());
        if reward == 0 {
            return Ok(0);
        }
        let mut accounts = self.accounts.lock().unwrap();
        let mut issued = self.issued_base_units.lock().unwrap();
        let new_issued = (*issued)
            .checked_add(reward)
            .ok_or("issued supply overflow")?;
        if new_issued > crate::economics::MAX_SUPPLY {
            return Err("issued supply cap exceeded".into());
        }
        let x = accounts.entry(recipient.to_owned()).or_insert(Account {
            balance: 0,
            staked: 0,
            nonce: 0,
        });
        x.balance = x
            .balance
            .checked_add(reward)
            .ok_or("reward balance overflow")?;
        *issued = new_issued;
        Ok(reward)
    }

    pub fn seal_genesis(&self) {
        *self.genesis_sealed.lock().unwrap() = true
    }
    pub fn total_supply_base_units(&self) -> u128 {
        self.accounts
            .lock()
            .unwrap()
            .values()
            .try_fold(0u128, |acc, x| {
                acc.checked_add(x.balance)?.checked_add(x.staked)
            })
            .unwrap_or(u128::MAX)
    }
    pub fn total_supply(&self) -> u64 {
        (self.total_supply_base_units() / crate::economics::ATC_BASE_UNITS).min(u64::MAX as u128)
            as u64
    }
    pub fn balance_base_units(&self, id: &str) -> u128 {
        self.accounts
            .lock()
            .unwrap()
            .get(id)
            .map(|x| x.balance)
            .unwrap_or(0)
    }
    pub fn balance(&self, id: &str) -> u64 {
        (self.balance_base_units(id) / crate::economics::ATC_BASE_UNITS).min(u64::MAX as u128)
            as u64
    }
    pub fn nonce(&self, id: &str) -> u64 {
        self.accounts
            .lock()
            .unwrap()
            .get(id)
            .map(|x| x.nonce)
            .unwrap_or(0)
    }
    pub fn slash_stake(&self, id: &str, amount: u64) -> Result<u64, String> {
        if amount == 0 {
            return Err("slash amount must be non-zero".into());
        }
        let mut a = self.accounts.lock().unwrap();
        let x = a.get_mut(id).ok_or("validator account not found")?;
        let amount = u128::from(amount)
            .checked_mul(crate::economics::ATC_BASE_UNITS)
            .ok_or("slash amount overflow")?;
        let applied = amount.min(x.staked);
        x.staked -= applied;
        Ok((applied / crate::economics::ATC_BASE_UNITS).min(u64::MAX as u128) as u64)
    }

    /// Deterministic commitment of the canonical validator state.
    pub fn validator_root(&self) -> [u8; 32] {
        self.validator_state.lock().unwrap().root()
    }

    pub fn validator_snapshot(&self) -> BTreeMap<String, crate::validator_state::ValidatorRecord> {
        self.validator_state.lock().unwrap().snapshot()
    }

    /// Apply one canonical validator lifecycle transition.
    pub fn apply_validator_transition(
        &self,
        transition: &crate::validator_state::ValidatorTransition,
    ) -> Result<(), String> {
        self.validator_state.lock().unwrap().apply(transition)
    }

    pub fn staked_base_units(&self, id: &str) -> u128 {
        self.accounts
            .lock()
            .unwrap()
            .get(id)
            .map(|x| x.staked)
            .unwrap_or(0)
    }
    pub fn staked(&self, id: &str) -> u64 {
        (self.staked_base_units(id) / crate::economics::ATC_BASE_UNITS).min(u64::MAX as u128) as u64
    }
    pub fn root(&self) -> [u8; 32] {
        let a = self.accounts.lock().unwrap();
        let mut b = Vec::new();
        for (k, v) in a.iter() {
            b.extend_from_slice(&(k.len() as u32).to_be_bytes());
            b.extend_from_slice(k.as_bytes());
            b.extend_from_slice(&v.balance.to_be_bytes());
            b.extend_from_slice(&v.staked.to_be_bytes());
            b.extend_from_slice(&v.nonce.to_be_bytes())
        }
        let account_root = simple_hash(&b);
        let supply = a
            .values()
            .try_fold(0u128, |acc, x| {
                acc.checked_add(x.balance)?.checked_add(x.staked)
            })
            .unwrap_or(u128::MAX);
        let dao_root = self.dao.lock().unwrap().root();
        let mut combined = Vec::from(b"ATC-STATE-V2");
        combined.extend_from_slice(&account_root);
        combined.extend_from_slice(&supply.to_be_bytes());
        combined.extend_from_slice(&self.issued_base_units().to_be_bytes());
        combined.extend_from_slice(&dao_root);
        let validator_snapshot = self.validator_snapshot();
        if !validator_snapshot.is_empty() {
            combined.extend_from_slice(&self.validator_root());
        }
        simple_hash(&combined)
    }
    pub fn apply_dao_payload(
        &self,
        payload: &[u8],
        block: u64,
        sender: &str,
    ) -> Result<(), String> {
        let account_snapshot = self.snapshot();
        let dao_snapshot = self.dao_snapshot();
        let voting_power = self.staked(sender);
        let effect = self
            .dao
            .lock()
            .unwrap()
            .apply(payload, block, sender, voting_power);
        let effect = match effect {
            Ok(e) => e,
            Err(e) => {
                let _ = self.restore_dao(&dao_snapshot);
                return Err(e);
            }
        };
        let result = match effect {
            None => Ok(()),
            Some(crate::dao_state::DaoEffect::TreasuryDeposit { amount }) => {
                let mut a = self.accounts.lock().unwrap();
                let x = a.entry(sender.to_owned()).or_insert(Account {
                    balance: 0,
                    staked: 0,
                    nonce: 0,
                });
                if x.balance < amount {
                    Err("insufficient balance for DAO treasury deposit".into())
                } else {
                    x.balance -= amount;
                    Ok(())
                }
            }
            Some(crate::dao_state::DaoEffect::TreasuryPayout { recipient, amount }) => {
                let mut a = self.accounts.lock().unwrap();
                let x = a.entry(recipient).or_insert(Account {
                    balance: 0,
                    staked: 0,
                    nonce: 0,
                });
                x.balance = x
                    .balance
                    .checked_add(amount)
                    .ok_or("recipient balance overflow".to_string())?;
                Ok(())
            }
        };
        if let Err(e) = result {
            self.restore(account_snapshot);
            let _ = self.restore_dao(&dao_snapshot);
            return Err(e);
        }
        Ok(())
    }
    pub fn dao_snapshot(&self) -> Vec<u8> {
        self.dao.lock().unwrap().encode()
    }
    pub fn restore_dao(&self, bytes: &[u8]) -> Result<(), String> {
        *self.dao.lock().unwrap() = crate::dao_state::DaoState::decode(bytes)?;
        Ok(())
    }
    pub fn apply_batch(&self, txs: &[Transaction]) -> Result<(), MempoolError> {
        let mut a = self.accounts.lock().unwrap();
        let mut staged = a.clone();
        let mut validators = self.validator_state.lock().unwrap();
        let validator_snapshot = validators.snapshot();
        for tx in txs {
            if let Err(e) = Self::apply_to(&mut staged, tx) {
                validators.restore(validator_snapshot.clone());
                return Err(e);
            }
            match tx.tx_type {
                TxType::Stake if validators.get(&tx.sender_did).is_some() => {
                    let transition = crate::validator_state::ValidatorTransition::Stake {
                        address: tx.sender_did.clone(),
                        amount: tx.amount,
                    };
                    if let Err(e) = validators.apply(&transition) {
                        validators.restore(validator_snapshot.clone());
                        return Err(MempoolError::InvalidValidatorTransition(e));
                    }
                }
                TxType::Unstake if validators.get(&tx.sender_did).is_some() => {
                    let transition = crate::validator_state::ValidatorTransition::Unstake {
                        address: tx.sender_did.clone(),
                        amount: tx.amount,
                    };
                    if let Err(e) = validators.apply(&transition) {
                        validators.restore(validator_snapshot.clone());
                        return Err(MempoolError::InvalidValidatorTransition(e));
                    }
                }
                TxType::Validator => {
                    let transition =
                        match crate::validator_state::ValidatorTransition::decode(&tx.payload) {
                            Ok(v) => v,
                            Err(e) => {
                                validators.restore(validator_snapshot.clone());
                                return Err(MempoolError::InvalidValidatorTransition(e));
                            }
                        };
                    match &transition {
                        crate::validator_state::ValidatorTransition::Register {
                            address, ..
                        }
                        | crate::validator_state::ValidatorTransition::RotateKey {
                            address, ..
                        }
                        | crate::validator_state::ValidatorTransition::Unregister {
                            address, ..
                        } if address != &tx.sender_did => {
                            validators.restore(validator_snapshot.clone());
                            return Err(MempoolError::InvalidValidatorTransition(
                                "validator transition sender mismatch".into(),
                            ));
                        }
                        crate::validator_state::ValidatorTransition::Slash { .. } => {
                            validators.restore(validator_snapshot.clone());
                            return Err(MempoolError::InvalidValidatorTransition(
                                "slashing transitions require verified consensus evidence".into(),
                            ));
                        }
                        _ => {}
                    }
                    if let crate::validator_state::ValidatorTransition::Register { stake, .. } =
                        &transition
                    {
                        let account = staged.get(&tx.sender_did).ok_or(
                            MempoolError::InvalidValidatorTransition(
                                "validator account missing".into(),
                            ),
                        )?;
                        if account.staked != *stake {
                            validators.restore(validator_snapshot.clone());
                            return Err(MempoolError::InvalidValidatorTransition(
                                "validator stake does not match account stake".into(),
                            ));
                        }
                    }
                    if let Err(e) = validators.apply(&transition) {
                        validators.restore(validator_snapshot.clone());
                        return Err(MempoolError::InvalidValidatorTransition(e));
                    }
                }
                _ => {}
            }
        }
        *a = staged;
        Ok(())
    }
    pub fn snapshot(&self) -> BTreeMap<String, Account> {
        self.accounts.lock().unwrap().clone()
    }
    pub fn restore(&self, snapshot: BTreeMap<String, Account>) {
        *self.accounts.lock().unwrap() = snapshot
    }
    pub fn apply(&self, tx: &Transaction) -> Result<(), MempoolError> {
        self.apply_batch(std::slice::from_ref(tx))
    }
    fn apply_to(a: &mut BTreeMap<String, Account>, tx: &Transaction) -> Result<(), MempoolError> {
        let s = a.get(&tx.sender_did).cloned().unwrap_or(Account {
            balance: 0,
            staked: 0,
            nonce: 0,
        });
        if s.nonce != tx.nonce {
            return Err(MempoolError::InvalidNonce {
                expected: s.nonce,
                got: tx.nonce,
            });
        }
        let fee = tx.max_fee();
        match tx.tx_type {
            TxType::Transfer | TxType::Contract => {
                // Fees are whole-ATC obligations: a sender's sub-ATC dust
                // remainder must never be consumed by a transfer fee.
                let fee = if tx.tx_type == TxType::Transfer {
                    let remainder = s.balance.saturating_sub(tx.amount);
                    let whole_atc = remainder - remainder % crate::economics::ATC_BASE_UNITS;
                    fee.min(whole_atc)
                } else {
                    fee
                };
                let debit = match tx.tx_type {
                    TxType::Contract => fee,
                    _ => tx
                        .amount
                        .checked_add(fee)
                        .ok_or(MempoolError::InsufficientBalance)?,
                };
                if s.balance
                    < debit.saturating_add(if tx.tx_type == TxType::Contract {
                        tx.amount
                    } else {
                        0
                    })
                {
                    return Err(MempoolError::InsufficientBalance);
                }
                let mut ns = s;
                ns.balance -= debit;
                ns.nonce = ns.nonce.checked_add(1).ok_or(MempoolError::InvalidNonce {
                    expected: u64::MAX,
                    got: tx.nonce,
                })?;
                a.insert(tx.sender_did.clone(), ns);
                if tx.tx_type == TxType::Transfer {
                    let r = tx.recipient_did.as_ref().ok_or(MempoolError::NoRecipient)?;
                    let x = a.entry(r.clone()).or_insert(Account {
                        balance: 0,
                        staked: 0,
                        nonce: 0,
                    });
                    x.balance = x
                        .balance
                        .checked_add(tx.amount)
                        .ok_or(MempoolError::InsufficientBalance)?;
                }
            }
            TxType::Stake => {
                if s.balance
                    < tx.amount
                        .checked_add(fee)
                        .ok_or(MempoolError::InsufficientBalance)?
                {
                    return Err(MempoolError::InsufficientBalance);
                }
                let mut ns = s;
                ns.balance -= tx
                    .amount
                    .checked_add(fee)
                    .ok_or(MempoolError::InsufficientBalance)?;
                ns.staked = ns
                    .staked
                    .checked_add(tx.amount)
                    .ok_or(MempoolError::InsufficientStake)?;
                ns.nonce = ns.nonce.checked_add(1).ok_or(MempoolError::InvalidNonce {
                    expected: u64::MAX,
                    got: tx.nonce,
                })?;
                a.insert(tx.sender_did.clone(), ns);
            }
            TxType::Validator => {
                if s.balance < fee {
                    return Err(MempoolError::InsufficientBalance);
                }
                let mut ns = s;
                ns.balance -= fee;
                ns.nonce = ns.nonce.checked_add(1).ok_or(MempoolError::InvalidNonce {
                    expected: u64::MAX,
                    got: tx.nonce,
                })?;
                a.insert(tx.sender_did.clone(), ns);
            }
            TxType::Unstake => {
                if s.staked < tx.amount || s.balance < fee {
                    return Err(MempoolError::InsufficientStake);
                }
                let mut ns = s;
                ns.staked -= tx.amount;
                ns.balance = ns
                    .balance
                    .checked_add(tx.amount)
                    .and_then(|v| v.checked_sub(fee))
                    .ok_or(MempoolError::InsufficientBalance)?;
                ns.nonce = ns.nonce.checked_add(1).ok_or(MempoolError::InvalidNonce {
                    expected: u64::MAX,
                    got: tx.nonce,
                })?;
                a.insert(tx.sender_did.clone(), ns);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod supply_tests {
    use super::*;
    use crate::economics::MAX_ATC_SUPPLY;

    #[test]
    fn genesis_supply_cannot_exceed_360_million_atc() {
        let state = StateDb::new();
        state.genesis_credit("alice", MAX_ATC_SUPPLY).unwrap();
        assert_eq!(state.total_supply(), MAX_ATC_SUPPLY);
        assert!(state.genesis_credit("bob", 1).is_err());
    }

    #[test]
    fn genesis_allocation_is_sealed_after_genesis() {
        let state = StateDb::new();
        state.genesis_credit("alice", 100).unwrap();
        state.seal_genesis();
        assert!(state.genesis_credit("bob", 1).is_err());
        assert_eq!(state.total_supply(), 100);
    }

    #[test]
    fn block_reward_uses_canonical_policy_and_tracks_base_units() {
        let state = StateDb::new();
        state.genesis_credit("genesis", 1_000_000).unwrap();
        let before = state.issued_base_units();
        assert_eq!(
            state.apply_block_reward(0, "validator"),
            Ok(500 * crate::economics::ATC_BASE_UNITS)
        );
        assert_eq!(state.balance("validator"), 500);
        assert_eq!(
            state.balance_base_units("validator"),
            500 * crate::economics::ATC_BASE_UNITS
        );
        assert_eq!(
            state.issued_base_units(),
            before + 500 * crate::economics::ATC_BASE_UNITS
        );
    }

    #[test]
    fn base_unit_transfer_preserves_sub_atc_dust() {
        let state = StateDb::new();
        state.genesis_credit("alice", 1).unwrap();
        let dust = crate::economics::ATC_BASE_UNITS - 1;
        let tx = Transaction::new_with_chain_id(
            658467,
            TxType::Transfer,
            "alice".into(),
            Some("bob".into()),
            dust,
            1,
            1,
            0,
            1,
            Vec::new(),
            [0; 64],
            [0; 32],
            [0; 32],
        );
        state.apply(&tx).unwrap();
        assert_eq!(state.balance_base_units("bob"), dust);
        assert_eq!(state.balance("bob"), 0);
        assert_eq!(
            state.balance_base_units("alice"),
            crate::economics::ATC_BASE_UNITS - dust
        );
        assert_eq!(
            state.total_supply_base_units(),
            crate::economics::ATC_BASE_UNITS
        );
    }

    #[test]
    fn supply_is_part_of_state_root() {
        let a = StateDb::new();
        let b = StateDb::new();
        a.genesis_credit("alice", 100).unwrap();
        assert_ne!(a.root(), b.root());
    }
}
