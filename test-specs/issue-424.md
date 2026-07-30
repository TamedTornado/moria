# Issue 424 — Implement canonical placement and quaternion operations

References: `architecture.md` TECH-007 and TECH-071; issue M-016.

## Input validation

- `PlacementFixedFormat::try_new` accepts fractional bits 0 and 16 and cell extents 1 and `i32::MAX`; rejects split 17, extent 0, and extent `2_147_483_648`.
- Quaternion component registration rejects zero norm or failed shell/range checks; axis-angle validates the axis even for an identity angle.
- Placement/domain admission rejects any translated/rotated corner not representable in `i32`.

## Transformation correctness

- `(1,1,0,0)` normalizes to `(11585,11585,0,0)`. Every accepted quaternion satisfies the shell, first-nonzero `(w,x,y,z)` sign rule, composition sequence, and inverse rule.
- Rational quaternion rotation uses the stored norm denominator, displayed matrix order, one ties-even final reduction, and transpose inverse. Generated vectors prove orthogonality and the maximum-radius displacement bound.
- Axis-angle uses unsigned `angle >> 1`; odd low bits truncate, full-turn aliases zero, and maximum word uses `0x7fff_ffff`.
- Local-to-world and inverse placement use the exact pivot/translation/rotation operation order with no reassociation.

## Edge and error paths

- Cover extrema, half ties, zero/full/half turns, negative axes, composition overflow, invalid orientation, zero axis, and unrepresentable axis.
- Compile-fail fixtures reject float constructors/conversions and interchange with cells, density, participant values, or other fixed domains.

Presentation conversion is one-way only and cannot return a canonical type.
