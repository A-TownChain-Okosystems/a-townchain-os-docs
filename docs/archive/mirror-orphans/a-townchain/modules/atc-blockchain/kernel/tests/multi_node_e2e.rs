use atc_blockchain::network::{NetworkMessage, PeerTransport};
use atc_blockchain::{Block, Node};
use std::sync::{Arc, Mutex};

#[derive(Default)]
struct Link {
    messages: Mutex<Vec<NetworkMessage>>,
}

impl PeerTransport for Link {
    fn broadcast(&self, message: NetworkMessage) -> Result<(), String> {
        self.messages
            .lock()
            .map_err(|_| "link poisoned".to_string())?
            .push(message);
        Ok(())
    }

    fn send_to(&self, _peer_id: &str, message: NetworkMessage) -> Result<(), String> {
        self.broadcast(message)
    }
}

#[test]
fn multi_node_block_and_state_commit_are_identical() {
    let chain_id = 658467_u64;
    let node_a = Node::new(chain_id, "validator-a".into());
    let node_b = Node::new(chain_id, "validator-b".into());

    let genesis_a = node_a.create_genesis(1).expect("node A genesis");
    node_b
        .chain
        .genesis(genesis_a.clone())
        .expect("node B genesis");
    node_b
        .storage
        .commit(genesis_a.clone())
        .expect("node B persist genesis");
    node_b.state.seal_genesis();
    node_b.consensus.set_height(0);

    let link = Arc::new(Link::default());
    link.broadcast(NetworkMessage::Block(genesis_a.clone()))
        .expect("broadcast genesis");

    let block = Block::new(
        1,
        genesis_a.id,
        "validator-a".into(),
        361,
        Vec::new(),
        genesis_a.state_root,
        [0; 32],
        [0; 64],
    );

    node_a.chain.append(block.clone()).expect("node A append");
    node_b.chain.append(block.clone()).expect("node B append");

    assert_eq!(node_a.chain.height(), node_b.chain.height());
    assert_eq!(
        node_a.chain.last().unwrap().id,
        node_b.chain.last().unwrap().id
    );
    assert_eq!(
        node_a.chain.last().unwrap().state_root,
        node_b.chain.last().unwrap().state_root
    );

    link.broadcast(NetworkMessage::Block(block.clone()))
        .expect("broadcast block");
    let messages = link.messages.lock().unwrap();
    assert!(messages
        .iter()
        .any(|m| matches!(m, NetworkMessage::Block(b) if b.id == block.id)));
}
