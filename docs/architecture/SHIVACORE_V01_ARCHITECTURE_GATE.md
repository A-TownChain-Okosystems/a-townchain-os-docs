# ShivaCore v0.1 — Architektur-Gate (VERBINDLICH)

> **Status:** VERBINDLICHER ARCHITEKTUR-GATE (AD-013) · **Datum:** 06.09.2026
> **Basierdokument:** `SHIVACORE_KERNEL_ARCHITECTURE.md` (AD-012)
> **Geltung:** Alle SC-ARCH-Regeln sind normativ (Freeze) und müssen vor jeder
> Kernel-Implementierung geprüft werden. Ausnahmen nur über DECISIONS_REGISTER.

## Architektur-Gate (Kernaussage)

> **ShivaCore ist ein echter Capability-Microkernel.
> Globus OS ist das darauf aufbauende Userspace-Betriebssystem.**

Zweck: Vermeidung einer schleichenden Vermischung von Kernel- und OS-Funktionen.

---

## 1. Zielarchitektur (verbindlich)

```
┌─────────────────────────────────────────────────────────────────────┐
│                         APPLICATION SPACE                           │
│  Genesis │ Aurora │ Music │ Video │ 3D │ Office │ Browser │ Apps    │
├─────────────────────────────────────────────────────────────────────┤
│                         RUNTIME SPACE                               │
│  ATCLang Runtime │ WASM │ POSIX Layer │ Container │ Game Runtime    │
├─────────────────────────────────────────────────────────────────────┤
│                       GLOBUS OS SERVICES                            │
│  Init │ VFS │ Network │ GPU │ Audio │ USB │ Input │ Window │ AI    │
│  A-TownChain │ Wallet │ Crypto │ Package │ Update │ Logging        │
├─────────────────────────────────────────────────────────────────────┤
│                       SHIVACORE MICROKERNEL                         │
│  Scheduler │ IPC │ CSpace │ Memory │ Threads │ Interrupts │ Syscalls │
│                     Capability Security │ HAL                       │
├─────────────────────────────────────────────────────────────────────┤
│                         HARDWARE / HAL                              │
│          x86_64 │ AArch64 │ RISC-V │ CPU │ RAM │ MMIO │ DMA        │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 2. Korrektur: Capabilities sind KEINE Kryptografie-Objekte (Kern-Modell)

Für den Microkernel-Kern gilt normativ:

```
Capability Handle → CSpace Slot → Kernel Capability Object → Object + Rights
```

**Die Unfälschbarkeit entsteht durch Kernel-enforced Handles, CSpace-Isolation und
Objektverwaltung — NICHT durch Kryptografie.**

Kryptografische Tokens (DID/Signaturen, Remote-Caps) sind eine **spätere Ergänzung
für Cross-Domain- und persistent delegierte Berechtigungen** — nicht Teil des
Kern-Enforcement.

*Implementierungs-Delta:* `capability.rs` (K29) ist bereits handle-basiert
(CapabilityTable + Rights-Bitfield) ✅. `did.rs`/`remote_caps.rs` (K29) sind die
Cross-Domain-Erweiterung und bleiben davon getrennt ✅.

---

## 3. Korrektur: Root Server (globus-init)

`globus-init` ist **NICHT** „Process 0 mit Unix-Root-Rechten". Stattdessen:

```
ShivaCore → Initial Task
            ├── CSpace Root
            ├── Initial Address Space
            ├── BootInfo
            ├── Untyped Memory Caps
            ├── IRQ Caps
            └── Device Caps
                 ↓
            globus-init (erhält explizite Initial-Capabilities)
```

---

## 4. Kernel Object Model (zentraler Architekturbaustein)

```
Kernel Object
│
├── Thread          ├── Endpoint        ├── IRQ
├── AddressSpace    ├── Notification    ├── Device
├── PageTable       ├── CNode           ├── UntypedMemory
├── Frame           └── Timer
```

Capabilities referenzieren Kernel-Objekte:

```
Capability
├── Object Reference     ├── Badge
├── Rights               ├── Type
└── Derivation Metadata
```

Beispiel: `CAP #42 · Type: Endpoint · Object: VFS_ENDPOINT · Rights: SEND|RECEIVE|CALL`

---

## 5. Capability Derivation Tree (Delegation)

```
ROOT CAP
   ├── Network → NetServer → App A, App B
   └── Storage → VFS → App A, App C
```

Beispiel GPU: Root → GPU Service → Genesis → Render Worker.
Genesis erhält `GPU_FRAMEBUFFER`, `GPU_COMMAND_QUEUE`, `GPU_DMA_BUFFER` —
**keinen** Zugriff auf beliebigen physischen Speicher.

---

## 6. IPC als Bus des Betriebssystems

Globus OS kommuniziert **nicht** primär über globale Libraries, sondern über
Kernel-vermittelte IPC-Objekte:

```
Aurora AI →(IPC)→ AI Manager →(IPC)→ GPU Service →(IPC)→ GPU Driver → Hardware
```

Gleiches Modell für: App→VFS, App→Network, App→Audio, App→Camera,
App→Blockchain, App→Wallet, App→AI. Einheitliche Service-Architektur.

---

## 7. Shared Memory als Performance Plane (Control/Data Plane Split)

```
CONTROL PLANE:  IPC Message · Capability · Command · Event · Metadata
DATA PLANE:     Shared Memory · DMA · Ring Buffer · Zero-Copy · Mapping
```

Beispiel Aurora: IPC → Model Service (Kontrolle), Shared Memory → Tensor Buffer
→ GPU Service (Daten). Große Tensoren laufen **nicht** durch den IPC-Nachrichtenpfad.

---

## 8. Scheduler: Scheduling Domains (statt reinem Priority-Zahlwert)

```
HARD REALTIME · SOFT REALTIME · INTERACTIVE · SYSTEM · NORMAL · BACKGROUND
```

Zuweisungsbeispiele: Audio Engine→Soft RT, Input/Desktop→Interactive,
Blockchain Node→Normal, Aurora Training/Mining→Background.

MLFQ ist für v0.1 brauchbar; Realtime ist **keine** „höchste normale Priorität",
sondern eine eigene Domain.

*Implementierungs-Delta:* `scheduler.rs` (K29) ist DA-HEFT (Accelerator-Scheduling)
— als Forschungs-/Beschleuniger-Scheduler zu führen; v0.1-Domains sind separat
zu spezifizieren.

---

## 9. Harte Architekturregel: Keine KI-Logik im Kernel

```
SHIVACORE MUST NOT depend on: Aurora AI · DefenderGPT · A-TownChain · Genesis
                              · ATCLang compiler · GUI
```

Richtung ist AUSSCHLIESSLICH: Applications → Services → ShivaCore → Hardware.
Niemals: ShivaCore → Aurora AI → Blockchain.

Damit bleibt ShivaCore **bootstrapping-fähig und deterministisch**.

---

## 10. Security Architecture (5 Domains + DefenderGPT-Governance)

```
Capability Isolation · Memory Isolation · Process Isolation
            ↓
     Service Isolation
            ↓
   Application Sandbox
```

DefenderGPT: **beobachten und Empfehlungen geben** (Security Telemetry über
ShivaCore Events). Die **Enforcement-Entscheidung bleibt deterministisch im Kernel**.
Verbindlicher Governance-Punkt.

---

## 11. Syscall-Modell: Objektorientierte Kernel-ABI

```
TASK:       thread_create · thread_exit · thread_control
MEMORY:     map · unmap · protect · share
IPC:        send · receive · call · reply
CAPABILITY: copy · move · revoke · delete
IRQ:        bind · wait · ack
TIME:       sleep · timer_create
```

Die konkrete Syscall-ID ist sekundär. Entscheidend: **Objektorientierung der Kernel-ABI.**

---

## 12. Boot Chain (final)

```
UEFI → Limine → ShivaCore (CPU/Memory/Interrupt/Scheduler/CSpace/IPC Init)
→ globus-init (Memory/Process/Driver Manager, VFS, Network, Security, Service Manager)
→ Globus OS (Window Manager, Aurora, ATCLang, A-TownChain, Applications)
```

---

## 13. Architecture Freeze Rules (SC-ARCH-001 … SC-ARCH-010)

| Regel | Inhalt |
|---|---|
| SC-ARCH-001 | ShivaCore ist ein Capability-basierter Microkernel |
| SC-ARCH-002 | Hardware-Treiber laufen grundsätzlich außerhalb des Kernel-Adressraums |
| SC-ARCH-003 | IPC erfolgt ausschließlich über Kernel-vermittelte IPC-Objekte |
| SC-ARCH-004 | Direkter Speicherzugriff zwischen Prozessen ist verboten ohne Memory-Capability-Mapping |
| SC-ARCH-005 | Userspace Services dürfen bei Absturz neu gestartet werden, ohne Kernel-Neustart |
| SC-ARCH-006 | AI-Systeme besitzen keine impliziten Kernel-Rechte |
| SC-ARCH-007 | Blockchain-Komponenten besitzen keine impliziten Kernel-Rechte |
| SC-ARCH-008 | ATCLang besitzt keine impliziten Kernel-Rechte |
| SC-ARCH-009 | Alle privilegierten Operationen werden über Capabilities autorisiert |
| SC-ARCH-010 | Der Kernel darf nicht von Userspace-Komponenten abhängen |

---

## 14. Implementierungs-Reihenfolge (SC-001 … SC-013)

```
SC-001 Kernel Object Model → SC-002 Capability Model → SC-003 CSpace/Derivation
→ SC-004 IPC Semantics → SC-005 Memory Model → SC-006 Thread/TCB Model
→ SC-007 Scheduler → SC-008 Syscall ABI → SC-009 Boot Protocol
→ SC-010 Userspace Root Server → SC-011 Driver Model → SC-012 Globus OS Service ABI
→ SC-013 ATCLang ABI
```

Erst DANACH Anbindung von Aurora, A-TownChain und Genesis.

**Trennung:** ShivaCore = Trust Base · Globus OS = OS-Layer · ATCLang = Native Language ·
Aurora = AI Platform · A-TownChain = Blockchain Platform · Genesis = Game Platform.

Darauf baut anschließend die **ShivaCore v0.1.0 technische Spezifikation** auf:
normative MUST/SHOULD/MAY-Regeln, Datenstrukturen, ABI, Fehlercodes, Testvektoren.

---

*Verwandt: `SHIVACORE_KERNEL_ARCHITECTURE.md` (AD-012, Basierdokument) ·
`SHIVACORE_KERNEL_STATUS.md` (Ist-Stand) · `DECISIONS_REGISTER.md` (AD-013)*
