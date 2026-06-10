# 📄 Issue #1 — Smart Contract Implementation (ATC Token Standards)

> **Labels:** enhancement · blockchain
> **Priorität:** 🔴 High · **Milestone:** v2.1.0
> **Referenz:** [GitHub Issue #1](https://github.com/ShivaCoreDev/a-townchain-os/issues/1)

---

## Ziel

Implementierung der nativen ATC Smart Contracts für alle definierten Token-Standards des A-TownChain Ökosystems (ATC-001 bis ATC-9900) — zunächst als Python-Contracts, später als Solidity On-Chain Contracts (→ Issue #12).

---

## Hintergrund

Die aktuelle `blockchain/smart_contracts.py` ist ein Placeholder. Die vollständigen ATC Token Standards aus der ATC-Referenzmatrix müssen als vollwertige, testbare Contracts implementiert werden.

---

## Technische Spezifikation

### Verzeichnisstruktur (Ziel)

```
blockchain/contracts/
├── atc001/
│   └── genesis_token.py          # ATC-001: Genesis Block Token
├── atc8300/
│   └── atc_token.py              # ATC-8300: Fungible Token (ERC20)
├── atc9000/
│   └── shivamon_contract.py      # ATC-9000: NFT ✅ bereits implementiert
├── atc9900/
│   └── governance_contract.py    # ATC-9900: DAO/Governance (→ Issue #9)
└── base/
    └── base_contract.py          # Gemeinsame Basisklasse
```

### ATC-001 Genesis Token

```python
class GenesisToken:
    """
    ATC-001 — Der erste Token auf der A-TownChain.
    Einmalig geprägt beim Genesis Block.
    Nicht transferierbar — symbolischer Ursprungs-Token.
    """
    TOKEN_ID    = "ATC-001-GENESIS"
    TOTAL_SUPPLY = 1          # Genau 1 Genesis Token
    OWNER       = "ShivaCoreDev"
    MINTED_AT   = "2025-01-01T00:00:00Z"
    TRANSFERABLE = False
```

### ATC-8300 Fungible Token (vollständig)

Ergänzungen zur bestehenden `blockchain/atcoin/atcoin.py`:

```python
# Noch fehlende Methoden:
def burn(self, address: str, amount: Decimal) -> dict:
    """Token vernichten — reduziert total_supply permanent."""

def pause(self) -> bool:
    """Alle Transfers einfrieren (nur Contract-Owner)."""

def unpause(self) -> bool:
    """Transfers wieder freigeben."""

def snapshot(self) -> dict:
    """Snapshot aller Balances zu einem bestimmten Zeitpunkt."""

def get_tx_history(self, address: str, limit: int = 50) -> list:
    """TX-Historie einer Adresse abrufen."""
```

### Basisklasse für alle Contracts

```python
# blockchain/contracts/base/base_contract.py
class BaseContract:
    def __init__(self, owner: str):
        self.owner      = owner
        self.paused     = False
        self.deployed_at = int(time.time())
        self.version    = "1.0.0"

    def only_owner(self, caller: str) -> bool:
        if caller != self.owner:
            raise PermissionError("Only contract owner")
        return True

    def not_paused(self) -> bool:
        if self.paused:
            raise RuntimeError("Contract is paused")
        return True
```

---

## Implementierungs-Phasen

### Phase 1 — Grundstruktur ✅ teilweise
- [x] `ATCoin` Basis-Contract (ERC20-kompatibel, ATC-8300)
- [x] Transfer, Approve, Allowance
- [ ] `burn()`, `pause()`, `snapshot()`
- [ ] `BaseContract` Basisklasse

### Phase 2 — ATC Standards
- [ ] ATC-001 Genesis Token Contract
- [ ] ATC-8300: fehlende Methoden ergänzen
- [ ] ATC-9900 Governance Contract (→ koordiniert mit Issue #9)

### Phase 3 — Tests
- [ ] `tests/test_atc8300.py` — Unit Tests (Coverage > 90%)
- [ ] Test: Transfer, Approve, Allowance, Burn, Pause
- [ ] Test: Halving-Formel bei verschiedenen Block-Heights
- [ ] Test: Max-Supply Limit

### Phase 4 — Integration
- [ ] Contract ABI-Schema exportieren (`config/abis/`)
- [ ] Backend-Routes vollständig verdrahten
- [ ] Gateway Route `/api/blockchain/contract/:name`

---

## Akzeptanzkriterien

- [ ] Alle ATC Standards implementiert und testbar
- [ ] Unit Test Coverage > 90%
- [ ] API-Endpoints funktional via Gateway
- [ ] Keine Breaking Changes zu bestehenden Routes
- [ ] Dokumentation in `docs/contracts/`

---

## Referenzen

- `blockchain/atcoin/atcoin.py` — bestehende ATC-8300 Basis
- `blockchain/contracts/shivamon/shivamon_contract.py` — Referenz-Implementierung
- ATC Token Referenzmatrix (ATC-001 bis ATC-9900)
# 📄 Issue #2 — Gemini AI Integration

> **Labels:** enhancement · ai · priority:high
> **Priorität:** 🔴 High · **Milestone:** v2.1.0
> **Referenz:** [GitHub Issue #2](https://github.com/ShivaCoreDev/a-townchain-os/issues/2)

---

## Ziel

Anbindung der **Google Gemini 2.0 API** als KI-Gehirn des A-TownChain OS. Der AI Orchestrator im Dashboard wird zu einem vollwertigen Live-Chat-Interface mit A-TownChain-Kontext.

---

## Architektur

```
Frontend (AI Panel)
  └─→ POST /api/ai/query  (via Gateway :4000)
              │
     backend/api/routes/ai.py
              │
     AIOrchestrator.query()
              │
     Google Gemini 2.0 API
              │
     Response (Streaming / SSE)
              │
     Frontend Live-Typing-Effekt
```

---

## Technische Spezifikation

### Backend — `backend/api/routes/ai.py`

```python
import google.generativeai as genai
import os

SYSTEM_PROMPT = """
Du bist der A-TownChain AI Orchestrator — das KI-Gehirn von ShivaOS.
Du kennst das gesamte A-TownChain Ökosystem:
  - ATC-8300 Fungible Token (A-Town Coin)
  - ATC-9000 Shivamon NFTs
  - SHA-256 PoW + PoS + PoH Hybrid Consensus
  - API Gateway Architektur
  - Node-Netzwerk (FULL, LIGHT, VALIDATOR, MINER)
Antworte präzise, technisch korrekt und im Stil eines Blockchain-Experten.
"""

genai.configure(api_key=os.getenv("GEMINI_API_KEY"))
model = genai.GenerativeModel("gemini-2.0-flash")

@ai_bp.route("/query", methods=["POST"])
def query():
    prompt = request.json.get("prompt", "")
    chat   = model.start_chat(history=[])
    resp   = chat.send_message(SYSTEM_PROMPT + "\n\nUser: " + prompt)
    return jsonify({
        "response": resp.text,
        "model":    "gemini-2.0-flash",
        "tokens":   resp.usage_metadata.total_token_count
    })

@ai_bp.route("/stream", methods=["POST"])
def stream():
    """Server-Sent Events für Live-Typing-Effekt."""
    prompt = request.json.get("prompt", "")
    def generate():
        for chunk in model.generate_content(prompt, stream=True):
            yield f"data: {chunk.text}\n\n"
        yield "data: [DONE]\n\n"
    return Response(generate(), mimetype="text/event-stream")
```

### Umgebungsvariablen

```bash
# .env
GEMINI_API_KEY=your-gemini-api-key-here
GEMINI_MODEL=gemini-2.0-flash
GEMINI_MAX_TOKENS=2048
GEMINI_TEMPERATURE=0.7
```

### Frontend — AI Panel Update

```javascript
// Streaming-Antwort empfangen
async function queryAI(prompt) {
  const res = await fetch(`${GATEWAY}/ai/stream`, {
    method: "POST",
    headers: { ...headers },
    body: JSON.stringify({ prompt })
  });
  const reader = res.body.getReader();
  const decoder = new TextDecoder();
  let output = "";
  while (true) {
    const { done, value } = await reader.read();
    if (done) break;
    const chunk = decoder.decode(value);
    output += chunk.replace("data: ", "");
    document.getElementById("ai-output").textContent = output;
  }
}
```

---

## Aufgaben

- [ ] `backend/api/routes/ai.py` vollständig implementieren
- [ ] `google-generativeai` zu `backend/requirements.txt` hinzufügen
- [ ] `.env.example` mit `GEMINI_API_KEY` erweitern
- [ ] System-Prompt mit A-TownChain Kontext definieren
- [ ] Streaming-Endpoint (`/api/ai/stream`) für SSE
- [ ] Token-Zähler + verbrauchte Kosten loggen
- [ ] Chat-History im Frontend (letzte 10 Nachrichten)
- [ ] Gateway Route `/api/ai/` registrieren
- [ ] Frontend AI-Panel verdrahten (Live-Typing-Effekt)
- [ ] Fehlerbehandlung: API-Key fehlt / Rate Limit erreicht

---

## Akzeptanzkriterien

- [ ] AI antwortet auf A-TownChain-Fragen korrekt und kontextbewusst
- [ ] Streaming-Antwort mit Live-Typing im Dashboard
- [ ] Token-Verbrauch wird geloggt
- [ ] Graceful Degradation wenn API-Key fehlt

---

## Referenzen

- `backend/api/server.py` — Placeholder bereits vorhanden
- Gemini API Docs: https://ai.google.dev/gemini-api/docs
- `gateway/router.py` — Route `ai` bereits registriert
# 📄 Issue #3 — Shivamon Battle UI

> **Labels:** enhancement · frontend · priority:high
> **Priorität:** 🔴 High · **Milestone:** v2.1.0
> **Referenz:** [GitHub Issue #3](https://github.com/ShivaCoreDev/a-townchain-os/issues/3)

---

## Ziel

Visuelles, animiertes Battle-System im ShivaOS Dashboard. Shivamon kämpfen Runde für Runde mit HP-Bars, Schadensanzeigen und XP-Gewinn — vollständig im Browser, verdrahtet mit dem bestehenden `ShivamonContract.battle()`.

---

## UI-Layout

```
┌──────────────────────────────────────────────────┐
│  ⚔️  SHIVAMON BATTLE ARENA                        │
├──────────────────┬────────┬─────────────────────┤
│  ATTACKER        │   VS   │  DEFENDER            │
│  🔥 Ignarex-001  │        │  💧 Aquarix-007      │
│  HP ████████░░   │        │  HP ██████░░░░       │
│  Lv.3 · Rare     │        │  Lv.5 · Epic         │
├──────────────────┴────────┴─────────────────────┤
│  BATTLE LOG                                      │
│  > Runde 1: Ignarex trifft für 52 Schaden!      │
│  > Runde 1: Aquarix antwortet mit 38 Schaden!   │
│  > Runde 2: Ignarex trifft für 47 Schaden!      │
│  > 🏆 IGNAREX GEWINNT! +60 XP · Level Up! ⬆️   │
├─────────────────────────────────────────────────┤
│  [▶ KAMPF STARTEN]  [🔄 NEUE AUSWAHL]           │
└─────────────────────────────────────────────────┘
```

---

## Technische Spezifikation

### HTML-Struktur

```html
<div id="page-battle" class="page-content">

  <!-- Auswahl-Phase -->
  <div id="battle-select">
    <div class="battle-slot" id="slot-attacker">
      <!-- Dropdown: eigene Shivamon -->
    </div>
    <div class="vs-divider">⚔️ VS</div>
    <div class="battle-slot" id="slot-defender">
      <!-- Input: Gegner Token-ID oder eigene Collection -->
    </div>
  </div>

  <!-- Kampf-Arena -->
  <div id="battle-arena" style="display:none">
    <div class="fighter" id="fighter-a">...</div>
    <div class="fighter" id="fighter-d">...</div>
  </div>

  <!-- Battle Log -->
  <div id="battle-log"></div>

  <!-- Ergebnis -->
  <div id="battle-result" style="display:none">...</div>

</div>
```

### Animierter Kampfablauf (JavaScript)

```javascript
async function runBattle(attackerId, defenderId) {
  const result = await ATC_API.battleShivamon(attackerId, defenderId);
  const rounds = result.rounds;

  // Runden animiert durchspielen
  for (const round of rounds) {
    await sleep(800);
    appendLog(`Runde ${round.round}: ${round.attacker} trifft für ${round.damage} Schaden!`);
    updateHPBar(round.attacker === attackerName ? "defender" : "attacker", round.defender_hp);
    playHitAnimation(round.attacker === attackerName ? "attacker" : "defender");
  }

  await sleep(1000);
  showResult(result.winner, result.xp_gained);
}

function updateHPBar(side, currentHp) {
  const bar  = document.getElementById(`hp-bar-${side}`);
  const pct  = Math.max(0, (currentHp / maxHp[side]) * 100);
  bar.style.width    = pct + "%";
  bar.style.background = pct > 50 ? "var(--green)" : pct > 25 ? "var(--orange)" : "var(--pink)";
}
```

### CSS — Kampf-Animationen

```css
@keyframes hit-shake {
  0%,100% { transform: translateX(0); }
  25%      { transform: translateX(-8px); }
  75%      { transform: translateX(8px); }
}
@keyframes damage-float {
  0%   { opacity: 1; transform: translateY(0); }
  100% { opacity: 0; transform: translateY(-40px); }
}
.fighter.hit { animation: hit-shake 0.3s ease; }
.damage-number {
  position: absolute; font-family: 'Orbitron', sans-serif;
  font-size: 20px; color: var(--pink); font-weight: 900;
  animation: damage-float 1s ease forwards;
}
```

---

## Aufgaben

- [ ] Battle-Seite in `frontend/index.html` — Sidebar `⚔️ Battle`
- [ ] Shivamon-Auswahl aus eigener Collection (Dropdown)
- [ ] Gegner-Eingabe (Token-ID oder eigene Collection)
- [ ] Animierter Kampfablauf Runde für Runde (800ms Delay)
- [ ] HP-Bar Echtzeit-Update mit Farbwechsel (Grün → Orange → Rot)
- [ ] Floating Damage Numbers
- [ ] Level-Up Animation nach XP-Gewinn
- [ ] Battle-Log als scrollbares Protokoll
- [ ] Battle-History Tabelle (letzte 20 Kämpfe via API)
- [ ] API-Call `POST /api/game/shivamon/battle` verdrahten

---

## Akzeptanzkriterien

- [ ] Kampf läuft animiert Runde für Runde ab
- [ ] HP-Bars aktualisieren sich korrekt
- [ ] Gewinner und XP werden angezeigt
- [ ] Level-Up wird visuell kommuniziert
- [ ] Funktioniert ohne Backend (Mock-Modus)
# 📄 Issue #4 — NFT Persistenz (SQLite)

> **Labels:** enhancement · backend · priority:high
> **Priorität:** 🔴 High · **Milestone:** v2.1.0
> **Referenz:** [GitHub Issue #4](https://github.com/ShivaCoreDev/a-townchain-os/issues/4)

---

## Ziel

Shivamon NFTs, ATC-Wallets und Transaktionen persistent in einer **SQLite-Datenbank** speichern. Aktuell gehen alle Daten bei Server-Neustart verloren — das ist ein Production-Blocker.

---

## Datenbankschema

```sql
-- Wallets
CREATE TABLE wallets (
    address     TEXT PRIMARY KEY,          -- ATC... (35 Zeichen)
    public_key  TEXT NOT NULL,
    label       TEXT DEFAULT 'Main Wallet',
    balance     REAL DEFAULT 0.0,
    created_at  INTEGER NOT NULL
    -- KEIN private_key! Niemals in DB speichern.
);

-- Transaktionen
CREATE TABLE transactions (
    tx_id       TEXT PRIMARY KEY,          -- SHA-256 Hash
    tx_type     TEXT NOT NULL,             -- transfer | mint | burn
    from_addr   TEXT,
    to_addr     TEXT NOT NULL,
    amount      REAL NOT NULL,
    fee         REAL DEFAULT 0.001,
    timestamp   INTEGER NOT NULL,
    block_height INTEGER
);

-- Shivamon NFTs
CREATE TABLE shivamon (
    token_id    TEXT PRIMARY KEY,          -- SHV-...
    name        TEXT NOT NULL,
    element     TEXT NOT NULL,
    rarity      TEXT NOT NULL,
    owner       TEXT NOT NULL,
    generation  INTEGER DEFAULT 1,
    level       INTEGER DEFAULT 1,
    xp          INTEGER DEFAULT 0,
    wins        INTEGER DEFAULT 0,
    losses      INTEGER DEFAULT 0,
    hp          INTEGER, attack INTEGER,
    defense     INTEGER, speed INTEGER, special INTEGER,
    dna_hash    TEXT NOT NULL,
    moves       TEXT,                      -- JSON Array
    minted_at   INTEGER NOT NULL,
    FOREIGN KEY (owner) REFERENCES wallets(address)
);

-- Blöcke
CREATE TABLE blocks (
    height      INTEGER PRIMARY KEY,
    hash        TEXT NOT NULL UNIQUE,
    prev_hash   TEXT NOT NULL,
    miner       TEXT,
    validator   TEXT,
    reward      REAL,
    difficulty  INTEGER,
    nonce       INTEGER,
    poh_hash    TEXT,
    timestamp   INTEGER NOT NULL,
    tx_count    INTEGER DEFAULT 0
);
```

---

## Implementierung

### Repository Pattern

```python
# backend/db/repository.py
import sqlite3, json
from pathlib import Path

DB_PATH = Path("data/atcoin.db")

class WalletRepository:
    def save(self, address, public_key, label, balance): ...
    def find(self, address) -> dict: ...
    def update_balance(self, address, new_balance): ...

class ShivamonRepository:
    def save(self, nft: ShivamonNFT): ...
    def find(self, token_id) -> ShivamonNFT: ...
    def find_by_owner(self, owner) -> list: ...
    def update(self, nft: ShivamonNFT): ...

class BlockRepository:
    def save(self, block: dict): ...
    def find_latest(self, n=10) -> list: ...
    def get_height(self) -> int: ...
```

### Migration (In-Memory → SQLite)

```python
# backend/db/migrate.py
def migrate_from_memory(wallet_instance, contract_instance, consensus_instance):
    """Einmalige Migration beim ersten Start."""
    repo_w = WalletRepository()
    repo_s = ShivamonRepository()
    repo_b = BlockRepository()

    for address, data in wallet_instance.accounts.items():
        repo_w.save(address, data["public_key"], data["label"],
                    float(wallet_instance.atcoin.balance_of(address)))

    for token_id, nft in contract_instance.tokens.items():
        repo_s.save(nft)

    for block in consensus_instance.blocks:
        repo_b.save(block)
```

---

## Aufgaben

- [ ] `data/` Verzeichnis anlegen + `.gitignore` (DB-Dateien nicht committen)
- [ ] `backend/db/__init__.py` + `backend/db/schema.sql`
- [ ] `backend/db/repository.py` — WalletRepository, ShivamonRepository, BlockRepository, TxRepository
- [ ] `backend/db/migrate.py` — Migration-Script
- [ ] `ATCWallet` — Persistenz-Layer einbinden
- [ ] `ShivamonContract` — Persistenz-Layer einbinden
- [ ] `HybridConsensus` — Blocks persistent speichern
- [ ] Tägliches Backup-Script (`build/backup.py`)
- [ ] Tests: `tests/test_persistence.py`

---

## Sicherheitshinweise

> ⚠️ **Private Keys werden NIEMALS in der Datenbank gespeichert.**
> Nur `address` (public) und `public_key` werden persistiert.
> Der Private Key existiert nur im Moment der Wallet-Erstellung im RAM.

---

## Akzeptanzkriterien

- [ ] Server-Neustart behält alle Wallets, NFTs und Blöcke
- [ ] Migration von In-Memory funktioniert fehlerfrei
- [ ] Keine Private Keys in der DB
- [ ] DB-Datei ist in `.gitignore`
# 📄 Issue #5 — ATC Blockchain Explorer

> **Labels:** enhancement · frontend · priority:medium
> **Priorität:** 🟡 Medium · **Milestone:** v2.1.0
> **Referenz:** [GitHub Issue #5](https://github.com/ShivaCoreDev/a-townchain-os/issues/5)

---

## Ziel

Eigener Blockchain-Explorer im ShivaOS Dashboard — Blöcke, Transaktionen, Adressen und Shivamon-Token durchsuchen und visualisieren.

---

## UI-Sektionen

```
┌────────────────────────────────────────────────────┐
│  🔍 ATC EXPLORER                  [Suche...]       │
├──────────┬──────────┬──────────┬───────────────────┤
│ Blöcke   │   TXs   │ Adressen │ Shivamon          │
│ 1.247    │ 38.902  │ 412      │ 9.042 geminted    │
├──────────┴──────────┴──────────┴───────────────────┤
│ LETZTE BLÖCKE                                      │
│ #1247 | 0xa3f9... | 12 TXs | vor 8 Sek | ⛏ Miner │
│ #1246 | 0xb4e8... |  7 TXs | vor 18 Sek| ⛏ Miner │
├────────────────────────────────────────────────────┤
│ LETZTE TRANSAKTIONEN                               │
│ TX-A3F9... | ATC7F3A→ATC9B2C | 150 ATC | ✅      │
└────────────────────────────────────────────────────┘
```

---

## Technische Spezifikation

### Frontend-Komponenten

```javascript
// Explorer-Tabs: Blocks | Transactions | Addresses | Shivamon
// Auto-Refresh: alle 10 Sekunden

async function loadExplorer() {
  const [chainInfo, blocks, coinInfo] = await Promise.all([
    ATC_API.getBlockchainInfo(),
    ATC_API.getBlocks(),
    ATC_API.getCoinInfo()
  ]);
  renderStats(chainInfo, coinInfo);
  renderBlockList(blocks.blocks);
}

function renderBlockRow(block) {
  return `
    <tr onclick="showBlockDetail('${block.hash}')">
      <td>#${block.height}</td>
      <td>${block.hash.slice(0,12)}...</td>
      <td>${block.transactions?.length || 0} TXs</td>
      <td>${timeAgo(block.timestamp)}</td>
      <td>${block.miner?.slice(0,10)}...</td>
    </tr>
  `;
}
```

### Suchfunktion

```javascript
async function explorerSearch(query) {
  query = query.trim();
  if (query.startsWith("ATC"))       return showAddressDetail(query);
  if (query.startsWith("SHV-"))      return showShivamonDetail(query);
  if (query.startsWith("TX-"))       return showTxDetail(query);
  if (!isNaN(query))                 return showBlockDetail(parseInt(query));
  showNotif("❌ Unbekanntes Format");
}
```

---

## Aufgaben

- [ ] Explorer-Seite in `frontend/index.html` (Sidebar: `🔍 Explorer`)
- [ ] Stats-Header: Chain-Height, TXs, Adressen, Shivamon-Count
- [ ] Block-Liste (letzte 20, auto-refresh alle 10s)
- [ ] Block-Detail-Ansicht (Hash, Miner, Validator, TXs, PoH-Hash)
- [ ] TX-Liste mit Von/An/Betrag/Status
- [ ] TX-Detail-Ansicht
- [ ] Adress-Suche: Balance + TX-History + Shivamon-Collection
- [ ] Shivamon-Token-Tracker
- [ ] Globale Suchleiste (Block-Nr, TX-ID, ATC-Adresse, SHV-Token-ID)
- [ ] Live-Updates via Polling (10s Interval)
- [ ] Verlinkung: Block → TXs → Adressen

---

## Akzeptanzkriterien

- [ ] Alle Blöcke und TXs abrufbar und dargestellt
- [ ] Suche funktioniert für alle Typen
- [ ] Live-Refresh ohne Seitenneuladen
- [ ] Mobile-freundliches Layout (responsive)
# 📄 Issue #6 — ECDSA Signatur (Sichere TX-Autorisierung)

> **Labels:** security · backend · priority:high
> **Priorität:** 🔴 High · **Milestone:** v2.1.0
> **Referenz:** [GitHub Issue #6](https://github.com/ShivaCoreDev/a-townchain-os/issues/6)

---

## Ziel

Alle Transaktionen und NFT-Transfers müssen mit dem **Private Key des Senders signiert** werden. Ohne ECDSA-Verifikation kann aktuell jeder beliebige Transfers auslösen, wenn die Adresse bekannt ist — das ist ein kritisches Sicherheitsproblem.

---

## Sicherheitsanalyse — Aktueller Zustand

| Endpoint | Aktuell | Soll |
|----------|---------|------|
| `POST /api/wallet/send` | Keine Signatur-Prüfung | ECDSA Signatur required |
| `POST /api/game/shivamon/transfer` | Keine Signatur-Prüfung | ECDSA Signatur required |
| `POST /api/game/shivamon/battle` | Keine Auth | Owner-Signatur required |
| `POST /api/governance/vote` | — (noch nicht impl.) | ECDSA required |

---

## Technische Spezifikation

### ECDSA Implementation (secp256k1)

```python
# blockchain/wallet/ecdsa.py
from cryptography.hazmat.primitives.asymmetric import ec
from cryptography.hazmat.primitives import hashes, serialization
import hashlib, json

class ECDSASigner:

    @staticmethod
    def generate_keypair() -> tuple[bytes, bytes]:
        """Generiert ein echtes ECDSA secp256k1 Schlüsselpaar."""
        private_key = ec.generate_private_key(ec.SECP256K1())
        public_key  = private_key.public_key()
        priv_bytes  = private_key.private_bytes(
            serialization.Encoding.Raw,
            serialization.PrivateFormat.Raw,
            serialization.NoEncryption()
        )
        pub_bytes = public_key.public_bytes(
            serialization.Encoding.X962,
            serialization.PublicFormat.UncompressedPoint
        )
        return priv_bytes.hex(), pub_bytes.hex()

    @staticmethod
    def sign(tx_data: dict, private_key_hex: str) -> str:
        """Signiert eine Transaktion mit dem Private Key."""
        message     = json.dumps(tx_data, sort_keys=True).encode()
        msg_hash    = hashlib.sha256(message).digest()
        private_key = ec.derive_private_key(
            int(private_key_hex, 16), ec.SECP256K1()
        )
        signature = private_key.sign(msg_hash, ec.ECDSA(hashes.Prehashed()))
        return signature.hex()

    @staticmethod
    def verify(tx_data: dict, signature_hex: str, public_key_hex: str) -> bool:
        """Verifiziert eine Transaktion-Signatur."""
        try:
            message    = json.dumps(tx_data, sort_keys=True).encode()
            msg_hash   = hashlib.sha256(message).digest()
            public_key = ec.EllipticCurvePublicKey.from_encoded_point(
                ec.SECP256K1(), bytes.fromhex(public_key_hex)
            )
            public_key.verify(
                bytes.fromhex(signature_hex),
                msg_hash,
                ec.ECDSA(hashes.Prehashed())
            )
            return True
        except Exception:
            return False
```

### TX-Signatur-Format

```json
{
  "tx": {
    "from":      "ATC7F3A9B2C1D4E5F6A7B8C9D0E1F2A3B4C5",
    "to":        "ATC9B2C1D4E5F6A7B8C9D0E1F2A3B4C5D6E7",
    "amount":    150.0,
    "fee":       0.001,
    "nonce":     42,
    "timestamp": 1747691880
  },
  "signature":  "3045022100a3f9b2c1....",
  "public_key": "04a3f9b2c1d4e5f6..."
}
```

### Backend Middleware

```python
# gateway/middleware/signature_verify.py
from blockchain.wallet.ecdsa import ECDSASigner

def require_signature(f):
    @wraps(f)
    def decorated(*args, **kwargs):
        data      = request.json or {}
        tx        = data.get("tx", {})
        signature = data.get("signature")
        pub_key   = data.get("public_key")
        if not ECDSASigner.verify(tx, signature, pub_key):
            return jsonify({"error": "Invalid signature"}), 401
        return f(*args, **kwargs)
    return decorated
```

---

## Aufgaben

- [ ] `blockchain/wallet/ecdsa.py` — ECDSA secp256k1 implementieren
- [ ] `ATCKeyGenerator` auf echtes ECDSA umstellen
- [ ] `gateway/middleware/signature_verify.py` — Signatur-Middleware
- [ ] `POST /api/wallet/send` — Signatur-Prüfung aktivieren
- [ ] `POST /api/game/shivamon/transfer` — Signatur-Prüfung aktivieren
- [ ] Nonce-System (verhindert Replay-Attacks)
- [ ] Frontend: Signatur im Wallet-UI generieren
- [ ] `cryptography` zu `requirements.txt` hinzufügen
- [ ] Tests: `tests/test_ecdsa.py`

---

## Akzeptanzkriterien

- [ ] Transfers ohne gültige Signatur werden abgelehnt (HTTP 401)
- [ ] Replay-Attacks durch Nonce verhindert
- [ ] Tests: Sign + Verify korrekt, falsche Signatur abgelehnt
- [ ] Kein Breaking Change für bestehende Dev-Flows (Dev-Mode ohne Signatur via Flag)
# 📄 Issue #7 — Build System (EXE / AppImage)

> **Labels:** enhancement · build · priority:medium
> **Priorität:** 🟡 Medium · **Milestone:** v2.2.0
> **Referenz:** [GitHub Issue #7](https://github.com/ShivaCoreDev/a-townchain-os/issues/7)

---

## Ziel

A-TownChain OS als **standalone Desktop-Anwendung** verpacken — ein einziger Doppelklick startet Gateway, Backend und öffnet das ShivaOS Dashboard im Browser. Kein Python, keine Installation nötig.

---

## Ziel-Architektur

```
ATownChainOS.exe / ATownChainOS.AppImage
│
├── Embedded Python Runtime (via PyInstaller)
├── gateway/main.py      → startet automatisch auf Port 4000
├── backend/main.py      → startet automatisch auf Port 5000
├── frontend/index.html  → wird in Standard-Browser geöffnet
└── data/atcoin.db       → lokale SQLite-Datenbank
```

---

## Build-Konfiguration

### PyInstaller Spec

```python
# build/atownchain.spec
block_cipher = None

a = Analysis(
    ['launcher.py'],
    pathex=['.'],
    binaries=[],
    datas=[
        ('frontend/', 'frontend/'),
        ('config/', 'config/'),
        ('blockchain/wallet/wordlist.py', 'blockchain/wallet/'),
    ],
    hiddenimports=['flask', 'flask_cors', 'requests', 'cryptography'],
    hookspath=[],
)

pyz = PYZ(a.pure, a.zipped_data, cipher=block_cipher)

exe = EXE(
    pyz, a.scripts, a.binaries, a.zipfiles, a.datas,
    name='ATownChainOS',
    icon='assets/icon.ico',
    onefile=True,
    console=False,
)
```

### Launcher

```python
# launcher.py — Startet alle Services + öffnet Browser
import subprocess, threading, time, webbrowser, sys

def start_gateway():
    subprocess.Popen([sys.executable, "gateway/main.py"])

def start_backend():
    subprocess.Popen([sys.executable, "backend/main.py"])

def open_dashboard():
    time.sleep(3)  # Warten bis Services starten
    webbrowser.open("http://localhost:3000")

if __name__ == "__main__":
    threading.Thread(target=start_gateway, daemon=True).start()
    threading.Thread(target=start_backend, daemon=True).start()
    open_dashboard()
```

### GitHub Actions CI/CD

```yaml
# .github/workflows/build.yml
name: Build Release
on:
  push:
    tags: ['v*']
jobs:
  build-windows:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
      - run: pip install pyinstaller
      - run: pyinstaller build/atownchain.spec
      - uses: actions/upload-artifact@v4
        with:
          name: ATownChainOS-Windows
          path: dist/ATownChainOS.exe

  build-linux:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: pip install pyinstaller
      - run: pyinstaller build/atownchain.spec
      - run: chmod +x dist/ATownChainOS
```

---

## Aufgaben

- [ ] `launcher.py` — Service-Starter implementieren
- [ ] `build/atownchain.spec` — PyInstaller Konfiguration
- [ ] `build/build.py` — Build-Script (lokaler Build)
- [ ] `assets/icon.ico` + `assets/icon.png` — App-Icon
- [ ] `.github/workflows/build.yml` — CI/CD Pipeline
- [ ] Windows EXE testen
- [ ] Linux AppImage testen
- [ ] Installer-Größe optimieren (< 100 MB Ziel)

---

## Akzeptanzkriterien

- [ ] Doppelklick auf EXE öffnet Dashboard im Browser
- [ ] Kein Python / pip nötig auf dem Zielrechner
- [ ] Windows 10+ und Ubuntu 20.04+ unterstützt
- [ ] GitHub Actions baut automatisch bei neuem Release-Tag
# 📄 Issue #8 — Multi-Node Testnet

> **Labels:** enhancement · blockchain · networking · priority:high
> **Priorität:** 🔴 High · **Milestone:** v2.2.0
> **Referenz:** [GitHub Issue #8](https://github.com/ShivaCoreDev/a-townchain-os/issues/8)

---

## Ziel

Mehrere A-TownChain Nodes zu einem echten P2P-Testnetzwerk verbinden — Blöcke werden propagiert, Konsens dezentral erreicht, neue Nodes können sich synchronisieren.

---

## Netzwerk-Architektur

```
Bootstrap Node (immer online)
  node-001:5005  ←── neue Nodes melden sich hier an

      ↕  P2P (TCP/UDP)

node-002:5105        node-003:5205
(VALIDATOR)          (MINER)
    ↕                    ↕
node-004:5305        node-005:5405
(FULL)               (LIGHT)
```

---

## Protokoll-Spezifikation

### Nachrichten-Typen

```python
MSG_TYPES = {
    "HANDSHAKE":   {"version": str, "chain_height": int, "node_type": str},
    "NEW_BLOCK":   {"block": dict},
    "NEW_TX":      {"tx": dict},
    "GET_BLOCKS":  {"from_height": int, "to_height": int},
    "BLOCKS":      {"blocks": list},
    "GET_PEERS":   {},
    "PEERS":       {"peers": list},
    "PING":        {"timestamp": int},
    "PONG":        {"timestamp": int},
}
```

### Node Discovery

```python
# blockchain/nodes/discovery.py
class NodeDiscovery:
    BOOTSTRAP_NODES = [
        "127.0.0.1:5005",   # lokaler Bootstrap
        # Mainnet: "node1.atownchain.io:5005"
    ]

    def announce(self, my_address: str):
        """Meldet sich bei Bootstrap-Nodes an."""

    def get_peers(self) -> list:
        """Gibt bekannte Peers zurück."""

    def health_check(self):
        """Regelmäßiger Ping an alle Peers."""
```

### Docker Compose (5 lokale Nodes)

```yaml
# docker-compose.testnet.yml
version: '3.8'
services:
  node-001:
    build: .
    environment:
      NODE_ID: node-001
      NODE_TYPE: full
      NODE_PORT: 5005
      IS_BOOTSTRAP: "true"
    ports: ["5005:5005"]

  node-002:
    build: .
    environment:
      NODE_ID: node-002
      NODE_TYPE: validator
      NODE_PORT: 5105
      BOOTSTRAP: "node-001:5005"
      STAKE: "50000"
    depends_on: [node-001]

  node-003:
    build: .
    environment:
      NODE_ID: node-003
      NODE_TYPE: miner
      NODE_PORT: 5205
      BOOTSTRAP: "node-001:5005"
    depends_on: [node-001]
```

---

## Aufgaben

- [ ] `blockchain/nodes/discovery.py` — Node Discovery via UDP
- [ ] `blockchain/nodes/p2p.py` — TCP-basiertes P2P Protokoll
- [ ] `broadcast_block()` an alle Peers implementieren
- [ ] Initial Sync: neuer Node lädt Chain von Peers
- [ ] Longest-Chain-Rule (Fork-Auflösung)
- [ ] `docker-compose.testnet.yml` für 5 lokale Nodes
- [ ] Testnet Faucet: `GET /api/faucet/:address` (100 Test-ATC)
- [ ] Node-Monitoring im Dashboard (Live-Status aller Peers)
- [ ] Checkpoint-System (vertrauenswürdige Block-Hashes in `config/`)

---

## Akzeptanzkriterien

- [ ] 5 Nodes starten via `docker-compose up`
- [ ] Block wird von Miner an alle Peers propagiert
- [ ] Neuer Node synchronisiert sich vollständig
- [ ] Fork wird korrekt aufgelöst (Longest Chain gewinnt)
- [ ] Dashboard zeigt Live-Node-Status
# 📄 Issue #9 — Governance Contract (ATC-9900 DAO)

> **Labels:** enhancement · blockchain · governance · priority:medium
> **Priorität:** 🟡 Medium · **Milestone:** v2.2.0
> **Referenz:** [GitHub Issue #9](https://github.com/ShivaCoreDev/a-townchain-os/issues/9)

---

## Ziel

Implementierung des **ATC-9900 Governance Standards** — dezentrale Abstimmung über Protokoll-Änderungen, Parameter-Updates und Ökosystem-Entscheidungen.

---

## Governance-Mechanismus

```
ATC-Holder reicht Proposal ein
          │
          ▼
Voting-Phase (default: 7 Tage)
  └─ Jeder ATC-Holder kann abstimmen
  └─ 1 ATC = 1 Stimme (oder gestakt: 1.5x Gewicht)
          │
          ▼
Quorum erreicht? (mind. 10% der Total Supply)
  ├─ Nein → Proposal FAILED (Quorum nicht erreicht)
  └─ Ja   → Mehrheit > 50%?
              ├─ Ja  → Proposal PASSED → Execute
              └─ Nein→ Proposal REJECTED
```

---

## Datenmodell

```python
@dataclass
class Proposal:
    proposal_id:  str         # "GOV-" + SHA-256[:12]
    creator:      str         # ATC-Adresse
    title:        str
    description:  str
    options:      list[str]   # z.B. ["Ja", "Nein", "Enthaltung"]
    votes:        dict        # option → total_atc
    voters:       dict        # address → option (verhindert Doppelabstimmung)
    created_at:   int
    deadline:     int         # Unix-Timestamp
    quorum:       float       # Mindest-Beteiligung (default: 0.10 = 10%)
    status:       str         # "active" | "passed" | "failed" | "rejected"
    executed_at:  int | None
```

---

## Contract-Methoden

```python
class GovernanceContract:

    def create_proposal(self, creator, title, description,
                        options, duration_days=7) -> dict: ...
    # Erstellt neuen Proposal, zieht 100 ATC Einreichungsgebühr

    def vote(self, voter, proposal_id, option) -> dict: ...
    # Stimmt ab — Voting Power = ATC Balance

    def execute_proposal(self, proposal_id) -> dict: ...
    # Nach Deadline: wertet aus, setzt Status

    def get_proposals(self, status=None) -> list: ...
    def get_proposal(self, proposal_id) -> dict: ...
    def get_voting_power(self, address) -> dict: ...
    # Gibt Balance + Stake-Bonus zurück
```

---

## Aufgaben

- [ ] `blockchain/contracts/governance/governance_contract.py`
- [ ] `backend/api/routes/governance_routes.py` — REST API
- [ ] Automatische Proposal-Auswertung nach Deadline (Cron)
- [ ] Delegated Voting: Stimmrecht übertragen
- [ ] Frontend: Governance-Seite (Sidebar `🏛 Governance`)
- [ ] Proposal erstellen, abstimmen, Ergebnisse als Chart
- [ ] Gateway Route `/api/governance/` registrieren
- [ ] Tests: `tests/test_governance.py`

---

## Akzeptanzkriterien

- [ ] Proposal erstellen und abstimmen funktioniert
- [ ] Quorum-Check korrekt (10% der Total Supply)
- [ ] Doppelabstimmung verhindert
- [ ] Status nach Deadline automatisch gesetzt
# 📄 Issue #10 — Cross-Chain Bridge (ATC ↔ EVM)

> **Labels:** enhancement · blockchain · bridge · priority:low
> **Priorität:** 🟢 Low · **Milestone:** v3.0.0
> **Referenz:** [GitHub Issue #10](https://github.com/ShivaCoreDev/a-townchain-os/issues/10)

---

## Ziel

Eine sichere Bridge zwischen A-TownChain und EVM-kompatiblen Chains — ATC-Token werden auf A-TownChain eingefroren und als **Wrapped ATC (wATC)** auf Ethereum/Polygon ausgegeben.

---

## Lock-and-Mint Mechanismus

```
A-TownChain                          Ethereum (Sepolia)
──────────                           ──────────────────
User locked 1000 ATC                 →  Relayer erkennt Lock-Event
  in Lock Contract                   →  Mint 1000 wATC an User-Adresse
                                         (ERC20 auf Ethereum)

User burnt 500 wATC                  ←  Relayer erkennt Burn-Event
  auf Ethereum                       ←  Release 500 ATC aus Lock Contract
```

---

## Sicherheits-Architektur

| Maßnahme | Beschreibung |
|----------|-------------|
| Multi-Sig Relayer | 3/5 Signaturen benötigt für jede Bridge-TX |
| Rate Limiting | Max 10.000 ATC pro TX |
| Timelock | > 100.000 ATC → 24h Verzögerung |
| Emergency Pause | Owner kann Bridge sofort stoppen |
| Audit Trail | Jede Bridge-TX on-chain geloggt |

---

## Aufgaben

- [ ] `blockchain/bridge/bridge_contract.py` — Lock/Release
- [ ] `blockchain/bridge/relayer.py` — Event-Listener Service
- [ ] Solidity `WrappedATC.sol` auf Ethereum
- [ ] Multi-Sig Relayer Implementierung
- [ ] `POST /api/bridge/lock`, `GET /api/bridge/status/:id`
- [ ] Frontend Bridge-Interface (Sidebar: `🌉 Bridge`)
- [ ] Testnet Deployment (Sepolia + Polygon Mumbai)

> ⚠️ **Komplexität:** Dieses Feature hat die höchste Komplexität im gesamten Projekt.
> Empfohlen erst nach vollständigem v2.2.0 Release.
# 📄 Issue #11 — Shivamon Breeding (Gen 2 NFTs)

> **Labels:** enhancement · game · nft · priority:medium
> **Priorität:** 🟡 Medium · **Milestone:** v2.2.0
> **Referenz:** [GitHub Issue #11](https://github.com/ShivaCoreDev/a-townchain-os/issues/11)

---

## Ziel

Zwei Shivamon NFTs können gezüchtet werden und erzeugen ein einzigartiges Kind-NFT der **Generation 2** mit gemischten Stats, Hybrid-DNA und vererbter Rarity.

---

## Breeding-Algorithmus

```python
def breed(self, parent1_id: str, parent2_id: str, owner: str) -> dict:

    p1, p2 = self.tokens[parent1_id], self.tokens[parent2_id]

    # 1. Cooldown-Check (24h)
    now = int(time.time())
    if now - p1.last_breed < 86400:
        return {"success": False, "error": "Parent 1 in cooldown"}

    # 2. DNA mischen
    child_dna = hashlib.sha256(
        (p1.dna_hash + p2.dna_hash + str(now)).encode()
    ).hexdigest()

    # 3. Stats vererben (50/50 Mix + ±10% Mutation)
    child_stats = ShivamonStats(
        hp      = int((p1.stats.hp + p2.stats.hp) / 2 * random.uniform(0.9, 1.1)),
        attack  = int((p1.stats.attack + p2.stats.attack) / 2 * random.uniform(0.9, 1.1)),
        defense = int((p1.stats.defense + p2.stats.defense) / 2 * random.uniform(0.9, 1.1)),
        speed   = int((p1.stats.speed + p2.stats.speed) / 2 * random.uniform(0.9, 1.1)),
        special = int((p1.stats.special + p2.stats.special) / 2 * random.uniform(0.9, 1.1)),
    )

    # 4. Element: dominant (70%) / rezessiv (30%)
    child_el = random.choices([p1.element, p2.element], weights=[70, 30])[0]

    # 5. Rarity vererben
    child_rarity = self._inherit_rarity(p1.rarity, p2.rarity)

    # 6. Kind-NFT minten (Generation = max(p1.gen, p2.gen) + 1)
    return self.mint(owner=owner, element=child_el.value.split()[1],
                     rarity=child_rarity.value,
                     generation=max(p1.generation, p2.generation) + 1,
                     dna_override=child_dna)
```

### Rarity-Vererbungsmatrix

| Eltern | Common | Uncommon | Rare | Epic | Legendary | Genesis |
|--------|--------|----------|------|------|-----------|---------|
| C + C | 85% | 15% | — | — | — | — |
| U + U | 10% | 60% | 30% | — | — | — |
| R + R | — | 10% | 50% | 40% | — | — |
| E + E | — | — | 5% | 45% | 50% | — |
| L + L | — | — | — | 5% | 55% | 40% |
| G + G | — | — | — | — | 30% | 70% |

---

## Aufgaben

- [ ] `ShivamonContract.breed()` implementieren
- [ ] Cooldown-Tracking pro Shivamon (24h)
- [ ] DNA-Mixing Algorithmus
- [ ] Stat-Vererbung mit Mutations-Faktor
- [ ] Rarity-Vererbungsmatrix
- [ ] Breeding-Kosten: 25 ATC abziehen
- [ ] Max Generation: 10 (kein weiteres Breeding)
- [ ] `POST /api/game/shivamon/breed` API-Route
- [ ] Frontend Breeding-Interface im Shivamon-Tab
- [ ] Parent-Preview: geschätzte Kind-Stats anzeigen

---

## Akzeptanzkriterien

- [ ] Breeding erzeugt valides Gen-2 Shivamon
- [ ] Stats sind Mix beider Eltern (±10%)
- [ ] Cooldown von 24h wird korrekt durchgesetzt
- [ ] Breeding-Kosten von 25 ATC werden abgezogen
- [ ] Max Generation 10 wird erzwungen
# 📄 Issue #12 — Solidity Smart Contracts (On-Chain)

> **Labels:** enhancement · blockchain · solidity · priority:medium
> **Priorität:** 🟡 Medium · **Milestone:** v2.2.0
> **Referenz:** [GitHub Issue #12](https://github.com/ShivaCoreDev/a-townchain-os/issues/12)

---

## Ziel

Die Python-Contract-Implementierungen in **Solidity** übersetzen für eine echte EVM-kompatible On-Chain Deployment-Option auf der A-TownChain EVM-Schicht.

---

## Contract-Spezifikationen

### ATCToken.sol (ATC-8300)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/Pausable.sol";

contract ATCToken is ERC20, Ownable, Pausable {

    uint256 public constant MAX_SUPPLY       = 21_000_000 * 10**18;
    uint256 public constant HALVING_INTERVAL = 210_000;
    uint256 public constant INITIAL_REWARD   = 50 * 10**18;

    mapping(address => bool) public miners;

    constructor() ERC20("A-Town Coin", "ATC") Ownable(msg.sender) {}

    function mint(address to, uint256 amount)
        external onlyMiner whenNotPaused {
        require(totalSupply() + amount <= MAX_SUPPLY, "Max supply");
        _mint(to, amount);
    }

    function getBlockReward(uint256 blockHeight)
        public pure returns (uint256) {
        uint256 halvings = blockHeight / HALVING_INTERVAL;
        return halvings >= 64 ? 0 : INITIAL_REWARD >> halvings;
    }

    function burn(uint256 amount) external {
        _burn(msg.sender, amount);
    }

    modifier onlyMiner() {
        require(miners[msg.sender], "Not a miner");
        _;
    }
}
```

### Shivamon.sol (ATC-9000)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

contract Shivamon is ERC721 {
    using Counters for Counters.Counter;
    Counters.Counter private _tokenIds;

    uint256 public constant MAX_SUPPLY  = 9900;
    uint256 public          mintingFee  = 10 * 10**18; // 10 ATC

    struct ShivamonData {
        uint8   element;    // 0=Fire 1=Water 2=Earth 3=Air 4=Shadow 5=Neon 6=Quantum
        uint8   rarity;     // 0=Common ... 5=Genesis
        uint8   level;
        uint16  generation;
        uint32  hp; uint32 attack; uint32 defense;
        uint32  speed; uint32 special;
        bytes32 dnaHash;
        uint256 mintedAt;
    }

    mapping(uint256 => ShivamonData) public shivamonData;

    ATCToken public atcToken;

    constructor(address _atcToken) ERC721("Shivamon", "SHV") {
        atcToken = ATCToken(_atcToken);
    }

    function mint(address to, uint8 element, uint8 rarity)
        external returns (uint256) {
        require(_tokenIds.current() < MAX_SUPPLY, "Max supply");
        require(atcToken.transferFrom(msg.sender, address(this), mintingFee),
                "ATC payment failed");

        _tokenIds.increment();
        uint256 newId = _tokenIds.current();
        _mint(to, newId);

        bytes32 dna = keccak256(abi.encodePacked(to, newId, block.timestamp));
        uint256 d   = uint256(dna);

        shivamonData[newId] = ShivamonData({
            element: element, rarity: rarity, level: 1, generation: 1,
            hp:      uint32(50 + (d & 0xFF) % 100),
            attack:  uint32(40 + ((d >> 8) & 0xFF) % 80),
            defense: uint32(40 + ((d >> 16) & 0xFF) % 80),
            speed:   uint32(35 + ((d >> 24) & 0xFF) % 70),
            special: uint32(45 + ((d >> 32) & 0xFF) % 90),
            dnaHash: dna,
            mintedAt: block.timestamp
        });
        return newId;
    }
}
```

---

## Aufgaben

- [ ] `blockchain/contracts/solidity/` Verzeichnis anlegen
- [ ] `ATCToken.sol` schreiben + Tests
- [ ] `Shivamon.sol` schreiben + Tests
- [ ] `ATCGovernance.sol` schreiben + Tests
- [ ] Hardhat-Projekt einrichten (`hardhat.config.js`)
- [ ] Deployment-Script für Testnet
- [ ] ABI-Export nach `config/abis/`
- [ ] Backend: Web3.py On-Chain Queries
- [ ] Test Coverage > 90%

---

## Akzeptanzkriterien

- [ ] Alle Contracts deployen auf Sepolia Testnet
- [ ] ATCToken: Transfer, Mint, Burn, Halving korrekt
- [ ] Shivamon: Mint, Transfer, DNA korrekt
- [ ] ABI im Backend integriert
- [ ] Test Coverage > 90%
# 📄 Issue #13 — ATC Marketplace (Shivamon kaufen & verkaufen)

> **Labels:** enhancement · game · marketplace · priority:medium
> **Priorität:** 🟡 Medium · **Milestone:** v2.2.0
> **Referenz:** [GitHub Issue #13](https://github.com/ShivaCoreDev/a-townchain-os/issues/13)

---

## Ziel

Dezentraler NFT-Marktplatz im ShivaOS Dashboard — Shivamon listen, kaufen, verkaufen und Angebote machen. Alle Trades werden in ATC abgewickelt.

---

## Marketplace-Mechanismus

```
Seller listet Shivamon für 500 ATC
  └─→ NFT wird im Contract gesperrt (escrow)
  └─→ Listing erscheint im Marketplace

Buyer klickt "Kaufen"
  └─→ 500 ATC Transfer: Buyer → Seller (abzgl. 2.5% Fee → Treasury)
  └─→ NFT Transfer: Escrow → Buyer
  └─→ Listing wird entfernt

Seller bricht ab
  └─→ NFT aus Escrow zurück an Seller
  └─→ Listing wird entfernt
```

---

## Contract-Methoden

```python
class MarketplaceContract:

    def list_for_sale(self, token_id: str, seller: str,
                      price_atc: float) -> dict: ...
    # Sperrt NFT in Escrow, erstellt Listing

    def buy(self, token_id: str, buyer: str) -> dict: ...
    # Prüft Balance, führt ATC + NFT Transfer durch, zieht 2.5% Fee

    def cancel_listing(self, token_id: str, seller: str) -> dict: ...
    # Gibt NFT aus Escrow zurück

    def make_offer(self, token_id: str, buyer: str,
                   offer_atc: float) -> dict: ...
    # Legt ein Angebot ab (Seller kann annehmen)

    def get_listings(self, element=None, rarity=None,
                     min_price=None, max_price=None) -> list: ...

    def get_stats(self) -> dict: ...
    # Floor Prices, Volumen, Top Sales
```

### Listing-Datenmodell

```python
@dataclass
class Listing:
    listing_id:  str        # "LST-" + SHA-256[:12]
    token_id:    str        # SHV-...
    seller:      str        # ATC-Adresse
    price_atc:   float
    shivamon:    dict       # Snapshot des NFT zum Listing-Zeitpunkt
    listed_at:   int
    expires_at:  int        # Optional: 30 Tage
    status:      str        # "active" | "sold" | "cancelled"
```

---

## Frontend-Layout

```
🛒 MARKETPLACE

Filter: [Element ▼] [Rarity ▼] [Preis: 0 — 9999] [Sortierung ▼]

Stats: Floor: 45 ATC · 24h Volumen: 3.420 ATC · Listings: 127

┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐
│ 🔥       │ │ ⚡       │ │ 🌀       │ │ 💧       │
│Ignarex   │ │Voltrix   │ │Quantrix  │ │Aquarix   │
│Epic      │ │Legendary │ │Rare      │ │Genesis✨ │
│ATK: 180  │ │ATK: 290  │ │ATK: 140  │ │ATK: 450  │
│          │ │          │ │          │ │          │
│ 350 ATC  │ │ 1200 ATC │ │  89 ATC  │ │4500 ATC  │
│[KAUFEN]  │ │[KAUFEN]  │ │[KAUFEN]  │ │[KAUFEN]  │
└──────────┘ └──────────┘ └──────────┘ └──────────┘
```

---

## Aufgaben

- [ ] `blockchain/contracts/marketplace/marketplace_contract.py`
- [ ] Escrow-Mechanismus (NFT im Contract sperren)
- [ ] 2.5% Marketplace-Fee → ATC Treasury
- [ ] `backend/api/routes/marketplace_routes.py`
- [ ] Filter: Element · Rarity · Preis-Range · Level
- [ ] Frontend Marketplace-Seite (Sidebar: `🛒 Market`)
- [ ] Eigene Listings verwalten (Preis ändern, abbrechen)
- [ ] Floor Price + Handelsvolumen Stats
- [ ] Offer-System (Gegenangebote)
- [ ] Tests: `tests/test_marketplace.py`

---

## Akzeptanzkriterien

- [ ] Listing, Kauf und Abbrechen funktionieren korrekt
- [ ] ATC wird korrekt transferiert (inkl. 2.5% Fee)
- [ ] NFT geht an Käufer, ATC an Verkäufer
- [ ] Filter funktionieren
- [ ] Floor Price und Volumen werden korrekt berechnet
# 🌐 Issue #14 — Bootstrap Node — P2P Discovery Service

## Status: ✅ ABGESCHLOSSEN

**Milestone:** v2.2.0  
**Abhängigkeit:** Keine  
**Abhängig von:** Issue #14 wird von #15, #16, #17 benötigt  

---

## 📋 Aufgabenbeschreibung

Bootstrap Node ist der **Einstiegspunkt für das P2P-Netzwerk**. Alle neuen Nodes verbinden sich zuerst mit dem Bootstrap-Node, um andere Peers zu entdecken und sich ins Netzwerk zu integrieren.

### Was wurde implementiert:

1. **Node Discovery Service** (`blockchain/nodes/discovery.py`)
   - UDP-basiertes Protokoll für Peer-Discovery
   - ANNOUNCE/PEER_LIST Handshake
   - Peer-Registry Verwaltung
   - Health-Check (entfernt offline Peers)

2. **P2P Network Base** (`blockchain/nodes/p2p_propagation.py`)
   - TCP-basiertes Messaging
   - Block- und TX-Propagation
   - Duplikat-Filter
   - Peer-Verbindungsverwaltung

---

## ✅ Implementierte Komponenten

### 1. NodeDiscovery Klasse

**Datei:** `blockchain/nodes/discovery.py`

```python
class NodeDiscovery:
    """UDP-basierter Node-Discovery Service"""
    
    def __init__(self, node_id: str, my_port: int, is_bootstrap: bool = False)
    def start()                          # Startet Discovery-Service
    def stop()                           # Stoppt Service
    def announce()                       # Meldet Node beim Bootstrap an
    def listen()                         # UDP-Listener
    def health_check()                   # Entfernt offline Peers (alle 30s)
    def get_peers() -> Dict             # Gibt Peer-Liste zurück
    def get_peer_count() -> int         # Anzahl Peers
    def save_peers(path)                # Persistierung in JSON
    def load_peers(path)                # Laden aus JSON
    def get_status() -> dict            # Status-Information
```

**Protokoll (UDP):**

```
ANNOUNCE (Node → Bootstrap):
  {
    "type": "ANNOUNCE",
    "node_id": "peer-001",
    "port": 5105,
    "version": "2.0"
  }

PEER_LIST (Bootstrap → Node):
  {
    "type": "PEER_LIST",
    "peers": {
      "bootstrap-001": {"host": "127.0.0.1", "port": 5005},
      "validator-001": {"host": "127.0.0.1", "port": 5105},
      ...
    }
  }

PING/PONG (Health-Check):
  {"type": "PING"}
  {"type": "PONG"}
```

### 2. P2PNetwork Klasse

**Datei:** `blockchain/nodes/p2p_propagation.py`

```python
class P2PBroadcaster:
    """TCP-basiertes P2P-Netzwerk"""
    
    def __init__(self, node_id: str, port: int, consensus=None)
    def start()                         # Startet TCP-Listener
    def stop()                          # Stoppt Network
    def connect_to_peer(node_id, host, port) -> bool
    def broadcast_block(block: dict)    # Sendet Block an alle Peers
    def broadcast_tx(tx: dict)          # Sendet TX an alle Peers
    def register_handler(msg_type, handler)  # Registriert Custom-Handler
    def get_status() -> dict
```

**Nachrichtentypen (TCP):**

```
NEW_BLOCK:
  {"type": "NEW_BLOCK", "sender": "miner-001", "block": {...}}

NEW_TX:
  {"type": "NEW_TX", "sender": "node-002", "tx": {...}}

GET_BLOCKS:
  {"type": "GET_BLOCKS", "from_height": 1, "to_height": 50}

BLOCKS:
  {"type": "BLOCKS", "blocks": [{...}, {...}]}

GET_HEIGHT:
  {"type": "GET_HEIGHT"}

HEIGHT:
  {"type": "HEIGHT", "height": 1247}

HANDSHAKE:
  {"type": "HANDSHAKE", "node_id": "node-002", "version": "2.0", "chain_height": 1247}
```

---

## 🏗 Architektur

```
Bootstrap Node (is_bootstrap=True)
├── NodeDiscovery (UDP :6005)
│   ├── listen() → empfängt ANNOUNCE
│   ├── _send_peer_list() → sendet PEER_LIST
│   ├── health_check() → entfernt offline Peers
│   └── peers = {node_id → PeerInfo}
│
└── P2PBroadcaster (TCP :5005)
    ├── start_listener() → wartet auf eingehende Verbindungen
    ├── _handle_incoming_connection() → akzeptiert Peers
    ├── broadcast_block() → sendet an alle
    └── peers = {node_id → socket}


Regular Node
├── NodeDiscovery (UDP :6105)
│   ├── announce() → sendet ANNOUNCE an Bootstrap
│   ├── listen() → empfängt PEER_LIST
│   └── peers = [bootstrap-001, validator-001, ...]
│
└── P2PBroadcaster (TCP :5105)
    ├── connect_to_peer() → verbindet sich mit Peers
    ├── handshake() → HTTP-Handshake
    └── broadcast_block() → sendet an verbundene Peers
```

---

## 📝 Konfiguration

**Datei:** `config/settings.json`

```json
{
  "network": {
    "bootstrap_nodes": ["127.0.0.1:5005"],
    "max_peers": 50,
    "ping_interval_sec": 30,
    "peer_timeout_sec": 90,
    "handshake_timeout_sec": 10,
    "discovery_port_offset": 1000
  }
}
```

---

## 🚀 Verwendungsbeispiel

### Bootstrap Node starten:

```python
from blockchain.nodes.discovery import NodeDiscovery
from blockchain.nodes.p2p_propagation import P2PBroadcaster

# Discovery-Service
discovery = NodeDiscovery(
    node_id="bootstrap-001",
    my_port=5005,
    is_bootstrap=True
)
discovery.start()

# P2P Network
p2p = P2PBroadcaster(
    node_id="bootstrap-001",
    port=5005
)
p2p.start()

# Warte auf Peers...
time.sleep(10)
print(f"Connected peers: {discovery.get_peer_count()}")
```

### Regular Node verbindet sich:

```python
# Discovery-Service
discovery = NodeDiscovery(
    node_id="node-002",
    my_port=5105,
    is_bootstrap=False,
    bootstrap_nodes=["127.0.0.1:5005"]
)
discovery.start()

# P2P Network
p2p = P2PBroadcaster(
    node_id="node-002",
    port=5105
)
p2p.start()

# Verbinde mit Peers aus Discovery
time.sleep(2)  # Warte auf ANNOUNCE/PEER_LIST
peers = discovery.get_peers()
for node_id, peer_info in peers.items():
    p2p.connect_to_peer(
        node_id,
        peer_info.host,
        peer_info.port
    )
```

---

## 🧪 Tests

**Datei:** `tests/test_discovery.py` und `tests/test_p2p_propagation.py`

```bash
pytest tests/test_discovery.py -v
pytest tests/test_p2p_propagation.py -v

# Output:
# test_peer_info_to_dict PASSED
# test_discovery_init PASSED
# test_discovery_init_non_bootstrap PASSED
# test_get_peers_empty PASSED
# test_add_peer_manually PASSED
# test_get_status PASSED
# test_peer_persistence PASSED
# test_message_to_bytes PASSED
# test_message_from_bytes PASSED
# test_broadcaster_init PASSED
# test_register_handler PASSED
# ...
```

---

## 📦 Abhängigkeiten

- `socket` (stdlib)
- `json` (stdlib)
- `threading` (stdlib)
- `logging` (stdlib)
- `dataclasses` (stdlib)
- `time` (stdlib)

**Keine externen Abhängigkeiten!**

---

## ✨ Features

✅ UDP-basierte Node-Discovery  
✅ Bootstrap-Peer-Registry  
✅ TCP-P2P-Messaging  
✅ Duplikat-Filter (Message-Hash)  
✅ Health-Check (Peer-Timeout)  
✅ Peer-Persistierung (JSON)  
✅ Logging und Debugging  
✅ Thread-Safe Operationen  
✅ Graceful Error Handling  
✅ Auto-Reconnect nach Neustart  

---

## 🔗 Nächste Schritte

Nach Issue #14 folgt:

- **Issue #15:** Block Propagation — P2P Block Broadcasting ✅ DONE
- **Issue #16:** Initial Sync — Neue Nodes synchronisieren
- **Issue #17:** Longest-Chain-Rule — Fork-Auflösung
- **Issue #18:** Docker Compose — 5-Node Testnet

---

## 📖 Dokumentation

- [TESTNET.md](../architecture/TESTNET.md#3-node-discovery-issue-14) — Vollständige Testnet-Doku
- [TESTNET_INDEX.md](../issues/TESTNET_INDEX.md) — Abhängigkeitsbaum

---

**Implementiert:** 2026-05-22  
**Autor:** ShivaCoreDev × Aurora AI  
**Status:** ✅ Code Complete — Bereit für Testing & Issue #15/16
# 🧪 Issue #20 — API-Gateway Tests

**Erstellt:** 2026-06-03  
**Version:** v2.1.0  
**Status:** 🔄 In Progress  
**Priorität:** High  

## Übersicht

Vollständige Test-Suite für das A-TownChain API-Gateway (Port 4000).

## Zu testende Komponenten

| Modul | Test-Datei | Status |
|---|---|---|
| `gateway/main.py` | `tests/test_gateway.py` | 🔄 In Progress |
| `gateway/router.py` | `tests/test_gateway.py` | 🔄 In Progress |
| `gateway/middleware/auth.py` | `tests/test_gateway.py` | 🔄 In Progress |
| `gateway/middleware/rate_limit.py` | `tests/test_gateway.py` | 🔄 In Progress |
| `gateway/middleware/logger.py` | `tests/test_gateway.py` | 🔄 In Progress |
| `gateway/middleware/signature_verify.py` | `tests/test_gateway.py` | 🔄 In Progress |

## Test-Cases (15 gesamt)

### Health & Routing
- `test_health_endpoint_exists` — GET /health → 200
- `test_health_response_json` — JSON mit status:ok
- `test_unknown_route_returns_404` — 404 für unbekannte Routen

### Auth-Middleware
- `test_missing_token_rejected` — kein Token → abgelehnt
- `test_invalid_token_rejected` — falscher Token → False
- `test_valid_atc_address_format` — ATC-Adresse Format

### Rate-Limit
- `test_rate_limit_import` — Modul importierbar
- `test_rate_limit_counter_increments` — Counter hochzählen
- `test_rate_limit_blocks_excess` — Überschreitung → gesperrt

### Signature-Verify
- `test_signature_middleware_import` — Modul importierbar
- `test_empty_signature_rejected` — leere Signatur → ungültig

### Router
- `test_router_import` — Router importierbar
- `test_expected_routes_registered` — 4 API-Routen vorhanden
- `test_gateway_port_config` — Port 4000 in Config

### Logger
- `test_logger_import` — Modul importierbar
- `test_logger_captures_request` — Request-Logging

## Ausführen

```bash
python -m unittest tests.test_gateway -v
```

## Akzeptanzkriterien
- [ ] Alle 15 Tests grün
- [ ] Coverage ≥ 80%
- [ ] CI/CD Pipeline integriert
- [ ] Kein bestehender Test gebrochen
# 📋 Offene Issues & To-Dos — A-TownChain OS

> Stand: 2026-06-09 | Quelle: GitHub API  
> Gesamt: **18 offen** | **201 offene Sub-Tasks** | **4 geschlossen**

---

## 🔴 Priority HIGH

---

### #22 🚀 KAI-OS v1.3.2-beta — Substrate-Integration + DevOps
**Sprint:** 2.1 | **Milestone:** M1 | **Sub-Tasks:** 2 offen, 4 erledigt

- [x] Substrate Node Template eingebunden
- [x] pallet-poh implementiert (276 Zeilen, 8 Tests)
- [x] pallet-ai-registry implementiert
- [x] pallet-agent-registry implementiert
- [ ] PR `main` → `main` mergen
- [ ] Wiki Auto-Sync Workflow aktivieren (wiki-sync.yml)

---

### #20 🧪 API-Gateway-Tests — Unit & Integrationstests Port 4000
**Dateien:** `gateway/main.py`, `gateway/router.py`, `gateway/middleware/`

- [ ] `tests/test_gateway.py` — Unit-Tests für `create_app()`
- [ ] `tests/test_gateway.py` — Integrationstests für alle Routen
- [ ] Test: Auth-Middleware (gültige/ungültige API-Keys)
- [ ] Test: Rate-Limiter (Burst-Schutz, 100 req/min)
- [ ] Test: Service-Routing zu Core:5000 / Chain:5001 / Wallet:5002 / AI:5003 / Game:5004
- [ ] Test: Request-Logger Ausgabe
- [ ] Test: Signature-Verify Middleware
- [ ] CI: Tests in GitHub Actions einbinden (Coverage-Ziel: >80%)

---

### #17 ⛓ [Testnet] Longest-Chain-Rule — Fork-Auflösung
**Datei:** `shivaos/consensus/shiva_consensus.py`

- [ ] `HybridConsensus.resolve_fork(chain_a, chain_b)` implementieren
- [ ] Fork-Erkennung: zwei Blöcke mit gleichem `prev_hash`
- [ ] Längste gültige Chain gewinnt (kumulative Höhe / Arbeit)
- [ ] Orphan-Block Pool: verworfene Blöcke zwischenspeichern
- [ ] Reorganisation (Reorg): Chain-Swap bei längerer Konkurrenz-Chain
- [ ] Reorg-Limit: max. 100 Blöcke tief
- [ ] Unit-Tests: `test_fork_resolution.py` (min. 5 Szenarien)
- [ ] Dokumentation: Fork-Auflösungs-Diagramm in Wiki
- [ ] Integration mit Block-Propagation (#15)
- [ ] Monitoring: Fork-Events im Dashboard anzeigen

---

### #16 🔄 [Testnet] Initial Sync — Neue Nodes synchronisieren
**Dateien:** `shivaos/net/atcnet.py`, `blockchain/nodes/node.py`

- [ ] `sync_from_peer(peer_address)` — Chain-Download in 50-Block-Batches
- [ ] Sync-Fortschritt im Dashboard anzeigen (%)
- [ ] Checkpoint-System: vertrauenswürdige Block-Hashes hardcoden
- [ ] Parallelisierung: von mehreren Peers gleichzeitig syncen
- [ ] Sync-Abbruch & Resume bei Verbindungsabbruch
- [ ] Validierung jedes heruntergeladenen Blocks (Hash + Signatur)
- [ ] Sync-Timeout: 30s pro Peer
- [ ] Unit-Tests: `test_initial_sync.py`
- [ ] Dokumentation in `docs/architecture/TESTNET.md` aktualisieren

---

### #15 📡 [Testnet] Block Propagation — P2P Block Broadcasting
**Datei:** `blockchain/nodes/p2p_propagation.py` (bereits vorhanden)

- [ ] `broadcast_block(block)` — Block an alle Peers senden
- [ ] `broadcast_tx(tx)` — Transaktion an alle Peers senden
- [ ] Deduplication: bereits gesehene Blöcke ignorieren (Bloom Filter / Set)
- [ ] Retry-Logik bei fehlgeschlagener Übertragung (3 Versuche)
- [ ] Gossip-Protokoll: exponentielles Backoff
- [ ] Peer-Score: unzuverlässige Peers downranken
- [ ] Integration mit Bootstrap Node (#14)
- [ ] Unit-Tests: `test_p2p_propagation.py` erweitern
- [ ] Monitoring: Propagationszeit messen

---

### #14 🌐 [Testnet] Bootstrap Node — P2P Discovery Service
**Datei:** `blockchain/nodes/discovery.py` (bereits vorhanden — vervollständigen)

- [ ] UDP-basierter Node-Announcer finalisieren
- [ ] Bootstrap-Node Config in `config/settings.json` eintragen
- [ ] `NodeRegistry` — aktive Peers verwalten (max. 200)
- [ ] Heartbeat: inaktive Nodes nach 60s entfernen
- [ ] REST-Endpoint: `GET /peers` — aktive Node-Liste zurückgeben
- [ ] Persistenz: bekannte Peers in `data/peers.json` speichern
- [ ] Docker-ready: Bootstrap-Node als separater Service in docker-compose
- [ ] TLS-Verschlüsselung für Peer-Verbindungen
- [ ] Geografische Verteilung: Region-Awareness
- [ ] Unit-Tests: `test_discovery.py` erweitern

---

### #8 🌐 Multi-Node Testnet — P2P Netzwerk live
**Abhängig von:** #14, #15, #16, #17, #18

- [ ] Phase 1: Node-Discovery via UDP Broadcast
- [ ] Phase 2: Peer-Verbindungen aufbauen (TCP, max. 25 Peers pro Node)
- [ ] Phase 3: Block-Sync zwischen Nodes
- [ ] Phase 4: Konsens dezentral erreichen (PoI+PoS)
- [ ] Mindestens 3 Nodes synchronisiert (lokal)
- [ ] End-to-End-Test: TX auf Node A → Bestätigung auf Node C
- [ ] Monitoring: Netzwerk-Topologie visualisieren
- [ ] Stress-Test: 100 TX/s unter Last
- [ ] Dokumentation: Testnet-Setup-Guide schreiben
- [ ] Video-Demo: Testnet live
- [ ] Public Testnet: Bootstrap-Node auf VPS deployen
- [ ] Faucet: Test-ATC automatisch ausgeben
- [ ] Explorer: Testnet-Blöcke live anzeigen (#5)
- [ ] Announcement: Testnet-Launch auf GitHub ankündigen

---

### #3 ⚔️ Shivamon Battle UI — Animierte Kämpfe im Browser
**Dateien:** `frontend/index.html`, `frontend/assets/js/battle.js` (neu)

- [ ] Battle-Seite in `frontend/index.html` (Sidebar: `⚔️ Battle`)
- [ ] Shivamon-Auswahl: eigene Collection vs. Gegner-Token-ID
- [ ] Animierter Kampfablauf — Runde für Runde mit HP-Bars
- [ ] Angriffs-Animationen per CSS/Canvas
- [ ] Battle-Log: Aktionen als Text-Stream
- [ ] Sieg/Niederlage-Screen mit XP-Gewinn
- [ ] Backend: `POST /api/game/battle` Endpoint

---

### #2 🤖 Gemini AI Integration ⚠️ ISSUE SCHLIESSEN
**Status:** ✅ Vollständig implementiert am 2026-06-08

- [x] `backend/api/routes/ai.py` — Gemini API Endpoint
- [x] API-Key Management via `.env`
- [x] Gemini AI Orchestrator v2.1.0
- [x] ATCPromptContext + Cache
- [x] REST Routes vollständig
- [x] Settings UI mit localStorage
- [x] Live-Chat im Dashboard
- [ ] ⚠️ **Issue #2 auf GitHub schließen** (Label `done` vorhanden, Issue noch offen)

---

## 🟡 Priority MEDIUM

---

### #19 📊 [Testnet] Node-Monitoring Dashboard
**Datei:** `frontend/index.html`

- [ ] Node-Monitor Seite (Sidebar: `🌐 Nodes`)
- [ ] Node-Karten: ID · Typ · Online/Offline · Peers · Chain-Height · Mempool
- [ ] Live-Refresh alle 5 Sekunden (WebSocket oder Polling)
- [ ] Sync-Fortschrittsbalken pro Node
- [ ] Verbindungs-Graph: welche Nodes sind verbunden
- [ ] Alert bei Node-Ausfall (rot markieren)
- [ ] Backend: `GET /api/nodes/status` Endpoint
- [ ] Historische Daten: Node-Uptime-Chart (letzte 24h)
- [ ] Export: Node-Status als JSON-Report
- [ ] Responsive Design (Mobile-Kompatibilität)
- [ ] Dark-Mode Unterstützung

---

### #18 🐳 [Testnet] Docker Compose — 5-Node lokales Netzwerk
**Datei:** `docker-compose.testnet.yml`, `Dockerfile`

- [ ] `Dockerfile` für A-TownChain OS Backend
- [ ] `docker-compose.testnet.yml` mit 5 Services (Bootstrap, Validator, Miner, 2x Full Node)
- [ ] Umgebungsvariablen pro Node-Typ (.env.testnet)
- [ ] Internes Docker-Netzwerk `atc-testnet`
- [ ] Volume für Chain-Daten (Persistenz zwischen Restarts)
- [ ] Health-Checks für alle Services
- [ ] `make testnet-up` / `make testnet-down` Shortcut (Makefile)
- [ ] Log-Aggregation: alle Node-Logs in einer Ausgabe
- [ ] Port-Mapping Dokumentation
- [ ] README: Testnet-Start-Guide
- [ ] GitHub Actions: Testnet-Smoke-Test nach jedem Push
- [ ] docker-compose.override.yml für lokale Entwicklung

---

### #13 🛒 ATC Marketplace — Shivamon kaufen & verkaufen
**Datei:** `blockchain/contracts/marketplace/marketplace_contract.py`

- [ ] `list_for_sale(token_id, seller, price_atc)`
- [ ] `buy(token_id, buyer)`
- [ ] `cancel_listing(token_id, seller)`
- [ ] `get_listings()` — alle aktiven Listings
- [ ] `get_listing(token_id)` — einzelnes Listing
- [ ] Auktions-Mechanismus (Highest Bidder nach X Blöcken)
- [ ] Escrow: ATC wird beim Kauf gesperrt
- [ ] Royalties: 2.5% an Original-Ersteller
- [ ] Frontend: Marketplace-Seite (Sidebar: `🛒 Market`)
- [ ] Filter: Preis / Rarity / Element / Level
- [ ] Sortierung: Preis aufsteigend/absteigend
- [ ] Warenkorb / Watchlist
- [ ] Transaktions-Bestätigungs-Dialog
- [ ] Backend: `GET/POST /api/game/marketplace`
- [ ] On-Chain: Sale-Event loggen
- [ ] Tests: `test_marketplace.py`
- [ ] Dokumentation: Marketplace-Guide
- [ ] Gas-Fee Kalkulator
- [ ] Batch-Listing: mehrere Shivamon auf einmal listen
- [ ] Rabatt-System für ATC-Staker
- [ ] Notification: Kauf-Bestätigung per Event

---

### #12 ⛓ Solidity Smart Contracts — On-Chain ATC Token
**Datei:** `blockchain/contracts/solidity/`

- [ ] `ATCToken.sol` — ERC20 (ATC-8300)
- [ ] `Shivamon.sol` — ERC721 NFT (ATC-9000)
- [ ] `ATCGovernance.sol` — DAO Voting (ATC-9900)
- [ ] `GenesisToken.sol` — ATC-001 Genesis
- [ ] `Marketplace.sol` — ERC721 Marketplace
- [ ] Hardhat-Konfiguration (`hardhat.config.js`)
- [ ] Unit-Tests für alle Contracts (`test/*.js`)
- [ ] Deployment-Script (Sepolia / Polygon Mumbai)
- [ ] ABI-Export für Frontend-Integration
- [ ] Etherscan-Verifikation
- [ ] Slither Security-Audit

---

### #11 🥚 Shivamon Breeding — Gen 2 NFT Züchtung
**Datei:** `blockchain/contracts/shivamon/shivamon_contract.py`

- [ ] `breed(parent1_id, parent2_id, owner)` implementieren
- [ ] DNA-Mixing: Stats 50/50 + zufällige Mutation (5% Chance)
- [ ] Breed-Cooldown: 24h zwischen Züchtungen
- [ ] Breeding-Fee in ATC-Token
- [ ] Rarity-System: Common(60%) / Rare(25%) / Epic(10%) / Legendary(5%)
- [ ] Gen-2 NFT minten mit neuer Token-ID
- [ ] Max-Generation: Gen-5 Limit
- [ ] Frontend: Breeding-UI im Dashboard
- [ ] Animations: Ei-Schlüpf-Animation
- [ ] Backend: `POST /api/game/breed`
- [ ] Tests: `test_breeding.py`
- [ ] On-Chain: Breeding-Event loggen
- [ ] Genealogie-Baum: Eltern-Kind-Beziehung anzeigen
- [ ] Seltenheits-Garantie: 1 Epic pro 10 Breeds
- [ ] Breeding-Statistiken im Dashboard
- [ ] Gemeinsames Breeding mit anderem User (PvP Breed)
- [ ] Seasonal Breeding Events (limitierte Rassen)

---

### #9 🏛 Governance Contract (ATC-9900) — DAO Voting
**Datei:** `blockchain/contracts/governance/governance_contract.py`

- [ ] `Proposal` Datenstruktur
- [ ] `create_proposal(title, description, options, voting_period)`
- [ ] `vote(proposal_id, voter, option)` — ATC-gewichtetes Voting
- [ ] `execute_proposal(proposal_id)` — nach Ablauf ausführen
- [ ] Quorum-Prüfung (min. 10% ATC-Supply)
- [ ] Veto-Mechanismus (33% Nein = abgelehnt)
- [ ] Delegation: Voting-Power übertragen
- [ ] Frontend: Governance-Seite (Sidebar: `🏛 DAO`)
- [ ] Proposal-Timeline: Abstimmungsfrist anzeigen
- [ ] Results-Chart: Balkendiagramm der Stimmen
- [ ] On-Chain: Voting-Ergebnis unveränderlich
- [ ] Tests: `test_governance.py`
- [ ] Dokumentation: DAO-Guide
- [ ] Emergency-Shutdown: Multi-Sig Override
- [ ] Time-Lock: 24h zwischen Proposal-Annahme und Ausführung
- [ ] Snapshot: Voting-Power bei Proposal-Erstellung einfrieren
- [ ] Off-Chain Signaling: Meinungsumfrage vor Proposal

---

### #7 📦 Build System — EXE / AppImage Installer
**Datei:** `build/build.py`

- [ ] PyInstaller-Config ausbauen (build.spec)
- [ ] Single-File EXE: Gateway + Backend + Frontend
- [ ] Auto-Start: Gateway bootet beim EXE-Start
- [ ] Embedded Browser (PyWebView) für Dashboard
- [ ] Windows: `.exe` + NSIS-Installer
- [ ] Linux: `.AppImage` Build
- [ ] GitHub Actions: Auto-Build bei Release-Tag

---

### #5 🌐 ATC Blockchain Explorer — Block & TX Browser
**Datei:** `frontend/index.html` (Sidebar: `🔍 Explorer`)

- [ ] Explorer-Seite im Dashboard
- [ ] Block-Liste: Height, Hash, Timestamp, TX-Anzahl
- [ ] TX-Detail: von/an/Betrag/Status
- [ ] Adress-Suche: Balance + TX-History
- [ ] Live-Update: neuer Block → Explorer aktualisiert sich
- [ ] Backend: `GET /api/blockchain/explorer/blocks`
- [ ] Pagination für lange Block-Listen

---

## 🟢 Priority LOW

---

### #10 🌉 Cross-Chain Bridge — ATC ↔ EVM Interoperabilität
**Datei:** `blockchain/bridge/bridge_contract.py` (neu)

- [ ] Lock/Mint Mechanismus implementieren
- [ ] Lock-Contract auf A-TownChain
- [ ] Mint-Contract auf EVM-Seite (Wrapped-ATC)
- [ ] Bridge-Relayer-Service
- [ ] Unterstützte Chains: Sepolia, Polygon Mumbai, BSC Testnet
- [ ] Multi-Sig für Bridge-Operationen (3-of-5)
- [ ] Frontend: Bridge-UI (Sidebar: `🌉 Bridge`)
- [ ] Security-Audit vor Mainnet
- [ ] Rate-Limiting: max. 10.000 ATC pro Stunde
- [ ] Emergency-Pause-Mechanismus
- [ ] Oracle: Kurs-Feed ATC/ETH
- [ ] Dokumentation: Bridge-Guide
- [ ] Tests: `test_bridge.py`
- [ ] Monitoring: Bridge-Volume Dashboard
- [ ] Bug-Bounty für Bridge-Schwachstellen
- [ ] Versicherungsfonds: 5% aller Bridge-Fees
- [ ] Cross-Chain Messaging (nicht nur Token)
- [ ] Layer-2 Integration (Arbitrum/Optimism)

---

## ✅ Geschlossene Issues

| # | Titel | Geschlossen |
|---|-------|-------------|
| #21 | build(deps): flask-cors 4.0.1→6.0.0 | 2026-06-08 |
| #6  | 🔐 ECDSA Signatur — TX-Autorisierung | 2026-05-20 |
| #4  | 💾 NFT Persistenz — SQLite | 2026-05-20 |
| #1  | 🔗 Smart Contract — ATC Token Standards | 2026-05-22 |

---

## 📊 Fortschritt-Übersicht

| Kategorie | Issues offen | Sub-Tasks offen | Sub-Tasks erledigt |
|-----------|-------------|-----------------|-------------------|
| 🌐 P2P / Networking | 5 | 51 | 0 |
| ⛓ Blockchain / Contracts | 4 | 57 | 4 |
| 🎮 Gaming / NFT | 3 | 44 | 0 |
| 🖥 Frontend / UI | 3 | 25 | 0 |
| 🛠 DevOps / Build | 2 | 19 | 0 |
| 🤖 AI / KI | 1 | 1 | 6 |
| **Gesamt** | **18** | **197** | **10** |

*Auto-generiert von Superagent — 2026-06-09*
# 📋 A-TownChain OS — Issue Dokumentation

> Technische Spezifikation aller offenen GitHub Issues
> **Stand:** v2.0.0 Genesis Release · 2026-05-19

---

## Übersicht

| Issue | Titel | Prio | Milestone | Doku |
|-------|-------|------|-----------|------|
| [#1](https://github.com/ShivaCoreDev/a-townchain-os/issues/1) | 🔗 Smart Contract Implementation | 🔴 High | v2.1 | [→](./ISSUE_01_SMART_CONTRACTS.md) |
| [#2](https://github.com/ShivaCoreDev/a-townchain-os/issues/2) | 🤖 Gemini AI Integration | 🔴 High | v2.1 | [→](./ISSUE_02_GEMINI_AI.md) |
| [#3](https://github.com/ShivaCoreDev/a-townchain-os/issues/3) | ⚔️ Shivamon Battle UI | 🔴 High | v2.1 | [→](./ISSUE_03_BATTLE_UI.md) |
| [#4](https://github.com/ShivaCoreDev/a-townchain-os/issues/4) | 💾 NFT Persistenz (SQLite) | 🔴 High | v2.1 | [→](./ISSUE_04_PERSISTENZ.md) |
| [#5](https://github.com/ShivaCoreDev/a-townchain-os/issues/5) | 🔍 ATC Blockchain Explorer | 🟡 Medium | v2.1 | [→](./ISSUE_05_EXPLORER.md) |
| [#6](https://github.com/ShivaCoreDev/a-townchain-os/issues/6) | 🔐 ECDSA Signatur | 🔴 High | v2.1 | [→](./ISSUE_06_ECDSA.md) |
| [#7](https://github.com/ShivaCoreDev/a-townchain-os/issues/7) | 📦 Build System EXE/AppImage | 🟡 Medium | v2.2 | [→](./ISSUE_07_BUILD.md) |
| [#8](https://github.com/ShivaCoreDev/a-townchain-os/issues/8) | 🌐 Multi-Node Testnet | 🔴 High | v2.2 | [→](./ISSUE_08_TESTNET.md) |
| [#9](https://github.com/ShivaCoreDev/a-townchain-os/issues/9) | 🏛 Governance Contract ATC-9900 | 🟡 Medium | v2.2 | [→](./ISSUE_09_GOVERNANCE.md) |
| [#10](https://github.com/ShivaCoreDev/a-townchain-os/issues/10) | 🌉 Cross-Chain Bridge | 🟢 Low | v3.0 | [→](./ISSUE_10_BRIDGE.md) |
| [#11](https://github.com/ShivaCoreDev/a-townchain-os/issues/11) | 🥚 Shivamon Breeding Gen 2 | 🟡 Medium | v2.2 | [→](./ISSUE_11_BREEDING.md) |
| [#12](https://github.com/ShivaCoreDev/a-townchain-os/issues/12) | ⛓ Solidity On-Chain Contracts | 🟡 Medium | v2.2 | [→](./ISSUE_12_SOLIDITY.md) |
| [#13](https://github.com/ShivaCoreDev/a-townchain-os/issues/13) | 🛒 ATC Marketplace | 🟡 Medium | v2.2 | [→](./ISSUE_13_MARKETPLACE.md) |

---

## Milestone-Planung

### v2.1.0 (Priority: High)
`#6 ECDSA` → `#4 Persistenz` → `#1 Smart Contracts` → `#2 Gemini AI` → `#3 Battle UI` → `#5 Explorer`

### v2.2.0 (Priority: Medium)
`#8 Testnet` → `#9 Governance` → `#11 Breeding` → `#12 Solidity` → `#13 Marketplace` → `#7 Build`

### v3.0.0 (Priority: Low / Future)
`#10 Cross-Chain Bridge`

---

## Empfohlene Implementierungsreihenfolge

```
1. #6  ECDSA         ← Sicherheitsfundament
2. #4  Persistenz    ← Datenstabilität
3. #1  Smart Contracts
4. #2  Gemini AI     ← AI-Gehirn
5. #3  Battle UI     ← Game-Feature
6. #5  Explorer      ← Transparenz-Tool
7. #8  Testnet       ← Dezentralisierung
8. #9  Governance    ← DAO
9. #11 Breeding      ← Game-Tiefe
10. #12 Solidity     ← On-Chain
11. #13 Marketplace  ← Ökonomie
12. #7  Build        ← Distribution
13. #10 Bridge       ← Interop
```

---

> **Docs:** `docs/issues/` · **Repo:** [ShivaCoreDev/a-townchain-os](https://github.com/ShivaCoreDev/a-townchain-os)
> **Autor:** ShivaCoreDev × Aurora AI · v2.0.0 · 2026-05-19
# 🌐 Testnet Issues — Index

> Sub-Issues von [Issue #8 — Multi-Node Testnet](https://github.com/ShivaCoreDev/a-townchain-os/issues/8)
> **Milestone:** v2.2.0

| Issue | Titel | Prio | Doku |
|-------|-------|------|------|
| [#14](https://github.com/ShivaCoreDev/a-townchain-os/issues/14) | 🌐 Bootstrap Node — P2P Discovery | 🔴 High | [TESTNET.md#3](../architecture/TESTNET.md#3-node-discovery-issue-14) |
| [#15](https://github.com/ShivaCoreDev/a-townchain-os/issues/15) | 📡 Block Propagation | 🔴 High | [TESTNET.md#4](../architecture/TESTNET.md#4-block-propagation-issue-15) |
| [#16](https://github.com/ShivaCoreDev/a-townchain-os/issues/16) | 🔄 Initial Sync | 🔴 High | [TESTNET.md#5](../architecture/TESTNET.md#5-initial-sync-issue-16) |
| [#17](https://github.com/ShivaCoreDev/a-townchain-os/issues/17) | ⛓ Longest-Chain-Rule | 🔴 High | [TESTNET.md#6](../architecture/TESTNET.md#6-longest-chain-rule-issue-17) |
| [#18](https://github.com/ShivaCoreDev/a-townchain-os/issues/18) | 🐳 Docker Compose Testnet | 🟡 Medium | [TESTNET.md#7](../architecture/TESTNET.md#7-docker-compose-issue-18) |
| [#19](https://github.com/ShivaCoreDev/a-townchain-os/issues/19) | 📊 Node-Monitoring Dashboard | 🟡 Medium | [TESTNET.md#8](../architecture/TESTNET.md#8-node-monitoring-dashboard-issue-19) |

## Implementierungsreihenfolge
```
#14 Bootstrap Node
  └─→ #15 Block Propagation
        └─→ #16 Initial Sync
              └─→ #17 Longest-Chain-Rule
                    └─→ #18 Docker Compose
                          └─→ #19 Monitoring Dashboard
```

→ [Vollständige Testnet-Dokumentation](../architecture/TESTNET.md)
# 🌐 Testnet Issues — Index

> Sub-Issues von [Issue #8 — Multi-Node Testnet](https://github.com/ShivaCoreDev/a-townchain-os/issues/8)
> **Milestone:** v2.2.0

| Issue | Titel | Prio | Doku |
|-------|-------|------|------|
| [#14](https://github.com/ShivaCoreDev/a-townchain-os/issues/14) | 🌐 Bootstrap Node — P2P Discovery | 🔴 High | [TESTNET.md#3](../architecture/TESTNET.md#3-node-discovery-issue-14) |
| [#15](https://github.com/ShivaCoreDev/a-townchain-os/issues/15) | 📡 Block Propagation | 🔴 High | [TESTNET.md#4](../architecture/TESTNET.md#4-block-propagation-issue-15) |
| [#16](https://github.com/ShivaCoreDev/a-townchain-os/issues/16) | 🔄 Initial Sync | 🔴 High | [TESTNET.md#5](../architecture/TESTNET.md#5-initial-sync-issue-16) |
| [#17](https://github.com/ShivaCoreDev/a-townchain-os/issues/17) | ⛓ Longest-Chain-Rule | 🔴 High | [TESTNET.md#6](../architecture/TESTNET.md#6-longest-chain-rule-issue-17) |
| [#18](https://github.com/ShivaCoreDev/a-townchain-os/issues/18) | 🐳 Docker Compose Testnet | 🟡 Medium | [TESTNET.md#7](../architecture/TESTNET.md#7-docker-compose-issue-18) |
| [#19](https://github.com/ShivaCoreDev/a-townchain-os/issues/19) | 📊 Node-Monitoring Dashboard | 🟡 Medium | [TESTNET.md#8](../architecture/TESTNET.md#8-node-monitoring-dashboard-issue-19) |

## Implementierungsreihenfolge
```
#14 Bootstrap Node
  └─→ #15 Block Propagation
        └─→ #16 Initial Sync
              └─→ #17 Longest-Chain-Rule
                    └─→ #18 Docker Compose
                          └─→ #19 Monitoring Dashboard
```

→ [Vollständige Testnet-Dokumentation](../architecture/TESTNET.md)
