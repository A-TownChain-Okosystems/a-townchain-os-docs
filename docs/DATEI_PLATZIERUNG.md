# 📂 Datei-Platzierungs-Analyse — A-TownChain Ökosystem

> **Erstellt:** 2026-08-06 14:45 UTC | **Agent:** Aurora (MasterBrain · Base44)

---

## Zusammenfassung

| Problem | Anzahl | Aktion |
|---------|--------|--------|
| Dateien im Monorepo die in andere Repos gehören | 459 (124K Zeilen) | In Ziel-Repos kopieren |
| Dateien im falschen modularen Repo | 134 | Verschieben |
| Verbleibende Duplikate (Monorepo ↔ modular) | 247 | Aus Monorepo entfernen |
| Fehlende Dateien in Ziel-Repos | 203 (36K Zeilen) | Aus Monorepo kopieren |

---

## A. Dateien im Monorepo die in andere Repos gehören (459 Dateien, 124K Zeilen)

| Ziel-Repo | Dateien | Zeilen | Beispiele |
|-----------|---------|--------|-----------|
| atc-kernel | 86 | 16.959 | kai_cli.atc, consensus.atc, poh_integration.py |
| atc-shivacore | 65 | 53.938 | Cargo.toml, main.rs, atcfs.rs, consensus.rs |
| atclang | 62 | 18.941 | compiler.py, type_checker.py, lexer.py, atcvm.py |
| atc-blockchain | 57 | 7.587 | fork_resolution.atc, gas_fee.atc, hybrid_consensus.atc |
| atc-aistudio | 36 | 7.155 | federated_learning.atc, render_pipeline.atc, mod_system.atc |
| atc-franchise | 34 | 4.638 | registry.atc, revenue.atc, token.atc, routes.py |
| atc-gateway | 29 | 2.186 | main.atc, auth.atc, logger.atc, router.atc |
| atc-contracts | 26 | 3.727 | atc8300.atc, atcoin.py, keygen.py, ecdsa.py |
| atc-ci | 16 | 1.545 | monitor.atc, prometheus_metrics.atc, blockchain_alerts.yml |
| atcnet | 12 | 2.440 | bootstrap_client.atc, discovery.atc, gossip.atc |
| atc-backend | 11 | 1.821 | kai_routes.atc, orchestrator.atc, ai_routes.atc |
| atc-frontend | 10 | 739 | api.js, package.json, tsconfig.json |
| atc-shivamon | 6 | 1.032 | game_routes.py, marketplace_routes.py |
| atc-cli | 4 | 623 | ecdsa_impl.atc, bigquery_pipeline.atc |
| atc-atcpkg | 3 | 498 | manager.atc |
| atc-mobile | 2 | 350 | biometric_auth.atc, wallet_api.atc |

## B. Dateien im falschen modularen Repo (134 Dateien)

### atc-aistudio (34 falsch)
- server.ts → atc-backend
- src/DesktopApp.tsx → atc-frontend
- src/standardsData.ts → atc-standards
- src/marketplaceApps.ts → atc-shivamon
- workspace/*.ts → atc-cli
- +24 weitere

### atc-atclang (18 falsch)
- lexer/lexer.py → atclang
- vm/atcvm.py → atc-vm
- stdlib/*.py (7 Dateien) → atc-stdlib
- repl/repl.py → atc-cli
- +8 weitere

### atc-blockchain (33 falsch)
- smart_contracts.py → atc-contracts
- smart_contract_registry.py → atc-franchise
- network/*.atc (6 Dateien) → atcnet
- +23 weitere

### atc-contracts (7 falsch)
- shivamon/*.py → atc-shivamon
- marketplace/*.py → atc-shivamon
- wallet/*.py → atc-mobile
- governance.atc → atc-blockchain

### atc-kernel (5 falsch)
- consensus/*.atc → atc-blockchain
- net/atcnet.* → atcnet

### atclang (12 falsch)
- vm.py → atc-vm
- stdlib/atc_stdlib.py → atc-stdlib
- repl/repl.py → atc-cli
- programs/*.atc → jeweilige Ziel-Repos

### Andere (25 falsch)
- atc-franchise: revenue.atc → atc-contracts, routes.py → atc-backend
- atc-gateway: service_discovery.atc → atcnet
- atc-atcpkg: tools/manager.atc → atc-shivacore-tools
- atc-backend: wallet.atc → atc-mobile
- atc-shivamon: game_routes.py → atc-backend

## C. Verbleibende Duplikate (247 Dateien)

- 247 Dateien identisch in Monorepo UND modularem Repo
- Hauptsächlich: atclang/ ↔ atc-atclang (15+ Dateien), shivacore/ ↔ atc-shivacore (65+ Dateien)
- Aktion: Aus Monorepo entfernen (kanonische Version im modularen Repo)

## D. Fehlende Dateien in Ziel-Repos (203 Dateien, 36K Zeilen)

| Repo | Fehlend | Zeilen | Top-Dateien |
|------|---------|--------|-------------|
| atc-kernel | 62 | 12.344 | start.atc, kai_cli.atc, message_bus_ad03.atc |
| atclang | 32 | 7.409 | io_ext.py, io.py, chain.py |
| atc-aistudio | 36 | 7.155 | federated_learning.atc, render_pipeline.atc |
| atc-franchise | 26 | 3.863 | world_factory_ad22.atc, economy_factory_ad26.atc |
| atc-contracts | 13 | 1.665 | atcoin.atc, atc8300_token.atc, keygen.atc |
| atc-ci | 16 | 1.545 | prometheus_metrics.atc, monitor.atc |
| atcnet | 6 | 926 | discovery.atc, p2p_node.atc |
| atc-cli | 4 | 623 | ecdsa_impl.atc, bigquery_pipeline.atc |
| atc-frontend | 7 | 405 | jest.config.js, tsconfig.json |
| atc-blockchain | 1 | 151 | snapshot.atc |

---

## Aktionsplan

1. **247 Duplikate aus Monorepo entfernen** — kanonische Version im modularen Repo
2. **203 fehlende Dateien in Ziel-Repos kopieren** — aus Monorepo in jeweiliges Repo
3. **134 falsch platzierte Dateien verschieben** — zwischen modularen Repos
4. **Monorepo als reine Orchestration behalten** — nur Verweise, nicht Code-Duplikate

---

*Auto-generiert 2026-08-06 · Aurora (MasterBrain · Base44)*
