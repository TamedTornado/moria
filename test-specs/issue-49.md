# Issue 49 — Implement deterministic terrain and biome evaluation

References: `docs/tdd/systems.md §Generation systems/Pure evaluators`; `docs/tdd/data-model.md §Column`.

## Properties that must hold

- For every seed/config/coordinate, column, biome, terrain and brick classification output is invariant under call order, thread partition, cache state and hash insertion order.
- For every column, runs are ordered, nonoverlapping, cover full vertical bounds and number <=64; conservative uniform/procedural classification never misclassifies a boundary brick.

## Entity configurations to test

- All region faces/corners, negative coordinates, meadow/forest thresholds, uniform air/water/geology, and surface/material/cave boundary bricks.
- Cold/cache-hit/evicted evaluation and randomized parallel coordinate permutations.

## Edge cases and type boundaries

- Outside bounds returns OutOfBounds without alias; 65-run construction is rejected as ColumnRuns.

## Error paths

- Untouched uniform/procedural classification allocates no [Voxel;4096]; uncertain bounds select detailed rather than a false compact classification.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.

