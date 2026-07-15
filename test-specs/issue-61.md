# Issue 61 — Add deterministic bounded ray casting

References: `docs/tdd/api.md §Read-only world observations/ray_cast`.

## Properties that must hold

- For every legal ray/mask, deterministic Q16 3-D DDA visits <=448 voxels within <=64 m and returns the same hit/distance/quantized normal independent of cache/order.
- For every hit, SOLID and WATER masks use only solid_collision and water_volume; no work-sized collection is allocated.

## Entity configurations to test

- Zero, axis-aligned and diagonal rays; negative coordinates; starts on faces/corners/inside matching voxel; min/exact 64 m; combined masks.
- Cold inactive and active/delta worlds compared with brute-force small-world oracle.

## Edge cases and type boundaries

- Zero/unnormalized direction or empty mask => InvalidInput; 64 m + 1 Q8 => RayDistance; potential 449th visit => RayVoxelVisits, all before partial output.

## Error paths

- Out-of-bounds origins/traversal follow documented bounds error and never wrap.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.

