# 🏃 A-TownChain OS — Sprint Roadmap & Todo-Tracker

> **Letztes Update:** 03.08.2026 15:30 (Europe/Berlin)
> **Status:** Verifiziert durch Parser-Lauf, pytest, find/grep
> **Aktiver Sprint:** 2.1 (ATCLang Bootstrap, 93%), 2.3 (Consensus, 95%), 2.4 (Kernel, 85%), 2.6 (Governance, 85%)

---

## Sprint-Übersicht (verifiziert 03.08.2026)

| Sprint | Titel | Status | % | Issues | Module | Verifiziert durch |
|--------|-------|--------|---|--------|--------|------------------|
| 1.1-1.6 | Whitepaper & Forschung | ✅ DONE | 100% | #1-22 | — | Issues geschlossen |
| **2.1** | ATCLang Node Bootstrap | 🔵 AKTIV | 93% | #72,73,74,81 | 30 .py + 198 .atc | Parser 186/198, 51 Tests |
| **2.2** | P2P + Testnet | ✅ DONE | 100% | #75,82,83,84 | 13 .atc | 26 Tests grün |
| **2.3** | Consensus + Gas | 🔵 AKTIV | 95% | #76 | 12 .atc | Code-Analyse |
| **2.4** | Kernel + Syscalls | 🔵 AKTIV | 85% | #77 | 35 .atc | 2 parse errors (:: operator) |
| **2.5** | NFT + Marketplace | ✅ DONE | 100% | — | 13 .atc | Code-Analyse |
| **2.6** | Governance + Security | 🔵 AKTIV | 85% | #78 | 4 .atc | Code-Analyse |
| **2.7** | Testing + CI/CD | 🟡 PLANNED | 10% | #79 | Workflows exist | CI/CD files da, ATCLang Tests fehlen |
| **2.8** | Multi-Node Testnet | 🟡 PLANNED | 15% | — | Launcher+Monitor | .atc vorhanden, Integration fehlt |
| **3.0** | AI Orchestration | 🟡 PLANNED | 20% | #80 | 14 .atc | Module vorhanden, nicht integriert |
| 3.1-3.6 | Alpha Release | 🟡 PLANNED | 0% | #69 | — | — |
| 4.0-4.2 | Mainnet + Future | 🟡 PLANNED | 0% | #70,71 | — | — |

## Blocker (→ Michael)
- **AD-004** Chain-ID 9000 — REOPENED, Entscheidung nötig
- **AD-005** ATC-97 Agent Protocol — Spezifikation unvollständig
- **#79** CI/CD Pipeline Fix — Branch-Protection blockiert API-Push

## Parser-Fix-Fortschritt
| Fix | Status | Dateien gelöst |
|-----|--------|---------------|
| 1. `::` Path-Operator | ✅ GEPUSHT (7228450) | 7 |
| 2. `if let Some(x) = expr` | 🔲 TODO | 4 |
| 3. `Ok(())` / Unit-Typ | 🔲 TODO | 3 |
| 4. `map { k => v }` + `as` Cast | 🔲 TODO | 2 |
| 5. `&mut` Referenz | 🔲 TODO | 1 |
| 6. Tuple in Generics | 🔲 TODO | 1 |
| + `return;` in Inline-Block | 🔲 TODO | 1 |

---
*Aurora · 03.08.2026 15:30 (Europe/Berlin)*
