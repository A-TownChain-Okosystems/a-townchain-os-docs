# A-TownChain OS / KAI-OS — Ecosystem Documentation Hub

This repository is the documentation hub for the A-TownChain ecosystem. It contains system-level documentation, architecture references, roadmaps, whitepapers and historical archives. It is **not** the canonical source for every component implementation or for normative standards.

**Documentation refresh:** 2026-10-09  
**License:** Apache-2.0

## Start here

- [Current ecosystem status snapshot](docs/ECOSYSTEM_CURRENT_STATUS.md) — dated repository inventory, declared status labels, SSOT boundaries and evidence rules.
- [Agent policy](docs/AGENT_POLICY.md) — required change and evidence discipline.
- [Agent coordination](docs/AGENT_COORDINATION.md) — coordination notes; check freshness before relying on them.
- [Decision register](docs/DECISIONS_REGISTER.md) — architecture decisions, subject to current canonical sources and subsequent decisions.
- [Roadmap](docs/ROADMAP.md) · [Status](STATUS.md) · [Changelog](CHANGELOG.md)

## Current inventory

The GitHub inventory fetched on 2026-10-09 returned **33 repositories total: 26 not archived and 7 archived**. This is an inventory count, not a maintenance, implementation, audit or production-readiness assertion. See the [snapshot](docs/ECOSYSTEM_CURRENT_STATUS.md) for the list and evidence limitations.

Older statements in this repository that describe **128 repositories, 2 active and 126 archived** are stale and must not be used as current inventory. Historical sprint reports and test counts retain their original dates and do not establish current behavior.

## Architecture and source-of-truth rules

- **Normative standards:** [`atc-standards`](https://github.com/A-TownChain-Okosystems/atc-standards) owns the canonical standards registry. The local `docs/standards/` tree is a historical/reference snapshot only.
- **Blockchain core:** [`a-townchain`](https://github.com/A-TownChain-Okosystems/a-townchain).
- **ATC-VM:** canonical implementation at `a-townchain/components/vm`; do not create a competing production implementation.
- **Algorithm/consensus:** canonical implementation at `a-townchain/components/algorithm`.
- **ShivaCore kernel:** canonical source at `globus-os/modules/atc-shivacore/kernel/`; `atc-shivacore` is for supporting specifications, tooling and governance.
- **System integration/orchestration:** `a-townchain-os` and `a-townchain-ecosystem` coordinate integration, compliance and evidence; they do not replace component owners.
- **Aurora AI:** AI/service layer, not the ShivaCore trusted computing base, consensus authority, ATC-VM or canonical chain authority.

Treat repository-specific status files and exact-SHA CI evidence as authoritative for the component they cover. A system-level document cannot upgrade a component's implementation or verification state.

## Evidence and maturity language

- **SPECIFIED** means requirements or a design are documented.
- **IMPLEMENTED** means an implementation exists; correctness is not implied.
- **VERIFIED** requires relevant evidence tied to the exact source SHA, with the workflow run, job, step, exit status and logs.
- **AUDITED** and **PRODUCTION_READY** require their own applicable gates and evidence. They must not be inferred from a passing test, a compliance badge, a PR merge, or an older report.
- Preserve blocked and residual findings. Never weaken CI or branch-protection gates to make status appear green.

## Historical material

Files such as `REALITY_STATUS.md`, `STATUS.md`, archived wiki chapters and older sprint reports contain time-bound claims. Their dates and methods must be read in context. The previous `STATUS.md` snapshot is explicitly marked as stale; the current limited inventory is in [ECOSYSTEM_CURRENT_STATUS.md](docs/ECOSYSTEM_CURRENT_STATUS.md). Neither document substitutes for repository-specific exact-SHA evidence.

## Contributing

Make documentation changes on a dedicated branch and submit them through a pull request. Respect canonical source repositories, generated-file ownership, existing required checks and review policy. Do not copy mutable standards counts into this hub; link to the current generated state in `atc-standards` instead.

Copyright (c) 2026 A-TownChain-Okosystems. Licensed under Apache-2.0.
