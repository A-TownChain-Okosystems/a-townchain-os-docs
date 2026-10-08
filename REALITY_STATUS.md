# REALITY_STATUS — historical snapshot retired

> **This file is no longer the current source of truth.** The previous contents were a dated collection of repository/code observations from August–September 2026. They remain available in this file's Git history, but their counts, test results and maturity claims must not be treated as current.

Use [ECOSYSTEM_CURRENT_STATUS.md](docs/ECOSYSTEM_CURRENT_STATUS.md) for the limited organization inventory checked on 2026-10-09. That snapshot records the GitHub inventory and repository-declared README status labels; it does **not** independently verify implementation, tests, security, conformance or production readiness.

For a component-specific state, consult the component's canonical repository and current status/evidence files. A claim of **VERIFIED** requires evidence tied to the exact source SHA, including the workflow run, job, step, exit status and logs. Historical green runs, README labels, audit scores and merge references are not sufficient by themselves.

Canonical ownership rules:
- Normative standards: [`atc-standards`](https://github.com/A-TownChain-Okosystems/atc-standards).
- Canonical ATC-VM implementation: `a-townchain/components/vm`.
- Canonical algorithm/consensus implementation: `a-townchain/components/algorithm`.
- Canonical ShivaCore kernel source: `globus-os/modules/atc-shivacore/kernel/`.
- `a-townchain-ecosystem` is the integration/compliance/evidence control plane, not a duplicate component implementation.
- `a-townchain-os` coordinates integration and validation; it does not replace component SSOT ownership.

Do not infer `IMPLEMENTED` from documentation alone or `VERIFIED` from implementation alone. Security audit, conformance, production readiness and Mainnet readiness are separate states. Missing or stale evidence must remain **NOT VERIFIED / UNKNOWN**. No CI or release gate may be weakened to make the reported state appear green.
