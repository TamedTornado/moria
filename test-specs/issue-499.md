# Issue 499 — Track dirty truth and evict only unreachable state

References: `content-persistence.md` TECH-050; issue M-051.

## Valid transitions

- Each confirmed root derives a persistent dirty-key set relative to newest durable checkpoint by stable merge of changed scar/metadata keys.
- Successful checkpoint advances durable root and recomputes remaining dirty keys from later confirmed roots.
- Reclaim occurs only after a logical node/brick is absent from every listed root/pin/submission; base cache additionally requires no interest/admitted-use pin.

## Invalid transitions and guards

- Checkpoint success cannot clear keys changed by later ticks.
- Resident detail eviction cannot remove immutable scar leaf, dirty journal reference, rollback path, or required base identity.
- Presentation resources are independently discardable and never protect or dirty canonical truth.

## Lifecycle and concurrency

- Start checkpoint at t, confirm t+1/t+2 with overlapping/disjoint keys, then commit t; exact later dirty union remains.
- Hold live, rollback, durable/recovery, replay/correction/query/checkpoint, artifact, and GPU pins independently and in combinations; reclaim waits for final one.
- Withdraw interest and evict cache detail, then rematerialize exact scar-over-base state.
- Fail final shutdown checkpoint and verify every dirty root/key summary is returned; no root is reported durable.
- Race durable advancement with later commit/publication; one consistent dirty set is visible.
