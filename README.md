# 🧠⛓️ A-TownChain OS / KAI-OS — Offizielle Dokumentation

> ## 🤖 Für KI-Agenten — Pflichtlektüre vor jeder Änderung
> 1. [`docs/AGENT_POLICY.md`](docs/AGENT_POLICY.md) — verbindliche Regeln, Reality-Check, Konsolidierungsziel
> 2. [`docs/AGENT_COORDINATION.md`](docs/AGENT_COORDINATION.md) — wer arbeitet gerade woran, Agent-IDs
> 3. [`docs/DECISIONS_REGISTER.md`](docs/DECISIONS_REGISTER.md) — verbindliche Architektur-Entscheidungen

> Dezentrales, KI-gesteuertes Blockchain-Betriebssystem
> **A-TownChain OS** (technisch) · **KAI-OS** (Produktname)

Dies ist der **kanonische Dokumentations-Hub** des A-TownChain-Ökosystems.

**Version:** 1.0.0 | **Stand:** 03.09.2026 | **Lizenz:** Apache-2.0
**Autor:** Michael Wroblewski | **Agent:** Aurora (Base44 Superagent)

---

## Architektur-Policy v1.0

- 🔴 **ATCLang First** — Kern-Logik in ATCLang (ATC-99)
- 🔴 **SHA-256** — TX-Hashing (AD-001 RESOLVED)
- 🔴 **Chain-ID 658467** — Apache-2.0e Non-EVM Chain-ID, ASCII 'ATC' (AD-004 RESOLVED)
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
| Mainnet-Launch | per AD-023 kein Termin (qualitaetsgetrieben) |

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
- [Software Wiki — Master Index](docs/software-wiki/README.md) | [Master Registry](docs/software-wiki/MASTER_REGISTRY.md)
- [Standards Registry](docs/standards/STANDARDS_REGISTRY.md) | [KAI-OS Wiki](docs/kai-os-wiki.md)
- [Launch-Checkliste & Roadmap](docs/project/) | [Whitepaper](docs/whitepaper/)
- [Compliance (BaFin)](docs/compliance/) | [Lizenz-Übersicht](docs/LICENSING_OVERVIEW.md)
- [Agent Master Rules](AGENT_MASTERRULES.md) | [ATCLang First Policy](ATCLANG_FIRST.md)

## Hub-Struktur

```
docs/
├── software-wiki/    # aktueller Globus Software Wiki Master Index + Registry
├── standards/        # Standards-Referenzen
├── wiki/             # bestehende/ältere Wiki-Kapitel
├── whitepaper/      # Whitepaper + Einzelkapitel
├── compliance/      # Compliance und Richtlinien
├── project/         # Projekt- und Launch-Dokumentation
├── issues/ sprints/ # Issue- und Sprint-Dokumentation
├── archive/         # Historische Dokumentation
└── monorepo-legacy/ # Historische Monorepo-Docs
```

> **Dokumentationsregel:** Das Software Wiki ist die Navigations- und Registry-Schicht. Es ersetzt keine Code-SSOT, Architektur-SSOT, Standards-SSOT oder Release-Evidenz. Historische Dokumente werden nicht stillschweigend als aktueller Systemstand behandelt.

## Repository-Struktur

```
a-townchain-os/       # Integration / Orchestration
a-townchain-os-docs/  # Dokumentations-Hub
```

---

*A-TownChain OS / Globus Software Wiki · Documentation Hub*

## Lizenzmodell

A-TownChain etabliert ein monetarisiertes, autonomes Open-Source-Ökosystem.

- **ATC-LIC** — Smart Contract Lizenzen
- **ATS-LIC** — System & Hardware Lizenzen
- **Compliance-Handbuch** — regulatorische Dokumentation

Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. Apache-2.0.

---

## ATC Compliance & Governance

**ATC COMPLIANCE: R3** — bestehender Audit-Status; Details siehe `.atc/repository.yaml` und Governance-Dokumentation.

- **Purpose:** Dokumentations-Hub des Ökosystems.
- **Architecture:** DECISIONS_REGISTER, Roadmaps, Audits und Standards-Referenzen.
- **Features:** Software Wiki, Master Registry, Whitepaper, Compliance und Projekt-Dokumentation.
- **Development:** Governance-Regeln und Naming gemäß kanonischer Standards.
- **Testing:** Dokumentations-/Link-Audits und Governance-CI.
- **Security:** SECURITY.md und definierte Release-Gates.
- **Version:** CHANGELOG.md; SemVer.
- **License:** Apache-2.0.

**Last Updated:** 2026-09-30
