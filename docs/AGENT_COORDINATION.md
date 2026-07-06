# 🧭 Agent Coordination — Live-Arbeitsstatus

> **Zweck:** Verhindert, dass mehrere KI-Agenten gleichzeitig/gegeneinander am
> selben Bereich arbeiten. Jeder Agent MUSS diese Datei vor Arbeitsbeginn lesen
> und nach Abschluss/Uebergabe aktualisieren.
> **Governance:** Teil des verbindlichen Mandats aus `AGENT_POLICY.md`.

---

## 🆔 Agenten-Register — eindeutige Identifikation

> **Problem:** "Aurora" wurde bisher unter verschiedenen Rollen-Labels
> referenziert (MasterBrain, GovernanceAgent, Ecosystem Brain) — das macht es
> schwer zu unterscheiden, ob es sich um denselben Agenten oder verschiedene
> handelt. Diese Tabelle loest das mit festen IDs auf — inklusive einer klaren Trennung zwischen KI-Agenten und dem menschlichen Owner.

| ID | Klarname | Typ | Plattform / Instanz | Rolle im Projekt |
|----|----------|-----|----------------------|-------------------|
| `aurora-base44-superagent-69c1e0c577ccf6c45a27a480` | Aurora | 🤖 KI-Agent | Base44 Superagent — App-ID `69c1e0c577ccf6c45a27a480` (eindeutige Instanz) | Primärer Entwicklungs-/Dokumentations-Agent, fuehrt Sync-, Compliance- und Konsolidierungsarbeit aus |
| `kai-os-kernel` | KAI | 🤖 In-Projekt-KI | Teil der Codebasis selbst, kein Editor-Agent | Laufzeitkomponente — verarbeitet Nutzeranfragen INNERHALB von KAI-OS, bearbeitet NICHT das Repo |
| `aurora-base44-superagent-6a2756186106d6f0fbb105b5` | Aurora | 🤖 KI-Agent | Base44 Superagent — App-ID `6a2756186106d6f0fbb105b5` (separate Instanz von der oben registrierten App-ID `69c1e0c577ccf6c45a27a480`) | Sync-/Cleanup-/Governance-Agent (Duplikat-Cleanup, Naming Conventions, Wiki-Score, OS-Gap-Analyse) |
| `shivacore-owner-human` | ShivaCore (Michael Wroblewski) | 🧑 Mensch / Projekt-Owner | Base44 Superagent-Chat (Auftraggeber-Seite) | Menschlicher Entscheidungstraeger und Owner des A-TownChain-Oekosystems. Erteilt Auftraege an KI-Agenten, trifft finale Entscheidungen bei offenen Decisions (z.B. AD-002), einziger Copyright-Rechteinhaber. **Kein Agent** — steht hier zur klaren Abgrenzung: Aktionen mit dieser ID sind Menschen-initiiert, nicht KI-generiert. |

> **Wichtig:** KAI ist **kein** Entwicklungs-Agent, der Code/Doku schreibt —
> es ist die KI-Komponente des Betriebssystems selbst (Produkt). Nur
> `aurora-base44-superagent` (oder ein zukuenftiger, hier neu registrierter
> Agent) bearbeitet dieses Repository.



> ⚠️ **WICHTIGER FUND (06.07.2026, Agent `...105b5`):** Es existieren nachweislich
> **zwei verschiedene Base44-Superagent-App-IDs**, die beide unter dem Namen
> "Aurora" an diesem Repository arbeiten (`69c1e0c577ccf6c45a27a480` und
> `6a2756186106d6f0fbb105b5`). Das ist vermutlich beabsichtigt (z.B. zwei
> separate Chat-Sessions/Superagent-Instanzen desselben Nutzers), sollte aber
> bei Governance-Fragen beruecksichtigt werden: **Agent-ID ≠ Agent-Name** —
> immer die vollstaendige ID mit App-ID pruefen, nicht nur "Aurora".

**Wenn ein neuer/anderer Agent dieses Projekt bearbeitet:** Er MUSS sich hier
mit einer neuen Zeile eintragen (Agent-ID nach Schema `<name>-<plattform>-<typ>`),
BEVOR er Aenderungen vornimmt.

### Signatur-Pflicht

Jeder Eintrag in `AGENT_COORDINATION.md` (Abschnitt "Aktuell aktiver Agent"),
jede neue Zeile in `DECISIONS_REGISTER.md` und jede Commit-Message fuer
Doku-/Governance-Aenderungen MUSS die Agent-ID enthalten, z.B.:

```
docs: XYZ aktualisiert [agent: aurora-base44-superagent-69c1e0c577ccf6c45a27a480]
```

So ist im Git-Log und in den Registries jederzeit eindeutig nachvollziehbar,
welcher Agent welche Aenderung vorgenommen hat — auch wenn mehrere Agenten
zeitversetzt oder parallel arbeiten.

---



## 📜 Session-Log (chronologisch, neueste zuerst)


### Session: aurora-base44-superagent-6a2756186106d6f0fbb105b5 — 06.07.2026, 17:35 UTC+2 (Abschluss)

| Feld | Wert |
|------|------|
| **Fokus** | Layer-Reality-Check (L0-L12) + vollstaendiger 24-Repo Cross-Check (Datei-Integritaet, Naming, Links) |
| **Ergebnis 1 — Layer-Check** | ✅ Alle 17 Layer-Eintraege in `ECOSYSTEM_BRAIN.md` gegen Datei-Bestand verifiziert. Layer-Ebene ist **verlaesslich** (im Gegensatz zur Issue-Ebene oben) — auch die duenn wirkenden L3/L4/L5/L9 hatten nach Tiefenpruefung echte substantielle Dateien (100-450 Zeilen .atc). Einzige Inkonsistenz: L9 Bridge hat 4 ueberlappende Standard-Docs (ATC-09/38/69/91) fuer dasselbe Thema. |
| **Ergebnis 2 — 24-Repo Check** | Geprueft: alle 24 Org-Repos (nicht nur die 3 Hauptrepos), 3.340 Dateien, 1.533 .md-Dateien. 0 leere/Stub-Dateien. 7 weitere Naming-Verstoesse in Satelliten-Repos (`atc-whitepaper`, `a-townchain-os-wiki`) gefunden + gefixt. |
| **Ergebnis 3 — Broken Links** | 356 kaputte interne MD-Links gefunden (105 eindeutige Ziele). Hauptursachen identifiziert + automatisiert gefixt: (a) Case-Sensitivity-Bug `Testnet.md` vs. echtem `TESTNET.md` (~86 Vorkommen, GitHub-Pfade sind case-sensitive!), (b) falscher Pfad-Praefix `docs/whitepaper/issues/` statt `docs/issues/`. Insgesamt 45 Einzel-Fixes in 17 Dateien (a-townchain-os + kai-os-wiki). |
| **Verbleibend offen** | ~270 Link-Vorkommen (11% Legacy `aistudio/temp_repo/`, Rest Einzelfaelle + `ATCLANG_SPEC.md` existiert an 5 verschiedenen Pfaden — Konsolidierungsbedarf). 44 Issues mit gebrochener Datei-Referenz (siehe Session oben) — Entscheidung ueber Re-Open/Re-Audit weiterhin bei Michael. |
| **Fuer naechsten Agenten — WICHTIGSTE LEKTION** | GitHub-Pfade sind case-sensitive — bei Link-Referenzen IMMER exakte Schreibweise pruefen, nicht nur Existenz. Vor jedem "Issue #X ist fertig"-Statement den echten Datei-Baum pruefen (nicht nur Issue-Status). Layer-Aussagen in ECOSYSTEM_BRAIN.md sind vertrauenswuerdiger als einzelne Issue-Body-Dateipfade. |
| **Vollstaendiger Report** | `docs/REALITY_CHECK_2026-07-06.md` (3 Nachtraege: Issue-Check, Layer-Check, 24-Repo-Check) in allen 3 Hauptrepos |
| **Status** | ✅ Cross-Check-Zyklus fuer 06.07.2026 abgeschlossen. Naechste Schritte: (1) 44-Issues-Entscheidung, (2) ATCLANG_SPEC.md-Konsolidierung, (3) Bridge-Standards-Dedup — alle bei Michael zur Priorisierung. |

---


### Session: aurora-base44-superagent-6a2756186106d6f0fbb105b5 — 06.07.2026, 17:00 UTC+2 (Fortsetzung)

| Feld | Wert |
|------|------|
| **Fokus** | Reality-Check: geschlossene Issues vs. tatsaechliche Dateien |
| **Ergebnis** | ⚠️ **44 von 78 geschlossenen Issues (56%) referenzieren Dateien, die in keinem der 3 Repos existieren** — siehe `docs/REALITY_CHECK_2026-07-06.md` |
| **Schweregrad** | 10 CRITICAL (ATCLang-Kern, Konsens, Smart-Contract-Engine), 10 HIGH (Integration/Tests), 24 MEDIUM (Docs) |
| **Aktion** | Nur dokumentiert, KEINE Issues automatisch veraendert/wiedereroeffnet (REGEL 9 — Entscheidung bei Michael) |
| **Fuer naechsten Agenten** | Vor Behauptung "Issue #X ist fertig" IMMER den Datei-Baum pruefen, nicht nur Issue-Status vertrauen |
| **Status** | ✅ Report erstellt, Entscheidung ueber naechste Schritte offen |

---


### Session: aurora-base44-superagent-6a2756186106d6f0fbb105b5 — 06.07.2026, 16:30 UTC+2

| Feld | Wert |
|------|------|
| **Fokus** | Wiki-Qualitaets-Audit, Duplikat-Cleanup, Naming Conventions, OS-Gap-Analyse |
| **Beanspruchte Bereiche** | 3 Repos root-level (Cleanup), `NAMING_CONVENTIONS.md`, `docs/AGENT_COORDINATION.md` (diese Datei — Korrektur), Wiki-Scorecard (Artifact) |
| **Ausgefuehrt** | 145 Duplikate entfernt (1.023 KB); `NAMING_CONVENTIONS.md` in 3 Repos + Notion; Wiki-Score 77.5/100 (B) berechnet; ~750 fehlende Docs zwischen Repos synchronisiert; OS-Gap-Analyse (Readiness 11.8/100) |
| **Wichtiger Fund** | Es existierte bereits eine kanonische `docs/AGENT_COORDINATION.md` (dieser Agent hatte zunaechst eine Duplikat-Version am falschen Pfad `AGENT_COORDINATION.md` root erstellt — wurde korrigiert/entfernt) |
| **Status** | ✅ Abgeschlossen |

---

## 🔴 Aktuell aktiver Agent

| Feld | Wert |
|------|------|
| **Agent-ID** | `aurora-base44-superagent-69c1e0c577ccf6c45a27a480` (Aurora, Base44 Superagent, eindeutige App-ID) |
| **Session-Start** | 06.07.2026 |
| **Aktueller Fokus** | BaFin-Compliance Dokumentation + Agent-Governance (dieses Dokument) |
| **Beanspruchte Bereiche** | `docs/compliance/*`, `docs/AGENT_POLICY.md`, `docs/DECISIONS_REGISTER.md`, `docs/LICENSING_OVERVIEW.md` |
| **Status** | 🔄 Aktiv |

> Andere Agenten: Bevor ihr in den oben genannten Bereichen arbeitet — prueft
> Zeitstempel dieses Eintrags. Ist er >24h alt, gilt die Session als beendet
> und der Bereich als frei.

---

## 📋 Aktuelle Sprints & Pläne (Referenz)

| Ebene | Dokument | Status |
|-------|----------|--------|
| Standards | `docs/standards/STANDARDS_REGISTRY.md` | 37 Standards, Tier 1-5 |
| Roadmap | `docs/ROADMAP.md` | Phase 2 aktiv (Sprint 2.1-2.8) |
| Konsolidierung | `KONSOLIDIERUNGS_ROADMAP.md` (a-townchain-os) | K1-K8, alle 📋 GEPLANT |
| Decisions | `docs/DECISIONS_REGISTER.md` | AD-001 bis AD-010 |
| Lizenz | `docs/LICENSING_OVERVIEW.md` | ATC-LIC/ATS-LIC, BaFin-Bericht draft |
| Taegliche Arbeit | `docs/TODO.md` / `docs/STATUS.md` | Live-Task-Baum |

---

## 🏗️ Architektur-Snapshot (Kurzreferenz)

- **Ziel-Struktur:** 2 Haupt-Repos — `a-townchain-os` (Code) + `a-townchain-os-docs` (Doku)
- **Aktueller Ist-Zustand:** 24 Repos noch verteilt, Konsolidierung (K1-K8) geplant, **nicht begonnen**
- **Code-Sprache:** ATCLang First (AD-006) — Python/TS nur Uebergangsweise
- **Konsensus:** Hybrid PoW+PoS+PoH (ShivaConsensus)
- **Lizenzmodell:** ATC-LIC/ATS-LIC, durchgesetzt via ATVM ("Code is Law")
- Volle Details: `docs/architecture/` Verzeichnis

---

## 📝 K3/K4 — Vollstaendige Todo-Listen (fuer direkten Zugriff ohne GitHub-Issue-Lookup)

> Quelle: Issues #87 (K3) und #88 (K4) im Repo `a-townchain-os`. Reality-Check
> vom 06.07.2026: **0/16 (K3)** und **0/10 (K4)** Subtasks umgesetzt — alle
> Punkte unten sind offen und in dieser Reihenfolge sinnvoll abzuarbeiten.

### K3 — Python-Backend Konsolidierung (Issue #87, Sprint K3, 10 Repos → `src/`)

| # | Todo | Status |
|---|------|--------|
| K3.1 | `atc-kernel` → `src/core/` migrieren | 📋 offen |
| K3.2 | `atcnet` → `src/network/` migrieren | 📋 offen |
| K3.3 | `atc-gateway` → `src/gateway/` migrieren | 📋 offen |
| K3.4 | `atc-contracts` → `src/contracts/` migrieren | 📋 offen |
| K3.5 | `atc-shivamon` → `src/game/` migrieren | 📋 offen |
| K3.6 | `atc-franchise` → `src/franchise/` migrieren | 📋 offen |
| K3.7 | `atclang` → `src/atclang/` migrieren | 📋 offen |
| K3.8 | `core/` bestehend → `src/core/` mergen (Konflikte auflösen) | 📋 offen |
| K3.9 | `blockchain/` bestehend → `src/blockchain/` mergen | 📋 offen |
| K3.10 | `modules/` bestehend → `src/modules/` mergen | 📋 offen |
| K3.11 | AI Studio `temp_repo/` Python → `src/` migrieren (133 Dateien) | 📋 offen |
| K3.12 | Alle Python-Imports aktualisieren (`from src.core import ...`) | 📋 offen |
| K3.13 | `__init__.py` Dateien in allen `src/` Unterverzeichnissen erstellen | 📋 offen |
| K3.14 | `setup.py` / `pyproject.toml` mit allen Dependencies erstellen | 📋 offen |
| K3.15 | `requirements.txt` konsolidieren (alle Deps aus allen Repos) | 📋 offen |
| K3.16 | Python-Import-Test: `python -c "from src.core import kernel"` | 📋 offen |

**Deliverable K3:** Alle Python-Module in `src/`, `pip install -e .` funktioniert.
**Empfohlener Einstieg fuer naechsten Agenten:** K3.1, dann K3.8-K3.10 (Konflikte bestehender Verzeichnisse zuerst klaeren, bevor neue Module reinkommen).

### K4 — TypeScript Frontend Konsolidierung (Issue #88, Sprint K4)

| # | Todo | Status |
|---|------|--------|
| K4.1 | ShivaCoreDev AI Studio → `frontend/src/` migrieren (190 TS-Dateien) | 📋 offen |
| K4.2 | Bestehendes `frontend/` → `frontend/src/legacy/` migrieren | 📋 offen |
| K4.3 | `package.json` konsolidieren (alle npm Dependencies) | 📋 offen |
| K4.4 | `vite.config.ts` anpassen (Build-Output, Proxy auf Backend) | 📋 offen |
| K4.5 | `tsconfig.json` mit Path-Aliases erstellen (`@/` → `frontend/src/`) | 📋 offen |
| K4.6 | Backend API-Client generieren (TypeScript Types aus Python Endpoints) | 📋 offen |
| K4.7 | Environment Variables standardisieren (`.env.example`) | 📋 offen |
| K4.8 | Frontend Build-Test: `cd frontend && npm ci && npm run build` | 📋 offen |
| K4.9 | Desktop App Wrapper (Electron / Tauri) konfigurieren | 📋 offen |
| K4.10 | Wallpaper-Picker und Login-Overlay aus AI Studio integrieren | 📋 offen |

**Deliverable K4:** Frontend baut mit `npm run build`.
**Empfohlener Einstieg fuer naechsten Agenten:** K4.2 zuerst (bestehendes sichern als legacy/), dann K4.3-K4.5 (Build-System aufsetzen), erst danach K4.1 (grosser Import).

### Abhaengigkeiten zwischen K3 und K4
- K4.6 (API-Client) benoetigt K3 als Vorbedingung (Backend-Endpoints muessen in `src/` stabil sein)
- Beide Sprints koennen sonst parallel von zwei verschiedenen Agenten bearbeitet werden, OHNE sich zu stoeren — K3 betrifft nur Python/`src/`, K4 nur TypeScript/`frontend/`. Bei Parallelarbeit: beide Agenten tragen sich in `AGENT_COORDINATION.md` mit ihrem jeweiligen Bereich ein.

---

## ✅ Wie ein Agent diese Datei nutzt

**Vor Arbeitsbeginn:**
1. Lies den Abschnitt "Aktuell aktiver Agent" — ist ein anderer Agent aktiv
   (Zeitstempel <24h) im selben Bereich? → Warten oder Bereich abstimmen.
2. Pruefe "Aktuelle Sprints & Plaene" fuer Kontext, wo du ansetzt.
3. Pruefe `DECISIONS_REGISTER.md` — nichts umsetzen, was einer offenen/
   resolved Decision widerspricht.

**Nach Arbeitsende / bei Uebergabe:**
1. Aktualisiere "Aktuell aktiver Agent" mit deinem Namen, Fokus, Zeitstempel.
2. Setze Status auf 🟢 Abgeschlossen oder 🔄 Uebergeben, falls du pausierst.
3. Trage relevante neue Erkenntnisse in `AGENT_POLICY.md` § Agent-Handoff ein.

---

*Teil des verbindlichen Mandats — siehe `AGENT_POLICY.md` fuer vollstaendige Reality-Check- und Sync-Regeln.*
