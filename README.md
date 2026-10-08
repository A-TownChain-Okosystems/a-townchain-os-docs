# A-TownChain OS / KAI-OS — Dokumentations-Hub

> Dokumentations-, Architektur- und Governance-Einstieg für das A-TownChain-Ökosystem.

**Dokumentationssnapshot:** 2026-10-09 · **Lizenz:** Apache-2.0

## Pflichtlektüre für Änderungen

1. [Agent Policy](docs/AGENT_POLICY.md) — Änderungs- und Evidenzregeln.
2. [Agent Coordination](docs/AGENT_COORDINATION.md) — Koordination; Zeitstempel und Aktualität prüfen.
3. [Decision Register](docs/DECISIONS_REGISTER.md) — Entscheidungen im Kontext späterer Beschlüsse und kanonischer Quellen lesen.
4. [Current Ecosystem Status](docs/ECOSYSTEM_CURRENT_STATUS.md) — begrenzte Inventur und SSOT-Grenzen vom 2026-10-09.
5. [Current Architecture Reference](docs/ARCHITECTURE_CURRENT.md) — konsolidiertes Ownership-, Security- und Evidence-Modell.
6. [Roadmap](docs/ROADMAP.md) · [Status](STATUS.md) · [Changelog](CHANGELOG.md).

## Inventur und Reifegrad

Die GitHub-Inventur vom 2026-10-09 ergab **33 Repositories: 26 nicht archiviert und 7 archiviert**. „Nicht archiviert“ bedeutet nicht automatisch aktiv gepflegt, implementiert, verifiziert oder produktionsreif. Die Statusangaben der Einzel-Repositories sind Deklarationen, sofern sie nicht durch passende, aktuelle Evidenz unabhängig belegt wurden.

Ältere Angaben wie **128 Repositories, 2 aktiv und 126 archiviert**, alte Modul-/Testzahlen und frühere Link-Audit-Ergebnisse sind historische Aussagen und keine aktuellen Metriken.

## Kanonische Zuständigkeiten (SSOT)

| Domäne | Kanonische Quelle | Abgrenzung |
|---|---|---|
| Normative Standards | [`atc-standards`](https://github.com/A-TownChain-Okosystems/atc-standards) | `docs/standards/` in diesem Hub ist Archiv-/Referenzmaterial, keine normative SSOT. |
| Blockchain Core / Chain-State | [`a-townchain`](https://github.com/A-TownChain-Okosystems/a-townchain) | Protokoll, State und Chain-Integration. |
| ATC-VM | `a-townchain/components/vm` | Keine konkurrierende Produktions-VM in einem separaten Repository behaupten. |
| Algorithmus / Konsens | `a-townchain/components/algorithm` | Konsens- und Algorithmus-Implementierung bleibt in der kanonischen Quelle. |
| ShivaCore Kernel / TCB | [`globus-os/modules/atc-shivacore/kernel/`](https://github.com/A-TownChain-Okosystems/globus-os/tree/main/modules/atc-shivacore/kernel) | `atc-shivacore` pflegt unterstützende Spezifikationen, Governance und Support. |
| OS und Systemdienste | [`globus-os`](https://github.com/A-TownChain-Okosystems/globus-os) | Userspace/Services oberhalb des ShivaCore-TCB. |
| AI / Agents | [`aurora-ai`](https://github.com/A-TownChain-Okosystems/aurora-ai) | Aurora AI ist nicht TCB, Konsensautorität, ATC-VM oder kanonische Chain-Autorität. |
| Toolchain | [`atc-toolchain`](https://github.com/A-TownChain-Okosystems/atc-toolchain) | Entwicklungs-/Build-/Governance-Werkzeuge; nicht die Zielsoftware selbst. |
| Integration und Evidence | [`a-townchain-ecosystem`](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem) | Control Plane für Integration, Compliance und Nachweise; keine konkurrierenden Komponentenimplementierungen. |
| Systemorchestrierung | [`a-townchain-os`](https://github.com/A-TownChain-Okosystems/a-townchain-os) | Cross-Repo-Integration und Validierung; ersetzt keine Komponenten-SSOT. |

## Statussemantik und Nachweisregeln

- **SPECIFIED:** Anforderungen oder Design sind dokumentiert.
- **IMPLEMENTED:** Implementierung vorhanden; Korrektheit und Vollständigkeit sind damit nicht bewiesen.
- **VERIFIED:** relevante Prüfung ist an den exakten behaupteten Quell-SHA gebunden. Evidence enthält Workflow-Run, Job, Step, Exit-Status und Logs.
- **AUDITED**, **CONFORMANT**, **PRODUCTION_READY** und **MAINNET_READY** sind eigenständige Zustände mit eigenen Kriterien.
- Historische grüne Runs, Merge-Referenzen, README-Badges und auditierte Scores allein belegen keinen aktuellen Status.
- Fehlgeschlagene, blockierte und residuale Findings bleiben sichtbar, bis sie korrigiert und auf dem neuen SHA erneut geprüft wurden.
- Keine CI-, Branch-Protection-, Security- oder Release-Gates abschwächen, um einen grünen Status zu erzeugen.

## Wiki, Standards und historische Dokumente

- [KAI-OS Wiki](docs/kai-os-wiki.md)
- [Standards-Referenzsnapshot](docs/standards/STANDARDS_REGISTRY.md) — zur Orientierung; normative Wahrheit bleibt in `atc-standards`.
- [Whitepaper](docs/whitepaper/)
- [Compliance-Dokumente](docs/compliance/) — Dokumentation ist kein Nachweis einer behördlichen Zulassung oder rechtlichen Konformität.
- [Historisches Archiv](docs/archive/)

Historische Wiki-Kapitel, Sprintberichte, Roadmaps und Compliance-Dokumente müssen anhand von Datum, Quellen-SHA und ursprünglichem Zweck bewertet werden. Nicht verifizierte Inhalte nicht als aktuelle Implementierung oder Freigabe darstellen.

## Mitwirken

Dokumentationsänderungen werden auf einem dedizierten Branch committed und per Pull Request eingereicht. Generated views, Registry-Zahlen und normative Standards dürfen nicht manuell in diesem Hub überschrieben werden. Vor Merge sind die geforderten Checks und die Review-Policy zu erfüllen.

Copyright © 2026 A-TownChain-Okosystems. Apache-2.0.
