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
| AD-004 | Chain-ID (Platzhalter 9000) | 🔴 OPEN (reopened 06.07.2026) | — | Michael |
| AD-005 | ATC-97 Agent Protocol Spec | ⏳ DECISION | 3.0 | **Aurora** (Spec ausarbeiten) |
| AD-006 | Python vs Substrate | ✅ RESOLVED | — | Aurora (ATCLang First) |
| AD-007 | EVM Registry | ✅ RESOLVED | — | Aurora (Non-EVM) |
| AD-008 | Reality-Check: 44 Issues re-auditieren/re-open? | ⏳ DECISION | — | **Michael** |
| AD-009 | ATCLANG_SPEC.md-Konsolidierung (5 Kopien) + Bridge-Standards-Dedup (ATC-09/38/69/91) | ⏳ DECISION | — | **Michael** |

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

### AD-004 — Chain-ID NICHT final 🔴
- **Status:** REOPENED 06.07.2026 — Michael: "Wir haben noch keine Chain-ID, 9000 ist ID von Ethereum(-Oekosystem)" (bestaetigt: 9000 = Evmos Testnet auf chainlist.org)
- **Entscheidung:** Noch offen. 9000 ist nur Platzhalter im Code, keine echte Chain-ID vergeben
- **Begründung:** XDC Network nutzt 9000 im EVM-Registry, aber A-TownChain ist Non-EVM → kein Konflikt
- **Gültig seit:** 2026-06-14

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

### AD-005 — ATC-97 Agent Interaction Protocol ⏳
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

### AD-009 — ATCLANG_SPEC.md-Konsolidierung + Bridge-Standards-Dedup ⏳
- **Status:** DECISION — Michael muss Prioritaet festlegen
- **Problem 1:** `ATCLANG_SPEC.md` existiert an 5 verschiedenen Pfaden parallel
  (`atclang/`, `docs/atclang/ATCLANG_SPEC_FULL.md`, `module-docs/atclang/`,
  `docs/wiki/kai-os/code/atclang/`, `aistudio/temp_repo/atclang/`) — Ursache fuer ~270 verbleibende
  kaputte Links
- **Problem 2:** Cross-Chain Bridge hat 4 ueberlappende Standard-Dokumente fuer dasselbe Thema
  (ATC-09, ATC-38, ATC-69, ATC-91) — Dokumentations-Drift im Standards-Register
- **Details:** `docs/REALITY_CHECK_2026-07-06.md` (Nachtrag 1 + 2)
- **Gefunden von:** Aurora (aurora-base44-superagent-6a2756186106d6f0fbb105b5), 06.07.2026

---

*Decisions Register v1.0.0 — Aurora (MasterBrain · Base44) · 05.07.2026*
