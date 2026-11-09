//! Cryptographic hashing primitives.
use sha2::{Digest, Sha256};
pub fn simple_hash(data: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(data);
    h.finalize().into()
}
pub fn hash_pair(left: [u8; 32], right: [u8; 32]) -> [u8; 32] {
    let mut b = [0u8; 64];
    b[..32].copy_from_slice(&left);
    b[32..].copy_from_slice(&right);
    simple_hash(&b)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deterministic() {
        assert_eq!(simple_hash(b"atc"), simple_hash(b"atc"));
        assert_ne!(simple_hash(b"atc"), simple_hash(b"ATC"));
    }
}
