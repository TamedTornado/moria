# Issue 348 — Recovery PR #321: Harden bounded capsule sweep queries

References: `docs/tdd/api.md` §Read-only world observations/capsule limits and result semantics; `docs/tdd/systems.md` §Collision and movement systems.

## Properties that must hold

- For every legal capsule overlap/sweep, broad-phase candidates are deduplicated, exact voxel-AABB results match a brute-force small-world oracle, contacts are coordinate/normal sorted and deduplicated, and `safe_fraction_q16` is the largest collision-free fraction.
- For all legal sweeps, radius is 32..128 Q8, vertical half-segment is at most 192 Q8, Euclidean displacement is at most 3,072 Q8, conservative sweep work is at most 8,192 candidates, overlap work at most 512, and exact hits at most 512.
- For every mask, `SOLID` tests only `solid_collision`, `WATER` only `water_volume`, and their union either; render triangles and cache/activation state cannot change results.

## Entity configurations to test

- Radius 32/128, half-segment 0/192, zero/exact-3,072 displacement, exact-8,192 sweep candidates, exact-512 overlap candidates/hits, full player short sweep, and 0.18 m camera 9 m probe.
- Axis/diagonal/negative-coordinate motion, start inside contact, face/edge/corner grazing, simultaneous normals, region faces, active/inactive/delta cells, water-only/solid-only/combined masks.
- Legal maximum dimensions paired with a displacement that exceeds the combined work bound, proving combined validation rather than independent-only checks.

## Edge cases

- Radius 31/129 returns `LimitExceeded(CapsuleRadius)`; half-segment 193 returns `LimitExceeded(CapsuleHeight)`; displacement 3,073 returns `LimitExceeded(SweepDisplacement)`; 8,193 conservative sweep candidates returns `LimitExceeded(SweepCandidateWork)`; and 513 exact contacts returns `LimitExceeded(ResultCount)`, all before partial output.
- A 513-candidate overlap is rejected before sampling or returning hits. TDD gap: the 512-cell overlap ceiling is explicit, but `QueryLimitKind` defines no overlap-work variant and the text does not assign another exact variant; tests must assert rejection/no partial result until the enum contract is amended, without guessing a variant.
- Overflow in Q8 endpoint, Euclidean length, expanded AABB, or candidate estimate is rejected rather than wrapped.

## Error paths

- Empty/invalid masks return `InvalidInput`; an out-of-bounds query returns `OutOfBounds`; every validation or limit failure returns no partial `SweepResult`/hit vector and performs no authoritative traversal beyond validation.
- Completion order, hash order, and cache state must not alter safe fraction/contact ordering.
