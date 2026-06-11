# KAI-OS Kernel — A-TownChain OS Core

**Version:** v2.0.0 | **Status:** Aktiv

## Ubersicht
Der KAI-OS Kernel ist der Kern des A-TownChain OS.
Verwaltet Prozesse, IPC, Dateisystem und AI-Integration.

## Module
- kernel.py: Haupt-Kernel (Process-Manager)
- ipc/ipc_bus.py: IPC Message Bus (Issue #51)
- atcfs/atcfs.py: Dezentrales Dateisystem
- ai_kernel/ai_kernel.py: LLM Router + HuggingFace
- process/process_mgr.py: Prozess-Scheduler

## IPC Topics
block.new | tx.broadcast | peer.connect | ai.query | fs.write

## Offene Issues
- **#51** IPC Bus: Vollstandige Kernel-Integration (Sprint 2.4)

_Stand: 2026-06-11 | Aurora Auto-Sync_
