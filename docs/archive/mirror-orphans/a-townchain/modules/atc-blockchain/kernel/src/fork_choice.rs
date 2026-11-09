//! Deterministic fork-choice boundary.
//!
//! The canonical Node is currently append-only: it does not maintain side
//! branches and therefore must never use a "longer tip wins" comparison to
//! silently reorg state. This boundary is deliberately finality-aware so a
//! future fork-choice implementation cannot select a candidate that crosses
//! the finalized prefix.

use crate::Block;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForkChoiceError {
    FinalityViolation,
}

/// Compare two candidate tips without permitting a finalized-prefix change.
///
/// Height is the current deterministic tie-breaker for candidates above the
/// finalized boundary; block ID provides a stable tie-breaker at equal height.
/// A conflict at or below finalized_height is never selectable.
pub fn choose<'a>(
    a: &'a Block,
    b: &'a Block,
    finalized_height: Option<u64>,
) -> Result<&'a Block, ForkChoiceError> {
    if a.height == b.height && a.id != b.id && finalized_height.is_some_and(|h| a.height <= h) {
        return Err(ForkChoiceError::FinalityViolation);
    }

    Ok(if a.height > b.height {
        a
    } else if b.height > a.height {
        b
    } else if a.id <= b.id {
        a
    } else {
        b
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Block;

    fn block(height: u64, id: u8) -> Block {
        Block {
            id: [id; 32],
            height,
            parent_hash: [0; 32],
            proposer: "validator".into(),
            timestamp: 0,
            transactions: Vec::new(),
            tx_root: [0; 32],
            state_root: [0; 32],
            receipt_root: [0; 32],
            signature: [0; 64],
        }
    }

    #[test]
    fn conflicting_finalized_height_is_not_selectable() {
        let a = block(5, 1);
        let b = block(5, 2);
        assert_eq!(
            choose(&a, &b, Some(5)),
            Err(ForkChoiceError::FinalityViolation)
        );
    }

    #[test]
    fn conflicting_unfinalized_height_remains_deterministic() {
        let a = block(5, 1);
        let b = block(5, 2);
        assert_eq!(choose(&a, &b, Some(4)).unwrap().id, a.id);
    }

    #[test]
    fn higher_tip_is_selected_only_above_finality_boundary() {
        let a = block(5, 1);
        let b = block(6, 2);
        assert_eq!(choose(&a, &b, Some(5)).unwrap().id, b.id);
    }
}
