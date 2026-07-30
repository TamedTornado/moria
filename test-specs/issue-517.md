# Issue 517 — Exercise participant ownership and failure policies headlessly

References: `validation.md` TECH-060 participant slice; issue M-122.

## Properties

- For every participant operation and failure-policy row, no participant state/effect/event may install unless its complete source-bound product passes validation and the owning frontier publishes.
- CPU/GPU participant preparation never mutates source token; all outputs write into pre-reserved Moria sinks and install only with a complete frontier bundle.
- Two participants read only SourceState(n); completion timing is irrelevant and events are delivered only after confirmation.

## Policy/state matrix

- For `NoAdvanceExplicitRetry` and `FailWorld`, exercise genesis, ordinary tick, correction, durable restore, device loss/recovery, checkpoint export, and shutdown exactly as TECH-029’s table.
- Assert exact receipt state, retryability/world state, unchanged/terminal frontier, and no publication without every participant.

## Boundary and error paths

- CPU: wrong downcast/source/schema, capacity, dropped lease, duplicate/late/cancelled sink. GPU: mixed ranges/attempts, alias, missing status, incompatible pipeline, overflow, stale generation, bad commitment.
- Snapshot/export/restore/reconstruct and recovery compare every participant/RNG commitment; staged failures reclaim after last use.
- Duplicate event sequence, wrong schema, per-event/aggregate count/byte overflow fail the tick with no event delivery.
- Registration rejects same-tick dependencies; effects order only by `(ParticipantId,local_sequence)`. No DAG, handoff, automatic retry, or fallback is accepted.
