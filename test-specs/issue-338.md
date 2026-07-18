# Issue 338 — Recovery PR #307: Honor configured subsoil depth

References: `docs/tdd/config.md` §Authoritative region configuration; `docs/tdd/data-model.md` §Geological feature and §Material and voxel truth; `docs/tdd/systems.md` §Generation systems/Pure evaluators.

## Properties that must hold

- For every generated land column, the subsoil band begins below the topsoil band and its vertical extent is derived from `terrain.subsoil_depth_q8`; no fixed/magic depth may override the configured value.
- For all equivalent seed/config/coordinates, subsoil material results are deterministic across evaluation order, thread count, cache state, and caller.
- For every change to only `subsoil_depth_q8`, coordinates outside the old/new band difference remain unchanged, while coordinates strictly inside the added/removed band switch between subsoil and the host geology according to feature precedence.

## Entity configurations to test

- Default topsoil depth 256 Q8 and subsoil depth 768 Q8 on flat, sloped, negative-coordinate, cliff, cave-adjacent, water-bed, and object/ruin-overridden columns.
- Two otherwise identical configs with shallow and deep subsoil; sample at least one voxel safely inside their symmetric band difference and assert the exact material transition.
- Column, scalar voxel, and brick classifier observations of the same boundary must agree; boundary-crossing bricks must not be classified as uniform host rock.

## Edge cases

- Depth zero/invalid values are rejected by config validation rather than interpreted as a default; checked Q8 subtraction near region minimum must not underflow or wrap.
- Cave/object/ruin precedence must still win where specified, so those samples are negative controls rather than evidence that depth was ignored.

## Error paths

- Invalid depth or arithmetic overflow returns typed invalid-config/out-of-bounds behavior before world readiness and leaves no partially initialized store.
- TDD gap: inclusive/exclusive sampling at the exact topsoil/subsoil and subsoil/host interfaces is not stated. Tests must use voxel centers strictly inside each band; the TDD should be amended before asserting exact interface-cell ownership.

