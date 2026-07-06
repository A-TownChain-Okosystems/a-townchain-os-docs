# 🔐 Hybrid Consensus — Technische Dokumentation

> **Algorithmus:** SHA-256 PoW + PoS + PoH
> **Datei:** `blockchain/consensus/hybrid_consensus.py`

---

## Überblick

A-TownChain verwendet einen **dreistufigen hybriden Konsens-Mechanismus**:

```
Schritt 1: PoH   → Kryptographischer Zeitbeweis (Proof of History)
Schritt 2: PoW   → Miner sucht SHA-256 Hash mit N führenden Nullen
Schritt 3: PoS   → Validator bestätigt Block (gewichtet nach Stake)
```

Alle drei Schritte müssen erfüllt sein, damit ein Block gültig ist.

---

## SHA-256 Proof of Work

| Parameter | Wert |
|-----------|------|
| Algorithmus | SHA-256 (doppelt) |
| Difficulty | Anfangswert: 3 führende Nullen |
| Ziel-Blockzeit | 10 Sekunden |
| Difficulty-Anpassung | Nach jedem Block |
| Halving-Intervall | 210.000 Blöcke |
| Start-Reward | 50 ATC |

### Difficulty-Anpassung

```python
def adjust_difficulty(self, avg_block_time: float, target: float = 10.0) -> int:
    if avg_block_time < target * 0.9:   # zu schnell → schwerer
        self.difficulty += 1
    elif avg_block_time > target * 1.1: # zu langsam → leichter
        if self.difficulty > 1:
            self.difficulty -= 1
    self.target = "0" * self.difficulty
    return self.difficulty
```

---

## Proof of Stake

| Parameter | Wert |
|-----------|------|
| Min. Stake | 10.000 ATC |
| Auswahl | Weighted Random (proportional zum Stake) |
| Slashing | 50% Verlust bei double-sign |
| Unstaking | Sofort möglich |

### Validator-Auswahl (deterministisch)

```python
def select_validator(self, seed: str) -> str:
    # Seed = Block-Hash → deterministisch, nicht manipulierbar
    rng = random.Random(int(hashlib.sha256(seed.encode()).hexdigest(), 16))
    total = sum(self.validators.values())
    r, cumulative = rng.uniform(0, total), 0
    for addr, stake in self.validators.items():
        cumulative += stake
        if r <= cumulative:
            return addr
```

---

## Proof of History

| Parameter | Wert |
|-----------|------|
| Algorithmus | Rekursives SHA-256 |
| Sequenz | Unbegrenzt (monoton steigend) |
| Verifikation | Unabhängig von Netzwerk möglich |
| Inspiration | Solana PoH |

```python
def tick(self, data: bytes = None) -> dict:
    combined = (self.current_hash + (data.hex() if data else "")).encode()
    self.current_hash = hashlib.sha256(combined).hexdigest()
    self.sequence += 1
    return {"sequence": self.sequence, "hash": self.current_hash}
```

---

## Block-Erstellung (Hybrid)

```
Input: transactions[], miner_address

1. poh_entry = poh.tick(json(transactions))
   → Zeitstempel-Beweis für diesen Block

2. block_data = {
     height, prev_hash,
     poh_hash, poh_sequence,
     transactions, miner, timestamp
   }

3. pow_result = pow.mine_block(block_data)
   → Nonce + gültiger Hash

4. validator = pos.select_validator(pow_result.hash)
   → Deterministisch aus Stake-Gewichten

5. block_data += { hash, nonce, validator, reward }

6. blocks.append(block_data)
   → Block final
```

---

> **Dokument:** `docs/architecture/CONSENSUS.md`
> **Datum:** 2026-05-19 · **Autor:** ShivaCoreDev × Aurora AI
