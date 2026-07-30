# Issue 432 — Validate collision shapes and canonical transforms

References: `collision-presentation.md` TECH-051; issue M-052.

## Input validation

- Accept the closed Point/Aabb/Sphere/Capsule/OrientedBox set with nonnegative extents/radii; accept zero-length capsule and zero radius under their specified degenerate paths.
- Reject negative extent/radius, invalid quaternion, world/request-bound overflow, and any transform intermediate that cannot fit the declared integer wire.

## Transformation correctness

- Transform points/endpoints through exact inverse placement; transform AABB orientation as inverse volume orientation and oriented box as normalized inverse-volume × shape.
- A cell uses `[E*x,E*(x+1))` per axis: point high planes are excluded; positive-size touching is overlap.
- Fractions are reduced with positive denominators; contact point/normal conversion to world space follows exact rational rotation, ties-even reduction, and directed-normal renormalization.
- `TimeOfImpactWire` accepts `0..=0x1_0000_0000`; world contact point is placement raw `i32[3]`, normal is Q1.14 `i16[3]`.

## Edge and error paths

- Cover translated/rotated dynamic volumes, pivot edges, extrema, closed-low/open-high points, face/edge/corner touches, exact time one, zero delta, and directed-normal sign preservation.
- Overflow, zero rotated normal, failed shell, invalid rational/singular case, or unrepresentable contact yields typed collision arithmetic failure: query unavailable or tick `FailedNoAdvance`, never a fabricated no-hit.

Rendering is outside this component.
