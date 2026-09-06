# atc-ui

> ## 🤖 Fuer KI-Agenten — Pflichtlektuere vor jeder Aenderung
> Governance liegt zentral im Wiki-Repo `a-townchain-os-docs`:
> 1. [`AGENT_POLICY.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_POLICY.md) — verbindliche Regeln, Reality-Check, Konsolidierungsziel
> 2. [`AGENT_COORDINATION.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_COORDINATION.md) — wer arbeitet gerade woran, Todos, Agent-IDs
> 3. [`DECISIONS_REGISTER.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/DECISIONS_REGISTER.md) — verbindliche Architektur-Entscheidungen

> **Neon Design System & Reusable UI Components für KAI-OS**

[![Layer](https://img.shields.io/badge/Layer-L10-purple)](https://github.com/A-TownChain-Okosystems)
[![KAI-OS](https://img.shields.io/badge/KAI--OS-v1.0.0-blue)](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs)
[![Org](https://img.shields.io/badge/Org-A--TownChain--Okosystems-green)](https://github.com/A-TownChain-Okosystems)
[![Wiki](https://img.shields.io/badge/Wiki-📖-blue)](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/tree/main/docs/archive/wiki/atc-ui-wiki)

---

## 📖 Beschreibung

Das Repository **atc-ui** liefert die zentralen UI-Komponenten, das Neon Design System sowie vorgefertigte Application Shells (`DesktopApp.tsx`, `LoginOverlay.tsx`) für Anwendungen im A-TownChain OS Ökosystem.

---

## 🏗️ Architektur

atc-ui ist als modular wiederverwendbare Komponenten-Bibliothek aufgebaut. Sie stellt konsistente Themes, Cyberpunk/Neon UI Controls und Authentifizierungs-Dialoge bereit:

```
+-------------------------------------------------------+
|                    atc-ui (L10)                       |
|  +-------------------+  +--------------------------+  |
|  | DesktopApp Shell  |  | LoginOverlay & Auth      |  |
|  +-------------------+  +--------------------------+  |
|  | Neon Design System|  | Custom Inputs & Buttons  |  |
|  +-------------------+  +--------------------------+  |
+-------------------------------------------------------+
```

---

## 🧩 Komponenten

- **`DesktopApp.tsx` / `index.html`**: Universelle Desktop Application Container Shell mit Window Management und Navigation Drawer.
- **`LoginOverlay.tsx`**: Modaler Authentifizierungs-Overlays-Dialog mit Wallet-Login, Biometric-Prompt und Seed-Phrase Verification.
- **`assets/js/api.js`**: Standardisierter API Consumer für UI-Komponenten.
- **`DESIGN.md`**: Design-Guidelines, Farbpaletten (Neon Cyan `#00f3ff`, Neon Pink `#ff0055`, Dark Slate `#0a0e1a`) und Typografie-Regeln.

---

## 🚀 Usage

Einbinden der UI-Komponenten in Frontend-Projekte oder direktes Hosting der Standalone-UI Demo:

```bash
# Statischen Demo-Server starten
python3 -m http.server 8080

# Im Browser aufrufen
open http://localhost:8080
```

---

## 🛠️ Build & Installation

```bash
# Repo klonen
git clone https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-ui.git
cd atc-ui

# Entwicklungs-Vorschau starten
python3 -m http.server 8080
```

---

## 🗺️ Verwandte Repos

| Repo | Layer | Beschreibung |
|------|-------|-------------|
| [atc-frontend](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-frontend) | `L10` | React/TS Desktop UI & Dashboard |
| [atc-wallet](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-wallet) | `L10` | Wallet Application UI |
| [atc-explorer](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-explorer) | `L10` | Block Explorer UI |

---

## 📖 Wiki

Dokumentation und Komponentenspezifikationen finden Sie im [atc-ui-wiki](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/tree/main/docs/archive/wiki/atc-ui-wiki).

---

## Lizenz

Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. **All Rights Reserved.**

Dieses Projekt nutzt das **ATC-LIC Lizenzmodell** — ein monetarisiertes, autonomes Open-Source-Oekosystem. Unlizenzierter Code wird von der ATVM physisch nicht ausgefuehrt.

- [ATC-LIC — Smart Contract Licenses](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/standards/ATC-LIC-SMART_CONTRACT_LICENSE.md)
- [ATC-LIC — System & Hardware Licenses](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/standards/ATC-LIC-SYSTEM_HARDWARE_LICENSE.md)
- [Compliance-Handbuch (BaFin)](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/compliance/COMPLIANCE_HANDBUCH.md)
- [Lizenz-Uebersicht](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/LICENSING_OVERVIEW.md)
