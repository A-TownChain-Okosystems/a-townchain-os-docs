# ShivaCore Kernel — Status & Agent-Handoff

> **Datum:** 07.07.2026 17:48 | **Autor:** Agent `aurora-base44-superagent-69c1e0c577ccf6c45a27a480`
> **Verifiziert durch tatsaechliche Test-Ausfuehrung, nicht nur Code-Lesen.**

## ⚠️ Wichtigster Fund: ZWEI separate Kernel-Implementierungen existieren

| | `core/kernel.py` | `shivaos/kernel/kernel.py` |
|---|---|---|
| **Ansatz** | EventBus + ModuleLoader | `ShivaKernel`-Klasse: Prozess-/Speicher-/IPC-Management |
| **Kernkonzepte** | Event-getriebene Architektur | ProcessType/ProcessState, MemRegion, Channel/IPC, Gas-Accounting |
| **Tests vorhanden** | ✅ `tests/test_kernel.py` (16 Tests) | ❌ Kein Testfile gefunden |
| **Test-Ergebnis (heute verifiziert)** | **16/16 PASSED** (0.74s) | Nicht getestet — nur Import verifiziert (OK, keine Syntaxfehler) |
| **Reifegrad** | Hoeher — durch Tests abgesichert | Umfangreicher (381 Zeilen), aber unverifiziert |

## Offene Frage fuer naechsten Agenten/Entwickler

Sind das zwei **konkurrierende** Implementierungen (nur eine sollte ueberleben)
oder zwei **komplementaere** Schichten (z.B. `core/kernel.py` als
Event-Bus-Basis, `shivaos/kernel/kernel.py` als Prozess-Layer darueber)?
Das ist aktuell nicht dokumentiert und sollte vor der K3-Migration
(`src/core/` Konsolidierung, siehe `AGENT_POLICY.md`) geklaert werden.

## Naechste sinnvolle Schritte (nicht automatisch gestartet)

1. Klaeren: Konkurrenz oder Komplementaer? → Entscheidung ins `DECISIONS_REGISTER.md`
2. Falls komplementaer: Beziehung in `ARCHITECTURE.md`/`ATC_ECOSYSTEM_STANDARDS.md` dokumentieren
3. Falls konkurrierend: eine Implementierung als Referenz waehlen, die andere archivieren
4. Fuer `shivaos/kernel/kernel.py`: Testfile analog zu `tests/test_kernel.py` erstellen (aktuell 0% Testabdeckung)

## Reality-Check (gemaess AGENT_POLICY.md)

Diese Datei basiert auf tatsaechlicher Code-Ausfuehrung heute (07.07.2026):
`python3 -m pytest tests/test_kernel.py` → 16 passed. Import-Test fuer
`shivaos/kernel/kernel.py` erfolgreich, aber kein Test-Run moeglich (kein
Testfile vorhanden). Keine Behauptung ohne Ausfuehrungs-Beleg.

*Erstellt von Agent `aurora-base44-superagent-69c1e0c577ccf6c45a27a480`.*


---

## ✅ HANDOFF ABGESCHLOSSEN (07.07.2026 17:54)

**Entscheidung getroffen:** siehe `DECISIONS_REGISTER.md` AD-008.
`core/kernel.py` (EventBus) = Fundament-Schicht, getestet (16/16).
`shivaos/kernel/kernel.py` (ShivaKernel) = Prozess-Layer darueber,
komplementaer, kein Konflikt. Keine Datei geloescht oder archiviert.

**Einzige offene Folgeaufgabe:** Testfile fuer `shivaos/kernel/kernel.py`
— separates Ticket, nicht Teil dieses Handoffs.

**Status: RESOLVED.**


---

## 📚 Forschungs-Notiz: Ideenpool "moderner dezentraler KI-OS-Kernel" (08.07.2026)

> Externe Recherche/Konzept-Zusammenfassung, keine ShivaCore-spezifische
> Spezifikation. Hier nur als **Gap-Analyse gegen den echten Kernel-Stand**
> archiviert, nicht als neue Anforderung.

### Vorgeschlagene Konzepte vs. tatsaechlicher ShivaCore-Stand

| Konzept aus der Recherche | Im echten ShivaCore-Kernel vorhanden? |
|---|---|
| Capability-basierte Zugriffskontrolle | ❌ Nein — aktuell einfache owner/stake-Felder, keine delegierbaren Capabilities |
| Mikrokernel-Trennung (Speicher/Scheduling/IPC privilegiert, Rest Userspace) | ⚠️ Teilweise — `ShivaKernel` buendelt Prozess+Speicher+IPC in einer Klasse, keine harte Trennung |
| TEE-Unterstuetzung (SGX/SEV/TrustZone) | ❌ Nein |
| DIDs / dezentrale dentitaet pro Knoten | ⚠️ ATC-Adressen existieren (`owner`-Feld), aber kein W3C-DID-Modell (siehe ATC-03, Status PARTIAL) |
| Heterogenes Scheduling (CPU/GPU/NPU-Klassen) | ❌ Nein — einfache Prioritaets-Zahl (0-255), kein Ressourcen-Klassen-Modell |
| Byzantinische Fehlertoleranz im Kernel | ❌ Nein — BFT lebt in ShivaConsensus (separate Schicht), nicht im Kernel |
| Formale Verifikation (seL4-Stil) | ❌ Nein — Python-Implementierung, nicht formal verifizierbar wie C/Rust+Beweise |

### Einordnung

Die Recherche beschreibt ein **Forschungsniveau-System** (vergleichbar mit
seL4/Zircon + Web3-Identitaet + KI-Orchestrierung) — deutlich groesser als
der aktuelle ShivaCore-Kernel (381 Zeilen Python, EventBus + Prozess-
verwaltung). Kein Widerspruch zu AD-008 (komplementaere Schichten), aber
ein moeglicher **langfristiger Nordstern** fuer die Kernel-Weiterentwicklung.

### Konkreter naechster Schritt (falls gewuenscht, nicht automatisch gestartet)

Von den 7 Konzepten waere **Capability-basierte Zugriffskontrolle** der
sinnvollste erste Baustein — baut direkt auf dem bestehenden `owner`/
`stake`-Feld in `KernelProcess` auf, ohne Kernarchitektur-Bruch, und ist in
begrenztem Umfang tatsaechlich implementierbar (im Gegensatz zu TEEs/seL4-
Verifikation, die eigene Hardware/Toolchains brauchen).

*Archiviert von Agent `aurora-base44-superagent-69c1e0c577ccf6c45a27a480`.*


---

## ✅ Milestone: Capability-System implementiert (08.07.2026, ECHTER CODE)

> Reagiert auf die Layer-1/Layer-4-Konzepte aus der Kernel-Architektur-
> Recherche vom 08.07.2026 (siehe Forschungs-Notiz oben) — aber als
> **tatsaechlich lauffaehiger, getesteter Code**, nicht als weitere Spec.

**Datei:** `shivaos/kernel/capabilities.py` (159 Zeilen) +
`shivaos/tests/test_capabilities.py` (10 Tests, **alle gruen**, verifiziert
vor dem Push: `python3 -m pytest shivaos/tests/test_capabilities.py` →
`10 passed in 0.04s`).

**Was es tut:** `Capability` (unveraenderliches Zugriffs-Ticket:
resource_type + resource_id + Rights-Flags READ/WRITE/EXECUTE/DELEGATE) +
`CapabilityManager` (grant/check/require/delegate/revoke). Delegation kann
Rechte nur einschraenken, nie erweitern. Revoke kaskadiert auf alle
delegierten Kind-Capabilities.

**Bewusst NICHT enthalten:** noch keine Integration in `ShivaKernel.alloc()`
/`create_channel()` selbst (d.h. der bestehende Kernel-Code erzwingt Capability-
Pruefung noch nicht automatisch) — das waere der naechste Schritt, kein
Big-Bang. TEEs, DIDs, heterogenes Scheduling bleiben weiterhin offene
Forschungsideen, nicht Teil dieses Commits.

**Naechster moeglicher Schritt (nicht automatisch gestartet):** `alloc()`
und `create_channel()` in `shivaos/kernel/kernel.py` so anpassen, dass sie
eine gueltige Capability verlangen statt direkten Zugriff zu erlauben.

*Implementiert von Agent `aurora-base44-superagent-69c1e0c577ccf6c45a27a480`.*


---

## ✅ Milestone: Capability-Durchsetzung IM Kernel integriert (08.07.2026)

> Reaktion auf "AethelKernel"-Vorschlag: kein neuer Markenname, kein Rust/
> seL4-Rewrite (Sandbox-Grenzen) — aber der konkrete Kern der Idee
> (Capability-Layer) ist jetzt tatsaechlich TEIL des laufenden
> `shivaos/kernel/kernel.py`, nicht nur ein separates Modul daneben.

**Aenderungen (additiv, nichts Bestehendes gebrochen):**
- `alloc()` vergibt automatisch eine Capability (ALL rights) an den Prozess
- `create_channel()` vergibt automatisch eine Capability fuer den Kanal
- `free()` widerruft die zugehoerige Capability
- Neu: `read_memory()` / `write_memory()` / `send_with_capability()` /
  `recv_with_capability()` — pruefen die Capability, werfen `CapabilityError`
  bei fehlendem/falschem Zugriff

**Verifiziert vor Push:** `pytest shivaos/tests/test_kernel_capabilities.py`
→ **7/7 passed**. Regressionscheck: bestehende Tests (`test_kernel.py` +
`test_capabilities.py`) weiterhin **26/26 passed** — nichts kaputt gemacht.

**Bewusst nicht gemacht:** die alten `channel_send`/`channel_recv`/direkter
`region.read/write` bleiben ungated bestehen (Rueckwaertskompatibilitaet).
Ob sie irgendwann komplett durch die gated Varianten ersetzt werden, ist
eine offene Folgeentscheidung, kein automatischer naechster Schritt.

*Implementiert von Agent `aurora-base44-superagent-69c1e0c577ccf6c45a27a480`.*


---

## ✅ Milestone: DA-HEFT Scheduler mit Hardware-Interface (08.07.2026)

> Antwort auf den Scheduler-Vorschlag (Layer 3): echte, getestete Python-
> Implementierung des DA-HEFT-Algorithmus (Deadline-Aware Heterogeneous
> Earliest-Finish-Time) -- **hardware-agnostisch designed**, damit spaeter
> echte GPU/NPU-Anbindung moeglich ist, ohne den Algorithmus anzufassen.

**Architektur (Dependency Inversion, bewusst):**
- `accelerator.py`: `Accelerator` (ABC-Interface) + `SimulatedAccelerator`
  (Referenzimplementierung fuer diese Sandbox -- ehrlich als Simulation
  gekennzeichnet, KEIN Anspruch auf echte Hardware)
- `daheft.py`: `DAHEFTScheduler` kennt NUR das `Accelerator`-Interface,
  nie eine konkrete Implementierung

**Was das fuer "spaeter echte Hardware" bedeutet:** Jemand mit echter
GPU/NPU muss nur EINE neue Klasse schreiben, die `Accelerator` implementiert
(z.B. `OnnxRuntimeAccelerator`, `CudaAccelerator`) -- `daheft.py` bleibt
unveraendert. Das ist der uebliche, ehrliche Weg zu "hardware-ready" ohne
heute etwas vorzutaeuschen, das nicht existiert (kein GPU/NPU in dieser
Sandbox).

**Algorithmus:** kritischer-Pfad-Priorisierung (upward rank) → EFT-Berechnung
je Beschleuniger → Deadline-Admission-Control (mit Degradation bei
unmoeglicher Deadline statt Absturz) → thermische Verfuegbarkeitspruefung →
Auswahl nach minimalem EFT×Energie-Produkt.

**Verifiziert vor Push:** `pytest shivaos/tests/test_daheft_scheduler.py`
→ **8/8 passed**, inklusive Nachbau des Autonomous-Driving-Beispiels
(Conv2D→ReLU→MatMul, 5ms Deadline, GPU ueberhitzt bei 85°C/80°C-Limit →
NPU uebernimmt komplette Kette, Deadline eingehalten). Regressionscheck:
**41/41 Tests gesamt gruen** (alle bisherigen Kernel-Tests unveraendert).

*Implementiert von Agent `aurora-base44-superagent-69c1e0c577ccf6c45a27a480`.*


---

## ✅ Milestone: DID + Remote-Capability-Tickets, ECHTE Kryptografie (08.07.2026)

> Antwort auf Layer 4 (Remote-Capability-Delegation): im Unterschied zum
> Scheduler (Hardware-abhaengig, nur simulierbar) ist dieser Teil OHNE
> Hardware vollstaendig echt umsetzbar -- Signaturen, Replay-Schutz,
> Delegationsketten sind reale, funktionierende Kryptografie/Logik.

**`shivaos/kernel/did.py`:** `NodeIdentity` mit echtem Ed25519-Schluesselpaar
(`cryptography`-Bibliothek), `sign()`/`verify()` funktionieren kryptografisch
korrekt. **Ehrliche Einschraenkung:** privater Schluessel liegt im
Prozessspeicher, NICHT in einer Hardware-Enklave (TrustZone/SGX gibt es
hier nicht) -- das wird nicht vorgetaeuscht.

**`shivaos/kernel/remote_capability.py`:** `RemoteCapabilityTicket`
(signiert, mit Nonce), `RemoteCapabilityResolver` (Signaturpruefung,
Subject-Check, Replay-Schutz via `NonceStore`, Deadline-Check),
`resolve_chain()` fuer mehrstufige Delegation (Bob→Charlie→Alice) mit
Attenuation-Pruefung (Rechte/Operationen/Deadline duerfen pro Kettenglied
nur EINGESCHRAENKT werden).

**Verifiziert vor Push:** `pytest shivaos/tests/test_remote_capability.py`
→ **9/9 passed**, inklusive: manipuliertes Ticket abgelehnt, Replay-Angriff
abgelehnt, falscher Subject abgelehnt, Rechte-Erweiterung in Kette
abgelehnt, abgelaufenes Ticket abgelehnt. Regressionscheck: **50/50 Tests
gesamt gruen**.

**Bewusst NICHT enthalten:** Hardware-Enklaven, DHT-Netzwerktransport
(wie das Ticket physisch zum Zielknoten kommt, ist wie in der Spezifikation
richtig beschrieben bewusst außerhalb des Kernels), formale TLA+/Coq-
Verifikation -- alles offene Forschungsideen, nicht Teil dieses Commits.

*Implementiert von Agent `aurora-base44-superagent-69c1e0c577ccf6c45a27a480`.*


---

## ✅ Milestone: Capability-System in Rust (03.08.2026)

> Erster Rust-Code-Beitrag zum ShivaCore Kernel: Port des Python-
> Capability-Systems nach Rust. Die gleiche Logik (Erzeugung, Delegation
> mit Attenuation, kaskadierender Widerruf) — jetzt im Kernel nativ.

**`atc-shivacore/kernel/src/capability.rs`** (neu, 324 Zeilen):
- `Rights` als Bitfield (READ/WRITE/EXEC/DELEGATE), `BitOr`/`BitAnd` impl
- `Capability` struct (id, resource_type, resource_id, rights, owner, parent)
- `CapabilityTable` mit Spinlock-geschuetzter BTreeMap
- `create()` — Kernel erzeugt Capability
- `delegate()` — Attenuation-Pruefung (Rechte-Monotonie), Owner-Check, DELEGATE-Check
- `check()` — der Kern-Check fuer jeden geschuetzten Syscall
- `revoke()` — kaskadierender Widerruf (alle abgeleiteten Capabilities)

**Verifiziert:** `cargo test` mit Rust 1.97 → **8/8 passed**
(create_and_check, delegate_attenuation, rejects_rights_expansion,
requires_delegate_right, wrong_owner, revoke_cascade, list_for_process,
rights_operations). Die Tests sind im selben File als `#[cfg(test)]`-Modul.

**In main.rs registriert:** `mod capability;` hinzugefuegt.

*Implementiert von Agent `aurora-base44-superagent-69c1e0c577ccf6c45a27a480`.*


---

## ✅ Milestone: ProcessManager in Rust (03.08.2026)

> K-Sprint 3b: Prozessverwaltung mit integrierter Capability-Durchsetzung.
> Implementiert den `ProcessManager`-Trait aus `ats1000.rs`.

**`atc-shivacore/kernel/src/process.rs`** (neu, ~350 Zeilen):
- `ProcessControlBlock` (PCB) — PID, ProzessType, Prioritaet (0-255, ATC-0008), Zustand, Parent/Children
- `ProcessState` — Ready / Running / Blocked / Terminated(ExitCode)
- `spawn()` — erzeugt Prozess + **automatische Memory-Capability** (READ/WRITE/EXEC/DELEGATE) fuer eigenen Adressraum
- `spawn_child()` — Kind-Prozess mit Parent-Verknuepfung
- `kill()` — **kaskadierender Capability-Widerruf** (alle Caps des Prozesses + alle davon abgeleiteten) + Zustand→Terminated + Entfernung aus Parent-Children-Liste
- `wait()` — Exit-Code-Abfrage
- Zustandsautomaten: Ready↔Running (Schedule/Preempt), →Blocked→Ready (IPC/IO-Wait/Wakeup)
- `check_capability()` / `delegate_capability()` — Direktzugriff auf Capability-Tabelle

**Integration mit Capability-System:** Jeder Prozess bekommt beim Spawn automatisch eine Memory-Cap fuer seinen Adressraum. kill() ruft `CapabilityTable::revoke()` fuer alle Caps auf — der Prozess verliert sofort alle Ressourcen, inkl. Caps die er an andere delegiert hat (kaskadierend).

**Verifiziert:** `cargo test` mit Rust 1.97 → **18/18 passed** (8 Capability + 10 Process Tests). Tests decken: Spawn+Cap-Erzeugung, Mehrfach-Spawn, Kill+Cap-Widerruf+kaskadierend, Double-Kill-Reject, Parent-Child-Verknuepfung, Kill-entfernt-aus-Parent, Zustandsuebergaenge, Active-Count, Prioritaetserhaltung.

*Implementiert von Agent `aurora-base44-superagent-69c1e0c577ccf6c45a27a480`.*


---

## ✅ Milestone: DA-HEFT Scheduler in Rust (03.08.2026)

> K-Sprint 4: Port des Python DA-HEFT-Schedulers nach Rust.
> Hardware-agnostisch, deadline-aware, thermisch gesichert.

**`atc-shivacore/kernel/src/scheduler.rs`** (neu, ~280 Zeilen):
- `Accelerator` Trait — Hardware-Abstraktion fuer CPU/GPU/NPU/TPU; echte Hardware spaeter ohne Algorithmus-Aenderung
- `SimulatedAccelerator` — simulierte Beschleuniger fuer Tests
- `Task` struct — compute_flops, memory_mb, deadline, dependencies, PID
- `compute_upward_ranks()` — iterativ aus Successor-Map (korrekter HEFT-Rank, Entry-Tasks hoechste Prioritaet)
- `schedule()` — sortiert nach upward-rank (absteigend), weist fruehesten Finish-Time zu
- `deadline_misses()` / `utilization()` — Statistik-Funktionen
- Thermisches Throttling (ueberspringt Beschleuniger >85°C)
- Speicher-Constraint (ueberspringt Beschleuniger mit zu wenig VRAM)

**Verifiziert:** `cargo test` mit Rust 1.97 → **28/28 passed** (8 Capability + 10 Process + 10 Scheduler). Tests decken: Basis-Scheduling, heterogene Zuweisung, Abhaengigkeits-Ordering, Deadline-Awareness, thermisches Throttling, Speicher-Constraint, upward-rank Prioritaet, leere Eingaben, Auslastung, Total-Overload.

*Implementiert von Agent `aurora-base44-superagent-69c1e0c577ccf6c45a27a480`.*
