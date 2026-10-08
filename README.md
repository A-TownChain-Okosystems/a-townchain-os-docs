# 🧠⛓️ A-TownChain OS / KAI-OS — Offizielle Dokumentation

> ## 🤖 Für KI-Agenten — Pflichtlektüre vor jeder Änderung
> 1. [`docs/AGENT_POLICY.md`](docs/AGENT_POLICY.md) — verbindliche Regeln, Reality-Check, Konsolidierungsziel
> 2. [`docs/AGENT_COORDINATION.md`](docs/AGENT_COORDINATION.md) — wer arbeitet gerade woran, Agent-IDs
> 3. [`docs/DECISIONS_REGISTER.md`](docs/DECISIONS_REGISTER.md) — verbindliche Architektur-Entscheidungen

> Dezentrales, KI-gesteuertes Blockchain-Betriebssystem
> **A-TownChain OS** (technisch) · **KAI-OS** (Produktname)

Dies ist der **kanonische Dokumentations-Hub** des A-TownChain-Ökosystems.

**Version:** 1.0.0 | **Dokumentations-Refresh:** 09.10.2026 | **Lizenz:** Apache-2.0
**Autor:** Michael Wroblewski | **Agent:** Aurora (Base44 Superagent)

---

## Architektur- und SSOT-Policy

- **Normative Standards:** `atc-standards` ist die kanonische Registry; `docs/standards/` in diesem Hub ist nur ein historischer Referenz-Snapshot.
- **Blockchain Core:** `a-townchain`.
- **ATC-VM:** kanonische Implementierung unter `a-townchain/components/vm`.
- **Algorithmus/Konsens:** kanonische Implementierung unter `a-townchain/components/algorithm`.
- **ShivaCore Kernel:** kanonische Quelle unter `globus-os/modules/atc-shivacore/kernel/`; `atc-shivacore` bleibt Specs/Governance/Support.
- **Integration:** `a-townchain-os` und `a-townchain-ecosystem` koordinieren Integration und Evidence, nicht konkurrierende Implementierungen.
- **Aurora AI:** Service-/AI-Schicht, nicht ShivaCore-TCB, Konsensautorität, ATC-VM oder kanonische Chain-Autorität.

## Metriken (Inventar-Snapshot 09.10.2026)

| Metrik | Wert |
|--------|------|
| Repositories | 33 total — **26 nicht archiviert, 7 archiviert** (GitHub-Inventur 09.10.2026) |
| Status-Evidence | Repository-README-Angaben sind deklarierte Zustände, keine unabhängige Verifikation |
| Normative Standards | Kanonische Registry: `atc-standards`; `docs/standards/` ist Archiv-/Referenzmaterial |
| ShivaCore Kernel | Kanonische Quelle: `globus-os/modules/atc-shivacore/kernel/`; keine aktuelle Produktionsfreigabe abgeleitet |
| Systemorchestrierung | `a-townchain-os` koordiniert Integration; Komponentenimplementierungen bleiben in ihren kanonischen Repositories |
| Interne Links | Frühere Link-Audits sind historische Evidence; aktueller Zustand muss neu geprüft werden |
| Mainnet | Kein freigegebener Mainnet-Termin; Release-Readiness nur anhand aktueller Gates/Evidence |

## Aktueller Status — Einordnung

| Bereich | Aktuelle Einordnung |
|---|---|
| Gesamtbestand | 33 Repositories; 26 nicht archiviert, 7 archiviert (GitHub-Inventur 09.10.2026) |
| `a-townchain` | Development / nicht production-ready; Mainnet NO-GO laut aktuellem README |
| `atc-algorithm` | MVP/Skeleton laut README; kein produktionsreifer Konsens behauptet |
| `atc-vm` | README beschreibt R1-Skeleton; kanonische VM-Implementierung liegt in `a-townchain/components/vm` |
| `globus-os` / ShivaCore | Entwicklungsstatus; Implementierung und Verifikation sind getrennte Aussagen |
| Historische K-Sprints | Alte Sprint- und Testzahlen bleiben historische Berichte und sind keine aktuelle Verifikation |

Die Übersicht ist eine begrenzte Inventur und README-Sichtung, kein vollständiger Audit-Lauf. Details und Evidenzgrenzen stehen in [ECOSYSTEM_CURRENT_STATUS.md](docs/ECOSYSTEM_CURRENT_STATUS.md).

## Quick Links

- [Roadmap](docs/ROADMAP.md) | [Status](STATUS.md) | [TODO](TODO.md) | [Changelog](CHANGELOG.md)
- [Standards Registry](docs/standards/STANDARDS_REGISTRY.md) | [KAI-OS Wiki](docs/kai-os-wiki.md)
- [Launch-Checkliste & Roadmap](docs/project/) | [Whitepaper](docs/whitepaper/)
- [Compliance (BaFin)](docs/compliance/) | [Lizenz-Übersicht](docs/LICENSING_OVERVIEW.md)
- [Agent Master Rules](AGENT_MASTERRULES.md) | [ATCLang First Policy](ATCLANG_FIRST.md)

## Hub-Struktur

```
docs/
├── standards/       # ATC-01…35 + ATC-LIC/ATS-LIC (kanonisch)
├── wiki/            # KI-OS Wiki-Struktur (Kapitel je Modul)
├── whitepaper/      # Whitepaper + Einzelkapitel
├── compliance/      # BaFin-Konformität, Richtlinien
├── project/         # Master-Component-Plan, Launch-Checkliste
├── issues/ sprints/ # Issue- und Sprint-Dokumentation
├── archive/
│   ├── wiki/        # 65 Modul-Wikis ( verschmolzen, mit Index)
│   └── kai-os-legacy/
└── monorepo-legacy/ # Historische Monorepo-Docs (eingefroren)
```

## Repository-Struktur (Dual-Repo-Modell)

```
a-townchain-os/        # Systemorchestrierung und Integrationsvalidierung
a-townchain-os-docs/  # Doku-Hub (dieses Repo): Standards, Wiki, Whitepaper, Archive
```

---

*A-TownChain OS / KAI-OS · Dokumentations-Hub · Snapshot 09.10.2026 · Status stets gegen kanonische Quellen prüfen*

## Lizenzmodell

A-TownChain etabliert ein **monetarisiertes, autonomes Open-Source-Ökosystem**.

**"Code is Law" auf Lizenzebene:** Unlizenzierter Code wird von der ATVM physisch gar nicht erst ausgeführt.

- **ATC-LIC** — Smart Contract Lizenzen mit automatischer Royalty-Durchsetzung
- **ATS-LIC** — System & Hardware Lizenzen mit TPM-Verifikation
- **Compliance-Handbuch** — BaFin-konforme Dokumentation

→ [Lizenz-Übersicht](docs/LICENSING_OVERVIEW.md) | [ATC-LIC](docs/standards/ATC-LIC-SMART_CONTRACT_LICENSE.md) | [ATS-LIC](docs/standards/ATS-LIC-SYSTEM_HARDWARE_LICENSE.md) | [Compliance-Handbuch](docs/compliance/COMPLIANCE_HANDBUCH.md)

Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. Apache-2.0.

## Verwandte Vision-Projekte

- [`atc-genesis-engine`](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-genesis-engine) — Vision-/Konzept-Modul für eine potenzielle zukünftige Game-Engine (Genesis Engine) und deren Ausbaustufen. Reines Konzeptmaterial, kein aktueller Teil der A-TownChain-Kernentwicklung.

---

**Last Updated:** 2026-10-09 — inventory and status wording refreshed; see [ECOSYSTEM_CURRENT_STATUS.md](docs/ECOSYSTEM_CURRENT_STATUS.md).

---

## ATC Compliance & Governance (ATC-STD-201 / 202 / 203)

**ATC COMPLIANCE: R3** — repository registry classification (R-Level aus `.atc/repository.yaml`). This label is not a current audit approval or production-readiness claim; check exact-SHA workflow evidence.
Architekturentscheidungen: zentral im [DECISIONS_REGISTER](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/DECISIONS_REGISTER.md) (AD-Nummern verbindlich; lokale Entscheidungen in `docs/decisions/`).

- **Purpose:** Dokumentations-Hub des Oekosystems (parallel zu L0-L7).
- **Scope:** Dokumentations-Hub für die aktuell gelisteten 33 Repositories (26 nicht archiviert, 7 archiviert; Snapshot 09.10.2026).
- **Architecture:** DECISIONS_REGISTER (AD-001…), Roadmaps (M1-M8, AD-027), Audits, Projekt-Doku; Standards: kanonisch im atc-standards-Repo (AD-030), Hub docs/standards = Archiv-Snapshot.
- **Features:** Wiki, Whitepaper, Compliance-Handbuch, BaFin-Bericht, REPOSITORY_MAP.
- **Installation:** Modul-Build je Sprache (markdown); Integration via Monorepo-Workspace (a-townchain-os, sync_modules.py).
- **Development:** Conventional Commits; Governance-Regeln aus atc-standards; Naming gemaess ATC-STD-000 §7.
- **Testing:** Frühere interne Link-Audits (873+94 Links geprüft, 0 kaputt) sind historische Evidence; aktuelle Gate-Zustände müssen über die zugehörigen Run-/Job-Logs belegt werden.
- **Security:** SECURITY.md; S-Klasse S3; ATC-STD-203 Release-Gates; Emergency-Prozess ATC-STD-000 §32.
- **Roadmap:** Einordnung in die Lauffaehigkeits-Roadmap M1-M8 (AD-027) und Bauhierarchie L0-L7 (AD-026).
- **Version:** CHANGELOG.md; SemVer; Releases als ATC-REL-X.Y.Z.
- **License:** Apache-2.0 — Apache-2.0, Michael Wroblewski / ShivaCore / A-TownChain-Okosystems (ATC-LIC/ATS-LIC).
