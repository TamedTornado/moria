# Issue 58 — Expose bounded sample and column reads

References: `docs/tdd/api.md §Read-only world observations`.

## Properties that must hold

- For every ready in-bounds read, active and inactive/cache-cold results are identical, committed deltas overlay base, and calls are side-effect-free observationally.
- For all reads, advertised complexity/allocation caps hold: scalar <=64 object candidates and <=16 feature evaluators; columns <=64 runs; identity/bounds/route references remain immutable.

## Entity configurations to test

- Before readiness; all region faces/corners and negative coordinates; inactive procedural, active detailed and edited-evicted cells; water/solid/partial matter.
- Identity, bounds, voxel, point, column, water surface, route and active-band calls at empty/min/max results.

## Edge cases and type boundaries

- Before ready gives NotReady; outside gives OutOfBounds; 65 runs/route points give typed LimitExceeded with no partial result.

## Error paths

- External consumer cannot obtain mutable references, store handles or cache-dependent results.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.

