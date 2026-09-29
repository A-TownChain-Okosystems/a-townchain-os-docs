# Globus Master Registry — Schema v1

## Purpose

Machine-readable registry for components, services, APIs, SDKs, plugins, AI models/agents, pipelines, repositories, packages, protocols, assets and documentation.

## Required identity

Each object has:

| Field | Requirement |
|---|---|
| `id` | Stable unique identifier |
| `name` | Human-readable name |
| `type` | Registry object type |
| `status` | Explicit lifecycle/implementation state |
| `repository` | Canonical repository when applicable |
| `version` | Version or `UNVERSIONED` |
| `source` | Canonical source path/reference |
| `evidence` | Evidence references where claims are made |

## Recommended object

```yaml
id: GOS-COMP-001
name: Example Component
type: component
status: ANALYZED
repository: A-TownChain-Okosystems/example
source:
  path: src/
version: UNVERSIONED
layer: L4
languages:
  - Rust
dependencies: []
apis: []
permissions: []
hardware_requirements: []
tests:
  unit: UNKNOWN
  integration: UNKNOWN
  e2e: UNKNOWN
security:
  status: UNKNOWN
evidence:
  exact_sha: null
  workflow: null
  run: null
  job: null
  result: null
  step: null
  exit_code: null
  log: null
lifecycle:
  first_seen: null
  last_verified: null
  deprecated: false
license: UNKNOWN
notes: []
```

## Status semantics

- `UNANALYZED`: discovered but not assessed.
- `ANALYZED`: assessed against source/architecture.
- `IMPLEMENTED`: implementation exists and is evidenced.
- `FIXED`: a previously identified defect was changed; verification still required.
- `RERUNNING`: verification is in progress.
- `VERIFIED`: required verification evidence exists for the defined scope.
- `RESIDUAL`: known unresolved issue remains.
- `DEPRECATED`: no longer the active implementation.
- `ARCHIVED`: retained for historical purposes.

These statuses must never be collapsed into a single generic "complete" flag.

## ID namespaces

Recommended namespaces:

```text
GOS-COMP-###   Component
GOS-APP-###    Application
GOS-SVC-###    Service
GOS-API-###    API
GOS-SDK-###    SDK
GOS-PLG-###    Plugin
GOS-MDL-###    AI Model
GOS-AGT-###    AI Agent
GOS-PIP-###    Pipeline
GOS-LIB-###    Library
GOS-REP-###    Repository
GOS-PKG-###    Package
GOS-HW-###     Hardware target
GOS-PRO-###    Protocol
GOS-STD-###    Documentation/standard reference
GOS-AST-###    Asset
GOS-DOC-###    Documentation object
```

## Evidence model

A verification record should resolve to:

```text
Exact SHA
  → Workflow
  → Run
  → Job
  → Result
  → Step
  → Exit code
  → Log
  → Source
  → Root Cause
  → Minimal Fix
  → Commit
  → Rerun
  → Verification
```

Source presence alone is never implementation evidence.

## Ownership

The registry indexes technical objects; it does not replace repository ownership, architecture SSOTs, standards SSOTs or release gates.
