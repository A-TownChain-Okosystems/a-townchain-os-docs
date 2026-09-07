# A-TownChain Ecosystem — Decisions Register

> **Stand:** 05.07.2026 13:55 | **Autor:** Aurora (MasterBrain)
> **Agenten-Rolle:** GovernanceAgent (via MasterBrain)

---

## Entscheidungs-Übersicht

| ID | Titel | Status | Sprint | Zuständig |
|----|-------|--------|--------|-----------|
| AD-001 | Hash-Algorithmus | ✅ RESOLVED | — | Aurora |
| AD-002 | EventBus vs IPCBus | ⏳ VALIDATE | 2.4 | **Michael** |
| AD-003 | Flash-Loan Voting Snapshot | ✅ RESOLVED | 2.6 | Aurora |
| AD-004 | Chain-ID 658467 | ✅ RESOLVED (03.09.2026) | 658467 | Michael |
| AD-005 | ATC-97 Agent Protocol Spec | 📐 DRAFT_REVIEW | 3.0 | **Aurora** (Spec drafted — pending Michael review) |
| AD-006 | Python vs Substrate | ✅ RESOLVED | — | Aurora (ATCLang First) |
| AD-007 | EVM Registry | ✅ RESOLVED | — | Aurora (Non-EVM) |
| AD-008 | Reality-Check: 44 Issues re-auditieren/re-open? | ⏳ DECISION | — | **Michael** |
| AD-009 | ATCLANG_SPEC.md-Konsolidierung ✅ (kanonisch: atclang/ATCLANG_SPEC.md) / Bridge-Standards-Dedup (ATC-09/38/69/91) ⏳ | 🟡 TEIL-GELOEST | 08.07.2026 | **Michael** (nur noch Bridge-Dedup) |
| AD-023 | Mainnet-Launch aufgehoben (kein Launch-Ziel/Deadline mehr) | ✅ RESOLVED (06.09.2026) | — | Michael |
| AD-024 | Zielarchitektur Repository-Landkarte — ALLE Repositories erstellen (P0+P1+P2) | ✅ RESOLVED (06.09.2026, 21:30) | — | Michael |
| AD-025 | Spiel umbenannt: Shivamon → Genesis Chronicles | ✅ RESOLVED (06.09.2026, 21:40) | — | Michael |

---

## Resolved Decisions

### AD-001 — Hash-Algorithmus ✅
- **Entscheidung:** SHA-256 (nicht Keccak-256)
- **Begründung:** Non-EVM Chain, keine Ethereum-Kompatibilität nötig
- **Gültig seit:** 2026-06-14

### AD-003 — Flash-Loan Voting Snapshot ✅
- **Entscheidung:** Snapshot bei Proposal-Erstellung (wie Compound/Aave)
- **Implementierung:** dao_live.atc — Snapshot-Mechanismus implementiert
- **Gültig seit:** 05.07.2026

### AD-004 — Chain-ID final: 658467 ✅
- **Status:** RESOLVED 03.09.2026 — Michael entschieden (SUPERSEDED: Platzhalter 9000)
- **Entscheidung:** Chain-ID = **658467** ('ATC' in ASCII: 65-84-67) — systemweit umgestellt
  (Monorepo-Commits 8316604+dc1a094, Hub-Commit dieser Änderung)
- **Begründung:** 9000 ist öffentlich belegt (Evmos Testnet im EVM-Registry) — Kollisionsrisiko
  für die bewusst Non-EVM A-TownChain. 658467 ist praktisch kollisionsfrei.
- **Bewusst unverändert:** Listen-Ports 9000, ATC-9000 NFT-Standard, BIP44-Coin-Type
  `m/44'/9000'` (Wallet-Derivation), Negativ-Testfall 9999, network_magic 0x0A0C23A0
- **Gültig seit:** 2026-09-03

### AD-006 — Python vs Substrate ✅
- **Entscheidung:** Weder Python noch Substrate — Alles wird ATCLang
- **Begründung:** ATCLang First Policy (ATC-99), proprietäre Sprache für gesamte Codebasis
- **Gültig seit:** 2026-06-12

### AD-007 — EVM Registry ✅
- **Entscheidung:** EVM Registry irrelevant — A-TownChain ist Non-EVM
- **Begründung:** Kein EVM-Registry-Eintrag nötig für Non-EVM Chains
- **Gültig seit:** 2026-06-14

---

## Open Decisions

### AD-002 — EventBus vs IPCBus ⏳
- **Status:** VALIDATE — Michael muss entscheiden
- **Problem:** Inter-Modul-Kommunikation via EventBus (Publish/Subscribe) oder IPCBus (direkte Messages)
- **Implementierung:** ipc_bus.atc (101L) vorhanden, EventBus als Alternative
- **Sprint:** 2.4
- **Blocker:** Nein — aber Entscheidung nötig vor Sprint 2.4 Abschluss

### AD-005 — ATC-97 Agent Interaction Protocol 📐 DRAFT_REVIEW
- **Status:** DECISION — Aurora arbeitet Spezifikation aus
- **Problem:** Message-Format, Fehlerbehandlung und Timeouts für Agent-Kommunikation nicht spezifiziert
- **Implementierung:** kai_routes.atc (228L) — teilweise
- **Sprint:** 3.0
- **Aurora arbeitet:** ATC-97 DRAFT Spezifikation bis Sprint 3.0

---


### AD-008 — Reality-Check: 44 Issues mit gebrochener Datei-Referenz ⏳
- **Status:** DECISION — Michael muss ueber weiteres Vorgehen entscheiden
- **Problem:** 44 von 78 geschlossenen Issues (56%) referenzieren im Issue-Body Dateien, die in
  keinem der 3 Repos nachweisbar sind (auch nicht unter geaendertem Pfad) — 10 davon CRITICAL
  (ATCLang-Kern #72/#73/#81, Konsens #74, Smart-Contract-Engine #76, Sharding #84, Latency #83)
- **Optionen:** (1) Re-Audit einzeln, (2) Re-Open mit Label `reality-check-failed`,
  (3) nur als Backlog-Referenz dokumentieren ohne Issue-Historie zu aendern, (4) Audit-Score korrigieren
- **Details:** `docs/REALITY_CHECK_2026-07-06.md` (Hauptreport)
- **Gefunden von:** Aurora (aurora-base44-superagent-6a2756186106d6f0fbb105b5), 06.07.2026

### AD-010 — WHITEPAPER.md referenziert veraltete Solidity-Architektur ⏳
- **Status:** DECISION — Michael muss Prioritaet fuer Content-Rewrite festlegen
- **Problem:** docs/whitepaper/WHITEPAPER.md verlinkt auf ~15 Dateien einer alten
  Solidity/Python-Architektur (blockchain/contracts/solidity/*.sol, backend/api/routes/,
  gateway/router.py, architecture/*.md), die durch die ATCLang-Migration nicht mehr existieren.
  Reine Link-Reparatur reicht nicht -- der Fliesstext beschreibt noch die alte Architektur.
- **Umfang:** ~15-19 kaputte Referenzen je Repo (siehe Markdown-Audit 08.07.2026)
- **Optionen:** (1) Volle Whitepaper-Ueberarbeitung auf ATCLang-Architektur, (2) Whitepaper als
  historisches Dokument kennzeichnen + neues WHITEPAPER_V2.md erstellen, (3) nur Links entfernen
- **Gefunden von:** Aurora (aurora-base44-superagent-6a2756186106d6f0fbb105b5), 08.07.2026

**Update 08.07.2026:** 17/19 Referenzen behoben -- Solidity-Dateien (KAIGovernance.sol,
KAIMarketplace.sol, KAIBridge.sol, GenesisToken.sol, deploy.ts) auf die realen ATCLang-Aequivalente
(governance_contract.atc, marketplace_contract.atc, bridge_contract.atc, genesis_token.atc,
generate_validators.atc) umverlinkt, restliche Pfad-Tiefen korrigiert. Verbleibend: 1 Referenz auf
ATCLANG_SPEC.md (blockiert durch AD-009 5-fach-Duplikat, keine eindeutige Zieldatei waehlbar).
Status auf TEIL-GELOEST gesetzt, Rest haengt an AD-009-Entscheidung.

### AD-009 — ATCLANG_SPEC.md-Konsolidierung ✅ / Bridge-Standards-Dedup ⏳
- **Status:** Problem 1 GELOEST (08.07.2026) -- Problem 2 (Bridge-Dedup) weiterhin DECISION
- **Problem 1:** `ATCLANG_SPEC.md` existiert an 5 verschiedenen Pfaden parallel
  (`atclang/`, `docs/atclang/ATCLANG_SPEC_FULL.md`, `module-docs/atclang/`,
  `docs/wiki/kai-os/code/atclang/`, `aistudio/temp_repo/atclang/`) — Ursache fuer ~270 verbleibende
  kaputte Links
- **Problem 2:** Cross-Chain Bridge hat 4 ueberlappende Standard-Dokumente fuer dasselbe Thema
  (ATC-09, ATC-38, ATC-69, ATC-91) — Dokumentations-Drift im Standards-Register
- **Details:** `docs/REALITY_CHECK_2026-07-06.md` (Nachtrag 1 + 2)
- **Gefunden von:** Aurora (aurora-base44-superagent-6a2756186106d6f0fbb105b5), 06.07.2026

**Loesung Problem 1 (08.07.2026):** Kanonische Version bestimmt: `atclang/ATCLANG_SPEC.md`
(v1.0.0, Standard ATC-92, Status STABLE, Datum 2026-07-05) -- deutlich vollstaendiger als die
4 anderen Kopien (v0.1.0-alpha Stub 913 Bytes, v0.2.0-alpha FULL-Version 9184 Bytes). Diese
Version wurde 1:1 nach `atclang/ATCLANG_SPEC.md` in a-townchain-os-docs und kai-os-wiki kopiert
(existierte dort noch nicht). Die 4 anderen Kopien in allen 3 Repos (module-docs/, docs/atclang/,
docs/wiki/kai-os/code/, docs/wiki/kai-os/docs/) wurden durch Redirect-Stubs ersetzt, die auf die
kanonische Datei verweisen -- Links bleiben so funktionsfaehig, aber es gibt nur noch 1 Quelle
der Wahrheit. Alle WHITEPAPER.md-Referenzen entsprechend korrigiert.



### AD-011: Issue #18 Node-Rollen-Abweichung (Docker Testnet)
**Status:** ENTSCHIEDEN (sachlich begruendet) | **Datum:** 08.07.2026

- **Kontext:** Issue #18 spezifiziert ein 5-Node-Docker-Testnet als 1 Bootstrap + 1 Validator
  + 1 Miner + 2 Full Nodes. Die tatsaechliche Implementierung (`docker-compose.yml`, Root)
  nutzt 1 Bootstrap + 2 Validatoren + 1 Full Node + 1 Archive Node -- kein dedizierter
  "Miner"-Service.
- **Begruendung:** A-TownChain ist PoS/PoH-basiert (nicht PoW) -- ein "Miner"-Service ist
  architektonisch nicht vorgesehen. Die Abweichung ist eine Altlast aus einer frueheren
  PoW-Planungsphase des Issues, keine fehlende Implementierung.
- **Entscheidung:** Issue #18 gilt trotz Rollen-Abweichung als sachlich erfuellt. Keine
  Code-Aenderung noetig, nur Dokumentations-Klarstellung.
- **Details:** `docs/CLUSTER_ARCHITECTURE.md` Abschnitt 4.
- **Gefunden von:** Aurora (aurora-base44-superagent-6a2756186106d6f0fbb105b5), 08.07.2026

---

*Decisions Register v1.0.0 — Aurora (MasterBrain · Base44) · 05.07.2026*

---

### AD-012: ShivaCore als Microkernel/Hybrid-Microkernel von Globus OS 🏗️ VERBINDLICH
**Status:** ENTSCHIEDEN | **Datum:** 06.09.2026

- **Kontext:** ShivaCore (Rust-Kernel, K29, 674/674 Tests) ist der unterliegende Systemkern
  von Globus OS. Bisherige Wachstumshistorie (K29) hat Blockchain-Komponenten in den
  Kernel-Crate eingebracht. Abgrenzung zu Aurora AI, Globus Shell und ATCLang war nirgends
  verbindlich festgeschrieben.
- **Entscheidung:** ShivaCore wird als Microkernel/Hybrid-Microkernel konzipiert.
  Kernel-Kontext enthaelt NUR sicherheits-/zeitkritische Primitive: Scheduler, Memory,
  IPC, Interrupts, Capability Security, Process/Thread Core, Minimal VFS, HAL, Syscall ABI.
  Filesystem/Network/GPU/Audio/AI/Blockchain/Container/ATCLang-Runtimes laufen im
  User/Service Space. ATCLang erreicht den Kernel ausschliesslich ueber
  Syscall ABI + Capability Check (kein direkter Kernel-Speicherzugriff).
  4 Sicherheitszonen (L0 Hardware, L1 Kernel, L2 Privileged Services, L3 User/AI/Games)
  plus Sandbox-Modell. Blockchain ist System Service (A-TownChain Node Service), AI
  arbeitet ueber Kernel-Event-Bridge mit Capability Tokens — keine unbeschraenkte
  Kernel-Kontrolle.
- **Migration:** Blockchain-Komponenten im Kernel-Crate (blockchain/consensus/genesis/
  gossip_bridge/security_audit.rs) sind in den Service-Space zu migrieren
  (eigener Architektur-Sprint, API-kompatibel via KernelState/cross_subsystem).
  Offene Ziel-Subsysteme: Syscall ABI, HAL, Interrupt Manager, Driver Manager,
  Time Manager, Virtual Memory.
- **Details:** `docs/architecture/SHIVACORE_KERNEL_ARCHITECTURE.md` (verbindliche
  Spezifikation mit Delta-Analyse Ist-Stand K29 vs. Ziel).
- **Naechster Baustein:** ShivaCore v0.1 Kernel-Spezifikation (Boot → Memory Model →
  Scheduler → IPC → Syscalls → Capability Model → Driver Model → ATCLang ABI).

---

### AD-013: ShivaCore v0.1 Architektur-Gate — SC-ARCH-Freeze-Regeln 🏗️ VERBINDLICH
**Status:** ENTSCHIEDEN (Freeze) | **Datum:** 06.09.2026

- **Kontext:** AD-012 definierte die Microkernel-Architektur. Vor Implementierungsbeginn
  wurde der Entwurf normativ geschaerft, um Kernel-/OS-Vermischung zu verhindern.
- **Kern-Gate:** ShivaCore ist ein echter Capability-Microkernel; Globus OS ist das
  darauf aufbauende Userspace-Betriebssystem.
- **Schaerfungen:** (1) Capabilities im Kern sind Kernel-enforced Handles
  (CSpace-Slots, Objektverwaltung), NICHT kryptografisch — Krypto nur fuer
  Cross-Domain (K29-remote_caps bleibt getrennte Erweiterung). (2) globus-init
  erhaelt explizite Initial-Capabilities via Initial Task (CSpace Root, BootInfo,
  Untyped/IRQ/Device Caps) — KEIN Unix-Root-Modell. (3) Kernel Object Model:
  Thread/AddressSpace/PageTable/Frame/CNode/Endpoint/Notification/IRQ/Device/
  UntypedMemory/Timer. (4) IPC als Bus des OS; Control Plane (IPC) vs. Data Plane
  (Shared Memory/DMA/Zero-Copy). (5) Scheduling Domains (HARD/Soft RT, Interactive,
  System, Normal, Background) statt reiner Prioritaetszahl. (6) Kernel MUSS unabhaengig
  sein von AI/Blockchain/ATCLang/GUI (deterministisch, bootstrapping-faehig).
  (7) DefenderGPT beobachtet/empfiehlt nur — Enforcement bleibt deterministisch im
  Kernel. (8) Objektorientierte Syscall-ABI (TASK/MEMORY/IPC/CAPABILITY/IRQ/TIME).
  (9) Boot Chain: UEFI→Limine→ShivaCore→globus-init→Globus OS.
- **Freeze-Regeln:** SC-ARCH-001 bis SC-ARCH-010 (siehe Gate-Dokument, Tabelle 13).
- **Implementierungs-Reihenfolge:** SC-001 (Kernel Object Model) bis SC-013
  (ATCLang ABI); Aurora/A-TownChain/Genesis erst danach anbinden.
- **Details:** `docs/architecture/SHIVACORE_V01_ARCHITECTURE_GATE.md`
---

## AD-014: 5-Produkt-Repo-Struktur

**Datum:** 06.09.2026 · **Status:** RESOLVED/UMGESETZT · **Entscheider:** Owner (ShivaCore) · **Umsetzung:** Agent Aurora

**Beschluss:** Neben dem Dual-Repo-Modell (Monorepo + Docs-Hub) werden 5 eigenständige
Produkt-Repos eingerichtet: `atclang` (ATCLang), `a-townchain` (Blockchain),
`globus-os` (Betriebssystem/ShivaCore-Kernel), `aurora-ai` (KI), `genesis-engine` (Game-Engine).
Module wurden aus `src/modules/` in die Produkt-Repos kopiert (Monorepo unverändert,
bleibt Integrations-/Deployment-Zentrale). `atclang` als vormals archiviertes Repo
reaktiviert (Historie erhalten, Inhalt aus Monorepo-Audits verlustfrei gesichert).

**Pflichten:** Produkt-Entwicklung im Produkt-Repo; Integration/Deployment im Monorepo;
Doku im Docs-Hub. Details & Mapping: [REPOSITORY_MAP.md](REPOSITORY_MAP.md).
API-Verifikation: 5/5 Repos aktiv, HEAD-Commits bestätigt (06.09.).
---

## AD-015: ShivaCore-Kernel-Repo-Trennung

**Datum:** 06.09.2026 · **Status:** RESOLVED/UMGESETZT · **Entscheider:** Owner (ShivaCore) · **Umsetzung:** Agent Aurora

**Beschluss:** Der ShivaCore-Kernel erhält ein eigenständiges Repository: `atc-shivacore`
(reaktiviertes Archiv-Repo, 181-Commit-K29-Entwicklungshistorie erhalten).
Inhalt: `modules/atc-shivacore` (Kernel-Crate, K29, 674/674 Tests, Chain-ID 658467)
+ `modules/atc-shivacore-tools` — Stand Monorepo 06.09.2026.
Entsprechend AD-012/AD-013 (Kernel ≠ OS): GlobusOS-Repo behält nur den Userspace
(Shell, Desktop, FS, Net, Registry, Bootloader, Drivers, Editionen); Kernel-Referenz
im globus-os-README. Monorepo bleibt Integrations-Zentrale (Unified Cargo Workspace).
Kernel-Entwicklung künftig im Kernel-Repo; Service-Space-Migration der
Blockchain-Module aus dem Kernel-Crate bleibt eigener Sprint (unverändert).

API-Verifikation 06.09.: atc-shivacore aktiv (HEAD gepusht), globus-os Rückbau
gepusht, Beschreibungen/Topics aktualisiert.

---

## AD-016: Löschung der 124 archivierten Alt-Repos

**Datum:** 06.09.2026 · **Status:** RESOLVED/UMGESETZT · **Entscheider:** Owner (ShivaCore) · **Umsetzung:** Agent Aurora

**Beschluss:** Nach vollständiger Daten-Sicherung wurden alle 124 archivierten
Alt-Repos (Namenspräfix `archiviert-`) aus der Organisation gelöscht.
Vorgehen: (1) SHA-256-Verifikation über alle 2.966 Dateien gegen Monorepo+Hub,
(2) Rettung von 527 Original-Ständen mit abweichendem Hash nach
docs/archive/repos-rescue/ (Commits 53a6a41, 51d949f), (3) Issues-Dump des
einzigen Issues führenden Alt-Repos (atc-genesis-engine, 9 Issues),
(4) API-Löschung 124/124, 0 Fehler (delete_repo-Scope, vom Owner autorisiert).

**Ergebnis:** Organisation umfasst exakt 8 aktive Repos: a-townchain-os
(Integration/Monorepo), a-townchain-os-docs (Wiki/Hub), atc-shivacore (Kernel),
atclang, a-townchain, globus-os, aurora-ai, genesis-engine (Produkte).
Git-Commit-Historien der gelöschten Repos sind mit ihnen entfernt; der finale
Datei-Stand jedes Repos ist vollständig erhalten (Hub-Rescue bzw. Monorepo).

---

## AD-017: Source-of-Truth & Modul-Sync (Produkt-Repos -> Monorepo)

**Datum:** 06.09.2026 · **Status:** RESOLVED/VERBINDLICH · **Entscheider:** Owner (ShivaCore, Entscheidung an Agent Aurora delegiert) · **Umsetzung:** Agent Aurora

**Beschluss:** Kanonische Entwicklungsquelle fuer Modul-Code sind die
Produkt-Repos (atc-shivacore, atclang, a-townchain, globus-os, aurora-ai,
genesis-engine). Der Monorepo a-townchain-os ist ausschliesslich
INTEGRATIONSZIEL (Cargo-Workspace, Launch-Stack, Docker, CI, Gesamttests).

**Regeln:**
1. Modul-Code wird nie mehr direkt im Monorepo bearbeitet — nur ueber
   `scripts/sync_modules.py` einspielen (strikte Richtung: Produkt -> Monorepo).
2. Modi: `--check` = SHA-256-Divergenz-Bericht, `--sync` = einspielen
   (Produkt-Stand gewinnt). Sync-Punkte: vor Releases, Integration-Builds,
   Mainnet-Freeze, bei CI-Abweichung.
3. Monorepo-Integrationsdateien (Workspace-Files, docker/, scripts/, .github/,
   Meta-Files) bleiben vom Sync unberuehrt.
4. Dateien, die nur im Monorepo existieren, werden vom Sync NIE geloescht.
5. Startzustand 06.09.: 1.377 Dateien geprueft, 0 divergent, 0 neu (sauber).

**Rationale:** AD-014/AD-015 verlagern die Produkt-Entwicklung in die
Produkt-Repos (atc-shivacore 181 Commits, atclang 91 Commits Historie);
der Mainnet-Launch am 15.09. benoetigt den Monorepo als Workspace-Zentrale.
Damit ist die im Cross-Repo-Audit identifizierte Luecke (kein Sync-Mechanismus
-> Divergenz ab erstem produktseitigen Commit) geschlossen.

---

## AD-018: Gesamtvault im Wiki-Repository — Neuaufbau der Repositories

**Datum:** 06.09.2026 · **Status:** RESOLVED/UMGESETZT · **Entscheider:** Owner (ShivaCore) · **Umsetzung:** Agent Aurora

**Beschluss:** Der komplette Stand des Monorepos a-townchain-os (Commit
8606e50: Launch-Stack, 60 Module, 2.500+ Dateien) wurde strukturgetreu nach
`docs/archive/monorepo-full/` in diesem Wiki-Repository verschoben. Der
Monorepo wurde daraufhin geleert (Redirect-README). Die uebrigen Repositories
(Monorepo + Produkt-Repos) werden neu aufgebaut; dieser Vault ist die
kanonische Quelle dafuer. Git-Historien bleiben als Reversibilitaetsnetz.

**Mainnet-Hinweis:** Der Launch-Stack (docker-compose, 10 Dienste) liegt
jetzt ausschließlich hier; fuer den 15.09.2026 muss der Rebuild ihn als
ersten Schritt restaurieren (Kopie aus dem Vault, nicht Neuentwicklung).

---

## AD-019: ATCLang 1.0 Master-Architektur verbindlich — Rebuild Phase 1

**Datum:** 06.09.2026 · **Status:** RESOLVED/UMGESETZT · **Entscheider:** Owner (ShivaCore) · **Umsetzung:** Agent Aurora

**Beschluss:** ATCLang-Zielarchitektur 1.0.0 EINGEFROREN (ARCHITECTURE
FREEZE: GO): Trennung nach Verantwortlichkeit, nicht nach Schlagworten —
frontend/semantics/ir/compiler/bytecode/artifact/vm/runtime/contracts/abi/
host/security/profiles/stdlib/package/cli. Kernentscheidungen: runtime
statt execution (VM = wie, Runtime = in welchem Zustand), artifact
Singular, security zentral (determinism dort, nicht in profiles),
Konsensgrenze mit Verbotsliste fuer Contract-Kontext, Source-of-Truth:
specs/ -> Implementierung -> Conformance.

**Konsolidierung (atclang 044604f):** Duplikate entfernt (atc-vm: 116-LOC-
Stub vs 978 LOC kanonisch; atc-stdlib 4/9 identisch; atc-atclang: kleinere
Parallel-Implementierungen) — bewahrt im Monorepo-Vault + Git-Historie.
modules/atclang -> src/atclang (kanonisch), frontend/-Zusammenfuehrung,
1.0-Subsystem-Platzhalter. Pipeline: Import + compile_source End-to-End
OK; pytest 74/106 gruen, 32 rot (Bytecode-API-Konformanz = Phase 2).
Reihenfolge geloeschter Mehrfachquellen: MULTIPLE SOURCES OF TRUTH →
aufgeloest (eine Implementierung je Verantwortlichkeit).

---

## AD-020: Rebuild-Startzustand — alle Repos geleert, Wiki-Vault ist Quelle

**Datum:** 06.09.2026 · **Status:** RESOLVED/UMGESETZT · **Entscheider:** Owner (ShivaCore) · **Umsetzung:** Agent Aurora

**Beschluss:** Alle Repositories (ausser Wiki-Hub und atclang) wurden geleert;
der Neuaufbau startet aus dem Wiki-Vault. Vor der Leerung wurde byte-Identitaet
verifiziert: 1.239 Modul-Dateien der 5 Produkt-Repos + 2.157 Dateien des
Monorepos sind identisch in docs/archive/monorepo-full/ gesichert (0 fehlen).

**Endzustand der 8 Repos:**
- a-townchain-os-docs: WIKI-VAULT (alles Wissen: monorepo-full, monorepo-legacy,
  wiki-Archive, repos-rescue, Standards, Architektur-Entscheidungen)
- atclang: REBUILD PHASE 1 abgeschlossen (AD-019: 1.0-Struktur, 044604f)
- a-townchain-os, atc-shivacore, a-townchain, globus-os, aurora-ai,
  genesis-engine: GELEERT (Redirect-README; Git-Historien bleiben als
  Reversibilitaetsnetz erhalten)

**Reihenfolge des Wiederaufbaus:** Empfohlen zuerst Launch-Stack-Restauration
aus dem Vault (Mainnet 15.09.), dann atc-shivacore gem. AD-013 (SC-001…),
danach die uebrigen Produkt-Repos.

---

## AD-021: Sprachstrategie — ATCLang Rust-first, Python Referenz; Aurora Rust-Core + Python-AI-Layer

**Datum:** 06.09.2026 · **Status:** RESOLVED/VERBINDLICH · **Entscheider:** Owner (ShivaCore) · **Umsetzung:** Agent Aurora

**Beschluss:** ATCLang wird Rust-first: Compiler (Lexer/Parser/AST/Semantics/
ATC-IR/Optimizer/Codegen), Bytecode-Verifier, ATVM, Runtime, ABI/Artifact-
Validator, Security/Sandbox und CLI kanonisch in Rust. Python bleibt als
(1) Referenz-Implementierung — bestehender Code wird NICHT weggeworfen
(kein Rebuild), (2) SDK, (3) Test-/Fuzzing-Tooling, (4) AI-Integration.

**Phasen:** Phase 1: Python-Referenz weiterentwickeln (Sprachentwicklung).
Phase 2: Rust-Kernkomponenten (atclang-core/-ir/-bytecode/-verifier/-vm/
-runtime/-abi). Phase 3: Rust = kanonische Produktionsimplementierung.

**Dual-Stack-Differential-Modell:** Rust-Stack und Python-Referenz muessen
fuer identische Programme identische ASTs, IR, Bytecode und State-Transitions
erzeugen — Differential Testing wird zum verbindlichen Conformance-Kriterium.

**Aurora AI:** Rust Core (Model Manager, Scheduler, HAL, Security, IPC,
Plugin-Runtime) + Python AI-Layer (PyTorch/ONNX/LLM auf ROCm).
Kurzform: ATCLang -> Rust-first. Aurora -> Rust-Core + Python-AI-Layer.

**Begruendung:** Compiler != Runtime, beides Rust. ATCLang ist keine
Skriptsprache: Smart Contracts erfordern Memory Safety, Determinismus,
Geschwindigkeit, geringe Overheads, sichere Concurrency, Ressourcen-
Kontrolle und eine harte Host/VM-Security-Boundary.

---

## AD-022: ATCLang Architecture Baseline v1.0 — Rust Canonical Core, 21-Crate-Layout, Gates G0-G19

**Datum:** 06.09.2026 · **Status:** RESOLVED/VERBINDLICH · **Entscheider:** Owner (ShivaCore) · **Umsetzung:** Agent Aurora

**Beschluss:** ATCLang 1.0 = Rust Canonical Core + Python Reference/Tooling +
formal spezifizierte Bytecode/ABI/Artifact/VM-Schichten. Zielstruktur: crates/
mit 21 Rust-Crates (atclang-lexer, -parser, -ast, -semantics, -ir,
-ir-verifier, -compiler, -bytecode, -bytecode-verifier, -abi, -artifact, -vm,
-runtime, -host, -security, -profiles, -contracts, -gas, -state, -package,
-cli), python/ (sdk, reference, testing, fuzzing, tools), specs/ (language,
bytecode, abi, artifact, vm, gas, security, profiles), tests/ (conformance,
determinism, differential, …).

**Kernsatz:** Der Compiler erzeugt Code. Der Verifier entscheidet, ob Code
gueltig ist. Die ATVM fuehrt ausschliesslich verifizierten Code
deterministisch aus.

**Kernpunkte:** ATC-IR als zentrale Schicht (Typen, Kontroll-/Datenfluss,
Calls, Storage, Capabilities, Gas, Entry Points, ABI) · Independent Verifier
als Trust Boundary (vertraut nie dem Compiler) · ATCA-Artifact als Deployment
Unit (Magic…Signature) · Profiles CONTRACT/APPLICATION/SYSTEM/UNRESTRICTED ·
Capability-Security Deny-by-Default · Determinismus als Protokollregel ·
Konsensus-Grenze Transaction→…→Consensus · Differential Testing
Rust-vs-Python (Abweichung = FAIL) · Security Gates G0-G19, KEIN FREEZE vor
G18 (Security Audit) · Python kontrolliert NIE die Konsensus-Ausfuehrung.

**Migration:** KEIN zweiter Rewrite — Python-Referenz bleibt erhalten;
Pfad: preserve → specification extraction → formal Spec → Rust Canonical
Core → Differential Testing → Conformance → Security Audit → Freeze.
G0 (Repository Cleanup) erledigt; G1 (Language Specification) naechstes Gate,
specs/-Skelett angelegt. Verfeinert AD-019/AD-021.
Doku: atclang docs/ATCLANG_BASELINE_V1.md (Commit a7e1bd4).

---

## AD-023: Mainnet-Launch aufgehoben — kein Launch-Ziel, keine Deadline

**Datum:** 06.09.2026 · **Status:** RESOLVED/VERBINDLICH · **Entscheider:** Owner (Michael) · **Umsetzung:** Agent Aurora

**Beschluss:** Der Mainnet-Launch ist als Ziel und als Deadline **aufgehoben**.
Das Datum 15.09.2026 ist kein Launch-Termin mehr. Es gibt **kein Datum** für
einen Mainnet-Launch — ein späterer Launch ist nicht terminiert und wird erst
durch eine neue Owner-Entscheidung festgelegt.

**Was das bedeutet:**
1. AD-018/AD-020-Reihenfolge wird **nicht** mehr durch die Mainnet-Deadline
   getrieben. Der Launch-Stack wird aus dem Vault restauriert, wenn es
   gebraucht wird (Testnet/Integration), nicht als Mainnet-Vorbereitung.
2. Die Rebuild-Priorität verschiebt sich auf **Qualität statt Termin**:
   atc-shivacore SC-001…SC-013 und die ATCLang-Gates G0-G19 (AD-022) ohne
   Zeitdruck — insbesondere G18 (Security Audit) vor jedem Freeze.
3. Roadmap-Milestone 4.0/4.1 („Mainnet Launch") ist **entfernt**; ein
   künftiger Launch erhält eine neue Milestone-Nummer und ein neues Datum,
   sobald der Owner es beschließt.
4. Historische Erwähnungen (Issues #36/#52/#69–71, AD-018-Text, Wiki-Kapitel,
   BaFin-Bericht) bleiben als Historie unangetastet; sie sind durch AD-023
   **superseded**, was den Launch-Zeitpunkt betrifft.

**Bewusst unverändert:** Testnet (5 Nodes, Docker) bleibt vollständiges Ziel;
Chain-ID 658467 (AD-004) bleibt final und wird beim künftigen Launch genutzt.

**Gültig seit:** 06.09.2026 (21:25 UTC+2)

---

## AD-024: Zielarchitektur Repository-Landkarte — ALLE Repositories erstellen (RESOLVED)

**Datum:** 06.09.2026 · **Status:** ✅ RESOLVED/UMGESETZT · **Entscheider:** Owner (Michael, 21:30 UTC+2: „Alle repository erstellen") · **Umsetzung:** Agent Aurora

**Beschluss (Variante a — erweitert):** SÄMTLICHE Repos der Landkarte werden JETZT
angelegt (P0+P1+P2 = 14 neue Repos). AD-016-Regel „exakt 8 aktive Repos" ist damit
fortgeschrieben: Ziel sind 22 aktive Repos mit vertikalen Produktgrenzen.
Wobei möglich wird Vault-Inhalt restauriert (atc-sdk, atc-contracts, atc-wallet,
atc-explorer, shivamon); neue Repos (atc-node, atc-indexer, atc-mining, atc-interop,
atc-oracle, atc-storage, atc-launchpad, atc-marketplace, atc-compute) erhalten
professionelle Grundstruktur (README/Architektur/LICENSE/STATUS).

**Vorschlag:** Wachstum der Organisation **vertikal entlang konkreter Produkte/Protokolle**
(Protocol → Node → SDK → Wallet/Explorer/Contracts → Applications), nicht horizontal
über weitere Core-Repos. Bestehende 8 Repos bleiben (🟢); neue vertikale Repos in Phasen:

- **Phase 1 (P0):** `atc-sdk` (Developer-Plattform), `atc-node` (Node-Binary getrennt vom
  Protocol-Core), `atc-contracts` (Standards + Contracts, NICHT im Core), `atc-wallet`
  (eigene Security Boundary: Keys/Seed/Signing/Recovery/Hardware)
- **Phase 2 (P1):** `atc-explorer` + `atc-indexer`, `atc-mining`, `atc-interop`
  (Bridges/IBC = eigene Sicherheitsdomäne), `shivamon` (Game getrennt von genesis-engine:
  Engine = Technologie, Shivamon = Produkt)
- **Phase 3 (P2):** `atc-oracle`, `atc-storage`, `atc-launchpad`, `atc-marketplace`,
  `atc-compute`

**Abgleich mit bestehender Struktur (AD-014–AD-020) — WICHTIG:**
1. Die meisten „neuen" Repos **existieren bereits als Vault-Module** in den Produkt-Repos:
   `a-townchain` enthält lt. REPOSITORY_MAP bereits atc-blockchain, atcnet, atc-wallet,
   atc-contracts, atc-bridge, atc-dex, atc-explorer, atc-assets, atc-zkp, atc-governance,
   atc-dns, atc-testnet (369 Dateien). atc-sdk, atc-contracts, atc-wallet, atc-explorer,
   atc-shivamon sind im Vault (`docs/archive/monorepo-full/`) nachweislich vorhanden.
   NEU ohne Vault-Bestand: atc-node, atc-indexer, atc-mining, atc-interop, atc-oracle,
   atc-storage.
2. **Rollen-Korrekturen zur Analyse:** `atc-shivacore` ist der **Microkernel** (AD-012/013,
   SC-001…SC-013), KEINE AI-/Runtime-Schicht. `a-townchain` ist bereits als Blockchain-Produkt-Repo
   zugewiesen (Chain-ID 658467). AI liegt bei `aurora-ai`.
3. **Konflikt mit AD-016:** AD-016 stellte auf „exakt 8 aktive Repos" ab (nach Löschung
   von 124 Alt-Repos wegen Wartungs-Overflow). AD-024 würde die Org mittelfristig wieder
   auf ~20 Repos wachsen lassen — mit saubereren Grenzen, aber mehr Wartungsoverhead.
4. **Timing per AD-023:** Kein Termin-Druck. Empfehlung: Repos erst anlegen, wenn der
   Rebuild den jeweiligen Bereich erreicht (Qualitäts-Gates), und dann als
   **Abspaltung aus dem Vault** (Restore), nicht als Neuentwicklung.

**Vorgeschlagene Promotion-Kriterien (statt fixer Phasen-Termine):**
- `atc-sdk`: sobald ATCLang Rust-ABI/Artifact-Stabilisierung erreicht ist (AD-022-Gates)
- `atc-node`: sobald Protocol-Core-Interface (AD-013-Kette) eingefroren ist
- `atc-wallet`: sobald atclang-Runtime Signing/HD-Wallet-Referenz (BIP44 m/44'/9000') stabil
- `atc-contracts`: sobald ATVM-Contract-Kontext (AD-019/022) verifiziert ausführt
- Shivamon-Split: sobald genesis-engine den ECS-/World-Kern stabil liefert

**Entscheidung offen:** (a) P0-Repos SOFORT als eigenständige Repos anlegen (widerruft
AD-016-„exakt 8") oder (b) vertikale Grenzen als Modulgrenzen in den bestehenden
Produkt-Repos führen und Repos erst bei Erreichen der Promotion-Kriterien abspalten.
Empfehlung des Agents: (b) — qualitätsgetrieben per AD-023, keine Repos vor Content.

---

## AD-025: Spiel umbenannt — Shivamon → Genesis Chronicles

**Datum:** 06.09.2026 · **Status:** RESOLVED/UMGESETZT · **Entscheider:** Owner (Michael) · **Umsetzung:** Agent Aurora

**Beschluss:** Das Spiel „Shivamon" heißt ab sofort **„Genesis Chronicles"**.
Umbenannt wurden: GitHub-Repo (`shivamon` → `genesis-chronicles`, Alte URL
leitet automatisch um), Spiel-Titel und NFT-Bezeichnung in der gesamten Doku,
Contract-Identifier (`ShivamonContract` → `GenesisChroniclesContract`,
`ShivamonNFT` → `GenesisChroniclesNFT`, `ShivamonStats` →
`GenesisChroniclesStats`), Datei-/Verzeichnisnamen (snake_case) und der
Package-Name (`genesis-chronicles`).

**Bewusst unverändert:** Das Wiki-Vault-Archiv
(`docs/archive/monorepo-full/.../atc-shivamon/`) als historische Quelle;
AD-024-Entscheidungstext (Historie); Genesis-Engine-Repo-Name. Die ATC-9000
NFT-Standard-Nummer bleibt. Git-Historie des Repos bewahrt alle alten Namen.

**Kontext:** Repo war am 06.09. per AD-024 aus Vault-Inhalt (atc-shivamon)
restauriert und als eigenständiges Produkt-Repo neben genesis-engine angelegt
(Engine = Technologie, Spiel = Produkt). Keine Live-Systeme betroffen.

---

## AD-026: Repository-Bauhierarchie — sequenzieller Aufbau nach Abhängigkeit

**Datum:** 07.09.2026 · **Status:** RESOLVED/VERBINDLICH · **Entscheider:** Owner (ShivaCore: „erst atclang, dann Kernel, KI, Blockchain, Betriebssystem usw.") · **Umsetzung:** Agent Aurora

**Beschluss:** Die 22 Repos bauen in fester Reihenfolge aufeinander auf:
L0 atclang → L1 atc-shivacore → L2 aurora-ai → L3 a-townchain → L4 globus-os
→ L5 Blockchain-Services (node, contracts, wallet, sdk, storage, compute,
oracle, indexer, explorer, interop, mining, marketplace, launchpad) →
L6 genesis-engine → genesis-chronicles → L7 a-townchain-os (Integration).
Wiki-Hub parallel. Ein Layer startet erst nach Gate-Erreichung des
vorherigen. AD-020-Empfehlung (Launch-Stack zuerst) damit ersetzt —
AD-023 hat die Launch-Deadline aufgehoben.

**GitHub-seitig umgesetzt:** Layer-Tags [L0]…[L7]/[Hub] in allen 22
Repo-Beschreibungen, Topics build-layer-N, verbindliche Tabelle in
REPOSITORY_MAP.md, AGENT_MANIFEST im Monorepo gespiegelt.
