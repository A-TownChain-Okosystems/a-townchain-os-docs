# NON_CANONICAL — Python-ATCLang (Referenzimplementierung)

> **Status:** REFERENZ, NICHT AUTORITATIV. Keine Produktionsautoritaet, keine TCB-Zugehoerigkeit,
> keine eigenstaendige VM-Autoritaet. Gekennzeichnet am 16.11.2026 auf Grundlage von `SCR-0143`.

## Was hier liegt

`vm/atcvm.py` (48.156 Bytes), `parser/parser.py` (38.219), `compiler/optimizer.py` (23.348),
`compiler/compiler.py` (21.790), `lexer/lexer.py` (21.041), `compiler/type_checker.py` (20.606),
`repl/repl.py`, `parser/ast_nodes.py`, `v03/atclang_v03_features.py` und ein `stdlib/`
(`crypto.py`, `crypto_ext.py`, `wallet.py`, `chain.py`, `primitives.py`, `encoding.py`,
`collections.py`, `math.py`, `string.py`, `io.py`).

## Einstufung nach `ATC-STD-100` v2.0.0

Die Repository-Matrix fuehrt fuer `atc-vm`: „Rust, Sekundaer Python (nur Referenz, nie TCB)".
Dieser Bestand ist genau diese Referenz. Er ist kein Bestandteil der Ausfuehrungskette.

## Warum er nicht autoritativ ist

1. **Kein Buildartefakt, keine Testanbindung.** Kein `requirements.txt`, kein `pyproject.toml`,
   kein `setup.py`, keine Tests, kein README im Verzeichnis.
2. **Keine Einbindung.** Keine Importstelle ausserhalb dieses Verzeichnisses, kein Aufruf aus
   `atclang` (Rust) oder `atc-vm` (Rust), keine Containerbindung.
3. **Zwei Rust-Autoritaeten existieren bereits und sind hier benannt:**
   `atclang/crates/atc-core/src/` (`lexer.rs`, `ast.rs`, `ir.rs`, `lower.rs`, `bytecode.rs`, `artifact.rs`)
   fuer Compiler und Toolchain, `atc-vm/src/vm.rs` fuer die Bytecode-Ausfuehrung. Eine dritte,
   Python-basierte VM-Autoritaet ist ausgeschlossen.

## Konsequenz

- Referenzcode bleibt lesbar und erhalten; keine Loeschung ohne Owner-Entscheidung.
- **Abweichungen zur normativen ATCLang-Spezifikation sind Findings** (ATC-IR-001, ATC-BC-001,
  ATC-VM-001, ATC-ABI-001, ATC-STDLIB-001), nicht Normvarianten.
- Das `stdlib/crypto.py` und `stdlib/wallet.py` dieses Bestands duerfen nicht als Krypto- oder
  Wallet-Autoritaet gelesen werden; dafuer gelten `ATC-CRYPTO-001` und die Rust-Linie in
  `a-townchain/components/node` (DID) sowie `atc-wallet` in Rust.
