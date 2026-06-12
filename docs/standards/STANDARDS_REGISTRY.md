# 📐 A-TownChain Standards Registry

> **Gepflegt von:** StandardsAgent (Aurora Ecosystem Brain)
> **Stand:** 2026-06-12 | **Kategorien:** ATC · ATS · KIP · AIP

---

## ATC Standards (A-TownChain Core)

### Konsensus (ATC-1000 Serie)

#### ATC-1000 v1.1 — Proof of History ✅ ACCEPTED
**Algorithmus:** SHA-3_256 VDF Hash-Kette
**Formel:** `H_n = SHA3_256(H_(n-1) || seq.to_bytes(8) || data?)`
**VDF-Delay:** 100µs pro Tick (nicht parallelisierbar)
**Implementierung:** `blockchain/consensus/poh.py`
**Wiki:** Kap. 37

#### ATC-1001 v1.0 — Proof of Work ✅ ACCEPTED
**Algorithmus:** SHA-256 Target-Hash
**Halving:** `reward = 50 / 2^(height // 210_000)`
**Ziel-Blockzeit:** 10 Sekunden
**Implementierung:** `blockchain/consensus/pow.py`
**Wiki:** Kap. 4

#### ATC-1002 v1.0 — Proof of Stake ✅ ACCEPTED
**Auswahl:** `P(validator_i) = stake_i / Σstake`
**Deterministisch:** via SHA-256(seed).hexdigest() als RNG-Seed
**MIN_STAKE:** 10.000 ATC
**Slashing:** -50% bei Double-Sign
**Implementierung:** `blockchain/consensus/pos.py`
**Wiki:** Kap. 4

#### ATC-1003 v1.0 — Fork Resolution ✅ ACCEPTED
**Priorität:** `max(height, total_work, poh_ticks, -timestamp)`
**Nakamoto-Konsens** + PoH-Gewichtung als Tiebreaker
**Implementierung:** `blockchain/consensus/fork_resolution.py`
**Schwäche:** Kein Orphan-Pool (Sprint 2.3)

#### ATC-1004 v1.0 — Initial Sync ✅ ACCEPTED
**Protokoll:** `CHAIN_INFO → Best-Peer(max height, min latency) → Batch(50) → Validate → Apply`
**Implementierung:** `blockchain/nodes/initial_sync.py`

---

### Kryptografie (ATC-2000 Serie)

#### ATC-2000 v1.0 — ECDSA Signatur ✅ ACCEPTED
**Kurve:** secp256k1
**Hash:** SHA-256 (⚠️ AD-001: Keccak-256 für ETH-Kompatibilität offen)
**Format:** DER-kodiert
**Implementierung:** `blockchain/wallet/ecdsa.py`

---

### DeFi (ATC-3000 + ATC-4000 Serie)

#### ATC-3000 v1.0 — Gas Fee EIP-1559 ✅ ACCEPTED
**Formel:** `new_fee = old_fee × (1 ± min(|used - target| / target, 0.125))`
**Target:** 5M Gas/Block | **Limit:** 10M Gas/Block
**Burning:** 50% der Base Fee
**Implementierung:** `blockchain/consensus/gas_fee.py`
**Wiki:** Kap. 56

#### ATC-4000 v1.0 — AMM Constant-Product ✅ ACCEPTED
**Invariante:** `x * y = k`
**Swap-Formel:** `out = (in × (10000-fee) × reserve_out) / (reserve_in × 10000 + in × (10000-fee))`
**Fee:** 0.30% (30 Basispunkte)
**Protocol Fee:** 0.05% → Treasury
**Reentrancy-Guard:** ✅ implementiert
**Implementierung:** `blockchain/dex/amm.py`
**Wiki:** Kap. 26

---

### Token Standards (ATC-8000 Serie)

#### ATC-8300 v1.0 — Fungible Token ✅ ACCEPTED
ERC-20-kompatibel. Symbol: FFT. DAO-Stimmrecht: 1 FFT = 1 Vote.

#### ATC-9000 v1.0 — NFT Standard (Shivamon) ✅ ACCEPTED
ERC-721-kompatibel. DNA-Hash, Stats, Rarity×6, Breeding, Battle.
`DNA = SHA256(token_id + name + element + time + urandom(8))`

#### ATC-9900 v0.9 — Cross-Chain Bridge 🔄 REVIEW
3-of-5 Multi-Sig Relayer. Lock/Mint, Burn/Unlock. Daily-Limit.
Ethereum via Frontier-Pallet, Solana via Anchor/SPL.

---

## KIP Standards (KAI-OS Interface Protocols)

#### KIP-001 v0.1 — Kernel Interface Protocol 📝 DRAFT
Syscall-IDs, IPC-Topics, Kernel Module Registration.
Abgeleitet aus `shivaos/kernel/syscalls.py` + `modules/kernel/ipc/ipc_bus.py`.
**Status:** DERIVE — formale Spezifikation ausstehend.

---

## AIP Standards (Agent Interaction Protocols)

#### AIP-001 v0.1 — Agent Interaction Protocol 📝 DRAFT
Wie Agenten kommunizieren, Tasks delegieren, Ergebnisse melden.
**Status:** DECISION — formale Spezifikation fehlt komplett. (AD-005)

---

## ATS Standards (A-TownChain Testing Standards)

#### ATS-001 v0.1 — Testing Standard 📝 DRAFT
Mindest-Coverage, Naming-Conventions, CI/CD-Integration.
**Status:** DRAFT — basiert auf bestehenden Tests in `tests/`

---

## Geplante Standards (ResearchAgent Vorschläge)

| ID | Titel | Priorität | Sprint |
|----|-------|-----------|--------|
| ATC-5000 | ZKP — Zero-Knowledge Proofs | MEDIUM | 3.0 (#47) |
| ATC-6000 | Agent Registry Standard | MEDIUM | 2.5 |
| ATC-7000 | Federated Learning Protocol | LOW | 3.0 (#29) |
| KIP-002 | ShivaOS UI Rendering Protocol | LOW | 2.5 (#28) |
| AIP-002 | Multi-Agent Orchestration Protocol | HIGH | 2.3 |

---

*Generiert von StandardsAgent (Aurora Ecosystem Brain) · 2026-06-12*
