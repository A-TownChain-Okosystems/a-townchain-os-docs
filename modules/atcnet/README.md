# ATCNet — P2P Netzwerk-Layer

**Version:** v1.2.0 | **Status:** Aktiv

## Ubersicht
ATCNet ist der P2P-Layer des A-TownChain OS.
Verwaltet Peer-Discovery, Block-Propagation und Chain-Synchronisation.

## Ports
| Service | Port |
|---------|------|
| P2P     | 9000 |
| RPC     | 9001 |
| WS      | 9002 |

## Dateien
- bootstrap_client.py: Peer-Discovery (Kademlia)
- p2p_node.py: P2P Node Management
- gossip.py: Block/Tx Gossip-Protokoll

## Offene TODOs
- NAT-Traversal vervollstandigen
- Kademlia DHT vollstandig implementieren

_Stand: 2026-06-11 | Aurora Auto-Sync_
