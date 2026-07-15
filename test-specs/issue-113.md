# Issue 113 — Generate tilted strata, ore, and aquifer volumes

References: `docs/tdd/config.md §Geology and feature constraints`; `docs/tdd/data-model.md §Geological feature`.

## Properties that must hold

- For every seed/config/coordinate, tilted strata, finite aquifer and branching iron-vein results are integer/fixed-point deterministic, in bounds and invariant under evaluation order/cache/threading.
- For every coordinate, precedence is cave void > ore > aquifer > host stratum > subsoil/topsoil, and total feature dispatch remains <=16 without volume allocation.

## Entity configurations to test

- Each configured stratum thickness/orientation/host, aquifer band edges, vein branch/host intersections, and coordinates outside each finite bound.
- Curated cliff and cave-wall samples, including cave void adjacent to visible gravel/ore.

## Edge cases and type boundaries

- Invalid orientation/band/bounds/host config fails before readiness; feature 17 is rejected, not truncated.

## Error paths

- Overflow/degenerate fixed-point evaluation returns typed generation failure and cannot partially register features.

## Conformance-harness hook

Where applicable, adapt this case to issue 232's independent dense oracle and rerun with varied completion order, cache residency, batch partitioning, eviction/reactivation, and save/load boundaries.

