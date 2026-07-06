# A-TownChain OS — Dokumentation

## 📚 Inhaltsverzeichnis

### KAI-OS Wiki (Hauptdokumentation)
➡️ **[kai-os-wiki.md](../../../docs/kai-os-wiki.md)** — Vollständige technische Spezifikation

**Umfang:** 7.109 Zeilen · 43 Kapitel · 26 Sprints · L0–L12 Layer-Architektur

| Bereich | Kapitel | Beschreibung |
|---|---|---|
| Architektur & Grundlagen | 1–10 | System-Design, P2P, Storage, API |
| KI-Integration | 11–15 | Inference, Agenten, Federated Learning |
| Blockchain & Contracts | 4, 16–18 | Substrate, Ink!, Token-Standard |
| Governance & Security | 19–22, **25** | DAO, Incident Mgmt, **L0 Security Layer** |
| OS-Kernel | **24** | Hybrid Micro-Kernel, L1–L5 Multi-NFT |
| DeFi-Layer | **26** | L11: AMM, Lending, Oracle, Flash Loans |
| Gamification | **27** | L12: Soul-Bound-NFTs, Quest-Engine, KI-Rewards |
| Roadmap | 23 | 26 Sprints, 4 Phasen, Jul 2026–Okt 2027 |

### Layer-Architektur (L0–L12)

| Layer | Name | NFT-URI |
|---|---|---|
| L0 | Security *(Querschnitts-Schicht)* | `nft://kai-os/layer/0/security` |
| L1 | Hardware | `nft://kai-os/layer/1/<node-id>` |
| L2 | Micro-Kernel | `nft://kai-os/layer/2/kernel` |
| L3 | KI-Modul | `nft://kai-os/layer/3/ai` |
| L4 | Blockchain-Modul | `nft://kai-os/layer/4/blockchain` |
| L5 | P2P-Netzwerk | `nft://kai-os/layer/5/p2p` |
| L6 | Storage-Modul | `nft://kai-os/layer/6/storage` |
| L7 | API & CLI | `nft://kai-os/layer/7/api` |
| L8 | Governance | `nft://kai-os/layer/8/governance` |
| L9 | Agent | `nft://kai-os/layer/9/<agent-id>` |
| L10 | dApp | `nft://kai-os/layer/10/<dapp-id>` |
| L11 | DeFi | `nft://kai-os/layer/11/defi` |
| L12 | Gamification | `nft://kai-os/layer/12/gamification` |

### Architektur-Dokumente
- [CONSENSUS.md](../../../docs/architecture/CONSENSUS.md) — Hybrid Consensus (PoW/PoS/PoH)
- [GATEWAY.md](../../../docs/architecture/GATEWAY.md) — API Gateway
- [Testnet.md](../../../docs/architecture/TESTNET.md) — Testnet-Setup
- [WALLET_KEYGEN.md](../../../docs/architecture/WALLET_KEYGEN.md) — Wallet Key Generation

### Smart Contract Standards
- [ATC_TOKEN_STANDARD.md](../../../docs/contracts/ATC_TOKEN_STANDARD.md)
- [SHIVAMON_NFT_CONTRACT.md](../../../docs/contracts/SHIVAMON_NFT_CONTRACT.md)

### Offene Issues
📋 [Issues-Übersicht](README.md) — 14 aktive Issues (ECDSA, Governance, Bridge, Marketplace...)

---

*KAI-OS Wiki v1.2.0-alpha — Juni 2026*
