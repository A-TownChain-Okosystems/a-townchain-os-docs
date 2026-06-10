# CHANGELOG — A-TownChain OS

> **Hinweis:** Historische Einträge verwenden noch alte Repo-Namen.
> Mapping: `shivaos-kernel` → `atc-kernel` | `shivamon` → `atc-shivamon` | `kai-os-wiki` → `docs/kai-os-wiki.md`

# CHANGELOG — A-TownChain Ökosystem

## v2.0.0 — 2026-06-09 (Aktuell)

### 🔥 Neue Implementierungen
- **ATC-1000 PoH** (`blockchain/consensus/poh.py`) — vollständige Hash-Kette, 8 Tests
- **ATN-1000 Bootstrap Node** (`blockchain/nodes/bootstrap.py`) — UDP Discovery, Persistenz
- **ATN-1001 Bootstrap Client** (`atcnet/bootstrap_client.py`) — Announce, Ping, GetPeers
- **ATAUTH-1000 DID-Resolver** (`blockchain/wallet/did.py`) — vollständiges DID:kai System
- **ATS-1000 Orchestrator** (`backend/api/orchestrator/orchestrator.py`) — Circuit-Breaker, LB
- **ATC-9900 Franchise Factory** (`franchise_factory/factory.py`) — DAO, Vault, Royalty
- **ATC-5000 Bridge** (`atc-contracts/bridge/bridge_contract.py`) — Cross-Chain Lock-Mint
- **Battle Engine v2.0** (`shivamon/engine/battle_engine.py`) — Typ-Schwächen, VRF-RNG
- **ATCLang stdlib v0.3** (`atclang/stdlib/atc_stdlib.py`) — Blockchain-Builtins
- **ShivaOS IPC Bus** (`shivaos-kernel/ipc/ipc_bus.py`) — Message-Passing
- **Frontend API v2.0** (`atc-ui/assets/js/api.js`) — alle Endpoints

### 🐳 DevOps
- Docker Compose: 6 Services (bootstrap, backend, gateway, frontend, postgres, prometheus)
- Dockerfiles für alle Services
- Prometheus Monitoring-Config
- requirements.txt in allen 8 Code-Repos konsolidiert
- .env.example vollständig

### 🧪 Tests (19 gesamt, alle grün)
- `tests/test_poh.py` — 8 Tests
- `tests/test_did.py` — 7 Tests
- `tests/test_orchestrator.py` — 4 Tests
- `atcnet/tests/test_atcnet.py` — 4 Tests

### ♻️ Bereinigung
- `KAI_OS_SUMMARY.py` → DEPRECATED markiert
- `atc_issues_summary.py` → DEPRECATED markiert
- `shivaos-kernel/net/atcnet.py` → Redirect auf atcnet Repo
- `docs/DEPRECATED.md` — vollständige Dokumentation aller veralteten Komponenten

### 🌐 Verlinkung
- `ECOSYSTEM.md` in allen Haupt-Repos
- GitHub Topics in allen 23 Repos
- Wiki-Footer in allen 10 Wiki-Repos

---

## v1.3.3-beta — 2026-06-08

### KAI-OS Wiki
- Kapitel 33: ATX Standards Registry (186 Module)
- Kapitel 32: Bug-Tracker (17 Issues)
- Kapitel 31: Live-Projektstatus Dashboard
- Push nach ShivaCoreDev/kai-os-wiki ✅

### ATX Standards Registry
- 35 Standardfamilien, 186 Module
- vollständig auf L0-L12 gemappt

---

## v1.3.2-beta — 2026-06-07

### KAI-OS Wiki
- Kapitel 29: Mainnet Readiness Checklist (55 Punkte)
- Kapitel 30: DevOps-Automatisierung

### Substrate
- pallet-poh, pallet-ai-registry, pallet-agent-registry

---

## v1.3.1-beta — 2026-06-06

- 30 Kapitel KAI-OS Spezifikation
- Notion Master Roadmap synchronisiert
- GitHub Actions wiki-health.yml
