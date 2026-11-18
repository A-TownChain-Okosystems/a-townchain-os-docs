---
title: "Organisationsweiter technischer Reifegradbericht — 2026-11-18"
summary: "Bewertung aller 33 Repositories anhand von IMPLEMENTED / TESTED / CI_VERIFIED / E2E_VERIFIED / VERIFIED auf Exact-HEAD-Basis. Ergebnis: 0 VERIFIED, 0 E2E_VERIFIED, 16 CI_VERIFIED, 9 Repos CI-rot am HEAD."
date: 2026-11-18
scr: SCR-0144
---

# Technischer Reifegradbericht — A-TownChain-Okosystems

Stand: 18.11.2026. Alle Befunde sind an den exakten HEAD-SHA des jeweiligen
Standard-Branches gebunden (ls-remote + check-runs-API je SHA). Quellen:
GitHub API (Commits, Check-Runs, Workflow-Runs, Dependabot), git ls-tree,
Registry (`registry/standards.yaml`, `registry/standard-implementation.yaml`).

## 1. Definitionen (verbindlich für diesen Bericht)

| Stufe | Kriterium |
|---|---|
| **IMPLEMENTED** | Echter Implementierungscode vorhanden (nicht nur Specs/Stubs), Build-Manifest existiert, Build-Job bzw. lokaler Build-Nachweis grün |
| **TESTED** | Testsuite im Repository, Tests laufen grün (CI-Testjob am HEAD oder dokumentierter lokaler Lauf mit Testzahlen) |
| **CI_VERIFIED** | **Alle** Check-Runs am exakten HEAD-SHA grün (SHA-Bindung geprüft); Gate-Tiefe wird ausgewiesen |
| **E2E_VERIFIED** | End-to-End-/Integrationspfad über mindestens zwei Komponenten existiert und läuft grün, am HEAD oder als dokumentierter, reproduzierbarer Lauf |
| **VERIFIED** | Vollständige Evidenzkette: HEAD-gebundene Evidenz-Dokumentation + CI_VERIFIED + E2E_VERIFIED + Reproduzierbarkeit, keine offenen P0-Blocker |

Symbole: ✓ erfüllt · ~ teilweise/Beleg vorhanden · ✗ nicht erfüllt · n/a nicht anwendbar · ARCHIVED kein Reifeziel

## 2. Korrekturen gegenüber dem Wochenbericht

**Aktivitätsbegriffe (präzisiert):**
- *repo-aktiv* (nicht archiviert): 26
- *wochen-aktiv* (mind. 1 Commit oder PR im Fenster): 15
  - davon commit-aktiv: 13 · PR-only: 2 (atc-node, atc-sdk)
- *inaktiv* (keine Commit-/PR-Aktivität): 11
- archiviert: 7

**Commit-Zahlen (paginiert, exakt, HEAD-gebunden):** 1.796 Commits.
Aufschlüsselung nach Klassifikation der Commit-Subjects:

| Kategorie | Commits | Anteil |
|---|---:|---:|
| Dev (feat/fix/test/refactor/perf/security) | 427 | 24 % |
| Sync/Import/Konsolidierung/Register | 370 | 21 % |
| Dokumentation | 618 | 34 % |
| Infrastruktur (build/ci/style/chore) | 329 | 18 % |
| Dependabot | 4 | <1 % |
| Merge-Commits | 8 | <1 % |
| Sonstiges | 40 | 2 % |

Damit sind nur **24 % der Commits eigenständige Entwicklungsleistungen**;
reine Register-/Sync-Arbeit ist mit 21 % fast gleich groß.

## 3. Reifegrad-Matrix (alle 33 Repositories)

### Aktive Repositories (26)

| Repository | HEAD | IMPLEMENTED | TESTED | CI_VERIFIED | E2E_VERIFIED | VERIFIED | Evidenz / Anmerkung |
|---|---|:-:|:-:|:-:|:-:|:-:|---|
| atc-engineering | 0f94b4f7cc7f | ✓ | ✓ (11 Testdateien) | ✓ **100/100 Checks** | ✗ | ✗ | Reifstes Repo der Org; Gate-Tiefe sehr hoch |
| atc-zkp | a284eeab3dfe | ✓ (R1-Skelett) | ✓ (7) | ✓ 45/45 | ✗ | ✗ | Krypto-Code vorhanden, Proof-Systeme noch nicht produktiv |
| atc-sdk | 11e18587a4c2 | ✓ | ~ (1) | ✓ 37/37 | ✗ | ✗ | Signing-Vector-CI vorhanden |
| atc-vm | 21132a49355e | n/a (Governance-Rolle) | n/a | ✓ 37/37 | ✗ | ✗ | Bewusst kein Code; Spec-/Compatibility-Rolle bestätigt |
| atc-standards | 5ef28537b8d0 | ✓ | ✓ (7) | ✗ regenerate | ✗ | ✗ | Registry 534 Standards, 100 % Matrix-Coverage |
| aurora-ai | 2855956b32fd | ✓ | ✓ (12; 69 Modelltests dokumentiert) | ✓ 6/6 | ✗ | ✗ | Runtime-Integration (GlobusOS) nicht nachgewiesen |
| globus-os | a38738dfcd91 | ✓ (770 Commits/Woche) | ✗ (0 Testdateien) | ✗ cargo-tests | ✗ | ✗ | Syscall-ABI spezifiziert, Implementierung nicht verifiziert |
| a-townchain (SSOT) | 5a9add0e2414 | ✓ | ✓ (662 lokal, 11.11.) | ✗ **6/15 rot** | ~ (46 atc-system-Tests inkl. 11 E2E lokal) | ✗ | SSOT-HEAD rot: markdownlint, ruff, npm-tests, pytest, quality, determinism |
| a-townchain-ecosystem | e18bdb90dbdd | ✓ (Umbrella) | ~ | ✗ 2/4 (index) | ✗ (5 E2E-Workflows vorhanden) | ✗ | 176 Sync-Commits/Woche; Knowledge-Indexer rot |
| a-townchain-os | 8dbbb5ddfad1 | ✓ | ✓ (15) | ✓ 1/1 | ~ (1 Workflow) | ✗ | |
| a-townchain-os-docs | 2a7b159f2e54 | n/a (Doku) | n/a | ✓ 10/10 | n/a | ✗ | Doku-Repo; CI grün |
| atc-contracts | 876c03b2c8ec | ✓ | ✓ (37) | ✗ markdownlint | ✗ | ✗ | Code/Tests gut, Markdown-Gate rot |
| atc-algorithm | f25b43383433 | ✓ | ✗ (0 Tests) | ✓ 24/24 | ✗ | ✗ | Konsens-Kern ohne Testsuite |
| atc-compute | 89d833edf338 | ✓ | ✗ (0) | ✓ 16/16 | ✗ | ✗ | |
| atc-shivacore | d817de2deccb | ✓ | ~ (1) | ✓ 6/6 | ✗ | ✗ | Kernel-Grenze: Syscall-ABI definiert, Tests dünn |
| atc-ide | 84a233b4fbe5 | ✓ (73 TS-Dateien) | ~ (1) | ✓ 2/2 | ✗ | ✗ | Gate-Tiefe gering |
| atclang | 516482abbfa9 | ✓ | ✓ (6) | ✓ 3/3 (schmal) | ✗ | ✗ | Freeze/Conformance nicht ausgewiesen |
| genesis-engine | 3c2be8c11c00 | ✓ | ✓ (3) | ✓ 14/14 | ✗ | ✗ | |
| genesis-franchise-factory | 673fa560322b | ✓ | ✓ (6) | ✓ 11/11 | ✗ | ✗ | |
| .github (Hub) | 7f76c5092cee | ✓ (Validatoren) | ✓ (3) | ✓ 2/2 | n/a | ✗ | Control Plane |
| atc-launchpad | 83067c0bd13e | ✓ | ✗ (0) | ✗ cargo-tests | ✗ | ✗ | |
| atc-marketplace | 2e2bc090c982 | ✓ | ✗ (pytest rot) | ✗ | ✗ | ✗ | |
| atc-node | 821d049efc5b | ✓ (Rust) | ✗ (0 Testdateien) | ✗ markdownlint | ✗ | ✗ | **Kritischster Rückstand**: kein reproduzierbarer Build, keine Tests |
| genesis-chronicles | 361dbc0a9e43 | ✓ | ✗ (pytest rot) | ✗ (ruff, quality, pytest) | ✗ | ✗ | |
| atc-toolchain | e42aed3ece0e | ✗ (10 Dateien, 0 Build-Manifeste) | ✗ | ✓ 4/4 | ✗ | ✗ | Derzeit Spec/Doku, keine Toolchain |
| demo-repository | 58c6bf577cee | n/a | n/a | ✗ (0 Checks) | n/a | n/a | Demo, kein Reifeziel |

### Archivierte Repositories (7) — kein Reifeziel, Funktion im Monorepo

atc-wallet (812a31d29236), atc-explorer (c4da05304f21), atc-indexer (7f3da49f3046),
atc-interop (5905fca02274), atc-mining (227f2d700307), atc-oracle (8087d34ddb8c),
atc-storage (230746c1dc5e).

## 4. Die fünf Kennzahlen

| KPI | Wert | Quelle |
|---|---|---|
| **1. Implementierungsfortschritt** | 65 enforced (12 %) · 132 implemented (25 %) · 330 specification_only (62 %) · 7 reference | standard-implementation.yaml, 534/534 Matrix-Coverage |
| **2. Testabdeckung** | 11 von 26 aktiven Repos mit nachgewiesenen grünen Tests (42 %); 7 aktive Repos ohne jede Testdatei | check-runs + ls-tree |
| **3. Integrationsfortschritt** | **0 E2E_VERIFIED** über Repo-Grenzen; lokal belegt: atc-system (46 Tests, 11 E2E, 11.11.), VM 132 Tests; E2E-Workflows existieren in 3 Repos, keiner grün am HEAD | CHANGELOG + check-runs |
| **4. Sicherheitsstatus** | 184 offene Alerts: **16 kritisch, 47 hoch**, 94 mittel, 27 niedrig. Verteilung: a-townchain ≥100 (Konsolidierungsimporte), ecosystem 56, os-docs 20, aurora-ai 6. Security-Audit: NOT AUDITED | Dependabot API |
| **5. Verifikationsstatus** | **VERIFIED: 0** · CI_VERIFIED: 16 · CI rot am HEAD: 9 · E2E_VERIFIED: 0 | check-runs je HEAD |

## 5. Befund

Die Organisation hat in der Woche 1.796 Commits erzeugt, aber keinen einzigen
Reifegradsprung auf VERIFIED oder E2E_VERIFIED. Der kritische Engpass ist nicht
Feature-Volumen, sondern die **HEAD-stabile Gate-Lage**: Ausgerechnet der
Monorepo-SSOT (`a-townchain`) ist mit 6 von 15 Checks rot, darunter
`determinism`, `pytest` und `quality`. Gleichzeitig tragen 16 Repos formal
grüne CI, davon mehrere mit sehr schmaler Gate-Tiefe (atclang 3 Checks,
atc-ide 2, .github 2).

Die Architekturarbeit (Syscall-ABI, ATC-ARCH-002, atc-system-Integrationspfad)
ist real, aber rein spezifizierend: Sie erzeugt noch keine nachgewiesene
Systemfunktion. Das deckt sich mit der STATUS.md des Monorepos
(„Production Readiness: NO-GO, Security: NOT AUDITED“).

## 6. Empfohlene Baufolge (evidence-basiert)

| Prio | Ziel | Konkrete Maßnahme | Definition of Done |
|---|---|---|---|
| P0 | a-townchain SSOT | 6 rote Checks am HEAD beheben (determinism, pytest, npm-tests, ruff, quality, markdownlint); 100+ Dependabot-Alerts aus Konsolidierungsimporten triagieren | HEAD aller Checks grün, SHA-gebunden |
| P0 | atc-node | PR #15 (reproduzierbarer Build) und #16 (Cargo.lock) abschließen; erste Testsuite anlegen | Reproducible-Build-Evidenz + ≥1 grüne Testsuite |
| P0 | globus-os | Syscall-ABI implementieren statt spezifizieren; cargo-tests reparieren; Testsuite aufbauen (0 vorhanden) | Syscall-Tests grün am HEAD |
| P1 | atc-standards | `regenerate`-Drift beheben; 330 specification_only in Implementierungsaufträge mit Statuspflicht überführen | Generator-Check grün; Auftragsregister |
| P1 | aurora-ai | Runtime-E2E Aurora → Policy → Tool → Execution gegen GlobusOS/atc-system nachweisen | E2E-Workflow grün am HEAD |
| P1 | atclang / atc-vm | Compiler-/Bytecode-Freeze und Conformance-Suite (Golden Vectors, Differential-Tests) ausweisen | Conformance-Evidenz HEAD-gebunden |
| P2 | genesis-engine | Eigenständigen Implementierungsstatus erheben (Runtime/Editor/Netzwerk) | Statusbericht je Subsystem |
| P2 | Ökosystem | 176 Sync-Commits/Woche automatisieren; Sync an Monorepo-SSOT binden | Sync-Workflow statt manueller Commits |

## 7. Metadaten

- Erhebungszeitpunkt: 2026-11-18, 11:50 Europe/Berlin
- HEAD-Bindung: ls-remote je Standard-Branch + check-runs-API je exaktem SHA
- Rohdaten: maturity/data.json, maturity/checks.json, maturity/week_commits.json (Arbeitsbereich)
- scr: SCR-0144 · agent: ATC-AI-ARCH-001 · owner-auftrag: Michael Wroblewski
