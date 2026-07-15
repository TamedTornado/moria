# Issue 51 — Implement analytic object truth and presentation ownership

References: `docs/tdd/data-model.md §Objects, ruin, and dressing`; `docs/tdd/rendering.md §Terrain extraction` and `§Registered objects`.

## Properties that must hold

- For every coordinate, base provenance is total/deterministic and each current solid maps to exactly one TerrainChunk or NonRuinObject owner; nonsolid maps to none without altering collision.
- For every supported object/LOD, lazy dependency membership equals the explicit oracle, extractor reads are within the <=512-offset union stencil, and outside-dependency edits leave payload bytes unchanged.

## Entity configurations to test

- Each analytic shape, ruin sparse stamp including air-carves, object/terrain and object/ruin boundaries, overlapping curator candidates, exact delta reversion.
- Solid/nonsolid water/air/current placed cells for base sources Terrain, Object(id), Ruin(id).

## Edge cases and type boundaries

- Accepted-manifest overlap fails before readiness with stable IDs/first voxel; checked subtraction outside region is false.

## Error paths

- No physics, velocity, health, growth, fire, settling, felling, or dynamics component/resource is registered.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.
