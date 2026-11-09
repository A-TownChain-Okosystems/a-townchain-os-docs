//! Explicit Aurora to GlobusOS policy/IPC bridge.
use crate::Decision;
use crate::types::{AuroraRequest, RequestId};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeDecision {
    Allow,
    Deny,
    ApprovalRequired,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyRequest {
    pub request_id: RequestId,
    pub principal: String,
    pub action: String,
    pub resource: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyResponse {
    pub request_id: RequestId,
    pub decision: BridgeDecision,
}
pub trait GlobusPolicyEndpoint {
    fn evaluate(&self, r: &PolicyRequest) -> BridgeDecision;
}
pub fn translate(r: &AuroraRequest, d: Decision) -> PolicyResponse {
    let d = match d {
        Decision::Allow => BridgeDecision::Allow,
        Decision::Deny => BridgeDecision::Deny,
        Decision::ApprovalRequired => BridgeDecision::ApprovalRequired,
    };
    PolicyResponse {
        request_id: r.request_id.clone(),
        decision: d,
    }
}
