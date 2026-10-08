# A-TownChain Architecture Reference — Current Boundaries

**Snapshot:** 2026-10-09  
**Type:** system-level architecture index; not a release certification or substitute for canonical specifications.

## Source-of-truth map

| Domain | Canonical source | Responsibility |
|---|---|---|
| Normative standards | [atc-standards](https://github.com/A-TownChain-Okosystems/atc-standards) | Normative requirements, registry and generated compliance views |
| ATCLang | [atclang](https://github.com/A-TownChain-Okosystems/atclang) | Language/compiler and artifact validation; current README describes a Rust-only active path |
| Blockchain core | [a-townchain](https://github.com/A-TownChain-Okosystems/a-townchain) | Chain state, protocol integration and canonical component tree |
| ATC-VM | `a-townchain/components/vm` | Canonical VM implementation |
| Algorithm / consensus | `a-townchain/components/algorithm` | Canonical algorithm and consensus implementation; consensus choices must follow approved specs |
| Node runtime | [atc-node](https://github.com/A-TownChain-Okosystems/atc-node) | Node runtime/distribution layer; not a second protocol/VM authority |
| ShivaCore kernel / TCB | [globus-os/modules/atc-shivacore/kernel/](https://github.com/A-TownChain-Okosystems/globus-os/tree/main/modules/atc-shivacore/kernel) | Capability enforcement, memory/address-space primitives, scheduling, IPC and interrupt/timer boundaries |
| GlobusOS | [globus-os](https://github.com/A-TownChain-Okosystems/globus-os) | Userspace, system services and hardware integration above the TCB |
| Aurora AI | [aurora-ai](https://github.com/A-TownChain-Okosystems/aurora-ai) | AI services and agents; no kernel, consensus or chain authority |
| Toolchain | [atc-toolchain](https://github.com/A-TownChain-Okosystems/atc-toolchain) | Build, CLI, generators and governance tooling |
| Integration control plane | [a-townchain-ecosystem](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem) | Integration, compliance and evidence aggregation, not duplicate implementations |
| System orchestrator | [a-townchain-os](https://github.com/A-TownChain-Okosystems/a-townchain-os) | Cross-repository integration and validation |
| Documentation hub | This repository | Architecture references, wiki, historical material and system-level status snapshot |

## Runtime/security boundary

```text
User intent / application request
        ↓
Aurora AI (proposal, inference, planning)
        ↓
Policy decision
        ↓
Capability check + explicit approval when required
        ↓
Tool / service boundary
        ↓
Authoritative system service or canonical runtime
        ↓
Auditable result / evidence
```

AI may propose or orchestrate work, but must not bypass capability enforcement, policy, human approval requirements, deterministic chain rules or canonical VM behavior.

## Status and evidence model

- **SPECIFIED:** requirements/design are documented.
- **IMPLEMENTED:** code/artifact exists; no correctness inference.
- **VERIFIED:** required checks pass for the exact claimed SHA, with Run → Job → Step → exit status/log evidence.
- **BLOCKED / RESIDUAL:** unresolved finding; remains open until corrected and rerun.
- **AUDITED / CONFORMANT / PRODUCTION_READY / MAINNET_READY:** separate claims, each requiring its own applicable evidence.

A green unrelated workflow, historical audit score, PR merge reference, repository badge or README assertion does not upgrade the current source SHA. Never weaken CI, branch protection, security or release gates to manufacture a passing status.

## Important non-claims

- This model does not claim all component interfaces are connected end-to-end.
- It does not decide an unfinalized consensus algorithm.
- It does not certify kernel security, hardware boot, AI safety, standards conformance or production readiness.
- Component-specific blockers and current test evidence must be read from the canonical repository and exact-SHA workflow.

## Supporting references

- [Current ecosystem status snapshot](ECOSYSTEM_CURRENT_STATUS.md)
- [ShivaCore architecture boundary](architecture/SHIVACORE_KERNEL_ARCHITECTURE.md)
- [AI layer boundary](architecture/AI_LAYER.md)
- [Consensus source-of-truth note](architecture/CONSENSUS.md)
- [ATCLang compiler boundary](architecture/ATCLANG_COMPILER.md)
- [Cluster/deployment verification criteria](CLUSTER_ARCHITECTURE.md)
- [Normative standards SSOT](https://github.com/A-TownChain-Okosystems/atc-standards)
