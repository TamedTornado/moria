# Issue 48 — Load and validate shared render assets

References: `docs/tdd/assets.md §Shared render asset resources`, `§Placeholder strategy`, and `§Import and validation pipeline`.

## Properties that must hold

- For every declared runtime path, validation covers registry/license/budget, format, bounds, attributes, origins/LODs/clips, stamp and shader contracts before readiness.
- For all repeated instances, mesh/material handles come from the unique WorldRenderAssets resource; no spawn creates per-instance assets.

## Entity configurations to test

- Complete production set; each single missing/corrupt asset; declared development fallback; duplicate path; mismatched LOD origin/bounds; missing clip; invalid shader/stamp.
- Two and thousands of repeated placements must retain identical handle IDs and bounded shared allocations.

## Edge cases and type boundaries

- Development emits an observable fallback/warning only where declared; benchmark/release fails on material fallback or missing acceptance art.

## Error paths

- Fatal validation leaves lifecycle non-ready and spawns no authored object roots.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.
