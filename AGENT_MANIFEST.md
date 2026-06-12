# AI Agent Manifest — A-TownChain OS
> Version: 3.2.1 | Released: 2026-06-12 | Aurora v3.2

## Schnell-Onboarding (30 Sekunden)
1. Repos: a-townchain-os (Code) + a-townchain-os-docs (Docs)
2. Kanonische Module: modules/kernel/, modules/atcnet/, atclang/, monitoring/
3. Deprecated: core/atcfs.py, core/ai_kernel.py → Shims
4. Milestone: v3.2.1 RELEASED | Nächstes: v3.3.0

## Repositories
| Repo | URL | Inhalt |
|------|-----|--------|
| Code | https://github.com/A-TownChain-Okosystems/a-townchain-os | Ausführbarer Code |
| Docs | https://github.com/A-TownChain-Okosystems/a-townchain-os-docs | Wiki, Roadmap |

## Schlüssel-Imports (v3.2.1)
```python
from modules.kernel.atcfs.atcfs      import ATCFileSystem, ATCFS, get_vfs, get_atcfs
from modules.kernel.ai_kernel.ai_kernel import AIKernel, AIRequest, get_ai_kernel
from modules.kernel.ipc.ipc_bus      import IPCBus
from modules.kernel.process.process_mgr import ProcessManager, get_process_manager
from modules.atcnet.p2p_node         import P2PNode
from modules.atcnet.gossip           import GossipProtocol
from modules.atcnet.nat_traversal    import NATTraversal, get_nat
from atclang.type_checker            import TypeChecker
from monitoring.prometheus_metrics   import get_registry, start_metrics_server
from gateway.service_discovery       import ServiceDiscovery, get_discovery
from blockchain.propagation.block_gossip import BlockGossip
```

## Google Sheets Dashboard
- ID: 1xR5c24NrtYC58OsGrLaUHkQUiL_O6eYVyx8KmFcvBD4
- URL: https://docs.google.com/spreadsheets/d/1xR5c24NrtYC58OsGrLaUHkQUiL_O6eYVyx8KmFcvBD4
- Tabs: Modul-Status | Offene Aufgaben | Projekt-Übersicht | Commit-Log

## Notion
- Protokoll: 37bb826d-b85c-81c4-bdd4-cfc0dc74de7e
- Roadmap:   373b826d-b85c-8125-ba83-f04995191bf0
- To-Do:     986b826d-b85c-820a-b4e0-8107c069380b

## Master Sync
- Täglich: 08:05 Europe/Berlin
- Schritte: 20 | Aktiv: 17 | Übersprungen: 3

## Issue-Status
- Offen: 6 (#45, #46, #47, #49, #52, #67)
- Geschlossen: 57+

---
*Aurora Superagent v3.2 · 2026-06-12*
