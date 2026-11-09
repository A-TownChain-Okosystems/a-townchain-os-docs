//! ATC-VM execution adapter.
use crate::mempool::{Transaction, TxType};
use atc_vm::{ops::parse_ops, vm::Vm};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecutionReceipt {
    pub tx_id: [u8; 32],
    pub success: bool,
    pub gas_used: u64,
    pub return_data: Vec<u8>,
}
pub trait VmExecutor: Send + Sync {
    fn execute(&self, tx: &Transaction, state_root: [u8; 32]) -> Result<ExecutionReceipt, String>;
}
pub struct AtcVmExecutor {
    pub protocol: String,
    pub vm_version: String,
    pub genesis_id: String,
}
impl VmExecutor for AtcVmExecutor {
    fn execute(&self, tx: &Transaction, _: [u8; 32]) -> Result<ExecutionReceipt, String> {
        if tx.tx_type != TxType::Contract || tx.payload.starts_with(crate::dao_state::MAGIC) {
            return Ok(ExecutionReceipt {
                tx_id: tx.id,
                success: true,
                gas_used: tx.gas_cost(),
                return_data: Vec::new(),
            });
        }
        let program = parse_ops(
            std::str::from_utf8(&tx.payload).map_err(|_| "contract payload is not UTF-8")?,
        )
        .map_err(|e| format!("invalid ATC-VM program: {e:?}"))?;
        let mut vm = Vm::new(program);
        let context = atc_vm::context::ChainContext {
            chain_id: "atc".into(),
            network_id: "devnet".into(),
            genesis_id: self.genesis_id.clone(),
            protocol_version: self.protocol.clone(),
            vm_version: self.vm_version.clone(),
        };
        let out = vm
            .execute_state_transition(&context, &self.genesis_id, &self.protocol, &self.vm_version)
            .map_err(|e| format!("ATC-VM execution: {e:?}"))?;
        Ok(ExecutionReceipt {
            tx_id: tx.id,
            success: true,
            gas_used: tx.gas_cost(),
            return_data: out.into_iter().flat_map(|v| v.to_be_bytes()).collect(),
        })
    }
}
