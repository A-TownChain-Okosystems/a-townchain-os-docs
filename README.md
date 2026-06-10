# 📖 A-TownChain OS — Wiki & Roadmap

> **Offizielle Dokumentation, Roadmap, TODOs & technische Spezifikationen**
> für [A-TownChain OS](https://github.com/A-TownChain-Okosystems/a-townchain-os)

---

## 🗂️ Struktur

```
a-townchain-os-docs/
│
├── docs/
│   ├── wiki/                  ← Modul-Wikis (kernel, gateway, contracts, atclang, atcnet, ui, shivamon, franchise, standards)
│   │   └── kai-os/            ← KAI-OS 31-Kapitel Theorie-Architektur
│   ├── whitepaper/            ← Offizielles Whitepaper v2.1.0
│   ├── architecture/          ← Architektur-Doku (Kernel, Gateway, Consensus, Testnet)
│   ├── standards/             ← ATC-0001–0009 + ATS-1000–1009 Protokoll-Standards
│   ├── ai/                    ← AI Safety, LLM Router, Gemini Integration
│   ├── roadmap/               ← Sprints & Meilensteine
│   ├── issues/                ← Issue-Tracking & Priorisierung
│   └── blockchain/            ← Chain-Spezifikationen
│
├── module-docs/               ← Technische Spezifikationen je Modul
│   ├── kernel/                ← ARCHITECTURE.md, SECURITY.md, CHANGELOG.md
│   ├── gateway/               ← CHANGELOG.md, SECURITY.md, .atc Spec
│   ├── contracts/             ← DEPLOYMENT.md, SECURITY.md, CHANGELOG.md
│   ├── atclang/               ← ATCLANG_SPEC.md, CONTRIBUTING.md, CHANGELOG.md
│   ├── atcnet/                ← PROTOCOL.md, SECURITY.md, CHANGELOG.md
│   ├── ui/                    ← DESIGN.md, CHANGELOG.md
│   ├── shivamon/              ← GAME_SPEC.md, CHANGELOG.md
│   ├── franchise/             ← ARCHITECTURE.md, SECURITY.md, CHANGELOG.md
│   └── standards/             ← ATC_STANDARDS.md, ATS_STANDARDS.md
│
├── TODO/
│   └── MASTER_TODO.md         ← Alle offenen Issues & Aufgaben
│
├── STATUS.md                  ← Aktueller Projektstatus (auto-sync)
├── CHANGELOG.md               ← Versions-History
└── ECOSYSTEM.md               ← Ökosystem-Übersicht
```

## 🔗 Software-Repository

Der ausführbare Code liegt unter:
👉 **[A-TownChain-Okosystems/a-townchain-os](https://github.com/A-TownChain-Okosystems/a-townchain-os)**

## 📋 Offene Issues

| # | Titel | Priorität | Status |
|---|-------|-----------|--------|
| 🔴 #8 | Multi-Node Testnet | HIGH | Blocker |
| 🟡 #18 | Docker Compose 5-Node | MEDIUM | Offen |
| 🟡 #19 | Node-Monitoring Dashboard | MEDIUM | Offen |
| 🟡 #7 | Build System EXE/AppImage | MEDIUM | Offen |
| 🟡 #11 | Shivamon Breeding Gen 2 | MEDIUM | Offen |
| 🟡 #12 | Solidity Smart Contracts | MEDIUM | Offen |
| 🟡 #13 | ATC Marketplace | MEDIUM | Offen |
| 🟢 #10 | Cross-Chain Bridge | LOW | Offen |

## 🗺️ Kritischer Entwicklungspfad

```
#14 Bootstrap → #15 Propagation → #16 Sync → #17 Fork Resolution → #18 Docker → #8 Multi-Node Testnet
```

## 🔄 Auto-Sync

Dieses Repository wird täglich um 08:00 Uhr (Berlin) automatisch aktualisiert.
Letzte Aktualisierung: 2026-06-10 · Aurora (KAI-OS Agent)

---
*Apache 2.0 | A-TownChain-Okosystems*
