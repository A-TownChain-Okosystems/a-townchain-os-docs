# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
Migration: In-Memory → SQLite — Issue #4
Einmalig beim ersten Start ausfuehren.
"""
import time
from backend.db.repository import (
    WalletRepository, ShivamonRepository,
    TransactionRepository, BlockRepository
)


def migrate_from_memory(wallet_inst=None, contract_inst=None,
                        consensus_inst=None) -> dict:
    """
    Migriert alle In-Memory-Daten nach SQLite.
    Sicher: bereits vorhandene Eintraege werden uebersprungen.
    """
    stats = {"wallets": 0, "shivamon": 0, "blocks": 0, "errors": 0}

    w_repo = WalletRepository()
    s_repo = ShivamonRepository()
    b_repo = BlockRepository()

    # Wallets migrieren
    if wallet_inst and hasattr(wallet_inst, "accounts"):
        for addr, data in wallet_inst.accounts.items():
            ok = w_repo.save(
                address    = addr,
                public_key = data.get("public_key",""),
                label      = data.get("label","Main Wallet"),
                balance    = float(data.get("balance", 0.0))
            )
            if ok: stats["wallets"] += 1
            else:  stats["errors"]  += 1

    # Shivamon NFTs migrieren
    if contract_inst and hasattr(contract_inst, "tokens"):
        for token_id, nft in contract_inst.tokens.items():
            nft_dict = nft if isinstance(nft, dict) else vars(nft)
            ok = s_repo.save(nft_dict)
            if ok: stats["shivamon"] += 1
            else:  stats["errors"]   += 1

    # Blocks migrieren
    if consensus_inst and hasattr(consensus_inst, "blocks"):
        for block in consensus_inst.blocks:
            block_dict = block if isinstance(block, dict) else vars(block)
            ok = b_repo.save(block_dict)
            if ok: stats["blocks"] += 1
            else:  stats["errors"] += 1

    print(f"Migration abgeschlossen: {stats}")
    return stats
