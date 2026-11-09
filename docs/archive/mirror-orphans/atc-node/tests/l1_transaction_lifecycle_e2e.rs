use atc_blockchain::{
    consensus::{vote_signing_bytes, Vote},
    mempool::{Transaction as L1Transaction, TxType as L1TxType},
    Node,
};
use atc_wallet::{
    keys::WalletKey,
    tx::{Transaction as WalletTransaction, TxType as WalletTxType},
};
use ed25519_dalek::{Signer, SigningKey};
use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

const CHAIN_ID: u64 = atc_blockchain::chain_identity::NUMERIC_CHAIN_ID;
const GENESIS_BALANCE: u64 = 1_000_000;
const TRANSFER_AMOUNT: u128 = 1_000 * atc_blockchain::economics::ATC_BASE_UNITS;
const GAS_PRICE: u64 = 1;
const GAS_LIMIT: u64 = 1_000;

fn temp_path() -> PathBuf {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("atc-l1-tx-lifecycle-{now}"))
}

fn signed_vote(block: [u8; 32], voter: &str, seed: [u8; 32]) -> Vote {
    let signing = SigningKey::from_bytes(&seed);
    let mut vote = Vote {
        block,
        voter: voter.into(),
        approve: true,
        signature: [0; 64],
        public_key: signing.verifying_key().to_bytes(),
    };
    vote.signature = signing
        .sign(&vote_signing_bytes(CHAIN_ID, &vote))
        .to_bytes();
    vote
}

#[test]
fn tx_block_reward_state_finality_persistence_recovery() {
    let path = temp_path();
    std::fs::create_dir_all(&path).unwrap();

    let node =
        Node::open_storage(CHAIN_ID, "validator-a".into(), path.join("chain.journal")).unwrap();
    node.state.genesis_credit("alice", GENESIS_BALANCE).unwrap();
    let genesis = node.create_genesis_with_proposer(1, "atc-genesis").unwrap();

    node.register_validator("validator-a".into(), 1).unwrap();
    node.register_validator("validator-b".into(), 1).unwrap();
    node.register_validator_key(
        "validator-a",
        SigningKey::from_bytes(&[1u8; 32])
            .verifying_key()
            .to_bytes(),
    )
    .unwrap();
    node.register_validator_key(
        "validator-b",
        SigningKey::from_bytes(&[2u8; 32])
            .verifying_key()
            .to_bytes(),
    )
    .unwrap();
    node.set_vote_signer("validator-a", [1u8; 32]);

    let wallet_key = WalletKey::from_seed([7u8; 32]);
    let wallet_tx = WalletTransaction {
        chain_id: CHAIN_ID,
        tx_type: WalletTxType::Transfer,
        sender_did: "alice".into(),
        recipient_did: Some("bob".into()),
        amount: TRANSFER_AMOUNT,
        gas_price: GAS_PRICE,
        gas_limit: GAS_LIMIT,
        nonce: 0,
        timestamp: 2,
        payload: Vec::new(),
        poh_hash: [0; 32],
    };
    let signature = wallet_tx.sign(&wallet_key).unwrap();
    let public_key = wallet_key.public_key();
    assert!(wallet_tx.verify(&public_key, &signature).is_ok());
    let tx_id = wallet_tx.id(&signature).unwrap();

    let l1_tx = L1Transaction::new_with_chain_id(
        CHAIN_ID,
        L1TxType::Transfer,
        "alice".into(),
        Some("bob".into()),
        TRANSFER_AMOUNT,
        GAS_PRICE,
        GAS_LIMIT,
        0,
        2,
        Vec::new(),
        signature,
        public_key,
        [0; 32],
    );
    assert_eq!(l1_tx.id, tx_id);

    node.submit(l1_tx, 2).unwrap();
    let block = node.produce(3, 100).unwrap();

    assert_eq!(block.height, 1);
    assert_eq!(block.parent_hash, genesis.id);
    assert_eq!(block.transactions.len(), 1);
    assert_eq!(
        node.state.balance_base_units("alice"),
        u128::from(GENESIS_BALANCE) * atc_blockchain::economics::ATC_BASE_UNITS
            - TRANSFER_AMOUNT
            - u128::from(GAS_LIMIT)
    );
    assert_eq!(node.state.balance_base_units("bob"), TRANSFER_AMOUNT);
    assert_eq!(node.state.balance("validator-a"), 500);
    assert_eq!(
        node.state.issued_base_units(),
        1_000_500u128 * atc_blockchain::economics::ATC_BASE_UNITS
    );
    assert_eq!(node.storage.block(1).unwrap().id, block.id);
    assert_eq!(node.storage.state_root(1).unwrap(), block.state_root);

    node.submit_vote(signed_vote(block.id, "validator-a", [1u8; 32]))
        .unwrap();
    node.submit_vote(signed_vote(block.id, "validator-b", [2u8; 32]))
        .unwrap();
    assert!(node.consensus.weighted_finality(&block.id));
    assert!(node.finalize_weighted(&block).unwrap());
    assert_eq!(node.consensus.finalized().map(|x| x.0), Some(1));
    assert_eq!(
        node.storage.recover_finalized().unwrap().map(|x| x.0),
        Some(1)
    );

    drop(node);

    let recovered =
        Node::open_storage(CHAIN_ID, "validator-a".into(), path.join("chain.journal")).unwrap();
    recovered
        .register_validator_key(
            "validator-a",
            SigningKey::from_bytes(&[1u8; 32])
                .verifying_key()
                .to_bytes(),
        )
        .unwrap();
    recovered
        .register_validator_key(
            "validator-b",
            SigningKey::from_bytes(&[2u8; 32])
                .verifying_key()
                .to_bytes(),
        )
        .unwrap();
    recovered.set_vote_signer("validator-a", [1u8; 32]);
    assert_eq!(recovered.chain.height(), 1);
    assert_eq!(recovered.chain.last().unwrap().id, block.id);
    assert_eq!(
        recovered.state.balance_base_units("alice"),
        u128::from(GENESIS_BALANCE) * atc_blockchain::economics::ATC_BASE_UNITS
            - TRANSFER_AMOUNT
            - u128::from(GAS_LIMIT)
    );
    assert_eq!(recovered.state.balance_base_units("bob"), TRANSFER_AMOUNT);
    assert_eq!(recovered.state.balance("validator-a"), 500);
    assert_eq!(
        atc_blockchain::committed_state_root(
            recovered.state.root(),
            block.height,
            recovered
                .consensus
                .validator_snapshot_commitment(block.height),
        )
        .unwrap(),
        block.state_root
    );
    assert_eq!(recovered.consensus.finalized().map(|x| x.0), Some(1));
    assert_eq!(recovered.consensus.validator_stake("validator-a"), 1);
    assert_eq!(recovered.consensus.validator_stake("validator-b"), 1);

    std::fs::remove_dir_all(path).unwrap();
}
