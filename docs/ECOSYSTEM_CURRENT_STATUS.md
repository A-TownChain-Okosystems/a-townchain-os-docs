# Ecosystem Current Status Snapshot

**Snapshot date:** 2026-10-09  
**Source:** GitHub repository inventory and the current README files fetched for the listed repositories.  
**Evidence class:** inventory checked; project maturity labels below are repository-declared statements, **not an independent implementation or CI verification**.

## Inventory

The organization inventory returned **33 repositories: 26 not archived and 7 archived**. “Not archived” is a GitHub repository flag, not proof that a repository is actively maintained or production-ready.

### Not archived (26)

| Repository | README-declared status / current documentation note |
|---|---|
| `.github` | Organization policy and generated repository-scope inventory; generated counts must not be hand-edited. |
| `a-townchain` | Development; README explicitly says not production-ready and Mainnet NO-GO. |
| `a-townchain-ecosystem` | Architecture/integration and evidence control plane; current architecture PRs are proposals until merged and evidenced. |
| `a-townchain-os` | Development; integration/orchestration repository, not a substitute for canonical component repositories. |
| `a-townchain-os-docs` | Documentation hub. Older inventory and status material is stale; see this snapshot. |
| `atc-algorithm` | Development; README describes MVP/skeleton, not a production consensus engine. |
| `atc-compute` | Development/rebuild-era description in README; production readiness not established here. |
| `atc-contracts` | README labels M6 as CLAIMED, not VERIFIED. |
| `atc-engineering` | README reports partial implementation; not a production platform. |
| `atc-ide` | Experimental; README says no production build, test suite, or CI gates. |
| `atc-launchpad` | Development; README describes initial skeleton. |
| `atc-marketplace` | README is a short restored/legacy description; current maturity is not established by this inventory. |
| `atc-node` | Development; no production readiness implied by documented commands or tests. |
| `atc-sdk` | Development; protocol and VM semantics remain owned by canonical components. |
| `atc-shivacore` | Development / NOT_READY; supporting specifications and governance only. Canonical kernel source is in `globus-os/modules/atc-shivacore/kernel/`. |
| `atc-standards` | Canonical normative standards source. Read its generated state block and registry at the checked-out revision; do not copy mutable metrics into this hub. |
| `atc-toolchain` | Not independently assessed in this snapshot; consult its current status and exact-SHA evidence. |
| `atc-vm` | Development; README describes an R1 skeleton. Canonical production VM implementation is `a-townchain/components/vm`; this repo must not be treated as a competing implementation. |
| `atc-zkp` | Development / R1 skeleton according to README. |
| `atclang` | Development; README explicitly says not production-ready. |
| `aurora-ai` | Development; no production claim. Aurora AI is not the ShivaCore TCB or canonical chain/consensus authority. |
| `demo-repository` | GitHub demo/template repository; no ecosystem product maturity claim inferred. |
| `genesis-chronicles` | Development; game/reference project, not evidence of platform production readiness. |
| `genesis-engine` | Development. |
| `genesis-franchise-factory` | Experimental (R1) according to README. |
| `globus-os` | Development; operating-system implementation is not thereby verified or production-ready. |

### Archived (7)

`atc-explorer`, `atc-indexer`, `atc-interop`, `atc-mining`, `atc-oracle`, `atc-storage`, `atc-wallet`.

Archive status is taken from the repository inventory response on the snapshot date. Archived repositories are not targets for active documentation refresh unless they are explicitly reactivated.

## Canonical ownership / SSOT boundaries

- **Normative standards:** `atc-standards` is the source of truth. `a-townchain-os-docs/docs/standards/` is an archival/reference snapshot, not a competing normative registry.
- **Blockchain core:** `a-townchain`.
- **Canonical ATC-VM implementation:** `a-townchain/components/vm`. A separate repository must not silently become a second production VM.
- **Canonical algorithm/consensus implementation:** `a-townchain/components/algorithm`.
- **Canonical ShivaCore kernel source:** `globus-os/modules/atc-shivacore/kernel/`. `atc-shivacore` owns supporting specifications/governance, not a competing kernel.
- **Integration and evidence control plane:** `a-townchain-ecosystem`; it must not duplicate component implementations.
- **System orchestration:** `a-townchain-os`; it coordinates integration and validation, not ownership of every component implementation.
- **Aurora AI:** AI/service layer. It does not replace ShivaCore security enforcement, consensus, ATC-VM, or canonical chain authority.

## Evidence semantics

- **IMPLEMENTED** means code or a documented artifact exists; it does not mean the behavior has passed verification.
- **VERIFIED** requires evidence tied to the exact commit SHA being claimed, including the relevant workflow run, job, step, exit status and logs. A PR merge reference, a badge, a historical green run, or README wording alone is insufficient.
- **BLOCKED / RESIDUAL** findings remain visible until corrected and rechecked. A green unrelated workflow does not close them.
- Do not weaken required CI gates to make a status appear green. Do not label a component audited, production-ready, or Mainnet-ready unless the relevant release gates and evidence support that exact claim.
- This document does not independently run repository CI and does not upgrade any project state to VERIFIED.

## Stale source files

- The README previously stated **128 repositories, 2 active and 126 archived** and carried a 2026-09-03 update date. Those numbers do not match the organization inventory fetched on 2026-10-09.
- `STATUS.md` contains an auto-generated 2026-08-04 snapshot with obsolete metrics and superseded issue references; it must not be used as current live status.
- The previous `REALITY_STATUS.md` contained dated historical observations and claimed to be authoritative. It is now a deprecation notice; its previous contents remain in Git history. Use this snapshot only for the limited inventory and README-declared labels stated above.
- Repository-specific implementation state must be refreshed from each repository's canonical status/evidence files and exact-SHA CI records before any claim is upgraded.

## Refresh procedure

1. Fetch the current repository inventory and exclude archived repositories from the active refresh queue.
2. For each active repository, inspect `README.md`, `STATUS.md`, `ARCHITECTURE.md`, `ROADMAP.md`, the canonical registry entries and exact-SHA evidence.
3. Resolve ownership against the SSOT boundaries above before changing claims.
4. Commit documentation changes on a dedicated branch and submit a PR; do not edit generated registry views manually.
5. Keep claims conservative where evidence is missing, inaccessible, stale, or not tied to the current source SHA.
