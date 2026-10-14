# Master Architecture Evidence Matrix

**Purpose:** Evidence-only implementation coverage for the Master Architecture.
**Authority:** a-townchain-ecosystem/ARCHITECTURE.md defines the architectural component set. This document does not promote architecture claims into implementation claims.

## Evidence State Model
| State | Meaning |
|---|---|
| Architecture Defined | Component is defined in the Master Architecture. |
| Source Present | Authoritative source location has been identified. |
| Implemented | Behavior is implemented; source presence alone is insufficient. |
| Tested | Relevant tests exist and have been observed to pass where evidence is recorded. |
| CI-Verified | CI evidence is tied to the exact audited commit SHA. |
| E2E-Verified | The actual cross-component system path has been exercised. |
| Audited | Security/architecture/governance evidence exists for the component. |
| Release-Ready | All required project release gates are satisfied. |

**Rule:** file/module exists != implemented. CI PASS != E2E PASS != Audited != Release-Ready.

## Audited Baseline — 2026-09-27

| Component | Owner | Source of Truth | Implementation | Tests | CI @ exact SHA | E2E | Audit | Current blocker / note |
|---|---|---|---|---|---|---|---|---|
| Aurora Agent Runtime | aurora-ai | modules/atc-aurora-agents/src/; modules/atc-aurora-runtime/src/; modules/atc-aurora-core/src/ | Partial/Present | Present | PASS @ 87ed7bc87014708f3efad4c573871fe23f252b00 | Not verified | Not verified | Runtime exists; integration/E2E and release evidence remain open |
| Dialogue AI | Aurora / Genesis boundary | No authoritative runtime source identified on current aurora-ai/main | Not verified | Not verified | Not verified | Not verified | Not verified | Architecture-defined only |
| Quest AI | genesis-engine | Architecture path documented; authoritative Quest runtime source not identified | Partial / not verified | Not verified | FAIL @ 9adc30ea56e8219826c68fd17ca1b5ca012f938d | Not verified | Not verified | atc-genesis-runtime: ReplicationTransport trait bound failure |
| World AI | genesis-engine | No independent authoritative World-AI runtime identified | Partial / not verified | Not verified | FAIL on audited Genesis Engine SHA | Not verified | Not verified | Runtime ownership boundary still needs concrete implementation evidence |
| GlobusOS IPC | globus-os | modules/atc-shivacore/kernel/src/ipc.rs; capability.rs; system/ipc/src/lib.rs | Partial/Present | Present | FAIL @ e28a05542993a9be205a4df9bdd4f56137dbf390 | Not verified | Not verified | ShivaCore kernel build errors: x86_64, vec!, format! |
| Genesis Mod System | genesis-engine | No authoritative engine mod-runtime source identified | Not verified | Not verified | Not verified | Not verified | Not verified | Mod-related artifacts elsewhere do not establish Genesis Engine runtime ownership |
| ATC-VM | atc-vm | src/{assembler,context,lib,main,ops,vm}.rs | Present/Partial | Present | PASS @ 21132a49355eb5fd6ea4b8a7b4e4deb5d9be60f0 | Not verified | Security/governance evidence present; full audit not verified | Repository remains development status |

## Required Evidence Record
Every component audit must record:
1. Component
2. Authoritative owner repository
3. Exact source path(s)
4. Implementation evidence
5. Test name/path and result
6. Workflow -> run -> job -> step
7. Exact commit SHA
8. E2E path and result
9. Security / architecture / governance evidence
10. Release gate status
11. Open blocker / root cause
12. Minimal corrective action

## Coverage Backlog
The remaining Master Architecture components are Architecture Defined until individually audited. No implementation status is inferred from the architecture document.

### GlobusOS
Identity/Auth/Sessions; Wallet boundary; IPC/Capabilities; process/service lifecycle; VFS/storage; network/sockets/devices; graphics/compositor/GPU; audio; package management; A/B update/rollback; recovery/diagnostics; SDK.

### Aurora AI
ModelHub/provider abstraction; Agent Runtime; planning; conversation/dialogue intelligence; memory/context; RAG; tool/capability gateway; policy/approval/audit; multimodal; evaluation/observability; provenance/AI security; character/world/creature/companion/quest/content/script intelligence.

### Genesis Engine
ECS/runtime; world/simulation/streaming; character/NPC; creature/Shivamon; items/inventory/equipment; weapons/combat; levels/progression; economy/trading/rewards; quest runtime; dialogue runtime; world AI/event director; factions/reputation; multiplayer/networking; mod/extension; LiveOps; scripting; assets/rendering/animation/audio; security/anti-cheat/integrity; telemetry/analytics/replay; editor/SDK/build/packaging; deterministic runtime boundary.

### Genesis Production / Franchise
Franchise registry/lifecycle; World Factory; Character Factory; Lore/Canon; Quest/Narrative Factory; Economy/Gameplay Factory; Asset Intelligence; AI Content Pipeline; Creator/Multiplayer; Publishing; Commerce; Community; LiveOps; QA/Security/Evidence; provenance; Game Factory/Release Orchestration.

### A-TownChain
Node/networking; consensus; state; storage; indexer; explorer; wallet/SDK; contracts; ATCLang; ATC-VM; algorithm/economics; mining; oracle; ZKP; interoperability; compute; marketplace; launchpad.

### Developer / Creator Layer
ATC IDE; Genesis Editor; Engine SDK; OS SDK; ATCLang tooling; VM simulator; compliance/evidence tooling; CI/CD tooling; creator/mod tooling.

## Audit Rules
- Audit current main unless an explicit target SHA is supplied.
- Never substitute historical CI for current-SHA evidence.
- A successful unrelated workflow does not clear a failed system gate.
- Do not weaken gates to obtain green CI.
- Record root cause before proposing a fix.
- Prefer the smallest implementation fix that restores the intended contract.
- Preserve Standalone First, Ecosystem Second: each core repository remains the Source of Truth for its own build, tests, release, API/ABI and implementation.
- Ecosystem documentation records integration/evidence; it does not become a hidden implementation dependency.

## Next Audit Order
1. Resolve current Genesis Engine CI blocker and re-run at the resulting exact SHA.
2. Resolve current GlobusOS/ShivaCore CI blocker and re-run at the resulting exact SHA.
3. Audit Aurora Dialogue / Quest / World intelligence ownership and implementation boundaries.
4. Audit Genesis Mod / Extension and LiveOps runtime ownership.
5. Expand the same evidence record to every remaining Master Architecture component.
6. Establish E2E evidence for the canonical cross-layer path.