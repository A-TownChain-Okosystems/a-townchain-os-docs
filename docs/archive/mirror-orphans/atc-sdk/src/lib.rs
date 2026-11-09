#![allow(clippy::too_many_arguments)]
//! Canonical transaction construction for A-TownChain.
use atc_blockchain::{
    crypto::signing_bytes,
    mempool::{Transaction, TxType},
};
use ed25519_dalek::{Signer, SigningKey};

pub struct TransactionBuilder {
    pub chain_id: u64,
    pub tx_type: TxType,
    pub sender: String,
    pub recipient: Option<String>,
    pub amount: u128,
    pub gas_price: u64,
    pub gas_limit: u64,
    pub nonce: u64,
    pub timestamp: u64,
    pub payload: Vec<u8>,
    pub poh_hash: [u8; 32],
}

fn field(o: &mut Vec<u8>, v: &[u8]) {
    o.extend_from_slice(&(v.len() as u32).to_be_bytes());
    o.extend_from_slice(v);
}

fn dao(op: u8, body: impl FnOnce(&mut Vec<u8>)) -> Vec<u8> {
    let mut o = Vec::from(b"ATC-DAO-V2".as_slice());
    o.push(op);
    body(&mut o);
    o
}

impl TransactionBuilder {
    pub fn dao_create_proposal(
        chain_id: u64,
        sender: impl Into<String>,
        id: u64,
        start: u64,
        end: u64,
        title: &str,
        description: &str,
        action_recipient: Option<&str>,
        action_amount: u128,
        gas_price: u64,
        gas_limit: u64,
        nonce: u64,
        timestamp: u64,
    ) -> Self {
        Self::dao_tx(
            chain_id,
            sender,
            dao(0, |o| {
                o.extend_from_slice(&id.to_be_bytes());
                o.extend_from_slice(&start.to_be_bytes());
                o.extend_from_slice(&end.to_be_bytes());
                field(o, title.as_bytes());
                field(o, description.as_bytes());
                match action_recipient {
                    Some(v) => {
                        o.push(1);
                        field(o, v.as_bytes());
                    }
                    None => o.push(0),
                }
                o.extend_from_slice(&action_amount.to_be_bytes());
            }),
            gas_price,
            gas_limit,
            nonce,
            timestamp,
        )
    }

    pub fn dao_vote(
        chain_id: u64,
        sender: impl Into<String>,
        proposal_id: u64,
        vote_type: u8,
        gas_price: u64,
        gas_limit: u64,
        nonce: u64,
        timestamp: u64,
    ) -> Self {
        Self::dao_tx(
            chain_id,
            sender,
            dao(1, |o| {
                o.extend_from_slice(&proposal_id.to_be_bytes());
                o.push(vote_type);
            }),
            gas_price,
            gas_limit,
            nonce,
            timestamp,
        )
    }

    pub fn dao_finalize(
        chain_id: u64,
        sender: impl Into<String>,
        proposal_id: u64,
        gas_price: u64,
        gas_limit: u64,
        nonce: u64,
        timestamp: u64,
    ) -> Self {
        Self::dao_tx(
            chain_id,
            sender,
            dao(2, |o| o.extend_from_slice(&proposal_id.to_be_bytes())),
            gas_price,
            gas_limit,
            nonce,
            timestamp,
        )
    }

    pub fn dao_execute(
        chain_id: u64,
        sender: impl Into<String>,
        proposal_id: u64,
        gas_price: u64,
        gas_limit: u64,
        nonce: u64,
        timestamp: u64,
    ) -> Self {
        Self::dao_tx(
            chain_id,
            sender,
            dao(3, |o| o.extend_from_slice(&proposal_id.to_be_bytes())),
            gas_price,
            gas_limit,
            nonce,
            timestamp,
        )
    }

    pub fn dao_fund(
        chain_id: u64,
        sender: impl Into<String>,
        amount: u128,
        gas_price: u64,
        gas_limit: u64,
        nonce: u64,
        timestamp: u64,
    ) -> Self {
        Self::dao_tx(
            chain_id,
            sender,
            dao(4, |o| o.extend_from_slice(&amount.to_be_bytes())),
            gas_price,
            gas_limit,
            nonce,
            timestamp,
        )
    }

    fn dao_tx(
        chain_id: u64,
        sender: impl Into<String>,
        payload: Vec<u8>,
        gas_price: u64,
        gas_limit: u64,
        nonce: u64,
        timestamp: u64,
    ) -> Self {
        Self {
            chain_id,
            tx_type: TxType::Contract,
            sender: sender.into(),
            recipient: None,
            amount: 0u128,
            gas_price,
            gas_limit,
            nonce,
            timestamp,
            payload,
            poh_hash: [0; 32],
        }
    }

    pub fn transfer(
        chain_id: u64,
        sender: impl Into<String>,
        recipient: impl Into<String>,
        amount: u128,
        gas_price: u64,
        gas_limit: u64,
        nonce: u64,
        timestamp: u64,
    ) -> Self {
        Self {
            chain_id,
            tx_type: TxType::Transfer,
            sender: sender.into(),
            recipient: Some(recipient.into()),
            amount,
            gas_price,
            gas_limit,
            nonce,
            timestamp,
            payload: Vec::new(),
            poh_hash: [0; 32],
        }
    }

    pub fn stake(
        chain_id: u64,
        sender: impl Into<String>,
        amount: u128,
        gas_price: u64,
        gas_limit: u64,
        nonce: u64,
        timestamp: u64,
    ) -> Self {
        Self {
            chain_id,
            tx_type: TxType::Stake,
            sender: sender.into(),
            recipient: None,
            amount,
            gas_price,
            gas_limit,
            nonce,
            timestamp,
            payload: Vec::new(),
            poh_hash: [0; 32],
        }
    }

    pub fn unstake(
        chain_id: u64,
        sender: impl Into<String>,
        amount: u128,
        gas_price: u64,
        gas_limit: u64,
        nonce: u64,
        timestamp: u64,
    ) -> Self {
        Self {
            chain_id,
            tx_type: TxType::Unstake,
            sender: sender.into(),
            recipient: None,
            amount,
            gas_price,
            gas_limit,
            nonce,
            timestamp,
            payload: Vec::new(),
            poh_hash: [0; 32],
        }
    }

    pub fn payload(mut self, p: Vec<u8>) -> Self {
        self.payload = p;
        self
    }

    pub fn poh_hash(mut self, h: [u8; 32]) -> Self {
        self.poh_hash = h;
        self
    }

    pub fn sign(self, key: &SigningKey) -> Transaction {
        let mut tx = Transaction::new_with_chain_id(
            self.chain_id,
            self.tx_type,
            self.sender,
            self.recipient,
            self.amount,
            self.gas_price,
            self.gas_limit,
            self.nonce,
            self.timestamp,
            self.payload,
            [0; 64],
            key.verifying_key().to_bytes(),
            self.poh_hash,
        );
        tx.signature = key.sign(&signing_bytes(&tx)).to_bytes();
        tx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_signature_is_accepted() {
        let key = SigningKey::from_bytes(&[9u8; 32]);
        let tx = TransactionBuilder::transfer(
            658467,
            "alice",
            "bob",
            10u128 * atc_blockchain::economics::ATC_BASE_UNITS,
            1,
            1000,
            0,
            1,
        )
        .sign(&key);
        assert_eq!(tx.chain_id, 658467);
        assert_eq!(tx.signature.len(), 64);
        assert_ne!(tx.id, [0; 32]);
    }
}
