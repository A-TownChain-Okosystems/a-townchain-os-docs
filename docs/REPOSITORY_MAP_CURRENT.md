# Current Repository Map — A-TownChain-Okosystems

**Verified:** 2026-09-16  
**Source:** live GitHub organization inventory + `atc-standards` registry + `atc-engineering` integration graph

## Active organization repositories

The current organization inventory contains 31 repositories:

`.github`, `atc-standards`, `atclang`, `a-townchain`, `a-townchain-os`, `a-townchain-os-docs`, `atc-shivacore`, `atc-contracts`, `atc-node`, `atc-compute`, `atc-oracle`, `atc-sdk`, `atc-marketplace`, `atc-wallet`, `atc-vm`, `atc-zkp`, `aurora-ai`, `globus-os`, `genesis-engine`, `genesis-chronicles`, `atc-mining`, `atc-indexer`, `atc-interop`, `atc-launchpad`, `atc-explorer`, `atc-storage`, `atc-algorithm`, `atc-ide`, `genesis-franchise-factory`, `atc-engineering`, `demo-repository`.

`demo-repository` is private and therefore has a separate fleet-audit cloning requirement.

## Architecture boundaries

```text
ATCLang
   │ on-chain language / deterministic semantics
   ▼
ATC-VM
   │ execution boundary
   ▼
A-TownChain
   │ consensus / chain protocol
   ├── atc-node
   ├── atc-storage
   ├── atc-indexer
   ├── atc-oracle
   ├── atc-interop
   ├── atc-zkp
   ├── atc-wallet
   └── atc-algorithm / atc-mining

ShivaCore
   │ kernel / capability boundary
   ▼
GlobusOS
   │ OS userspace / platform
   └── Aurora AI

Genesis Engine
   └── Genesis Chronicles / Genesis Franchise Factory

atc-sdk / atc-ide / explorer / marketplace / launchpad
   └── developer, application and integration surfaces

a-townchain-os
   └── system integration / deployment surface
```

## Canonicality rules

1. `atc-standards` is the standards source of truth.
2. `atc-engineering` owns organization-wide audit/integration evidence and fleet controls.
3. Repository-local implementation remains authoritative for code and tests.
4. Historical repository maps dated before the current inventory are archival evidence, not current topology.
5. No repository is considered complete merely because governance files exist; implementation, tests, security evidence, integration evidence and documentation must all be verified.

## Historical-document rule

Documents that still describe the organization as 22, 26, or another historical repository count must be treated as historical snapshots unless explicitly updated. This file is the current navigation point.
