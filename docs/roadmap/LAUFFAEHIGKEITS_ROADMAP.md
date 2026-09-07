# Lauffähigkeits-Roadmap M1-M8 (AD-027, 07.09.2026 — VERBINDLICH)

**Ziel:** Das Ökosystem wird STÜCK FÜR STÜCK lauffähig. Nach jeder Meile läuft
ein echtes, verifizierbares Inkrement — kein Big-Bang (AD-023: Launch-Termin
offen). Jede Meile hat ein hartes Lauffähigkeits-Kriterium (Reality-Check:
Test-/Boot-/Run-Nachweis, nie nur Behauptung).

    M1 Sprache läuft        [L0 atclang]
    M2 Kernel läuft         [L1 atc-shivacore]        (baut auf M1)
    M3 KI läuft             [L2 aurora-ai]            (baut auf M2)
    M4 Blockchain läuft     [L3 a-townchain]         (baut auf M1+M2)
    M5 Betriebssystem läuft [L4 globus-os]            (baut auf M2)
    M6 Dienste laufen       [L5 13 Services]          (baut auf M4, teils M5)
    M7 Spiel läuft          [L6 genesis-*]           (baut auf M4+M5)
    M8 Ökosystem läuft      [L7 a-townchain-os]      (Integration ALLER)

## M1 — Sprache läuft (L0 atclang)

- **Status:** Python-Referenz läuft bereits (compile_source → CompiledModule,
  Pipeline grün). Kein Vault-Bedarf — das Repo steht als einziges aufgebaut.
- **Restschritte (Gates, AD-022):** G1 Language Spec → G2 AST/Semantics →
  G3 ATC-IR → G5 Bytecode Spec → G6 Independent Bytecode Verifier.
- **Kriterium:** Ein ATCLang-Programm wird zu einem ATCA-Artifact kompiliert,
  der Independent Verifier akzeptiert es, die ATVM führt es deterministisch
  aus (End-to-End-Demo hello.atc inkl. State-Transition).
- Rust Canonical Core folgt als Phase 2/3 (Differential Testing gegen Referenz).

## M2 — Kernel läuft (L1 atc-shivacore)

- **Vault-Restauration:** docs/archive/monorepo-full/src/modules/atc-shivacore
  (60 .rs-Dateien, K29-Stand) → atc-shivacore-Repo. Byte-exakt im Vault
  gesichert (AD-020) — Restauration statt Neubau.
- **Kriterium:** 674/674 + Boot L0-L10. **STATUS 07.09. (AD-028):** Verifiziert —
  Kernel-Crate 394/394 + Service-Space-Crate 280/280 = 674/674, Boot gruen,
  Rust 1.98.1 stable. Kernel ist AD-012-konform (nur noch Primitive im Crate).
- Danach Ausbau per AD-013 (SC-001…SC-013: echte Microkernel-Objekte).

## M3 — KI läuft (L2 aurora-ai)

- **Vault-Restauration:** aurora-core/agents/memory/runtime/ai + aistudio
  (377 Dateien) → aurora-ai-Repo.
- **Kriterium:** Ein Agent-Task läuft Ende-zu-Ende durch die Kernel-Event-Bridge
  mit Capability-Token (AD-012/AD-021: Rust Core + Python AI-Layer).
- Ausbaustufe: DA-HEFT-Scheduler, Model Manager (AD-021-Phasenbild).

## M4 — Blockchain läuft (L3 a-townchain)

- **Vault-Restauration:** atc-blockchain/atcnet/wallet/contracts/bridge/
  dex/explorer/… (369 Dateien) → a-townchain-Repo.
- **Kriterium:** Zwei Nodes synchronisieren (Gossip), eine Transaktion wird
  validiert; Genesis Block mit Chain-ID 658467; ein ATCLang-Contract läuft
  als ATCA-Artifact auf der ATVM (nutzt M1). Kernel-Anbindung als System
  Service (AD-012).
- Vault-Wissen: K26-K29 (Genesis/Bridge/Gossip/Audit) sind bereits
  implementiert und getestet gewesen — schnell wieder lauffähig.

## M5 — Betriebssystem läuft (L4 globus-os)

- **Vault-Restauration:** globus-*/bootloader/drivers (12 OS-Module) →
  globus-os-Repo.
- **Kriterium:** globus-init bootet Userspace-Services auf ShivaCore
  (Bootchain AD-013: UEFI→Limine→ShivaCore→globus-init), Initial-Caps
  statt Unix-Root.

## M6 — Dienste laufen (L5, nach M4 parallelisierbar)

- **Reihenfolge:** atc-node zuerst (ATC-01 Core Node) → atc-wallet
  (ATC-Präfix, BIP44) → atc-sdk → atc-explorer → atc-indexer → atc-storage →
  atc-compute → atc-oracle → atc-interop → atc-mining → atc-marketplace →
  atc-launchpad.
- **Kriterium (Kern-Dreiklang):** Full Node verbindet sich zur Chain, Wallet
  erzeugt ATC-Adresse und transagiert, Explorer zeigt die Blöcke live.

## M7 — Spiel läuft (L6)

- **Reihenfolge:** genesis-engine zuerst (Engine-Loop, ECS) →
  genesis-chronicles (Spiel auf Engine).
- **Kriterium:** Spiel-Loop läuft; NFT-Mint (Genesis Chronicles) landet als
  Transaktion auf der Chain (nutzt M4-Contracts + ATC-9000).

## M8 — Ökosystem läuft (L7 a-townchain-os Monorepo)

- **LETZTER Schritt (AD-026):** Launch-Stack-Restauration aus dem Vault
  (docker-compose 10 Dienste, scripts/, config/, tests/) in den Monorepo und
  Verdrahtung aller Layer im Cargo-Workspace (AD-017).
- **Kriterium:** docker-compose up → Stack healthy, alle Dienste gegen die
  laufende Chain von M4; scripts/start.sh bootet das Gesamtsystem.

## Warum diese Reihenfolge logisch ist

1. Jede Stufe liefert ein lauffähiges Inkrement — nichts wartet auf
   "alles fertig".
2. Jede Stufe nutzt die vorherige: Verträge brauchen die Sprache (M1), die
   Chain braucht Verträge + Kernel (M1/M2/M4), Dienste brauchen die Chain
   (M4), das Spiel braucht Engine + Chain (M4/M7).
3. Der Vault (AD-020) macht M2/M4/M5/M8 zu Restaurationen mit Test-Nachweis
   statt teuren Neubauten — die ersten lauffähigen Stufen kommen schnell.
4. Konsistent mit AD-026 (Layer L0-L7) und AD-022 (ATCLang-Gates G0-G19).

**Status-Übersicht (07.09.2026):** M2-GATE VERIFIZIERT (394+280=674/674, AD-028) · M1 in Arbeit (G1 offen) · M2-M7: Basisstaende aus dem Vault restauriert (L1-L6, 13 Repos) — Gate-Verifikationen (674/674 Kernel-Tests, Boot, Node-Sync) im Rebuild-Lauf ausstehend · M8-Stack (Kernstack, docker, scripts, config, tests) verbleibt bis zur Integration im Vault.
