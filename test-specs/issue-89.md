# Issue 89 — Define staged edit reconciliation accounting

References: `docs/tdd/api.md §World edit protocol`; `docs/tdd/systems.md §Mutation systems/queue_priority_surface_jobs`.

## Input validation and properties

- For every accepted request/batch, expected keys cover terrain, seams, water, objects, Horizon aggregate/derived/tombstone and dressing including empty removals; duplicates are idempotent, missing keys remain blocking.
- For every primary-ready message, all active-primary keys through its revision are renderer-acknowledged but terminal may remain pending; reconciliation emits exactly once only after all batches/final keys are ready or removed.

## Transformation correctness and entity configurations

- Zero-work, duplicate key/ack, superseded revision, empty removal, hidden/evicted item, multiple batches and multiple requests sharing keys.
- Primary-only complete while distant pending; final complete; later batch supersedes earlier output.

## Edge cases and type boundaries

- Stale/wrong/missing acknowledgement cannot discharge any item; eviction or hiding cannot fake completion.

## Error paths

- No-op follows normal accounting with zero expected presentation keys and exactly one terminal result.

## Rendering states

- Assert pending extraction, GPU-prepared-but-not-queued, primary-ready with background pending, empty-removal acknowledged, fully reconciled, and stale/error states. Only primary-ready and fully reconciled emit their respective public messages; missing work remains blocking.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.
