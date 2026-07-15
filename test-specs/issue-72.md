# Issue 72 — Implement deterministic progressive edit staging

References: `docs/tdd/data-model.md §Material and voxel truth`; `docs/tdd/systems.md §Mutation systems/stage_edit_batches`.

## Properties that must hold

- For every sphere/half-open box, staged coordinates equal mathematical membership after clipping; dig never raises and place never lowers density, and no out-of-shape cell changes.
- For every request, canonical brick cursor and batches of <=8 bricks are deterministic across thread/hash order and never mutate starting truth before commit or allocate the whole operation.

## Entity configurations to test

- Sphere radii 0.25/3/16 m; Q8 boundary weights at inner 70% and radius; hardness 1/255; strength 1/255; boxes one voxel/max; region clipping.
- Atomic versus progressive, pause/resume cursor, overlapping requests, water displacement, density reaching zero, exact base reversion.

## Edge cases and type boundaries

- Place air/water/unknown or zero strength rejected upstream; overflow/nonempty-after-clipping failure produces no staged batch.

## Error paths

- Maximum admitted volume proves bounded batch memory; a task/staging failure leaves cursor and truth coherent for retry.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.

