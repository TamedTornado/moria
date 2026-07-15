# Issue 81 — Transition active objects between authored and derived roots

References: `docs/tdd/data-model.md §Objects, ruin, and dressing`; `docs/tdd/rendering.md §Registered objects and forest scale`.

## Properties that must hold

- For every visible nonruin placement in Near/Middle/Far, exactly one logical root exists: authored iff no dependency delta, otherwise owner-filtered derived at current revision; all authored handles are shared.
- For all ruin placements, presentation is always TerrainChunk voxel-derived and never GLB/authored-root.

## Entity configurations to test

- Intact, object-cell edit, adjacent terrain/ruin halo edit, complete erase/tombstone, repeated edits, exact full dependency revert, eviction/reactivation, LOD transition.
- Two conflicting objects rejected before roots; thousands of identical placements share handles.

## Edge cases and type boundaries

- Stale token/revision/LOD derived result cannot install; transition atomically swaps/removes without duplicate or blank owner.

## Error paths

- No per-instance material/collision mesh/physics; adjacent delta remaining blocks authored restoration.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.

