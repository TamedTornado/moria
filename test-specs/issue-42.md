# Issue 42 — Add coordinate, material, and voxel truth types

References: `docs/tdd/data-model.md §Coordinate and scalar conventions` and `§Material and voxel truth`.

## Input validation and properties

- For every Q8 point, voxel conversion uses floor division, including all six faces adjacent to zero; min bounds are accepted and max-exclusive bounds rejected without wrap/alias.
- For every Voxel, size is exactly 4 bytes; material IDs 0..13 map to the canonical keys/classes; material_present, water_volume, and solid_collision use their separate definitions and density 128 threshold.

## Transformation correctness and entity configurations

- Coordinates at region corners/faces, ±1 Q8 around voxel and brick boundaries; density 0,1,127,128,255 for air, water, and each solid class.
- Brick local indices 0,15,16,255,256,4095 and invalid coordinates/IDs.

## Edge cases and type boundaries

- Out-of-bounds or overflowing conversions return the documented typed failure and create no BrickCoord.

## Error paths

- Unknown material IDs/config class changes fail validation; partial matter below 128 remains present but non-colliding.

## Rendering states

- This value component has no visual output. Assert observable not-ready, valid empty/partial-water/partial-solid/full-solid, and out-of-bounds states through public samples; none may allocate or expose a render entity.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.
