# atc-ci

> ## 🤖 Fuer KI-Agenten — Pflichtlektuere vor jeder Aenderung
> Governance liegt zentral im Wiki-Repo `a-townchain-os-docs`:
> 1. [`AGENT_POLICY.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_POLICY.md) — verbindliche Regeln, Reality-Check, Konsolidierungsziel
> 2. [`AGENT_COORDINATION.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_COORDINATION.md) — wer arbeitet gerade woran, Todos, Agent-IDs
> 3. [`DECISIONS_REGISTER.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/DECISIONS_REGISTER.md) — verbindliche Architektur-Entscheidungen


> **CI/CD Build, Testing & Automated Deployment Pipelines für A-TownChain OS**

[![Layer](https://img.shields.io/badge/Layer-L0-purple)](https://github.com/A-TownChain-Okosystems)
[![KAI-OS](https://img.shields.io/badge/KAI--OS-v1.0.0-blue)](https://github.com/A-TownChain-Okosystems/a-townchain-os/blob/main/docs/kai-os-wiki.md)
[![Org](https://img.shields.io/badge/Org-A--TownChain--Okosystems-green)](https://github.com/A-TownChain-Okosystems)
[![Wiki](https://img.shields.io/badge/Wiki-📖-blue)](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/tree/main/docs/archive/wiki/atc-ci-wiki)

---

## 📦 Description / Beschreibung

Das Repository `atc-ci` enthält die kontinuierlichen Integrations- und Test-Pipelines für das gesamte A-TownChain Ökosystem. Es steuert automatische Lintings, ATCLang Smart Contract Verifikationen, ShivaCore `no_std` Kernel-Tests, Docker Image Builds und Testnet-Deployments.

---

## 🏗️ Architektur

```
[ Git Push / PR ] ──> [ CI Matrix Orchestrator ]
                            │
      ┌─────────────────────┼─────────────────────┐
      ▼                     ▼                     ▼
[ ATCLang Syntax ]    [ Rust Kernel Tests ]   [ Security Audit ]
(atclang compile)     (cargo test --no-std)   (BaFin Compliance)
      │                     │                     │
      └─────────────────────┼─────────────────────┘
                            ▼
               [ Automated Artifact Build ]
               [ Docker / Binary Release  ]
```

---

## 🧱 Komponenten

- **`workflows/build-and-test.yml`**: Haupt-Pipeline für Unit- und Integrationstests.
- **`workflows/atclang-verify.yml`**: ATCLang Statische Code-Analyse und Bytecode-Validierung.
- **`workflows/security-audit.yml`**: Automatische Sicherheits- und BaFin-Compliance-Scans.
- **`workflows/testnet-deploy.yml`**: Continuous Deployment auf das A-TownChain Testnet.

---

## 🚀 Usage / Verwendung

### Lokaler Workflow-Test via act
```bash
act -W .github/workflows/build-and-test.yml
```

---

## 🛠️ Build & Setup

1. **Lokal ausführen:**
   ```bash
   bash scripts/run-local-ci.sh
   ```

---

## 🔗 Verwandte Repos & Abhängigkeiten

**Nutzt:** Alle A-TownChain Repos  
**Wird genutzt von:** Entwickler & Release Engine  
**Wiki Link:** [→ atc-ci-wiki](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/tree/main/docs/archive/wiki/atc-ci-wiki)

---

## 🌐 A-TownChain Ökosystem

| Repo | Layer | Beschreibung |
|------|-------|-------------|
| [a-townchain-os](https://github.com/A-TownChain-Okosystems/a-townchain-os) | `L2–L4` | Haupt-Repo — KAI-OS Core |
| [atc-kernel](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-kernel) | `L2` | Microkernel, IPC, ATCFS |
| [atcnet](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atcnet) | `L5` | P2P Netzwerk, Bootstrap |
| [atc-gateway](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-gateway) | `L7` | API Gateway Port 4000 |
| [atclang](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atclang) | `L2-L4` | Proprietäre Sprache |
| [atc-contracts](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-contracts) | `L4/L11` | Smart Contracts + Bridge |
| [shivamon](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-shivamon) | `L12` | NFT Gaming |
| [atc-franchise](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-franchise) | `L10/L8` | Business DAO |
| [atc-ui](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-ui) | `L10` | Neon Dashboard |
| [atc-standards](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-standards) | `L0` | Protokoll-Standards |

---

*Teil des [A-TownChain Ökosystems](https://github.com/A-TownChain-Okosystems) · v1.0.0 · Stand: 2026-08-05*

---

## Lizenz

Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. **All Rights Reserved.**

Dieses Projekt nutzt das **ATC-LIC Lizenzmodell** — ein monetarisiertes, autonomes
Open-Source-Oekosystem. Unlizenzierter Code wird von der ATVM physisch nicht ausgefuehrt.

- [ATC-LIC — Smart Contract Licenses](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/standards/ATC-LIC-SMART_CONTRACT_LICENSE.md)
- [ATC-LIC — System & Hardware Licenses](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/standards/ATC-LIC-SYSTEM_HARDWARE_LICENSE.md)
- [Compliance-Handbuch (BaFin)](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/compliance/COMPLIANCE_HANDBUCH.md)
- [Lizenz-Uebersicht](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/LICENSING_OVERVIEW.md)
