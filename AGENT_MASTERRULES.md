# 🧠 A-Town Ecosystem Brain — Masterregeln für KI-Agenten

> **Dokument-Typ:** Verbindliche Betriebsregeln
> **Gilt für:** Alle KI-Agenten die am A-TownChain Ökosystem arbeiten
> **Verwaltet von:** Aurora (MasterBrain) · Base44 Superagent
> **Stand:** 2026-09-14 | **Version:** 2.1
> **Autoritative Basis:** `atc-standards/governance/authority/authority-matrix.yaml` + `docs/DECISIONS_REGISTER.md` + `atc-standards` (kanonische Standards-Heimat)
>
> **Authority-Regel:** Die aktuelle personelle Owner-/Approver-Zuordnung wird ausschließlich aus der Authority-SSOT und dem jeweils gültigen §9-Approval-Record abgeleitet. Historische Approval-Records behalten ihre damalige Identität und dürfen nicht rückwirkend umgeschrieben werden.

---

## REGEL 0 — SPRACH- UND STACK-MODELL

> **ATCLang ist die Contract-Sprache der Chain. Rust ist der kanonische Kern. Python ist Referenz und AI-Layer.**

- **ATCLang** (atclang, L0): Contract-Sprache auf der ATVM (Chain-ID 658467).
- **Rust** = kanonisch für Compiler, ATVM, Bytecode-Verifier, ShivaCore Kernel und Chain-Infrastruktur.
- **Python** = Referenz-Implementierung, SDK/Testing/Fuzzing-Tooling und AI-Layer. Niemals Konsensus-Ausführung.
- **TypeScript** = Explorer/UI-Services.
- Solidity ist kein kanonischer Bestandteil des aktuellen Ökosystems.

---

## REGEL 1 — REALITY-CHECK (Oberste Pflicht)

Ein Agent darf niemals behaupten, eine Aktion ausgeführt zu haben, ohne einen bestätigten API-Response erhalten zu haben.

### Ausführungsstatus

| Symbol | Status | Bedeutung |
|---|---|---|
| ✅ | AUSGEFÜHRT | API-Response mit ID/SHA bestätigt |
| 🔄 | VORBEREITET | Content fertig, Push steht aus |
| 📋 | GEPLANT | Logik steht, kein API-Call |
| 🔲 | SIMULIERT | Nur lokal, kein externer Effekt |
| ❌ | FEHLGESCHLAGEN | API-Fehler erhalten |
| ⚠️ | UNKLAR | Response mehrdeutig |

---

## REGEL 2 — WISSENS-ZUSTANDSMODELL

Jede Information trägt einen expliziten Zustand. Niemals Information ohne Zustand verarbeiten oder weitergeben.

| Zustand | Symbol | Bedeutung |
|---|---|---|
| PROVEN | ✓✓ | Mathematisch/formal verifiziert |
| IMPLEMENTED | ✓ | Code + Tests + Wiki vorhanden |
| DOCUMENTED | 📄 | Nur dokumentiert, Code fehlt |
| DERIVED | ~ | Logisch abgeleitet |
| ASSUMPTION | ? | Annahme, Bestätigung empfohlen |
| TODO | ○ | Erkannte Lücke, muss implementiert werden |
| VALIDATE | ⚠️ | Widerspruch, menschliche Prüfung |
| DECISION | 🔴 | Fachentscheidung offen, Mensch erforderlich |
| CONFLICT | ✗ | Zwei Quellen widersprechen sich |

---

## REGEL 3 — CODE ↔ DOKUMENTATION ABGLEICH

Bei jeder Änderung:

1. Code analysieren.
2. Architektur prüfen.
3. Docs-Hub prüfen.
4. Roadmap prüfen.
5. Standards prüfen.
6. Issues prüfen.

**Keine Änderung darf isoliert existieren. Jede Änderung propagiert auf alle betroffenen Systeme.**

---

## REGEL 4 — WIKI-/DOKU-VALIDIERUNG

### Status-Lifecycle

```
IDEA → PROPOSED → DRAFT → REVIEW → CANDIDATE → APPROVED → STABLE → DEPRECATED → RETIRED
```

Die normative Definition des Lifecycle stammt aus der aktuell gültigen Fassung von ATC-STD-000 im Standards-SSOT. Historische Fassungen sind Audit-Historie, nicht aktueller Normzustand.

---

## REGEL 5 — ISSUE-ERSTELLUNG

Neue Wiki-Kapitel, Komponenten, Klassen und API-Endpunkte benötigen nachvollziehbare Issues mit messbaren Akzeptanzkriterien. Vor Erstellung ist auf Duplikate zu prüfen.

---

## REGEL 6 — GITHUB SYNC-PROTOKOLL

Vor jedem Push:

- Conventional Commit verwenden.
- Tests bzw. geeignete Validierung ausführen.
- Governance-CI muss grün sein.
- Standards- und Repository-IDs gegen das SSOT prüfen.
- Architektur- und Ownership-Zuordnung prüfen.

Nach jedem erfolgreichen Push:

- Commit-SHA dokumentieren.
- Betroffene CHANGELOG-/Status-/Roadmap-Artefakte synchronisieren.
- Generierte Views niemals als unabhängige SSOT behandeln.

---

## REGEL 7 — AUDIT-PROZESS

Für Hauptkomponenten sind Architektur-, Code-, Security-, Dokumentations- und Standard-Audits getrennt zu betrachten. Jeder Fund benötigt konkrete Quelle, Risiko, Empfehlung und Status.

**Ein Audit darf einen technischen P0 nicht allein durch eine Statusänderung schließen.**

---

## REGEL 8 — RELEASE-BLOCKER

Release ist gesperrt bei kritischen Sicherheitslücken, offenen technischen P0/P1-Gates, fehlenden Tests/Evidence, fehlender Conformance, nicht bestandener CI oder relevanten offenen Owner-Entscheidungen.

---

## REGEL 9 — ENTSCHEIDUNGS- UND AUTHORITY-GRENZEN

Diese Themen löst kein Agent automatisch. Die Rollen und personellen Zuständigkeiten werden aus dem Authority-SSOT abgeleitet.

### Aktueller Authority-SSOT

**Normative Quelle:** `atc-standards/governance/authority/authority-matrix.yaml`

**Aktueller Owner-/Approver-Referenzstand:** Alexander Wroblewski ist im aktuellen §9-Approval-Record vom 11.09.2026 als Owner/Entscheider dokumentiert. Diese Zuordnung wird nicht aus älteren Dokumenten abgeleitet.

**Historische Identität:** Michael Wroblewski / ShivaCoreDev bleibt in historischen Approval-, Security-, Copyright- und Audit-Artefakten unverändert erhalten. Das Auftreten dieser Identität in einem historischen Record begründet keine aktuelle Approver-Zuordnung.

**Wichtig:** Owner, Approver, Reviewer, Executor und Auditor sind getrennte Rollen. Agenten sind niemals Approver. Die konkrete Entscheidungsmatrix bleibt `authority-matrix.yaml` vorbehalten.

### Aktuelle Governance-Aussagen

| Thema | Aktueller Zustand | Quelle |
|---|---|---|
| ATC-STD-000 | v1.3.0 APPROVED | Standards Registry SSOT |
| Owner/Approver | aus Authority-SSOT + gültigem §9-Record | `governance/authority/authority-matrix.yaml` |
| Mainnet-Termin | kein genehmigtes Datum | `docs/DECISIONS_REGISTER.md` |
| P0-Readiness | technische Evidence erforderlich | jeweilige P0-Gates |

### Historische Records

Historische Approval-Records werden nicht nachträglich geändert. Wenn ein historischer Record eine frühere Version oder frühere Personenzuordnung enthält, muss diese als historische Information interpretiert werden.

### Grundsatz

```
Standard-Approval / SCR-Decision       → DECISION (Owner)
Kryptografie-/Konsensus-Wechsel       → DECISION + technische Evidence
Chain-ID / Netzwerk-Identität         → DECISION (Chain-ID 658467)
Token-Ökonomie                         → VALIDATE → DECISION
Neue Namespaces / ID-Bereiche          → SCR
Notfall-Sicherheitsfix                 → Emergency-Prozess + nachgeholtes SCR
```

---

## REGEL 10 — SYNC-REIHENFOLGE

Nach jeder Änderung sind Code, Dokumentation, Issues, Roadmap, CHANGELOG und relevante Governance-Views auf Konsistenz zu prüfen. Externe Systeme dürfen keine konkurrierende SSOT erzeugen.

---

## REGEL 11 — AGENTEN-ROLLEN

Agenten dürfen je nach Mandat Wissen erfassen, Architektur prüfen, Standards validieren, Roadmaps pflegen, Code erzeugen, testen, Security prüfen, Dokumentation pflegen oder Repository-Operationen ausführen. **Keine Agentenrolle besitzt Approver-Autorität.**

---

## REGEL 12 — GMAIL POLICY

```
Gmail darf SENDEN.
Gmail darf NICHT lesen, scannen oder auswerten.
```

---

## REGEL 13 — ATCLANG-ENTWICKLUNGSPOLITIK

ATCLang ist die On-Chain-/Contract-Sprache. Rust trägt die Chain-Infrastruktur und die kanonische VM-/Verifier-Ausführung. Die VM ist die definierte Grenze. Python bleibt Referenz und Test-/AI-Layer; Konsensus-Ausführung bleibt ausgeschlossen.

---

## REGEL 14 — KENNZAHLEN UND STATUS

Aktuelle Organisations-, Standard- und Repository-Zahlen dürfen nicht manuell in diesem Dokument gepflegt werden. Sie sind aus den jeweiligen SSOTs bzw. generierten State-Views zu beziehen.

Historische Kennzahlen gehören in `STATUS.md`, `CHANGELOG.md` oder Audit-Artefakte und müssen als historisch erkennbar sein.

---

## REGEL 15 — STANDARDS- UND NAMING-GOVERNANCE

1. Kanonische Standards-Heimat = `atc-standards`.
2. `registry/standards.yaml` ist SSOT für Standards.
3. `registry/repositories.yaml` ist SSOT für Repository-Registry-Daten.
4. Generierte Views sind Projektionen und dürfen keine konkurrierende Wahrheit erzeugen.
5. Änderungen an APPROVED Standards erfolgen über den vorgesehenen Governance-/SCR-Prozess.
6. IDs sind immutable.

---

## REGEL 16 — GENERIERTE VIEWS / SSOT

Registry-Zahlen, Versionen und Statuswerte müssen aus dem Registry-SSOT generiert werden. Generierte Dateien müssen ihre Registry-Version und Registry-SHA tragen. CI muss Abweichungen blockieren.

Eine manuelle Änderung eines generierten State-Blocks ist kein zulässiger Weg, einen Governance- oder Readiness-Zustand zu ändern.

---

## ANHANG A — REPOSITORY-/ARCHITEKTURMAP

Die aktuelle Repository- und Layer-Zuordnung wird aus `atc-standards/registry/repositories.yaml` und den aktuellen Architekturartefakten abgeleitet. Statische historische Karten bleiben Audit-Historie.

## ANHANG B — STANDARDS

Die aktuelle Liste und der aktuelle Status der Standards werden ausschließlich aus `atc-standards/registry/standards.yaml` abgeleitet.

## ANHANG C — VERBUNDENE DIENSTE

GitHub ist die verbindliche Primärquelle für Repository-Zustände. Externe Dienste dürfen nur im Rahmen ihrer expliziten Berechtigungen und Mandate synchronisiert werden.
