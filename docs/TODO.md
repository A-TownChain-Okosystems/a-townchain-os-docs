# ✅ KAI-OS / A-TownChain OS — Master To-Do Liste

> Letzte Aktualisierung: 2026-06-09 | Quelle: GitHub Issues + Sprint-Mapping
> Sortiert nach: Priorität → Sprint-Reihenfolge → Abhängigkeiten
> Generiert von: Superagent (KAI-OS Agent)

---

## 📊 Status-Übersicht

| Priorität | Issues | Offen | In Progress | Erledigt |
|-----------|--------|-------|-------------|---------|
| 🔴 HIGH | 8 | 7 | 7 | 0 |
| 🟡 MEDIUM | 9 | 8 | 8 | 1 |
| 🟢 LOW | 1 | 1 | 1 | 0 |
| ✅ CLOSED | 4 | — | — | 4 |
| **Gesamt** | **22** | **16** | **16** | **5** |

---

## 🔴 Priority HIGH — Kritischer Pfad (Sprint 2.1 → 2.8)

> **Reihenfolge:** Die Issues haben harte Abhängigkeiten. #22 → #14 → #15 → #16 → #17 → #8

---

### Sprint 2.1 · #22 🚀 KAI-OS v1.3.2-beta — Substrate-Integration
**Branch:** `main` → `main` | **Milestone:** M1 (Jul 2026)
**Dateien:** `runtime/`, `pallets/`, `.github/workflows/`

- [ ] Substrate Node Template klonen + anpassen
- [ ] `pallet-poh` integrieren (Proof of History) *(276 Zeilen, 8 Tests — bestehend)*
- [ ] `pallet-ai-registry` integrieren *(173 Zeilen — 0 Tests!)*
- [ ] `pallet-agent-registry` integrieren *(182 Zeilen — 0 Tests!)*
- [ ] ⚠️ Unit-Tests für `pallet-ai-registry` schreiben (min. 5)
- [ ] ⚠️ Unit-Tests für `pallet-agent-registry` schreiben (min. 5)
- [ ] GitHub Actions CI/CD Pipeline aufsetzen (`ci.yml`)
- [ ] Wiki Auto-Sync Workflow aktivieren (`wiki-sync.yml`)
- [ ] Docusaurus Deployment konfigurieren (`docusaurus.yml`)
- [ ] PR `main` → `main` mergen

**Definition of Done:** Node startet, Block #1 < 6s, RPC antwortet auf `kai_chainHead`

---

### Sprint 2.2 · #14 🌐 Bootstrap Node — P2P Discovery Service
**Datei:** `blockchain/nodes/discovery.py`
**Abhängigkeit von:** #22 ✅

- [ ] `blockchain/nodes/discovery.py` — UDP-basierter Node-Announcer
- [ ] `NodeRegistry` — Liste aktiver Peers verwalten
- [ ] Heartbeat: inaktive Nodes nach 60s entfernen
- [ ] Bootstrap-Node Config in `config/settings.json`
- [ ] REST-Endpoint: `GET /peers` — aktive Node-Liste zurückgeben
- [ ] Persistenz: bekannte Peers in `data/peers.json` speichern
- [ ] Docker-ready: Bootstrap-Node als separater Service

---

### Sprint 2.2 · #15 📡 Block Propagation — P2P Block Broadcasting
**Datei:** `blockchain/nodes/p2p.py` *(neu erstellen)*
**Abhängigkeit von:** #14

- [ ] `blockchain/nodes/p2p.py` — TCP-Verbindungs-Manager
- [ ] `broadcast_block(block)` — Block an alle Peers senden
- [ ] `broadcast_tx(tx)` — Transaktion an alle Peers senden
- [ ] Deduplication: bereits gesehene Blöcke ignorieren (Bloom Filter)
- [ ] Retry-Logik bei fehlgeschlagener Übertragung
- [ ] Integration mit Bootstrap Node (#14)
- [ ] Unit-Tests: `test_propagation.py`

---

### Sprint 2.2 · #16 🔄 Initial Sync — Neue Nodes synchronisieren
**Datei:** `shivaos/net/atcnet.py`, `blockchain/nodes/node.py`
**Abhängigkeit von:** #14, #15

- [ ] `sync_from_peer(peer_address)` — Chain-Download in 50-Block-Batches
- [ ] Parallelisierung: von mehreren Peers gleichzeitig syncen
- [ ] Checkpoint-System: vertrauenswürdige Block-Hashes hardcoden
- [ ] Sync-Fortschritt im Dashboard anzeigen (Prozent)
- [ ] Sync-Abbruch & Resume bei Verbindungsabbruch
- [ ] Validierung jedes heruntergeladenen Blocks
- [ ] Unit-Tests: `test_initial_sync.py`

---

### Sprint 2.2 · #17 ⛓ Longest-Chain-Rule — Fork-Auflösung
**Datei:** `shivaos/consensus/shiva_consensus.py`
**Abhängigkeit von:** #15, #16

- [ ] `HybridConsensus.resolve_fork(chain_a, chain_b)` implementieren
- [ ] Fork-Erkennung: zwei Blöcke mit gleichem `prev_hash`
- [ ] Längste gültige Chain gewinnt (kumulative Arbeit / Höhe)
- [ ] Orphan-Block Pool: verworfene Blöcke zwischenspeichern
- [ ] Reorganisation (Reorg): Chain-Swap bei längerer Konkurrenz-Chain
- [ ] Reorg-Limit: max. 100 Blöcke tief
- [ ] Unit-Tests: `test_fork_resolution.py` (min. 5 Szenarien)
- [ ] Integration mit Block-Propagation (#15)

---

### Sprint 2.2 · #8 🌐 Multi-Node Testnet — P2P Netzwerk live schalten
**Abhängigkeit von:** #14 ✅, #15 ✅, #16 ✅, #17 ✅ *(alle müssen fertig sein!)*

- [ ] Phase 1: Node-Discovery via UDP Broadcast
- [ ] Phase 2: Peer-Verbindungen aufbauen (TCP, max. 25 Peers)
- [ ] Phase 3: Block-Sync zwischen Nodes
- [ ] Phase 4: Konsens dezentral erreichen (PoH+PoW+PoS)
- [ ] Mindestens 3 Nodes im lokalen Testnet synchronisiert
- [ ] End-to-End-Test: TX auf Node A senden, auf Node C bestätigen
- [ ] **Meilenstein MK3:** Multi-Node Testnet live (5 Nodes) — Dez 2026

---

### Sprint 2.7 · #20 🧪 API-Gateway-Tests — Unit & Integrationstests Port 4000
**Datei:** `gateway/main.py`, `gateway/router.py`, `gateway/middleware/`
**Abhängigkeit von:** #22 (unabhängig, parallel möglich)

- [ ] Unit-Tests: `create_app()` Factory
- [ ] Integrationstests: alle Gateway-Routen (Core :5000 / Chain :5001 / Wallet :5002 / AI :5003 / Game :5004)
- [ ] Test: Auth-Middleware (gültige/ungültige API-Keys)
- [ ] Test: Rate-Limiter (Burst-Schutz, 100 req/min)
- [ ] Test: Signature-Verify Middleware (ECDSA secp256k1)
- [ ] Test: Request-Logger Ausgabe
- [ ] CI: Tests in GitHub Actions einbinden
- [ ] Coverage-Report generieren (Ziel: >80%)

---

### Sprint 2.8 · #3 ⚔️ Shivamon Battle UI — Animierte Kämpfe im Browser
**Datei:** `frontend/index.html`, `frontend/assets/js/battle.js` *(neu)*
**Abhängigkeit von:** #4 ✅ (Persistenz)

- [ ] Battle-Seite in `frontend/index.html` (Sidebar: `⚔️ Battle`)
- [ ] Shivamon-Auswahl: eigene Collection vs. Gegner-Token-ID
- [ ] Animierter Kampfablauf — Runde für Runde mit HP-Bars
- [ ] Angriffs-Animationen per CSS/Canvas
- [ ] Battle-Log: Aktionen als Text-Stream
- [ ] Sieg/Niederlage-Screen mit XP-Gewinn
- [ ] Backend: `POST /api/game/battle` Endpoint
- [ ] On-Chain: Battle-Ergebnis als Event loggen (ATC-9000)

---

## 🟡 Priority MEDIUM (Sprint 2.2 → 2.8, parallel)

---

### Sprint 2.2 · #19 📊 Node-Monitoring Dashboard
**Datei:** `frontend/index.html` (Sidebar: `🌐 Nodes`)
**Parallel zu:** #14, #15 (braucht Nodes-Endpoints)

- [ ] Node-Monitor Seite in ShivaOS Dashboard
- [ ] Node-Karten: ID · Typ · Online/Offline · Peers · Chain-Height · Mempool
- [ ] Live-Refresh alle 5s (WebSocket oder Polling)
- [ ] Sync-Fortschrittsbalken pro Node
- [ ] Verbindungs-Graph: welche Nodes sind verbunden
- [ ] Alert bei Node-Ausfall (rot markieren)
- [ ] Backend: `GET /api/nodes/status` Endpoint

---

### Sprint 2.2 · #18 🐳 Docker Compose — 5-Node lokales Netzwerk
**Datei:** `docker-compose.testnet.yml` *(neu)*, `Dockerfile`
**Parallel zu:** #14, #15

- [ ] `Dockerfile` für A-TownChain OS Backend
- [ ] `docker-compose.testnet.yml` — 5 Service-Definitionen:
  - 1× Bootstrap Node
  - 1× Validator Node
  - 1× Miner Node
  - 2× Full Nodes
- [ ] Internes Docker-Netzwerk `atc-testnet`
- [ ] Volume für Chain-Daten (Persistenz)
- [ ] Health-Checks für alle Services
- [ ] `make testnet-up` / `make testnet-down` Shortcut

---

### Sprint 2.3 · #12 ⛓ Solidity Smart Contracts — On-Chain ATC Token
**Datei:** `blockchain/contracts/solidity/`
**Hinweis:** `ATCToken.sol` + `Shivamon.sol` + `ATCGovernance.sol` + Tests **bereits vorhanden!**

- [x] `ATCToken.sol` — ERC-20 (ATC-8300) *(vorhanden: `code/blockchain/contracts/solidity/ATCToken.sol`)*
- [x] `Shivamon.sol` — ERC-721 (ATC-9000) *(vorhanden)*
- [x] `ATCGovernance.sol` — DAO (ATC-9900) *(vorhanden)*
- [x] Hardhat-Tests vorhanden (`test/ATCGovernance.test.js`)
- [x] Deployment-Script vorhanden (`scripts/deploy.js`)
- [ ] ⚠️ `GenesisToken.sol` — ATC-001 Genesis Block Token *(fehlt noch)*
- [ ] ⚠️ Hardhat Coverage >90% (Execution + Proposal-Tests vervollständigen)
- [ ] Deployment auf KAI-OS Testnet (Chain-ID 9000) via `deploy.js`
- [ ] ABI-Export nach `config/abis/` *(deploy.js macht das bereits)*
- [ ] ⚠️ **Issue #12 Status updaten** (ist fast fertig!)

---

### Sprint 2.3 · #5 🔍 ATC Blockchain Explorer — Block & TX Browser
**Datei:** `frontend/index.html` (Sidebar: `🔍 Explorer`)

- [ ] Explorer-Seite im Dashboard
- [ ] Block-Liste: Height, Hash, Timestamp, TX-Anzahl
- [ ] TX-Detail-Ansicht: von/an/Betrag/Status
- [ ] Adress-Suche: Balance + TX-History
- [ ] Live-Update: neuer Block → Explorer aktualisiert sich
- [ ] Backend: `GET /api/blockchain/explorer/blocks` Endpoint
- [ ] Pagination für lange Block-Listen

---

### Sprint 2.5 · #13 🛒 ATC Marketplace — Shivamon kaufen & verkaufen
**Datei:** `blockchain/contracts/marketplace/marketplace_contract.py` *(neu)*
**Abhängigkeit von:** #4 ✅ (Persistenz), #12

- [ ] `list_for_sale(token_id, seller, price_atc)` — NFT listen
- [ ] `buy(token_id, buyer)` — NFT kaufen + ATC transferieren
- [ ] `cancel_listing(token_id, seller)` — Listing entfernen
- [ ] `get_listings()` — alle aktiven Listings abrufen
- [ ] Frontend: Marketplace-Seite im Dashboard (Sidebar: `🛒 Market`)
- [ ] Preis-Filter + Sortierung nach Rarity/Element/Preis
- [ ] Transaktion-Bestätigungs-Dialog
- [ ] Backend: `GET/POST /api/game/marketplace` Endpoints

---

### Sprint 2.8 · #11 🥚 Shivamon Breeding — Gen 2 NFT Züchtung
**Datei:** `blockchain/contracts/shivamon/shivamon_contract.py`
**Abhängigkeit von:** #4 ✅, #12

- [ ] `breed(parent1_id, parent2_id, owner)` implementieren
- [ ] DNA-Mixing: Stats aus beiden Eltern kombinieren (50/50 + Mutation)
- [ ] Breed-Cooldown: 7 Tage zwischen Züchtungen *(Kap. 44: 7 Tage, nicht 24h!)*
- [ ] Breeding-Fee: 100 KAI (Burning)
- [ ] Rarity-System: gewichtet zufällig aus Eltern-Rarities
- [ ] Gen-2 NFT minten mit neuer Token-ID
- [ ] Frontend: Breeding-UI im Dashboard

---

### Sprint 2.3 · #9 🏛 Governance Contract (ATC-9900) — DAO Voting
**Datei:** `blockchain/contracts/governance/governance_contract.py`
**Hinweis:** `ATCGovernance.sol` bereits vorhanden — Python-Pendant implementieren

- [ ] `Proposal` Datenstruktur (ID, Titel, Beschreibung, Optionen, Deadline)
- [ ] `create_proposal(title, description, options, voting_period)`
- [ ] `vote(proposal_id, voter, option)` — gewichtetes Voting (ATC-Balance)
- [ ] `execute_proposal(proposal_id)` — nach Ablauf ausführen
- [ ] Quorum-Prüfung: min. 10% der ATC-Supply muss abstimmen
- [ ] Timelock: 48h nach Vote-Ende vor Ausführung
- [ ] Frontend: Governance-Seite (Sidebar: `🏛 DAO`)
- [ ] On-Chain: Voting-Ergebnis unveränderlich speichern

---

### Sprint 2.3 · #7 📦 Build System — EXE / AppImage Installer
**Datei:** `build/build.py`, `build/build.spec`

- [ ] `build/build.py` — PyInstaller-Config ausbauen
- [ ] Single-File EXE: Gateway + Backend + Frontend gebündelt
- [ ] Auto-Start: Gateway bootet beim EXE-Start
- [ ] Embedded Browser (CEF / PyWebView) für Dashboard
- [ ] Windows: `.exe` + Installer (NSIS oder Inno Setup)
- [ ] Linux: `.AppImage` Build
- [ ] macOS: `.dmg` (optional, Phase 3)
- [ ] GitHub Actions: automatischer Build bei Release-Tag

---

### ✅ #2 🤖 Gemini AI Integration — ABGESCHLOSSEN
**Status:** ✅ Implementiert 2026-06-08 | **Issue auf GitHub schließen!**

- [x] `backend/api/routes/ai.py` — Gemini API Endpoint
- [x] API-Key Management via `.env` (`GEMINI_API_KEY`)
- [x] Gemini AI Orchestrator v2.1.0
- [x] ATCPromptContext + Cache
- [x] REST Routes: `/api/ai/ask`, `/analyze`, `/contract`, `/debug-tx`, `/health`
- [ ] ⚠️ **Issue #2 auf GitHub manuell schließen!**

---

## 🟢 Priority LOW

---

### Sprint 3.10 · #10 🌉 Cross-Chain Bridge — ATC ↔ EVM
**Datei:** `blockchain/bridge/bridge_relayer.py` *(neu)*
**Abhängigkeit von:** #12, #22, Substrate-Mainnet
**Hinweis:** Architektur vollständig in Kap. 34 dokumentiert

- [ ] `bridge_contract.py` — Lock/Mint-Mechanismus auf Substrate
- [ ] Bridge-Relayer-Service (Python/Rust, Wormhole-Integration)
- [ ] 3-of-5 Multi-Sig für Bridge-Operations
- [ ] Unterstützte Chains: Substrate ↔ Ethereum (Sepolia), Substrate ↔ Solana
- [ ] Devnet End-to-End Test: ATC Substrate → ATC-SPL Solana → Rück-Bridge
- [ ] Emergency-Pause-Mechanismus
- [ ] Frontend: Bridge-UI im Dashboard (Sidebar: `🌉 Bridge`)
- [ ] Bug-Bounty: $50k für kritische Schwachstellen (Mainnet)

---

## ✅ Abgeschlossene Issues

| # | Titel | Abgeschlossen | Sprint |
|---|-------|--------------|--------|
| #21 | build(deps): flask-cors 4.0.1 → 6.0.0 (Dependabot) | 2026-06-08 | — |
| #6  | 🔐 ECDSA Signatur — Sichere TX-Autorisierung | 2026-05-20 | 1.x |
| #4  | 💾 NFT Persistenz — SQLite + Repository Pattern | 2026-05-18 | 1.x |
| #1  | 🔗 Smart Contract Implementation (Python Basis) | 2026-05-15 | 1.x |
| #2  | 🤖 Gemini AI Integration | 2026-06-08 | 2.3 |

---

## 🗺️ Sprint-Mapping (vollständig)

| Sprint | Zeitraum | Issues | Abhängigkeiten | Meilenstein |
|--------|----------|--------|---------------|-------------|
| **2.1** | Jul 2026 | #22 | — | MK1: Node produziert Blöcke |
| **2.2** | Aug 2026 | #14, #15, #16, #17, #8, #18, #19 | #22 ✅ | MK3: Multi-Node Testnet |
| **2.3** | Sep 2026 | #12, #9, #5, #7 | — (parallel) | — |
| **2.5** | Okt 2026 | #13 | #4 ✅, #12 | — |
| **2.7** | Nov 2026 | #20 | #22 ✅ | — |
| **2.8** | Dez 2026 | #11, #3 | #4 ✅, #12 | MK3 ✅ |
| **2.9** | Sep 2026 | Frontier EVM-Pallet | #22 | MetaMask Connect |
| **2.10** | Okt 2026 | IPFS-Mounting in ATCFS | #22 | — |
| **3.9** | Feb 2027 | Solana Anchor-Programme | #22 | — |
| **3.10** | Mär 2027 | #10 / Wormhole Bridge | #22, Solana | — |
| **3.11** | Apr 2027 | Solidity Suite Testnet | #12 ✅, EVM-Pallet | — |
| **3.12** | Mai 2027 | LLM-Router + RAG | #22 | — |
| **3.13** | Jun 2027 | Multi-Modal AI | LLM-Router | MK4: Alpha |
| **4.5** | Sep 2027 | Solana Mainnet | Sprint 3.9 | MK7: 100 Agenten |
| **4.6** | Okt 2027 | ETH Bridge Mainnet | Sprint 3.11 | MK5: Multi-Chain |
| **4.7** | Nov 2027 | Post-Quantum Krypto | #22 | — |
| **4.8** | Dez 2027 | AI Safety Audit | Constitutional AI | MK6: AI Safety |

---

## ⚡ Sofort-Aktionen (heute)

| # | Aktion | Aufwand |
|---|--------|---------|
| 1 | Issue #2 auf GitHub schließen | 2 min |
| 2 | Tests für `pallet-ai-registry` + `pallet-agent-registry` schreiben | 2h |
| 3 | `GenesisToken.sol` erstellen (fehlt für #12) | 1h |
| 4 | Breed-Cooldown in Code auf 7 Tage korrigieren (aktuell 24h!) | 15 min |
| 5 | `feature/bootstrap-node` + `feature/solidity-contracts` mergen | 30 min |
| 6 | `fix/implement-gateway-backend` mergen | 30 min |

---

> *Letzte Aktualisierung: 2026-06-09 03:00 | KAI-OS Agent*
