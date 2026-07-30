# Issue 522 — Generate CORDIC, axis, quaternion, and placement oracle data

References: `validation.md` TECH-059 orientation obligations; `architecture.md` TECH-007/071; issue M-080.

## Properties

- Independent reference artifacts fix every CORDIC iteration, axis-normalization comparison, quaternion normalization/composition/inverse, axis-angle step, rational rotation, and placement transform without production implementation reuse.
- For every accepted vector, CPU and WGSL bytes equal the retained oracle bytes.

## Configurations

- Include every quadrant center/midpoint/adjacent word, maximum turn, zero/basis/diagonal/extreme axes, odd half-angle truncation, zero/half/full turns, `(1,1,0,0)`, sign ties, shell boundaries, composition chains, pivots/radii/extreme coordinates.
- Retain per-iteration CORDIC state and independently proved shell, orthogonality, transpose inverse, composition closure, and maximum displacement.

## Error paths

- Retain exact ZeroAxis, UnrepresentableAxis, InvalidOrientation, overflow, and nonrepresentable transform fixtures.
- Regeneration must be deterministic and byte-identical; missing edge/proof, changed table/algorithm identity, or production code import invalidates artifact.
- No float/libm/shader transcendental may define oracle expectations.
