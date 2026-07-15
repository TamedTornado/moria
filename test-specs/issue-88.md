# Issue 88 — Install revision-matched render payloads and account memory

References: `docs/tdd/rendering.md §Edit update path` and `§Graphics memory accounting`.

## Properties that must hold

- For every deferred payload create/update/removal, installation requires current key/token/revision/LOD and swaps atomically; empty payload removes old handle.
- For all counted GPU resources, ledger deduplicates shared IDs, uses checked u64 totals/u32 IDs, records every documented category, and decrements dynamic resources on eviction/free.

## Entity configurations to test

- Terrain, water, object, Horizon aggregate/derived/tombstone, dressing and debug create/replace/empty removal; shared assets referenced many times.
- Repeated band crossings and edits returning to initial band; first vs return steady derived bytes.

## Edge cases and type boundaries

- Stale/duplicate/overflow payload rejects without ledger/install drift; free of unknown/already-freed ID is invariant error.

## Error paths

- Return steady derived bytes must be within 5% and no counted category grows monotonically; untracked_driver_overhead remains true.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.
