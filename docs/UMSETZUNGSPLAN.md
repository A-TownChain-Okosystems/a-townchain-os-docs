# 📋 Detaillierter Umsetzungsplan — Alle 70 Repositories

> **Erstellt:** 2026-08-06 13:30 UTC | **Agent:** Aurora (MasterBrain · Base44)
> **Version:** v1.0.0 | **Regelwerk:** AGENT_MASTERRULES.md (14 Regeln)

---

## Zusammenfassung

| Kategorie | Anzahl | Beschreibung |
|----------|--------|-------------|
| 🟢 AKTIV | 17 | Code vorhanden, wird weiterentwickelt |
| 🟡 GERÜST | 17 | Grundgerüst/Docs, Code teilweise |
| ⚪ EMPTY | 0 | Nur Standard-Files, kein Code |
| 🔵 WIKI | 36 | Nur Dokumentation |

---

## Phase 1: Aktive Repositories (14 Repos)

### atc-atclang — 🟢 AKTIV

**Layer:** L2–L4 | **Sprint:** 2.1 | **Zeilen:** 9,203 | .atc=1 .py=30 .rs=0 .ts=0 | Tests: 0 | Stubs: 4

| # | Priorität | Aufgabe | Abhängigkeit | Aufwand |
|---|-----------|---------|--------------|--------|
| 1 | P0 | 4 STUB/TODO-Marker in v03-Modulen auflösen | — | 4h |
| 2 | P0 | v03 Features (lexer, parser, vm) testen | — | 4h |
| 3 | P1 | Sync mit atclang-Compiler-Repo | — | 2h |
| 4 | P1 | atcos_main.atc v1.0 Parser-Kompatibilität | atclang v1.0 | 4h |

---

### atclang — 🟢 AKTIV

**Layer:** L2–L4 | **Sprint:** 2.1 | **Zeilen:** 5,939 | .atc=11 .py=11 .rs=0 .ts=0 | Tests: 0 | Stubs: 0

| # | Priorität | Aufgabe | Abhängigkeit | Aufwand |
|---|-----------|---------|--------------|--------|
| 1 | P0 | Parser: `if let` Pattern Matching | — | 2h |
| 2 | P0 | Parser: Unit Type `()` Support | — | 1h |
| 3 | P0 | Parser: `&mut` Referenzen | — | 3h |
| 4 | P0 | Parser: `in`/`not in` Containment-Operator | — | 2h |
| 5 | P0 | Parser: Tuple-Unpacking destructuring | — | 2h |
| 6 | P1 | Parser: Inline-Closures `fn() => {...}` | — | 3h |
| 7 | P1 | Lexer: f-String Desugaring verifizieren | — | 1h |
| 8 | P1 | Test-Suite: 50+ Tests für Parser/Lexer/VM | Parser-Fixes | 4h |
| 9 | P2 | ATCLang v0.4: Struct-Inheritance | Parser stabil | 8h |
| 10 | P2 | ATCLang v1.0: Power-Operator, for-in | v0.4 | 4h |

---

### a-townchain-os — 🟢 AKTIV

**Layer:** L0–L12 | **Sprint:** 2.1–3.2 | **Zeilen:** 324,198 | .atc=284 .py=240 .rs=63 .ts=199 | Tests: 39 | Stubs: 10

| # | Priorität | Aufgabe | Abhängigkeit | Aufwand |
|---|-----------|---------|--------------|--------|
| 1 | P0 | 10 STUB/TODO-Marker auflösen | — | 6h |
| 2 | P0 | CI/CD Fix (#79) — Workflows pushen | Michael | 2h |
| 3 | P0 | 37 bestehende Tests: Regression prüfen | — | 2h |
| 4 | P1 | 240 .py → ATCLang migrieren | atclang Parser | 40h |
| 5 | P1 | 284 .atc: Parse-Coverage 100% | Parser-Fixes | 8h |
| 6 | P1 | 63 .rs ShivaCore: Evaluieren | ATCLang Kernel | 8h |
| 7 | P1 | 199 .ts AI Studio: Frontend stabilisieren | — | 16h |
| 8 | P2 | Wiki-Kapitel 69 konsistent halten | — | laufend |

---

### atcnet — 🟢 AKTIV

**Layer:** L5 | **Sprint:** 2.2 | **Zeilen:** 1,857 | .atc=1 .py=6 .rs=0 .ts=0 | Tests: 1 | Stubs: 0

| # | Priorität | Aufgabe | Abhängigkeit | Aufwand |
|---|-----------|---------|--------------|--------|
| 1 | P0 | P2P Integration Tests: Discovery, NAT, Gossip | — | 4h |
| 2 | P1 | 6 .py → ATCLang migrieren | atclang | 6h |
| 3 | P1 | ATCNet Protocol v3.2: Vollständige Implementierung | — | 4h |
| 4 | P1 | Bootstrap Client: Multi-Node Testnet | — | 3h |

---

### atc-blockchain — 🟢 AKTIV

**Layer:** L3–L4 | **Sprint:** 2.3 | **Zeilen:** 9,516 | .atc=43 .py=26 .rs=0 .ts=0 | Tests: 1 | Stubs: 0

| # | Priorität | Aufgabe | Abhängigkeit | Aufwand |
|---|-----------|---------|--------------|--------|
| 1 | P0 | Consensus Tests: PoH, PoS, PoW, Hybrid, Fork | — | 4h |
| 2 | P0 | AD-004: Chain-ID 9000 Klärung | Michael | — |
| 3 | P1 | 26 .py → ATCLang migrieren | atclang | 12h |
| 4 | P1 | Smart Contract Registry: ATCLang-Only | — | 3h |
| 5 | P1 | Block Production: End-to-End Test | — | 6h |
| 6 | P1 | 43 .atc + 26 .py: Duplikate bereinigen | — | 4h |

---

### atc-kernel — 🟢 AKTIV

**Layer:** L2 | **Sprint:** 2.4 | **Zeilen:** 2,708 | .atc=4 .py=7 .rs=0 .ts=0 | Tests: 0 | Stubs: 0

| # | Priorität | Aufgabe | Abhängigkeit | Aufwand |
|---|-----------|---------|--------------|--------|
| 1 | P0 | ATCFS Dateisystem: Read/Write/Verify Tests | — | 4h |
| 2 | P1 | 7 .py → ATCLang migrieren | atclang | 6h |
| 3 | P1 | Syscall Interface (ATC-96): Implementierung | — | 4h |
| 4 | P1 | IPC Bus (AD-002 resolved): Message-Passing | — | 4h |

---

### atc-shivacore — 🟢 AKTIV

**Layer:** L1 | **Sprint:** 2.4 | **Zeilen:** 57,314 | .atc=0 .py=0 .rs=61 .ts=0 | Tests: 0 | Stubs: 0

| # | Priorität | Aufgabe | Abhängigkeit | Aufwand |
|---|-----------|---------|--------------|--------|
| 1 | P0 | Unit-Tests: block, scheduler, atcfs, vmm (61 Module) | — | 6h |
| 2 | P1 | Memory Management: Paging/Heap finalisieren | — | 12h |
| 3 | P1 | Syscall-Interface (ATC-96) implementieren | — | 8h |
| 4 | P1 | Security Audit Module: Tests | — | 4h |
| 5 | P2 | ShivaVM → ATCLang VM Migration evaluieren | atc-vm | 16h |
| 6 | P2 | Rust → ATCLang Kernel-Sprachsupport Plan | atclang v0.4 | 8h |

---

### atc-contracts — 🟢 AKTIV

**Layer:** L4 | **Sprint:** 2.5 | **Zeilen:** 2,339 | .atc=4 .py=9 .rs=0 .ts=0 | Tests: 0 | Stubs: 0

| # | Priorität | Aufgabe | Abhängigkeit | Aufwand |
|---|-----------|---------|--------------|--------|
| 1 | P0 | Contract Test-Framework (ATC-98) | atc-vm | 6h |
| 2 | P1 | 9 .py → ATCLang migrieren | atclang | 8h |
| 3 | P1 | ATC-8300 Token: Vollständige Implementierung | — | 4h |
| 4 | P1 | Governance: Voting+Timelock testen | — | 4h |
| 5 | P1 | Bridge: Lock/Mint/Burn/Release testen | — | 3h |

---

### atc-shivamon — 🟢 AKTIV

**Layer:** L12 | **Sprint:** 2.5 | **Zeilen:** 1,223 | .atc=1 .py=5 .rs=0 .ts=0 | Tests: 0 | Stubs: 0

| # | Priorität | Aufgabe | Abhängigkeit | Aufwand |
|---|-----------|---------|--------------|--------|
| 1 | P1 | Battle Engine: Vollständige Tests (Win/Lose/Draw) | — | 4h |
| 2 | P1 | 5 .py → ATCLang migrieren | atclang | 4h |
| 3 | P2 | Breeding/Minting: Edge Cases + DNA-System | — | 3h |
| 4 | P2 | Marketplace: Trading + Royalty Tests | — | 2h |

---

### atc-backend — 🟢 AKTIV

**Layer:** L7 | **Sprint:** 3.0 | **Zeilen:** 2,067 | .atc=8 .py=9 .rs=0 .ts=0 | Tests: 0 | Stubs: 0

| # | Priorität | Aufgabe | Abhängigkeit | Aufwand |
|---|-----------|---------|--------------|--------|
| 1 | P0 | API Route Tests: wallet, blockchain, governance, AI | — | 6h |
| 2 | P1 | 9 .py → ATCLang migrieren | atclang | 6h |
| 3 | P1 | DB Connection: Migration-Framework testen | — | 3h |
| 4 | P1 | Orchestrator: Multi-Service-Koordination testen | — | 3h |

---

### atc-gateway — 🟢 AKTIV

**Layer:** L7 | **Sprint:** 3.0 | **Zeilen:** 2,101 | .atc=11 .py=16 .rs=0 .ts=0 | Tests: 0 | Stubs: 0

| # | Priorität | Aufgabe | Abhängigkeit | Aufwand |
|---|-----------|---------|--------------|--------|
| 1 | P0 | Middleware Tests: auth, rate_limit, logger, signature | — | 4h |
| 2 | P1 | 16 .py → ATCLang migrieren | atclang | 8h |
| 3 | P1 | Circuit Breaker: Lastverhalten testen | — | 3h |
| 4 | P1 | Service Discovery: Health-Check Integration | — | 2h |
| 5 | P1 | 11 .atc + 16 .py: Duplikate bereinigen | — | 3h |

---

### atc-aistudio — 🟢 AKTIV

**Layer:** L10 | **Sprint:** 3.2 | **Zeilen:** 72,450 | .atc=0 .py=0 .rs=0 .ts=199 | Tests: 7 | Stubs: 0

| # | Priorität | Aufgabe | Abhängigkeit | Aufwand |
|---|-----------|---------|--------------|--------|
| 1 | P0 | Test-Abdeckung: 6 → 30+ Tests | — | 8h |
| 2 | P1 | Backend Integration: Blockchain/P2P/Wallet | atc-backend | 12h |
| 3 | P1 | ATCLang Editor: Syntax-Highlighting | atc-ide | 6h |
| 4 | P2 | 3D Render Engine Tab | — | 16h |
| 5 | P2 | Game Engine Tab | atc-shivamon | 16h |

---

### atc-genesis-engine — 🟢 AKTIV

**Layer:** L0 | **Sprint:** 3.2 | **Zeilen:** 1,119 | .atc=0 .py=4 .rs=0 .ts=0 | Tests: 1 | Stubs: 0

| # | Priorität | Aufgabe | Abhängigkeit | Aufwand |
|---|-----------|---------|--------------|--------|
| 1 | P1 | ECS: Vollständige Implementierung | — | 8h |
| 2 | P2 | Renderer2D: Tests + Features | — | 4h |
| 3 | P2 | Modulare KI-native Engine: Architektur | — | 8h |

---

### atc-frontend — 🟢 AKTIV

**Layer:** L10 | **Sprint:**  | **Zeilen:** 2,799 | .atc=0 .py=0 .rs=0 .ts=0 | Tests: 0 | Stubs: 0

| # | Priorität | Aufgabe | Abhängigkeit | Aufwand |
|---|-----------|---------|--------------|--------|
| 1 | P2 | Frontend-Komponenten: BootScreen, Dashboard, WalletView | atc-ui | 12h |
| 2 | P2 | Battle-UI: Shivamon Kampf-Interface | atc-shivamon | 8h |

---

### atc-standards — 🟢 AKTIV

**Layer:** L0 | **Sprint:**  | **Zeilen:** 1,148 | .atc=0 .py=0 .rs=0 .ts=0 | Tests: 0 | Stubs: 0

| # | Priorität | Aufgabe | Abhängigkeit | Aufwand |
|---|-----------|---------|--------------|--------|
| 1 | P1 | 99 Standards als maschinenlesbare YAML/JSON | — | 8h |
| 2 | P2 | Standard-Compliance-Checker Tool | — | 4h |

---

### atc-ui — 🟢 AKTIV

**Layer:** L10 | **Sprint:**  | **Zeilen:** 2,607 | .atc=0 .py=0 .rs=0 .ts=0 | Tests: 0 | Stubs: 0

| # | Priorität | Aufgabe | Abhängigkeit | Aufwand |
|---|-----------|---------|--------------|--------|
| 1 | P2 | Dashboard: System Status, Metrics, Alerts | atc-gateway | 12h |
| 2 | P2 | Neon Theme: CSS/Color System | — | 8h |

---

### atc-whitepaper — 🟢 AKTIV

**Layer:** L0 | **Sprint:**  | **Zeilen:** 4,305 | .atc=0 .py=0 .rs=0 .ts=0 | Tests: 0 | Stubs: 0

| # | Priorität | Aufgabe | Abhängigkeit | Aufwand |
|---|-----------|---------|--------------|--------|
| 1 | P1 | Whitepaper finalisieren (Architecture, Tokenomics, Governance) | — | 8h |

---

## Phase 2: Gerüste → Implementierung (12 Repos)

### atc-ci — 🟡 GERÜST

**Layer:** L0 | **Zeilen:** 167 | .atc=0 .py=0 .rs=0 .ts=0

| # | Priorität | Aufgabe | Abhängigkeit | Aufwand |
|---|-----------|---------|--------------|--------|
| 1 | P2 | Implementierung starten — L0 | Phase 1 | 16h |

---

### atc-bootloader — 🟡 GERÜST

**Layer:** L1 | **Zeilen:** 165 | .atc=0 .py=0 .rs=0 .ts=0

| # | Priorität | Aufgabe | Abhängigkeit | Aufwand |
|---|-----------|---------|--------------|--------|
| 1 | P2 | Implementierung starten — L1 | Phase 1 | 16h |

---

### atc-drivers — 🟡 GERÜST

**Layer:** L1 | **Zeilen:** 166 | .atc=0 .py=0 .rs=0 .ts=0

| # | Priorität | Aufgabe | Abhängigkeit | Aufwand |
|---|-----------|---------|--------------|--------|
| 1 | P2 | Implementierung starten — L1 | Phase 1 | 16h |

---

### atc-shivacore-tools — 🟡 GERÜST

**Layer:** L1 | **Zeilen:** 130 | .atc=0 .py=0 .rs=0 .ts=0

| # | Priorität | Aufgabe | Abhängigkeit | Aufwand |
|---|-----------|---------|--------------|--------|
| 1 | P2 | Implementierung starten — L1 | Phase 1 | 16h |

---

### atc-ide — 🟡 GERÜST

**Layer:** L10 | **Zeilen:** 171 | .atc=0 .py=0 .rs=0 .ts=0

| # | Priorität | Aufgabe | Abhängigkeit | Aufwand |
|---|-----------|---------|--------------|--------|
| 1 | P2 | Implementierung starten — L10 | Phase 1 | 16h |

---

### atc-linux-edition — 🟡 GERÜST

**Layer:** L10 | **Zeilen:** 146 | .atc=0 .py=0 .rs=1 .ts=0

| # | Priorität | Aufgabe | Abhängigkeit | Aufwand |
|---|-----------|---------|--------------|--------|
| 1 | P2 | egui Desktop-Client: Basis-GUI | atc-sdk | 12h |
| 2 | P2 | Blockchain-Integration: Sync + Wallet | atc-wallet | 12h |

---

### atc-mobile — 🟡 GERÜST

**Layer:** L10 | **Zeilen:** 538 | .atc=2 .py=2 .rs=0 .ts=0

| # | Priorität | Aufgabe | Abhängigkeit | Aufwand |
|---|-----------|---------|--------------|--------|
| 1 | P1 | Biometric Auth: Session/Lockout Tests | — | 2h |
| 2 | P2 | QR Code: Generate/Scan Integration | — | 3h |

---

### atc-wallet — 🟡 GERÜST

**Layer:** L10 | **Zeilen:** 163 | .atc=0 .py=0 .rs=0 .ts=0

| # | Priorität | Aufgabe | Abhängigkeit | Aufwand |
|---|-----------|---------|--------------|--------|
| 1 | P2 | Implementierung starten — L10 | Phase 1 | 16h |

---

### atc-windows-edition — 🟡 GERÜST

**Layer:** L10 | **Zeilen:** 140 | .atc=0 .py=0 .rs=1 .ts=0

| # | Priorität | Aufgabe | Abhängigkeit | Aufwand |
|---|-----------|---------|--------------|--------|
| 1 | P2 | egui Desktop-Client: Basis-GUI | atc-sdk | 12h |
| 2 | P2 | Blockchain-Integration: Sync + Wallet | atc-wallet | 12h |

---

### atc-stdlib — 🟡 GERÜST

**Layer:** L3 | **Zeilen:** 157 | .atc=0 .py=0 .rs=0 .ts=0

| # | Priorität | Aufgabe | Abhängigkeit | Aufwand |
|---|-----------|---------|--------------|--------|
| 1 | P2 | Implementierung starten — L3 | Phase 1 | 16h |

---

### atc-vm — 🟡 GERÜST

**Layer:** L4 | **Zeilen:** 161 | .atc=0 .py=0 .rs=0 .ts=0

| # | Priorität | Aufgabe | Abhängigkeit | Aufwand |
|---|-----------|---------|--------------|--------|
| 1 | P2 | Implementierung starten — L4 | Phase 1 | 16h |

---

### atc-dns — 🟡 GERÜST

**Layer:** L5 | **Zeilen:** 165 | .atc=0 .py=0 .rs=0 .ts=0

| # | Priorität | Aufgabe | Abhängigkeit | Aufwand |
|---|-----------|---------|--------------|--------|
| 1 | P2 | Implementierung starten — L5 | Phase 1 | 16h |

---

### atc-atcpkg — 🟡 GERÜST

**Layer:** L7 | **Zeilen:** 940 | .atc=2 .py=0 .rs=0 .ts=0

| # | Priorität | Aufgabe | Abhängigkeit | Aufwand |
|---|-----------|---------|--------------|--------|
| 1 | P1 | Package Manager: install/publish/verify testen | — | 3h |
| 2 | P2 | Dependency Resolution: Version-Konflikte | — | 3h |

---

### atc-cli — 🟡 GERÜST

**Layer:** L7 | **Zeilen:** 178 | .atc=0 .py=0 .rs=0 .ts=0

| # | Priorität | Aufgabe | Abhängigkeit | Aufwand |
|---|-----------|---------|--------------|--------|
| 1 | P2 | Implementierung starten — L7 | Phase 1 | 16h |

---

### atc-explorer — 🟡 GERÜST

**Layer:** L7 | **Zeilen:** 167 | .atc=0 .py=0 .rs=0 .ts=0

| # | Priorität | Aufgabe | Abhängigkeit | Aufwand |
|---|-----------|---------|--------------|--------|
| 1 | P2 | Implementierung starten — L7 | Phase 1 | 16h |

---

### atc-sdk — 🟡 GERÜST

**Layer:** L7 | **Zeilen:** 173 | .atc=0 .py=0 .rs=0 .ts=0

| # | Priorität | Aufgabe | Abhängigkeit | Aufwand |
|---|-----------|---------|--------------|--------|
| 1 | P2 | Implementierung starten — L7 | Phase 1 | 16h |

---

### atc-franchise — 🟡 GERÜST

**Layer:** L8–L10 | **Zeilen:** 705 | .atc=3 .py=2 .rs=0 .ts=0

| # | Priorität | Aufgabe | Abhängigkeit | Aufwand |
|---|-----------|---------|--------------|--------|
| 1 | P1 | Registry/Revenue/Token Contracts testen | — | 3h |
| 2 | P2 | Factory DAO: Voting + Proposal Lifecycle | — | 3h |

---

## Phase 3: Leere Repositories (8 Repos)

| Repo | Layer | Sprint | Ziel-Implementierung | Aufwand |
|------|-------|--------|---------------------|--------|

**Detaillierte Aufgaben pro leeren Repo:**

---

## Phase 4: Wiki-Repositories (36 Repos)

36 Wiki-Repos werden automatisch mit Code-Repos synchronisiert.

| Wiki | Code-Repo | MD | Status |
|------|-----------|----|--------|
| a-townchain-os-docs | a-townchain-os | 1126 | ✅ Ausführlich |
| kai-os-wiki | a-townchain-os | 391 | ✅ Ausführlich |
| a-townchain-os-wiki | a-townchain-os | 24 | 🟡 Basis |
| atclang-wiki | atclang | 16 | 🟡 Basis |
| atc-kernel-wiki | atc-kernel | 13 | 🟡 Basis |
| atc-contracts-wiki | atc-contracts | 10 | 🔴 Minimal |
| atc-franchise-wiki | atc-franchise | 9 | 🔴 Minimal |
| atc-shivamon-wiki | atc-shivamon | 9 | 🔴 Minimal |
| atc-atclang-wiki | atc-atclang | 8 | 🔴 Minimal |
| atc-backend-wiki | atc-backend | 8 | 🔴 Minimal |
| atc-drivers-wiki | atc-drivers | 8 | 🔴 Minimal |
| atc-gateway-wiki | atc-gateway | 8 | 🔴 Minimal |
| atc-sdk-wiki | atc-sdk | 8 | 🔴 Minimal |
| atc-stdlib-wiki | atc-stdlib | 8 | 🔴 Minimal |
| atc-ui-wiki | atc-ui | 8 | 🔴 Minimal |
| atc-vm-wiki | atc-vm | 8 | 🔴 Minimal |
| atcnet-wiki | atcnet | 8 | 🔴 Minimal |
| atc-atcpkg-wiki | atc-atcpkg | 7 | 🔴 Minimal |
| atc-blockchain-wiki | atc-blockchain | 7 | 🔴 Minimal |
| atc-bootloader-wiki | atc-bootloader | 7 | 🔴 Minimal |
| atc-ci-wiki | atc-ci | 7 | 🔴 Minimal |
| atc-cli-wiki | atc-cli | 7 | 🔴 Minimal |
| atc-dns-wiki | atc-dns | 7 | 🔴 Minimal |
| atc-standards-wiki | atc-standards | 6 | 🔴 Minimal |
| atc-aistudio-wiki | atc-aistudio | 5 | 🔴 Minimal |
| atc-explorer-wiki | atc-explorer | 5 | 🔴 Minimal |
| atc-frontend-wiki | atc-frontend | 5 | 🔴 Minimal |
| atc-genesis-engine-wiki | atc-genesis-engine | 5 | 🔴 Minimal |
| atc-ide-wiki | atc-ide | 5 | 🔴 Minimal |
| atc-linux-edition-wiki | atc-linux-edition | 5 | 🔴 Minimal |
| atc-shivacore-tools-wiki | atc-shivacore-tools | 5 | 🔴 Minimal |
| atc-shivacore-wiki | atc-shivacore | 5 | 🔴 Minimal |
| atc-wallet-wiki | atc-wallet | 5 | 🔴 Minimal |
| atc-windows-edition-wiki | atc-windows-edition | 5 | 🔴 Minimal |
| atc-mobile-wiki | atc-mobile | 4 | 🔴 Minimal |
| franchise-factory-wiki | atc-franchise | 2 | 🔴 Minimal |

**Wiki-Aufgaben (laufend):**
- Wiki-Kapitel 69 konsistent halten
- ROADMAP/STATUS bei Code-Änderungen aktualisieren
- ARCHITECTURE_TREES bei neuen Repos regenerieren
- FILE_REGISTER bei Datei-Änderungen aktualisieren
- 33 Modul-Wikis mit Code-Repos synchronisieren

---

## Blocker & Entscheidungen (Michael)

| ID | Blocker | Impact | Entscheidung |
|----|---------|--------|-------------|
| AD-004 | Chain-ID 9000 von XDC belegt | Blockiert Mainnet + L9-BRIDGE | Neue Chain-ID |
| #79 | CI/CD Workflow-Push blockiert | Blockiert automatische Tests | `ci-cd-fix/apply-fix.sh` |
| ATCLang | Parser-Coverage 198/198 | Blockiert .py→.atc Migration | 6 Parser-Fixes abschließen |
| ATC-97 | Agent Interaction Protocol | Blockiert Sprint 3.2 | Spezifikation finalisieren |

---

## Prioritäten-Matrix

### SOFORT (P0)

1. **ATCLang Parser-Fixes** (atclang + atc-atclang) — 6 Fixes, 13h — blockiert alles
2. **Test-Suiten aufbauen** — 14 aktive Repos haben 0 Tests — 60h
3. **AD-004 Chain-ID** — Michael Entscheidung
4. **CI/CD Fix (#79)** — Michael manuell

### KURZFRISTIG (P1, 1–2 Wochen)

5. **Python→ATCLang Migration** — 157 .py in 7 Repos — 80h
6. **Gerüste implementieren** — atc-vm, atc-cli, atc-ci, atc-stdlib — 60h
7. **Smart Contract Tests** — atc-contracts + atc-blockchain — 14h
8. **Whitepaper finalisieren** — 8h

### MITTELFRISTIG (P2, 1–2 Monate)

9. **Rust→ATCLang Migration** — 134 .rs — 160h
10. **Frontend-Implementierung** — atc-ui + atc-frontend — 40h
11. **AI Studio Integration** — 30h
12. **Leere Repos implementieren** — 8 Repos — 200h

### LANGFRISTIG (Sprint 4.0+)

13. **Kernel → ATCLang Kernel-Support** — 80h
14. **Mainnet Launch Vorbereitung** — 40h
15. **ATCLang v1.0** (Compiler, VM, Stdlib, Tests) — 100h

---

## Aufwand-Schätzung

| Phase | Dauer | Repos |
|-------|-------|-------|
| Phase 1: Aktive Repos | ~280h | 14 |
| Phase 2: Gerüste | ~100h | 12 |
| Phase 3: Leere Repos | ~200h | 8 |
| Phase 4: Wiki-Sync | Laufend | 36 |
| **Total** | **~580h** | **70** |

*580h ≈ 14.5 Wochen Vollzeit bei 40h/Woche*

---

*Umsetzungsplan v1.0.0 · 2026-08-06 · Aurora (MasterBrain · Base44) · AGENT_MASTERRULES.md*
