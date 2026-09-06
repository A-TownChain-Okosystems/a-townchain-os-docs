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

## Sync-Regel (AD-017, 06.09.2026)

Produkt-Repos = kanonische Modul-Quelle; Monorepo = Integrationsziel.
Einspielen nur via `a-townchain-os/scripts/sync_modules.py` (`--check`/`--sync`).
Modul-Code nie direkt im Monorepo bearbeiten. Details: DECISIONS_REGISTER AD-017.

## Rebuild-Stand (AD-020, 06.09.2026)

| Repo | Zustand |
|---|---|
| a-townchain-os-docs | Wiki-VAULT (alle Inhalte, 15 MB Archive) |
| atclang | Neu aufgebaut (AD-019 Phase 1) |
| a-townchain-os, atc-shivacore, a-townchain, globus-os, aurora-ai, genesis-engine | Geleert — Redirect-README, Wiederaufbau aus Vault |

Git-Historien aller Repos bleiben erhalten (Reversibilität).

## Zielarchitektur-Landkarte (AD-024 UMGESETZT, 06.09.2026 — alle 14 Repos erstellt)

Alle 14 Repos wurden am 06.09.2026 (21:30–21:45 UTC+2) erstellt und initialisiert.
Vault-Restore: atc-sdk, atc-contracts, atc-wallet, atc-explorer, shivamon (Bestand aus
monorepo-full). Neu angelegt (Grundstruktur): atc-node, atc-indexer, atc-mining,
atc-interop, atc-oracle, atc-storage, atc-launchpad, atc-marketplace, atc-compute.
Organisation jetzt: 22 aktive Repos.

| Ziel-Repo | Heutiger Ort (Modul im Produkt-Repo/Vault) | Priorität | Promotion-Kriterium |
|---|---|---|---|
| atc-sdk | Monorepo-Plattformmodul atc-sdk (Vault) | P0 | ATCLang Rust-ABI stabil (AD-022-Gates) |
| atc-node | NEU (kein Vault-Bestand) — aus a-townchain/Protocol-Core | P0 | Protocol-Interface-Freeze (AD-013) |
| atc-contracts | a-townchain-Modul atc-contracts (Vault) | P0/P1 | ATVM-Contract-Kontext verifiziert |
| atc-wallet | a-townchain-Modul atc-wallet (Vault) | P0/P1 | Runtime-Signing/HD-Referenz stabil |
| atc-explorer + atc-indexer | a-townchain-Modul atc-explorer (Vault); Indexer NEU | P1 | Chain-RPC stabil |
| atc-mining | NEU (Core definiert PoW-Regeln; Mining-Stack separat) | P1 | Consensus-Rebuild fertig |
| atc-interop | a-townchain-Modul atc-bridge (Vault); IBC/Relayer NEU | P1 | Bridge-Security-Konzept (eigene Sicherheitsdomäne) |
| genesis-chronicles (ex-`shivamon`, umbenannt per AD-025) | Vault-Restore atc-shivamon | P1 | ECS-/World-Kern stabil |
| atc-oracle | NEU (Lücke „Oracle & External Data Binding") | P2 | Bedarfsfall DeFi/AI-Feeds |
| atc-storage | NEU | P2 | Asset-/NFT-Metadaten-Bedarf |
| atc-launchpad / atc-marketplace | a-townchain-Module atc-dex/atc-assets (Vault) | P2 | TGE-/Produkt-Entscheidung |

> Regel: KEIN neues Repo ohne Content — Abspaltung immer als Restore/Ableitung aus dem
> Vault bzw. Produkt-Repo, nie als leeres Skeleton (Lehre aus der 70-Repo-Ära).
