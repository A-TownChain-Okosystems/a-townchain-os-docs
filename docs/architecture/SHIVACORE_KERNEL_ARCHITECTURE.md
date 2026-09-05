# ShivaCore Kernel-Architektur — Offizielle Spezifikation (Globus OS)

> **Status:** VERBINDLICH (Architektur-Entscheidung AD-012) · **Datum:** 06.09.2026
> **Geltungsbereich:** ShivaCore Kernel als unterliegender Systemkern von Globus OS
> **Abgrenzung:** ShivaCore ist NICHT Aurora AI, NICHT der Globus Shell Layer, NICHT ATCLang.

---

## 1. Architektur-Hierarchie (System-Schichtenmodell)

```
┌─────────────────────────────────────────────────────────────┐
│                       USER / APPS                            │
├─────────────────────────────────────────────────────────────┤
│                  Globus Desktop / UI                        │
├─────────────────────────────────────────────────────────────┤
│                    Globus Shell                             │
│             CLI · GUI · ATCLang Interface                   │
├─────────────────────────────────────────────────────────────┤
│                    ATCLang Runtime                           │
├─────────────────────────────────────────────────────────────┤
│                  Globus OS Services                          │
│ Process · Network · Storage · AI · Blockchain · Security    │
├─────────────────────────────────────────────────────────────┤
│                    SHIVACORE KERNEL                          │
│                                                             │
│  Scheduler       Memory        IPC        VFS                │
│  Process Mgmt    Drivers       Security   Time               │
│  Interrupts      Hardware Abstraction    Syscalls           │
├─────────────────────────────────────────────────────────────┤
│                  Hardware Abstraction                        │
│                 CPU · GPU · RAM · I/O                       │
├─────────────────────────────────────────────────────────────┤
│                         HARDWARE                             │
└─────────────────────────────────────────────────────────────┘
```

ShivaCore ist die **unterste Software-Schicht** von Globus OS. Aurora AI, A-TownChain
und Genesis werden darauf als isolierte, aber tief integrierbare Systemplattformen betrieben.

---

## 2. Kernel-Subsysteme

| Subsystem | Aufgabe |
|---|---|
| Boot Core | Kernel-Initialisierung und Hardware-Start |
| Scheduler | CPU-Zeit, Threads und Prioritäten |
| Process Manager | Prozesse, Tasks und Lebenszyklus |
| Memory Manager | Virtual Memory, Heap, Pages, Protection |
| IPC Core | Kommunikation zwischen Prozessen/Services |
| VFS | Einheitliche Dateisystem-Abstraktion |
| Driver Manager | Hardware-Treiberverwaltung |
| HAL | Hardware-Abstraktion |
| Interrupt Manager | Hardware-/Software-Interrupts |
| Security Core | Rechte, Isolation, Capabilities |
| Time Manager | Systemzeit, Timer, Scheduling-Ticks |
| Network Core | Netzwerk-Grundfunktionen |
| Syscall Interface | definierte User→Kernel-Schnittstelle |

---

## 3. Microkernel statt monolithischem Kernel

ShivaCore wird als **Microkernel/Hybrid-Microkernel** konzipiert (AD-012).

**Im Kernel-Kontext** laufen nur sicherheitskritische und zeitkritische Primitive:

```
SHIVACORE
│
├── Scheduler
├── Memory
├── IPC
├── Interrupts
├── Capability Security
├── Process/Thread Core
├── Minimal VFS Interface
├── HAL
└── Syscall ABI
```

**Außerhalb des Kernel-Kontexts** (User/Service Space) laufen komplexe Komponenten:

```
User / Service Space
│
├── Filesystem Services
├── Network Services
├── GPU Services
├── Audio
├── USB
├── Bluetooth
├── AI Runtime
├── Blockchain Runtime
├── Container Runtime
└── ATCLang Runtime
```

**Entscheidender Vorteil:** Ein Fehler in einem Netzwerk- oder GPU-Treiber zerstört
nicht automatisch den gesamten Kernel.

---

## 4. ShivaCore + ATCLang (Syscall-Pfad)

ATCLang manipuliert **niemals direkt** Kernel-Speicher. Strenge Kette:

```
ATCLang Application → ATCLang Runtime → System API → Syscall ABI → ShivaCore Kernel
```

Beispiel `file.open()`:

```
file.open()
  → ATCLang Runtime → sys_open()
  → Capability Check → VFS
  → Filesystem Service → Storage Driver
```

Damit arbeitet ATCLang systemnah, **ohne** die Kernel-Isolation aufzugeben.

---

## 5. Capability Security

Capability-Based Security Model (statt Root-basierter Rechtevergabe):

```
Process A                    Process B
 ├── CAP_FILE_READ            ├── CAP_FILE_READ
 ├── CAP_NETWORK              └── CAP_AUDIO
 └── CAP_GPU
```

Eine Anwendung besitzt nur die Ressourcen, die ihr explizit zugewiesen wurden.
Dies ist die Grundlage für DefenderGPT / den Security-Core.

---

## 6. ShivaCore AI Integration (Kernel Events → AI Bridge)

Die KI ist **kein Bestandteil des Kernel-Cores**:

```
SHIVACORE → Kernel Events → AI Kernel Bridge
                             ├── Aurora AI
                             ├── DefenderGPT
                             └── OwnerGPT
                             → AI Service Layer
```

Einsatzfelder: Systemzustände analysieren, Prozesse überwachen, Anomalien erkennen,
Ressourcen optimieren, Fehler diagnostizieren, Sicherheitsereignisse bewerten, Logs analysieren.

**Grenze:** Die KI erhält keine uneingeschränkte Kernel-Kontrolle. Sie arbeitet
ausschließlich über definierte Kernel APIs und Capability Tokens.

---

## 7. Blockchain-Integration (System Service, nicht Kernel)

```
ShivaCore → ATC System Interface → A-TownChain Node Service
                                    ├── Consensus
                                    ├── P2P
                                    ├── Mempool
                                    ├── State
                                    ├── Smart Contracts
                                    └── Wallet
```

Der Kernel stellt nur Infrastruktur bereit: CPU, Memory, Network, Storage,
Kryptografie-Primitive, Secure RNG, Time, IPC. **Die Blockchain bleibt ein System Service.**

---

## 8. Kernel-Sicherheitszonen (Isolationsebenen)

| Level | Zone |
|---|---|
| LEVEL 0 | Hardware |
| LEVEL 1 | ShivaCore Kernel |
| LEVEL 2 | Privileged System Services |
| LEVEL 3 | User Applications / AI / Games |

Zusätzlich Sandboxes: AI Sandbox, Blockchain Sandbox, ATCLang Sandbox,
Application Sandbox, Driver Sandbox.

---

## 9. Ziel-Repository-Struktur (globus-os)

```
globus-os/
├── kernel/shivacore/     (boot/, arch/{x86_64,arm64,riscv64}, core/, scheduler/,
│                         process/, memory/, ipc/, interrupt/, syscall/,
│                         security/, capability/, time/, hal/, drivers/)
├── services/             (filesystem/, network/, graphics/, audio/, input/,
│                         device/, ai/, blockchain/)
├── runtime/              (atclang/, wasm/)
├── shell/                (atc-globus-shell/)
├── security/ · drivers/ · userspace/ · tools/ · tests/ · docs/ · build/
```

---

## 10. Technologiestack

| Bereich | Technologie |
|---|---|
| Kernel | Rust |
| Low-Level Boot | Assembly |
| Architektur | x86_64 / ARM64 / RISC-V |
| User Applications | ATCLang |
| Portable Runtime | WASM |
| System Services | Rust + ATCLang |
| IPC | Capability-based IPC |
| Graphics | Wayland-kompatible Architektur |
| AI | Aurora AI |
| Blockchain | A-TownChain |
| Security | Capability + Sandbox |
| Build | Cargo + ATCLang Toolchain |
| Testing | Rust Tests + Kernel Integration Tests |

---

## 11. Verantwortungsgrenzen des Ökosystems

```
SHIVACORE    → Hardware + Kernel primitives
GLOBUS OS    → Operating-System Services
ATCLANG      → System + Application Programming
GLOBUS SHELL → User/System Control Interface
AURORA AI    → AI Platform + AI Runtime
A-TOWNCHAIN  → Blockchain Infrastructure
GENESIS      → Game / GameFi Platform
```

---

## 12. Delta-Analyse: Ziel vs. Implementierungsstand (K29, 06.09.2026)

Ehrlicher Ist-Stand des Rust-Kernels (`a-townchain-os/src/modules/atc-shivacore/kernel/`,
674/674 Tests grün, Stable-Toolchain, Unified Cargo Workspace):

| Ziel-Subsystem (diese Spez.) | Ist-Stand K29 | Delta |
|---|---|---|
| Scheduler | ✅ `scheduler.rs` (DA-HEFT) | — |
| Process Manager | ✅ `process.rs` (PCB, States, Caps) | — |
| IPC Core | ✅ `ipc.rs` (Channel, Cap-gated) | — |
| Capability Security | ✅ `capability.rs` + `did.rs` + `remote_caps.rs` | — |
| Memory Manager | ✅ `memory_manager.rs` + `allocator.rs` (x86-boot-Feature) | Virtual Memory/Pages: offen |
| VFS (Minimal Interface) | ✅ `atcfs.rs` (content-addressed) | Voll-VFS: Service-Space |
| Time Manager | ⚠️ PoH als Zeitbasis (consensus-nah) | dediziertes Subsystem: offen |
| Syscall Interface | ❌ | **neu zu spezifizieren (ATCLang ABI)** |
| HAL | ⚠️ partiell via `x86-boot`-Feature (Boot-Teile) | dedizierte HAL-Schicht: offen |
| Interrupt Manager | ⚠️ Boot-Assembler hinter `x86-boot` | Kernel-Interrupt-Handling: offen |
| Driver Manager | ❌ | offen |
| Network Core | ✅ `atcnet.rs` (P2P-Grundfunktionen) | als Service auslagern |

**Wichtigste Ziel-Abweichung:** Blockchain-Komponenten (`blockchain.rs`, `consensus.rs`,
`genesis.rs`, `genesis_bridge.rs`, `gossip_bridge.rs`, `security_audit.rs`) liegen heute
**im Kernel-Crate** (K29-Wachstumshistorie). Gemäß dieser Spezifikation sind sie in den
Service-Space (A-TownChain Node Service, Abschnitt 7) zu migrieren — der Kernel behält nur
Primitive (Krypto, RNG, Time, IPC). Die Migration ist als eigener Architektur-Sprint zu
planen; die Kernel-API bleibt kompatibel via `KernelState`/`cross_subsystem`-Harness.

---

## 13. Nächster Architekturbaustein

**ShivaCore v0.1 Kernel-Spezifikation**, in dieser Reihenfolge:
Boot-Prozess → Memory Model → Scheduler → IPC → Syscalls → Capability Model →
Driver Model → ATCLang ABI.

---

*Verwandt: `docs/SHIVACORE_KERNEL_STATUS.md` (Implementierungs-Stand) ·
`docs/architecture/SHIVAOS_KERNEL.md` (Prozess-Layer-Konzept, AD-008) ·
`docs/DECISIONS_REGISTER.md` (AD-012)*
