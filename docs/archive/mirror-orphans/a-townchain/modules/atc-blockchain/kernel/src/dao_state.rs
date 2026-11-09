//! Deterministic DAO state machine executed as part of the chain state transition.
use crate::security::simple_hash;
use std::collections::BTreeMap;

pub const MAGIC: &[u8] = b"ATC-DAO-V2";
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Status {
    Pending,
    Active,
    Passed,
    Rejected,
    Queued,
    Executed,
    Cancelled,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Proposal {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub proposer: String,
    pub start: u64,
    pub end: u64,
    pub status: Status,
    pub yes: u64,
    pub no: u64,
    pub abstain: u64,
    pub action_recipient: Option<String>,
    pub action_amount: u128,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DaoEffect {
    TreasuryDeposit { amount: u128 },
    TreasuryPayout { recipient: String, amount: u128 },
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DaoState {
    pub proposals: BTreeMap<u64, Proposal>,
    pub votes: BTreeMap<(u64, String), (u8, u64)>,
    pub treasury: u128,
    pub allocations: BTreeMap<String, u64>,
    pub timelocks: BTreeMap<u64, u64>,
    pub quorum: u64,
    pub approval_bps: u16,
}
impl DaoState {
    pub fn new(quorum: u64, approval_bps: u16) -> Result<Self, String> {
        if quorum == 0 || approval_bps > 10_000 {
            return Err("invalid DAO parameters".into());
        }
        Ok(Self {
            proposals: BTreeMap::new(),
            votes: BTreeMap::new(),
            treasury: 0,
            allocations: BTreeMap::new(),
            timelocks: BTreeMap::new(),
            quorum,
            approval_bps,
        })
    }
    pub fn root(&self) -> [u8; 32] {
        simple_hash(&self.encode())
    }
    pub fn encode(&self) -> Vec<u8> {
        let mut o = Vec::from(MAGIC);
        o.extend_from_slice(&self.quorum.to_be_bytes());
        o.extend_from_slice(&self.approval_bps.to_be_bytes());
        o.extend_from_slice(&self.treasury.to_be_bytes());
        o.extend_from_slice(&(self.proposals.len() as u32).to_be_bytes());
        for p in self.proposals.values() {
            o.extend_from_slice(&p.id.to_be_bytes());
            o.push(match p.status {
                Status::Pending => 0,
                Status::Active => 1,
                Status::Passed => 2,
                Status::Rejected => 3,
                Status::Queued => 4,
                Status::Executed => 5,
                Status::Cancelled => 6,
            });
            put(&mut o, p.title.as_bytes());
            put(&mut o, p.description.as_bytes());
            put(&mut o, p.proposer.as_bytes());
            o.extend_from_slice(&p.start.to_be_bytes());
            o.extend_from_slice(&p.end.to_be_bytes());
            o.extend_from_slice(&p.yes.to_be_bytes());
            o.extend_from_slice(&p.no.to_be_bytes());
            o.extend_from_slice(&p.abstain.to_be_bytes());
            match &p.action_recipient {
                Some(v) => {
                    o.push(1);
                    put(&mut o, v.as_bytes())
                }
                None => o.push(0),
            }
            o.extend_from_slice(&p.action_amount.to_be_bytes())
        }
        o.extend_from_slice(&(self.votes.len() as u32).to_be_bytes());
        for ((id, v), (k, w)) in &self.votes {
            o.extend_from_slice(&id.to_be_bytes());
            put(&mut o, v.as_bytes());
            o.push(*k);
            o.extend_from_slice(&w.to_be_bytes())
        }
        o.extend_from_slice(&(self.allocations.len() as u32).to_be_bytes());
        for (k, v) in &self.allocations {
            put(&mut o, k.as_bytes());
            o.extend_from_slice(&v.to_be_bytes())
        }
        o.extend_from_slice(&(self.timelocks.len() as u32).to_be_bytes());
        for (k, v) in &self.timelocks {
            o.extend_from_slice(&k.to_be_bytes());
            o.extend_from_slice(&v.to_be_bytes())
        }
        o
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, String> {
        if !bytes.starts_with(MAGIC) {
            return Err("invalid DAO snapshot".into());
        }
        let mut q = MAGIC.len();
        let quorum = get_u64(bytes, &mut q)?;
        let approval = get_u16(bytes, &mut q)?;
        let treasury = get_u128(bytes, &mut q)?;
        let pn = get_u32(bytes, &mut q)? as usize;
        let mut proposals = BTreeMap::new();
        for _ in 0..pn {
            let id = get_u64(bytes, &mut q)?;
            let status = match get1(bytes, &mut q)? {
                0 => Status::Pending,
                1 => Status::Active,
                2 => Status::Passed,
                3 => Status::Rejected,
                4 => Status::Queued,
                5 => Status::Executed,
                6 => Status::Cancelled,
                _ => return Err("invalid DAO status".into()),
            };
            let title = get_str(bytes, &mut q)?;
            let description = get_str(bytes, &mut q)?;
            let proposer = get_str(bytes, &mut q)?;
            let start = get_u64(bytes, &mut q)?;
            let end = get_u64(bytes, &mut q)?;
            let yes = get_u64(bytes, &mut q)?;
            let no = get_u64(bytes, &mut q)?;
            let abstain = get_u64(bytes, &mut q)?;
            let action_recipient = match get1(bytes, &mut q)? {
                0 => None,
                1 => Some(get_str(bytes, &mut q)?),
                _ => return Err("invalid DAO action recipient flag".into()),
            };
            let action_amount = get_u128(bytes, &mut q)?;
            proposals.insert(
                id,
                Proposal {
                    id,
                    title,
                    description,
                    proposer,
                    start,
                    end,
                    status,
                    yes,
                    no,
                    abstain,
                    action_recipient,
                    action_amount,
                },
            );
        }
        let vn = get_u32(bytes, &mut q)? as usize;
        let mut votes = BTreeMap::new();
        for _ in 0..vn {
            let id = get_u64(bytes, &mut q)?;
            let voter = get_str(bytes, &mut q)?;
            let kind = get1(bytes, &mut q)?;
            let weight = get_u64(bytes, &mut q)?;
            votes.insert((id, voter), (kind, weight));
        }
        let an = get_u32(bytes, &mut q)? as usize;
        let mut allocations = BTreeMap::new();
        for _ in 0..an {
            let k = get_str(bytes, &mut q)?;
            let v = get_u64(bytes, &mut q)?;
            allocations.insert(k, v);
        }
        let tn = get_u32(bytes, &mut q)? as usize;
        let mut timelocks = BTreeMap::new();
        for _ in 0..tn {
            timelocks.insert(get_u64(bytes, &mut q)?, get_u64(bytes, &mut q)?);
        }
        if q != bytes.len() {
            return Err("trailing DAO snapshot".into());
        }
        Ok(Self {
            proposals,
            votes,
            treasury,
            allocations,
            timelocks,
            quorum,
            approval_bps: approval,
        })
    }
    pub fn apply(
        &mut self,
        payload: &[u8],
        block: u64,
        sender: &str,
        voting_power: u64,
    ) -> Result<Option<DaoEffect>, String> {
        if !payload.starts_with(MAGIC) {
            return Ok(None);
        }
        let mut q = MAGIC.len();
        let op = get1(payload, &mut q)?;
        let mut effect = None;
        match op {
            0 => {
                let id = get_u64(payload, &mut q)?;
                let start = get_u64(payload, &mut q)?;
                let end = get_u64(payload, &mut q)?;
                if end <= start || self.proposals.contains_key(&id) {
                    return Err("invalid or duplicate proposal".into());
                }
                let proposer = sender.to_owned();
                let title = get_str(payload, &mut q)?;
                let description = get_str(payload, &mut q)?;
                let action_recipient = match get1(payload, &mut q)? {
                    0 => None,
                    1 => Some(get_str(payload, &mut q)?),
                    _ => return Err("invalid DAO action recipient flag".into()),
                };
                let action_amount = get_u128(payload, &mut q)?;
                if action_amount > 0 && action_recipient.is_none() {
                    return Err("treasury action requires recipient".into());
                }
                self.proposals.insert(
                    id,
                    Proposal {
                        id,
                        title,
                        description,
                        proposer,
                        start,
                        end,
                        status: Status::Pending,
                        yes: 0,
                        no: 0,
                        abstain: 0,
                        action_recipient,
                        action_amount,
                    },
                );
            }
            1 => {
                let id = get_u64(payload, &mut q)?;
                let voter = sender.to_owned();
                let kind = get1(payload, &mut q)?;
                let weight = voting_power;
                if !matches!(kind, 0..=2) {
                    return Err("invalid vote type".into());
                }
                let p = self.proposals.get_mut(&id).ok_or("proposal not found")?;
                if p.status == Status::Pending && block >= p.start {
                    p.status = Status::Active
                }
                if p.status != Status::Active
                    || block >= p.end
                    || weight == 0
                    || self.votes.contains_key(&(id, voter.clone()))
                {
                    return Err("invalid DAO vote".into());
                }
                self.votes.insert((id, voter), (kind, weight));
                match kind {
                    0 => p.yes = p.yes.saturating_add(weight),
                    1 => p.no = p.no.saturating_add(weight),
                    2 => p.abstain = p.abstain.saturating_add(weight),
                    _ => unreachable!(),
                };
            }
            2 => {
                let id = get_u64(payload, &mut q)?;
                let p = self.proposals.get_mut(&id).ok_or("proposal not found")?;
                if p.status == Status::Pending && block >= p.start {
                    p.status = Status::Active
                }
                if p.status != Status::Active || block < p.end {
                    return Err("proposal voting window invalid".into());
                }
                let participation = p.yes.saturating_add(p.no).saturating_add(p.abstain);
                let decided = p.yes.saturating_add(p.no);
                p.status = if participation >= self.quorum
                    && decided > 0
                    && p.yes.saturating_mul(10_000)
                        >= decided.saturating_mul(self.approval_bps as u64)
                {
                    Status::Queued
                } else {
                    Status::Rejected
                };
                if p.status == Status::Queued {
                    self.timelocks.insert(id, block.saturating_add(1));
                }
            }
            3 => {
                let id = get_u64(payload, &mut q)?;
                let p = self.proposals.get_mut(&id).ok_or("proposal not found")?;
                if p.status != Status::Queued
                    || block < *self.timelocks.get(&id).ok_or("missing timelock")?
                {
                    return Err("DAO timelock not expired".into());
                }
                let action_amount = p.action_amount;
                let recipient = if action_amount > 0 {
                    Some(
                        p.action_recipient
                            .clone()
                            .ok_or("missing treasury action recipient")?,
                    )
                } else {
                    None
                };
                if action_amount > 0 && self.treasury < action_amount {
                    return Err("insufficient DAO treasury for proposal execution".into());
                }
                p.status = Status::Executed;
                self.timelocks.remove(&id);
                if action_amount > 0 {
                    let recipient = recipient.expect("validated treasury recipient");
                    self.treasury -= action_amount;
                    effect = Some(DaoEffect::TreasuryPayout {
                        recipient,
                        amount: action_amount,
                    });
                }
            }
            4 => {
                let amount = get_u128(payload, &mut q)?;
                if amount == 0 {
                    return Err("treasury deposit must be non-zero".into());
                }
                self.treasury = self
                    .treasury
                    .checked_add(amount)
                    .ok_or("treasury overflow")?;
                effect = Some(DaoEffect::TreasuryDeposit { amount });
            }
            5 => {
                return Err(
                    "direct DAO allocation is disabled; use an approved treasury proposal".into(),
                )
            }
            6 => {
                return Err(
                    "direct DAO release is disabled; use an approved treasury proposal".into(),
                )
            }
            _ => return Err("unknown DAO operation".into()),
        };
        if q != payload.len() {
            return Err("trailing DAO payload".into());
        }
        Ok(effect)
    }
}
fn put(o: &mut Vec<u8>, v: &[u8]) {
    o.extend_from_slice(&(v.len() as u32).to_be_bytes());
    o.extend_from_slice(v)
}
fn get1(b: &[u8], p: &mut usize) -> Result<u8, String> {
    if *p >= b.len() {
        return Err("truncated DAO payload".into());
    }
    let v = b[*p];
    *p += 1;
    Ok(v)
}
fn get_u16(b: &[u8], p: &mut usize) -> Result<u16, String> {
    if *p + 2 > b.len() {
        return Err("truncated DAO u16".into());
    }
    let v = u16::from_be_bytes(b[*p..*p + 2].try_into().unwrap());
    *p += 2;
    Ok(v)
}
fn get_u32(b: &[u8], p: &mut usize) -> Result<u32, String> {
    if *p + 4 > b.len() {
        return Err("truncated DAO u32".into());
    }
    let v = u32::from_be_bytes(b[*p..*p + 4].try_into().unwrap());
    *p += 4;
    Ok(v)
}
fn get_u64(b: &[u8], p: &mut usize) -> Result<u64, String> {
    if *p + 8 > b.len() {
        return Err("truncated DAO integer".into());
    }
    let v = u64::from_be_bytes(b[*p..*p + 8].try_into().unwrap());
    *p += 8;
    Ok(v)
}
fn get_u128(b: &[u8], p: &mut usize) -> Result<u128, String> {
    if *p + 16 > b.len() {
        return Err("truncated DAO integer".into());
    }
    let v = u128::from_be_bytes(b[*p..*p + 16].try_into().unwrap());
    *p += 16;
    Ok(v)
}
fn get_str(b: &[u8], p: &mut usize) -> Result<String, String> {
    if *p + 4 > b.len() {
        return Err("truncated DAO string".into());
    }
    let n = u32::from_be_bytes(b[*p..*p + 4].try_into().unwrap()) as usize;
    *p += 4;
    if *p + n > b.len() {
        return Err("truncated DAO string".into());
    }
    let s =
        String::from_utf8(b[*p..*p + n].to_vec()).map_err(|_| "invalid DAO utf8".to_string())?;
    *p += n;
    Ok(s)
}
