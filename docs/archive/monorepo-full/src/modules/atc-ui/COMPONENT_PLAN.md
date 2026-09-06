# 📋 Komponenten-Plan — atc-ui

> **Erstellt:** 2026-08-06 | **Agent:** Aurora (MasterBrain · Base44)

## Übersicht

**Repo:** `atc-ui`
**Name:** ATC UI — Terminal-UI
**Beschreibung:** Terminal-basierte UI. Renderer, Dashboard, Panels, Text-Boxen, Progress-Bars, Komponenten für Deployment, Hardware, Franchise, ZK-Circuits, Whitepaper-Viewer.
**Layer:** L7 — Application
**Sprint:** 3.0
**ATC-Standards:** ATC-24, ATC-45
**Komponenten:** 14

---

## Komponenten-Liste

| # | Datei | Zeilen | Typ | Beschreibung |
|---|-------|--------|-----|-------------|
| 1 | `assets/js/api.js` | 99 | .js | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 2 | `src/components/ATownDashboardView.tsx` | 302 | .tsx | — |
| 3 | `src/components/AtsSuite.tsx` | 51 | .tsx | — |
| 4 | `src/components/ConsensusIntegrationGuide.tsx` | 1,528 | .tsx | — |
| 5 | `src/components/DeFiLiquidityPoolView.tsx` | 255 | .tsx | — |
| 6 | `src/components/GitHubStatusDashboard.tsx` | 643 | .tsx | — |
| 7 | `src/components/MetricsDashboard.tsx` | 105 | .tsx | — |
| 8 | `src/components/OfficeSuiteView.tsx` | 271 | .tsx | — |
| 9 | `src/components/ProjectAuditDashboard.tsx` | 135 | .tsx | — |
| 10 | `src/components/SyncDashboardModal.tsx` | 88 | .tsx | — |
| 11 | `src/components/SystemHealthDashboard.tsx` | 246 | .tsx | — |
| 12 | `src/components/SystemHealthDashboardWidget.tsx` | 63 | .tsx | Mocking "internal metrics store" |
| 13 | `src/components/TerminalView.tsx` | 189 | .tsx | — |
| 14 | `src/components/ZkCircuitEditorView.tsx` | 108 | .tsx | — |

---

## Detaillierte Komponenten

### 1. `assets/js/api.js`

**Datei:** `assets/js/api.js`
**Zeilen:** 99
**Typ:** .js
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** —

**Status:** 🟢 IMPLEMENTIERT

---

### 2. `src/components/ATownDashboardView.tsx`

**Datei:** `src/components/ATownDashboardView.tsx`
**Zeilen:** 302
**Typ:** .tsx
**Beschreibung:** —
**Funktionen/Structs:** —

**Status:** 🟢 IMPLEMENTIERT

---

### 3. `src/components/AtsSuite.tsx`

**Datei:** `src/components/AtsSuite.tsx`
**Zeilen:** 51
**Typ:** .tsx
**Beschreibung:** —
**Funktionen/Structs:** —

**Status:** 🟢 IMPLEMENTIERT

---

### 4. `src/components/ConsensusIntegrationGuide.tsx`

**Datei:** `src/components/ConsensusIntegrationGuide.tsx`
**Zeilen:** 1,528
**Typ:** .tsx
**Beschreibung:** —
**Funktionen/Structs:** struct Validator, struct LeaderSelectionEngine;, select_leader, struct CryptoEngine;, generate_keypair, sign_message, verify_message, struct ATown; (+9 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 5. `src/components/DeFiLiquidityPoolView.tsx`

**Datei:** `src/components/DeFiLiquidityPoolView.tsx`
**Zeilen:** 255
**Typ:** .tsx
**Beschreibung:** —
**Funktionen/Structs:** struct AtcLiquidityPool, new, swap_atc_to_token

**Status:** 🟢 IMPLEMENTIERT

---

### 6. `src/components/GitHubStatusDashboard.tsx`

**Datei:** `src/components/GitHubStatusDashboard.tsx`
**Zeilen:** 643
**Typ:** .tsx
**Beschreibung:** —
**Funktionen/Structs:** —

**Status:** 🟢 IMPLEMENTIERT

---

### 7. `src/components/MetricsDashboard.tsx`

**Datei:** `src/components/MetricsDashboard.tsx`
**Zeilen:** 105
**Typ:** .tsx
**Beschreibung:** —
**Funktionen/Structs:** —

**Status:** 🟢 IMPLEMENTIERT

---

### 8. `src/components/OfficeSuiteView.tsx`

**Datei:** `src/components/OfficeSuiteView.tsx`
**Zeilen:** 271
**Typ:** .tsx
**Beschreibung:** —
**Funktionen/Structs:** —

**Status:** 🟢 IMPLEMENTIERT

---

### 9. `src/components/ProjectAuditDashboard.tsx`

**Datei:** `src/components/ProjectAuditDashboard.tsx`
**Zeilen:** 135
**Typ:** .tsx
**Beschreibung:** —
**Funktionen/Structs:** —

**Status:** 🟢 IMPLEMENTIERT

---

### 10. `src/components/SyncDashboardModal.tsx`

**Datei:** `src/components/SyncDashboardModal.tsx`
**Zeilen:** 88
**Typ:** .tsx
**Beschreibung:** —
**Funktionen/Structs:** —

**Status:** 🟢 IMPLEMENTIERT

---

### 11. `src/components/SystemHealthDashboard.tsx`

**Datei:** `src/components/SystemHealthDashboard.tsx`
**Zeilen:** 246
**Typ:** .tsx
**Beschreibung:** —
**Funktionen/Structs:** —

**Status:** 🟢 IMPLEMENTIERT

---

### 12. `src/components/SystemHealthDashboardWidget.tsx`

**Datei:** `src/components/SystemHealthDashboardWidget.tsx`
**Zeilen:** 63
**Typ:** .tsx
**Beschreibung:** Mocking "internal metrics store"
**Funktionen/Structs:** —

**Status:** 🟢 IMPLEMENTIERT

---

### 13. `src/components/TerminalView.tsx`

**Datei:** `src/components/TerminalView.tsx`
**Zeilen:** 189
**Typ:** .tsx
**Beschreibung:** —
**Funktionen/Structs:** —

**Status:** 🟢 IMPLEMENTIERT

---

### 14. `src/components/ZkCircuitEditorView.tsx`

**Datei:** `src/components/ZkCircuitEditorView.tsx`
**Zeilen:** 108
**Typ:** .tsx
**Beschreibung:** —
**Funktionen/Structs:** —

**Status:** 🟢 IMPLEMENTIERT

---

## Test-Strategie

1. Parse-Test: Jede .atc Datei muss mit ATCLang v0.3 Parser parsen
2. Unit-Tests: Mindestens 3 Tests pro Komponente
3. Integration-Test: Komponenten interagieren korrekt
4. Coverage-Ziel: >80%

## Dokumentations-Requirements

- ARCHITECTURE.md: Architektur-Baum + Komponenten-Übersicht ✅
- COMPONENT_PLAN.md: Dieser Plan ✅
- FILE_REGISTER.md: Datei-Liste ✅
- STATUS.md: Aktueller Status ✅
- ROADMAP.md: Sprint-Zuordnung ✅
- CHANGELOG.md: Änderungs-Historie ✅

---
*Auto-generiert 2026-08-06 · Aurora (MasterBrain · Base44)*
