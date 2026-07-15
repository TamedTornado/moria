# Issue 54 — Build the bounded immutable object index

References: `docs/tdd/data-model.md §Seed, region, and generated metadata`; `docs/tdd/systems.md §Generation systems/Pure evaluators`.

## Properties that must hold

- For every insertion order, dependency/sample tables and queries are sorted, deduplicated and equal brute-force activation, voxel-sampling, dependency and overlap oracles.
- For every accepted index, limits hold: <=16 cells/object/table, <=128 dependency bricks, <=1024 dependency and <=64 sample members/cell, <=256 broad and <=64 exact edit hits, <=1024 Horizon trees/cell, retained bytes <=16 MiB, zero dependency-coordinate allocation.

## Entity configurations to test

- Empty/single/full checked manifest; placements on grid boundaries; duplicate-cell candidates; maximum exact limits and one above each.
- Nonruin/nonruin and nonruin/ruin-air-carve conflicts with multiple candidate conflicts to verify stable first witness.

## Edge cases and type boundaries

- Any cap or checked-byte overflow rejects rather than truncates and retains no partial index.

## Error paths

- Conflicts report ordered lower/higher IDs and lexicographically first voxel before WorldReady.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.

