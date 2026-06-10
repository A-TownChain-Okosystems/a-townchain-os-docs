# 🤖 A-TownChain OS — Agent Manifest v1.0.0

> **Vollständige Datenkarte für KI-Agenten.**
> Alle GitHub-, Notion- und Google-Ressourcen mit exakten IDs, URLs, Strukturen und Zugriffsregeln.
> Stand: 2026-06-10 | Auto-generiert von Superagent (KAI-OS Automation)

---

## 1. IDENTITÄT DES PROJEKTS

| Feld | Wert |
|------|------|
| **Projektname** | A-TownChain OS (auch: KAI-OS v2.0) |
| **Organisation** | A-TownChain-Okosystems |
| **Typ** | Dezentrales AI-Blockchain-Betriebssystem |
| **Architektur** | 13-Layer (L0–L12) |
| **Sprachen** | Python 75%, ATCLang 15%, HTML/JS/Solidity 10% |
| **Lizenz** | Apache 2.0 |
| **Aktiver Agent** | Superagent (Base44) — täglicher Sync 08:00 Europe/Berlin |

---

## 2. GITHUB

### 2.1 Organisation

```
Organisation:  A-TownChain-Okosystems
GitHub URL:    https://github.com/A-TownChain-Okosystems
```

### 2.2 Aktive Repositories (2)

---

#### ⚙️ `a-townchain-os` — Software Repository

```
Full Name:     A-TownChain-Okosystems/a-townchain-os
URL:           https://github.com/A-TownChain-Okosystems/a-townchain-os
Clone:         https://github.com/A-TownChain-Okosystems/a-townchain-os.git
Branch:        main
Größe:         625 KB
Dateien:       250
Zweck:         Gesamter ausführbarer Quellcode
```

**Verzeichnisstruktur:**

```
a-townchain-os/
├── modules/                  (133 Dateien) ← 9 eigenständige Module
│   ├── kernel/               L2: ShivaOS Microkernel (IPC, ATCFS, Consensus)
│   ├── gateway/              L7: API Gateway :4000 (Auth, Rate-Limit, Router)
│   ├── contracts/            L6: Smart Contracts (ATC-8300, Governance, Bridge)
│   ├── atclang/              L1: ATCLang Compiler + VM + REPL + Stdlib
│   ├── atcnet/               L5: P2P Netzwerk (Kademlia DHT, Bootstrap)
│   ├── ui/                   L10: Neon Dashboard (HTML/JS/CSS)
│   ├── shivamon/             L12: NFT Gaming Engine (Battle, Breeding)
│   ├── franchise/            L8: Business DAO (Vault, Revenue, Token)
│   └── standards/            L0: ATC/ATS Protokoll-Standards
│
├── core/                     (6 Dateien)  Shared Services
│   ├── ai_kernel.py          L3: KI-Orchestrator
│   ├── event_bus.py          Event-System
│   ├── module_loader.py      Dynamisches Modul-Loading
│   └── kernel.py             Master-Entry
│
├── backend/                  (26 Dateien) API-Server & DB
│   ├── api/routes/           REST-Endpunkte (wallet, blockchain, ai, game, governance)
│   ├── api/orchestrator/     Orchestrator-Service
│   ├── db/                   Repository-Pattern + schema.sql
│   └── wallet/               Backend-Wallet-Service
│
├── blockchain/               (33 Dateien) Chain & Consensus
│   ├── consensus/            Hybrid PoH + PoS + PoW + fork_resolution
│   ├── contracts/            On-Chain Contracts (atc8300, governance, shivamon, solidity)
│   ├── nodes/                Bootstrap, Discovery, P2P Propagation, Sync
│   └── wallet/               ECDSA Keygen, DID, Signing
│
├── frontend/                 (7 Dateien)  ShivaOS Dashboard
│   ├── index.html            Neon UI Entry Point
│   ├── assets/js/api.js      API Client
│   ├── assets/css/           Styling
│   ├── battle/index.html     Shivamon Battle UI
│   └── bootscreen/           Boot-Animation
│
├── tests/                    (22 Dateien) Zentrales Test-Verzeichnis
│   ├── test_atclang.py
│   ├── test_ecdsa.py
│   ├── test_gateway.py
│   ├── test_p2p_propagation.py
│   ├── test_smart_contracts.py
│   └── unit/                 (9 weitere Unit-Tests)
│
├── docker/                   (3 Dateien)  Dockerfiles
│   ├── Dockerfile.backend
│   ├── Dockerfile.bootstrap
│   └── Dockerfile.gateway
│
├── docker-compose.yml        5-Node Testnetz (Issue #18)
├── config/                   kai_config.toml + settings.json
├── monitoring/               prometheus.yml (Issue #19)
├── build/build.py            Build-System (Issue #7)
├── start.py                  Haupt-Einstiegspunkt
├── requirements.txt          Python-Abhängigkeiten
└── STATUS.md                 Auto-Sync Status (täglich aktualisiert)
```

**API-Endpunkte (Gateway :4000):**

| Route | Methode | Beschreibung |
|-------|---------|-------------|
| `/api/wallet` | GET/POST | Wallet-Operationen |
| `/api/blockchain` | GET | Chain-Status |
| `/api/ai` | POST | KI-Orchestrator |
| `/api/game` | GET/POST | Shivamon Game |
| `/api/governance` | GET/POST | Governance-Votes |
| `/api/nodes` | GET | Node-Status |
| `/api/marketplace` | GET/POST | NFT-Marktplatz |

**Offene Issues:**

| # | Titel | Priorität | Labels |
|---|-------|-----------|--------|
| #8 | Multi-Node Testnet — P2P live | 🔴 HIGH | blockchain, networking |
| #18 | Docker Compose 5-Node Testnetz | 🟡 MEDIUM | devops, testnet |
| #19 | Node-Monitoring Dashboard | 🟡 MEDIUM | frontend, testnet |
| #7 | Build System EXE/AppImage | 🟡 MEDIUM | build |
| #11 | Shivamon Breeding Gen 2 NFT | 🟡 MEDIUM | game, nft |
| #12 | Solidity Smart Contracts On-Chain | 🟡 MEDIUM | blockchain, solidity |
| #13 | ATC Marketplace | 🟡 MEDIUM | marketplace |
| #10 | Cross-Chain Bridge ATC↔EVM | 🟢 LOW | bridge |

**Kritischer Entwicklungspfad:**
```
#14 Bootstrap → #15 Propagation → #16 Sync → #17 Fork Resolution → #18 Docker → #8 Multi-Node
```

---

#### 📖 `a-townchain-os-docs` — Dokumentation Repository

```
Full Name:     A-TownChain-Okosystems/a-townchain-os-docs
URL:           https://github.com/A-TownChain-Okosystems/a-townchain-os-docs
Clone:         https://github.com/A-TownChain-Okosystems/a-townchain-os-docs.git
Branch:        main
Dateien:       321
Zweck:         Wiki, Roadmap, Specs, TODOs, Standards
```

**Verzeichnisstruktur:**

```
a-townchain-os-docs/
├── docs/                     (281 Dateien)
│   ├── wiki/                 Modul-Wikis (11 Ordner)
│   │   ├── kai-os/           KAI-OS 31-Kapitel Architektur-Wiki (148 Dateien)
│   │   ├── kernel/           Kernel-Doku (10 Dateien)
│   │   ├── gateway/          Gateway-Routen & Auth (6 Dateien)
│   │   ├── contracts/        Smart Contract API (7 Dateien)
│   │   ├── atclang/          ATCLang Sprachspezifikation (13 Dateien)
│   │   ├── atcnet/           P2P Protokoll & Topologie (6 Dateien)
│   │   ├── ui/               Frontend-Architektur (6 Dateien)
│   │   ├── shivamon/         Game-Engine & NFT-Spec (7 Dateien)
│   │   ├── franchise/        Franchise DAO Konzept (8 Dateien)
│   │   ├── standards/        ATC/ATS Standards-Wiki (4 Dateien)
│   │   └── overview/         Gesamt-Architektur-Übersicht (9 Dateien)
│   ├── whitepaper/           Offizielles Whitepaper v2.1.0 (4 Dateien)
│   ├── architecture/         Architektur-Doku (7 Dateien)
│   │   ├── ATCFS.md          Filesystem-Spezifikation
│   │   ├── ATCNET_P2P.md     P2P-Protokoll
│   │   ├── CONSENSUS.md      Konsens-Algorithmus
│   │   ├── GATEWAY.md        Gateway-Architektur
│   │   ├── SHIVAOS_KERNEL.md Kernel-Design
│   │   ├── TESTNET.md        Testnet-Aufbau
│   │   └── WALLET_KEYGEN.md  Wallet & Keygen
│   ├── standards/            ATC/ATS Standards (7 Dateien)
│   │   ├── ATC/ATC_STANDARDS.md    ATC-0001–0009
│   │   └── ATS/ATS_STANDARDS.md    ATS-1000–1009
│   ├── ai/                   AI-Spezifikationen (3 Dateien)
│   │   ├── AI_SAFETY.md
│   │   ├── LLM_ROUTER.md
│   │   └── GEMINI_INTEGRATION.md
│   ├── issues/               Issue-Tracking (19 Dateien)
│   │   ├── ISSUE_01–ISSUE_20 (je Datei pro Issue)
│   │   └── OPEN_ISSUES_MASTER.md
│   ├── blockchain/           Chain-Spezifikationen (2 Dateien)
│   │   ├── SOLANA_INTEGRATION.md
│   │   └── ETHEREUM_INTEGRATION.md
│   ├── roadmap/              Roadmap (1 Datei)
│   │   └── ROADMAP_EXTENDED.md
│   └── atclang/              ATCLang Spec (1 Datei)
│       └── ATCLANG_SPEC_FULL.md
│
├── module-docs/              (35 Dateien) — Technische Specs je Modul
│   ├── kernel/               ARCHITECTURE.md, SECURITY.md, CHANGELOG.md, ATS_STANDARDS.md, README.md
│   ├── gateway/              CHANGELOG.md, SECURITY.md, gateway.atc
│   ├── contracts/            DEPLOYMENT.md, SECURITY.md, CHANGELOG.md, README.md
│   ├── atclang/              ATCLANG_SPEC.md, CONTRIBUTING.md, CHANGELOG.md, README.md
│   ├── atcnet/               PROTOCOL.md, SECURITY.md, CHANGELOG.md, README.md
│   ├── ui/                   DESIGN.md, CHANGELOG.md, README.md
│   ├── shivamon/             GAME_SPEC.md, CHANGELOG.md, README.md
│   ├── franchise/            ARCHITECTURE.md, SECURITY.md, CHANGELOG.md, README.md
│   └── standards/            ATC_STANDARDS.md, ATS_STANDARDS.md, OVERVIEW.md, README.md, ATC-0009-BRIDGE.md
│
├── TODO/
│   └── MASTER_TODO.md        Alle offenen Issues als Aufgabenliste
│
├── STATUS.md                 Aktueller Projektstatus (auto-sync täglich)
├── CHANGELOG.md              Versionshistorie
└── ECOSYSTEM.md              Ökosystem-Übersicht
```

### 2.3 Archivierte Repositories (22)

Alle vollständig in `a-townchain-os` und `a-townchain-os-docs` migriert:

```
A-TownChain-Okosystems/atc-kernel            → modules/kernel/
A-TownChain-Okosystems/atc-gateway           → modules/gateway/
A-TownChain-Okosystems/atc-contracts         → modules/contracts/
A-TownChain-Okosystems/atclang               → modules/atclang/
A-TownChain-Okosystems/atcnet                → modules/atcnet/
A-TownChain-Okosystems/atc-ui               → modules/ui/
A-TownChain-Okosystems/atc-shivamon         → modules/shivamon/
A-TownChain-Okosystems/atc-franchise        → modules/franchise/
A-TownChain-Okosystems/atc-standards        → modules/standards/
A-TownChain-Okosystems/atc-whitepaper       → docs/whitepaper/
A-TownChain-Okosystems/atc-kernel-wiki      → docs/wiki/kernel/
A-TownChain-Okosystems/atc-gateway-wiki     → docs/wiki/gateway/
A-TownChain-Okosystems/atc-contracts-wiki   → docs/wiki/contracts/
A-TownChain-Okosystems/atclang-wiki         → docs/wiki/atclang/
A-TownChain-Okosystems/atcnet-wiki          → docs/wiki/atcnet/
A-TownChain-Okosystems/atc-ui-wiki          → docs/wiki/ui/
A-TownChain-Okosystems/atc-shivamon-wiki    → docs/wiki/shivamon/
A-TownChain-Okosystems/atc-franchise-wiki   → docs/wiki/franchise/
A-TownChain-Okosystems/atc-standards-wiki   → docs/wiki/standards/
A-TownChain-Okosystems/a-townchain-os-wiki  → docs/wiki/overview/
A-TownChain-Okosystems/kai-os-wiki          → docs/wiki/kai-os/
A-TownChain-Okosystems/franchise-factory-wiki → docs/wiki/franchise/
```

---

## 3. NOTION

### 3.1 Haupt-Seiten (Workspace-Root)

| Titel | ID | URL |
|-------|-----|-----|
| Master Roadmap — A-TownChain OS × KAI-OS | `373b826d-b85c-8125-ba83-f04995191bf0` | [öffnen](https://www.notion.so/373b826db85c8125ba83f04995191bf0) |
| A-TownChain OS — Roadmap & Issues | `373b826d-b85c-814d-b7d3-df2d018e494f` | [öffnen](https://www.notion.so/373b826db85c814db7d3df2d018e494f) |
| A-TownChain OS — Projekt Roadmap | `373b826d-b85c-81e1-9659-e87599d649de` | [öffnen](https://www.notion.so/373b826db85c81e19659e87599d649de) |

### 3.2 Untergeordnete Seiten (wichtigste)

| Titel | ID | Zweck |
|-------|-----|-------|
| Tägliches Update-Protokoll | `37bb826d-b85c-81c4-bdd4-cfc0dc74de7e` | Auto-Sync Log |
| Kapitel 31: Live-Status | `379b826d-b85c-81f4-9b2b-f2a05496a4e1` | Aktueller Projektstatus |
| Kapitel 17: Roadmap | `379b826d-b85c-81d7-8826-ebdda0e7ca67` | Sprint-Planung |
| Kapitel 22: Incident Management | `379b826d-b85c-8143-8793-c0b41dedd099` | Fehlerverfolgung |
| Kapitel 16: Sicherheitsrichtlinien | `379b826d-b85c-81b2-9bc7-d7799de4dbef` | Security |
| Kapitel 15: Deployment & Betrieb | `379b826d-b85c-815d-9b5c-dd8154724f8e` | Deployment |
| Kapitel 14: Testing | `379b826d-b85c-81a3-af32-e0635e53ee38` | Test-Strategie |
| Kapitel 13: Fehlerbehandlung | `379b826d-b85c-8155-9daa-e7d9558547cc` | Debugging |
| Kapitel 5: Node als Validator | `379b826d-b85c-812c-a4ef-def231dd63d0` | Node-Setup |
| Kapitel 1: Keys generieren | `379b826d-b85c-8134-a030-c0ce98e87328` | Wallet-Setup |

### 3.3 Notion API-Zugriff

```
API Version:  2022-06-28
Base URL:     https://api.notion.com/v1/
Auth:         Bearer Token (NOTION_ACCESS_TOKEN)

# Seite lesen:
GET  https://api.notion.com/v1/pages/{page_id}

# Seite-Blöcke lesen:
GET  https://api.notion.com/v1/blocks/{page_id}/children

# Seite erstellen/aktualisieren:
POST https://api.notion.com/v1/pages
PATCH https://api.notion.com/v1/blocks/{block_id}

# Suchen:
POST https://api.notion.com/v1/search
```

---

## 4. GOOGLE

### 4.1 Google Sheets — Aktives Dashboard

```
Titel:         KAI-OS / A-TownChain OS — Sync Dashboard
Spreadsheet ID: 1xR5c24NrtYC58OsGrLaUHkQUiL_O6eYVyx8KmFcvBD4
URL:           https://docs.google.com/spreadsheets/d/1xR5c24NrtYC58OsGrLaUHkQUiL_O6eYVyx8KmFcvBD4/edit
```

**Tabs:**

| Tab | gid | Inhalt |
|-----|-----|--------|
| 📊 Übersicht | `0` | Projekt-KPIs, Repo-Metriken |
| 🐛 Issues | `1` | Alle offenen Issues mit Status |
| 📦 Commits & Sync | `2` | Commit-Historie, letzter Sync |
| 📈 Traffic | `3` | Clone-Statistiken, Views |

**Tab-Range-Referenzen:**
```
Übersicht:         '📊 Übersicht'!A1
Issues:            '🐛 Issues'!A1
Commits & Sync:    '📦 Commits & Sync'!A1
Traffic:           '📈 Traffic'!A1
```

### 4.2 Google Sheets — Legacy Dashboard (Archiv)

```
Titel:         KAI-OS & A-TownChain OS — Analyse Dashboard
Spreadsheet ID: 1s8fl7u6Rr5bM0Rc49BJy5_kJp4NrUIffo4Mb3UGfPtY
URL:           https://docs.google.com/spreadsheets/d/1s8fl7u6Rr5bM0Rc49BJy5_kJp4NrUIffo4Mb3UGfPtY/edit
Status:        ⚠️ Legacy — nicht mehr aktiv genutzt
```

| Tab | gid | Inhalt |
|-----|-----|--------|
| Issues Übersicht | `511446771` | |
| Traffic Daten | `1335704938` | |
| Commit Aktivität | `69356051` | |
| Repo Metriken | `1848976141` | |

### 4.3 Google Sheets API-Zugriff

```
API Base URL:  https://sheets.googleapis.com/v4/spreadsheets
Auth:          Bearer Token (GOOGLESHEETS_ACCESS_TOKEN)

# Sheet lesen:
GET ./{spreadsheetId}/values/{range}

# Sheet schreiben:
PUT ./{spreadsheetId}/values/{range}?valueInputOption=USER_ENTERED

# Batch-Update:
POST ./{spreadsheetId}/values:batchUpdate

# Metadaten:
GET ./{spreadsheetId}?fields=properties,sheets.properties
```

---

## 5. AUTO-SYNC AGENT

### 5.1 Aurora — KAI-OS Automation

```
Agent:         Superagent (Base44)
Agent ID:      6a2756186106d6f0fbb105b5
Chat URL:      https://app.base44.com/superagent/6a2756186106d6f0fbb105b5
Automation:    KAI-OS Daily Auto-Sync
Zeitplan:      Täglich 08:00 Uhr Europe/Berlin
```

**Was der Agent täglich macht:**
1. GitHub: Issues, Commits, Traffic aus `a-townchain-os` laden
2. STATUS.md in `a-townchain-os` aktualisieren (Push)
3. STATUS.md + TODO/MASTER_TODO.md in `a-townchain-os-docs` aktualisieren (Push)
4. Notion: Kapitel 31 (Live-Status) + Tägliches Update-Protokoll aktualisieren
5. Google Sheets: Alle 4 Tabs mit aktuellen Daten befüllen

### 5.2 Skill-Datei

```
Pfad:    .agents/skills/kai_os_sync/scripts/sync.py
Aufruf:  run_skill("kai_os_sync")
```

---

## 6. ARCHITEKTUR-KURZREFERENZ

### 6.1 13-Layer Stack

```
L12  Shivamon NFT Gaming     modules/shivamon/
L11  DeFi Bridge             modules/contracts/bridge/
L10  Dashboard UI            modules/ui/ + frontend/
L9   Agent Registry          backend/api/orchestrator/
L8   Franchise DAO           modules/franchise/
L7   API Gateway :4000       modules/gateway/
L6   Smart Contracts         modules/contracts/
L5   ATCNet P2P              modules/atcnet/
L4   Proof of History        blockchain/consensus/poh.py
L3   AI-Kernel               core/ai_kernel.py
L2   ShivaOS Microkernel     modules/kernel/
L1   ATCLang VM              modules/atclang/
L0   ATC/ATS Standards       modules/standards/
```

### 6.2 Service-Ports

| Service | Port | Datei |
|---------|------|-------|
| API Gateway | :4000 | modules/gateway/main.py |
| Backend Server | :5000 | backend/api/server.py |
| Node Bootstrap | :5001 | blockchain/nodes/bootstrap.py |
| P2P Network | :5002 | modules/atcnet/atcnet.py |

### 6.3 Wichtige Dateien

| Datei | Beschreibung |
|-------|-------------|
| `start.py` | Master-Einstiegspunkt |
| `requirements.txt` | Python-Abhängigkeiten |
| `docker-compose.yml` | 5-Node Testnet |
| `config/kai_config.toml` | Zentrale Konfiguration |
| `config/settings.json` | Runtime-Settings |
| `modules/atclang/programs/atcos_main.atc` | ATCLang OS-Hauptprogramm |
| `blockchain/consensus/hybrid_consensus.py` | Konsens-Algorithmus |
| `core/event_bus.py` | Inter-Modul-Kommunikation |

---

## 7. SCHNELL-REFERENZ FÜR AGENTEN

```python
# GitHub: Software-Repo klonen
git clone https://github.com/A-TownChain-Okosystems/a-townchain-os.git

# GitHub: Docs-Repo klonen  
git clone https://github.com/A-TownChain-Okosystems/a-townchain-os-docs.git

# GitHub: Issues abrufen (API)
GET https://api.github.com/repos/A-TownChain-Okosystems/a-townchain-os/issues

# GitHub: Datei pushen
PUT https://api.github.com/repos/A-TownChain-Okosystems/a-townchain-os/contents/{path}

# Notion: Master Roadmap lesen
GET https://api.notion.com/v1/blocks/373b826d-b85c-8125-ba83-f04995191bf0/children

# Notion: Live-Status aktualisieren
PATCH https://api.notion.com/v1/blocks/379b826d-b85c-81f4-9b2b-f2a05496a4e1

# Sheets: Issues-Tab schreiben
PUT https://sheets.googleapis.com/v4/spreadsheets/1xR5c24NrtYC58OsGrLaUHkQUiL_O6eYVyx8KmFcvBD4/values/🐛 Issues!A1
```

---

*Erstellt: 2026-06-10 | Superagent (KAI-OS Automation) | Apache 2.0*
*Nächste Auto-Aktualisierung: 2026-06-11 08:00 Europe/Berlin*
