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


---

## ✅ Milestone: IPC Subsystem in Rust (03.08.2026)

> K-Sprint 5: Channel-basierte Inter-Process Communication mit Capability-Durchsetzung.

**`atc-shivacore/kernel/src/ipc.rs`** (neu, ~230 Zeilen):
- `IpcSubsystem` — verwaltet alle Channels, erzeugt Channel mit auto-Caps
- `Channel` struct — owner, sender_cap, recv_cap, FIFO-Buffer, capacity, closed-Flag
- `Message` struct — sender PID, data (Vec\<u8\>), timestamp
- `create_channel()` — Owner bekommt automatisch WRITE+READ+DELEGATE Capabilities fuer den Channel
- `send()` — prueft WRITE-Capability, Channel nicht geschlossen/voll
- `recv()` — prueft READ-Capability, Channel nicht leer, FIFO-Entfernung
- `grant_access()` — delegiert Channel-Rechte an andere Prozesse (Attenuation, Owner-Check)
- `close_channel()` — schliesst Channel + kaskadierender Capability-Widerruf (alle Caps fuer diese Resource)
- `close_all_for()` — schliesst alle Channels eines Prozesses (wird von ProcessManager::kill() aufgerufen)
- `IpcError` — ChannelNotFound, ChannelClosed, ChannelFull, ChannelEmpty, NoWriteCapability, NoReadCapability

**Integration:** IPC nutzt die `CapabilityTable` direkt — jede send/recv-Operation ruft `caps.check()` auf. Prozesse ohne die entsprechende Capability werden abgewiesen. `close_all_for()` wird beim kill() eines Prozesses aufgerufen.

**Verifiziert:** `cargo test` mit Rust 1.97 → **40/40 passed** (8 Capability + 10 Process + 10 Scheduler + 12 IPC). Tests decken: Channel-Erzeugung+send/recv, send-ohne-WRITE-rejected, recv-ohne-READ-rejected, Cross-Process-Kommunikation via grant_access, Channel-full, Empty-Channel, close, non-Owner-close-rejected, close_all_for, wrong-owner-grant-rejected, FIFO-Order, pending-messages-count.

*Implementiert von Agent `aurora-base44-superagent-69c1e0c577ccf6c45a27a480`.*


---

## ✅ Milestone: IPC Subsystem in Rust (03.08.2026)

> K-Sprint 5: Channel-basierte Inter-Process Communication mit Capability-Durchsetzung.

**`atc-shivacore/kernel/src/ipc.rs`** (neu, ~230 Zeilen):
- `IpcSubsystem` — verwaltet alle Channels, erzeugt Channel mit auto-Caps
- `Channel` struct — owner, sender_cap, recv_cap, FIFO-Buffer, capacity, closed-Flag
- `Message` struct — sender PID, data (Vec\<u8\>), timestamp
- `create_channel()` — Owner bekommt automatisch WRITE+READ+DELEGATE Capabilities fuer den Channel
- `send()` — prueft WRITE-Capability, Channel nicht geschlossen/voll
- `recv()` — prueft READ-Capability, Channel nicht leer, FIFO-Entfernung
- `grant_access()` — delegiert Channel-Rechte an andere Prozesse (Attenuation, Owner-Check)
- `close_channel()` — schliesst Channel + kaskadierender Capability-Widerruf (alle Caps fuer diese Resource)
- `close_all_for()` — schliesst alle Channels eines Prozesses (wird von ProcessManager::kill() aufgerufen)
- `IpcError` — ChannelNotFound, ChannelClosed, ChannelFull, ChannelEmpty, NoWriteCapability, NoReadCapability

**Integration:** IPC nutzt die `CapabilityTable` direkt — jede send/recv-Operation ruft `caps.check()` auf. Prozesse ohne die entsprechende Capability werden abgewiesen. `close_all_for()` wird beim kill() eines Prozesses aufgerufen.

**Verifiziert:** `cargo test` mit Rust 1.97 → **40/40 passed** (8 Capability + 10 Process + 10 Scheduler + 12 IPC). Tests decken: Channel-Erzeugung+send/recv, send-ohne-WRITE-rejected, recv-ohne-READ-rejected, Cross-Process-Kommunikation via grant_access, Channel-full, Empty-Channel, close, non-Owner-close-rejected, close_all_for, wrong-owner-grant-rejected, FIFO-Order, pending-messages-count.

*Implementiert von Agent `aurora-base44-superagent-69c1e0c577ccf6c45a27a480`.*


---

## ✅ Milestone: IPC Capability-Gating Tests + Security-Fix (03.08.2026)

> 10 neue Edge-Case Tests fuer Capability-Gating in IPC + recv() Security-Fix.

**Neue Tests (10):**
1. `test_revoked_cap_blocks_send` — Capability widerrufen → senden blockiert
2. `test_revoked_cap_blocks_recv` — READ-Cap widerrufen → empfangen blockiert
3. `test_attenuated_cap_cannot_exceed_original` — Nur READ delegiert → kein WRITE moeglich
4. `test_delegation_chain_capability_gating` — Alice→Bob→Charlie Delegationskette, Charlie kann nicht weiter delegieren
5. `test_close_channel_revokes_all_delegated_caps` — Channel schliessen widerruft alle delegierten Caps
6. `test_isolated_channels_capability_gating` — Prozess kann nur auf eigene Channels zugreifen
7. `test_grant_then_revoke_blocks_access` — Grant → Revoke → Zugriff blockiert
8. `test_cross_channel_capability_isolation` — Cap fuer Channel A gibt keinen Zugriff auf Channel B
9. `test_send_to_nonexistent_channel` — Senden auf nichtexistenten Channel
10. `test_recv_from_nonexistent_channel` — Empfangen von nichtexistentem Channel

**Security-Fix:**
`recv()` prueft nun Capability VOR der Buffer-Inspektion. Vorher wurde `ChannelEmpty` preisgegeben, bevor die Capability geprueft wurde — ein unbefugter Prozess koennte so ermitteln, ob ein Channel Nachrichten enthaelt. Jetzt wird `NoReadCapability` zurueckgegeben, bevor Buffer-Informationen preisgegeben werden.

**Verifiziert:** `cargo test` mit Rust 1.97 → **50/50 passed** (8 Capability + 10 Process + 10 Scheduler + 22 IPC).

*Implementiert von Agent `aurora-base44-superagent-69c1e0c577ccf6c45a27a480`.*


---

## ✅ Milestone: DID + Remote-Capability-Tickets in Rust (03.08.2026)

> K-Sprint 6: Port von did.py + remote_capability.py nach Rust.
> Dezentrale Identitaet und kryptographisch signierte Capability-Delegation ueber Knotengrenzen.

**`atc-shivacore/kernel/src/did.rs`** (neu, ~130 Zeilen):
- `Did` — Dezentrale Identitaet im Format `did:shivacore:<hex-public-key>`
- `CryptoProvider` Trait — abstrahiertes Krypto-Interface (wie `Accelerator` Trait im Scheduler)
- `SoftwareSigner` — deterministischer Software-Signer fuer Tests; spaeter austauschbar gegen Ed25519Signer oder HardwareEnclaveSigner
- `sign()` / `verify()` — trait-basiert, Implementierung austauschbar

**`atc-shivacore/kernel/src/remote_caps.rs`** (neu, ~340 Zeilen):
- `RemoteCapabilityTicket` — issuer_did, subject_did, resource, constraints, nonce, signature, parent_ticket_nonce
- `ResourceDescriptor` — resource_type, resource_id, rights (nutzt kernel-eigenes `Rights` Bitfield)
- `Constraints` — max_operations, deadline_unix, energy_budget_uj
- `issue_ticket()` — Issuer signiert Ticket mit `CryptoProvider::sign()`
- `signing_payload()` — deterministische Byte-Repraesentation (sortiert, kanonisch)
- `LocalCap` — lokale Capability nach Einloesung, mit `consume_operation()` Zaehler und Auto-Widerruf
- `NonceStore` — Replay-Schutz (BTreeSet, exakt statt Bloom-Filter)
- `RemoteCapabilityResolver::resolve()` — validiert Signatur → Subject → Replay → Deadline → Constraints
- `RemoteCapabilityResolver::resolve_chain()` — mehrstufige Delegation (Alice→Bob→Charlie→Dave), prueft Kettenintegritaet (Nonce-Verkettung), Ressource-Konsistenz, Rechte-Attenuation, max_operations-Monotonie, Deadline-Monotonie

**Integration:** RCT nutzt `Rights` aus `capability.rs` und `CryptoProvider` aus `did.rs`. Die Trait-basierte Krypto-Abstraktion erlaubt spaeter den nahtlosen Wechsel zu echter Ed25519-Implementierung oder Hardware-Enklaven — die Algorithmus-Logik (Signaturpruefung, Replay-Schutz, Attenuation) bleibt unveraendert.

**Verifiziert:** `cargo test` mit Rust 1.97 → **72/72 passed** (8 Capability + 10 Process + 10 Scheduler + 22 IPC + 6 DID + 16 RCT). Tests decken: Ticket-Ausstellung+Einloesung, InvalidSignature (Forge-Versuch), WrongSubject, Replay-Schutz, Expired, ConstraintsTooStrict, LocalCap-Verbrauch (3 Ops dann Widerruf), Alice→Bob→Charlie Delegationskette, Kettenbruch (falscher Nonce), Rechte-Erweiterung abgewiesen, Ops-Erweiterung abgewiesen, Deadline-Erweiterung abgewiesen, Ressource-Mismatch, leere Kette, deterministische Payload, 3-Hop-Kette (Alice→Bob→Charlie→Dave).

*Implementiert von Agent `aurora-base44-superagent-69c1e0c577ccf6c45a27a480`.*


---

## ✅ Milestone: Ed25519-Signaturen in Rust (03.08.2026)

> K-Sprint 6b: Echte Ed25519-Kryptografie mit ed25519-dalek.

**`atc-shivacore/kernel/src/did.rs`** (erweitert, ~210 Zeilen):

Zwei Implementierungen des `CryptoProvider`-Traits:

1. **SoftwareSigner** — deterministische Pseudo-Signatur (XOR-basiert) fuer reproduzierbare Logik-Tests. `did:shivacore:<hex-key>` Format.

2. **Ed25519Signer** — echte Ed25519-Signaturen mit der `ed25519-dalek` crate:
   - `new()` — erzeugt frisches Schluesselpaar mit `OsRng` (kryptographisch sicherer Zufall)
   - `from_seed(&[u8; 32])` — deterministische Erzeugung aus Seed (fuer Tests)
   - `did:shivacore:ed25519:<hex-pubkey>` Format (32-byte Public Key = 64 hex chars)
   - `sign()` — erzeugt echte 64-Byte Ed25519-Signatur
   - `verify()` — extrahiert Public Key aus DID, konstruiert `VerifyingKey`, verifiziert mit `verify()`
   - Abweisung bei: falscher DID, manipuliertem Payload, manipulierter Signatur, zu kurzer Signatur

**Abhaengigkeiten:** `ed25519-dalek = "2.1"`, `rand = "0.8"` hinzugefuegt.

**Integration:** Beide Signer implementieren dasselbe `CryptoProvider`-Trait. Das RCT-System (`remote_caps.rs`) nutzt `CryptoProvider::sign()` / `CryptoProvider::verify()` — der Wechsel zwischen SoftwareSigner und Ed25519Signer erfordert keine Aenderung an der RCT-Logik. In Produktion kann spaeter ein `HardwareEnclaveSigner` (TrustZone/SGX/Secure Element) dieselbe Schnittstelle implementieren.

**Verifiziert:** `cargo test` mit Rust 1.97 → **81/81 passed** (8 Capability + 10 Process + 10 Scheduler + 22 IPC + 15 DID + 16 RCT). Neue Ed25519-Tests: DID-Format (64 hex chars), sign+verify, wrong-signer rejected, tampered-payload rejected, tampered-signature rejected, short-signature rejected, deterministic-from-seed (gleicher Seed → gleiche DID+Signatur), cross-verify-with-RCT-Szenario, 10KB-large-payload.

*Implementiert von Agent `aurora-base44-superagent-69c1e0c577ccf6c45a27a480`.*


---

## ✅ Milestone: Knowledge Graph in Rust (03.08.2026)

> K-Sprint 7: Nativer Triple-Store fuer strukturiertes Kernel-Wissen mit Capability-gated Zugriff.

**`atc-shivacore/kernel/src/knowledge_graph.rs`** (neu, ~380 Zeilen):

**Datenstruktur:**
- `Entity` — eindeutige EntityId, Label, entity_type, created_by (Pid), triples_count
- `ObjectValue` — Enum: Entity(EntityId), Integer(i64), String, Bytes, Boolean
- `Triple` — (subject: EntityId, predicate: Predicate, object: ObjectValue)
- `KnowledgeGraph` — BTreeMap< EntityId, Entity > + Vec< Triple > + drei Indices

**Indices (fuer schnelle Lookups):**
- `spo_index`: Subject → [Triple-Indices] (fuer outgoing())
- `osp_index`: Object-Entity → [Triple-Indices] (fuer incoming() / Rueckwaerts-Lookup)
- `pso_index`: Predicate → [Triple-Indices] (fuer Praedikat-basierte Queries)

**Capability-Integration:**
- `create_entity()` — vergibt automatisch READ+WRITE+DELEGATE Capability an den Creator
- `add_triple()` — prueft WRITE-Capability auf Subject-Entity
- `query()` — filtert Ergebnisse nach READ-Capability (unsichtbar ohne Cap)
- `remove_triple()` / `delete_entity()` — pruefen WRITE-Capability
- `grant_read()` — delegiert READ an andere Prozesse (Attenuation via CapabilityTable)
- `delete_entity()` — widerruft alle Capabilities fuer die Entity

**Query-Engine:**
- `QueryPattern` — Option-Felder: None = Wildcard (Match-All)
- `query()` — filtert nach Subject, Predicate, Object (beliebige Kombination)
- `outgoing(entity)` — alle Tripel mit entity als Subject
- `incoming(entity)` — alle Tripel mit entity als Object (Rueckwaerts-Lookup)
- `transitive_closure(start, predicate, max_depth)` — BFS ueber Praedikat-Kanten, mit Zyklus-Schutz (visited-Set), max_depth-Limit

**Verifiziert:** `cargo test` mit Rust 1.97 → **99/99 passed** (8 Capability + 10 Process + 10 Scheduler + 22 IPC + 15 DID + 16 RCT + 18 KG). Tests decken: Entity-Erzeugung, Triple+Query, Write-Cap-Reject, Read-Cap-Filter, grant_read, outgoing/incoming, transitive_closure (3-Hop, max_depth, Zyklus), Literal-Werte (String/Int/Bool), remove_triple, delete_entity, Wildcard-Query, EntityNotFound, Cross-Process-Isolation, Triple-Counter.

*Implementiert von Agent `aurora-base44-superagent-69c1e0c577ccf6c45a27a480`.*


---

## ✅ Milestone: MemoryManager + ATCFS in Rust (04.08.2026)

> K-Sprint 8: ats1000 Traits MemoryManager + FileSystem implementiert.

**`atc-shivacore/kernel/src/memory_manager.rs`** (neu, ~280 Zeilen):
- `KernelMemoryManager` — implementiert `ats1000::MemoryManager`-Trait
- Bump-Allocator mit 4KB-Alignment (Userspace-Simulation, 100 MiB Limit)
- `allocate()` — vergibt automatisch READ+WRITE+EXEC+DELEGATE Capability
- `deallocate()` — prueft WRITE-Cap, widerruft alle Capabilities
- `read_check()` — prueft READ-Cap vor Speicherzugriff
- `stats()` — total_allocated, peak_allocated, active_regions
- ats1000 Trait: `alloc()`, `free()`, `mmap()` implementiert

**`atc-shivacore/kernel/src/atcfs.rs`** (neu, ~420 Zeilen):
- `AtcFileSystem` — implementiert `ats1000::FileSystem`-Trait
- Content-Adressierung: `atc1` + SHA3-256(`atcfs_v1||` + data)
- `write_file()` / `read_file()` / `ls()` / `delete_file()` / `create_dir()`
- Owner-basierte Zugriffskontrolle: oeffentliche Pfade `/atc/` und `/tmp/`, Rest owner-only
- `export_manifest()` — root_hash + file_count fuer On-Chain-Anchoring
- ats1000 Trait: `open()`, `read()`, `write()`, `close()` mit File-Handles + Offset-Tracking

**ats1000 Trait Status:**
| Trait | Status | Implementierung |
|-------|--------|-----------------|
| ProcessManager | DONE | process.rs (K3b) |
| MemoryManager | DONE | memory_manager.rs (K8) |
| FileSystem | DONE | atcfs.rs (K8) |
| NetworkStack | STUB | (K7 ATCNet, offen) |

**Verifiziert:** cargo test -> 133/133 passed (8 Cap + 10 Proc + 10 Sched + 22 IPC + 15 DID + 16 RCT + 18 KG + 12 MemMgr + 22 ATCFS).
