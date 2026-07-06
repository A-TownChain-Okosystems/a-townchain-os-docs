# 🔍 REALITY STATUS — Verifizierter Ist-Zustand

> **WICHTIG FÜR ALLE KI-AGENTEN:** Diese Datei ist die einzige Quelle, deren Zahlen
> am 06.07.2026 durch tatsächliche Skript-Ausführung (nicht durch Lesen alter Doku)
> verifiziert wurden. Bei Widersprüchen zu README.md, ROADMAP.md, STATUS.md,
> MILESTONES.md, STANDARDS_REGISTRY.md oder Wiki-Kapiteln gilt **diese Datei**.
> Erstellt/verifiziert von: `aurora-base44-superagent-6a27614c7219ab1e4f951842`
> **Stand:** 06.07.2026, 22:19 UTC+2 — Methode: Parser-Lauf, `pytest`, GitHub-API, `find`/`grep` über beide Repos.

---

## 1. ATCLang — Code-Realität

| Metrik | Wert | Verifikationsmethode |
|---|---|---|
| `.atc`-Dateien gesamt | **176** | `find . -name "*.atc"` |
| Zeilen ATCLang gesamt | **30.953** | `cat *.atc \| wc -l` |
| **Parsen fehlerfrei** | **96 / 176 (54,5%)** | Eigener Parser-Lauf (`atclang/parser`), nicht nur Datei-Existenz |
| Parsen NICHT | **80 / 176 (45,5%)** | s. Abschnitt 2 |
| Solidity-Dateien | **0** | Non-EVM bestätigt |

⚠️ **Frühere Behauptungen "119/119" oder "126/127 parsen" (Sessions vom 05.07.) waren zum
Zeitpunkt der Aussage vermutlich korrekt für den damaligen Dateibestand — seither wurden
~57 neue `.atc`-Dateien in einem anderen Sprach-Dialekt hinzugefügt** (Module `franchise/`,
`meta/`, `civilization/`, `kernel/*_bus_ad*`), die der aktuelle Parser (v0.3) nicht unterstützt.

## 2. ATCLang v1.0-Dialekt-Problem (Sprint 2.1 Blocker, ungelöst)

Die 80 nicht-parsenden Dateien sind **kein Tippfehler-Bug**, sondern ein grundsätzlicher
Sprachversions-Konflikt:

- **Franchise/Meta/Civilization-Module (~57 Dateien):** nutzen `module X.Y[AD-nn] { }`-Wrapper,
  Generics (`Map<K,V>`, `List<T>`, `Option<T>`), Struct-Felder ohne Kommas, verschachtelte
  `import`-Statements innerhalb von Blöcken — nichts davon unterstützt der v0.3-Parser.
- **`modules/assets/*.atc` (16 Dateien, 2.042 Zeilen):** sind de facto **Python-Syntax mit
  `#`-Kommentaren und `enum X:`-Blöcken** — das ist gar kein ATCLang, sondern fälschlich
  als `.atc` benanntes Pseudocode. Muss komplett neu geschrieben werden, kein Parser-Fix möglich.
- **`atcos_main.atc` (1.158 Zeilen):** bereits als "v1.0-Showcase" bekannt (Vererbung, `for-in`,
  Power-Operator) — gleiches Grundproblem.

**Fix in dieser Session:** String-Pfad-Importe (`import "std/x.atc" as Y`) und gepunktete
Importe mit Bracket-Tag (`import GCL.Core[AD-00]`) wurden zum Parser hinzugefügt — das hat
92→96 Dateien gebracht. Der Rest braucht einen echten v1.0-Parser (Generics, Modul-Blöcke,
kommalose Structs) — das ist ein Mehrtage-Engineering-Sprint, kein Ein-Zeilen-Fix.

## 3. Python-Stub-Regression (WICHTIG — widerspricht "Migration Complete")

⚠️ **Mehrere Dateien behaupten "0 Python-Stubs" / "Migration Complete" (Stand 05.07.2026):**
`docs/wiki/chapter-70-atclang-migration-complete.md`, `docs/MIGRATION_MAP.md`,
`docs/standards/STANDARDS_REGISTRY.md` (ATC-99 Zeile), `docs/wiki/kai-os/docs/ROADMAP.md`.

**Das stimmt nicht mehr.** Tatsächlicher Stand heute:
- **72 reale (nicht-leere) Python-Dateien** außerhalb von `tests/` und `atclang/` (Compiler selbst).
- Davon **21 Dateien** wurden am 06.07. von einem anderen Agenten (`...105b5`, Session "K3
  Teilfortschritt") bewusst aus `aistudio/temp_repo/` zurück in den Haupt-Baum kopiert
  (`backend/`, `blockchain/`, `core/`, `gateway/`, `modules/kernel/ai_kernel/`), weil die
  Testsuite Python-Importe erwartet, die es in ATCLang-Form nicht gibt.
- **51 weitere Python-Dateien** liegen in `aistudio/temp_repo/` — ein bisher nicht konsolidiertes
  Parallel-Projekt (K3/K4 Konsolidierungsarbeit läuft, s. `AGENT_COORDINATION.md`).

**Konsequenz:** Die "ATCLang-Migration abgeschlossen"-Aussage ist **stale** und sollte von
keinem Agenten mehr unkritisch übernommen werden, bis K3/K4 wirklich abgeschlossen sind.

## 4. Testsuite (frisch ausgeführt, `pytest -q --continue-on-collection-errors`)

| Ergebnis | Anzahl |
|---|---|
| Gesammelt | 345 (von 349 — 4 Collection-Errors) |
| ✅ Grün | **302** |
| ❌ Rot | **30** |
| ⏭️ Skipped | 13 |
| 🚫 Collection-Error | 4 (`test_bootstrap.py`, `test_did.py`, `test_orchestrator.py`, `test_kai_integration.py`) |

Ursachen der 4 Collection-Errors: fehlende Module (`blockchain.nodes.bootstrap`,
`blockchain.wallet.did`), API-Mismatch (`AIRequest` fehlt in `ai_kernel.py`), zirkulärer
Import in `backend/api/routes/__init__.py`. **0 echte ATCLang-Tests** — Testsuite ist
komplett Python-basiert, obwohl Produktcode laut Mandat ATCLang sein soll.

## 5. GitHub Issues (live via API geprüft)

| Repo | Offen | Geschlossen | Gesamt | Quote |
|---|---|---|---|---|
| a-townchain-os | 11 | 79 | 90 | 87,8% geschlossen |
| a-townchain-os-docs | 0 | 0 | 0 | — |

⚠️ Frühere Zahl "78/82 (95,1%)" ist veraltet — seit K1-K8-Konsolidierungs-Issues
(#85–92) geöffnet wurden, hat sich Nenner und Zähler verschoben.
Unverändert offen: **44 von 79 geschlossenen Issues (56%) referenzieren nicht-existente
Dateien** (s. `docs/REALITY_CHECK_2026-07-06.md`) — Re-Open-Entscheidung liegt weiter bei Michael.

## 6. Wiki-Kapitel-Zahl ist NICHT verifizierbar — Metrik einstellen

README.md und ECOSYSTEM.md behaupten **"75 Kapitel"**. Tatsächlich:

- Nur **9 Dateien** folgen dem Muster `chapter-N-*.md` (Kapitel 63, 70–77).
- Die restlichen **134 Markdown-Dateien** unter `docs/wiki/` sind thematisch in Unterordnern
  organisiert (`kai-os/`, `standards/`, `overview/`, `contracts/`, …) — **ohne erkennbare
  1:1-Zuordnung zu einer Kapitelnummer 1–69**.
- `docs/wiki/kai-os/` ist zudem eine **komplette verschachtelte Kopie einer Repo-Struktur**
  (`code/backend/`, `code/blockchain/`, `docs/standards/`, …, 58 Dateien) — vermutlich ein
  alter, nie aufgeräumter Sync-Schnappschuss, kein echtes "Kapitel".

**Empfehlung an Michael:** Die "Wiki hat X Kapitel"-Kennzahl ist nicht mehr seriös messbar,
solange Kapitel nicht 1:1 als `chapter-N-*.md` vorliegen. Entweder alle Themen-Dateien
formal nummerieren, oder die Kennzahl aus Status-Reports streichen.

## 7. Standards-Registry — Duplikate & Bruch der Namenskonvention

- **101 Dateien** unter `docs/standards/` matchen `ATC-*.md` (nicht 98 oder 99 wie behauptet).
- **`ATC-0009-BRIDGE.md` existiert doppelt** (`docs/standards/ATC/` UND `module-docs/standards/`)
  — altes Nummernformat (4-stellig mit führender Null), das laut Session vom 05.07. bereits
  vollständig auf `ATC-01`–`ATC-99` migriert sein sollte.
- **`ATC-LIC-SMART_CONTRACT_LICENSE.md`** und **`ATS-LIC-SYSTEM_HARDWARE_LICENSE.md`** brechen
  die "nur ATC-01 bis ATC-99, keine anderen Präfixe"-Regel (ATS war laut Regelwerk bereits
  eliminiert). Diese Dateien referenzieren zudem ein **BaFin-Compliance-Handbuch**, dessen
  Existenz/Richtigkeit von einem anderen Agenten bereits als "unverifiziert" markiert wurde
  (s. `AGENT_COORDINATION.md`, Fund zu Agent `69c1e0c...a480`).

## 8. In dieser Session behoben ✅

| Fix | Datei(en) | Commit |
|---|---|---|
| ~~Chain-ID 9001→9000 vereinheitlicht~~ **ZURUECKGENOMMEN** | War falsch — s. Abschnitt 10 | `17a4096` (ueberholt) |
| Parser: String-Pfad-Importe unterstützt | `atclang/parser/parser.py` | `17a4096` |
| Dependency-Sicherheitsupdates (cryptography, requests, python-dotenv, pytest, flask, flask-cors) | `requirements.txt`, `backend/requirements.txt`, `requirements-kai.txt`, `aistudio/temp_repo/gateway/requirements.txt` | `17a4096` |
| npm audit fix (non-breaking) — 11→10 verbleibende Alerts | `aistudio/package-lock.json` | `17a4096` |

## 9. Offen — braucht Michaels Entscheidung (REGEL 9)

1. **Parser v1.0-Upgrade** für 80 Dateien (Generics, Modul-Blöcke) — eigener Sprint, kein Quick-Fix.
2. **`modules/assets/*.atc`** (16 Dateien) sind kein ATCLang — Neuschreiben oder löschen?
3. **44 Issues mit gebrochener Datei-Referenz** — re-open oder als historisch akzeptieren?
4. **K3/K4-Konsolidierung** (`aistudio/temp_repo/` → Haupt-Baum) — wann/wie abschließen, um die
   Python-Stub-Regression zu beenden?
5. **npm `uuid`-Vulnerability** — Fix erfordert Breaking-Change-Upgrade von `firebase-admin`.
6. **Wiki-Kapitel-Zählweise** — neu definieren oder Kennzahl aufgeben (s. Abschnitt 6).
7. **ATC-LIC/ATS-LIC/BaFin-Compliance-Doku** — Status prüfen, ggf. als DRAFT/unverifiziert kennzeichnen.

---
*Nächster Agent: Vor jeder "X ist fertig/behoben/abgeschlossen"-Aussage — dieses Dokument
aktualisieren, nicht nur eine neue Behauptung obendrauf schreiben.*


## 10. ⚠️ NACHTRAG (06.07.2026, 22:25) — AD-004 Chain-ID REOPENED, keine Chain-ID final

Michael hat direkt widersprochen: **"Wir haben noch keine Chain-ID, 9000 ist ID von Ethereum"**
(gemeint: Ethereum-Oekosystem/EVM-Registry). Verifiziert via chainlist.org: **Chain-ID 9000
ist auf Evmos Testnet registriert.**

Die vorherige "AD-004 RESOLVED"-Markierung (Begruendung: "Non-EVM macht Kollision irrelevant")
wurde von Michael **nicht akzeptiert und ist damit ungueltig**, auch wenn ein frueherer Agent
sie in `DECISIONS_REGISTER.md`/`AgentDecision`-Entity als "RESOLVED, resolved_by: Michael +
Aurora" eingetragen hatte — dieser Eintrag war offenbar falsch attribuiert oder ueberholt.

**Korrigiert in dieser Session:** `AgentDecision`-Entity (Base44) auf State `DECISION` (offen)
zurueckgesetzt, `docs/DECISIONS_REGISTER.md`, `docs/AGENT_POLICY.md`, `docs/ROADMAP.md`,
`docs/standards/STANDARDS_REGISTRY.md`, `docs/standards/OVERVIEW.md`,
`docs/roadmap/ROADMAP_EXTENDED.md` korrigiert: AD-004 = 🔴 OPEN/REOPENED, 9000 = nur Platzhalter.

**NICHT gemacht:** Die ~100+ restlichen Vorkommen von "Chain-ID 9000" in Code-Kommentaren,
Wiki-Seiten, Whitepaper, Issues wurden **nicht** massenhaft auf eine neue Zahl umgeschrieben —
das waere derselbe Fehler nochmal (Chain-ID automatisch entscheiden, REGEL 9 verbietet das
explizit). Diese Vorkommen sind ab sofort als **Platzhalter, nicht final** zu lesen, bis
Michael eine echte Chain-ID waehlt oder das Non-EVM-Argument erneut bestaetigt.

**Naechster Schritt:** Michael entscheidet zwischen (a) einer verifiziert freien neuen
Chain-ID, (b) Beibehaltung von 9000 als rein interne, nicht-oeffentlich-registrierte
Non-EVM-ID (dann muss die Begruendung explizit erneut bestaetigt werden), oder (c) einer
anderen Loesung. Erst danach macht ein Mass-Replace Sinn.

