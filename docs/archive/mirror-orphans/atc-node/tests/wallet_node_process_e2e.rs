use atc_blockchain::{chain_identity::NUMERIC_CHAIN_ID, economics::ATC_BASE_UNITS};
use atc_wallet::keys::WalletKey;
use atc_wallet::node::{NodeClient, TcpNodeClient};
use atc_wallet::tx::{Transaction, TxType};
use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::Duration;

fn free_addr() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.local_addr().unwrap().to_string()
}

fn start_node(addr: &str) -> Child {
    // Hermetic process test: a fresh data dir per run keeps the node from
    // loading stale journals written by an older storage format.
    let data_dir = std::env::temp_dir().join(format!(
        "atc-wallet-node-e2e-{}-{}",
        std::process::id(),
        addr.replace(':', "-")
    ));
    let _ = std::fs::remove_dir_all(&data_dir);
    // The node must not share the compiled-in default P2P port with any
    // other node on this machine: bind an ephemeral port instead.
    let p2p_addr = free_addr();
    Command::new(env!("CARGO_BIN_EXE_atc-node"))
        .env("ATC_RPC_ADDR", addr)
        .env("ATC_P2P_ADDR", &p2p_addr)
        .env("ATC_DATA_DIR", &data_dir)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("atc-node must start")
}

fn rpc(addr: &str, method: &str, params: Value) -> Value {
    for _ in 0..100 {
        if let Ok(mut stream) = TcpStream::connect(addr) {
            let request = json!({"jsonrpc":"2.0","method":method,"params":params,"id":1});
            stream.write_all(request.to_string().as_bytes()).unwrap();
            stream.write_all(b"\n").unwrap();
            let mut line = String::new();
            BufReader::new(stream).read_line(&mut line).unwrap();
            let value: Value = serde_json::from_str(&line).unwrap();
            if let Some(error) = value.get("error") {
                panic!("RPC error: {error}");
            }
            return value["result"].clone();
        }
        thread::sleep(Duration::from_millis(25));
    }
    panic!("node RPC did not become ready at {addr}");
}

#[test]
fn wallet_to_atc_node_mempool_block_state_process_e2e() {
    let addr = free_addr();
    let mut child = start_node(&addr);

    let _ = rpc(&addr, "ping", json!({}));

    let key = WalletKey::from_seed([7u8; 32]);
    let tx = Transaction {
        chain_id: NUMERIC_CHAIN_ID,
        tx_type: TxType::Transfer,
        sender_did: "alice".into(),
        recipient_did: Some("bob".into()),
        amount: 100 * ATC_BASE_UNITS,
        gas_price: 1,
        gas_limit: 2_000,
        nonce: 0,
        timestamp: 1,
        payload: Vec::new(),
        poh_hash: [0u8; 32],
    };
    let signature = tx.sign(&key).unwrap();
    let wallet_client = TcpNodeClient::from_wallet(&addr, &key);
    let tx_id = wallet_client.submit_transaction(&tx, &signature).unwrap();
    let expected_id = tx.id(&signature).unwrap();
    assert_eq!(tx_id, expected_id);

    let produced = rpc(&addr, "produce_block", json!({"timestamp":2,"max":100}));
    assert_eq!(produced["height"], 1);
    assert_eq!(produced["tx_count"], 1);

    assert_eq!(wallet_client.balance("bob").unwrap(), 100);

    let block = rpc(&addr, "block", json!({"height":1}));
    assert_eq!(block["height"], 1);
    assert_eq!(block["tx_count"], 1);

    let root = rpc(&addr, "state_root", json!({"height":1}));
    assert_eq!(root["height"], 1);
    assert!(!root["state_root"].as_str().unwrap().is_empty());

    let _ = child.kill();
    let _ = child.wait();
}
