# 🧠⛓️ A-TownChain OS / KAI-OS — Offizielle Dokumentation

> ## 🤖 Für KI-Agenten — Pflichtlektüre vor jeder Änderung
> 1. [`docs/AGENT_POLICY.md`](docs/AGENT_POLICY.md) — verbindliche Regeln, Reality-Check, Konsolidierungsziel
> 2. [`docs/AGENT_COORDINATION.md`](docs/AGENT_COORDINATION.md) — wer arbeitet gerade woran, Agent-IDs
> 3. [`docs/DECISIONS_REGISTER.md`](docs/DECISIONS_REGISTER.md) — verbindliche Architektur-Entscheidungen

> Dezentrales, KI-gesteuertes Blockchain-Betriebssystem
> **A-TownChain OS** (technisch) · **KAI-OS** (Produktname)

Dies ist der **kanonische Dokumentations-Hub** des A-TownChain-Ökosystems.

**Version:** 1.0.0 | **Stand:** 03.09.2026 | **Lizenz:** All Rights Reserved
**Autor:** Michael Wroblewski | **Agent:** Aurora (Base44 Superagent)

---

## Architektur-Policy v1.0

- 🔴 **ATCLang First** — Kern-Logik in ATCLang (ATC-99)
- 🔴 **SHA-256** — TX-Hashing (AD-001 RESOLVED)
- 🔴 **Chain-ID 9000** — Proprietäre Non-EVM Chain-ID (AD-004 RESOLVED)
- 🔴 **Dual-Repo-Modell** — Code in `a-townchain-os`, Doku hier (Mandat AGENT_POLICY/AD-89)

## Metriken (Stand 03.09.2026)

| Metrik | Wert |
|--------|------|
| Repositories | 128 total — **2 aktiv, 126 archiviert** (Konsolidierung Sept 2026) |
| Hub-Dateien | 1.700+ (kanonische Doku + Archive) |
| ATC-Standards | ATC-01…35 (5 Tiers, Tier 5 aktiv) + ATC-97 (AIP-Entwurf) + ATC-99 (ATCLang First) |
| ShivaCore Rust-Kernel | K29 abgeschlossen — 30 Module, 367/367 Tests grün |
| Monorepo | 2.237 Dateien, 60 Module, VERSION 1.0.0, 0 Audit-Fehler |
| Interne Links | 873 geprüft, 0 kaputt |
| Mainnet-Launch | 15.09.2026 |

## K-Sprint-Status

| Phase | Status |
|-------|--------|
| K3–K29 — ShivaCore Rust-Kernel | ✅ ABGESCHLOSSEN (367 Tests) |
| Konsolidierung 128→2 | ✅ ABGESCHLOSSEN (Migration, Datenverlust-Check, Archivierung) |
| K30 — Validator-Nodes (#70) | 🔵 AUSSTEHEND |
| K31 — Genesis Block (#71) | 🔵 AUSSTEHEND |
| #69 — Security (Dependabot 70 Vulns) | 🔴 OFFEN — vor K30 priorisiert |
| K32 — Pre-Launch Verify · K33 — External Audit | 🟡 GEPLANT |

## Quick Links

- [Roadmap](docs/ROADMAP.md) | [Status](STATUS.md) | [TODO](TODO.md) | [Changelog](CHANGELOG.md)
- [Standards Registry](docs/standards/STANDARDS_REGISTRY.md) | [KAI-OS Wiki](docs/kai-os-wiki.md)
- [Launch-Checkliste & Roadmap](docs/project/) | [Whitepaper](docs/whitepaper/)
- [Compliance (BaFin)](docs/compliance/) | [Lizenz-Übersicht](docs/LICENSING_OVERVIEW.md)
- [Agent Master Rules](AGENT_MASTERRULES.md) | [ATCLang First Policy](ATCLANG_FIRST.md)

## Hub-Struktur

```
docs/
├── standards/       # ATC-01…35 + ATC-LIC/ATS-LIC (kanonisch)
├── wiki/            # KI-OS Wiki-Struktur (Kapitel je Modul)
├── whitepaper/      # Whitepaper + Einzelkapitel
├── compliance/      # BaFin-Konformität, Richtlinien
├── project/         # Master-Component-Plan, Launch-Checkliste
├── issues/ sprints/ # Issue- und Sprint-Dokumentation
├── archive/
│   ├── wiki/        # 65 Modul-Wikis ( verschmolzen, mit Index)
│   └── kai-os-legacy/
└── monorepo-legacy/ # Historische Monorepo-Docs (eingefroren)
```

## Repository-Struktur (Dual-Repo-Modell)

```
a-townchain-os/        # Code-Monorepo: 60 Module, ShivaCore K29, VERSION 1.0.0
a-townchain-os-docs/  # Doku-Hub (dieses Repo): Standards, Wiki, Whitepaper, Archive
```

---

*A-TownChain OS / KAI-OS · v1.0.0 · Non-EVM · SHA-256 · Chain-ID 9000 · ATCLang First*

## Lizenzmodell

A-TownChain etabliert ein **monetarisiertes, autonomes Open-Source-Ökosystem**.

**"Code is Law" auf Lizenzebene:** Unlizenzierter Code wird von der ATVM physisch gar nicht erst ausgeführt.

- **ATC-LIC** — Smart Contract Lizenzen mit automatischer Royalty-Durchsetzung
- **ATS-LIC** — System & Hardware Lizenzen mit TPM-Verifikation
- **Compliance-Handbuch** — BaFin-konforme Dokumentation

→ [Lizenz-Übersicht](docs/LICENSING_OVERVIEW.md) | [ATC-LIC](docs/standards/ATC-LIC-SMART_CONTRACT_LICENSE.md) | [ATS-LIC](docs/standards/ATS-LIC-SYSTEM_HARDWARE_LICENSE.md) | [Compliance-Handbuch](docs/compliance/COMPLIANCE_HANDBUCH.md)

Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.

## Verwandte Vision-Projekte

- [`atc-genesis-engine`](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-genesis-engine) — Vision-/Konzept-Modul für eine potenzielle zukünftige Game-Engine (Genesis Engine) und deren Ausbaustufen. Reines Konzeptmaterial, kein aktueller Teil der A-TownChain-Kernentwicklung.

---

**Last Updated:** 2026-09-03 by Aurora (Base44 Superagent)
