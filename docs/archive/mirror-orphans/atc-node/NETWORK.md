# Persistent ATC Node Network

The normal `atc-node` process owns the persistent L1 network loop.

Runtime path:
`RPC transaction -> mempool -> scheduled proposer -> block broadcast -> block validation/state transition -> validator vote -> weighted quorum finality -> durable storage -> restart recovery`

CI verification target: two real `atc-node` processes, shared deterministic genesis, peer handshake, block synchronization, validator voting, weighted finality, and durable state.
