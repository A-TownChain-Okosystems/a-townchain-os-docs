# 🧠⛓️ A-TownChain OS / KAI-OS — Offizielle Dokumentation

> ## 🤖 Fuer KI-Agenten — Pflichtlektuere vor jeder Aenderung
> 1. [`docs/AGENT_POLICY.md`](docs/AGENT_POLICY.md) — verbindliche Regeln, Reality-Check, Konsolidierungsziel
> 2. [`docs/AGENT_COORDINATION.md`](docs/AGENT_COORDINATION.md) — wer arbeitet gerade woran, K3/K4 Todos, Agent-IDs
> 3. [`docs/DECISIONS_REGISTER.md`](docs/DECISIONS_REGISTER.md) — verbindliche Architektur-Entscheidungen


> Dezentrales, KI-gesteuertes Blockchain-Betriebssystem
> **A-TownChain OS** (technisch) · **KAI-OS** (Produktname)

**Version:** 1.0.0 | **Stand:** 05.07.2026 | **Lizenz:** Apache 2.0
**Autor:** Michael Wroblewski | **Agent:** Aurora (MasterBrain · Base44)

---

## Architektur-Policy v1.0

- 🔴 **ATCLang First** — Einzige Programmiersprache (ATC-99)
- 🔴 **Non-EVM Chain** — Keine Ethereum-Kompatibilität
- 🔴 **SHA-256** — TX-Hashing (AD-001 RESOLVED)
- 🔴 **Chain-ID 9000** — Proprietäre Non-EVM Chain-ID (AD-004 RESOLVED)
- ✅ **16 Dienste** — Vollständige Integration, täglicher Sync 08:00 Uhr

---

## Metriken

| Metrik | Wert |
|--------|------|
| Version | 1.0.0 |
| ATC-Standards | 99 (ATC-01–99) |
| Architektur-Tiers | 36 |
| Wiki-Kapitel | 69 | |
| ATCLang-Dateien | 92 (.atc) |
| ATCLang-Zeilen | 15.936 |
| ATCLang-Tests | 60/60 GRÜN |
| Python-Stubs | 0 (Migration abgeschlossen) |
| Solidity-Dateien | 0 |
| Audit-Score | 94/100 |
| Repositories | 24 (2 aktiv) |
| Dienste | 16 synchronisiert |

---

## Sprint-Status

| Sprint | Progress | Status |
|--------|----------|--------|
| 2.1 — ATCLang Compiler/VM | 90% | ✅ NAHE KOMPLETT |
| 2.2 — P2P + Testnet | 100% | ✅ FERTIG |
| 2.3 — Consensus + Gas | 90% | 🔵 AKTIV |
| 2.4 — Kernel + Syscalls | 90% | 🔵 AKTIV |
| 2.5 — NFT + Marketplace | 100% | ✅ FERTIG |
| 2.6 — Governance + Security | 80% | 🔵 AKTIV |
| 2.7 — Testing + CI/CD | 0% | 🟡 GEPLANT |
| 2.8 — Multi-Node Testnet | 0% | 🟡 GEPLANT |
| 3.0 — Backend & Gateway | 95% | 🔵 AKTIV |
| 3.2 — Distributed Intelligence | 55% | 🔵 AKTIV |

---

## Quick Links

- [Roadmap v2.0](docs/ROADMAP.md)
- [Status](STATUS.md)
- [Master TODO](TODO/MASTER_TODO.md)
- [Changelog](CHANGELOG.md)
- [Standards Registry](docs/standards/STANDARDS_REGISTRY.md)
- [Agent Master Rules](AGENT_MASTERRULES.md)
- [ATCLang First Policy](ATCLANG_FIRST.md)

---

## Repository-Struktur

```
a-townchain-os/          # Code-Repository (92 .atc, 15.936 Zeilen)
a-townchain-os-docs/     # Wiki & Dokumentation (69 Kapitel, 99 Standards)
```

---

*A-TownChain OS / KAI-OS · v1.0.0 · Non-EVM · SHA-256 · Chain-ID 9000 · ATCLang First*

## Lizenzmodell

A-TownChain etabliert ein **monetarisiertes, autonomes Open-Source-Oekosystem**.

**"Code is Law" auf Lizenzebene:** Unlizenzierter Code wird von der ATVM
physisch gar nicht erst ausgefuehrt.

- **ATC-LIC** — Smart Contract Lizenzen mit automatischer Royalty-Durchsetzung
- **ATC-LIC** — System & Hardware Lizenzen mit TPM-Verifikation
- **Compliance-Handbuch** — BaFin-konforme Dokumentation

→ [Lizenz-Uebersicht](docs/LICENSING_OVERVIEW.md) | [ATC-LIC](docs/standards/ATC-LIC-SMART_CONTRACT_LICENSE.md) | [ATC-LIC](docs/standards/ATC-LIC-SYSTEM_HARDWARE_LICENSE.md) | [Compliance-Handbuch](docs/compliance/COMPLIANCE_HANDBUCH.md)

Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.

## Verwandte Vision-Projekte

- [`atc-genesis-engine`](https://github.com/A-TownChain-Okosystems/atc-genesis-engine) — separates Vision-/Konzept-Repo fuer eine potenzielle zukuenftige Game-Engine (Genesis Engine) und deren Ausbaustufen. Reines Konzeptmaterial, kein Code, kein aktueller Teil der A-TownChain-Kernentwicklung.
