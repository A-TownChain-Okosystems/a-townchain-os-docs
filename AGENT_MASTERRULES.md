# 🧠 A-Town Ecosystem Brain — Masterregeln für KI-Agenten

> **Dokument-Typ:** Verbindliche Betriebsregeln
> **Gilt für:** Alle KI-Agenten die am A-TownChain Ökosystem arbeiten
> **Verwaltet von:** Aurora (MasterBrain) · Base44 Superagent
> **Stand:** 2026-09-07 | **Version:** 2.0 (ersetzt v1.0 vom 2026-06-12)
> **Autoritative Basis:** docs/DECISIONS_REGISTER.md (AD-001…AD-039) +
> atc-standards (kanonische Standards-Heimat, AD-030)

---

## REGEL 0 — SPRACH- UND STACK-MODELL (korrigiert v1.0)

> **ATCLang ist die Contract-Sprache der Chain. Rust ist der kanonische
> Kern. Python ist Referenz und AI-Layer. Nichts davon ist ein Stub.**

- **ATCLang** (atclang, L0): Contract-Sprache auf der ATVM (Chain-ID 658467).
  Gates G0-G19, KEIN FREEZE vor G18. Kernsatz: Compiler erzeugt Code —
  Verifier entscheidet Gueltigkeit — ATVM fuehrt ausschliesslich
  Verifiziertes deterministisch aus (AD-021/022).
- **Rust** = kanonisch fuer Compiler, ATVM, Bytecode-Verifier, ShivaCore
  Kernel (AD-012/013), Aurora AI-Core, Genesis Engine. Rust 1.98.1 Stable.
- **Python** = Referenz-Implementierung (Referenz bleibt!), SDK/Testing/
  Fuzzing-Tooling, Aurora AI-Layer (PyTorch/ONNX/LLM). **NIEMALS
  Konsensus-Ausfuehrung** (AD-021/022 — Differential Testing
  Rust-vs-Python ist verbindliches Conformance-Kriterium).
- **TypeScript** = Explorer/UI-Services.
- Solidity existiert NICHT mehr im Oekosystem (Bridge-Contracts in ATCLang).

> Die v1.0-Regel "Alles ist ATCLang, kein Rust" ist mit AD-021/022 (06.09.)
> UEBERHOLT und wird hiermit formell ersetzt.

---

## REGEL 1 — REALITY-CHECK (Oberste Pflicht)

Ein Agent darf **niemals** behaupten, eine Aktion ausgeführt zu haben,
ohne einen bestätigten API-Response erhalten zu haben.

### Ausführungsstatus — immer explizit angeben

| Symbol | Status | Bedeutung |
|--------|--------|-----------|
| ✅ | AUSGEFÜHRT | API-Response mit ID/SHA bestätigt |
| 🔄 | VORBEREITET | Content fertig, Push steht aus |
| 📋 | GEPLANT | Logik steht, kein API-Call |
| 🔲 | SIMULIERT | Nur lokal, kein externer Effekt |
| ❌ | FEHLGESCHLAGEN | API-Fehler erhalten |
| ⚠️ | UNKLAR | Response mehrdeutig |

### Checkliste vor jeder externen Aktion

```
GitHub Push:
  □ Token gültig?          (401 = nein → abbrechen)
  □ Repo existiert?         (404 = nein → abbrechen)
  □ SHA der Datei geladen?  (PUT ohne SHA = neue Datei)
  □ Response enthält commit.sha?

Issue/Ticket erstellen:
  □ API erreichbar?
  □ Kein Duplikat? (Titel prüfen)
  □ Response enthält number/id?

E-Mail senden:
  □ Token gültig?
  □ Response enthält Message-ID?
  □ WICHTIG: Nur senden — niemals lesen!

Notion aktualisieren:
  □ Token gültig?
  □ Datenbank-ID bekannt?
  □ Response enthält page id?
```

---

## REGEL 2 — WISSENS-ZUSTANDSMODELL

Jede Information trägt einen expliziten Zustand.
Niemals Information ohne Zustand verarbeiten oder weitergeben.

| Zustand | Symbol | Bedeutung |
|---------|--------|-----------|
| PROVEN | ✓✓ | Mathematisch/formal verifiziert |
| IMPLEMENTED | ✓ | Code + Tests + Wiki vorhanden |
| DOCUMENTED | 📄 | Nur dokumentiert, Code fehlt |
| DERIVED | ~ | Logisch abgeleitet |
| ASSUMPTION | ? | Annahme, Bestätigung empfohlen |
| TODO | ○ | Erkannte Lücke, muss implementiert werden |
| VALIDATE | ⚠️ | Widerspruch, menschliche Prüfung |
| DECISION | 🔴 | Fachentscheidung offen, Mensch required |
| CONFLICT | ✗ | Zwei Quellen widersprechen sich |

---


## REGEL 3 — CODE ↔ DOKUMENTATION ABGLEICH

Bei **jeder** Änderung — diese 6 Schritte in dieser Reihenfolge:

```
1. Code analysieren       Was existiert tatsächlich?
2. Architektur prüfen     Bauhierarchie L0-L7 korrekt? (AD-026)
3. Docs-Hub prüfen        Dokumentation aktuell? (Doku im Hub, AD-030)
4. Roadmap prüfen         M1-M8-Zuordnung stimmt? (AD-027)
5. Standards prüfen       ATC-/ATS-/ATC-STD-Nummer referenziert?
6. Issues prüfen          Ticket vorhanden?
```

6 Abgleichsfragen — alle müssen NEIN sein vor jedem Release:

```
□ Dokumentation ohne Implementierung?  → DOCUMENTED (Warnung)
□ Implementierung ohne Dokumentation?  → Hub erstellen (Blocker!)
□ Ticket ohne Umsetzung?               → TODO
□ Umsetzung ohne Ticket?               → Issue erstellen (Pflicht!)
□ Roadmap-Eintrag ohne Meilenstein?    → M-Gate ableiten
□ Meilenstein ohne Roadmap-Eintrag?   → Roadmap aktualisieren
```

**Keine Änderung darf isoliert existieren.**
**Jede Änderung propagiert auf alle betroffenen Systeme.**

---

## REGEL 4 — WIKI-/DOKU-VALIDIERUNG

### Status-Lifecycle (ATC-STD-000 §8: IDEA → … → RETIRED, kein Springen)

```
DRAFT → REVIEW → CANDIDATE → APPROVED → STABLE → DEPRECATED → RETIRED
```

### Vor APPROVED: Alle 5 Checks müssen grün sein

```
□ Technisch korrekt?          (Code stimmt mit Doku überein)
□ Konsistent mit Code?        (Klassen, Methoden, Signaturen exakt)
□ Konsistent mit Architektur? (Layer-Zuordnung L0-L7 stimmt)
□ Konsistent mit Standards?   (ATC-/ATS-/ATC-STD-Nummer vorhanden)
□ Konsistent mit Roadmap?     (M1-M8-Zuordnung stimmt)
```

---

## REGEL 5 — ISSUE-ERSTELLUNG (Automatisch)

**Trigger:** Neues Wiki-Kapitel, neue Komponente, neue Klasse, neuer API-Endpunkt

### Pflichtformat

```yaml
title:       "[Kap. XX] Komponente — Kurzbeschreibung"
body: |
  ## Was
  Kurze Beschreibung der Aufgabe

  ## Warum
  Begründung (Architektur / Standard / User Story)

  ## Akzeptanzkriterien
  - [ ] Messbare Bedingung 1
  - [ ] Messbare Bedingung 2
  - [ ] Messbare Bedingung 3

  ## Verknüpfungen
  - Wiki: Kap. XX
  - Standard: ATC-XXXX
  - Layer: LX
  - Sprint: X.X
  - Abhängigkeiten: #XX, #XX

labels:      [priority:HIGH, sprint:2.x, layer:LX]
```

**Nur erstellen wenn:**
- GitHub API erreichbar ✅
- Kein Duplikat (Titel-Check zuerst!) ✅

---


## REGEL 6 — GITHUB SYNC-PROTOKOLL

### Vor jedem Push (Pflicht-Checklist)

```
□ Conventional Commits:  feat|fix|docs|security|chore(...): Beschreibung
□ Tests vorhanden:       je Modul (Rust: cargo test; Python: pytest)
□ Governance-CI:         GATE PASS halten (governance-ci.yml, AD-039)
□ Naming:                IDs nach ATC-STD-000 §7 (min. 3-stellig, immutable)
□ Architektur:           Layer-Zuordnung L0-L7 konsistent (AD-026)
□ Standard:              ATC-/ATS-/ATC-STD-Nummer referenziert
```

### Nach jedem erfolgreichen Push (Pflicht)

```
□ Commit-SHA notieren (Reality-Check!)
□ CHANGELOG.md aktualisieren
□ VERSION erhöhen (bei Release; SemVer)
□ Docs-Hub synchron halten
□ Relevante Sync-Schritte aus REGEL 10 ausführen
```

### Push-Ausgabe immer so formatieren

```
✅ AUSGEFÜHRT: commit abc12345 — Beschreibung
❌ FEHLGESCHLAGEN: {"message": "..."} — Beschreibung
```

---

## REGEL 7 — AUDIT-PROZESS

Für jede Hauptkomponente fünf Audit-Typen:

| Typ | Was wird geprüft |
|-----|-----------------|
| Architektur-Audit | Layer-Zuordnung, Abhängigkeiten, Interfaces |
| Code-Audit | Klassen, Methoden, Signaturen, Logik |
| Sicherheits-Audit | Kryptografie, Input-Validierung, Replay-Schutz |
| Dokumentations-Audit | Wiki-Deckung, Konsistenz, Aktualität |
| Standard-Audit | ATC/KIP/AIP/ATS Compliance |

### Jeder Audit-Fund enthält

```
Problem:    Konkret, mit Datei + Zeile
Risiko:     CRITICAL / HIGH / MEDIUM / LOW
Sprint:     Wann beheben?
Empfehlung: Konkreter Fix-Befehl
Status:     TODO / IN_PROGRESS / DONE
```

### Audit-Score

```
90-100:  ✅ Release erlaubt
80-89:   ⚠️ Release mit Auflagen
< 80:    ❌ Release GESPERRT
```

---

## REGEL 8 — RELEASE-BLOCKER

Release ist **gesperrt** solange einer dieser Punkte gilt:

```
❌ Audit-Score unter 80/100
❌ Kritische Sicherheitslücke offen
❌ Neue Klasse/API ohne Wiki-Kapitel
❌ Neues Modul ohne Tests
❌ Offene CRITICAL oder HIGH Blocker-Issues
❌ Standard im DRAFT-Status (nicht APPROVED)
❌ Offenes DECISION-Item das Release betrifft
❌ CI/CD schlägt fehl
❌ Wiki-Kapitel im DRAFT-Status
```

---


## REGEL 9 — ENTSCHEIDUNGS-GRENZEN

Diese Themen löst kein Agent automatisch. Sie werden als offene
Entscheidung dokumentiert und an den Owner (Michael/ShivaCore) gemeldet.

### Offene Entscheidungen (Stand: 2026-09-07)

| ID | Impact | Thema | Wo dokumentiert |
|----|--------|-------|-----------------|
| ATC-STD-000 Approval | ✅ RESOLVED (07.09.) | Verfassung v1.1.0 APPROVED; v1.2.0 CANDIDATE (§37 ID-Allokation, §38 Security) wartet auf Owner-§9-Freigabe | atc-standards/approval/APPROVAL-DECISION.md + governance/ATC-STD-000.md |
| SCR-0003 Option | 🔴 CRITICAL | Branch-Absicherung: A = Agent auf PR-Flow / B = dokumentierte Ausnahme | atc-standards/change-requests/SCR-0003.md |
| SCR-0001 | ✅ RESOLVED (07.09.) | ID-Allokationsprozess akzeptiert + umgesetzt in ATC-STD-000 v1.2.0 §37 | atc-standards/change-requests/SCR-0001.md |
| SCR-0004 | 🟡 HIGH | Rollen- und Berechtigungsmodell | atc-standards/change-requests/SCR-0004.md |
| Mainnet-Termin | 🟠 MEDIUM | Launch-Datum offen (AD-023) — M8 folgt Roadmap, nicht Datum | DECISIONS_REGISTER AD-023 |

> **Rollenregel (AD-035):** Owner = alleiniger Approver. Agenten sind
> Autor/Reviewer/Executor — **NIE Approver**. SCR-Entscheidungen sind
> Owner-Vorbehalt.

> Die v1.0-Entscheidungen AD-001…AD-007 sind ALLE RESOLVED — verbindliche
> Historie im DECISIONS_REGISTER (u.a. Chain-ID = 658467, AD-004 RESOLVED).

### Grundsatz

```
Standard-Approval / SCR-Decision      → DECISION (Owner)
Kryptografie-Algorithmus-Wechsel      → DECISION (Owner)
Chain-ID / Netzwerk-Identität         → DECISION (Owner) — 658467 fix
Token-Ökonomie Parameter              → VALIDATE → DECISION
Neue Namespaces / ID-Bereiche          → SCR (ATC-STD-000 §7.7)
Notfall-Sicherheitsfix                → §32 Emergency → nachgeholtes SCR
```

---

## REGEL 10 — SYNC-REIHENFOLGE (Nach jeder Änderung)

Immer in dieser Reihenfolge, jeden Schritt mit Status:

```
 1. GitHub Code     → ✅/❌ mit Commit-SHA
 2. GitHub Wiki     → ✅/❌ mit Commit-SHA
 3. EcosystemNode   → Zustand in Base44 aktualisieren
 4. Wiki-Status     → PUBLISHED setzen
 5. Issues          → Verlinkungen prüfen / ergänzen
 6. Roadmap/Sprint  → Status aktualisieren
 7. CHANGELOG.md    → Eintrag hinzufügen
 8. Notion          → Tagesprotokoll aktualisieren
 9. Google Tasks    → Offene Tasks synchronisieren
10. Google Drive    → Report hochladen
11. Gmail           → Status-Report senden (NUR SENDEN, NIEMALS LESEN)
12. Outlook         → Status-Report senden
```

---

## REGEL 11 — 12 AGENTEN-ROLLEN

| # | Agent | Aufgabe | Trigger |
|---|-------|---------|---------|
| 1 | KnowledgeAgent | Alle Dienste scannen, Wissensgraph bauen | Jeder Sync |
| 2 | ArchitectAgent | Layer L0-L12, APIs, Abhängigkeiten | Code-Änderung |
| 3 | StandardsAgent | ATC/ATS/KIP/AIP pflegen | Neue Features |
| 4 | RoadmapAgent | Epics, Milestones, Sprints ableiten | Issue-Änderung |
| 5 | ProductAgent | Features priorisieren, User Stories | Neues Issue |
| 6 | CodingAgent | Code in ATCLang generieren, Bugs fixen | TODO erkannt |
| 7 | QAAgent | Tests in ATCLang generieren, Coverage | Code-Änderung |
| 8 | SecurityAgent | Audits, Kryptografie, Schwachstellen | Jeder Sync |
| 9 | DocumentationAgent | Wiki 62+ Kapitel aktuell halten | Code-Änderung |
| 10 | RepositoryAgent | Issues, Commits, PRs, Releases | Jeder Sync |
| 11 | GovernanceAgent | DAO Proposals, Standards, Richtlinien | Governance-Event |
| 12 | ResearchAgent | Lücken finden, neue Ideen, externe Quellen | Lücken erkannt |

**Orchestriert von: MasterBrain (Aurora · Base44)**

---

## REGEL 12 — GMAIL POLICY

```
✅ Gmail darf SENDEN
❌ Gmail darf NIEMALS lesen / scannen / auswerten
```

Diese Regel gilt absolut und hat keine Ausnahmen.

---


## REGEL 13 — ATCLANG-ENTWICKLUNGSPOLITIK (AD-021/022)

### Prioritäten

```
1. KEINE grossen Rebuilds mehr — nur vertikale Subsystem-Implementierung
2. Gates sequenziell: G1 ✅ (Language Spec) → G2 (Semantics) → G3 (ATC-IR)
   → G5 (Bytecode Spec) → G6 (Independent Verifier) … G19
3. Python-Referenz bleibt erhalten; Rust-Canonical nach Spec-Extraktion
4. Differential Testing Rust-vs-Python = verbindliches Kriterium
   (identische Programme → identische ASTs/IR/Bytecode/State-Transitions)
5. ATCLang-Verträge: ATCA-Artifact, ATVM-Verifikation, License Gate (ATC-LIC)
```

---

## REGEL 14 — ECOSYSTEM-KENNZAHLEN (Stand: 2026-09-07)

| Metrik | Wert |
|--------|------|
| Aktive Repos | 23 (AD-024 + atc-standards, AD-030) |
| Kernel-Tests | 674/674 (394 Kernel + 280 Service, Rust 1.98.1) |
| Monorepo-Workspace-Tests | 731/731 |
| Boot-Chain | L0-L10 grün (M2-Gate erfüllt, AD-028) |
| Chain-ID | 658467 |
| Standards | ATC-01…99 + ATS-1000…1007 (Legacy-Serien, atc-standards) + ATC-STD-000…203 + ATC-STD-300 (DTC) |
| ATC-STD-000 (Verfassung) | v1.1.0 APPROVED (07.09.2026, Release+Tag v1.1.0) |
| Repository-Governance | 22/22 GATE PASS (AD-039), CI je Push/PR |
| Offene GitHub-Issues | #69 (Dependabot 14 Vulns), #70 (Validators), #71 (Genesis Block) |
| Roadmap | M1 (G1 ✅, G2 offen) → M8 (AD-027) |
| Mainnet-Launch | Datum offen (AD-023) |

---

## REGEL 15 — STANDARDS- UND NAMING-GOVERNANCE (NEU, AD-030/034/038/039)

1. **Kanonische Standards-Heimat = atc-standards-Repo** (AD-030).
   Standard-Änderungen NUR dort; Hub docs/standards = Archiv-Snapshot.
2. **ATC-STD-000 §7 Naming ist normativ und CI-durchgesetzt:**
   IDs min. 3-stellig (ATC-STD-1/-01 = ungültig), Status NIEMALS in der ID,
   IDs sind immutable (nie umbenennen/wiederverwenden), neue Repos =
   atc-<domain>-<component>, Dateinamen nach §7.6.
   Regeln kommen NUR aus naming-conventions.schema.json; Validator S-16/S-17
   + CI lehnen ungültige Namen automatisch ab.
3. **SCR-Pflicht:** Änderungen an Standards nur via change-requests/SCR-NNN;
   für CANDIDATE-Standards sind dokumentierte Revisionen zulässig.
4. **Registry-Pflicht:** Kein Eintrag in standards.yaml/repositories.yaml =
   kein Standard/kein offizielles Repo (§25).
5. **Repository-Governance:** Jedes Repo hat governance-ci.yml — GATE PASS
   bei jedem Push halten; Governance-Dateien (.atc/, SECURITY.md, CODEOWNERS,
   REPOSITORY_STANDARD.md) nicht ohne Grund anfassen.
6. **Conventional Commits sind Pflicht** (V-16 heilt nur durch Disziplin).

---

## REGEL 16 — PROMPT ENGINEERING FUER AI-AGENTEN (NEU, ATC-SPEC-001)

Theorie-Referenz: ATC-SPEC-001 (APOS/ACE-Framework, docs/ai/).

```
Deterministische Aufgaben (Tools, Orchestrierung, Konsensus-Nähe)
  → Action/Process-Ebene: präzise Schritte, Nummerierung, Review-Prompt,
    Verifikation (Edge-Cases) vor finaler Antwort. Reproduzierbar!
Kreative Aufgaben (Design, Vision, Content)
  → Schema-Ebene: Multi-Perspektive, Verallgemeinerung erlaubt.
Immer: kognitive Schleife Lernen → Ausführen → Verifizieren → Schema.
```

Aurora AI (L2) = Action/Process (deterministisch, konsistent mit der
Differential-Testing-Pflicht). Genesis AI (L6) = Schema-Ebene.

---

## ANHANG A — REPOSITORIES (23 aktive, AD-026-Hierarchie)

```
L0 atclang → L1 atc-shivacore → L2 aurora-ai → L3 a-townchain
→ L4 globus-os → L5 13 Services → L6 genesis-engine/genesis-chronicles
→ L7 a-townchain-os (Integration)
[parallel] a-townchain-os-docs (Docs-Hub) · atc-standards (Norm)
```

Vollständige Karte: a-townchain-os-docs/docs/REPOSITORY_MAP.md
Registry (maschinenlesbar): atc-standards/registry/repositories.yaml

## ANHANG B — STANDARDS (kanonisch im atc-standards-Repo)

- **ATC-STD-000** Standards Governance & Specification (Verfassung, §7
  Naming) — v1.0.0 CANDIDATE
- **ATC-STD-201/202/203** Repository Structure / Naming & Classification /
  Security & Release — draft/proposed
- **Legacy-Serien:** ATC-01…99 (Ecosystem), ATC-0001…0008 (Core), ATS-1000…1007
  (ShivaOS), ATC-LIC/ATS-LIC (Lizenzmodell)
- Übersicht: atc-standards/registry/STANDARDS_REGISTRY.md + standards.yaml

## ANHANG C — VERBUNDENE DIENSTE

Siehe AGENT_MANIFEST.md (Abschnitt Integrationen). GitHub = verbindliche
Primär-Quelle; externe Dienste (Notion/Google/Microsoft) nur sofern
angebunden. Gmail: nur SENDEN, niemals lesen (REGEL 12).
