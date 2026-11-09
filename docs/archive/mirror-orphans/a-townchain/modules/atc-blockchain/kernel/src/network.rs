//! Authenticated-enough transport boundary for the A-TownChain L1 dev/test network.
//!
//! The wire format is deterministic and length-framed:
//!   magic(4) | version(1) | payload_len(u32 BE) | payload
//!
//! The first payload is always a Hello handshake containing chain id and node id.
//! The transport deliberately does not perform consensus itself; it moves complete
//! blocks/votes/transactions to the consensus boundary.

use crate::{
    consensus::Vote,
    mempool::{Transaction, TxType},
    Block,
};
use std::{
    collections::BTreeMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    time::Duration,
};

const MAGIC: &[u8; 4] = b"ATCP";
const VERSION: u8 = 2;
const MAX_VALIDATORS_PER_SNAPSHOT: usize = 4096;
const MAX_FRAME: usize = 8 * 1024 * 1024;
const MAX_TX_PER_BLOCK: usize = 500;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetworkMessage {
    Hello {
        chain_id: u64,
        node_id: String,
        height: u64,
        best_block: [u8; 32],
    },
    Transaction(Transaction),
    Block(Block),
    Vote(Vote),
    BlockRequest {
        from_height: u64,
    },
    BlockWithValidatorSnapshot {
        block: Block,
        activation_height: u64,
        validators: BTreeMap<String, u64>,
        validator_keys: BTreeMap<String, [u8; 32]>,
    },
    StatusRequest,
    Status {
        height: u64,
        best_block: [u8; 32],
        finalized: Option<(u64, [u8; 32])>,
    },
}

pub trait PeerTransport: Send + Sync {
    fn broadcast(&self, message: NetworkMessage) -> Result<(), String>;
    fn send_to(&self, peer_id: &str, message: NetworkMessage) -> Result<(), String>;
}

pub struct NullTransport;

impl PeerTransport for NullTransport {
    fn broadcast(&self, _message: NetworkMessage) -> Result<(), String> {
        Ok(())
    }

    fn send_to(&self, _peer_id: &str, _message: NetworkMessage) -> Result<(), String> {
        Ok(())
    }
}

pub struct TcpPeerTransport {
    peers: Mutex<Vec<(String, Arc<Mutex<TcpStream>>)>>,
    pub chain_id: u64,
    pub node_id: String,
}

impl TcpPeerTransport {
    pub fn new(chain_id: u64, node_id: impl Into<String>) -> Self {
        Self {
            peers: Mutex::new(Vec::new()),
            chain_id,
            node_id: node_id.into(),
        }
    }

    pub fn connect_stream_with_peer(
        &self,
        addr: &str,
        height: u64,
        best_block: [u8; 32],
    ) -> Result<(TcpStream, String), String> {
        let mut stream = TcpStream::connect(addr).map_err(|e| format!("connect {addr}: {e}"))?;
        stream.set_nodelay(true).map_err(|e| e.to_string())?;
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .map_err(|e| e.to_string())?;
        write_message(
            &mut stream,
            &NetworkMessage::Hello {
                chain_id: self.chain_id,
                node_id: self.node_id.clone(),
                height,
                best_block,
            },
        )?;
        match read_message(&mut stream)? {
            Some(NetworkMessage::Hello {
                chain_id, node_id, ..
            }) if chain_id == self.chain_id => {
                stream.set_read_timeout(None).map_err(|e| e.to_string())?;
                Ok((stream, node_id))
            }
            Some(_) => Err("peer handshake rejected".into()),
            None => Err("peer closed during handshake".into()),
        }
    }

    pub fn connect_stream(
        &self,
        addr: &str,
        height: u64,
        best_block: [u8; 32],
    ) -> Result<TcpStream, String> {
        self.connect_stream_with_peer(addr, height, best_block)
            .map(|(stream, _)| stream)
    }

    pub fn connect(&self, addr: &str, height: u64, best_block: [u8; 32]) -> Result<(), String> {
        let (stream, peer_id) = self.connect_stream_with_peer(addr, height, best_block)?;
        self.register_stream_with_peer_id(stream, peer_id)
    }

    pub fn accept(
        listener: &TcpListener,
        chain_id: u64,
        node_id: impl Into<String>,
        height: u64,
        best_block: [u8; 32],
    ) -> Result<(TcpStream, String, u64, [u8; 32]), String> {
        let (stream, _) = listener.accept().map_err(|e| e.to_string())?;
        Self::accept_stream(stream, chain_id, node_id, height, best_block)
    }

    pub fn accept_stream(
        mut stream: TcpStream,
        chain_id: u64,
        node_id: impl Into<String>,
        height: u64,
        best_block: [u8; 32],
    ) -> Result<(TcpStream, String, u64, [u8; 32]), String> {
        let local_node_id = node_id.into();
        stream.set_nodelay(true).map_err(|e| e.to_string())?;
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .map_err(|e| e.to_string())?;
        match read_message(&mut stream)? {
            Some(NetworkMessage::Hello {
                chain_id: peer_chain,
                node_id: peer_node_id,
                height: peer_height,
                best_block: peer_best_block,
            }) => {
                if peer_chain != chain_id {
                    return Err(format!(
                        "chain-id mismatch: local {chain_id}, peer {peer_chain}"
                    ));
                }
                write_message(
                    &mut stream,
                    &NetworkMessage::Hello {
                        chain_id,
                        node_id: local_node_id,
                        height,
                        best_block,
                    },
                )?;
                stream.set_read_timeout(None).map_err(|e| e.to_string())?;
                Ok((stream, peer_node_id, peer_height, peer_best_block))
            }
            _ => Err("invalid peer handshake".into()),
        }
    }

    pub fn register_stream_with_peer_id(
        &self,
        stream: TcpStream,
        peer_id: impl Into<String>,
    ) -> Result<(), String> {
        stream.set_nodelay(true).map_err(|e| e.to_string())?;
        self.peers
            .lock()
            .map_err(|_| "peer lock poisoned")?
            .push((peer_id.into(), Arc::new(Mutex::new(stream))));
        Ok(())
    }

    pub fn register_stream(&self, stream: TcpStream) -> Result<(), String> {
        self.register_stream_with_peer_id(stream, "")
    }

    pub fn peer_count(&self) -> usize {
        self.peers.lock().map(|p| p.len()).unwrap_or(0)
    }
}

impl PeerTransport for TcpPeerTransport {
    fn broadcast(&self, message: NetworkMessage) -> Result<(), String> {
        let peers = self.peers.lock().map_err(|_| "peer lock poisoned")?;
        for (_, peer) in peers.iter() {
            let mut stream = peer.lock().map_err(|_| "stream lock poisoned")?;
            write_message(&mut stream, &message)?;
        }
        Ok(())
    }

    fn send_to(&self, peer_id: &str, message: NetworkMessage) -> Result<(), String> {
        let peers = self.peers.lock().map_err(|_| "peer lock poisoned")?;
        let (_, peer) = peers
            .iter()
            .find(|(id, _)| id == peer_id)
            .ok_or("peer not found")?;
        let mut stream = peer.lock().map_err(|_| "stream lock poisoned")?;
        write_message(&mut stream, &message)
    }
}

pub fn write_message(stream: &mut TcpStream, message: &NetworkMessage) -> Result<(), String> {
    let payload = encode(message)?;
    if payload.len() > MAX_FRAME {
        return Err("network frame exceeds limit".into());
    }
    stream.write_all(MAGIC).map_err(|e| e.to_string())?;
    stream.write_all(&[VERSION]).map_err(|e| e.to_string())?;
    stream
        .write_all(&(payload.len() as u32).to_be_bytes())
        .map_err(|e| e.to_string())?;
    stream.write_all(&payload).map_err(|e| e.to_string())?;
    stream.flush().map_err(|e| e.to_string())
}

pub fn read_message(stream: &mut TcpStream) -> Result<Option<NetworkMessage>, String> {
    let mut magic = [0u8; 4];
    match stream.read_exact(&mut magic) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
        Err(e) => return Err(e.to_string()),
    }
    if &magic != MAGIC {
        return Err("invalid network magic".into());
    }
    let mut version = [0u8; 1];
    stream.read_exact(&mut version).map_err(|e| e.to_string())?;
    if version[0] != VERSION {
        return Err("unsupported network protocol version".into());
    }
    let mut len = [0u8; 4];
    stream.read_exact(&mut len).map_err(|e| e.to_string())?;
    let n = u32::from_be_bytes(len) as usize;
    if n > MAX_FRAME {
        return Err("network frame exceeds limit".into());
    }
    let mut payload = vec![0u8; n];
    stream.read_exact(&mut payload).map_err(|e| e.to_string())?;
    Ok(Some(decode(&payload)?))
}

fn put(out: &mut Vec<u8>, bytes: &[u8]) {
    out.extend_from_slice(&(bytes.len() as u32).to_be_bytes());
    out.extend_from_slice(bytes);
}
fn take<'a>(b: &'a [u8], p: &mut usize) -> Result<&'a [u8], String> {
    if *p + 4 > b.len() {
        return Err("truncated length".into());
    }
    let n = u32::from_be_bytes(b[*p..*p + 4].try_into().unwrap()) as usize;
    *p += 4;
    if *p + n > b.len() {
        return Err("truncated field".into());
    }
    let r = &b[*p..*p + n];
    *p += n;
    Ok(r)
}
fn fixed<const N: usize>(b: &[u8], p: &mut usize) -> Result<[u8; N], String> {
    if *p + N > b.len() {
        return Err("truncated fixed field".into());
    }
    let r = b[*p..*p + N].try_into().unwrap();
    *p += N;
    Ok(r)
}
fn tx_encode(t: &Transaction, o: &mut Vec<u8>) {
    o.extend_from_slice(&t.chain_id.to_be_bytes());
    o.push(t.tx_type as u8);
    put(o, t.sender_did.as_bytes());
    match &t.recipient_did {
        Some(x) => {
            o.push(1);
            put(o, x.as_bytes());
        }
        None => o.push(0),
    }
    o.extend_from_slice(&t.amount.to_be_bytes());
    o.extend_from_slice(&t.gas_price.to_be_bytes());
    o.extend_from_slice(&t.gas_limit.to_be_bytes());
    o.extend_from_slice(&t.nonce.to_be_bytes());
    o.extend_from_slice(&t.timestamp.to_be_bytes());
    put(o, &t.payload);
    o.extend_from_slice(&t.signature);
    o.extend_from_slice(&t.public_key);
    o.extend_from_slice(&t.poh_hash);
}
fn tx_decode(b: &[u8], p: &mut usize) -> Result<Transaction, String> {
    let chain_id = u64::from_be_bytes(fixed::<8>(b, p)?);
    let ty = match fixed::<1>(b, p)?[0] {
        0 => TxType::Transfer,
        1 => TxType::Stake,
        2 => TxType::Unstake,
        3 => TxType::Contract,
        4 => TxType::Validator,
        _ => return Err("invalid tx type".into()),
    };
    let sender = String::from_utf8(take(b, p)?.to_vec()).map_err(|_| "invalid sender")?;
    let recipient = match fixed::<1>(b, p)?[0] {
        0 => None,
        1 => Some(String::from_utf8(take(b, p)?.to_vec()).map_err(|_| "invalid recipient")?),
        _ => return Err("invalid recipient flag".into()),
    };
    let amount = u128::from_be_bytes(fixed::<16>(b, p)?);
    let gas_price = u64::from_be_bytes(fixed::<8>(b, p)?);
    let gas_limit = u64::from_be_bytes(fixed::<8>(b, p)?);
    let nonce = u64::from_be_bytes(fixed::<8>(b, p)?);
    let timestamp = u64::from_be_bytes(fixed::<8>(b, p)?);
    let payload = take(b, p)?.to_vec();
    let sig = fixed::<64>(b, p)?;
    let pk = fixed::<32>(b, p)?;
    let poh = fixed::<32>(b, p)?;
    Ok(Transaction::new_with_chain_id(
        chain_id, ty, sender, recipient, amount, gas_price, gas_limit, nonce, timestamp, payload,
        sig, pk, poh,
    ))
}
fn block_encode(b: &Block) -> Vec<u8> {
    let mut o = Vec::new();
    o.extend_from_slice(&b.height.to_be_bytes());
    o.extend_from_slice(&b.parent_hash);
    put(&mut o, b.proposer.as_bytes());
    o.extend_from_slice(&b.timestamp.to_be_bytes());
    o.extend_from_slice(&(b.transactions.len() as u32).to_be_bytes());
    for tx in &b.transactions {
        let mut x = Vec::new();
        tx_encode(tx, &mut x);
        put(&mut o, &x);
    }
    o.extend_from_slice(&b.tx_root);
    o.extend_from_slice(&b.state_root);
    o.extend_from_slice(&b.receipt_root);
    o.extend_from_slice(&b.signature);
    o.extend_from_slice(&b.id);
    o
}
fn block_decode(b: &[u8]) -> Result<Block, String> {
    let mut p = 0usize;
    let h = u64::from_be_bytes(fixed::<8>(b, &mut p)?);
    let parent = fixed::<32>(b, &mut p)?;
    let proposer = String::from_utf8(take(b, &mut p)?.to_vec()).map_err(|_| "invalid proposer")?;
    let ts = u64::from_be_bytes(fixed::<8>(b, &mut p)?);
    let n = u32::from_be_bytes(fixed::<4>(b, &mut p)?) as usize;
    if n > MAX_TX_PER_BLOCK {
        return Err("block transaction count exceeds protocol limit".into());
    }
    let mut txs = Vec::with_capacity(n);
    for _ in 0..n {
        let raw = take(b, &mut p)?;
        let mut q = 0;
        txs.push(tx_decode(raw, &mut q)?);
        if q != raw.len() {
            return Err("trailing tx bytes".into());
        }
    }
    let tx_root = fixed::<32>(b, &mut p)?;
    let state = fixed::<32>(b, &mut p)?;
    let receipt = fixed::<32>(b, &mut p)?;
    let sig = fixed::<64>(b, &mut p)?;
    let id = fixed::<32>(b, &mut p)?;
    if p != b.len() {
        return Err("trailing block bytes".into());
    }
    let out = Block::new(h, parent, proposer, ts, txs, state, receipt, sig);
    if out.tx_root != tx_root || out.id != id {
        return Err("block commitment mismatch".into());
    }
    Ok(out)
}
fn validator_snapshot_encode(
    activation_height: u64,
    validators: &BTreeMap<String, u64>,
    keys: &BTreeMap<String, [u8; 32]>,
    o: &mut Vec<u8>,
) -> Result<(), String> {
    if validators.len() != keys.len() || validators.len() > MAX_VALIDATORS_PER_SNAPSHOT {
        return Err("invalid validator snapshot size or key cardinality".into());
    }
    o.extend_from_slice(&activation_height.to_be_bytes());
    o.extend_from_slice(&(validators.len() as u32).to_be_bytes());
    for (address, stake) in validators {
        let key = keys
            .get(address)
            .ok_or("validator snapshot missing public key")?;
        put(o, address.as_bytes());
        o.extend_from_slice(&stake.to_be_bytes());
        o.extend_from_slice(key);
    }
    Ok(())
}

fn validator_snapshot_decode(
    b: &[u8],
    p: &mut usize,
) -> Result<crate::consensus::ActivationSnapshot, String> {
    let activation_height = u64::from_be_bytes(fixed::<8>(b, p)?);
    let n = u32::from_be_bytes(fixed::<4>(b, p)?) as usize;
    if n > MAX_VALIDATORS_PER_SNAPSHOT {
        return Err("validator snapshot exceeds protocol limit".into());
    }
    let mut validators = BTreeMap::new();
    let mut keys = BTreeMap::new();
    for _ in 0..n {
        let address =
            String::from_utf8(take(b, p)?.to_vec()).map_err(|_| "invalid validator address")?;
        let stake = u64::from_be_bytes(fixed::<8>(b, p)?);
        let key = fixed::<32>(b, p)?;
        ed25519_dalek::VerifyingKey::from_bytes(&key)
            .map_err(|_| "invalid validator public key")?;
        if address.is_empty() || stake == 0 || validators.insert(address.clone(), stake).is_some() {
            return Err("invalid or duplicate validator record".into());
        }
        keys.insert(address, key);
    }
    Ok((activation_height, validators, keys))
}

fn vote_encode(v: &Vote) -> Vec<u8> {
    let mut o = Vec::new();
    o.extend_from_slice(&v.block);
    put(&mut o, v.voter.as_bytes());
    o.push(v.approve as u8);
    o.extend_from_slice(&v.signature);
    o.extend_from_slice(&v.public_key);
    o
}
fn vote_decode(b: &[u8]) -> Result<Vote, String> {
    let mut p = 0;
    let block = fixed::<32>(b, &mut p)?;
    let voter = String::from_utf8(take(b, &mut p)?.to_vec()).map_err(|_| "invalid voter")?;
    let approve = fixed::<1>(b, &mut p)?[0];
    if approve > 1 {
        return Err("invalid vote flag".into());
    };
    let signature = fixed::<64>(b, &mut p)?;
    let public_key = fixed::<32>(b, &mut p)?;
    if p != b.len() {
        return Err("trailing vote bytes".into());
    };
    Ok(Vote {
        block,
        voter,
        approve: approve == 1,
        signature,
        public_key,
    })
}

fn encode(m: &NetworkMessage) -> Result<Vec<u8>, String> {
    let mut o = Vec::new();
    match m {
        NetworkMessage::Hello {
            chain_id,
            node_id,
            height,
            best_block,
        } => {
            o.push(0);
            o.extend_from_slice(&chain_id.to_be_bytes());
            put(&mut o, node_id.as_bytes());
            o.extend_from_slice(&height.to_be_bytes());
            o.extend_from_slice(best_block);
        }
        NetworkMessage::Transaction(t) => {
            o.push(1);
            tx_encode(t, &mut o)
        }
        NetworkMessage::Block(b) => {
            o.push(2);
            o.extend_from_slice(&block_encode(b))
        }
        NetworkMessage::Vote(v) => {
            o.push(3);
            o.extend_from_slice(&vote_encode(v))
        }
        NetworkMessage::BlockRequest { from_height } => {
            o.push(4);
            o.extend_from_slice(&from_height.to_be_bytes())
        }
        NetworkMessage::BlockWithValidatorSnapshot {
            block,
            activation_height,
            validators,
            validator_keys,
        } => {
            o.push(7);
            let raw_block = block_encode(block);
            put(&mut o, &raw_block);
            validator_snapshot_encode(*activation_height, validators, validator_keys, &mut o)?;
        }
        NetworkMessage::StatusRequest => o.push(5),
        NetworkMessage::Status {
            height,
            best_block,
            finalized,
        } => {
            o.push(6);
            o.extend_from_slice(&height.to_be_bytes());
            o.extend_from_slice(best_block);
            match finalized {
                Some((h, id)) => {
                    o.push(1);
                    o.extend_from_slice(&h.to_be_bytes());
                    o.extend_from_slice(id)
                }
                None => o.push(0),
            }
        }
    }
    Ok(o)
}
fn decode(b: &[u8]) -> Result<NetworkMessage, String> {
    let mut p = 0;
    let ty = fixed::<1>(b, &mut p)?[0];
    let m = match ty {
        0 => {
            let chain_id = u64::from_be_bytes(fixed::<8>(b, &mut p)?);
            let node_id =
                String::from_utf8(take(b, &mut p)?.to_vec()).map_err(|_| "invalid node id")?;
            let height = u64::from_be_bytes(fixed::<8>(b, &mut p)?);
            let best_block = fixed::<32>(b, &mut p)?;
            NetworkMessage::Hello {
                chain_id,
                node_id,
                height,
                best_block,
            }
        }
        1 => NetworkMessage::Transaction(tx_decode(b, &mut p)?),
        2 => NetworkMessage::Block(block_decode(&b[p..])?),
        3 => NetworkMessage::Vote(vote_decode(&b[p..])?),
        4 => NetworkMessage::BlockRequest {
            from_height: u64::from_be_bytes(fixed::<8>(b, &mut p)?),
        },
        5 => NetworkMessage::StatusRequest,
        7 => {
            let raw_block = take(b, &mut p)?;
            let block = block_decode(raw_block)?;
            let (activation_height, validators, validator_keys) =
                validator_snapshot_decode(b, &mut p)?;
            NetworkMessage::BlockWithValidatorSnapshot {
                block,
                activation_height,
                validators,
                validator_keys,
            }
        }
        6 => {
            let height = u64::from_be_bytes(fixed::<8>(b, &mut p)?);
            let best_block = fixed::<32>(b, &mut p)?;
            let has = fixed::<1>(b, &mut p)?[0];
            let finalized = if has == 1 {
                Some((
                    u64::from_be_bytes(fixed::<8>(b, &mut p)?),
                    fixed::<32>(b, &mut p)?,
                ))
            } else {
                None
            };
            NetworkMessage::Status {
                height,
                best_block,
                finalized,
            }
        }
        _ => return Err("unknown network message".into()),
    };
    if ty != 2 && ty != 3 && ty != 7 && p != b.len() {
        return Err("trailing network message bytes".into());
    }
    Ok(m)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;
    use std::thread;

    #[test]
    fn block_decode_rejects_excessive_transaction_count_before_allocation() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&1u64.to_be_bytes());
        bytes.extend_from_slice(&[0u8; 32]);
        bytes.extend_from_slice(&0u32.to_be_bytes());
        bytes.extend_from_slice(&0u64.to_be_bytes());
        bytes.extend_from_slice(&u32::MAX.to_be_bytes());

        let err = block_decode(&bytes).unwrap_err();
        assert_eq!(err, "block transaction count exceeds protocol limit");
    }

    #[test]
    fn validator_snapshot_sync_message_round_trips_deterministically() {
        let key = ed25519_dalek::SigningKey::from_bytes(&[91u8; 32])
            .verifying_key()
            .to_bytes();
        let mut validators = BTreeMap::new();
        validators.insert("validator-a".to_string(), 100);
        let mut keys = BTreeMap::new();
        keys.insert("validator-a".to_string(), key);
        let block = Block::new(
            1,
            [7; 32],
            "validator-a".into(),
            361,
            Vec::new(),
            [8; 32],
            [0; 32],
            [0; 64],
        );
        let message = NetworkMessage::BlockWithValidatorSnapshot {
            block: block.clone(),
            activation_height: 1,
            validators: validators.clone(),
            validator_keys: keys.clone(),
        };
        let decoded = decode(&encode(&message).unwrap()).unwrap();
        assert_eq!(decoded, message);
        assert!(
            matches!(decoded, NetworkMessage::BlockWithValidatorSnapshot { block: b, .. } if b.id == block.id)
        );
    }

    #[test]
    fn tcp_handshake_and_complete_block_transfer() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let server = thread::spawn(move || {
            let (mut stream, node, h, id) =
                TcpPeerTransport::accept(&listener, 658467, "node-b", 0, [0; 32]).unwrap();
            assert_eq!(node, "node-a");
            assert_eq!(h, 7);
            assert_eq!(id, [7; 32]);
            let msg = read_message(&mut stream).unwrap().unwrap();
            assert!(matches!(msg, NetworkMessage::Block(_)));
        });
        let transport = TcpPeerTransport::new(658467, "node-a");
        transport.connect(&addr.to_string(), 7, [7; 32]).unwrap();
        let b = Block::new(
            8,
            [7; 32],
            "node-a".into(),
            8,
            Vec::new(),
            [8; 32],
            [0; 32],
            [0; 64],
        );
        transport
            .broadcast(NetworkMessage::Block(b.clone()))
            .unwrap();
        server.join().unwrap();
        assert_eq!(b.height, 8);
    }
}
