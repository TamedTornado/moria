# Public API and Interaction Contracts

## Boundary

`moria-world` exposes plugins, immutable value types, a read-only Bevy `SystemParam`, and request/result messages. It does not export `WorldStore`, `BrickRecord`, mutable voxel slices, generation internals, or authoritative ECS components. Rust privacy is the primary enforcement: consumer crates cannot compile code that directly edits truth. The demo, benchmark, and future games are equivalent consumers.

The crate root exports only the items listed in this document. Adding a new public operation requires a contract, tests through an external test crate, and proof that the validation demo does not get a more powerful path than future consumers.

The `curation` Cargo feature additionally exports pure development tooling APIs used by `moria-curate`:

```rust
pub fn derive_manifest(seed: u64, config: &RegionConfig, stamp: &SparseVoxelStamp)
    -> Result<CuratedManifest, CurationError>;
pub fn validate_manifest(config: &RegionConfig, manifest: &CuratedManifest)
    -> Result<CurationReport, CurationError>;
```

These functions deterministically derive/query base generation and return values; they cannot access a live `WorldStore`, deltas, or render state. The shipped demo and benchmark do not enable or call this feature. It exists to keep manifest generation in the same validated generator rather than duplicating algorithms in the CLI.

## Startup and readiness

```rust
pub struct MoriaWorldPlugin {
    pub config: WorldPluginConfig,
}

pub struct WorldPluginConfig {
    pub region_config: Handle<RegionConfig>,
    pub curated_manifest: Handle<CuratedManifest>,
    pub initial_focus: WorldPointQ8,
    pub save_slot: SaveSlot,
    pub presentation: PresentationConfig,
}

pub enum WorldLifecycle {
    Uninitialized,
    Loading,
    Ready,
    Failed(WorldOpenError),
}

pub struct WorldReady {
    pub identity: WorldIdentity,
    pub control_ready_at: Duration,
}
```

The plugin transitions to `Ready` only after config/manifest/material assets validate, the initial collision neighborhood is queryable, initial terrain and water meshes needed by the camera are installed, required object assets have fallback-ready handles, and the initial player-support sample is solid. It then emits `WorldReady` exactly once. A missing decorative high-detail asset may use its declared placeholder and does not block readiness; corrupt config, stamp, or manifest is fatal.

`PresentationConfig.enabled` defaults to true for shipped consumers. When false, the plugin registers no render/water/object-visual/dressing systems and readiness omits camera-frustum mesh/visual conditions while retaining generation, queries, collision, edits, streaming truth, and persistence. This is the supported headless mode used by logic tests and the benchmark's post-run exact-load instance; it does not expose additional world access.

`WorldOpenError` distinguishes `Asset`, `ManifestIdentity`, `InvalidConfig`, `GenerationContract`, `Save`, and `InitialActivation` errors. The consumer may show the error and exit; it must not enter `Playing` after failure.

## Read-only world observations

```rust
#[derive(SystemParam)]
pub struct WorldRead<'w, 's> { /* private fields */ }

impl WorldRead<'_, '_> {
    pub fn identity(&self) -> &WorldIdentity;
    pub fn bounds(&self) -> WorldBounds;
    pub fn sample_voxel(&self, coord: VoxelCoord) -> Result<WorldSample, QueryError>;
    pub fn sample_point(&self, point: WorldPointQ8) -> Result<WorldSample, QueryError>;
    pub fn sample_column(&self, coord: ColumnCoord) -> Result<ColumnSample, QueryError>;
    pub fn ray_cast(&self, ray: WorldRayQ8, max_distance_q8: u32, mask: QueryMask)
        -> Result<Option<WorldHit>, QueryError>;
    pub fn sweep_capsule(&self, capsule: CapsuleQ8, displacement: Vec3Q8, mask: QueryMask)
        -> Result<SweepResult, QueryError>;
    pub fn overlap_capsule(&self, capsule: CapsuleQ8, mask: QueryMask)
        -> Result<Vec<WorldHit>, QueryError>;
    pub fn water_surface_at(&self, x_q8: i32, z_q8: i32)
        -> Result<Option<WaterSample>, QueryError>;
    pub fn route(&self) -> &TraversalRoute;
    pub fn active_band(&self, brick: BrickCoord) -> Option<ActiveBand>;
    pub fn diagnostic_snapshot(&self, request: DiagnosticPageRequest)
        -> Result<DiagnosticPage, QueryError>;
}
```

Queries are synchronous, side-effect free from the consumer's perspective, and always overlay committed deltas on deterministic base evaluation. They may populate an internal memoization cache through interior implementation details, but the return value cannot depend on cache state. An in-bounds query never fails merely because a brick is inactive; procedural evaluation supplies the answer. `QueryError::OutOfBounds` is returned outside the region and `QueryError::NotReady` is possible before `WorldReady`. Invalid masks/shapes and requests beyond the limits below return `QueryError::LimitExceeded(QueryLimitKind)` before traversal and never return a truncated/partial result. No query returns a mutable reference.

These are hard Product One API limits, chosen to cover the 9 m orbit camera, 1.8 m player capsule, per-tick movement, 64 m debug targeting, and paged diagnostic views while keeping synchronous work inside the 60 Hz frame budget:

```rust
pub const MAX_RAY_DISTANCE_Q8: u32 = 16_384;       // 64 m
pub const MAX_RAY_VOXEL_VISITS: u16 = 448;
pub const MIN_CAPSULE_RADIUS_Q8: u16 = 32;          // 0.125 m
pub const MAX_CAPSULE_RADIUS_Q8: u16 = 128;         // 0.5 m
pub const MAX_CAPSULE_HALF_SEGMENT_Q8: u16 = 192;  // 0.75 m; max total height 2.5 m
pub const MAX_SWEEP_DISPLACEMENT_Q8: u16 = 3_072;  // Euclidean length, 12 m
pub const MAX_SWEEP_CANDIDATE_TESTS: u16 = 8_192;
pub const MAX_OVERLAP_CANDIDATE_TESTS: u16 = 512;
pub const MAX_QUERY_HITS: u16 = 512;
pub const MAX_COLUMN_RUNS: u16 = 64;
pub const MAX_ROUTE_WAYPOINTS: u16 = 64;
pub const MAX_BASE_FEATURE_EVALUATORS: u8 = 16;

pub enum QueryError {
    NotReady,
    OutOfBounds,
    InvalidInput,
    LimitExceeded(QueryLimitKind),
    SnapshotExpired,
}
pub enum QueryLimitKind {
    RayDistance,
    RayVoxelVisits,
    CapsuleRadius,
    CapsuleHeight,
    SweepDisplacement,
    SweepCandidateWork,
    ResultCount,
    ColumnRuns,
    DiagnosticBricks,
    DiagnosticCells,
    DiagnosticChunks,
    DiagnosticFocuses,
}
```

The object index separately caps exact base-object candidates at 64 for any 4 m sample cell, and manifest validation caps the fixed terrain/geology/cave/ore/aquifer/water/ruin evaluator chain at 16. A scalar voxel evaluation therefore cannot fall back to scanning a 32 m forest cell, arbitrary feature vector, or the complete manifest. `ray_cast` uses deterministic 3-D DDA and rejects zero/unnormalized direction or an empty mask as `InvalidInput`, and a distance above 64 m as `LimitExceeded(RayDistance)`; the DDA stops after at most 448 visited voxels. Capsule radius must be 0.125–0.5 m and vertical half-segment at most 0.75 m. `sweep_capsule` additionally rejects Euclidean displacement above 12 m or a radius/height/displacement combination whose conservative centerline-DDA-expanded broad phase exceeds 8,192 voxel candidates. This combined work check permits the full-height player for short fixed-tick sweeps and the 0.18 m camera probe for its 9 m cast without permitting the pathological maximum height and distance together. Matched candidates share a 65,536 exact-contact-test budget; exhausting it returns `LimitExceeded(SweepCandidateWork)` before a dense query can perform unbounded per-candidate Q8 stepping. `overlap_capsule` tests at most 512 broad-phase cells under the dimension limits. Sweep/overlap return at most 512 exact hits; exceeding that invariant returns `LimitExceeded(ResultCount)` rather than silently dropping contacts.

The complexity and allocation contract for every synchronous public read is:

| Query | Input/result bound | Worst-case work and allocation |
|---|---|---|
| `identity`, `bounds` | One immutable value/reference | `O(1)`, no allocation |
| `sample_voxel`, `sample_point` | One in-bounds coordinate | `O(log D + 64 + F)`, no allocation; `D` is delta bricks and `F <= 16` is the fixed generated-feature evaluator count |
| `sample_column` | One in-bounds X/Z, at most 64 ordered runs | `O(F + R)`, `R <= 64`; one result vector capped at 64 |
| `ray_cast` | 64 m / 448 voxels, zero or one hit | `O(448 * (log D + 64 + F))`, no work-sized allocation |
| `overlap_capsule` | dimensions above, at most 512 candidates / 512 hits | `O(512 * (log D + 64 + F) + H log H)`, `H <= 512` result allocation |
| `sweep_capsule` | 12 m, 8,192 candidates, 65,536 exact contact tests / 512 contacts | `O(8192 * (log D + 64 + F) + 65536 + H log H)`, `H <= 512`; one bounded result vector |
| `water_surface_at` | one in-bounds X/Z; exactly two curated water bodies | `O(2)`, no allocation |
| `route` | at most 64 immutable waypoints | `O(1)`, borrowed immutable slice/reference |
| `active_band` | one valid brick | `O(log A)`, no allocation; `A` is active brick records |
| `diagnostic_snapshot` | page limits below | `O(log A + B + C + Foc)` or `O(log A + B*4096 + C + Foc)` with cells; bounded owned page only |

These are algorithmic maxima, not sufficient performance evidence. Gate F2 in [implementation-plan.md](implementation-plan.md) measures 256 distinct, previously unsampled inactive-forest calls, 1,000 normal player/camera/debug query bundles after ordinary renderer warmup, 128 maximum column/metadata pages, and 128 two-brick cell pages on the M4. Frame-critical calls must each have p99 at most 1.0 ms, the normal bundle p99 at most 2.0 ms, and no measured call above 4.0 ms. `sample_column` and a metadata-only diagnostic page have p99 at most 1.0 ms; a maximum cell-bearing diagnostic page has p99 at most 4.0 ms and no sample above 8.0 ms. The harness records iteration counts and rejects repeated coordinates in the cold set. A failure blocks downstream work; cache warming cannot be the only passing evidence.

`Vec3Q8` is three signed Q8 metre components. `WorldRayQ8` contains a Q8 origin and normalized Q16 direction. `CapsuleQ8` contains a Q8 center, radius, and vertical half-segment. `SweepResult` contains `safe_fraction_q16: u16` (`0..=65535`), end capsule, and a coordinate-sorted `Vec<WorldHit>`. `WorldBounds` contains min-inclusive/max-exclusive Q8 corners. Constructors validate normalization, the limits above, and integer overflow before a query runs.

`WorldSample` contains coordinate, material ID, density, state byte, `material_present`, `solid_collision`, `water_volume`, and current world revision. The three booleans are computed by the exact predicates in [data-model.md](data-model.md); they are not aliases. `QueryMask::SOLID` selects only `solid_collision`, `QueryMask::WATER` selects only `water_volume`, and their union selects either; it has no gameplay faction/category semantics. `WorldHit` contains hit voxel, Q8 hit point, a quantized face/gradient normal, material, matched class, distance, and revision.

`sweep_capsule` implements the supported player/camera collision observation. Given start capsule `C` and displacement `D`, it returns the largest collision-free fraction `t in [0, 65535]`, a stable set of contact normals, and end capsule `C + D*t`. With `QueryMask::SOLID` it evaluates only solid collision, not water or render triangles. Ties are sorted by voxel coordinate and normal axis so output is independent of hash/thread order.

The diagnostic observation is a bounded immutable page, not store access:

```rust
pub struct DiagnosticPageRequest {
    pub snapshot: Option<DiagnosticSnapshotToken>, // None starts a snapshot
    pub after_brick: Option<BrickCoord>,
    pub max_bricks: u16,       // 1..=256; <=2 when include_cells
    pub include_cells: bool,
}
pub struct DiagnosticPage {
    pub snapshot: DiagnosticSnapshotToken,
    pub frame: u64,
    pub revision: u64,
    pub bricks: Vec<DiagnosticBrick>, // lexicographic BrickCoord order
    pub render_chunks: Vec<DiagnosticRenderChunk>,
    pub focuses: Vec<DiagnosticFocus>,
    pub next_after_brick: Option<BrickCoord>,
}
pub struct DiagnosticBrick {
    pub coord: BrickCoord,
    pub bounds: AabbQ8,
    pub band: ActiveBand,
    pub purposes: FocusPurposeFlags,
    pub dirty: DiagnosticDirtyFlags,
    pub pin_count: u16,
    pub task: Option<DiagnosticTaskKind>,
    pub cells: Option<Vec<DiagnosticCell>>,
}
```

Only currently active or explicitly inspected bricks are enumerable. `max_bricks == 0` is `InvalidInput`, values above 256 return `LimitExceeded(DiagnosticBricks)`, and `include_cells && max_bricks > 2` returns `LimitExceeded(DiagnosticCells)` before snapshot creation. `DiagnosticCell` contains local index, material, density, `material_present`, `solid_collision`, and `water_volume`; with `include_cells`, all 4,096 cells are returned for each page brick so empty and partial cells remain inspectable. A cell-bearing page is therefore capped at 8,192 cells. `DiagnosticRenderChunk` contains key, Q8 bounds, LOD, band, revision, and resident/pending phase; a page contains at most 512 chunks and 16 focuses. Exceeding either cap returns `LimitExceeded(DiagnosticChunks|DiagnosticFocuses)` and causes the caller to retry with a smaller `max_bricks`; it never truncates. `DiagnosticFocus` contains public focus ID, position, and purpose. Render chunks/focuses included are those intersecting or contributing to the returned brick page; an empty terminal page returns the remaining zero-brick focus markers once. The first page's token identifies the active-index generation; if activation, dirty/task state, focus, or revision changes before a later page, that request returns `QueryError::SnapshotExpired` and the caller restarts with `snapshot: None`, preventing mixed-generation pages without retaining an unbounded copy. Internally, ordered active-brick/chunk indices make page construction `O(log A + B + C + F)` without cells and `O(log A + B*4096 + C + F)` with cells, where `B <= max_bricks`, `C <= 512`, and `F <= 16`; it never scans the 4-billion-voxel region. Returned vectors own immutable values and reveal no entity/store handles. The demo diagnostic renderer, benchmark coverage capture, and an external consumer all call this exact method; there is no crate-private diagnostic feed.

Boundary/property tests call every query at zero/minimum, exact maximum, and one unit beyond each applicable limit; cover negative coordinates, region faces/corners, diagonal ray DDA, zero displacement, maximum legal player and camera shapes, the exact 8,192-candidate sweep estimate, 513 synthetic contacts, 65 column runs, 65 route points, 257 metadata bricks, three cell-bearing bricks, 513 chunks, and 17 focuses; and assert deterministic `QueryError` before any partial output. Brute-force small-world oracles verify hit ordering, safe fractions, and no missed solid/water samples. Instrumented counters assert the advertised candidate/voxel/object-evaluator maxima for both active and inactive bricks.

## World edit protocol

```rust
pub struct WorldEditCommand {
    pub request_id: u64,
    pub operation: EditOperation,
    pub execution: EditExecution,
}

#[derive(SystemParam)]
pub struct WorldEditWrite<'w, 's> { /* private fields */ }

impl WorldEditWrite<'_, '_> {
    pub fn submit(&mut self, command: WorldEditCommand) -> Result<(), SubmitError>;
}

pub enum EditOperation {
    DigSphere {
        center: WorldPointQ8,
        radius_q8: u16,
        strength: u8,
    },
    PlaceSphere {
        center: WorldPointQ8,
        radius_q8: u16,
        strength: u8,
        material: MaterialId,
    },
    DigBox {
        min: VoxelCoord,
        max_exclusive: VoxelCoord,
        strength: u8,
    },
    PlaceBox {
        min: VoxelCoord,
        max_exclusive: VoxelCoord,
        strength: u8,
        material: MaterialId,
    },
}

pub enum EditExecution {
    Atomic,
    Progressive,
}

pub struct EditAccepted {
    pub request_id: u64,
    pub submitted_frame: u64,
    pub estimated_bricks: u32,
}

pub struct EditBatchCommitted {
    pub request_id: u64,
    pub revision: u64,
    pub batch_index: u32,
    pub changed_voxels: u32,
    pub changed_bricks: Vec<BrickCoord>,
    pub submitted_frame: u64,
    pub committed_frame: u64,
    pub remaining_bricks: u32,
}

pub struct EditRejected {
    pub request_id: u64,
    pub reason: EditRejectReason,
}

pub struct EditPrimaryPresentationReady {
    pub request_id: u64,
    pub revision: u64,
    pub ready_frame: u64,
    pub presented_bricks: u32,
    pub remaining_bricks: u32,
}

pub struct EditReconciliationComplete {
    pub request_id: u64,
    pub final_revision: u64,
    pub committed_batches: u32,
    pub changed_voxels: u64,
    pub reconciled_frame: u64,
}
```

The consumer submits `WorldEditCommand` through the public `WorldEditWrite` system parameter; the raw internal envelope/message type is private. `submit` stamps the current rendered-frame index at the call site. Requests are sorted by `request_id` within a drained batch and duplicate IDs in the process lifetime are rejected. `SubmitError::NotReady | QueueFull | LoadInProgress | InvalidBounds | AtomicWorkLimitExceeded` means no request was accepted and no lifecycle messages follow. Spheres accept radii from 0.25 m through 16 m; boxes must be nonempty, in bounds after clipping, and no larger than the configured progressive-operation voxel and brick limits. `Atomic` is accepted only when the conservative affected-brick count is at most `max_atomic_bricks`; larger valid operations require `Progressive`. Placement rejects air, water, unknown materials, and zero strength.

The API deliberately does not contain `Designation`, `Worker`, `Spell`, or `Mana`. A fortress game stores designations and has workers submit bounded edits as labor completes. An RPG submits a progressive sphere or box for destructive magic. Both use the same mutation protocol, revision ordering, backpressure, and observations.

For one accepted request, the library:

1. Validates the shape and execution mode without enumerating the entire operation into an unbounded allocation, reserves bounded queue/accounting capacity, and emits exactly one `EditAccepted`.
2. Enumerates affected bricks in canonical coordinate order. The scheduler interleaves ready requests with weighted round-robin fairness and caps staging, commit, snapshot, extraction, and installation work per frame.
3. Stages one bounded brick batch against a coherent starting revision, commits that batch atomically in `FixedUpdate`, updates deltas, increments the revision once for a nonempty batch, and emits `EditBatchCommitted`. Queries and collision see each committed batch immediately; they never see a partially committed batch.
4. Prioritizes dirty work intersecting active `Traversal`, `Camera`, `Inspection`, or `Mutation` focuses. Once the current committed revision for those primary representations is installed, it emits monotonic `EditPrimaryPresentationReady`; background and distant representations may continue reconciling.
5. Rebuilds all affected terrain/water seams, dressing, registered-object visuals, and active Horizon partitions. Every create/update/removal is tracked through extraction, GPU prepare/free, and render-queue acknowledgement. Exactly one `EditReconciliationComplete` is emitted after every accepted batch and every resulting presentation key reaches the final request revision.

A valid no-effect request emits `EditAccepted`, one zero-change `EditBatchCommitted`, and `EditReconciliationComplete` without fabricating presentation work. Stale async results are discarded and rescheduled against current truth. Cancellation is not part of Product One; backpressure occurs before acceptance, and accepted work must make progress until terminal completion.

There is no API for setting a voxel, submitting a mesh, replacing a brick, or modifying deltas. The selected material and target ray belong to the consumer debug tool, not to the world library.

The 3 m debug carve remains the Product One demonstration, not the substrate ceiling. Gate F2 also submits concurrent worker-sized box edits distributed through a 32 m x 32 m x 16 m volume and one progressive 16 m-radius dig. Acceptance is synchronous and bounded; the performance contract is that every accepted request makes observable truth and presentation progress without starvation, no mutation workload produces a rendered-frame interval above 33.3 ms, and the headed gate meets the throughput/fairness thresholds in [implementation-plan.md](implementation-plan.md). Full reconciliation is measured separately from first progress.

## Activation and inspection

```rust
pub struct FocusSource {
    pub id: u32,
    pub position: WorldPointQ8,
    pub purpose: FocusPurpose,
}

pub enum FocusPurpose { Traversal, Camera, Inspection, Mutation }

pub struct SetFocusSource(pub FocusSource);
pub struct RemoveFocusSource { pub id: u32 }
```

Consumers publish focus messages; they cannot activate or evict individual bricks. The demo maintains stable sources for player and camera and adds a short-lived inspection/mutation focus at the debug ray hit. The streamer combines all sources with configured band radii and priorities. Removing focus may evict derived detail, but committed deltas remain. `active_band` is observational and exists for the visualizer/benchmarks.

An accepted edit pins only its current bounded commit/presentation window: committed dirty bricks, face neighbors, and affected active object/Horizon owners remain pinned until their revision is installed. The entire uncommitted volume is never pinned or materialized at once. An absent Horizon cell is not activated solely for the edit; when later requested it builds from current deltas. Activation pressure cannot evict pinned work or discharge reconciliation by hiding it. Queue saturation returns `SubmitError::QueueFull` before acceptance; accepted requests retain reserved accounting capacity and a fair scheduler share until completion.

## Save/load protocol

```rust
pub struct SaveWorldRequest { pub request_id: u64 }
pub struct SaveWorldCompleted {
    pub request_id: u64,
    pub path: PathBuf,
    pub bytes: u64,
    pub changed_voxels: u32,
}
pub struct SaveWorldFailed { pub request_id: u64, pub error: SaveError }

pub struct LoadWorldRequest { pub request_id: u64 }
pub struct LoadWorldStarted { pub request_id: u64, pub prior_revision: u64 }
pub struct LoadWorldCompleted {
    pub request_id: u64,
    pub revision: u64,
    pub changed_voxels: u32,
}
pub struct LoadWorldFailed { pub request_id: u64, pub error: LoadError }

pub enum WorldTransactionState {
    Idle,
    Loading { request_id: u64, phase: LoadPhase },
}
pub enum LoadPhase { Staging, SwapPending, Rebuilding }
```

Save and load are asynchronous request/result messages so disk I/O never runs in fixed simulation. Save snapshots a sorted immutable delta set at the request's observed revision, encodes/writes it on `AsyncComputeTaskPool`, and reports the exact final byte count. Later edits do not change that in-flight snapshot. Only one disk operation runs at a time; another returns `SaveError::Busy` or `LoadError::Busy`.

Load safety is entirely library-owned and never inspects `DemoState` or any consumer FSM. A ready, idle world accepts one `LoadWorldRequest`, emits `LoadWorldStarted` exactly once, and moves its public read-only transaction state through `Idle -> Loading/Staging -> Loading/SwapPending -> Loading/Rebuilding -> Idle`. During all loading phases, new edits are synchronously rejected with `SubmitError::LoadInProgress`, queued edits accepted before `LoadWorldStarted` finish before staging begins, and save/load requests receive typed `Busy` results. Queries and sweeps continue to return one coherent revision.

Staging decodes and validates without changing current truth. Failure emits exactly one `LoadWorldFailed`, leaves truth/revision unchanged, and returns to `Idle`. Success swaps the complete delta map at one fixed-tick boundary, increments revision once, and invalidates every derived item whose sampled area intersects either the old or loaded delta set. No query can observe a partial map. `LoadWorldCompleted` is emitted only after active collision/query data and active presentation have reached the new revision, then transaction state returns to `Idle`; it is the sole success terminal result. This protocol remains safe if an external consumer keeps moving: a movement tick sees wholly pre-swap or wholly post-swap truth. The demo chooses to freeze controls from request submission through the terminal result for user experience, but that is not a precondition the library attempts to verify.

An absent slot is a valid empty delta set during initial startup and produces no error. Explicit in-session load of an absent slot returns `LoadError::NotFound` so the user action has a visible result.

## Player and camera action contracts

Physical bindings feed three semantic enums:

```rust
pub enum PlayerAction { Move, Sprint, Jump, Orbit, Zoom }
pub enum DebugAction {
    Dig, Place, SelectNextMaterial, SelectPreviousMaterial,
    ToggleBrickBounds, ToggleRawVoxels, ToggleStreamingBands,
    Save, Load,
}
pub enum UiAction { AdjustTimeOfDay, ToggleTimeSliderFocus }
```

`Move` and `Orbit` carry two-axis values; `Zoom` carries a scalar. `AdjustTimeOfDay` carries `-1`, `0`, or `+1` for keyboard/gamepad edge adjustment; the slider widget also emits an absolute normalized `0.0..=1.0` value while pointer-dragged. `Sprint` is held. `Jump`, dig/place, toggles, material selection, save, and load are edge actions latched in `PreUpdate` until one fixed tick consumes them. When multiple fixed ticks run in one render frame, an edge is consumed at most once. When zero fixed ticks run, it remains latched. Time adjustment is per-frame presentation input and is never consumed by fixed simulation.

The HUD always renders a time slider whose endpoints are the configured minimum/maximum hours. Pointer click/drag maps the clamped horizontal position linearly to an absolute hour value. Each negative/positive keyboard or D-pad edge changes the value by configured `time_keyboard_step_hours`, then clamps; there is no key-repeat beyond operating-system/new physical edges. `Tab` toggles slider focus. While focused or pointer-dragged, movement, camera capture, and debug actions are suppressed and their latches cleared; time adjustment and `Tab` remain active. Releasing pointer drag retains focus until `Tab` or clicking outside. Tests cover endpoint mapping, clamping, exact step, focus suppression, and unchanged time without input.

`move_character(intent, body, dt, &WorldRead) -> CharacterStep` is structured around a pure core. For each 1/60 s tick it:

- maps the normalized move vector through camera-relative horizontal axes;
- uses run or sprint speed, never exceeding the configured horizontal speed;
- applies gravity unless paddling and consumes jump only when grounded;
- sweeps the capsule against `QueryMask::SOLID`, resolves contacts with at most the configured iterations, and steps onto solid-collision ledges no taller than 0.3 m;
- clamps the capsule inside region bounds; and
- enters paddling when the capsule overlaps water and the sampled static surface lies between waist and head thresholds. Paddling constrains vertical position to the configured surface offset and supports horizontal movement; it never starts an underwater mode.

The property contract is: the returned capsule overlaps no voxel for which `solid_collision` is true, displacement cannot exceed speed/acceleration bounds for the tick, jump does not occur without grounded state, and paddling references an existing `WaterBodyDef` surface whose sampled voxel satisfies `water_volume`.

Camera orbit is per frame. `solve_camera(target, desired_pose, &WorldRead)` ray/capsule-casts from target to desired camera position against `QueryMask::SOLID` and shortens distance with a safety margin; it never uses terrain mesh triangles or treats water as a wall. Pitch and distance are clamped by config. Smoothing may lag the desired pose but the final camera near plane cannot cross a `solid_collision` sample. Underground light is enabled when the player's ambient/sky exposure probe is below threshold or the player is below the local surface by the configured depth; it is attached to the player and has no gameplay effect.

## Debug tool contract

The demo obtains a target by calling public `ray_cast(..., QueryMask::SOLID)` from the camera. Dig centers at the first solid hit. Place centers one radius along the hit normal from the surface so available space receives material. If there is no in-bounds hit, it submits no command and shows a one-frame `No target` diagnostic.

The tool defaults to a 3 m radius for the signature demonstration and cycles only placeable seed materials. Both dig and place have keyboard bindings in addition to mouse/gamepad bindings. Brick bounds, raw voxels, streaming bands, and time of day are presentation settings; toggles never alter truth. Raw mode pages through `diagnostic_snapshot(include_cells = true)` for active/on-demand inspection detail, displaying every `material_present` cell with a distinct water/solid style; it never eagerly expands the whole region. Brick and streaming views use the same pages' bounds, dirty/pin/task/band/chunk/focus fields. Time changes only the sun/sky presentation and is fixed until another adjustment.

## Benchmark CLI and output

```text
moria-bench --scenario <feasibility-mutation|flythrough|mutation-workloads>
            --output <path.json>
            [--resolution <WIDTHxHEIGHT>]
            [--seed <u64>]
            [--forest-proof <path.json>]
```

Unknown/missing arguments, non-curated seed, invalid resolution, or unwritable output exits with code 2 and prints a concise error. A runtime/contract failure exits 1 after writing a report with `passed: false` and failure reasons when possible. A complete passing run exits 0. The default resolution is 2560x1440 for the discrete target; the Mac acceptance harness explicitly supplies 1920x1080 and 2560x1440 runs. Benchmark runs must be release builds; a debug build records `passed: false` and identifies the profile.

`feasibility-mutation` requires the F1 artifact, exact M4/Metal/release/2560x1440 environment, and matching git/world/manifest digests; it writes the separate `MutationFeasibilityReport` and exercises the full production stages in [implementation-plan.md](implementation-plan.md). The flythrough follows manifest waypoints through every required scene and changes active bands naturally. `mutation-workloads` submits the interactive, colony-volume, and catastrophic public commands through `WorldEditWrite`, observes every lifecycle stage, then saves through `SaveWorldRequest`. The runner writes its selected self-contained JSON report atomically. Metric semantics and durations are in [benchmarks.md](benchmarks.md).

## Errors and observability

All commands/requests carry caller-selected `u64 request_id`. Every accepted edit produces exactly one `EditAccepted`, one or more monotonic `EditBatchCommitted` messages (exactly one zero-change batch for a no-op), zero or more monotonic `EditPrimaryPresentationReady` messages, and exactly one `EditReconciliationComplete`. A synchronous `SubmitError` means it was never accepted and no lifecycle follows. Every save/load request produces exactly one completed/failed terminal result, with `LoadWorldStarted` as a nonterminal acknowledgement. Errors are enums, not log-string contracts. Logs may add context but cannot be the only signal used by the demo or benchmark.

Telemetry is read-only:

```rust
pub struct WorldTelemetryRead<'w, 's> { /* private fields */ }

impl WorldTelemetryRead<'_, '_> {
    pub fn active_counts(&self) -> ActiveCounts;
    pub fn queue_depths(&self) -> QueueDepths;
    pub fn graphics_allocations(&self) -> GraphicsMemoryEstimate;
    pub fn frame_index(&self) -> u64;
    pub fn edit_observations(&self) -> &[EditObservation];
    pub fn transaction_state(&self) -> &WorldTransactionState;
}
```

`edit_observations` is a borrowed chronological view of the latest values in a fixed 256-entry ring; `QueueDepths` includes a monotonic `dropped_edit_observations` count. The demo/benchmark polls every frame and any nonzero drop invalidates its run rather than permitting an unbounded process-lifetime vector. Every other telemetry method is `O(1)` and allocation-free; the slice is `O(1)` to obtain and has at most 256 values. Counts exposed to GPU buffers are checked `u32`; host elapsed times and cumulative frame indices may be `u64`. Telemetry cannot return store references or edit data values. Boundary tests fill 256 entries, insert the 257th, and assert stable chronological wrap plus an incremented drop count.
