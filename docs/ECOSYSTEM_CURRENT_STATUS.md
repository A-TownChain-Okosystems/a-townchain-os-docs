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
| `atc-ide` | Experimental; README says no production build or application test suite. Markdown Lint workflow exists; PR [#2](https://github.com/A-TownChain-Okosystems/atc-ide/pull/2) corrects the inaccurate “no CI gates” statement. |
| `atc-launchpad` | Development; README describes initial skeleton. |
| `atc-marketplace` | README is a short restored/legacy description; current maturity is not established by this inventory. |
| `atc-node` | Development; no production readiness implied by documented commands or tests. |
| `atc-sdk` | Development; protocol and VM semantics remain owned by canonical components. |
| `atc-shivacore` | Development / NOT_READY; supporting specifications and governance only. Canonical kernel source is in `globus-os/modules/atc-shivacore/kernel/`. |
| `atc-standards` | Canonical normative standards source. Read its generated state block and registry at the checked-out revision; do not copy mutable metrics into this hub. |
| `atc-toolchain` | Development / R1 skeleton per README; exact-SHA verification not established in this snapshot. A license-description wording correction is proposed in PR [#2](https://github.com/A-TownChain-Okosystems/atc-toolchain/pull/2). |
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

> **Standards-state note:** the checked-in generated `atc-standards` README/STATUS snapshot inspected on 2026-10-09 is generated at `2026-10-07 22:43 UTC+2`, State ID `ATC-STATE-20261007-ca5c3b44`, registry SHA-256 `ca5c3b445fc45382417d7b3013a8229fc736aadcc6f2a3c8420736bdd316610b`. The generated views were not manually edited in this refresh. Regeneration must follow the canonical generator and current registry source.

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

## PR-head CI evidence sampled during this refresh

The observations below are tied to the listed PR head SHAs and workflow runs returned by GitHub on 2026-10-09. A pass applies only to that workflow on that SHA; it does not independently verify the whole repository or product.

| Repository / PR | PR head SHA | Observed result |
|---|---|---|
| [a-townchain-os-docs #40](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/pull/40) | See current PR head | Self-referential status: check the linked PR's required checks for its latest head; each documentation commit restarts the applicable checks. |
| [a-townchain #65](https://github.com/A-TownChain-Okosystems/a-townchain/pull/65) | `6a98d7fdd0eaca3d87e366c0f1731410f2e8df2f` | Tests, SDK Build, Governance, Dependency Review passed; Determinism and Code Quality failed; CodeQL in progress. |
| [atc-launchpad #8](https://github.com/A-TownChain-Okosystems/atc-launchpad/pull/8) | `034bb5dfdbf3888dc9f8f3946604d835dc44b244` | Governance, Dependency Review, RustSec and CodeQL passed. |
| [atc-vm #16](https://github.com/A-TownChain-Okosystems/atc-vm/pull/16) | `6693286f3ccd0c7fd3db4865937e3ae8ea0c7b57` | Test Suite, Governance and RustSec passed; Dependency Review queued and CodeQL in progress; this is not VM production verification. |
| [atc-zkp #7](https://github.com/A-TownChain-Okosystems/atc-zkp/pull/7) | `5807d4426f27f58b0348068d62e4ad52384125cd` | Test Suite, Governance, Dependency Review, RustSec, CodeQL and ZKP Quality Gates passed; Determinism failed. |
| [atc-algorithm #13](https://github.com/A-TownChain-Okosystems/atc-algorithm/pull/13) | `b13d66d324da6056238919489d2d5ca20922f591` | No PR-triggered workflow run returned for this latest head in the current query; prior head `42b169c30712ca64255f8f5b808b1f349c95ede6` had Test Suite and Determinism failures. |
| [a-townchain-ecosystem #105](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/pull/105) | `05a247f1b7052a8aa3626d922f3338767e550cd1` | Workflows queued at observation time. |
| [globus-os #46](https://github.com/A-TownChain-Okosystems/globus-os/pull/46) | `d6f168ec4fb4af80230716c62dc5e3e44b18360d` | Governance, Dependency Review, RustSec, CodeQL and SDK Build passed; GlobusOS System CI, ATC Test Suite and Rust CI failed. |
| [atc-toolchain #2](https://github.com/A-TownChain-Okosystems/atc-toolchain/pull/2) | `42bc760872de76cf1df36cfca05738d20ea7a4ff` | Governance, Toolchain Validation and Smoke Tests passed. |
| [atclang #21](https://github.com/A-TownChain-Okosystems/atclang/pull/21) | `329b3723989f1e5586f6b172683e34c3e48caaa3` | Dependency Review, RustSec, Governance, CodeQL, Determinism, Code Quality, Test Suite and independent audit passed. |
| [atc-node #13](https://github.com/A-TownChain-Okosystems/atc-node/pull/13) | `41f88d77a0893e5d1f58031a96ed943d3e3f1d7d` | Governance, Dependency Review, RustSec and ATC-STD-600 Conformance passed; Determinism failed; CodeQL in progress. |
| [atc-contracts #12](https://github.com/A-TownChain-Okosystems/atc-contracts/pull/12) | `2c5777a4e7ee6b11f50754ba048d72ee1dc6c495` | Governance, Dependency Review, CodeQL and Determinism passed; Test Suite failed. |
| [atc-marketplace #12](https://github.com/A-TownChain-Okosystems/atc-marketplace/pull/12) | `b644aea4a3736c3423db6f3b26e81bd2eb11dd72` | Governance, Dependency Review and CodeQL passed; no test workflow was returned in this query. |
| [aurora-ai #31](https://github.com/A-TownChain-Okosystems/aurora-ai/pull/31) | `5b68ba32cd59a795197bf5a93dce5f4084c4048f` | SDK Build, Governance, Dependency Review and CodeQL passed. |
| [atc-compute #8](https://github.com/A-TownChain-Okosystems/atc-compute/pull/8) | `3cfed16018252c3700a6acaba1234fe6ba45e821` | Code Quality, Governance, Dependency Review, RustSec and CodeQL passed. |
| [atc-sdk #14](https://github.com/A-TownChain-Okosystems/atc-sdk/pull/14) | `63806a1b8d50bd010a771e912f1ccbd13f057942` | Test Suite, SDK Build, Governance, Dependency Review and RustSec passed; CodeQL in progress. |
| [genesis-engine #30](https://github.com/A-TownChain-Okosystems/genesis-engine/pull/30) | `0bd0ce268c84b97001785fcd65ed8e400d05889c` | SDK Build, Determinism, Governance, Dependency Review and CodeQL passed; Enterprise CI in progress. |
| [atc-shivacore #24](https://github.com/A-TownChain-Okosystems/atc-shivacore/pull/24) | `17762d8fa00055e6d10894f0cc50cf66d238ceb8` | SDK Build, Governance, Dependency Review and RustSec passed; Determinism failed; CodeQL in progress. |
| [a-townchain-os #130](https://github.com/A-TownChain-Okosystems/a-townchain-os/pull/130) | `44188fffb63fc5518bead9fe31afaa881550051c` | Release Pipeline, Integration Gate and Dependency Review passed; Governance and PR Validation failed. |
| [genesis-chronicles #15](https://github.com/A-TownChain-Okosystems/genesis-chronicles/pull/15) | `4983e1c97d1ed4c371fc639dddb0822665311028` | Governance, Dependency Review and Code Quality passed; CodeQL in progress. |
| [atc-ide #2](https://github.com/A-TownChain-Okosystems/atc-ide/pull/2) | `8320f1faeee91e66d29038a7f45d623778f764e5` | Markdown Lint in progress; this is not application test or production-build evidence. |
| [genesis-franchise-factory #7](https://github.com/A-TownChain-Okosystems/genesis-franchise-factory/pull/7) | `d6edb8943c11429678b529dccca0a5d6d1f2af00` | Governance, Dependency Review, CodeQL, Test Suite and Code Quality queued. |

**Coverage limit:** This is a targeted CI snapshot for PR heads touched during this refresh, not a complete CI audit of all 26 non-archived repositories. No CI state is inferred for repositories absent from this table.

## Refresh procedure

1. Fetch the current repository inventory and exclude archived repositories from the active refresh queue.
2. For every non-archived repository, inspect `README.md`, `STATUS.md`, `ARCHITECTURE.md`, `ROADMAP.md`, canonical registry entries and exact-SHA evidence.
3. Resolve ownership against the SSOT boundaries above before changing claims.
4. Commit documentation changes on a dedicated branch and submit a PR; do not edit generated registry views manually.
5. Re-run required workflows after a documentation commit; a previous SHA's result is not evidence for the new head.
6. Keep claims conservative where evidence is missing, inaccessible, stale, or not tied to the current source SHA.
