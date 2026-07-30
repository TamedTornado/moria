# Issue 467 — Implement the runtime-neutral CPU participant adapter

References: `interfaces.md` TECH-029 and `architecture.md` TECH-016; issue M-036.

## Boundary contract

- Exercise exact callbacks for genesis, tick, snapshot restore, reconstruct, and snapshot export. Calls are nonblocking and receive non-clone immutable leases plus one Moria-owned bounded sink.
- State downcast is available only to the originating adapter; replay lease exposes exact ordered records; collider view exposes source-bound immutable bytes.

## Multi-system scenarios

- Implement both rollback strategies and both failure policies. Run every policy/site row across genesis, tick, correction, restore, device loss/recovery, checkpoint export, and shutdown.
- Prepare two participants from the same `SourceState(n)` with phase-zero input; complete in opposite orders and verify ID-sorted commitments/effects/events.
- Snapshot/export/restore and reconstruct from exact replay bytes reproduce final participant and RNG commitments.

## Failure propagation

- Reject wrong concrete state, participant, source frontier/root, schema, event/effect sequence, capacity, snapshot metadata, RNG count, duplicate/late/cancelled completion, dropped lease, and divergent commitment.
- `NoAdvanceExplicitRetry` retains ready source frontier; `FailWorld` enters failed where specified. No policy publishes without the participant.
- No Tokio/executor type, automatic retry, CPU fallback, same-tick handoff/DAG, behavior vocabulary, or adapter-global canonical state may surface.
