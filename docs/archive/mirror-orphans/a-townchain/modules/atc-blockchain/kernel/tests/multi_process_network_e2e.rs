use atc_blockchain::{
    network::PeerTransport,
    network::{NetworkMessage, TcpPeerTransport},
    Node,
};
use ed25519_dalek::SigningKey;
use std::{
    env,
    net::TcpListener,
    process::{Command, Stdio},
    sync::Arc,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

const CHAIN_ID: u64 = 658467;
const PORT_BASE: u16 = 39100;

fn paths() -> (String, String) {
    let root = env::var("ATC_MP_ROOT").unwrap();
    (format!("{root}/node-a"), format!("{root}/node-b"))
}

fn port() -> u16 {
    env::var("ATC_MP_PORT").unwrap().parse().unwrap()
}

fn key(seed: u8) -> SigningKey {
    SigningKey::from_bytes(&[seed; 32])
}

fn wait_height(node: &Node, height: u64) {
    for _ in 0..100 {
        if node.chain.height() >= height {
            return;
        }
        thread::sleep(Duration::from_millis(25));
    }
    panic!(
        "node did not reach height {height}, current {}",
        node.chain.height()
    );
}

fn start_child(role: &str, root: &str, port: u16) -> std::process::Child {
    Command::new(env::current_exe().unwrap())
        .arg("--exact")
        .arg("multi_process_l1_network_e2e")
        .arg("--nocapture")
        .env("ATC_MP_ROLE", role)
        .env("ATC_MP_ROOT", root)
        .env("ATC_MP_PORT", port.to_string())
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("spawn node process")
}

fn run_initial_node_a() {
    let (a_path, _) = paths();
    let node = Arc::new(Node::open_storage(CHAIN_ID, "validator-a".into(), &a_path).unwrap());
    if node.chain.last().is_none() {
        node.create_genesis_with_proposer(1, "genesis").unwrap();
    }
    if node.consensus.total_validator_stake() == 0 {
        // Complete each validator identity before adding the next one so no
        // incomplete registry can be persisted.
        node.register_validator("validator-a".into(), 1).unwrap();
        node.register_validator_key("validator-a", key(1).verifying_key().to_bytes())
            .unwrap();
        node.register_validator("validator-b".into(), 1).unwrap();
        node.register_validator_key("validator-b", key(2).verifying_key().to_bytes())
            .unwrap();
    }
    node.set_vote_signer("validator-a", [1u8; 32]);

    let listener = TcpListener::bind(("127.0.0.1", port())).unwrap();
    let transport = Arc::new(TcpPeerTransport::new(CHAIN_ID, "node-a"));
    let (stream, peer, peer_height, _) = TcpPeerTransport::accept(
        &listener,
        CHAIN_ID,
        "node-a",
        node.chain.height(),
        node.chain.last().unwrap().id,
    )
    .unwrap();
    assert_eq!(peer, "node-b");
    assert_eq!(peer_height, 0);
    transport
        .register_stream_with_peer_id(stream.try_clone().unwrap(), peer.to_string())
        .unwrap();
    node.set_transport(transport.clone());
    let reader = stream.try_clone().unwrap();
    let loop_handle = node
        .clone()
        .serve_tcp_stream_with_peer(reader, peer.to_string());

    let block = node.produce_reward_block(2).unwrap();
    for _ in 0..120 {
        if node.consensus.finalized().map(|x| x.0) == Some(block.height) {
            break;
        }
        thread::sleep(Duration::from_millis(25));
    }
    assert_eq!(node.consensus.finalized().map(|x| x.0), Some(block.height));

    let block2 = node.produce_reward_block(3).unwrap();
    assert_eq!(block2.height, 2);
    assert_eq!(node.chain.height(), 2);
    for _ in 0..120 {
        if node.consensus.finalized().map(|x| x.0) == Some(block2.height) {
            break;
        }
        thread::sleep(Duration::from_millis(25));
    }
    assert_eq!(node.consensus.finalized().map(|x| x.0), Some(block2.height));
    drop(loop_handle);
}

fn run_initial_node_b() {
    let (_, b_path) = paths();
    let node = Arc::new(Node::open_storage(CHAIN_ID, "validator-b".into(), &b_path).unwrap());
    if node.chain.last().is_none() {
        node.create_genesis_with_proposer(1, "genesis").unwrap();
    }
    // Node B intentionally starts with no validator snapshot. The network
    // sync must deliver the historical validator identity before block import.
    node.set_vote_signer("validator-b", [2u8; 32]);

    let transport = Arc::new(TcpPeerTransport::new(CHAIN_ID, "node-b"));
    let mut connected = None;
    for _ in 0..100 {
        match node.connect_tcp_peer(transport.clone(), &format!("127.0.0.1:{}", port())) {
            Ok(handle) => {
                connected = Some(handle);
                break;
            }
            Err(_) => thread::sleep(Duration::from_millis(25)),
        }
    }
    let handle = connected.expect("node-b could not connect to node-a");
    for _ in 0..100 {
        if node.chain.height() >= 1 {
            break;
        }
        if handle.is_finished() {
            let result = handle.join().expect("node-b receive loop panicked");
            panic!("node-b receive loop exited before height 2: {:?}", result);
        }
        thread::sleep(Duration::from_millis(25));
    }
    if node.chain.height() < 1 {
        panic!("node-b did not receive block 1; receive loop still running");
    }
    // Convergence signal: height 2 alone is not enough; the node must have
    // broadcast its own vote and observed the peer vote so weighted finality
    // is reached. Exiting on chain height alone would kill the receive
    // thread before its finality vote ever reaches the peer.
    for _ in 0..100 {
        if node.chain.height() >= 2 && node.consensus.finalized().map(|x| x.0) == Some(2) {
            break;
        }
        if handle.is_finished() {
            let result = handle
                .join()
                .expect("node-b receive loop panicked after vote");
            panic!("node-b receive loop exited before finality 2: {:?}", result);
        }
        thread::sleep(Duration::from_millis(25));
    }
    assert_eq!(node.chain.height(), 2);
    let block2 = node.chain.last().unwrap();
    assert_eq!(node.consensus.finalized().map(|x| x.0), Some(block2.height));
    drop(handle);
}

fn run_restart_node_a() {
    let (a_path, _) = paths();
    let node = Arc::new(Node::open_storage(CHAIN_ID, "validator-a".into(), &a_path).unwrap());
    node.register_validator_key("validator-a", key(1).verifying_key().to_bytes())
        .unwrap();
    node.register_validator_key("validator-b", key(2).verifying_key().to_bytes())
        .unwrap();
    node.set_vote_signer("validator-a", [1u8; 32]);
    assert_eq!(node.chain.height(), 2);
    assert_eq!(node.consensus.finalized().map(|x| x.0), Some(2));

    let block3 = node.produce_reward_block(3).unwrap();
    let listener = TcpListener::bind(("127.0.0.1", port())).unwrap();
    let transport = Arc::new(TcpPeerTransport::new(CHAIN_ID, "node-a"));
    let (stream, peer, peer_height, _) = TcpPeerTransport::accept(
        &listener,
        CHAIN_ID,
        "node-a",
        node.chain.height(),
        block3.id,
    )
    .unwrap();
    assert_eq!(peer, "node-b");
    assert_eq!(peer_height, 2);
    transport
        .register_stream_with_peer_id(stream.try_clone().unwrap(), peer.to_string())
        .unwrap();
    node.set_transport(transport);
    let handle = node
        .clone()
        .serve_tcp_stream_with_peer(stream.try_clone().unwrap(), peer.to_string());
    // B explicitly requests the missing height after restart; A serves it from durable storage.
    thread::sleep(Duration::from_millis(500));
    assert_eq!(node.storage.block(3).unwrap().id, block3.id);
    drop(handle);
}

fn run_restart_node_b() {
    let (_, b_path) = paths();
    let node = Arc::new(Node::open_storage(CHAIN_ID, "validator-b".into(), &b_path).unwrap());
    // Historical validator snapshots must be reconstructed from durable sync
    // state; no validator keys are injected during restart.
    node.set_vote_signer("validator-b", [2u8; 32]);
    assert_eq!(node.chain.height(), 2);
    assert_eq!(node.consensus.finalized().map(|x| x.0), Some(2));

    let transport = Arc::new(TcpPeerTransport::new(CHAIN_ID, "node-b"));
    let handle = node
        .connect_tcp_peer(transport.clone(), &format!("127.0.0.1:{}", port()))
        .unwrap();
    transport
        .send_to("node-a", NetworkMessage::BlockRequest { from_height: 3 })
        .unwrap();
    wait_height(&node, 3);
    assert_eq!(node.chain.last().unwrap().height, 3);
    assert_eq!(
        node.storage.block(3).unwrap().id,
        node.chain.last().unwrap().id
    );
    drop(handle);
}

#[test]
fn multi_process_l1_network_e2e() {
    let role = env::var("ATC_MP_ROLE").unwrap_or_default();
    if !role.is_empty() {
        match role.as_str() {
            "node-a" => run_initial_node_a(),
            "node-b" => run_initial_node_b(),
            "restart-a" => run_restart_node_a(),
            "restart-b" => run_restart_node_b(),
            _ => panic!("unknown role {role}"),
        }
        return;
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = env::temp_dir().join(format!("atc-mp-l1-{now}"));
    std::fs::create_dir_all(&root).unwrap();
    let root = root.to_string_lossy().into_owned();
    let port = PORT_BASE + (std::process::id() as u16 % 1000);

    let mut a = start_child("node-a", &root, port);
    thread::sleep(Duration::from_millis(100));
    let mut b = start_child("node-b", &root, port);
    assert!(a.wait().unwrap().success(), "node-a initial process failed");
    assert!(b.wait().unwrap().success(), "node-b initial process failed");

    let mut a2 = start_child("restart-a", &root, port);
    thread::sleep(Duration::from_millis(100));
    let mut b2 = start_child("restart-b", &root, port);
    assert!(
        a2.wait().unwrap().success(),
        "node-a restart process failed"
    );
    assert!(
        b2.wait().unwrap().success(),
        "node-b restart process failed"
    );

    std::fs::remove_dir_all(root).unwrap();
}
