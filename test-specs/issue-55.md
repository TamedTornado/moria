# Issue 55 — Implement sparse authoritative world storage

References: `docs/tdd/data-model.md §Sparse brick store`, `§Edit delta set and save file`, and `§Relationship invariants`.

## Properties that must hold

- For every in-bounds coordinate and revision, current_voxel equals delta-or-regenerated-base and all observed values belong to one coherent revision.
- For every exact reversion, base-equal cells and empty BrickDelta entries disappear; inactive untouched wilderness owns neither a 4096-voxel array nor mesh-sized truth.

## Entity configurations to test

- Procedural, uniform, detailed boundary, edited inactive, evicted/reactivated, all-base-reverted, and multi-brick worlds.
- Concurrent readers around an atomic batch: observations must be wholly pre- or post-commit.

## Edge cases and type boundaries

- Coordinate/revision overflow or invalid material is rejected without mutation; failed detail materialization cannot corrupt compact truth.

## Error paths

- External compile-fail consumers cannot name WorldStore/BrickRecord or mutate deltas/arrays.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.
