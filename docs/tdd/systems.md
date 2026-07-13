# Runtime Systems and Property Contracts

## Schedule model

The app configures `Time<Fixed>` to 60 Hz. Fixed systems may run zero, one, or several times in a rendered frame; no fixed system reads `just_pressed`, mouse deltas, or other frame-local edges directly.

```text
Startup
  load_boot_assets -> validate_world_identity -> begin_initial_activation

PreUpdate (every rendered frame)
  collect_physical_input
    -> map_actions
    -> latch_player_debug_ui_intents
    -> publish_world_commands

FixedUpdate (0..N ticks/frame)
  IntentSet
    -> EditSet
    -> CollisionDetectSet
    -> MovementResolveSet
    -> WaterStateSet
    -> FocusSet
    -> StreamPlanSet
    -> FixedTelemetrySet

Update (every rendered frame)
  poll_generation_tasks
    -> poll_mesh_tasks
    -> install_revision_matched_results
    -> refresh_objects_and_dressing
    -> update_camera_and_light
    -> update_water_and_sun_presentation
    -> update_debug_visualizers_and_hud
    -> poll_persistence_tasks
    -> update_frame_telemetry

PostUpdate
  finalize_visibility -> record_render_frame_boundary
```

Each arrow is an explicit `.after(...)` relation or chained system tuple. Systems are additionally gated by app/world state; [states.md](states.md) gives the gates.

## Generation systems

### `validate_world_identity`

Inputs: `RegionConfig`, `CuratedManifest`, material registry, ruin stamp bytes. Output: `WorldIdentity` or `WorldOpenError`.

Property: success occurs only if bounds/voxel/brick constants match the product contract, all material IDs/keys are canonical, the config/stamp digest equals seed and manifest, all feature/object coordinates are in bounds, the ruin uses only cut stone/air, no two non-ruin raw object shapes share a solid voxel, no non-ruin raw shape intersects an authored ruin-stamp coordinate, and curator route assertions pass. Overlap errors carry the stable IDs and lexicographically first conflicting coordinate. Failure leaves no `WorldStore` capable of accepting queries, edits, or spawning an authored object root.

### Pure evaluators

`evaluate_column(identity, coord) -> ColumnSample`, `evaluate_base_voxel(identity, coord) -> Voxel`, and `classify_brick(identity, brick) -> ProceduralClass` are pure functions. Each uses keyed integer hashes and Q16/Q8 interpolation. Evaluation order, thread count, cache contents, and caller package cannot affect results.

`build_object_index(&[ObjectPlacement], &ObjectIndexConfig) -> Result<ObjectSpatialIndex, ManifestError>` runs during validation. For each placement it evaluates analytic raw bounds in constant space, expands them by the compile-time union-stencil extrema to obtain `dependency_bounds`, stores those two boxes in one fixed-size record, and inserts its `u32` manifest index into at most 16 horizontal grid cells. It never enumerates raw-shape or dependency coordinates. Construction rejects a dependency box covering more than 128 voxel bricks or measured retained capacity over 16 MiB and is retained only after validation succeeds. `validate_object_shape_disjointness(&ObjectSpatialIndex, &RuinPoi) -> Result<(), ManifestError>` obtains candidate pairs from shared grid cells, deduplicates and sorts them, intersects raw-shape AABBs, and evaluates both raw fixed-point shapes over the intersected voxel box in lexicographic order with early exit. It likewise compares each candidate against every transformed authored coordinate of the sparse ruin stamp, including air-carves. This is bounded by the configured per-cell/per-object index limits and finite AABBs; it never scans the region.

`placement_ids_in(&ObjectSpatialIndex, AabbQ8) -> Vec<ObjectId>` visits only overlapped 32 m cells and exact-filters raw bounds for activation. `surface_dependency_ids_for_changes(&ObjectSpatialIndex, &[VoxelCoord]) -> Vec<ObjectId>` visits each unique horizontal grid cell touched by the sorted changed-coordinate bounds once, deduplicates candidates, rejects nonintersecting dependency boxes, and exact-filters candidates by applying `dependency_contains` to changed coordinates with early exit. Both return IDs sorted ascending. With `c` intersected cells, `q` changed coordinates, `p <= c*1024` unique candidates, `k` returned IDs, and stencil size `s <= 512`, the dependency query is bounded by `O(c*1024 + p log p + p*q*s + k log k)` and `O(p + k)` transient IDs; it creates no coordinate set per placement. Activation resolves an ID by binary search in the sorted manifest. Retained index memory, including all dependency metadata and allocation capacities, must be at most 16 MiB.

Property tests:

- evaluating the same coordinate in random orders and parallel partitions returns identical values;
- every column covers the full vertical interval with ordered, non-overlapping runs;
- water is bounded by its footprint/surface and has a carved non-water bed;
- the cave route is connected and void, and declared aquifer/ore wall samples expose the expected materials;
- broad terrain includes meadow/forest density ranges, cliff slope/strata exposure, and required object counts/species;
- generated accepted placements meet configured forest/object counts and contain no raw solid-shape pair overlap or ruin-stamp conflict; deliberately overlapping manifests return the stable `ObjectShapeOverlap`/`ObjectRuinOverlap` error before world readiness;
- indexed AABB results equal a brute-force oracle, contain no duplicate, and remain sorted across insertion orders; activation spawns exactly those IDs, eviction removes their visuals, and reactivation returns the identical ID list without rebuilding or mutating the index;
- for small random analytic shapes and every supported LOD, lazy `dependency_contains` membership is exactly equal to the test-only explicit sorted-set oracle; an instrumented extractor read trace is a subset of that oracle, and coordinates outside it leave the payload byte-identical;
- the checked-in full manifest builds the object index without any dependency-coordinate allocation, within 16 MiB retained capacity and the 128-brick/16-cell bounds; cold-start benchmark telemetry records its build duration and retained bytes, and the normal process-to-ready result must still pass 5,000 ms;
- all samples outside bounds return `OutOfBounds`, never wrap or alias.

### `begin_initial_activation`

It creates traversal/camera focus at the manifest spawn waypoint and requests only the collision neighborhood, visible surface LOD chunks, static water patches, registered objects, and dressing needed for the first view. It does not scan all region voxels or columns.

Property: before `WorldReady`, a support capsule at spawn is collision-queryable and non-overlapping, all high-priority initial jobs are complete, and the installed render chunks cover the initial camera frustum. Startup failure is terminal and reported.

## Input systems

`collect_physical_input` reads Bevy keyboard, mouse, and gamepad resources. `map_actions` applies `ActionMap`, including dead zones and mouse sensitivity. `latch_player_debug_ui_intents` stores continuous axes for the current frame and OR-latches edge actions until consumed.

Properties:

- gameplay/debug systems contain no `KeyCode`, `MouseButton`, or `GamepadButton` values;
- one physical edge maps to at most one semantic edge per action;
- a latched edge is consumed once by the first eligible fixed tick and cannot repeat in subsequent ticks from the same frame;
- menu/time-slider focus suppresses movement/debug actions but not the dedicated UI action set.

Default physical bindings are documented in [config.md](config.md) and shown in the diagnostic HUD.

## Mutation systems

### `validate_and_stage_edits` (`EditSet`)

The system drains requests, sorts by `request_id`, rejects duplicates/invalid input/queue overflow, and evaluates the fixed-point spherical kernel against current truth. It stages `(coord, old, new)` records in sorted order. This phase has no externally visible partial mutation.

Property: every staged coordinate is in bounds and its voxel cell center lies within the closed sphere. No coordinate outside that sphere changes. Dig never increases density; place never decreases density. With the same starting world and ordered request list, the staged values are byte-identical.

### `commit_edits` (`EditSet`, after validation)

The system applies a staged request atomically, increments one revision, updates base-relative deltas, dirties changed bricks and face seams, invalidates surface anchors, pins work, and emits `EditCommitted`.

Property: after the system returns, every query sees either all values before the request or all values after it—never a mix. Collision uses after-values immediately. Deltas contain exactly cells whose after-value differs from regenerated base. A no-op edit does not increment revision and reports zero changed voxels with immediate readiness.

### `queue_priority_surface_jobs`

For each distinct affected near render-tile/vertical-cave key, this takes an immutable one-voxel-halo snapshot at the committed revision and submits CPU mesh extraction. Jobs are prioritized above ordinary streaming. Changed chunks remain rendered with their prior mesh for at most two frames after the request's observed frame; raw/collision truth is already current. If the edit removes an entire surface, install removes the old mesh.

Property: all affected mesh tiles/chunks, face seams, water patches, registered-object visuals, and dressing batches are represented in the completion barrier for the request. The barrier cannot emit `EditSurfaceReady` until every derived item is installed/removed at the edit revision. Results from older revisions are discarded and rescheduled. The barrier deadline is measured from the `submitted_frame` stamped by `WorldEditWrite`, even if that rendered frame executes zero fixed ticks or publication occurs after the current fixed drain.

## Collision and movement systems

Occupancy detection and response are separate. `detect_character_contacts` calls the generic world capsule sweep and produces `CharacterContacts` (time fraction, normals, touched voxel/material). It knows no player movement rule. `resolve_character_movement` interprets those contacts to slide, ground, step, or stop. Camera collision uses the same query primitive but its own shortening response.

The character body is an upright capsule. Per fixed tick:

1. Convert intent to desired horizontal acceleration.
2. Apply gravity/jump or paddling constraint.
3. Subdivide only when displacement would exceed half a voxel (0.125 m); this prevents tunneling without tying substeps to frame rate.
4. Sweep for each substep, advance to the safe fraction, project remaining displacement along deterministic sorted contact planes, and iterate at most four contacts.
5. Attempt a 0.30 m step-up only when grounded, forward motion is blocked, and the raised capsule plus forward/down sweeps are clear.
6. Run an overlap assertion/recovery bounded to one voxel; a debug assertion/test fails if a non-overlapping starting capsule ends overlapped.

Properties after every step: capsule is in region bounds; capsule does not overlap any voxel whose `solid_collision` predicate is true; horizontal velocity does not exceed the selected run/sprint bound; position delta is bounded by integrated velocity; and a newly dug void is traversable in the first tick after commit. Movement never samples render meshes. Water may overlap the capsule because `water_volume` is not solid collision.

`update_water_contact` samples static water separately after solid response. Entry occurs when the capsule intersects water and the surface is above the waist threshold; exit occurs when no overlap or supported dry ground rises above the surface. Paddling pins the body center relative to the static surface, applies horizontal speed/drag, and suppresses gravity/jump. It cannot descend into an underwater state. Water is not returned as a solid collision contact.

## Focus and streaming systems

### `update_focus_sources` (`FocusSet`)

It converts current player, camera, debug inspection, and benchmark points to Q8 positions. Player/camera sources update every fixed tick; mutation sources are pinned until edit readiness.

### `plan_active_bands` (`StreamPlanSet`)

For each focus, it determines desired brick/terrain/object LOD by distance and purpose. It produces sorted activation, LOD-change, and eviction queues with priorities: committed edit > collision/traversal > current camera frustum > inspection > near visual > far visual > prefetch. Hysteresis prevents oscillation at band edges.

Property: all bricks intersecting the character's next-tick swept broad phase are queryable synchronously, committed/edit-pinned bricks are never evicted, and an unchanged focus does not continuously enqueue the same work. Moving away can release detailed arrays and meshes but never deltas or deterministic object placement.

### `materialize_bricks`

Workers first call the conservative classifier. Uniform/procedural results remain compact. Only a surface/cave/material boundary, delta, raw view request, or collision need expands a `[Voxel; 4096]`. Results return immutable payloads with brick/revision/requested purpose.

Property: an untouched uniform brick allocates zero voxel arrays. At most one accepted materialization result per `(brick, revision, purpose)` is installed. Queue budgets cap work started and bytes installed per frame; budgets are configuration values measured by benchmarks.

### `evict_inactive_detail`

At the end of the streaming update, it removes unpinned detailed arrays, render entities, object visuals, and dressing outside hysteresis bands. Delta maps and registry metadata remain. Shared assets are retained globally rather than cloned per entity.

## Terrain meshing systems

`extract_chunk_mesh(snapshot, lod, surface_policy) -> MeshPayload` is a pure CPU function described in [rendering.md](rendering.md). Its snapshot contains current density/material, regenerated base `VoxelSource`, and `solid_presentation_owner` values with a one-voxel halo, so tasks never borrow mutable world state or infer routing from material kind. Topology is generated from the global current solid scalar, then candidate primitives are filtered by the owner and spatial-key fields in the requested payload. Output vertex/index buffers use `u32` indices and material blend data with 32-bit fields.

Properties:

- adjacent chunks at the same LOD generate matching boundary positions/normals from identical samples;
- a terrain/ruin payload contains no primitive owned by `NonRuinObject(_)`, a derived-object payload for ID `x` contains no primitive except `NonRuinObject(x)`, and the union contains each generated normal-world primitive exactly once;
- transition skirts close cracks between different LODs without becoming collision truth;
- vertex positions are finite and inside the brick plus seam margin;
- index values fit `u32`, triangles reference valid vertices, and per-allocation byte counts fit `u32` before upload;
- natural and sharp policies affect normal/feature placement only, never the `solid_collision` predicate or saved truth.

`install_chunk_meshes` checks revision/LOD/request tokens, swaps Bevy mesh handles, updates instrumented byte counts, and despawns empty chunks. An edit mesh is installed by swapping handles as one deferred command batch so different chunks of one edit are not intentionally revealed across extra frames. Completion is acknowledged only when the Bevy render sub-app has extracted every affected handle/revision for that rendered frame, not merely when the main-world command was queued.

## Object and dressing systems

`spawn_registered_object_visuals` runs only for a validated, ready world and calls the immutable spatial index for newly active area AABBs. For each non-ruin instance it evaluates authored eligibility with the compact record and `delta_intersects_dependency`: an eligible placement clones kind/species handles from `WorldRenderAssets`, while an ineligible placement starts directly with a current-revision owner-filtered derived root. Only active candidates pay the bounded sparse-brick query; opening the region does not scan deltas or dependency coordinates for inactive placements. This applies after save-load and streaming reactivation as well as first opening; no instance creates a material. Visibility ranges and frustum culling are enabled. The manifest disjointness invariant means every base non-ruin raw shape is wholly attributed to its own ID, so a complete authored GLB cannot represent a cell assigned to another object. Terrain snapshots exclude every `NonRuinObject(_)` primitive at every LOD, including before or while either object root is spawned. A ruin ID is handed to the terrain voxel-mesh owner instead of spawning an authored root: its active stamped/current cells have `TerrainChunk` ownership and are included in revisioned material-aware chunk extraction. Solid collision remains the voxel shape in base/current evaluation, so hiding an LOD visual does not remove collision. Activation creates exactly one eligible authored or voxel-derived presentation owner for each visible sorted returned ID; eviction removes that presentation but neither changes placements nor rebuilds the index.

`refresh_modified_object_visuals` asks the immutable object index for every non-ruin `ObjectId` whose lazy `ObjectSurfaceDependency` contains at least one changed coordinate; it does not filter candidates by the changed coordinate's base provenance. For each returned active ID, it recomputes authored eligibility with `delta_intersects_dependency`, which probes at most 128 sparse delta bricks and allocates no dependency collection. `Intact -> VoxelDerived` atomically despawns the shared GLB and installs the matching-ID owner-filtered payload for an object-, terrain-, or ruin-attributed dependency delta. Subsequent dependency edits replace only revision-matched derived payloads. `VoxelDerived -> Intact` removes the derived payload and restores the shared GLB only when no dependency delta remains, so exact reversion of object cells cannot restore it while an adjacent terrain/ruin boundary edit persists. A delta's presentation owner still comes from regenerated base provenance, so a sphere spanning object and terrain produces separate object and terrain payloads without reassignment. Each transition, including an empty derived payload, and every terrain/chunk counterpart is in the originating edit's `EditSurfaceReady` barrier through render-extraction acknowledgement.

`Ruin(ObjectId)` changes are handled by the normal dirty terrain/mesh/seam path, not `refresh_modified_object_visuals`. Initial activation extracts the stamp into a `RuinVisualState::VoxelDerived { revision }` presentation. An edit or exact revert extracts the ruin's current sampled cells and replaces only matching-revision chunk payloads. The originating request's completion barrier contains every dirty chunk and seam covering changed or halo-sampled ruin cells and remains pending until those handles at the edit revision have been installed and observed by render extraction. An empty ruin result despawns the old payload at that revision and also satisfies the barrier; stale task results are discarded and rescheduled.

Properties: each active non-ruin placement has exactly one of an eligible intact shared root or current-revision voxel-derived root, never both. `Intact` implies every coordinate in `ObjectSurfaceDependency(id)` equals base; a delta outside that set cannot change the ID's owner-filtered extraction. Each active ruin has exactly one current-revision voxel-derived terrain-chunk owner and never an authored/GLB root, including after exact reversion. Every root/chunk attribution records its source key; modified visible solid cells agree with public samples and the pure owner predicate; and visual despawn never modifies truth. No object system adds velocity, rigid-body, health, growth, burning, felling, or settling data.

Pure extraction tests seed adjoining terrain, ruin, and disjoint non-ruin sources, apply object-only, terrain-only, boundary-spanning, and exact-revert deltas, and assert stable base attribution plus disjoint owner-filtered primitive sets whose union equals unfiltered global extraction. Dependency-footprint property tests compare lazy membership to the explicit-set oracle, mutate every coordinate read while producing an object's owner-filtered payload and assert `dependency_contains`, and mutate samples outside it to assert the payload is byte-identical. Sparse-query properties compare `delta_intersects_dependency` and changed-ID discovery with brute-force explicit-set oracles across empty, boundary, full-brick, and exact-revert delta maps while asserting no dependency-sized allocation. Dedicated headless tests begin with an intact object and (a) place a terrain-owned solid in adjacent base air and (b) dig adjacent terrain-owned solid: each commit must remove the GLB, install the current revision's owner-filtered derived root, and withhold `EditSurfaceReady` until both object-root and terrain/chunk/seam handles are observed by render extraction. A sequence test performs object edit -> adjacent terrain edit -> exact object-cell revert and asserts the root remains voxel-derived at the last revision with no GLB; only exact reversion of the adjacent delta permits an atomic derived-to-intact restoration and its readiness acknowledgement.

A separate intact-overlap headless test supplies two overlapping non-ruin placements (and a non-ruin/ruin conflict), asserts validation returns the coordinate-bearing error before `WorldReady`, advances the app, and asserts that no authored or derived object root and no terrain chunk was spawned from the rejected world. Remaining headless transition tests cover accepted non-ruin intact/edit/full-dependency-revert and ruin activate/edit/revert/erase; they assert that terrain chunks never contain non-ruin primitives in intact or derived states, exactly one non-ruin root exists through each atomic transition, the ruin remains terrain-chunk-derived, and `EditSurfaceReady` waits for all affected root/chunk/seam extraction acknowledgements.

`derive_dressing(snapshot) -> Vec<DressingInstance>` samples upward-facing topsoil surfaces using deterministic blue-noise/hash candidates. `refresh_dressing` despawns the old revision immediately when an anchor is dirty, then installs instances for the new mesh revision using shared grass/clutter handles.

Property: all instances have an eligible current anchor; there are no instances on water, cave ceilings, steep faces, cut stone, or empty edited cells. Re-evaluating an unchanged revision produces the same keys/transforms.

## Water and environmental presentation

`build_water_patch` derives a horizontal mesh at each `WaterBodyDef.surface_y` clipped to the body footprint and visible carved banks. It does not run a fluid tick. An edit to the bank/bed may expose or occlude the predefined static volume, so only intersecting water patches rebuild; water level and footprint never propagate.

`set_fixed_time_of_day` maps the slider value to sun direction, illuminance, sky color, and ambient settings. No clock advances it. `update_underground_light` uses player depth/sky probe and smoothly changes presentation intensity per frame.

Properties: time remains constant without input; water surface elevation remains constant through edits; no system writes neighboring water voxels due to pressure or flow.

## Camera and diagnostics

`update_orbit_camera` consumes frame look/zoom, smooths desired yaw/pitch/distance, then uses `WorldRead::sweep_capsule(QueryMask::SOLID)` to keep a small camera probe out of solid collision. Smoothing occurs before the final collision clamp so it cannot push the camera through terrain. A zero-length safe segment places the camera at the target safety offset rather than inside the wall.

Diagnostic systems are presentation-only:

- brick view pages active 4 m boundaries and colors clean/dirty/edit-pinned/task-pending states;
- raw-voxel view requests cell pages and instances `material_present` cells, distinguishing solid and translucent water;
- streaming view pages render-chunk bounds/LOD/bands and focus markers and combines them with public queue counts;
- HUD shows selected tool/material, fixed time, save/load result, frame rate, and contract warnings.

Each visualizer uses `WorldRead::diagnostic_snapshot` plus public telemetry, with a stable cursor reset whenever the snapshot frame/revision changes. It must not query private terrain entities in the consumer. Tests compile an external consumer and compare its snapshot page sequence to the demo adapter's sequence. Debug geometry has an independent instrumented memory category and is disabled in benchmark runs unless the scenario explicitly validates it.

## Persistence systems

`snapshot_deltas` clones the sorted compact delta values at a fixed-tick boundary. `encode_save` and disk write run off-thread. `poll_save_task` reports the terminal message and never blocks the render schedule. Load similarly decodes and validates off-thread; the library-owned transaction gate rejects new edits, drains already accepted edits, and `commit_loaded_deltas` swaps only at a fixed boundary. Consumer state is never read.

Properties: save bytes are deterministic for an identical identity/delta set; save never includes an unchanged voxel or derived data; failed write preserves the previous slot because rename is last; failed load leaves current truth untouched; a save/load round trip reproduces every material, density, and state byte; loading against another seed/config is rejected.

## Benchmark systems

`drive_flythrough` evaluates a time-parameterized camera spline through all tagged route waypoints at fixed scripted speed. It does not teleport between scenes. `drive_carve_storm` targets a deterministic list of valid surface hits and submits through the same public `WorldEditWrite` path as the demo. It does not advance until the expected terminal/readiness event arrives, making every latency sample attributable.

`capture_metrics` records rendered-frame intervals, edit frame/time pairs, startup milestone times, instrumented graphics allocation changes, save completion bytes, resolution, build identity, and adapter/machine fields. Measurement uses monotonic clocks but does not influence world results.

Property: a completed report contains every required metric and all mandatory machine fields; absent unsupported driver metadata is explicit null. A result without machine profile is invalid and causes a nonzero benchmark exit.

## Tests and fixed-time helper

`moria_world::testing::run_fixed_ticks(app, count)` configures manual/virtual time so exactly `count` `FixedUpdate` executions occur, calls `app.update()` as required by Bevy 0.19 semantics, and asserts the observed fixed-tick counter. Tests never sleep and never rely on one update equaling one tick without this helper.

Test layers:

- pure unit/property tests for evaluators, kernels, collision, mesh topology, delta/save encoding, and metric math;
- headless plugin slices using `MinimalPlugins`, injected config/assets, public messages, and explicit fixed ticks;
- headless demo integration with scripted semantic `PlayerIntent` proving the cliff/steps/cave/dig traversal properties;
- headed manual/render benchmark runs for visual appearance, GPU behavior, window input, camera near-plane behavior, and acceptance hardware targets.

Rendering is not asserted from headless screenshots. The implementation author does not weaken acceptance tests to fit code; the pipeline's independent test/review stage owns final adversarial coverage.
