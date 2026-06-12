# 🔴 A-TownChain — Offene Entscheidungen

> **Gepflegt von:** MasterBrain (Aurora Ecosystem Brain)
> **Stand:** 2026-06-12

Diese Entscheidungen können NICHT automatisch getroffen werden.
**Michael Wroblewski muss diese priorisieren.**

---

## 🔴 CRITICAL — Mainnet-Blocker

### AD-004: Chain-ID 9000 — Konflikt mit XDC Network
**Agent:** RoadmapAgent + KnowledgeAgent
**Zustand:** ASSUMPTION → DECISION

Chain-ID 9000 ist in der EVM Chain Registry bereits von **XDC Network** belegt.
Wenn A-TownChain eine EVM-kompatible Chain sein soll, führt das zu Konflikten
in Wallets (MetaMask etc.) und Bridges.

**Optionen:**
1. Eigene, nicht-EVM Chain → Chain-ID 9000 kein Problem
2. Neue EVM Chain-ID wählen (z.B. 91337, 2025, etc.)
3. Sub-Network unter Polkadot/Substrate → kein eigener Chain-ID-Konflikt

**Betroffene Module:** Mainnet Config, Bridge, alle Wallet-Integrationen

---

### AD-006: Python vs. Substrate Runtime — Migration
**Agent:** ArchitectAgent
**Zustand:** DECISION

Sprint 2.1 plant Substrate Node Template (Rust). Aktuell: 149 Python-Module.
Wie und wann findet die Migration statt?

**Optionen:**
1. Parallelbetrieb (Python für Entwicklung, Substrate für Produktion)
2. Vollständige Migration zu Rust/Substrate (Breaking Change)
3. Python bleibt Hauptsprache, Substrate nur für Consensus-Pallets

**Zeitplan:** Entscheidung muss vor Sprint 2.1 (Jul 2026) getroffen werden.

---

## 🟡 HIGH — Sicherheits-Entscheidungen

### AD-001: SHA-256 vs. Keccak-256 für TX-Hashing
**Agent:** SecurityAgent
**Zustand:** DECISION

**Problem:** ECDSA-Wallet verwendet SHA-256. Ethereum-Standard ist Keccak-256.
Für die Frontier-Bridge (ETH-Kompatibilität) könnte das zu Problemen führen.

**Optionen:**
1. SHA-256 behalten (A-TownChain eigener Standard — kein ETH-Kompatibilitätsproblem)
2. Zu Keccak-256 migrieren (bricht alle bestehenden Wallets/Signaturen)

---

### AD-003: Voting-Power Snapshot (Flash-Loan-Schutz)
**Agent:** GovernanceAgent
**Zustand:** VALIDATE

**Problem:** DAOGovernance prüft Balance zum Vote-Zeitpunkt — anfällig für Flash-Loan-Attacks.
**Fix:** Snapshot der Stimmrechte zum Proposal-Erstellungs-Zeitpunkt (Compound/Aave-Standard).
**Aufwand:** ~2-3 Tage Implementierung in `dao_live.py`
**Empfehlung:** Implementieren vor Mainnet-Launch.

---

## 🟠 MEDIUM — Architektur-Entscheidungen

### AD-002: EventBus vs. IPCBus — Redundanz oder Trennung?
**Agent:** ArchitectAgent
**Zustand:** VALIDATE

`core/event_bus.py` und `modules/kernel/ipc/ipc_bus.py` existieren beide.
Ohne Wiki-Kapitel für EventBus ist unklar ob das bewusst oder Redundanz ist.

**Aktion:** Michael bitte `core/event_bus.py` prüfen — soll es behalten werden?

---

### AD-005: AIP-001 Agent Interaction Protocol
**Agent:** StandardsAgent
**Zustand:** DECISION

Das Protokoll für Agent-zu-Agent Kommunikation fehlt komplett.
KI-Agenten kommunizieren via IPC Bus, aber Message-Format, Timeouts,
Fehlerbehandlung sind nicht spezifiziert.

**Empfehlung:** AIP-001 als Kapitel 63 im Wiki definieren. Kann automatisch abgeleitet werden.

---

*Automatisch generiert von MasterBrain (Aurora) · 2026-06-12*
*Bei jeder Sync-Runde geprüft und aktualisiert*
