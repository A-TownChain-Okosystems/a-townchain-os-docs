//! Fail-closed authorization policy for GlobusOS userspace services.
//!
//! This crate decides whether a requested operation is permitted. It does not
//! create kernel capabilities and it never grants authority implicitly.

use std::collections::{HashMap, HashSet};

/// Principal making an authorization request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PrincipalId(pub u64);

/// Protected resource namespace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Resource {
    Filesystem,
    Network,
    Device,
    Identity,
    Wallet,
    Update,
    Process,
    Browser,
}

/// Operation requested against a resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operation {
    Read,
    Write,
    Execute,
    Connect,
    Sign,
    Manage,
}

/// Authorization decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    Allow,
    Deny,
}

/// Reason attached to every policy decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionReason {
    ExplicitGrant,
    MissingGrant,
    RevokedGrant,
    ResourceMismatch,
    OperationMismatch,
}

/// Immutable authorization request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PolicyRequest {
    pub principal: PrincipalId,
    pub resource: Resource,
    pub operation: Operation,
}

/// Explicit policy grant bound to one principal/resource/operation tuple.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PolicyGrant {
    pub principal: PrincipalId,
    pub resource: Resource,
    pub operation: Operation,
}

/// Result of a policy evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PolicyDecision {
    pub decision: Decision,
    pub reason: DecisionReason,
}

/// In-memory deny-by-default policy engine.
#[derive(Debug, Default)]
pub struct PolicyEngine {
    grants: HashSet<PolicyGrant>,
    revoked_principals: HashSet<PrincipalId>,
}

impl PolicyEngine {
    /// Creates an empty policy engine. Empty means deny-all.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an explicit grant. Existing grants are idempotent.
    pub fn grant(&mut self, grant: PolicyGrant) {
        self.grants.insert(grant);
    }

    /// Revokes every grant belonging to a principal.
    pub fn revoke_principal(&mut self, principal: PrincipalId) {
        self.revoked_principals.insert(principal);
    }

    /// Removes a principal revocation without restoring any implicit access.
    pub fn clear_principal_revocation(&mut self, principal: PrincipalId) {
        self.revoked_principals.remove(&principal);
    }

    /// Removes one explicit grant.
    pub fn revoke(&mut self, grant: PolicyGrant) -> bool {
        self.grants.remove(&grant)
    }

    /// Evaluates a request without side effects.
    pub fn evaluate(&self, request: PolicyRequest) -> PolicyDecision {
        if self.revoked_principals.contains(&request.principal) {
            return PolicyDecision {
                decision: Decision::Deny,
                reason: DecisionReason::RevokedGrant,
            };
        }

        let grant = PolicyGrant {
            principal: request.principal,
            resource: request.resource,
            operation: request.operation,
        };

        if self.grants.contains(&grant) {
            PolicyDecision {
                decision: Decision::Allow,
                reason: DecisionReason::ExplicitGrant,
            }
        } else {
            PolicyDecision {
                decision: Decision::Deny,
                reason: DecisionReason::MissingGrant,
            }
        }
    }

    /// Returns whether a request is explicitly allowed.
    pub fn is_allowed(&self, request: PolicyRequest) -> bool {
        self.evaluate(request).decision == Decision::Allow
    }

    /// Returns the number of active grants.
    pub fn grant_count(&self) -> usize {
        self.grants.len()
    }

    /// Returns whether a principal is currently revoked.
    pub fn is_revoked(&self, principal: PrincipalId) -> bool {
        self.revoked_principals.contains(&principal)
    }
}

/// Deterministic policy set useful for service manifests.
#[derive(Debug, Default)]
pub struct PolicySet {
    engines: HashMap<PrincipalId, PolicyEngine>,
}

impl PolicySet {
    /// Creates an empty policy set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns or creates the policy engine for a principal.
    pub fn principal_mut(&mut self, principal: PrincipalId) -> &mut PolicyEngine {
        self.engines.entry(principal).or_default()
    }

    /// Evaluates a request. Unknown principals are denied.
    pub fn evaluate(&self, request: PolicyRequest) -> PolicyDecision {
        self.engines
            .get(&request.principal)
            .map(|engine| engine.evaluate(request))
            .unwrap_or(PolicyDecision {
                decision: Decision::Deny,
                reason: DecisionReason::MissingGrant,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request() -> PolicyRequest {
        PolicyRequest {
            principal: PrincipalId(7),
            resource: Resource::Network,
            operation: Operation::Connect,
        }
    }

    #[test]
    fn empty_policy_denies_by_default() {
        assert_eq!(
            PolicyEngine::new().evaluate(request()),
            PolicyDecision {
                decision: Decision::Deny,
                reason: DecisionReason::MissingGrant
            }
        );
    }

    #[test]
    fn explicit_grant_allows_exact_tuple() {
        let mut engine = PolicyEngine::new();
        engine.grant(PolicyGrant {
            principal: PrincipalId(7),
            resource: Resource::Network,
            operation: Operation::Connect,
        });
        assert!(engine.is_allowed(request()));
        assert!(!engine.is_allowed(PolicyRequest {
            operation: Operation::Write,
            ..request()
        }));
    }

    #[test]
    fn revocation_overrides_existing_grant() {
        let mut engine = PolicyEngine::new();
        let grant = PolicyGrant {
            principal: PrincipalId(7),
            resource: Resource::Wallet,
            operation: Operation::Sign,
        };
        engine.grant(grant);
        assert!(engine.is_allowed(PolicyRequest {
            resource: Resource::Wallet,
            operation: Operation::Sign,
            ..request()
        }));
        engine.revoke_principal(PrincipalId(7));
        assert_eq!(
            engine
                .evaluate(PolicyRequest {
                    resource: Resource::Wallet,
                    operation: Operation::Sign,
                    ..request()
                })
                .reason,
            DecisionReason::RevokedGrant
        );
    }

    #[test]
    fn policy_set_denies_unknown_principal() {
        let set = PolicySet::new();
        assert_eq!(set.evaluate(request()).decision, Decision::Deny);
    }
}
