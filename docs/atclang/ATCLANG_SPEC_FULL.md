# 📝 ATCLang — Vollständige Sprachspezifikation
**Version:** v0.2.0-alpha | **Stand:** 09.06.2026 | **Dateien:** `atclang/`

---

## Überblick

ATCLang ist die proprietäre Programmiersprache des A-TownChain Ökosystems. Sie wurde vollständig von Grund auf entwickelt — keine Abhängigkeit von Python-Syntax, Solidity oder anderen Sprachen.

**Design-Ziele:**
- Blockchain-native: Smart Contracts als Erstklassige Sprachkonstrukte
- Statisch typisiert mit Typ-Inferenz
- Kein GC: deterministisches Speichermodell für on-chain Ausführung
- Proprietäre VM: Stack-basiert, 30+ Opcodes

---

## Architektur-Überblick

```
ATCLang Quellcode (.atc)
        │
   [ATCLexer]          ← lexer/lexer.py (562 Zeilen, 67+ Token-Typen)
        │
   Token-Stream
        │
   [ATCParser]         ← parser/parser.py (rekursiver Descent)
        │
   AST (Abstract Syntax Tree)
        │
   [ATCCompiler]       ← compiler/compiler.py
        │
   Bytecode
        │
   [ATCVM]             ← vm/atcvm.py (886 Zeilen, Stack-basiert)
        │
   Ausführungsergebnis
```

---

## Lexer — Token-Typen

**Datei:** `atclang/lexer/lexer.py` (562 Zeilen)

### Schlüsselwörter (Keywords)
```
contract  fn       state    return   if       else
elif      while    for      in       let      const
import    from     use      pub      priv     static
emit      require  assert   revert   self     caller
true      false    null     new      delete   break
continue  match    case     default  type     impl
trait     struct   enum     mod      extern   unsafe
```

### Typen
```
u8   u16  u32  u64   u128  u256
i8   i16  i32  i64   i128
f32  f64
bool  string  bytes  address  hash
Map<K,V>   Vec<T>   Option<T>   Result<T,E>
```

### Operatoren
```
+  -  *  /  %  **         Arithmetik
==  !=  <  >  <=  >=      Vergleich
&&  ||  !                 Logik
&  |  ^  ~  <<  >>        Bitweise
=  +=  -=  *=  /=  %=    Zuweisung
->  =>  ::  .  ..  ...   Sonstige
```

### Literale
```
42          Integer
0xFF        Hex-Integer
3.14        Float
"hello"     String
b"deadbeef" Bytes
true/false  Boolean
0x1A2B...   Address (automatisch erkannt wenn 35 Zeichen mit ATC-Präfix)
```

---

## Parser — Grammatik

**Datei:** `atclang/parser/parser.py` (rekursiver Descent)

### Top-Level Konstrukte

```atclang
// Contract-Deklaration
contract TokenName : ATC-8300 {
    state balances: Map<Address, u128>
    state total_supply: u128 = 1_000_000

    fn transfer(to: Address, amount: u128) -> bool {
        require(self.balances[caller] >= amount, "Insufficient balance")
        self.balances[caller] -= amount
        self.balances[to]     += amount
        emit Transfer(caller, to, amount)
        return true
    }
}
```

```atclang
// Struct
struct Block {
    hash:      bytes32,
    prev_hash: bytes32,
    height:    u64,
    timestamp: u64,
}

// Enum
enum TxStatus {
    Pending,
    Confirmed(u64),   // Block-Höhe
    Failed(string),   // Fehler-Nachricht
}

// Trait
trait Mintable {
    fn mint(to: Address, amount: u128) -> bool
    fn burn(from: Address, amount: u128) -> bool
}
```

### Ausdrücke (Expressions)
```atclang
// Arithmetik
let x: u64 = safe_add(100, 200)    // Overflow-safe Addition
let y: u64 = x * 2 - 50

// Conditional
let result = if x > 100 { "big" } else { "small" }

// Match
match status {
    TxStatus::Pending      => "waiting",
    TxStatus::Confirmed(n) => "confirmed at block " + to_string(n),
    TxStatus::Failed(err)  => "error: " + err,
}

// Map-Zugriff
let bal: u128 = self.balances[addr]
self.balances[addr] = bal + amount

// Methoden-Aufruf
let h: bytes32 = sha256("data")
require(is_valid_address(to), "Invalid address")
```

---

## VM — Virtuelle Maschine

**Datei:** `atclang/vm/atcvm.py` (886 Zeilen)

Die ATCVM ist eine Stack-basierte virtuelle Maschine mit einem separaten Heap für Contract-State.

### VM-Architektur
```
┌─────────────────────────────────────┐
│            ATCVM                    │
├─────────────┬───────────────────────┤
│  Stack      │  Heap                 │
│  (Operanden)│  (Contract-State)     │
├─────────────┴───────────────────────┤
│  Program Counter (PC)               │
│  Call Stack (Funktionsaufrufe)      │
│  Gas Counter                        │
│  Event Log                          │
└─────────────────────────────────────┘
```

### Opcode-Tabelle

| Opcode | Wert | Beschreibung |
|--------|------|-------------|
| `PUSH` | 0x01 | Wert auf Stack legen |
| `POP` | 0x02 | Obersten Wert entfernen |
| `DUP` | 0x03 | Obersten Wert duplizieren |
| `SWAP` | 0x04 | Obere zwei Werte tauschen |
| `ADD` | 0x10 | Addition |
| `SUB` | 0x11 | Subtraktion |
| `MUL` | 0x12 | Multiplikation |
| `DIV` | 0x13 | Division |
| `MOD` | 0x14 | Modulo |
| `EQ` | 0x20 | Gleichheit |
| `NEQ` | 0x21 | Ungleichheit |
| `LT` | 0x22 | Kleiner als |
| `GT` | 0x23 | Größer als |
| `AND` | 0x30 | Logisches AND |
| `OR` | 0x31 | Logisches OR |
| `NOT` | 0x32 | Logisches NOT |
| `JUMP` | 0x40 | Unbedingter Sprung |
| `JUMPI` | 0x41 | Bedingter Sprung |
| `CALL` | 0x50 | Funktionsaufruf |
| `RETURN` | 0x51 | Rückgabe |
| `LOAD` | 0x60 | State laden |
| `STORE` | 0x61 | State schreiben |
| `SHA256` | 0x70 | Hash berechnen |
| `EMIT` | 0x80 | Event emittieren |
| `REQUIRE` | 0x81 | Assertion |
| `REVERT` | 0x82 | Transaktion rückgängig |
| `CALLER` | 0x90 | Aufrufer-Adresse |
| `TIMESTAMP` | 0x91 | Block-Zeitstempel |
| `BLOCKNUM` | 0x92 | Block-Höhe |
| `STOP` | 0xFF | Ausführung beenden |

### Gas-System
```
Basis-Gas pro Transaktion: 21.000
Zusatz pro Opcode:
  - PUSH/POP/DUP:  3 Gas
  - ADD/SUB/MUL:   5 Gas
  - DIV/MOD:       8 Gas
  - SHA256:        30 Gas
  - LOAD/STORE:    200 Gas (State-Zugriff)
  - CALL:          700 Gas
  - EMIT:          375 Gas
```

---

## Standard Library (Stdlib)

**Datei:** `atclang/stdlib/atc_stdlib.py`

| Funktion | Signatur | Beschreibung |
|----------|---------|-------------|
| `sha256` | `(s: string) -> bytes32` | SHA-256 Hash |
| `sha3_256` | `(s: string) -> bytes32` | SHA3-256 Hash |
| `keccak256` | `(s: string) -> bytes32` | Keccak-256 (Solidity-kompatibel) |
| `require` | `(cond: bool, msg: string)` | Assertion mit Fehlermeldung |
| `assert_eq` | `(a, b, msg?)` | Gleichheits-Assertion |
| `safe_add` | `(a: u64, b: u64) -> u64` | Overflow-sichere Addition |
| `safe_sub` | `(a: u64, b: u64) -> u64` | Underflow-sichere Subtraktion |
| `safe_mul` | `(a: u64, b: u64) -> u64` | Overflow-sichere Multiplikation |
| `is_valid_address` | `(addr: string) -> bool` | ATC-Adresse validieren |
| `zero_address` | `() -> address` | Null-Adresse (ATC + 32×'0') |
| `block_timestamp` | `() -> u64` | Aktueller Block-Zeitstempel |
| `block_number` | `() -> u64` | Aktuelle Block-Höhe |
| `emit` | `(name: string, data: map)` | Contract-Event senden |
| `to_json` | `(v: any) -> string` | JSON-Serialisierung |
| `from_json` | `(s: string) -> any` | JSON-Deserialisierung |

---

## Vollständiges Beispiel — ATC-8300 Counter Contract

```atclang
// counter.atc — Einfacher Counter Contract
contract Counter {

    // State-Variablen (persistent on-chain)
    state count:   u64   = 0
    state owner:   Address

    // Konstruktor
    fn init(owner_addr: Address) {
        self.owner = owner_addr
        emit ContractDeployed(owner_addr, block_timestamp())
    }

    // State-mutating Funktion
    fn increment() {
        require(is_valid_address(caller), "Invalid caller")
        self.count = safe_add(self.count, 1)
        emit Incremented(caller, self.count)
    }

    fn increment_by(amount: u64) {
        require(amount > 0, "Amount must be positive")
        require(amount <= 1000, "Max increment: 1000")
        self.count = safe_add(self.count, amount)
        emit IncrementedBy(caller, amount, self.count)
    }

    fn reset() {
        require(caller == self.owner, "Only owner can reset")
        self.count = 0
        emit Reset(caller)
    }

    // Read-only Funktion
    fn get_count() -> u64 {
        return self.count
    }

    fn get_owner() -> Address {
        return self.owner
    }
}
```

---

## REPL — Interaktive Shell

**Datei:** `atclang/repl/repl.py`

```bash
$ python -m atclang.repl

ATCLang REPL v0.2.0 — Tippe :help für Befehle
ATC> let x: u64 = 42
ATC> x * 2
→ 84
ATC> sha256("hello")
→ "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
ATC> :load counter.atc
Geladen: Counter (3 Funktionen)
ATC> Counter.increment()
→ Event: Incremented(ATC000..., 1)
ATC> Counter.get_count()
→ 1
ATC> :exit
```

**REPL-Befehle:**
| Befehl | Beschreibung |
|--------|-------------|
| `:help` | Hilfe anzeigen |
| `:load <datei>` | .atc Datei laden |
| `:contracts` | Geladene Contracts auflisten |
| `:state <contract>` | Contract-State anzeigen |
| `:reset` | Alle State-Variablen zurücksetzen |
| `:exit` | REPL beenden |
