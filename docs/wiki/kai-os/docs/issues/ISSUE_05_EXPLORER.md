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
