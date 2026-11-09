//! Deterministic receipt root.
use crate::{execution::ExecutionReceipt, security::simple_hash};
pub fn root(receipts: &[ExecutionReceipt]) -> [u8; 32] {
    let mut b = Vec::new();
    for r in receipts {
        b.extend_from_slice(&r.tx_id);
        b.push(r.success as u8);
        b.extend_from_slice(&r.gas_used.to_be_bytes());
        b.extend_from_slice(&simple_hash(&r.return_data));
    }
    simple_hash(&b)
}
