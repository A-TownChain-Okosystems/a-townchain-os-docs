use std::{
    io::{Read, Write},
    net::TcpStream,
    process::{Command, Stdio},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

fn rpc(addr: &str, method: &str) -> serde_json::Value {
    let mut stream = TcpStream::connect(addr).expect("rpc connect");
    let body = format!(r#"{{"jsonrpc":"2.0","method":"{method}","id":1}}"#);
    stream.write_all(body.as_bytes()).unwrap();
    stream.write_all(b"\n").unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .unwrap();
    let mut out = String::new();
    stream.read_to_string(&mut out).unwrap();
    serde_json::from_str(out.trim()).unwrap()
}

fn rpc_block(addr: &str, height: u64) -> serde_json::Value {
    let mut stream = TcpStream::connect(addr).expect("rpc block connect");
    let body =
        format!(r#"{{"jsonrpc":"2.0","method":"block","params":{{"height":{height}}},"id":1}}"#);
    stream.write_all(body.as_bytes()).unwrap();
    stream.write_all(b"\n").unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .unwrap();
    let mut out = String::new();
    stream.read_to_string(&mut out).unwrap();
    serde_json::from_str(out.trim()).unwrap()
}

fn wait_status(addr: &str, min_height: u64) -> serde_json::Value {
    for _ in 0..80 {
        if let Ok(mut stream) = TcpStream::connect(addr) {
            let body = r#"{"jsonrpc":"2.0","method":"status","id":1}"#;
            let _ = stream.write_all(body.as_bytes());
            let _ = stream.write_all(b"\n");
            let _ = stream.set_read_timeout(Some(Duration::from_secs(1)));
            let mut out = String::new();
            if stream.read_to_string(&mut out).is_ok() {
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(out.trim()) {
                    if v.get("result")
                        .and_then(|x| x.get("height"))
                        .and_then(|x| x.as_u64())
                        .unwrap_or(0)
                        >= min_height
                    {
                        return v;
                    }
                }
            }
        }
        thread::sleep(Duration::from_millis(250));
    }
    panic!("node did not reach height {min_height}: {addr}");
}

fn wait_finalized_status(addr: &str, min_finalized_height: u64) -> serde_json::Value {
    for _ in 0..80 {
        if let Ok(mut stream) = TcpStream::connect(addr) {
            let body = r#"{"jsonrpc":"2.0","method":"status","id":1}"#;
            let _ = stream.write_all(body.as_bytes());
            let _ = stream.write_all(b"\n");
            let _ = stream.set_read_timeout(Some(Duration::from_secs(1)));
            let mut out = String::new();
            if stream.read_to_string(&mut out).is_ok() {
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(out.trim()) {
                    if v.get("result")
                        .and_then(|x| x.get("finalized"))
                        .and_then(|x| x.get("height"))
                        .and_then(|x| x.as_u64())
                        .unwrap_or(0)
                        >= min_finalized_height
                    {
                        return v;
                    }
                }
            }
        }
        thread::sleep(Duration::from_millis(250));
    }
    panic!("node did not reach finalized height {min_finalized_height}: {addr}");
}

fn assert_common_finalized_block(a_rpc: &str, b_rpc: &str) {
    let a_status = wait_finalized_status(a_rpc, 1);
    let b_status = wait_finalized_status(b_rpc, 1);
    let a_finalized = a_status["result"]["finalized"]["height"]
        .as_u64()
        .unwrap_or(0);
    let b_finalized = b_status["result"]["finalized"]["height"]
        .as_u64()
        .unwrap_or(0);
    let common_finalized_height = a_finalized.min(b_finalized);
    assert!(
        common_finalized_height >= 1,
        "nodes have no common finalized height: A={a_finalized}, B={b_finalized}"
    );

    let a_block = rpc_block(a_rpc, common_finalized_height);
    let b_block = rpc_block(b_rpc, common_finalized_height);
    assert_eq!(
        a_block["result"]["block_id"].as_str(),
        b_block["result"]["block_id"].as_str(),
        "nodes disagree at common finalized height {common_finalized_height}"
    );
}

#[test]
fn persistent_two_node_consensus_path() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = std::env::temp_dir().join(format!("atc-node-network-{now}"));
    let a_dir = root.join("a");
    let b_dir = root.join("b");
    std::fs::create_dir_all(&a_dir).unwrap();
    std::fs::create_dir_all(&b_dir).unwrap();

    let a_rpc = "127.0.0.1:40171";
    let a_p2p = "127.0.0.1:40172";
    let b_rpc = "127.0.0.1:40173";
    let b_p2p = "127.0.0.1:40174";
    let validators = format!(
        "atc-node-a:1:{}{},atc-node-b:1:{}{}",
        "01".repeat(32),
        "",
        "02".repeat(32),
        ""
    );

    let mut a = Command::new(env!("CARGO_BIN_EXE_atc-node"))
        .env("ATC_NODE_ID", "atc-node-a")
        .env("ATC_RPC_ADDR", a_rpc)
        .env("ATC_P2P_ADDR", a_p2p)
        .env("ATC_DATA_DIR", &a_dir)
        .env("ATC_VALIDATORS", &validators)
        .env("ATC_PEERS", "")
        .env("ATC_BLOCK_INTERVAL_SECS", "1")
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let mut b = Command::new(env!("CARGO_BIN_EXE_atc-node"))
        .env("ATC_NODE_ID", "atc-node-b")
        .env("ATC_RPC_ADDR", b_rpc)
        .env("ATC_P2P_ADDR", b_p2p)
        .env("ATC_DATA_DIR", &b_dir)
        .env("ATC_VALIDATORS", &validators)
        .env("ATC_PEERS", a_p2p)
        .env("ATC_BLOCK_INTERVAL_SECS", "1")
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let a_status = wait_status(a_rpc, 2);
    let b_status = wait_status(b_rpc, 2);

    // Heights are sampled sequentially while both nodes keep producing blocks.
    // Compare their common finalized block instead of requiring transient heights
    // to match at two different wall-clock instants.
    assert_common_finalized_block(a_rpc, b_rpc);
    assert!(
        a_status["result"]["finalized"]["height"]
            .as_u64()
            .unwrap_or(0)
            >= 1,
        "node A did not reach weighted finality"
    );
    assert!(
        b_status["result"]["finalized"]["height"]
            .as_u64()
            .unwrap_or(0)
            >= 1,
        "node B did not reach weighted finality"
    );

    let persisted_height = a_status["result"]["height"].as_u64().unwrap();
    let persisted_block = rpc_block(a_rpc, persisted_height);
    let persisted_block_id = persisted_block["result"]["block_id"]
        .as_str()
        .expect("persisted block id")
        .to_string();

    let _ = rpc(a_rpc, "ping");
    let _ = a.kill();
    let _ = b.kill();
    let _ = a.wait();
    let _ = b.wait();

    let mut recovered_a = Command::new(env!("CARGO_BIN_EXE_atc-node"))
        .env("ATC_NODE_ID", "atc-node-a")
        .env("ATC_RPC_ADDR", a_rpc)
        .env("ATC_P2P_ADDR", a_p2p)
        .env("ATC_DATA_DIR", &a_dir)
        .env("ATC_VALIDATORS", &validators)
        .env("ATC_PEERS", "")
        .env("ATC_BLOCK_INTERVAL_SECS", "1")
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let recovered_status = wait_status(a_rpc, persisted_height);
    // wait_status wartet auf >= persisted_height; der Knoten laeuft mit
    // 1s-Blockintervall weiter, daher duerfen beide Hoehen hoeher sein —
    // gesichert wird nur: kein Verlust persistierter bzw. finalisierter Höhe.
    let recovered_height = recovered_status["result"]["height"].as_u64();
    assert!(
        recovered_height >= Some(persisted_height),
        "restart lost persisted height: {recovered_height:?} < {persisted_height}"
    );
    let finalized_before = a_status["result"]["finalized"]["height"]
        .as_u64()
        .unwrap_or(0);
    let finalized_after = recovered_status["result"]["finalized"]["height"]
        .as_u64()
        .unwrap_or(0);
    assert!(
        finalized_after >= finalized_before,
        "restart lost finalized height: {finalized_after} < {finalized_before}"
    );
    let recovered_block = rpc_block(a_rpc, persisted_height);
    assert_eq!(
        recovered_block["result"]["block_id"].as_str(),
        Some(persisted_block_id.as_str()),
        "restart changed the persisted block"
    );

    let mut recovered_b = Command::new(env!("CARGO_BIN_EXE_atc-node"))
        .env("ATC_NODE_ID", "atc-node-b")
        .env("ATC_RPC_ADDR", b_rpc)
        .env("ATC_P2P_ADDR", b_p2p)
        .env("ATC_DATA_DIR", &b_dir)
        .env("ATC_VALIDATORS", &validators)
        .env("ATC_PEERS", a_p2p)
        .env("ATC_BLOCK_INTERVAL_SECS", "1")
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let next_height = persisted_height + 1;
    let _recovered_a_next = wait_status(a_rpc, next_height);
    let _recovered_b_next = wait_status(b_rpc, next_height);
    // The two status responses are not an atomic snapshot. Verify the shared
    // finalized chain prefix after both nodes have advanced past recovery.
    assert_common_finalized_block(a_rpc, b_rpc);

    let _ = recovered_a.kill();
    let _ = recovered_b.kill();
    let _ = recovered_a.wait();
    let _ = recovered_b.wait();
    std::fs::remove_dir_all(root).unwrap();
}
