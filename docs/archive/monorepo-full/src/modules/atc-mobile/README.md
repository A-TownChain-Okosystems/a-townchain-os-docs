# atc-mobile

> ## 🤖 Fuer KI-Agenten — Pflichtlektuere vor jeder Aenderung
> Governance liegt zentral im Wiki-Repo `a-townchain-os-docs`:
> 1. [`AGENT_POLICY.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_POLICY.md) — verbindliche Regeln, Reality-Check, Konsolidierungsziel
> 2. [`AGENT_COORDINATION.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_COORDINATION.md) — wer arbeitet gerade woran, Todos, Agent-IDs
> 3. [`DECISIONS_REGISTER.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/DECISIONS_REGISTER.md) — verbindliche Architektur-Entscheidungen

> **React Native Mobile App & Mobile Wallet Bridge**

[![Layer](https://img.shields.io/badge/Layer-L10-purple)](https://github.com/A-TownChain-Okosystems)
[![KAI-OS](https://img.shields.io/badge/KAI--OS-v1.0.0-blue)](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs)
[![Org](https://img.shields.io/badge/Org-A--TownChain--Okosystems-green)](https://github.com/A-TownChain-Okosystems)
[![Wiki](https://img.shields.io/badge/Wiki-📖-blue)](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/tree/main/docs/archive/wiki/atc-mobile-wiki)

---

## 📖 Beschreibung

Das Repository **atc-mobile** stellt die mobile Anwendung (React Native) und zugehörige API-Anbindungen (`wallet_api.atc`) bereit, um A-TownChain OS Funktionen auf iOS- und Android-Geräten verfuegbar zu machen.

---

## 🏗️ Architektur

atc-mobile verbindet ein mobiles React Native UI mit nativer Biometrie und einer leichten Python/ATC Wallet-Bridge für Offline-Signierung und Push-Benachrichtigungen:

```
+-------------------------------------------------------+
|                  atc-mobile (L10)                     |
|  +--------------------+  +-------------------------+  |
|  | React Native UI    |  | Biometric Vault (Secure) |  |
|  +--------------------+  +-------------------------+  |
|  | wallet_api.atc     |  | Push Notification Node  |  |
|  +--------------------+  +-------------------------+  |
+--------------------------+----------------------------+
                           | Mobile Gateway API
                           v
              +--------------------------+
              |   atc-gateway (:4000)    |
              +--------------------------+
```

---

## 🧩 Komponenten

- **React Native Mobile Shell**: Plattformübergreifende Benutzeroberfläche für iOS und Android.
- **`wallet_api.atc`**: ATCLang-basierte Wallet-API-Anbindung für schnelle Transaktionsverarbeitung.
- **`wallet/`**: Module für Kontoverwaltung, QR-Code Scanner und Secure Enclave Speicherung.
- **Biometric Security**: FaceID / TouchID Integration zur Freigabe von Signaturprozessen.

---

## 🚀 Usage

Starten der mobilen Entwicklungsumgebung:

```bash
# Python API Bridge testen
python3 __init__.py
```

---

## 🛠️ Build & Installation

```bash
# Repo klonen
git clone https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-mobile.git
cd atc-mobile
```

---

## 🗺️ Verwandte Repos

| Repo | Layer | Beschreibung |
|------|-------|-------------|
| [atc-wallet](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-wallet) | `L10` | Wallet Application Core |
| [atc-gateway](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-gateway) | `L7` | Central API Gateway |
| [atc-sdk](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-sdk) | `L8` | Software Development Kit |

---

## 📖 Wiki

Dokumentation und Mobile-Architekturguides finden Sie im [atc-mobile-wiki](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/tree/main/docs/archive/wiki/atc-mobile-wiki).

---

## Lizenz

Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. **All Rights Reserved.**

Dieses Projekt nutzt das **ATC-LIC Lizenzmodell** — ein monetarisiertes, autonomes Open-Source-Oekosystem. Unlizenzierter Code wird von der ATVM physisch nicht ausgefuehrt.

- [ATC-LIC — Smart Contract Licenses](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/standards/ATC-LIC-SMART_CONTRACT_LICENSE.md)
- [ATC-LIC — System & Hardware Licenses](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/standards/ATC-LIC-SYSTEM_HARDWARE_LICENSE.md)
- [Compliance-Handbuch (BaFin)](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/compliance/COMPLIANCE_HANDBUCH.md)
- [Lizenz-Uebersicht](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/LICENSING_OVERVIEW.md)

## Abhängigkeiten
- [`A-TownChain-Okosystems/atc-backend`](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-backend)
