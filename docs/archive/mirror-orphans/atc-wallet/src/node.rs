//! Real L1 node transport.
//!
//! TcpNodeClient speaks the canonical atc-node JSON-RPC endpoint. The node
//! owns validation, mempool admission, block production, state transition and
//! persistence; the wallet only constructs and signs transactions.

use crate::keys::WalletKey;
use crate::tx::{Transaction, TxType};
use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeError {
    Transport(String),
    Rejected(String),
    Unsupported,
    Protocol(String),
}

pub trait NodeClient {
    fn submit_transaction(
        &self,
        tx: &Transaction,
        signature: &[u8; 64],
    ) -> Result<[u8; 32], NodeError>;
    fn balance(&self, address: &str) -> Result<u64, NodeError>;
}

#[derive(Debug, Clone)]
pub struct TcpNodeClient {
    endpoint: String,
    public_key: [u8; 32],
    timeout: Duration,
}

impl TcpNodeClient {
    pub fn new(endpoint: impl Into<String>, public_key: [u8; 32]) -> Self {
        Self {
            endpoint: endpoint.into(),
            public_key,
            timeout: Duration::from_secs(5),
        }
    }

    pub fn from_wallet(endpoint: impl Into<String>, key: &WalletKey) -> Self {
        Self::new(endpoint, key.public_key())
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    fn call(&self, method: &str, params: Value) -> Result<Value, NodeError> {
        let mut addrs = self
            .endpoint
            .to_socket_addrs()
            .map_err(|e| NodeError::Transport(e.to_string()))?;
        let addr = addrs
            .next()
            .ok_or_else(|| NodeError::Transport("node endpoint resolved to no address".into()))?;
        let mut stream = TcpStream::connect_timeout(&addr, self.timeout)
            .map_err(|e| NodeError::Transport(e.to_string()))?;
        stream
            .set_read_timeout(Some(self.timeout))
            .map_err(|e| NodeError::Transport(e.to_string()))?;
        stream
            .set_write_timeout(Some(self.timeout))
            .map_err(|e| NodeError::Transport(e.to_string()))?;

        let request = json!({"jsonrpc":"2.0","method":method,"params":params,"id":1});
        stream
            .write_all(request.to_string().as_bytes())
            .and_then(|_| stream.write_all(b"\n"))
            .map_err(|e| NodeError::Transport(e.to_string()))?;
        let mut line = String::new();
        BufReader::new(stream)
            .read_line(&mut line)
            .map_err(|e| NodeError::Transport(e.to_string()))?;

        let response: Value =
            serde_json::from_str(&line).map_err(|e| NodeError::Protocol(e.to_string()))?;
        if let Some(error) = response.get("error") {
            let code = error.get("code").and_then(Value::as_i64).unwrap_or(-32000);
            let message = error
                .get("message")
                .and_then(Value::as_str)
                .unwrap_or("node rejected request");
            if code == -32001 {
                return Err(NodeError::Rejected(message.to_string()));
            }
            return Err(NodeError::Protocol(message.to_string()));
        }
        response
            .get("result")
            .cloned()
            .ok_or_else(|| NodeError::Protocol("JSON-RPC response missing result".into()))
    }
}

impl NodeClient for TcpNodeClient {
    fn submit_transaction(
        &self,
        tx: &Transaction,
        signature: &[u8; 64],
    ) -> Result<[u8; 32], NodeError> {
        let params = json!({
            "chain_id": tx.chain_id,
            "tx_type": tx_type_number(tx.tx_type),
            "sender_did": tx.sender_did,
            "recipient_did": tx.recipient_did,
            // u128 amounts exceed JSON number precision: transport as decimal
            // string, which the node RPC accepts for u128 params.
            "amount": tx.amount.to_string(),
            "gas_price": tx.gas_price,
            "gas_limit": tx.gas_limit,
            "nonce": tx.nonce,
            "timestamp": tx.timestamp,
            "payload": hex::encode(&tx.payload),
            "signature": hex::encode(signature),
            "public_key": hex::encode(self.public_key),
            "poh_hash": hex::encode(tx.poh_hash)
        });
        let result = self.call("submit_transaction", params)?;
        let encoded = result
            .get("tx_id")
            .and_then(Value::as_str)
            .ok_or_else(|| NodeError::Protocol("submit_transaction missing tx_id".into()))?;
        let bytes = hex::decode(encoded).map_err(|e| NodeError::Protocol(e.to_string()))?;
        bytes
            .try_into()
            .map_err(|_| NodeError::Protocol("node returned non-32-byte tx id".into()))
    }

    fn balance(&self, address: &str) -> Result<u64, NodeError> {
        self.call("balance", json!({"address":address}))?
            .get("balance")
            .and_then(Value::as_u64)
            .ok_or_else(|| NodeError::Protocol("balance response missing numeric balance".into()))
    }
}

fn tx_type_number(tx_type: TxType) -> u8 {
    match tx_type {
        TxType::Transfer => 0,
        TxType::Stake => 1,
        TxType::Unstake => 2,
        TxType::Contract => 3,
    }
}

pub struct UnconfiguredNode;

impl NodeClient for UnconfiguredNode {
    fn submit_transaction(
        &self,
        _tx: &Transaction,
        _signature: &[u8; 64],
    ) -> Result<[u8; 32], NodeError> {
        Err(NodeError::Unsupported)
    }
    fn balance(&self, _address: &str) -> Result<u64, NodeError> {
        Err(NodeError::Unsupported)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tx_type_mapping_is_canonical() {
        assert_eq!(tx_type_number(TxType::Transfer), 0);
        assert_eq!(tx_type_number(TxType::Stake), 1);
        assert_eq!(tx_type_number(TxType::Unstake), 2);
        assert_eq!(tx_type_number(TxType::Contract), 3);
    }
}
