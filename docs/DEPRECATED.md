# 📦 DEPRECATED — Veraltete Komponenten

> Dieser Stand: 2026-06-09 · A-TownChain OS v2.0.0

---

## 🗑️ Gelöschte / Ersetzte Dateien

| Datei | Ersetzt durch | Grund |
|-------|--------------|-------|
| `atc_issues_summary.py` (Root) | `tools/atc_issues_summary.py` | Duplikat im Root |
| `ecdsa_final.py` (Root) | `tools/ecdsa_final.py` | Duplikat im Root |
| `ecdsa_impl.py` (Root) | `tools/ecdsa_impl.py` | Duplikat im Root |
| `bootscreen_complete.py` (Root) | `frontend/bootscreen/bootscreen_complete.py` | Duplikat im Root |
| `KAI_OS_SUMMARY.py` | — | Veraltet, durch ECOSYSTEM.md ersetzt |
| `sync_report.html` | — | Temporäres Debugging-File |
| `atc-ui/` (Unterordner) | Eigenes Repo: `atc-ui` | Eigenes Repo |
| `frontend/atc-ui/index.html` | `A-TownChain-Okosystems/atc-ui` | Duplikat |
| `atc-standards/ATC/ATC_STANDARDS.md` | `docs/standards/ATC_STANDARDS.md` | Duplikat |
| `atc-standards/ATS/ATS_STANDARDS.md` | `docs/standards/ATS_STANDARDS.md` | Duplikat |
| `plugins/wallet.py` | `backend/wallet/wallet.py` | Veraltet |
| `docs/README.md` | `README.md` | Duplikat |
| `docs/repo/README.md` | `README.md` | Duplikat |

---

## 🔄 Umbenannte Repositories

| Alt | Neu | Datum |
|-----|-----|-------|
| `shivaos-kernel` | `atc-kernel` | 2026-06-09 |
| `shivamon` | `atc-shivamon` | 2026-06-09 |
| `franchise-factory` | `atc-franchise` | 2026-06-09 |
| `atownchain-whitepaper` | `atc-whitepaper` | 2026-06-09 |
| `ShivaCoreDev/kai-os-wiki` | Archiviert → `a-townchain-os/docs/kai-os-wiki.md` | 2026-06-09 |
| `ShivaCoreDev/franchise-factory-wiki` | Archiviert → `atc-franchise-wiki` | 2026-06-09 |

---

## 🔒 Archivierte Repos (read-only)

| Repo | Redirect |
|------|---------|
| `ShivaCoreDev/kai-os-wiki` | → `A-TownChain-Okosystems/a-townchain-os` |
| `ShivaCoreDev/franchise-factory-wiki` | → `A-TownChain-Okosystems/atc-franchise-wiki` |

---

## ⚠️ Legacy-Code (behalten, aber nicht mehr aktiv weiterentwickelt)

| Datei | Status | Anmerkung |
|-------|--------|-----------|
| `shivaos/` | Legacy | Aufgeteilt in `atc-kernel`, Komponenten in `blockchain/` |
| `tools/ecdsa_final.py` | Legacy | Ersetzt durch `blockchain/wallet/ecdsa.py` |
| `tools/ecdsa_impl.py` | Legacy | Ersetzt durch `blockchain/wallet/ecdsa.py` |
| `frontend/bootscreen/` | Legacy | Prototyp, wird durch atc-ui ersetzt |
| `core/kernel.py` | Merged | Funktionalität in `shivaos/kernel/kernel.py` |
| `blockchain/smart_contracts.py` | Legacy | Ersetzt durch `blockchain/contracts/` Ordner |

---

*Automatisch gepflegt von Aurora AI · Stand: 2026-06-09*
