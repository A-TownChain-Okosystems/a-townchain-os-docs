//! Canonical validator state and deterministic lifecycle transitions.
//!
//! This module is deliberately independent of ConsensusEngine.  It is the
//! state-machine representation that can be replayed from canonical blocks.
use crate::security::simple_hash;
use std::collections::BTreeMap;

const DOMAIN: &[u8] = b"ATC-VALIDATOR-STATE-V1";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidatorRecord {
    pub stake: u128,
    pub public_key: [u8; 32],
    pub activation_height: u64,
    pub active: bool,
    pub slashed_base_units: u128,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValidatorTransition {
    Register {
        address: String,
        stake: u128,
        public_key: [u8; 32],
        activation_height: u64,
    },
    RotateKey {
        address: String,
        public_key: [u8; 32],
        activation_height: u64,
    },
    Stake {
        address: String,
        amount: u128,
    },
    Unstake {
        address: String,
        amount: u128,
    },
    Slash {
        address: String,
        evidence_id: [u8; 32],
        evidence_height: u64,
        penalty: u128,
        activation_height: u64,
    },
    Unregister {
        address: String,
        activation_height: u64,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidatorState {
    validators: BTreeMap<String, ValidatorRecord>,
}

impl Default for ValidatorState {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidatorTransition {
    /// Canonical binary payload. All integer fields are big-endian and every
    /// variable-length address is length-prefixed with u32.
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::from(b"ATC-VAL-TX-V1" as &[u8]);
        let put_address = |out: &mut Vec<u8>, address: &str| {
            out.extend_from_slice(&(address.len() as u32).to_be_bytes());
            out.extend_from_slice(address.as_bytes());
        };
        match self {
            Self::Register {
                address,
                stake,
                public_key,
                activation_height,
            } => {
                out.push(0);
                put_address(&mut out, address);
                out.extend_from_slice(&stake.to_be_bytes());
                out.extend_from_slice(public_key);
                out.extend_from_slice(&activation_height.to_be_bytes());
            }
            Self::RotateKey {
                address,
                public_key,
                activation_height,
            } => {
                out.push(1);
                put_address(&mut out, address);
                out.extend_from_slice(public_key);
                out.extend_from_slice(&activation_height.to_be_bytes());
            }
            Self::Stake { address, amount } => {
                out.push(2);
                put_address(&mut out, address);
                out.extend_from_slice(&amount.to_be_bytes());
            }
            Self::Unstake { address, amount } => {
                out.push(3);
                put_address(&mut out, address);
                out.extend_from_slice(&amount.to_be_bytes());
            }
            Self::Slash {
                address,
                evidence_id,
                evidence_height,
                penalty,
                activation_height,
            } => {
                out.push(4);
                put_address(&mut out, address);
                out.extend_from_slice(evidence_id);
                out.extend_from_slice(&evidence_height.to_be_bytes());
                out.extend_from_slice(&penalty.to_be_bytes());
                out.extend_from_slice(&activation_height.to_be_bytes());
            }
            Self::Unregister {
                address,
                activation_height,
            } => {
                out.push(5);
                put_address(&mut out, address);
                out.extend_from_slice(&activation_height.to_be_bytes());
            }
        }
        out
    }

    pub fn id(&self) -> [u8; 32] {
        simple_hash(&self.encode())
    }
}

impl ValidatorTransition {
    pub fn decode(bytes: &[u8]) -> Result<Self, String> {
        const MAGIC: &[u8] = b"ATC-VAL-TX-V1";
        if !bytes.starts_with(MAGIC) {
            return Err("invalid validator transition magic".into());
        }
        let mut p = MAGIC.len();
        let tag = *bytes.get(p).ok_or("missing validator transition tag")?;
        p += 1;
        let read_u32 = |p: &mut usize| -> Result<u32, String> {
            if bytes.len() < *p + 4 {
                return Err("truncated u32".into());
            }
            let v = u32::from_be_bytes(bytes[*p..*p + 4].try_into().unwrap());
            *p += 4;
            Ok(v)
        };
        let read_u64 = |p: &mut usize| -> Result<u64, String> {
            if bytes.len() < *p + 8 {
                return Err("truncated u64".into());
            }
            let v = u64::from_be_bytes(bytes[*p..*p + 8].try_into().unwrap());
            *p += 8;
            Ok(v)
        };
        let read_u128 = |p: &mut usize| -> Result<u128, String> {
            if bytes.len() < *p + 16 {
                return Err("truncated u128".into());
            }
            let v = u128::from_be_bytes(bytes[*p..*p + 16].try_into().unwrap());
            *p += 16;
            Ok(v)
        };
        let read_address = |p: &mut usize| -> Result<String, String> {
            let n = read_u32(p)? as usize;
            if bytes.len() < *p + n {
                return Err("truncated address".into());
            };
            let s = String::from_utf8(bytes[*p..*p + n].to_vec())
                .map_err(|_| "invalid validator address")?;
            *p += n;
            Ok(s)
        };
        let read_key = |p: &mut usize| -> Result<[u8; 32], String> {
            if bytes.len() < *p + 32 {
                return Err("truncated public key".into());
            };
            let k = bytes[*p..*p + 32].try_into().unwrap();
            *p += 32;
            Ok(k)
        };
        let read_id = |p: &mut usize| -> Result<[u8; 32], String> { read_key(p) };
        let out = match tag {
            0 => Self::Register {
                address: read_address(&mut p)?,
                stake: read_u128(&mut p)?,
                public_key: read_key(&mut p)?,
                activation_height: read_u64(&mut p)?,
            },
            1 => Self::RotateKey {
                address: read_address(&mut p)?,
                public_key: read_key(&mut p)?,
                activation_height: read_u64(&mut p)?,
            },
            2 => Self::Stake {
                address: read_address(&mut p)?,
                amount: read_u128(&mut p)?,
            },
            3 => Self::Unstake {
                address: read_address(&mut p)?,
                amount: read_u128(&mut p)?,
            },
            4 => Self::Slash {
                address: read_address(&mut p)?,
                evidence_id: read_id(&mut p)?,
                evidence_height: read_u64(&mut p)?,
                penalty: read_u128(&mut p)?,
                activation_height: read_u64(&mut p)?,
            },
            5 => Self::Unregister {
                address: read_address(&mut p)?,
                activation_height: read_u64(&mut p)?,
            },
            _ => return Err("unknown validator transition tag".into()),
        };
        if p != bytes.len() {
            return Err("trailing validator transition bytes".into());
        }
        Ok(out)
    }
}

impl ValidatorState {
    pub fn new() -> Self {
        Self {
            validators: BTreeMap::new(),
        }
    }

    pub fn get(&self, address: &str) -> Option<&ValidatorRecord> {
        self.validators.get(address)
    }

    pub fn snapshot(&self) -> BTreeMap<String, ValidatorRecord> {
        self.validators.clone()
    }
    pub fn consensus_view_at_height(
        &self,
        height: u64,
    ) -> (BTreeMap<String, u128>, BTreeMap<String, [u8; 32]>) {
        let mut stakes: BTreeMap<String, u128> = BTreeMap::new();
        let mut keys = BTreeMap::new();
        for (address, record) in &self.validators {
            if record.active && record.activation_height <= height && record.stake > 0 {
                stakes.insert(address.clone(), record.stake);
                keys.insert(address.clone(), record.public_key);
            }
        }
        (stakes, keys)
    }

    pub fn consensus_view_commitment(&self, height: u64) -> [u8; 32] {
        let (stakes, keys) = self.consensus_view_at_height(height);
        let mut bytes = Vec::from(b"ATC-VALIDATOR-SET-V2".as_slice());
        for (address, stake) in &stakes {
            bytes.extend_from_slice(&(address.len() as u32).to_be_bytes());
            bytes.extend_from_slice(address.as_bytes());
            bytes.extend_from_slice(&stake.to_be_bytes());
            bytes.extend_from_slice(keys.get(address).expect("consensus view key"));
        }
        simple_hash(&bytes)
    }

    pub fn restore(&mut self, validators: BTreeMap<String, ValidatorRecord>) {
        self.validators = validators;
    }

    pub fn apply(&mut self, transition: &ValidatorTransition) -> Result<(), String> {
        match transition {
            ValidatorTransition::Register {
                address,
                stake,
                public_key,
                activation_height,
            } => {
                if address.is_empty() || *stake == 0 || *public_key == [0; 32] {
                    return Err("invalid validator registration".into());
                }
                if self.validators.contains_key(address) {
                    return Err("validator already registered".into());
                }
                self.validators.insert(
                    address.clone(),
                    ValidatorRecord {
                        stake: *stake,
                        public_key: *public_key,
                        activation_height: *activation_height,
                        active: true,
                        slashed_base_units: 0,
                    },
                );
            }
            ValidatorTransition::RotateKey {
                address,
                public_key,
                activation_height,
            } => {
                if *public_key == [0; 32] {
                    return Err("validator public key must not be zero".into());
                }
                let v = self
                    .validators
                    .get_mut(address)
                    .ok_or("validator not found")?;
                v.public_key = *public_key;
                v.activation_height = *activation_height;
            }
            ValidatorTransition::Stake { address, amount } => {
                let v = self
                    .validators
                    .get_mut(address)
                    .ok_or("validator not found")?;
                v.stake = v
                    .stake
                    .checked_add(*amount)
                    .ok_or("validator stake overflow")?;
            }
            ValidatorTransition::Unstake { address, amount } => {
                let v = self
                    .validators
                    .get_mut(address)
                    .ok_or("validator not found")?;
                v.stake = v
                    .stake
                    .checked_sub(*amount)
                    .ok_or("insufficient validator stake")?;
            }
            ValidatorTransition::Slash {
                address,
                evidence_id,
                evidence_height: _,
                penalty,
                activation_height,
            } => {
                if *evidence_id == [0; 32] || *penalty == 0 {
                    return Err("invalid slashing transition".into());
                }
                let v = self
                    .validators
                    .get_mut(address)
                    .ok_or("validator not found")?;
                let applied = (*penalty).min(v.stake);
                v.stake -= applied;
                v.slashed_base_units = v
                    .slashed_base_units
                    .checked_add(applied)
                    .ok_or("slash accounting overflow")?;
                v.activation_height = *activation_height;
            }
            ValidatorTransition::Unregister {
                address,
                activation_height,
            } => {
                let v = self
                    .validators
                    .get_mut(address)
                    .ok_or("validator not found")?;
                v.active = false;
                v.activation_height = *activation_height;
            }
        }
        Ok(())
    }

    /// Deterministic commitment over the complete validator state.
    pub fn root(&self) -> [u8; 32] {
        let mut bytes = Vec::from(DOMAIN);
        for (address, validator) in &self.validators {
            bytes.extend_from_slice(&(address.len() as u32).to_be_bytes());
            bytes.extend_from_slice(address.as_bytes());
            bytes.extend_from_slice(&validator.stake.to_be_bytes());
            bytes.extend_from_slice(&validator.public_key);
            bytes.extend_from_slice(&validator.activation_height.to_be_bytes());
            bytes.push(u8::from(validator.active));
            bytes.extend_from_slice(&validator.slashed_base_units.to_be_bytes());
        }
        simple_hash(&bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(n: u8) -> [u8; 32] {
        [n; 32]
    }

    #[test]
    fn validator_root_changes_for_key_rotation_and_slash() {
        let mut state = ValidatorState::new();
        state
            .apply(&ValidatorTransition::Register {
                address: "validator-a".into(),
                stake: 100,
                public_key: key(1),
                activation_height: 1,
            })
            .unwrap();
        let registered = state.root();
        state
            .apply(&ValidatorTransition::RotateKey {
                address: "validator-a".into(),
                public_key: key(2),
                activation_height: 2,
            })
            .unwrap();
        assert_ne!(registered, state.root());
        let rotated = state.root();
        state
            .apply(&ValidatorTransition::Slash {
                address: "validator-a".into(),
                evidence_id: [7; 32],
                evidence_height: 1,
                penalty: 25,
                activation_height: 3,
            })
            .unwrap();
        assert_ne!(rotated, state.root());
        assert_eq!(state.get("validator-a").unwrap().stake, 75);
    }

    #[test]
    fn validator_state_is_replay_deterministic() {
        let transitions = [
            ValidatorTransition::Register {
                address: "a".into(),
                stake: 100,
                public_key: key(1),
                activation_height: 1,
            },
            ValidatorTransition::Stake {
                address: "a".into(),
                amount: 50,
            },
            ValidatorTransition::RotateKey {
                address: "a".into(),
                public_key: key(2),
                activation_height: 2,
            },
            ValidatorTransition::Slash {
                address: "a".into(),
                evidence_id: [9; 32],
                evidence_height: 2,
                penalty: 20,
                activation_height: 3,
            },
            ValidatorTransition::Unregister {
                address: "a".into(),
                activation_height: 4,
            },
        ];
        let mut left = ValidatorState::new();
        let mut right = ValidatorState::new();
        for transition in &transitions {
            left.apply(transition).unwrap();
        }
        for transition in &transitions {
            right.apply(transition).unwrap();
        }
        assert_eq!(left, right);
        assert_eq!(left.root(), right.root());
    }
}
