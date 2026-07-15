# Issue 83 — Render static water from generated definitions

References: `docs/tdd/data-model.md §Water body`; `docs/tdd/rendering.md §Water`.

## Properties that must hold

- For every river/lake patch, geometry stays within WaterBodyDef footprint at fixed surface level and is clipped by current carved banks; intersecting edits dirty only affected patches.
- For every water voxel, material_present and water_volume are true at nonzero density while solid_collision is false; all patches share one water material.

## Entity configurations to test

- River/lake interiors, bed/surface/footprint boundaries, exposed/covered bank edit, streamed LOD patches, active/inactive queries.
- Edit nearby outside footprint versus intersecting bank; eviction/reactivation/load.

## Edge cases and type boundaries

- Stale revision/token patch rejects; asset/material failure follows declared fallback/fatal policy.

## Error paths

- No flow/pressure/drain/splash/propagation/neighbour writes/state or collision-wall behavior is registered.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.

