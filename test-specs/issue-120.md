# Issue 120 — Snapshot deltas at a fixed revision

References: `docs/tdd/api.md §Save/load protocol`; `docs/tdd/data-model.md §Edit delta set and save file`.

## Properties that must hold

- For every accepted save request, snapshot identity/revision/counts and sorted delta values are fixed at the observed boundary; all later edits leave it and encoded bytes unchanged.
- For every snapshot, base-equal cells, empty bricks and all derived/consumer data are absent.

## Entity configurations to test

- Empty and heavily edited maps; exact reversion before snapshot; edit immediately before/after request boundary; concurrent later commits.
- Evicted/inactive edited bricks and multiple brick/local ordering.

## Edge cases and type boundaries

- Snapshot count/size overflow or inconsistent revision fails save without partial snapshot and live truth remains usable.

## Error paths

- Identity/counts handed to encoder and terminal Save evidence must match exactly; stale/derived data inclusion is invariant failure.

## Conformance-harness hook

Where applicable, adapt this case to issue 232's independent dense oracle and rerun with varied completion order, cache residency, batch partitioning, eviction/reactivation, and save/load boundaries.

