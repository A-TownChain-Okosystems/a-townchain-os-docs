# 🌐 A-TownChain Ökosystem — Master Index v2.0.0

> **Org:** [A-TownChain-Okosystems](https://github.com/A-TownChain-Okosystems) · **Version:** v2.0.0 · **Stand:** 2026-06-09
>
> Ein vollständiges KI-Blockchain-Betriebssystem.
> **13 Layer (L0–L12)** · **26 Sprints** · **4 Phasen** · **21 Repositories**

---

## 🗺️ Layer-Architektur

```
┌────────────────────────────────────────────────────────────────────┐
│  L12  atc-shivamon       NFT Gaming, Battle, Breeding              │
│  L11  atc-contracts      DeFi: Token, Staking, Bridge, Oracle      │
│  L10  atc-ui / atc-franchise  Dashboard, Business DAO              │
│  L9   a-townchain-os     KI-Agenten, Orchestrator                  │
│  L8   atc-contracts      Governance DAO (ATC-9900)                 │
│  L7   atc-gateway        API Gateway :4000                         │
│  L6   a-townchain-os     ATCFS Dateisystem                         │
│  L5   atcnet             P2P Netzwerk, Kademlia DHT                │
│  L4   a-townchain-os     Blockchain, Consensus (PoH→PoS→PoW)      │
│  L3   a-townchain-os     KI/AI Registry, Gemini Integration        │
│  L2   atc-kernel         Microkernel, IPC, Prozess-Manager         │
│  L1   (ATPHY Standards)  Hardware-Abstraktion                      │
├────────────────────────────────────────────────────────────────────┤
│  L0   atc-standards      Security S1–S6 (Querschnitt)              │
└────────────────────────────────────────────────────────────────────┘
    atclang ─── Proprietäre Sprache für alle Layer
```

---

## 📦 Code-Repositories

| Repo | Layer | Beschreibung |
|------|-------|-------------|
| [a-townchain-os](https://github.com/A-TownChain-Okosystems/a-townchain-os) | `L2–L4` | **Einziges Haupt-Repo** — KAI-OS Core, AI, Blockchain |
| [atc-kernel](https://github.com/A-TownChain-Okosystems/atc-kernel) | `L2` | ShivaOS Microkernel, IPC, ATCFS |
| [atcnet](https://github.com/A-TownChain-Okosystems/atcnet) | `L5` | P2P Stack, Kademlia DHT, Bootstrap |
| [atc-gateway](https://github.com/A-TownChain-Okosystems/atc-gateway) | `L7` | API Gateway :4000, Circuit-Breaker |
| [atclang](https://github.com/A-TownChain-Okosystems/atclang) | `L2–L4` | Proprietäre Sprache v0.3.0 |
| [atc-contracts](https://github.com/A-TownChain-Okosystems/atc-contracts) | `L4/L11` | Smart Contracts, Token, NFT, Bridge |
| [atc-shivamon](https://github.com/A-TownChain-Okosystems/atc-shivamon) | `L12` | NFT Gaming, Battle Engine |
| [atc-franchise](https://github.com/A-TownChain-Okosystems/atc-franchise) | `L10/L8` | Business DAO, Vault, Revenue-Share |
| [atc-ui](https://github.com/A-TownChain-Okosystems/atc-ui) | `L10` | Neon Dashboard |
| [atc-standards](https://github.com/A-TownChain-Okosystems/atc-standards) | `L0` | Protokoll-Standards |
| [atc-whitepaper](https://github.com/A-TownChain-Okosystems/atc-whitepaper) | `L0` | Whitepaper v2.1.0 |

---

## 📚 Dokumentations-Repositories (Wiki)

| Wiki | Dokumentiert | Layer |
|------|-------------|-------|
| [a-townchain-os-wiki](https://github.com/A-TownChain-Okosystems/a-townchain-os-wiki) | a-townchain-os | L2–L4 |
| [atc-kernel-wiki](https://github.com/A-TownChain-Okosystems/atc-kernel-wiki) | atc-kernel | L2 |
| [atcnet-wiki](https://github.com/A-TownChain-Okosystems/atcnet-wiki) | atcnet | L5 |
| [atc-gateway-wiki](https://github.com/A-TownChain-Okosystems/atc-gateway-wiki) | atc-gateway | L7 |
| [atclang-wiki](https://github.com/A-TownChain-Okosystems/atclang-wiki) | atclang | L2–L4 |
| [atc-contracts-wiki](https://github.com/A-TownChain-Okosystems/atc-contracts-wiki) | atc-contracts | L4/L11 |
| [atc-shivamon-wiki](https://github.com/A-TownChain-Okosystems/atc-shivamon-wiki) | atc-shivamon | L12 |
| [atc-franchise-wiki](https://github.com/A-TownChain-Okosystems/atc-franchise-wiki) | atc-franchise | L10/L8 |
| [atc-ui-wiki](https://github.com/A-TownChain-Okosystems/atc-ui-wiki) | atc-ui | L10 |
| [atc-standards-wiki](https://github.com/A-TownChain-Okosystems/atc-standards-wiki) | atc-standards | L0 |

---

## 🔗 Abhängigkeits-Graph

```
atc-whitepaper
      │
      ▼
atc-standards (L0) ─────────────────────────────┐
      │                                          │
      ▼                                          │
atclang (L2–L4) ──────────────────┐              │
      │                           │              │
      ▼                           ▼              │
atc-kernel (L2)            atc-contracts (L4/L11)│
      │                           │              │
      ▼                           │              │
atcnet (L5)                       │              │
      │                           │              │
      ▼                           ▼              │
a-townchain-os (L2–L4) ◄──────────               │
      │                                          │
      ▼                                          │
atc-gateway (L7) ◄────────────────────           │
      │                           │              │
      ├──────────────┬────────────┘              │
      ▼              ▼                           │
atc-ui (L10)   atc-shivamon (L12)                │
atc-franchise  atc-contracts                     │
               │                                 │
               └── alle nutzen atc-standards ────┘
```

---

## 🚀 Schnellstart

```bash
git clone https://github.com/A-TownChain-Okosystems/a-townchain-os.git
cd a-townchain-os
cp .env.example .env
docker-compose up -d
curl http://localhost:4000/health
```

---

## 📊 Status (2026-06-09)

| Metrik | Wert |
|--------|------|
| Repositories | 21 (11 Code + 10 Wiki) |
| ATX-Module | 186 |
| KAI-OS Wiki | 31 Kapitel |
| Tests | 19/19 ✅ |
| Version | v2.0.0 |
| Sprint | 2.1–2.2 |

---

→ **Org:** [github.com/A-TownChain-Okosystems](https://github.com/A-TownChain-Okosystems)

*Maintained by [A-TownChain-Okosystems](https://github.com/A-TownChain-Okosystems) · Stand: 2026-06-09*
