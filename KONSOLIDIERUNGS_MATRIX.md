# 🗺️ Konsolidierungs-Matrix — Sprint K1 (Repository Audit & Mapping)

> **Deliverable für Issue #85 [K1]** | **Erstellt:** 06.07.2026 | **Autor:** Aurora (aurora-base44-superagent-6a2756186106d6f0fbb105b5)
> **Methodik:** Vollständiges Klonen von `a-townchain-os` + Analyse aller 24 Org-Repos via GitHub API

---

## K1.1 — Datei-Inventar (JSON-Schema-Basis)

| Repo-Kategorie | Anzahl | Repos |
|---|---|---|
| **Aktive Monorepo-Hubs** | 2 | `a-townchain-os` (2.148 KB, Code), `a-townchain-os-docs` (2.376 KB, Doku) |
| **Aktive Satelliten-Doku** | 3 | `kai-os-wiki` (1.692 KB), `a-townchain-os-wiki` (57 KB, Archiv-Snapshot), `atc-whitepaper` (172 KB) |
| **Archiviert (real migriert)** | 19 | 9× `atc-<modul>` + 9× `atc-<modul>-wiki` + `franchise-factory-wiki` |

Datei-Typen in `a-townchain-os` (Hauptrepo, ohne `aistudio/`-Legacy-Ordner):

| Typ | Anzahl |
|---|---|
| `.py` | 84 |
| `.atc` (ATCLang) | 176 |
| `.ts` / `.tsx` | **0** ⚠️ |
| `.js` (frontend) | 3 |

---

## K1.2 — Duplikate (bereits im Reality-Check behoben)

- 145 Dateien (aistudio/temp_repo-Kopien) entfernt — siehe frühere Cleanup-Sessions
- `TESTNET_INDEX_new.md` (byte-identisches Duplikat) in `kai-os-wiki` gelöscht (06.07.2026)
- `ATCLANG_SPEC.md` existiert weiterhin an **5 Pfaden parallel** — offene Entscheidung **AD-009**

---

## K1.3–K1.5 — Abhängigkeitsgraph (Python- & TS-Imports)

### Python — Top-importierte interne Module (aus 84 .py-Dateien)

| Rang | Modul | Import-Häufigkeit |
|---|---|---|
| 1 | `atclang.vm.atcvm` | 13× |
| 2 | `atclang.parser.ast_nodes` | 12× |
| 3 | `atclang.parser.parser` | 6× |
| 3 | `atclang.compiler.compiler` | 6× |
| 5 | `gateway.router` | 5× |
| 6 | `atclang.compiler.type_checker` | 5× |
| 7 | `atclang.lexer.lexer` | 4× |
| 8 | `gateway.middleware.auth` | 3× |
| 8 | `gateway.middleware.rate_limit` | 3× |
| 8 | `core.atcfs` | 3× |
| 8 | `blockchain.wallet.multisig` | 3× |

**Kernbefund:** Der Import-Graph ist **VM-zentriert** — `atclang.vm.atcvm` und `atclang.parser.ast_nodes`
sind die am stärksten verknüpften Module (höchstes Konsolidierungsrisiko bei Refactoring, da am meisten
Abhängige). `gateway.*` bildet einen zweiten, klar abgegrenzten Cluster (Middleware-Kette: auth →
rate_limit → signature_verify → logger).

### TypeScript — ⚠️ Kritischer Befund

**0 von 0 TS/TSX-Dateien mit Imports gefunden — weil `frontend/` faktisch keinen TypeScript-Code enthält.**
`frontend/` besteht nur aus 6 Dateien (README, CSS, 1× `api.js`, 2× HTML) — kein `package.json`,
kein `tsconfig.json`, kein `src/`-Verzeichnis. Das bestätigt den bereits in `AGENT_COORDINATION.md`
dokumentierten Befund zu Issue **#88 [K4]**: Es gibt aktuell **nichts zu "zusammenführen"**, weil die
eigentlichen 148 React/TS-Komponenten unverändert im `aistudio/`-Rohordner liegen und nie nach
`frontend/` überführt wurden. **K4 ist keine Merge-Aufgabe, sondern eine Erst-Integration.**

---

## K1.6 — Dead-Code-Kandidaten (Heuristik: Basisname nirgends importiert)

7 Kandidaten gefunden, nach manueller Prüfung gegen `__init__.py`-Re-Exports reduziert auf:

| Datei | Größe | Status |
|---|---|---|
| `atclang/stdlib/atc_stdlib.py` | 2.353 B | ⚠️ Nicht in `__init__.py` re-exportiert, nirgends importiert |
| `atclang/stdlib/collections_ext.py` | 3.958 B | ⚠️ Nicht in `__init__.py` re-exportiert, nirgends importiert |
| `atclang/stdlib/crypto_ext.py` | 5.131 B | ⚠️ Nicht in `__init__.py` re-exportiert, nirgends importiert |
| `atclang/stdlib/io_ext.py` | 3.485 B | ⚠️ Nicht in `__init__.py` re-exportiert, nirgends importiert |
| `atclang/repl/repl.py` | 6.597 B | ⚠️ Nicht in `__init__.py` re-exportiert, nirgends importiert |
| `atclang/stdlib/chain.py` | 1.216 B | ✅ Falsch-positiv — wird über `__init__.py` re-exportiert |
| `atclang/stdlib/string.py` | 2.363 B | ✅ Falsch-positiv — wird über `__init__.py` re-exportiert |

**Empfehlung:** Die 5 markierten `_ext`-Module + REPL sind vermutlich funktionsfähiger, aber nicht
verdrahteter Code (kein Dead Code im Sinne von "kaputt", sondern "noch nicht angeschlossen"). Vor
Löschung: prüfen, ob sie Teil der geplanten Stdlib-Erweiterung (Sprint 2.1, Standard Library
Expansion) sind — nicht automatisch entfernt.

---

## K1.7 — Konflikt-Liste

| Konflikt | Betroffene Pfade | Status |
|---|---|---|
| `ATCLANG_SPEC.md` 5-fach an unterschiedlichen Pfaden | `atclang/`, `docs/atclang/ATCLANG_SPEC_FULL.md`, `module-docs/atclang/`, `docs/wiki/kai-os/code/atclang/`, `aistudio/temp_repo/atclang/` | ⏳ **AD-009** offen |
| Bridge-Standards-Dopplung (4 Dokumente, ein Thema) | ATC-09, ATC-38, ATC-69, ATC-91 | ⏳ **AD-009** offen |
| Naming-Inkonsistenzen (13 von 19 Konzepten) | siehe `NAMING_CONVENTIONS.md` | ✅ Behoben (05.–06.07.2026) |

---

## K1.8 — Konsolidierungs-Matrix (Quelle → Ziel)

| Quelle (Repo) | Inhalt | Ziel in Monorepo | Migrationsstatus |
|---|---|---|---|
| `atc-kernel` (+ Wiki) | AI-Kernel, LLM-Router | `modules/kernel/` | ✅ Migriert |
| `atc-gateway` (+ Wiki) | API-Gateway, Middleware | `modules/gateway/` + `gateway/` | ✅ Migriert |
| `atc-contracts` (+ Wiki) | Smart Contracts | `modules/contracts/` + `blockchain/contracts/` | ✅ Migriert |
| `atc-franchise` (+ Wiki) | Genesis Franchise Factory | `modules/franchise/` | ✅ Migriert |
| `atc-shivamon` (+ Wiki) | Shivamon NFT/Game-Modul | `modules/shivamon/` | ✅ Migriert |
| `atc-ui` (+ Wiki) | ShivaOS UI-Komponenten | `modules/ui/` | ✅ Migriert |
| `atc-standards` (+ Wiki) | ATC-Standards-Register | `modules/standards/` | ✅ Migriert |
| `atcnet` (+ Wiki) | Netzwerk-Layer | `modules/atcnet/` | ✅ Migriert |
| `atclang` (+ Wiki) | ATCLang-Sprachkern | `atclang/` (Root) | ✅ Migriert |
| `franchise-factory-wiki` | Franchise-Doku | `docs/` (via a-townchain-os-docs) | ✅ Migriert |
| **`aistudio/` (Rohordner in a-townchain-os)** | 148 React/TS-Komponenten, 133 Python-Backend-Dateien | `frontend/src/` (React/TS) + `backend/` (Python) | ❌ **Noch NICHT migriert — das ist K4 (#88) und Teil von K3 (#87)** |

**Kernaussage für K2–K4:** Die eigentliche Backend-Konsolidierung (9 `atc-*`-Modul-Repos → `modules/`)
ist **bereits erledigt**. Die verbleibende, tatsächlich offene Arbeit für K3/K4 ist ausschließlich die
Integration des `aistudio/`-Rohordners (React/TS-Frontend + zugehöriges Python-Backend) in die
Ziel-Struktur `frontend/src/` bzw. `backend/`. K2 (Monorepo-Struktur) ist im Kern ebenfalls bereits
durch die bestehende Ordnerstruktur von `a-townchain-os` erfüllt.

---

*Konsolidierungs-Matrix v1.0 — Aurora (aurora-base44-superagent-6a2756186106d6f0fbb105b5) · 06.07.2026*
