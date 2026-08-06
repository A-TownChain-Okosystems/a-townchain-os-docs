# 📋 Umsetzungsplan — A-TownChain Ökosystem (70 Repos)

> **Erstellt:** 2026-08-06 13:20 UTC | **Agent:** Aurora (MasterBrain · Base44)
> **Version:** v1.0.0 | **Status:** Aktiv

---

## Repo-Klassifizierung

| Status | Bedeutung | Repos |
|--------|-----------|-------|
| 🟢 AKTIV | Code vorhanden, wird weiterentwickelt | 14 |
| 🟡 GERÜST | Grundgerüst/Docs nur, Code fehlt | 12 |
| 🔵 WIKI | Nur Dokumentation (36 Wikis) | 36 |
| ⚪ EMPTY | Nur Standard-Files, kein Inhalt | 8 |

---

## Phase 1: Kritische Pfade (Sprint 2.1–2.4)

### 1.1 atclang / atc-atclang — ATCLang Compiler ⬅️ BLOCKER
**Status:** 🟢 AKTIV — 5.680 + 8.666 Zeilen, 30+11 Python Module, 11+1 .atc
**Tests:** 0 — ❌ KRITISCH

| Aufgabe | Priorität | Aufwand | Abhängigkeit |
|---------|-----------|---------|--------------|
| Parser: `if let` Pattern Matching | P0 | 2h | — |
| Parser: Unit Type `()` Support | P0 | 1h | — |
| Parser: `&mut` Referenzen | P0 | 3h | — |
| Parser: `in`/`not in` Containment-Operator | P0 | 2h | — |
| Parser: Tuple-Unpacking (`a, b = func()`) | P1 | 2h | — |
| Parser: Inline-Closures (`fn() => {...}`) | P1 | 3h | — |
| Test-Suite: 50+ Test-Dateien | P0 | 4h | Alle Parser-Fixes |
| ATCLang v0.4 Feature: Struct-Inheritance | P2 | 8h | Parser stabil |
| ATCLang v1.0 Feature: Power-Operator, for-in | P2 | 4h | v0.4 |

### 1.2 a-townchain-os — Monorepo ⬅️ ZENTRAL
**Status:** 🟢 AKTIV — 196K Zeilen, 284 .atc, 240 .py, 63 .rs, 199 .ts
**Tests:** 37 — ⚠️ Gering

| Aufgabe | Priorität | Aufwand | Abhängigkeit |
|---------|-----------|---------|--------------|
| 13 STUB-Marker auflösen → ATCLang migrieren | P0 | 6h | atclang Parser stabil |
| atcos_main.atc v1.0 Parser-Kompatibilität | P1 | 4h | atclang v1.0 |
| Python-Stubs → ATCLang (240 .py verbleibend) | P1 | 40h | atclang v0.4 |
| Test-Abdeckung: 37 → 100+ Tests | P1 | 8h | — |
| Rust ShivaCore Module: 63 .rs → ATCLang migrieren | P2 | 80h | ATCLang Kernel-Support |
| AI Studio (199 .ts): Frontend stabilisieren | P2 | 16h | — |

### 1.3 atc-shivacore — Bare-Metal Kernel
**Status:** 🟢 AKTIV — 54K Zeilen, 61 .rs Dateien
**Tests:** 0 — ❌ KRITISCH

| Aufgabe | Priorität | Aufwand | Abhängigkeit |
|---------|-----------|---------|--------------|
| Rust → ATCLang Kernel-Sprachsupport evaluieren | P1 | 8h | atclang v0.4 |
| Unit-Tests für Kernel-Module (block, scheduler, atcfs) | P0 | 6h | — |
| Memory Management: Paging/Heap finalisieren | P1 | 12h | — |
| Syscall-Interface (ATC-96) implementieren | P1 | 8h | — |
| ShivaVM (27 Opcodes) → ATCLang VM Migration | P2 | 16h | atc-vm |

### 1.4 atc-blockchain — Blockchain Core
**Status:** 🟢 AKTIV — 8.913 Zeilen, 43 .atc, 26 .py
**Tests:** 0 — ❌

| Aufgabe | Priorität | Aufwand | Abhängigkeit |
|---------|-----------|---------|--------------|
| 26 .py Stubs → ATCLang migrieren | P1 | 12h | atclang Parser stabil |
| Consensus-Module Tests (PoH, PoS, PoW, Hybrid) | P0 | 4h | — |
| Smart Contract Registry: ATCLang-Only | P1 | 3h | — |
| Block Production Pipeline: End-to-End Test | P1 | 6h | — |
| AD-004: Chain-ID 9000 Klärung | P0 | — | Michael Entscheidung |

---

## Phase 2: Infrastruktur (Sprint 2.5–3.0)

### 2.1 atc-contracts — Smart Contracts
**Status:** 🟢 AKTIV — 2.062 Zeilen, 4 .atc, 9 .py
**Tests:** 0

| Aufgabe | Priorität | Aufwand | Abhängigkeit |
|---------|-----------|---------|--------------|
| 9 .py → ATCLang migrieren (bridge, shivamon, governance) | P1 | 8h | atclang |
| Contract Test-Framework (ATC-98) | P0 | 6h | atc-vm |
| ATC-8300 Token: Vollständige Implementierung | P1 | 4h | — |
| Governance Contract: Voting+Timelock testen | P1 | 4h | — |

### 2.2 atc-gateway — API Gateway
**Status:** 🟢 AKTIV — 1.673 Zeilen, 11 .atc, 16 .py
**Tests:** 0

| Aufgabe | Priorität | Aufwand | Abhängigkeit |
|---------|-----------|---------|--------------|
| 16 .py → ATCLang migrieren | P1 | 8h | atclang |
| Middleware Tests (auth, rate_limit, logger, signature) | P0 | 4h | — |
| Circuit Breaker: Lastverhalten testen | P1 | 3h | — |
| Service Discovery: Health-Check Integration | P1 | 2h | — |

### 2.3 atc-backend — Backend Services
**Status:** 🟢 AKTIV — 1.833 Zeilen, 8 .atc, 9 .py
**Tests:** 0

| Aufgabe | Priorität | Aufwand | Abhängigkeit |
|---------|-----------|---------|--------------|
| 9 .py → ATCLang migrieren (orchestrator, routes) | P1 | 6h | atclang |
| API Route Tests: wallet, blockchain, governance, AI | P0 | 6h | — |
| DB Connection: Migration-Framework testen | P1 | 3h | — |

### 2.4 atcnet — P2P Network
**Status:** 🟢 AKTIV — 1.555 Zeilen, 1 .atc, 6 .py
**Tests:** 1

| Aufgabe | Priorität | Aufwand | Abhängigkeit |
|---------|-----------|---------|--------------|
| 6 .py → ATCLang migrieren (bootstrap, node, gossip) | P1 | 6h | atclang |
| P2P Integration Tests: Discovery, NAT, Gossip | P0 | 4h | — |
| ATCNet Protocol v3.2: Vollständige Implementierung | P1 | 4h | — |

### 2.5 atc-franchise — Business DAOs
**Status:** 🟢 AKTIV — 490 Zeilen, 3 .atc, 2 .py
**Tests:** 0

| Aufgabe | Priorität | Aufwand | Abhängigkeit |
|---------|-----------|---------|--------------|
| 2 .py → ATCLang migrieren (factory, routes) | P2 | 3h | atclang |
| Registry/Revenue/Token Contracts testen | P1 | 3h | — |

### 2.6 atc-shivamon — NFT Gaming
**Status:** 🟢 AKTIV — 1.032 Zeilen, 1 .atc, 5 .py
**Tests:** 0

| Aufgabe | Priorität | Aufwand | Abhängigkeit |
|---------|-----------|---------|--------------|
| 5 .py → ATCLang migrieren (game_routes, marketplace, shivamon_contract) | P2 | 4h | atclang |
| Battle Engine: Vollständige Tests | P1 | 4h | — |
| Breeding/Minting: Edge Cases | P2 | 3h | — |

### 2.7 atc-kernel — Microkernel
**Status:** 🟢 AKTIV — 2.169 Zeilen, 4 .atc, 7 .py
**Tests:** 0

| Aufgabe | Priorität | Aufwand | Abhängigkeit |
|---------|-----------|---------|--------------|
| 7 .py → ATCLang migrieren (kernel, atcfs, syscalls) | P1 | 6h | atclang |
| ATCFS Dateisystem: Read/Write/Verify Tests | P0 | 4h | — |
| Syscall Interface (ATC-96): Implementierung | P1 | 4h | — |

### 2.8 atc-mobile — Mobile Wallet
**Status:** 🟢 AKTIV — 354 Zeilen, 2 .atc, 2 .py
**Tests:** 0

| Aufgabe | Priorität | Aufwand | Abhängigkeit |
|---------|-----------|---------|--------------|
| 2 .py → ATCLang migrieren | P2 | 2h | atclang |
| Biometric Auth: Session/Lockout Tests | P1 | 2h | — |
| QR Code: Generate/Scan Integration | P2 | 3h | — |

---

## Phase 3: Gerüste → Implementierung (Sprint 3.1–4.0)

### 3.1 Repos mit Grundgerüst (nur Docs, kein Code)

| Repo | Layer | Aktuell | Ziel | Aufwand |
|------|-------|---------|------|---------|
| atc-bootloader | L1 | 7 Files (nur Docs) | UEFI Bootloader in Rust/ATCLang | 40h |
| atc-ci | L0 | 7 Files (nur Docs) | GitHub Actions Workflows | 8h |
| atc-cli | L7 | 7 Files (nur Docs) | CLI-Tool (status, wallet, mine) | 16h |
| atc-dns | L5 | 7 Files (nur Docs) | Dezentraler DNS-Resolver | 12h |
| atc-drivers | L1 | 7 Files (nur Docs) | Hardware-Treiber (USB, GPU, Audio) | 24h |
| atc-explorer | L7 | 7 Files (nur Docs) | Block Explorer Web-App | 16h |
| atc-frontend | L10 | 12 Files (nur Docs) | Web-Frontend (Bootscreen, Battle-UI) | 20h |
| atc-ide | L10 | 7 Files (nur Docs) | ATCLang IDE/Playground | 24h |
| atc-sdk | L7 | 7 Files (nur Docs) | Rust-Crate + TypeScript-Package | 16h |
| atc-stdlib | L3 | 7 Files (nur Docs) | Standard Library für Userspace | 20h |
| atc-ui | L10 | 10 Files (nur Docs) | Neon Dashboard UI | 20h |
| atc-vm | L4 | 7 Files (nur Docs) | Smart Contract VM (27 Opcodes) | 16h |
| atc-wallet | L10 | 7 Files (nur Docs) | Desktop-Wallet (Key-Mgmt, TX-Signing) | 16h |

### 3.2 Repos mit minimaler Implementierung

| Repo | Layer | Aktuell | Ziel | Aufwand |
|------|-------|---------|------|---------|
| atc-atcpkg | L7 | 2 .atc, 353 Zeilen | Vollständiger Package Manager | 12h |
| atc-genesis-engine | L0 | 4 .py, 254 Zeilen | Modulare KI-native Engine | 40h |
| atc-linux-edition | L10 | 1 .rs, 15 Zeilen | egui Desktop-Client | 24h |
| atc-windows-edition | L10 | 1 .rs, 16 Zeilen | egui Desktop-Client | 24h |
| atc-standards | L0 | 0 Code | ATC-01 bis ATC-99 Spezifikationen | 8h |
| atc-whitepaper | L0 | 0 Code | Whitepaper finalisieren | 8h |
| atc-shivacore-tools | L1 | 0 Code | Build-Skripte, Tooling | 8h |

---

## Phase 4: AI Studio & Frontend (Sprint 3.2+)

### 4.1 atc-aistudio — AI Studio
**Status:** 🟢 AKTIV — 248 Files, 199 .ts/.tsx, 57K Zeilen
**Tests:** 6

| Aufgabe | Priorität | Aufwand | Abhängigkeit |
|---------|-----------|---------|--------------|
| TypeScript → ATCLang Migration evaluieren | P2 | 80h+ | ATCLang Frontend-Support |
| Test-Abdeckung: 6 → 30+ Tests | P1 | 8h | — |
| Backend Integration: Blockchain/P2P/Wallet | P1 | 12h | atc-backend |
| ATCLang Editor Component: Syntax-Highlighting | P1 | 6h | atc-ide |
| 3D Render Engine Tab | P2 | 16h | — |
| Game Engine Tab | P2 | 16h | atc-shivamon |

---

## Phase 5: Wiki-Repos (Laufend)

### Wiki-Strategie
36 Wiki-Repos werden automatisch mit den Code-Repos synchronisiert.

| Wiki | Status | Aktion |
|------|--------|--------|
| a-townchain-os-docs | ✅ Aktuell | Haupt-Wiki, 1.112 MD Files |
| kai-os-wiki | ✅ Aktuell | Legacy Wiki, 739 Files, 176 .atc |
| a-townchain-os-wiki | ✅ Archiv | 26 Files, nur MD |
| 33 Modul-Wikis | 🟡 Sync | Auto-Sync mit Code-Repos |

**Wiki-Aufgaben:**
- Wiki-Kapitel auf 69 konsistent halten (KAI-OS Wiki)
- Roadmap/Status bei Code-Änderungen aktualisieren
- ARCHITECTURE_TREES bei neuen Repos regenerieren
- FILE_REGISTER bei Datei-Änderungen aktualisieren

---

## Blocker & Entscheidungen (Michael)

| ID | Blocker | Impact | Benötigte Entscheidung |
|----|---------|--------|----------------------|
| AD-004 | Chain-ID 9000 | Blockiert Mainnet Launch | Neue freie Chain-ID wählen |
| #79 | CI/CD Workflow | Blockiert automatische Tests | GitHub Workflow manuell pushen |
| ATCLang | Parser-Coverage | Blockiert alle .py→.atc Migrationen | 12 Parser-Fixes (oben) abschließen |

---

## Prioritäten-Matrix

```
SOFORT (P0):
  1. ATCLang Parser-Fixes (12 Items) → 20h
  2. Test-Suite aufbauen (alle aktiven Repos) → 30h
  3. AD-004 Chain-ID Klärung → Michael
  4. CI/CD Fix (#79) → Michael

KURZFRISTIG (P1, 1–2 Wochen):
  5. Python→ATCLang Migration (157 .py Stubs) → 80h
  6. Gerüste implementieren (atc-vm, atc-cli, atc-ci) → 40h
  7. Smart Contract Tests → 10h

MITTELFRISTIG (P2, 1–2 Monate):
  8. Rust→ATCLang Migration (134 .rs) → 160h
  9. Frontend-Implementierung (atc-ui, atc-frontend) → 40h
  10. AI Studio Integration → 30h

LANGFRISTIG (Sprint 4.0+):
  11. ShivaCore Kernel → ATCLang Kernel-Support → 80h
  12. Mainnet Launch Vorbereitung → 40h
  13. ATCLang v1.0 (Compiler, VM, Stdlib, Tests) → 100h
```

---

## Aufwand-Schätzung

| Phase | Dauer | Abhängigkeit |
|-------|-------|--------------|
| Phase 1: Kritische Pfade | ~120h | Parser-Fixes zuerst |
| Phase 2: Infrastruktur | ~80h | Phase 1 |
| Phase 3: Gerüste | ~200h | Phase 1+2 |
| Phase 4: AI Studio | ~120h | Phase 2+3 |
| Phase 5: Wiki-Sync | Laufend | Alle Phasen |
| **Total** | **~520h** | |

*520h ≈ 13 Wochen Vollzeit bei 40h/Woche*

---

*Umsetzungsplan v1.0.0 · 2026-08-06 · Aurora (MasterBrain · Base44)*
