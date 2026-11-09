use atc_blockchain::Node;
use atc_indexer::MemoryIndexer;
use atc_sdk::TransactionBuilder;
use ed25519_dalek::{Signer, SigningKey};
use std::sync::Arc;
#[test]
fn sdk_node_mempool_consensus_vm_state_storage_indexer() {
    let chain_id = 658467;
    let node = Node::new(chain_id, "validator-1".into());
    let indexer = Arc::new(MemoryIndexer::new());
    node.set_indexer(indexer.clone());
    node.state
        .genesis_credit("alice", 1_000_000)
        .expect("genesis allocation must respect supply cap");
    node.register_validator("validator-1".into(), 100).unwrap();
    node.register_validator_key(
        "validator-1",
        SigningKey::from_bytes(&[6u8; 32])
            .verifying_key()
            .to_bytes(),
    )
    .unwrap();
    node.set_vote_signer("validator-1", [6u8; 32]);
    node.create_genesis(1).unwrap();
    let key = SigningKey::from_bytes(&[7u8; 32]);
    let tx = TransactionBuilder::transfer(
        chain_id,
        "alice",
        "bob",
        100 * atc_blockchain::economics::ATC_BASE_UNITS,
        1,
        1000,
        0,
        2,
    )
    .sign(&key);
    let txid = tx.id;
    node.submit(tx, 2).unwrap();
    assert_eq!(node.pool.get_pending_batch(10).len(), 1);
    let block = node.produce(3, 10).unwrap();
    assert_eq!(block.transactions[0].id, txid);
    assert_eq!(node.state.balance("bob"), 100);
    assert!(indexer.blocks().is_empty());
    let vote_key = SigningKey::from_bytes(&[6u8; 32]);
    let voter = "validator-1".to_string();
    let mut vote = atc_blockchain::consensus::Vote {
        block: block.id,
        voter: voter.clone(),
        approve: true,
        signature: [0; 64],
        public_key: vote_key.verifying_key().to_bytes(),
    };
    let mut vb = Vec::new();
    vb.extend_from_slice(b"ATC-VOTE-V1");
    vb.extend_from_slice(&chain_id.to_be_bytes());
    vb.extend_from_slice(&vote.block);
    vb.push(1);
    vb.extend_from_slice(&(voter.len() as u32).to_be_bytes());
    vb.extend_from_slice(voter.as_bytes());
    vote.signature = vote_key.sign(&vb).to_bytes();
    node.submit_vote(vote).unwrap();
    assert!(node.finalize(&block, 1).unwrap());
    assert!(indexer.contains(block.id));
    assert_eq!(node.storage.block(1).unwrap().id, block.id);
    assert_eq!(
        node.storage.state_root(1),
        Some(
            atc_blockchain::committed_state_root(
                node.state.root(),
                1,
                node.consensus.validator_snapshot_commitment(1),
            )
            .unwrap()
        )
    );
}

#[test]
fn storage_restart_recovers_chain_and_state() {
    let path = std::env::temp_dir().join(format!(
        "atc-e2e-{}-{}.journal",
        std::process::id(),
        658467u64
    ));
    for ext in [
        "",
        "state",
        "validators",
        "finality",
        "slashing",
        "issuance",
    ] {
        let target = if ext.is_empty() {
            path.clone()
        } else {
            path.with_extension(ext)
        };
        let _ = std::fs::remove_file(target);
    }
    let node = Node::open_storage(658467, "validator-1".into(), &path).unwrap();
    node.state
        .genesis_credit("alice", 1_000_000)
        .expect("genesis allocation must respect supply cap");
    node.register_validator("validator-1".into(), 100).unwrap();
    node.register_validator_key(
        "validator-1",
        SigningKey::from_bytes(&[10u8; 32])
            .verifying_key()
            .to_bytes(),
    )
    .unwrap();
    node.set_vote_signer("validator-1", [10u8; 32]);
    node.create_genesis(1).unwrap();
    let key = SigningKey::from_bytes(&[8u8; 32]);
    let tx = TransactionBuilder::transfer(
        658467,
        "alice",
        "bob",
        25 * atc_blockchain::economics::ATC_BASE_UNITS,
        1,
        1000,
        0,
        2,
    )
    .sign(&key);
    node.submit(tx, 2).unwrap();
    let block = node.produce(3, 10).unwrap();
    let vote_key = SigningKey::from_bytes(&[10u8; 32]);
    let voter = "validator-1".to_string();
    let mut vote = atc_blockchain::consensus::Vote {
        block: block.id,
        voter: voter.clone(),
        approve: true,
        signature: [0; 64],
        public_key: vote_key.verifying_key().to_bytes(),
    };
    let mut vb = Vec::new();
    vb.extend_from_slice(b"ATC-VOTE-V1");
    vb.extend_from_slice(&658467u64.to_be_bytes());
    vb.extend_from_slice(&vote.block);
    vb.push(1);
    vb.extend_from_slice(&(voter.len() as u32).to_be_bytes());
    vb.extend_from_slice(voter.as_bytes());
    vote.signature = vote_key.sign(&vb).to_bytes();
    node.submit_vote(vote).unwrap();
    assert!(node.finalize(&block, 1).unwrap());
    drop(node);
    let reopened = Node::open_storage(658467, "validator-1".into(), &path).unwrap();
    reopened
        .register_validator_key(
            "validator-1",
            SigningKey::from_bytes(&[10u8; 32])
                .verifying_key()
                .to_bytes(),
        )
        .unwrap();
    reopened.set_vote_signer("validator-1", [10u8; 32]);
    assert_eq!(reopened.chain.height(), 1);
    assert_eq!(reopened.state.balance("bob"), 25);
    assert_eq!(reopened.storage.block(1).unwrap().id, block.id);
    assert!(
        reopened.state.issued_base_units()
            > 1_000_000u128 * atc_blockchain::economics::ATC_BASE_UNITS
    );
    assert_eq!(reopened.consensus.validator_stake("validator-1"), 100);
    assert_eq!(reopened.consensus.finalized(), Some((1, block.id)));
    let key2 = SigningKey::from_bytes(&[9u8; 32]);
    let tx2 = TransactionBuilder::transfer(658467, "alice", "carol", 10, 1, 1000, 1, 4).sign(&key2);
    reopened.submit(tx2, 4).unwrap();
    let block2 = reopened.produce(5, 10).unwrap();
    assert_eq!(block2.height, 2);
    for ext in [
        "",
        "state",
        "validators",
        "finality",
        "slashing",
        "issuance",
    ] {
        let target = if ext.is_empty() {
            path.clone()
        } else {
            path.with_extension(ext)
        };
        let _ = std::fs::remove_file(target);
    }
}

#[test]
fn dao_transactions_persist_and_recover() {
    let chain_id = 658467;
    let proposer = "alice".to_string();
    let temp = std::env::temp_dir().join(format!(
        "atc-dao-{}-{}.journal",
        std::process::id(),
        chain_id
    ));
    for ext in [
        "",
        "state",
        "validators",
        "finality",
        "slashing",
        "issuance",
    ] {
        let target = if ext.is_empty() {
            temp.clone()
        } else {
            temp.with_extension(ext)
        };
        let _ = std::fs::remove_file(target);
    }
    let node = Node::open_storage(chain_id, proposer.clone(), &temp).unwrap();
    node.state
        .genesis_credit(&proposer, 1_000_000)
        .expect("genesis allocation must respect supply cap");
    node.register_validator(proposer.clone(), 100).unwrap();
    node.register_validator_key(
        &proposer,
        SigningKey::from_bytes(&[11u8; 32])
            .verifying_key()
            .to_bytes(),
    )
    .unwrap();
    node.set_vote_signer(proposer.clone(), [11u8; 32]);
    node.create_genesis(0).unwrap();
    let key = SigningKey::from_bytes(&[11u8; 32]);

    let stake = TransactionBuilder::stake(
        chain_id,
        &proposer,
        100_000 * atc_blockchain::economics::ATC_BASE_UNITS,
        1,
        2000,
        0,
        1,
    )
    .sign(&key);
    node.submit(stake, 1).unwrap();
    node.produce(1, 10).unwrap();

    let create = TransactionBuilder::dao_create_proposal(
        chain_id,
        &proposer,
        7,
        2,
        5,
        "Treasury",
        "Fund audit",
        Some("bob"),
        125 * atc_blockchain::economics::ATC_BASE_UNITS,
        1,
        6000,
        1,
        2,
    )
    .sign(&key);
    node.submit(create, 2).unwrap();
    node.produce(2, 10).unwrap();

    let fund = TransactionBuilder::dao_fund(
        chain_id,
        &proposer,
        500 * atc_blockchain::economics::ATC_BASE_UNITS,
        1,
        6000,
        2,
        3,
    )
    .sign(&key);
    node.submit(fund, 3).unwrap();
    node.produce(3, 10).unwrap();

    let vote = TransactionBuilder::dao_vote(chain_id, &proposer, 7, 0, 1, 6000, 3, 4).sign(&key);
    node.submit(vote, 4).unwrap();
    node.produce(4, 10).unwrap();

    let finalize =
        TransactionBuilder::dao_finalize(chain_id, &proposer, 7, 1, 6000, 4, 5).sign(&key);
    node.submit(finalize, 5).unwrap();
    node.produce(5, 10).unwrap();

    let execute = TransactionBuilder::dao_execute(chain_id, &proposer, 7, 1, 6000, 5, 6).sign(&key);
    node.submit(execute, 6).unwrap();
    node.produce(6, 10).unwrap();

    let dao = atc_blockchain::dao_state::DaoState::decode(&node.state.dao_snapshot()).unwrap();
    assert_eq!(
        dao.proposals.get(&7).unwrap().status,
        atc_blockchain::dao_state::Status::Executed
    );
    assert_eq!(
        dao.treasury,
        375 * atc_blockchain::economics::ATC_BASE_UNITS
    );
    assert_eq!(node.state.balance("bob"), 125);

    drop(node);
    let recovered = Node::open_storage(chain_id, proposer, &temp).unwrap();
    let recovered_dao =
        atc_blockchain::dao_state::DaoState::decode(&recovered.state.dao_snapshot()).unwrap();
    assert_eq!(
        recovered_dao.proposals.get(&7).unwrap().status,
        atc_blockchain::dao_state::Status::Executed
    );
    assert_eq!(
        recovered_dao.treasury,
        375 * atc_blockchain::economics::ATC_BASE_UNITS
    );
    assert_eq!(recovered.state.balance("bob"), 125);
    assert_eq!(recovered.state.staked("alice"), 100_000);
    assert_eq!(recovered.chain.height(), 6);
    let _ = std::fs::remove_file(&temp);
    let _ = std::fs::remove_file(temp.with_extension("state"));
}
