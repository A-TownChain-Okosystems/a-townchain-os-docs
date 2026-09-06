# atc-devtools

Developer Tools für das A-TownChain-Ökosystem.

## Features (geplant)
- VSCode-Extension (Syntax-Highlighting, Autocomplete, Debugger)
- ATCLang-Linter (Style-Checks, Best-Practices)
- Kernel-Debugger (QEMU-Integration, GDB-Stub)
- Profiler (CPU, Memory, Syscall-Trace)
- SDK-Extensions (Rust-Crate, TypeScript-Package)
- Contract-Testing-Framework (Unit, Integration, E2E)
- Migration-Tools (Solidity → ATCLang)

## Architektur
```
atc-devtools/
├── vscode/
│   ├── src/               # VSCode-Extension
│   ├── syntaxes/          # ATCLang Grammar
│   └── package.json
├── linter/
│   ├── src/               # Linter-Logic
│   └── rules/             # Linting-Rules
├── profiler/
│   └── src/               # Profiler
└── tests/
```

## Verwandte Repos
- [atc-ide](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-ide) — Browser-basierte IDE
- [atc-sdk](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-sdk) — SDK für Entwickler


## Abhängigkeiten
- [`A-TownChain-Okosystems/atc-atclang`](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-atclang)
- [`A-TownChain-Okosystems/atc-sdk`](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-sdk)

## Copyright
Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.
