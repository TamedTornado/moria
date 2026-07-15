# Issue 62 — Add bounded capsule overlap and sweep queries

References: `docs/tdd/api.md §Read-only world observations/capsules`; `docs/tdd/systems.md §Collision and movement systems`.

## Properties that must hold

- For every legal capsule query, overlap/sweep results and safe_fraction_q16 match a brute-force voxel-AABB oracle and contacts are sorted/deduplicated by coordinate then normal.
- For every mask, only authoritative solid/water predicates participate; queries never inspect render triangles.

## Entity configurations to test

- Radius 32/128 Q8, half-segment 0/192, displacement zero/exact 3072, exact 8192 sweep candidates, 512 overlap candidates/hits, diagonal/negative/boundary motion.
- Active/inactive/delta worlds; initial overlap, grazing corner, simultaneous normals, water-only and union masks.

## Edge cases and type boundaries

- One below/above each dimension, displacement, work and result cap; 513 contacts returns ResultCount, never truncation.

## Error paths

- Invalid/overflowing shape rejects before traversal and leaves no partial SweepResult.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.

