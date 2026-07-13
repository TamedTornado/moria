# Systems and Execution Contracts

## System sets

`moria-world` declares ordered sets rather than relying on insertion order:

```text
PreUpdate: InputCollect -> PublicCommandAccept
FixedUpdate: Intent -> MoveIntegrate -> VoxelCollision -> MovementFinalize
Update: MutationDispatch -> MutationCollect -> ActivationSelect ->
        StreamDispatch -> WorkerCollect -> CameraAndLight -> Diagnostics
PostUpdate: MutationCommit -> NormalRenderCommit -> DressingCommit -> FrameMetrics
```

The demo input set precedes world command acceptance. Mutation work has exclusive capacity; within normal work, traversal-near, visible-near, mid, and far jobs run in that order. A task result carries source coordinate/object, requested revision, LOD, `generation_fingerprint`, and cancellation epoch; any mismatch discards it without side effects.

## Boot and generation

`build_region_manifest(config) -> Result<RegionManifest, GenerationError>` is pure. It derives broad terrain/biome fields, constrained hydrology, geological feature volumes, object placements, ruin metadata, and named route waypoints. `validate_manifest(manifest, config, query) -> ValidationReport` checks every required generated feature and uses the actual composite base query for clearance/intersection tests.

Boot proceeds incrementally:

1. Read the compiled `BASE_SAMPLER_REVISION`, hash and validate both base-truth assets, then recompute and validate the component and aggregate fingerprints from `config.md`. Because the revision is the first normative member of `generation_fingerprint`, this happens before any compatible-save validation or base sampling. Only after those checks succeed, build the deterministic manifest/object index on a worker.
2. Read and validate the single save file if present; install its indexed deltas before spawn-area queries.
3. Activate the player spawn’s collision and near-render set at mutation/near priority.
4. Spawn the player only after a downward solid query finds the validated meadow start and its capsule clearance is empty.
5. Publish `ControlReady`; stream mid/far representations afterward.

Manifest generation must fit inside the five-second control-ready budget and therefore may construct metadata/indices but may not expand the full voxel region. Failure reports the exact constraint/asset/save cause and enters `BootFailed`.

### Generation properties

- Every column returns a finite surface inside region Y bounds with topsoil/subsoil above host geology except where water, cliff, cave mouth, or ruin rules intentionally cut/replace it.
- River/lake water cells have carved empty/non-solid cells above them and solid basin/channel support below; no water is merely overlaid on uncarved terrain.
- The cave empty volume is connected from mouth to floor and has capsule clearance along validator samples.
- The aquifer band and exactly one named iron-vein segment intersect the cave-route inspection tube; diagnostic metadata identifies both.
- Tilted strata normals at the outcrop exceed the configured minimum departure from vertical layering and are exposed to air along the viewpoint ray.
- Object placement produces both tree species, bushes, boulders, stumps, and scattered rocks in the required biome/route cells; object solid bounds do not block the validated route.
- Exactly one ruin stamp is present, its stair treads/risers remain cut stone, and the configured player can ascend it under the fixed-step movement validator.

## Activation and streaming

`select_active_bricks(focuses, frustum, manifest, config) -> ActivationPlan` is pure. It classifies surface-crossing bricks, visible cave/portal bricks, water, objects, and edited/inspection bricks by the highest requested need. It never expands uniform deep air/stone/water just because a radius intersects them.

The runtime diffs the plan against current `BrickRuntime` entries and dispatches jobs. Each normal job samples no more than one brick plus halo (`5,832` cells). The normal pool has `max(1, logical_cores - 3)` workers and at most two queued/running jobs per worker; selection work beyond that stays as compact coordinates in the activation plan rather than spawning tasks. Normal jobs check a cancellation epoch every 256 samples. Retention hysteresis prevents oscillation. On deactivation, render/cache entities despawn, dense arrays may be dropped/collapsed, and delta entries remain. Returning to a location reproduces the same truth and derived placement.

Far rendering is surface-only; nearby caves are activated by player proximity, the cave portal visibility graph, or an inspect ray. Traversal focus always overrides render LOD so collision/query fidelity remains 25 cm regardless of visible representation.

Properties:

- Moving focus without edits changes only activation/cache/render state, never `sample_voxel`.
- A uniform untouched brick allocates no 4096-element voxel array.
- Static manifest/object indices are built once at boot and reused; they are not reinserted each frame.
- A deactivated edited brick reconstructs byte-identical current samples after reactivation.

## Mutation pipeline

Command acceptance validates and quantizes the sphere, enumerates the integer voxel AABB, then filters voxel cells by sphere overlap. At most one interactive command is in flight. At the maximum radius, the enclosing cell box is at most `25^3 = 15,625` cells, changed cells occupy at most `3^3 = 27` bricks, and the one-brick dependency box is at most `5^3 = 125` bricks. The manifest validator also proves that any eligible radius-3 m sphere depends on at most 24 voxel-object records. The authoritative delta transaction and before/after owner dirty-set calculation run immediately in `MutationDispatch`; phase-immutable water/air samples do not enter the changed set.

Mutation extraction runs on a dedicated project `TaskPool` with exactly two worker threads and preallocated scratch arenas. Normal streaming can neither enqueue there nor consume those threads. Acceptance machines require at least four logical cores; normal work deliberately leaves three logical cores unclaimed (main/render plus the two mutation workers). On command acceptance the mutation epoch cancels queued normal work and causes already-running normal jobs to abandon at their next <=256-cell checkpoint, but mutation correctness does not rely on their cancellation because the pools are separate. A mutation is split into at most four deterministic extraction jobs of at most 32 dependency bricks/186,624 sampled halo cells each. The compiled composite-extraction bound is 531,441 packed 32-byte dual vertices (17,006,112 bytes) and 9,600,000 `u32` indices (38,400,000 bytes) over the 80-cell dependency cube; terrain/object fragments partition that one output and cannot multiply it. Raw replacement is at most 15,625 48-byte instances (750,000 bytes), water replacement is capped at 4,000,000 bytes, and descriptors/alignment are capped at 1,000,000 bytes, for a total 61,156,112-byte prepared bound below the reserved 96 MiB. The two-worker preparation deadline is 12.0 ms on acceptance hardware and is recorded per job; a miss is a release failure.

Collision needs no triangle rebuild: the next fixed tick samples the new delta revision. A `MutationCommitBarrier` captures the union of before/after terrain-brick, object, water, raw-view, and dressing-removal keys visible at acceptance. Dressing is hidden synchronously and regeneration does not block the barrier. Prepared smooth/object primitives are a single topology output partitioned into at most 125 terrain ranges and 24 object ranges; at most 27 water chunks and 27 raw chunks can also change. Range descriptors are batched, not installed as hundreds of independent asset allocations.

Two persistently allocated 48 MiB mutation upload packets and a 224-entry range-swap table are reserved at boot. If diagnostics are enabled their prepared data shares this bound. The mathematical extractor/index bounds fit these packets; exceeding a compile-time bound is an acceptance-mode fatal invariant violation, never an unbounded allocation or a false commit. Normal uploads are suspended while a mutation barrier exists. The exact presented-frame admission schedule for an acceptance on frame `F` is:

1. During `F`, install collision truth, hide invalid dressing, capture keys, signal the exclusive workers, and present the old complete surface. Sampling, delta commit, and dirty detection together are capped at 4 ms main-thread time.
2. Before the render-prepare cutoff for `F+1`, workers must finish packet 0/1 preparation; `PostUpdate F+1` queues packet 0 and at most 112 range descriptors using at most 4 ms main-thread mutation work. No partial new epoch is made visible.
3. `PostUpdate F+2` queues packet 1 and the remaining at most 112 descriptors using at most 4 ms, then flips one visibility epoch so all barrier keys change revision together and emits `SurfaceCommitted` with `presented_frame = F + 2` (or earlier when one packet suffices).

The dedicated two-worker completion before the `F+1` cutoff, packet capacity, per-frame range/byte caps, visibility flip, and 4 ms CPU cap are hard release acceptance invariants measured on each target, not best-effort priority. Missing any deadline fails the run. Old meshes remain visible until the epoch flip, and a stale task can never enter either packet.

If a second scripted benchmark mutation supersedes a still-building artifact, its revision replaces the requested revision and the old result is discarded. Interactive tools receive `Busy` until commit. Worker failure is a fatal diagnostic in acceptance mode; it may not emit a false commit.

Properties:

- Applying dig twice with strength 255 cannot add matter; applying place twice with one material cannot reduce density.
- Changed density remains in 0..255 and density 0 canonicalizes to air.
- Dig changes only `Solid`; `Fluid` and `Empty` samples remain byte-identical and no Product One mutation sequence can create a fluid delta.
- A mutation crossing a brick boundary invalidates both sides’ meshing halos.
- The union of owner-partitioned surface primitives equals one composite extraction exactly; a cross-object/terrain edit and exact revert cause no gap/duplicate and restore the shared base handle.
- Collision at revision R never reads a mesh and reflects R at the first subsequent fixed tick.
- The representative accepted 3 m operation reaches visible commit in at most two presented frames and obeys the frame/main-thread ceilings in `config.md` on acceptance hardware.

## Terrain and object collision

Product One uses a project-owned kinematic capsule controller so collision remains a direct voxel-occupancy query. No physics mesh is authoritative.

Collision detection and response are separate pure stages: `sweep_capsule(query, capsule, displacement) -> Vec<VoxelHit>` enumerates/sorts contacts without interpreting player behavior, while `resolve_kinematic_motion(capsule, velocity, hits, config) -> MotionResolution` applies slide/step response. Static manifest/object spatial indices are built once at boot; local swept-AABB voxel enumeration avoids all-region pairwise work.

For each fixed tick:

1. Normalize camera-relative planar movement intent and choose run/sprint/paddle acceleration.
2. Apply acceleration, gravity, and an edge-buffered jump when grounded.
3. Form the swept capsule AABB for desired displacement and enumerate solid candidate voxels via `overlap_aabb`.
4. Calculate capsule-versus-voxel-AABB time/contact using conservative advancement; select earliest time, breaking ties by voxel coordinate then normal axis.
5. Advance to contact minus epsilon, remove velocity into the normal, and repeat for at most four contacts.
6. For near-vertical obstructions no higher than 0.5 m, test a step-up/forward/down sequence against occupancy; accept only with final clearance and walkable ground.
7. Probe ground and water, finalize transform/velocity, and keep the capsule inside regional X/Z and top/bottom bounds.

The controller evaluates slopes from the local density gradient/contact neighbourhood while occupancy remains the collision authority. A surface is walkable when its normal is within 50 degrees of up. Tunnelling is prevented by swept tests rather than frame-sized overlap correction.

Water contact uses fluid queries and generated surface metadata. When the capsule’s lower portion crosses a static surface and there is water below, vertical motion is critically damped toward the configured surface offset, gravity is suppressed, and planar speed becomes paddle speed. Leaving the water footprint restores normal motion. There is no dive action, underwater mode, current, splash simulation, or fluid displacement.

## Input intent

`InputPlugin` is the only feature that reads `ButtonInput<KeyCode>`, mouse motion, or cursor capture. It writes semantic `ActionState` each rendered frame. Continuous axes/holds replace their current value; edge actions receive a monotonically increasing sequence number and remain buffered until acknowledged.

`PlayerIntent` contains move axis, sprint held, and optional jump edge. `DebugIntent` contains optional dig/place/toggle/save/load edges, radius/material selection, and time-slider adjustment. Movement systems contain no key/button names. Menu/diagnostic cursor control is separate from movement intent even though there is no full menu.

## Camera and underground light

Each frame after authoritative player movement, camera input updates yaw/pitch and computes a desired orbit point. A sphere cast implemented through `WorldQuery::sphere_contacts`/stepped DDA tests the segment from player pivot to desired point. The camera is placed before the first solid contact by its collision radius, never closer than the configured minimum; when obstruction clears, it eases outward with the configured half-life. Inward collision correction is immediate, so the camera does not clip through newly placed matter. This is presentation logic in `Update`, not fixed simulation.

The underground classifier queries solid coverage vertically above the player and the cave manifest volume. It enables the player-child light under the contract in `config.md`. The light never influences generation, collision, or save data.

## Derived surface and dressing systems

Meshing consumes an immutable composite brick/halo snapshot plus immutable base-provenance owners and returns CPU vertex/index/material ranges plus source revision; it does not access ECS or GPU state. One topology enumeration is partitioned by the `SurfaceOwner` contract in `data-model.md`; terrain and object jobs never independently emit the same crossing. Upload/apply runs on the main/render boundary. Validated base keys share object meshes; any changed extraction stencil creates a unique revisioned range for that instance, and exact stencil reversion restores its shared handle.

Dressing generation consumes committed terrain triangles, material IDs, biome density, and stable surface-cell hashes. It emits anchors only above the upward-normal threshold and shares mesh/material variants. Detection of invalidated anchors is separate from regeneration response.

## Save/load system

Save takes a sorted immutable snapshot of deltas at one revision and serializes on an IO worker. Completion only follows flush and atomic rename. Load fully reads, decompresses, validates, and constructs a replacement map off-thread, then swaps it in one main-thread transaction and invalidates the union of old/new edited bricks plus halos. Detailed contracts are in `persistence.md`.

## Diagnostics

Brick view draws shared unit-cube line meshes transformed to 4 m boundaries. Raw mode creates instanced occupied voxel cubes only in the near/inspect set and uses material palette handles. Streaming mode overlays band colors and circles without changing regular activation. The time slider only writes `FixedSolarTime`.

All diagnostics are keyboard controlled and use a compact text legend/status readout. There is no settings flow, construction palette, scripting language, or polished debug UI.

## Explicitly absent systems

No schedules, messages, or placeholder update loops are created for combat, stats, NPCs, AI, navigation, room detection, labor, building blueprints, mechanisms, structural integrity, cave-ins, object rigid bodies, felling, growth, fire, wetness, damage, granular settling, fluid flow/pressure/draining, weather, seasons, ecology, multiplayer, scripting, language models, gas, pricing, spells, or progression. Material/object metadata reserved for future behavior remains inert.

## Test strategy

- Pure unit/property tests cover coordinate conversion, fixed-point base sampling, feature constraints, sphere affected sets, density operations, delta minimality, activation selection, collision resolution, camera obstruction distance, and dressing eligibility.
- Headless plugin tests use `MinimalPlugins` plus only required world features. They seed resources/entities, submit public commands, call `app.update()`, and assert ECS/query/events. They never create a window or renderer.
- Fixed tests configure Bevy fixed time and use a shared `run_fixed_ticks(&mut App, n)` helper. They assert position/contact after exact ticks and never sleep or assume an update is a tick.
- State tests verify all legal transitions/lifecycle entry actions and rejection/failure paths.
- Integration tests run N fixed ticks with scripted semantic intent and assert properties: player never overlaps solid matter, a dig opens traversable occupancy, an edit survives cache unload/reactivation and save/load, and every accepted command produces exactly one terminal commit or rejection.
- Determinism tests hash the manifest and a checked-in set of base samples on each supported target. Property tests assert no feature/object query returns out-of-bounds targets and all generated required objects/features have exactly the required cardinality.
- Rendering is not unit-tested. Human route review and release benchmark/testbed runs verify visual quality, seams, LOD transitions, water, lighting, raw/brick/band modes, and the public milestone captures.

Release scheduling acceptance additionally preloads normal near/mid/far queues to their configured in-flight caps with workers actively running, enables raw view, then issues a quantized boundary/corner-aligned 3 m hillside carve that also intersects one object and dressing. It records normal cancellation checkpoints, mutation worker start/finish, packet bytes/range counts, per-frame main-thread time, presented revisions, and asserts collision by the next fixed tick, no frame over 25 ms, no apply frame over 4 ms, and one atomic visible commit no later than `F+2`.

Performance acceptance is not weakened into ordinary unit tests; it is measured by the release-mode scenarios in `benchmarks.md` on identified hardware.
