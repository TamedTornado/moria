# Issue 78 — Extract owner-partitioned terrain meshes

References: `docs/tdd/rendering.md §Terrain extraction` and `§Material-aware dual contouring`.

## Properties that must hold

- For every immutable halo snapshot, terrain/ruin plus per-object filtered payloads are pairwise disjoint and their union equals unfiltered global extraction exactly once.
- For every payload, positions are finite/in bounds+seam margin, indices reference vertices and fit u32, allocations fit u32, topology crosses density 128, and up to four material weights are normalized.

## Entity configurations to test

- Uniform empty/solid, single crossing, singular QEF fallback, organic/rock/ore/masonry, edit cut, terrain-object and ruin-object boundaries, all LODs.
- Same snapshot under task/thread permutations and owner-specific extraction.

## Edge cases and type boundaries

- Nonfinite/singular QEF uses crossing centroid; invalid/oversized index or allocation rejects before upload.

## Error paths

- Snapshot task carries source key/token/revision/LOD and cannot borrow mutable WorldStore; stale install is rejected downstream.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.
