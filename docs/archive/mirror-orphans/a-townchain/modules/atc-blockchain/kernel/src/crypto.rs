//! Cryptographic verification and canonical transaction signing.
use crate::mempool::Transaction;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
pub trait SignatureVerifier: Send + Sync {
    fn verify(&self, message: &[u8], signature: &[u8; 64], public_key: &[u8]) -> bool;
}
pub struct Ed25519Verifier;
impl SignatureVerifier for Ed25519Verifier {
    fn verify(&self, message: &[u8], signature: &[u8; 64], public_key: &[u8]) -> bool {
        let Ok(pk) = <[u8; 32]>::try_from(public_key) else {
            return false;
        };
        let Ok(key) = VerifyingKey::from_bytes(&pk) else {
            return false;
        };
        key.verify(message, &Signature::from_bytes(signature))
            .is_ok()
    }
}
fn put_bytes(out: &mut Vec<u8>, v: &[u8]) {
    out.extend_from_slice(&(v.len() as u32).to_be_bytes());
    out.extend_from_slice(v)
}
pub fn signing_bytes(tx: &Transaction) -> Vec<u8> {
    let mut b = Vec::new();
    b.extend_from_slice(b"ATC-TX-DOMAIN-V2");
    b.extend_from_slice(&tx.chain_id.to_be_bytes());
    b.push(tx.tx_type as u8);
    put_bytes(&mut b, tx.sender_did.as_bytes());
    match &tx.recipient_did {
        Some(v) => {
            b.push(1);
            put_bytes(&mut b, v.as_bytes())
        }
        None => b.push(0),
    }
    b.extend_from_slice(&tx.amount.to_be_bytes());
    b.extend_from_slice(&tx.gas_price.to_be_bytes());
    b.extend_from_slice(&tx.gas_limit.to_be_bytes());
    b.extend_from_slice(&tx.nonce.to_be_bytes());
    b.extend_from_slice(&tx.timestamp.to_be_bytes());
    put_bytes(&mut b, &tx.payload);
    b.extend_from_slice(&tx.poh_hash);
    b
}
