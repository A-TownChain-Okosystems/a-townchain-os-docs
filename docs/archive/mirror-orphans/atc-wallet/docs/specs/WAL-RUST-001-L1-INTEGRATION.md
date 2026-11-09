---
spec_id: WAL-RUST-001
title: "Rust Wallet / A-TownChain L1 Integration"
version: 0.1.0
status: IMPLEMENTATION-PENDING-EVIDENCE
repository: atc-wallet
---

# WAL-RUST-001 — Rust Wallet / A-TownChain L1 Integration

## Objective

The wallet Rust core must construct and sign transactions using the byte layout
accepted by the current A-TownChain L1 kernel.

## Current implementation

- Cryptographic key generation: Ed25519 using the operating-system CSPRNG.
- Private key boundary: private material remains inside WalletKey.
- Address payload: RIPEMD160(SHA256(public key)).
- Address encoding: Base58 with double-SHA256 checksum.
- Transaction signing preimage: current L1 kernel ATC-TX-DOMAIN-V2 layout.
- Transaction signature: Ed25519 over the exact signing bytes.
- Signature verification: Ed25519 public-key verification.
- Ethereum RLP/EIP-155/Keccak are not used.

## Important specification conflict

WAL-SIGN-001 is still SPEC-DRAFT and specifies secp256k1/RFC6979/Low-S,
while the current L1 kernel uses Ed25519 verification. This implementation
follows the current executable L1 kernel rather than silently introducing an
incompatible secp256k1 path.

A protocol-level migration to secp256k1 MUST therefore update the L1 kernel,
wallet, SDK and conformance vectors together.

## Address-version boundary

WAL-ADDR-001 requires network-specific version bytes, but the concrete values
are not present in the current frozen runtime contract. The Rust address API
therefore accepts the version byte explicitly instead of inventing a mainnet
value.

## Evidence gates

1. cargo test --manifest-path components/atc-wallet/Cargo.toml
2. cross-component signing-vector test against atc-blockchain kernel
3. transaction submission test against atc-node
4. multi-process network E2E
5. CI evidence: Run-ID + Commit-SHA
