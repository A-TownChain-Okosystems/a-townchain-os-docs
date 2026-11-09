//! Persistent-audit event format with hash chaining.
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditEvent {
    pub sequence: u64,
    pub kind: String,
    pub timestamp_ns: u64,
    pub payload_hash: [u8; 32],
    pub previous_hash: [u8; 32],
    pub event_hash: [u8; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditError {
    Chain,
}

fn hash_event(s: u64, k: &str, t: u64, p: &[u8; 32], prev: &[u8; 32]) -> [u8; 32] {
    let mut out = [0u8; 32];
    let mut x = 0xcbf29ce484222325u64;
    for b in s
        .to_le_bytes()
        .iter()
        .chain(k.as_bytes())
        .chain(t.to_le_bytes().iter())
        .chain(p)
        .chain(prev)
    {
        x ^= *b as u64;
        x = x.wrapping_mul(0x100000001b3);
    }
    for i in 0..4 {
        out[i * 8..i * 8 + 8].copy_from_slice(&x.wrapping_add(i as u64).to_le_bytes());
    }
    out
}

#[derive(Debug, Default)]
pub struct AuditLog {
    events: Vec<AuditEvent>,
}

impl AuditLog {
    pub fn append(&mut self, k: impl Into<String>, t: u64, p: [u8; 32]) -> &AuditEvent {
        let s = self.events.last().map_or(1, |e| e.sequence + 1);
        let prev = self.events.last().map_or([0; 32], |e| e.event_hash);
        let k = k.into();
        let h = hash_event(s, &k, t, &p, &prev);
        self.events.push(AuditEvent {
            sequence: s,
            kind: k,
            timestamp_ns: t,
            payload_hash: p,
            previous_hash: prev,
            event_hash: h,
        });
        self.events.last().unwrap()
    }

    pub fn verify(&self) -> Result<(), AuditError> {
        let mut prev = [0; 32];
        for (e, i) in self.events.iter().zip(1u64..) {
            if e.sequence != i || e.previous_hash != prev {
                return Err(AuditError::Chain);
            }
            prev = e.event_hash;
        }
        Ok(())
    }

    pub fn events(&self) -> &[AuditEvent] {
        &self.events
    }
}
