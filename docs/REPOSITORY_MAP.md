# 📦 Repository-Aufteilung — 5-Produkt-Struktur

> **Beschluss:** 06.09.2026 (AD-014), erweitert durch AD-015 (Kernel-Repo) | **Agent:** Aurora | **Status:** UMGESETZT (API-verifiziert)

Das A-TownChain-Ökosystem wird neben Monorepo und Docs-Hub in **5 eigenständige
Produkt-Repos** gegliedert. Die Module wurden aus `a-townchain-os/src/modules/`
in die Produkt-Repos überführt (Copy — der Monorepo bleibt Integrations- und
Deployment-Zentrale, inkl. Unified Cargo Workspace und Kernel-Build).

## Produkt-Repos

| Repo | Produkt | Module | Umfang |
|---|---|---|---|
| [`atclang`](https://github.com/A-TownChain-Okosystems/atclang) | ATCLang (Programmiersprache) | atclang, atc-atclang, atc-vm, atc-stdlib | 138 Dateien / 32.347 Zeilen |
| [`a-townchain`](https://github.com/A-TownChain-Okosystems/a-townchain) | Blockchain (Chain-ID 658467) | atc-blockchain, atcnet, atc-wallet, atc-contracts, atc-bridge, atc-dex, atc-explorer, atc-assets, atc-zkp, atc-governance, atc-dns, atc-testnet | 369 Dateien / 21.604 Zeilen |
| [`globus-os`](https://github.com/A-TownChain-Okosystems/globus-os) | Betriebssystem (Userspace) | atc-kernel, atc-globus-shell/desktop/fs/net/registry/os, atc-bootloader, atc-drivers, atc-linux/windows-edition, atc-mobile | 361 Dateien / 86.395 Zeilen |
| [`atc-shivacore`](https://github.com/A-TownChain-Okosystems/atc-shivacore) | **ShivaCore-Kernel** (Microkernel, AD-012/013) | atc-shivacore (Kernel-Crate, 674/674 Tests), atc-shivacore-tools | 61 .rs-Dateien |
| [`aurora-ai`](https://github.com/A-TownChain-Okosystems/aurora-ai) | KI-System | atc-aurora-core/agents/memory/runtime/ai, atc-aistudio | 377 Dateien / 80.170 Zeilen |
| [`genesis-engine`](https://github.com/A-TownChain-Okosystems/genesis-engine) | Game-Engine | atc-genesis-engine/ecs/creatures/world, atc-game, atc-shivamon | 115 Dateien / 5.350 Zeilen |

## Verbleib im Monorepo (`a-townchain-os`)

Kern-Stack (`src/`: gateway, blockchain, core, contracts, franchise, game, frontend),
Plattform-Module (atc-backend, atc-gateway, atc-frontend, atc-ui, atc-cli, atc-sdk,
atc-ide, atc-devtools, atc-ci, atc-deploy, atc-monitoring, atc-security, atc-social,
atc-analytics, atc-atcpkg, atc-franchise, atc-standards, atc-whitepaper),
Unified Cargo Workspace, Docker-Stack, Tests.

## Regeln (ab 06.09.2026)

1. **Produkt-Entwicklung** (neue Features eines Produkts) läuft im jeweiligen Produkt-Repo.
2. **Integration/Deployment** (Workspace-Build, Docker, Mainnet-Stack) bleibt im Monorepo.
3. **Dokumentation** kanonisch im Docs-Hub (Architektur AD-012/AD-013, Standards, Entscheidungen).
4. Produkt-Repos: Modulpfad `modules/<modulname>/`, README + LICENSE nach Ökosystem-Standard (All Rights Reserved).
5. `atclang` behält seine alte Git-Historie (91 Commits); alte Inhalte sind via Datenverlust-Audit (03.09., 0 Verluste) im Monorepo + Hub-Archiv gesichert.

*Live-Stand API-verifiziert am 06.09.2026: alle 5 Produkt-Repos aktiv, HEAD-Commits bestätigt.*

## Alt-Repos: GELÖSCHT (06.09.2026)

Alle 124 archivierten Alt-Repos wurden am 06.09.2026 endgültig gelöscht
(Owner-Entscheidung). Vor der Löschung wurde JEDE der 2.966 Dateien per
SHA-256 gegen Monorepo + Hub verifiziert: 527 Dateien mit abweichendem
Original-Stand (pristin vor Copyright-/Link-Fixes) wurden nach
`docs/archive/repos-rescue/` gerettet, der Rest war byte-identisch in
`a-townchain-os` (src/modules/) bzw. hier im Hub vorhanden — 0 Datenverlust.
Die 9 Issues des Alt-Repos atc-genesis-engine liegen als
`docs/archive/repos-rescue/_ISSUES_DUMP.json`. Details: AD-016.

