# MASTER_OVERVIEW — A-TownChain Okosystems Gesamtdokument

> **ATC-DOC: MASTER-OVERVIEW-001** · Status: ACTIVE · Stand: 2026-10-12 (Europe/Berlin)
> Konsolidiert: Wiki · Roadmap · Sprints · Todos · Komponenten- und Funktionskatalog · Quellen.
> Generator: Aurora (aurora-base44-superagent-6a1d37209d7f46ac065d39c5). Evidence-basiert nach vierstufiger Klassifikation (NORMATIVE → IMPLEMENTED → RESEARCH → LEGACY/UNVERIFIED).

## 1. Organisation

33 Repositories: **26 aktiv** (davon 1 privat: `demo-repository`) + **7 archiviert** (Referenz: atc-storage, atc-explorer, atc-indexer, atc-interop, atc-mining, atc-oracle, atc-wallet).

| Layer | Repos |
|---|---|
| GOVERNANCE | atc-standards, .github |
| DOCS / Wiki | a-townchain-os-docs (dieses Repo), a-townchain-ecosystem (Integrations-/Audit-Layer) |
| CORE | a-townchain, atclang, atc-shivacore, aurora-ai, globus-os, genesis-engine, genesis-chronicles |
| PROTOCOL SERVICES | atc-node, atc-vm, atc-zkp, atc-algorithm, atc-compute, atc-contracts |
| PRODUKTE / TOOLS | atc-sdk, atc-launchpad, atc-marketplace, atc-ide, atc-toolchain, atc-engineering, genesis-franchise-factory, demo-repository |

## 2. Wiki

Zentrale Wiki- und Dokumentationsflächen im Repo `a-townchain-os-docs` (HEAD `6b311f70d`):

- `wiki/` — Themen-Wikis: aistudio-wiki, atclang-wiki, atcnet, atcpkg-wiki, backend-wiki, blockchain-wiki, bootloader-wiki, ci-wiki, cli-wiki, contracts, dns-wiki, drivers-wiki, explorer-wiki, frontend-wiki, gateway, genesis-engine-wiki, franchise, franchise-factory u.a.
- Referenzmodelle (Root): ATC_ARCHITECTURE_REFERENCE_MODEL.md (v0.1.0, Draft bis Owner-Review), ATC_OS_ARCHITECTURE_REFERENCE_MODEL.md, ATC_VM_ARCHITECTURE_REFERENCE_MODEL.md, ARCHITECTURE.md, ECOSYSTEM.md, ECOSYSTEM_STATUS.md, MILESTONES.md, FILE_REGISTER.md, REALITY_STATUS.md, COMPONENT_PLAN.md, CONNECTION_MAP.md
- Governance-Wiki: Standards-Registry in atc-standards (533 Standards inkl. ATC-AAS, ATC-ENT, SC-001..020, ZKP; Verfassung ATC-STD-000 v1.2.0)
- Integrations-/Audit-Layer: a-townchain-ecosystem (Komponenten-/Funktionskatalog, Census, Readiness-Gates; Katalog aktuell auf PR #115)

## 3. Roadmap

### A-TownChain OS Meilensteine (v4.0, s. MILESTONES.md)

| MK | Name | Status |
|---|---|---|
| MK1–MK3, MK5, MK6 | Core OS, ATCLang, Blockchain/Konsens, Contract Engine, Network/Sharding | Erfüllt |
| MK4 | GCL v2.0 Communication | In Arbeit |
| MK7 | AI Agent Protocol (Sprint 3.0) | In Arbeit |
| MK8 | Security Audit | In Arbeit |
| MK9 | Mainnet Launch | Geplant Q4 2026 |
| MK10–MK15 | Genesis Engine, Multi-Chain, AAA, Franchise, MetaFactory, Civilization | In Arbeit / Geplant |

Bilanz lt. MILESTONES.md: 5/15 vollständig erfüllt. Wichtig: Hybrid-Konsens PoH+PoS+PoW ist RESEARCH-/Spezifikationsziel (Freeze offen), NICHT final/frozen. ATC-07 (Sharding) hat Governance-Drift (Issue #84 closed, Implementierung UNVERIFIED) — Bereinigung offen.

### Governance-Roadmap (Task DB, Stand 2026-10-12)

- Phase 1 weitgehend abgeschlossen: 103 Standards APPROVED (u.a. Verfassung v1.2.0, AAS-001..025, ENT-001..015, SC-001..020, README/MD), F-160 RESOLVED
- In Arbeit: README-/MD-Konformitäts-Rollout auf 26 Repos (#116/#117), Agent-Manifeste #111, F-001 Dependency Graph, F-002 Interface Registry, SC-Gate-Rollout #118
- ATC-AURORA-SWARM-001 (ASN, 47 REQ-ASW): CANDIDATE, §9-Freigabe ASW-G0 offen

### KAI-OS / GlobusOS

- KAI-OS Phase 2 (Prototyp): Kernel-Evidence, ShivaVM-Modultests offen
- GlobusOS: Boot laeuft auf Intel/AMD UEFI (PR #58/#59 verifiziert); Secure Boot, ACPI, Bare-Metal = RESIDUAL_NOT_ASSERTED; Boot-Evidence-Provenance PR #73 in Arbeit
- Offen: ATVM vs. ShivaVM kanonische Contract-VM (Governance-Entscheidung, v3.x)

### Notion

Master Roadmap (A-TownChain OS + KAI-OS, konsolidiert): Notion-Seite 373b826d-b85c-8125-ba83-f04995191bf0, Auto-Sync GitHub → Task DB → Notion aktiv.

## 4. Sprints (aktiv)

| Sprint | Repo | Zustand |
|---|---|---|
| Sprint 3.0 — AIP-001 Agent Interaction Protocol | a-townchain-os #80 | In Arbeit (Spezifikation) |
| Katalog-Completeness (PR #115, 98 Funktionen) | a-townchain-ecosystem | Draft, 19/20 Checks gruen, nur scanner-tests rot (wartende Workflow-Assertion, s. Owner-Aktionen) |
| GlobusOS Boot-Evidence-Provenance (PR #73) | globus-os | Draft; Secure Boot/ACPI/Bare-Metal RESIDUAL |
| ATCLang PR-Backlog (#14–#20) | atclang | 6/7 gruen, #19 wartet MD025-Policy-Pin |
| Sprint 3.3 Security / Sprint 4.0 Mainnet | a-townchain-os | Issues geschlossen (Vorbereitung MK8/MK9) |
| K1–K50 Kernel-Sprints | atc-shivacore / a-townchain-os | 60 Kernel-Module, ~2146 Rust-Tests, 30 ATCLang-Interfaces |

## 5. Todos

### Offene GitHub-Issues (46 org-weit, Stand 2026-10-12)

| Repo | Issue | Titel | Labels |
|---|---|---|---|
| .github | #12 | [AD-008] Sprach-Migration L5/L6: 8 Repos von Python/TypeScript auf Soll-Sprachen | — |
| a-townchain | #70 | P1: Verify deterministic A-TownChain runtime, persistence and multi-node consensus | P1 |
| a-townchain | #64 | SCR-0127: Resolve missing canonical components/contracts migration | architecture, migration |
| a-townchain | #31 | P0: Canonical Crypto Contract reconciliation — secp256k1 TX/Account, Ed25519 Identity | P0 |
| a-townchain | #20 | P1 audit: evidence binding and ZKP placeholder boundary | P1, security |
| a-townchain-ecosystem | #112 | P0: Vollständigen Quellcodeabgleich für Komponenten- und Funktionskatalog durchführen | — |
| a-townchain-ecosystem | #98 | Assurance Residual Register — PR #97 freeze record | — |
| a-townchain-ecosystem | #87 | P0: Fehlende Protokolle, Laufzeiten und Standardbibliothek systematisch schließen | P0 |
| a-townchain-ecosystem | #71 | Master Architecture: canonical A-TownChain layer model L0–L7 + Cross-Layer Control Pl | architecture |
| a-townchain-os | #124 | P1 audit: Integration Gate workspace path | audit |
| a-townchain-os | #122 | P1/P2 audit: workspace documentation and evidence drift | documentation, governance |
| a-townchain-os | #112 | [GATE-Iteration 2] Code closure verified — external production gates remain | — |
| a-townchain-os-docs | #41 | P0/P1: Census 2026-10-09 konsolidieren und Gap-Audit mit Exact-SHA-Evidence abschließ | documentation, audit |
| a-townchain-os-docs | #33 | CONSISTENCY-001: migrate active docs/config/wallet artifacts to canonical L1 values | consistency |
| atc-algorithm | #3 | Owner-Aktion: Dependency Graph auf Org-Ebene aktivieren — sonst bleibt P0-Gate rot (C | governance, owner-action |
| atc-engineering | #13 | P0: Organisationsweiter Funktions- und Komponenten-Census mit maschinenlesbarer Dokum | documentation, P0 |
| atc-engineering | #12 | P0: Machine-readable PR comments and gate-failure remediation loop | P0, automation |
| atc-engineering | #10 | P0: Red-Gate Triage and Fail-Closed Auto-Remediation Loop | P0, automation |
| atc-engineering | #2 | P0 — Configure ATC_FLEET_READ_TOKEN for complete private-repository fleet audit | P0, security |
| atc-launchpad | #3 | Owner-Aktion: Dependency Graph auf Org-Ebene aktivieren — sonst bleibt P0-Gate rot (C | governance, owner-action |
| atc-node | #10 | P2: RPC serve() beendet sich bei per-Verbindungs-Fehlern (EPIPE) — Devnet-Flake Root  | — |
| atc-shivacore | #19 | Fix DependencyGraph::dependencies() placeholder in LKM subsystem | bug, security |
| atc-standards | #79 | ci: restore pull-request creation permission for regenerate-views | governance |
| atc-standards | #77 | governance: enforce generated-artifact consistency for every generator | — |
| atc-standards | #76 | governance: audit main-HEAD provenance across organization | — |
| atc-standards | #70 | governance: resolve pre-existing dependency security pinning conflict | governance, security |
| atc-standards | #53 | P0: Reconcile frozen ATC-STD-600 transaction signature contract | P0 |
| atc-standards | #50 | L1-TX-001: Canonical Transaction Contract and deterministic serialization | — |
| atc-standards | #13 | Owner-Aktion: 14 CI-Gates pushen — 12× Determinism + Integration + Docs (Ziel 26/26 P | — |
| atc-vm | #8 | P1: implement enforced gas metering and canonical bytecode verification | P1, security |
| atclang | #12 | P1 audit blockers: reference VM security and Rust production boundary | security, P1 |
| atclang | #9 | P1 security: fail-closed Python reference VM crypto/auth/network stubs | security, P1 |
| aurora-ai | #37 | P0: Reconcile Aurora AI target architecture with canonical architecture and implement | documentation, architecture |
| aurora-ai | #35 | P1: Aurora AI isolated runtime, model inference and policy-bound tool execution | P1 |
| aurora-ai | #33 | P0: Aurora-AI-Grundfunktionen katalogisieren, absichern und SHA-verifizieren | — |
| genesis-engine | #16 | GENESIS: Species & Civilization Framework | architecture, gameplay |
| globus-os | #67 | feat(identity): implement username/password account lifecycle with 24-word recovery | — |
| globus-os | #63 | P0: Fail-closed GitHub API retrieval — reject empty/incomplete evidence | ci, governance |
| globus-os | #59 | governance: close post-merge assurance residuals for PR #58 | governance |
| globus-os | #57 | P1: GlobusOS user-space boot, service manager, storage, networking and recovery | P1 |
| globus-os | #51 | P0: Betriebssystem-Grundfunktionen katalogisieren, SSOT abgleichen und verifizieren | — |
| globus-os | #45 | P0: ShivaCore TCB completion — Architecture Delta, VMM, Build, Hardware and Evidence | P0 |
| globus-os | #44 | CI-RED: Rust CI "Check" fehlschlaegt am main (e28a055) — Kernel-Tests werden nie ausg | bug, kernel |
| globus-os | #20 | [P1][LKM][SECURITY] Enforce dependency ordering, fail-closed imports, and export/impo | P1, kernel |
| globus-os | #19 | P1: LKM export builder incorrectly registers exported symbols as imports | bug, P1 |
| globus-os | #18 | [P1][STUB][LKM] Replace DependencyGraph::dependencies placeholder with owned determin | P1, stub |

### Offene PR-Backlogs

| Repo | Offene PRs |
|---|---|
| .github | 2 |
| a-townchain | 17 |
| a-townchain-ecosystem | 33 |
| a-townchain-os | 4 |
| a-townchain-os-docs | 5 |
| atc-algorithm | 2 |
| atc-compute | 1 |
| atc-contracts | 6 |
| atc-engineering | 4 |
| atc-launchpad | 1 |
| atc-marketplace | 1 |
| atc-node | 3 |
| atc-sdk | 3 |
| atc-shivacore | 2 |
| atc-standards | 22 |
| atc-vm | 5 |
| atc-zkp | 2 |
| atclang | 1 |
| aurora-ai | 4 |
| demo-repository | 1 |
| genesis-chronicles | 1 |
| genesis-engine | 14 |
| genesis-franchise-factory | 1 |
| globus-os | 34 |

Groeßte Backlogs: globus-os (34), a-townchain-ecosystem (33, inkl. #115 Katalog), atc-standards (22), a-townchain (17), genesis-engine (14). Review-Last konzentriert auf diese fuenf Repos.

### Task DB — offene/blocked (Auszug)

- Blocked: SCR-0086 Evidence-Promotion (Owner workflow-Scope), INFRA-DEPGRAPH-001 Dependency Graph auf 6 Repos (UI-Aktivierung, RESIDUAL/BLOCKED)
- Open: ATVM-vs-ShivaVM-Entscheidung, Kernel-Evidence-Runs, Exact-SHA-Evidence Data/Network/Application, ATC-07-Drift, Konsens-Freeze v3.0.0, SCR-0007 REQ-ID-Rollout, F-003..F-007

### Owner-Aktionen (nicht agent-seitig loesbar)

1. **2-Zeilen-Workflow-Edit** (Ecosystem-CI): `assert active_scanned == 26` ersetzen durch `active_blocked == 0` + `active_scanned + active_excluded_private == active_expected` (Vorlage: PR #115 Kommentar issuecomment-6115900594; alternativ demo-repository public stellen oder Census-Credential provisionieren)
2. **MD025-Policy-Pin**: .github/workflows feat/org-markdownlint Pin 1c198f3→61f1941 (ATCLang #19 + markdownlint-Fehlalarme)
3. **Dependency Graph org-weit aktivieren** (UI: org Security Configuration → Dependency scanning → alle Repos), danach Re-Run der dependency-review-Jobs
4. **PR-Reviews**: #115 (Katalog) und ATCLang #14–#20
5. **ASW-G0 §9-Freigabe** ATC-AURORA-SWARM-001
6. **SCR-0086**: Branch-Protection-Blocker fuer Evidence-Commits loesen

## 6. Komponenten- und Funktionskatalog

Ort: `a-townchain-ecosystem/docs/architecture/ATC-COMPONENT-FUNCTION-CATALOG-001.json` + `.md` — aktuell auf **PR #115 (Draft, Head 8a4708794)**, noch nicht auf main.

- **30 Komponenten / 98 Funktionen** (63 erstes Supplement → 71 ATCLang-Pass → 98 Komponenten-/Spezifikations-Pass)
- Source-Bindings: **66 census-verifizierte Lexical-Matches** (funktionale Verifikation ausstehend), **27 nicht im Lexical-Scan gefunden** (12 Spec-/Doc-/Data-Bindings + 15 ATC-VM-Private-Symbole PRIVATE_SYMBOL_NOT_IN_PUBLIC_CENSUS), **5 explizite Blocked-Records** (archiviert/Referenz)
- repository_coverage: 33 Repos (26 aktiv, 7 archiviert), fail-closed Status je Eintrag
- Reconcile-Status: RESIDUAL (0 blockierende Findings); Validator PASS_STRUCTURAL_ONLY; Census-Basis a-townchain a70b16f2d (VM-Resource-Bounds-Fixes PR #71)
- Katalog-Baselines sind SHA-spezifisch; bei Repo-Bewegung (wie a-townchain 0419b444d→a70b16f2d) ist ein Refresh erforderlich (vorbereitet, uncommittet auf dem PR-Branch)
- Funktionsliste im Detail: Katalog-MD (Abschnitt je Komponente, FN-IDs mit Source-Pfad, Symbol, exaktem SHA, Evidence-URL)

## 7. Quellen (Evidence-Basis 2026-10-12)

| Repo | main SHA |
|---|---|
| .github | `af5d803da` |
| a-townchain | `a70b16f2d` |
| a-townchain-ecosystem | `5a66f276a` |
| a-townchain-os | `3f85a6e61` |
| a-townchain-os-docs | `6b311f70d` |
| atc-algorithm | `5a5628044` |
| atc-compute | `d0831dfb7` |
| atc-contracts | `a7c8b417f` |
| atc-engineering | `2e91ec61a` |
| atc-ide | `2a17feda7` |
| atc-launchpad | `bd1a48c69` |
| atc-marketplace | `5c3bab7da` |
| atc-node | `a817db43a` |
| atc-sdk | `34eef3459` |
| atc-shivacore | `21c997696` |
| atc-standards | `dc0ff69d8` |
| atc-toolchain | `82fe4a238` |
| atc-vm | `21132a493` |
| atc-zkp | `ffa8ec109` |
| atclang | `21c595dc3` |
| aurora-ai | `1b5ff1824` |
| demo-repository | `d9b0da9c9` |
| genesis-chronicles | `a3f417b23` |
| genesis-engine | `8326eb53e` |
| genesis-franchise-factory | `673fa5603` |
| globus-os | `c7337ef4f` |

- Census: tools/run_org_source_census.py (a-townchain-ecosystem), letzte Vollprüfung 2026-10-12: RESIDUAL, 25 gescannt + 1 privat ausgeschlossen (demo-repository, EXCLUDED_PRIVATE_NO_CI_ACCESS), 0 blockiert; 4.925 Quelldateien, 35.581 Kandidaten-Symbole
- Katalog-Audit: docs/audit/ATC-CATALOG-COMPLETENESS-AUDIT-001.md (PR #115, inkl. Ausschluss-Rationale R-1)
- Standards: Registry atc-standards/registry/standards.yaml (SSOT), Verfassung ATC-STD-000 v1.2.0, 103 APPROVED
- Notion Master Roadmap: 373b826d-b85c-8125-ba83-f04995191bf0
- MILESTONES.md / REALITY_STATUS.md / FILE_REGISTER.md (dieses Repo, SHA 6b311f70d)
- Offene Issues: Live-Scan aller 26 aktiven Repos via GitHub API (2026-10-12)

## Pflege

Dieses Dokument wird bei Sync-Laeufen aktualisiert (GitHub → Task DB → Notion Automation). Aenderungen nur mit Evidence; Statusaussagen strikt nach vierstufiger Evidence-Klassifikation. Neue Todos aus Issues/Findungen fließen automatisch in Abschnitt 5 ein.
