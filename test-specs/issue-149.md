# Issue 149 — Add public-API dig and place controls

References: `docs/tdd/api.md §Debug tool contract`; `docs/tdd/states.md §Debug tool state`.

## Boundary contracts

- Camera public ray_cast(SOLID) produces target; DigSphere centers first hit, PlaceSphere centers one radius along hit normal; submit only through WorldEditWrite and observe public lifecycle.
- Tool cycles only configured placeable materials, publishes inspection/mutation focus and freezes one request's material/shape until primary-ready feedback.

## Multi-system scenarios

- Hit each face/normal and region edge; no hit; dig/place; next/previous wrap; request progresses through multiple batches/primary events.
- Input from keyboard/mouse/gamepad semantic actions; stale ray after edit and concurrent material-cycle attempt.

## Failure propagation

- No hit submits nothing and emits one-frame No target; query/submit rejection shows typed feedback and unfreezes safely without lifecycle assumption.
- Compile/import checks forbid store/voxel/mesh path; air/water/wood/leaf are never selected.

## Ordering guarantees

- Ray -> frozen command/focus -> submit -> accepted/batches -> primary feedback (terminal observed separately). Later input cannot mutate in-flight request.

## Input validation, Transformation correctness, and Rendering states

- Validate semantic action, operation, selected material, hit normal, radius and bounds before submission. Given a hit point/normal, assert the exact dig center or one-radius-offset place center and frozen payload.
- Render distinct no-target, synchronous-error, accepted/pending, primary-ready and terminal states. No-target/error must not look accepted, and primary-ready must not claim terminal completion.

## Conformance-harness hook

Where applicable, adapt this case to issue 232's independent dense oracle and rerun with varied completion order, cache residency, batch partitioning, eviction/reactivation, and save/load boundaries.
