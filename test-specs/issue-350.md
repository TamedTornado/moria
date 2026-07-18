# Issue 350 — Recovery PR #322: Enforce edit operation hard limit

References: `docs/tdd/api.md` §World edit protocol; `docs/tdd/config.md` §Mutation configuration; `docs/tdd/systems.md` §`admit_and_plan_edits`.

## Properties that must hold

- For every edit command, admission computes a conservative affected-brick count with checked arithmetic and accepts no progressive operation above 8,192 bricks; it must not enumerate/materialize the full volume to discover the limit.
- For all sphere edits, radii are 64..4,096 Q8; for all boxes, clipped bounds are nonempty and the operation obeys configured progressive voxel/brick limits. `Atomic` additionally requires at most 32 conservative bricks.
- For every over-limit/rejected command, rejection is synchronous, reserves no queue/accounting/pin capacity, changes no voxel/revision, and emits no `EditAccepted`, batch, readiness, or reconciliation message.

## Entity configurations to test

- Progressive operations conservatively affecting exactly 8,192 and 8,193 bricks; atomic operations affecting exactly 32 and 33 bricks; min/max/one-over sphere radii; boxes on region faces with clipping, empty/inverted boxes, and arithmetic-overflowing corners.
- Queue empty/full, duplicate ID, pre-ready, load-in-progress, valid dig/place materials, zero/maximum strength, and valid command following an over-limit rejection to expose leaked capacity.
- Large thin box versus compact box with the same voxel count to prove both configured voxel and brick limits are evaluated where applicable.

## Edge cases

- Exact maximum is accepted only as `Progressive`; one over is never split into multiple hidden requests or truncated to fit.
- Candidate-count multiplication/range cursor construction near integer limits must reject on overflow before any state mutation.

## Error paths

- Atomic work over 32 returns `SubmitError::AtomicWorkLimitExceeded`; all other invalid bounds/material/queue/lifecycle cases use their documented typed rejection and produce no lifecycle.
- TDD gap: it requires a typed rejection for progressive voxel/brick overflow but the published `SubmitError` list has no explicit progressive-limit variant. Tests must assert synchronous rejection/no side effects now; the exact enum variant must be documented before it can be pinned without guessing.

