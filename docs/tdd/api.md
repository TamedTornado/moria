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

Queries are synchronous, side-effect free from the consumer's perspective, and always overlay committed deltas on deterministic base evaluation. They may populate an internal memoization cache through interior implementation details, but the return value cannot depend on cache state. An in-bounds query never fails merely because a brick is inactive; procedural evaluation supplies the answer. `QueryError::OutOfBounds` is returned outside the region. `QueryError::NotReady` is possible before `WorldReady`. No query returns a mutable reference.

`Vec3Q8` is three signed Q8 metre components. `WorldRayQ8` contains a Q8 origin and normalized Q16 direction. `CapsuleQ8` contains a Q8 center, radius, and vertical half-segment. `SweepResult` contains `safe_fraction_q16: u16` (`0..=65535`), end capsule, and a coordinate-sorted `Vec<WorldHit>`. `WorldBounds` contains min-inclusive/max-exclusive Q8 corners. Constructors validate normalization, nonnegative sizes, and integer overflow before a query runs.

`WorldSample` contains coordinate, material ID, density, state byte, `material_present`, `solid_collision`, `water_volume`, and current world revision. The three booleans are computed by the exact predicates in [data-model.md](data-model.md); they are not aliases. `QueryMask::SOLID` selects only `solid_collision`, `QueryMask::WATER` selects only `water_volume`, and their union selects either; it has no gameplay faction/category semantics. `WorldHit` contains hit voxel, Q8 hit point, a quantized face/gradient normal, material, matched class, distance, and revision.

`sweep_capsule` implements the supported player/camera collision observation. Given start capsule `C` and displacement `D`, it returns the largest collision-free fraction `t in [0, 65535]`, a stable set of contact normals, and end capsule `C + D*t`. With `QueryMask::SOLID` it evaluates only solid collision, not water or render triangles. Ties are sorted by voxel coordinate and normal axis so output is independent of hash/thread order.

The diagnostic observation is a bounded immutable page, not store access:

```rust
pub struct DiagnosticPageRequest {
    pub snapshot: Option<DiagnosticSnapshotToken>, // None starts a snapshot
    pub after_brick: Option<BrickCoord>,
    pub max_bricks: u16,       // 1..=1024; <=64 when include_cells
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

Only currently active or explicitly inspected bricks are enumerable. `DiagnosticCell` contains local index, material, density, `material_present`, `solid_collision`, and `water_volume`; with `include_cells`, all 4,096 cells are returned for each page brick so empty and partial cells remain inspectable. `DiagnosticRenderChunk` contains key, Q8 bounds, LOD, band, revision, and resident/pending phase. `DiagnosticFocus` contains public focus ID, position, and purpose. Render chunks/focuses included are those intersecting or contributing to the returned brick page; an empty terminal page returns the remaining zero-brick focus markers once. The first page's token identifies the active-index generation; if activation, dirty/task state, focus, or revision changes before a later page, that request returns `QueryError::SnapshotExpired` and the caller restarts with `snapshot: None`, preventing mixed-generation pages without retaining an unbounded copy. Internally, ordered active-brick/chunk indices make page construction `O(log A + B + C + F)` without cells and `O(log A + B*4096 + C + F)` with cells, where `B <= max_bricks`; it never scans the 4-billion-voxel region. Returned vectors own immutable values and reveal no entity/store handles. The demo diagnostic renderer, benchmark coverage capture, and an external consumer all call this exact method; there is no crate-private diagnostic feed.

## World edit protocol

```rust
pub struct WorldEditCommand {
    pub request_id: u64,
    pub operation: EditOperation,
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
}

pub struct EditCommitted {
    pub request_id: u64,
    pub revision: u64,
    pub changed_voxels: u32,
    pub changed_bricks: Vec<BrickCoord>,
    pub submitted_frame: u64,
    pub committed_frame: u64,
}

pub struct EditRejected {
    pub request_id: u64,
    pub reason: EditRejectReason,
}

pub struct EditSurfaceReady {
    pub request_id: u64,
    pub revision: u64,
    pub submitted_frame: u64,
    pub committed_frame: u64,
    pub ready_frame: u64,
    pub latency: Duration,
}
```

The consumer submits `WorldEditCommand` through the public `WorldEditWrite` system parameter; the raw internal envelope/message type is private. `submit` stamps the current rendered-frame index at the call site before enqueueing, so the deadline starts in the frame where the consumer action publishes the operation rather than when a later fixed tick drains it. The public `WorldFrameSet::PublishCommands` set runs after action mapping; consumers may submit later in the same frame, but this does not move the stamped deadline. Requests are sorted by `request_id` within a drained batch; duplicate IDs in the process lifetime are rejected. `SubmitError::NotReady | QueueFull | LoadInProgress` means no request was accepted and no edit lifecycle messages follow. Product One accepts radii from 0.25 m through 3 m. This bounds the proof API to the demonstrated maximum and makes the two-frame job budget testable; a larger operation returns `RadiusOutOfRange`. A center may touch the region edge, but only in-bounds voxels are considered. Placement rejects air, water, unknown materials, and a zero strength. Dig erodes any solid material, including registered-object cells, according to hardness; non-ruin authored/voxel-derived swaps and the ruin's always-voxel-derived chunk revisions are part of the completion barrier below.

For one accepted request, the library:

1. Computes the closed spherical voxel set with fixed-point squared-distance comparisons.
2. Stages all new voxel values without exposing a partial revision.
3. Commits a nonempty batch atomically in `FixedUpdate`, updates deltas, and increments the revision once. A valid no-effect batch keeps the current revision.
4. Emits exactly one `EditCommitted`, including a sorted, duplicate-free brick list. Zero-effect valid edits still emit `EditCommitted` with `changed_voxels = 0` and immediately emit `EditSurfaceReady` for the same frame.
5. Makes all queries and collision see the new revision immediately after commit.
6. Rebuilds dirty terrain/water seams and dressing and refreshes every intersected registered-object visual; emits exactly one `EditSurfaceReady` only after all affected terrain, water, object, and dressing presentation for that revision has been installed or removed.

There is no API for setting a voxel, submitting a mesh, replacing a brick, or modifying deltas. The selected material and target ray belong to the consumer debug tool, not to the world library.

The representative operation is `DigSphere { radius_q8: 768, strength: 255 }` (3 m). Every accepted representative edit must satisfy `ready_frame <= submitted_frame + 2` and `ready_frame <= committed_frame + 2` on acceptance hardware. This remains true when its submission frame has zero fixed ticks or when it is submitted after that frame's edit-drain cutoff; the implementation must reserve capacity or return a synchronous `SubmitError` instead of accepting work it cannot budget. The frame counter increments once per render extraction, not per fixed tick. If an accepted task misses the deadline, telemetry records a contract failure; the world remains correct and installs the result rather than showing fabricated geometry.

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

An accepted edit pins its affected bricks and face neighbors until collision truth, meshes, seams, registered-object visuals, and dressing reach the edit revision. Activation pressure cannot evict pinned work. Queue saturation returns `SubmitError::QueueFull` before acceptance; it never accepts an edit it cannot schedule for the two-frame target.

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
moria-bench --scenario <flythrough|carve-storm>
            --output <path.json>
            [--resolution <WIDTHxHEIGHT>]
            [--seed <u64>]
```

Unknown/missing arguments, non-curated seed, invalid resolution, or unwritable output exits with code 2 and prints a concise error. A runtime/contract failure exits 1 after writing a report with `passed: false` and failure reasons when possible. A complete passing run exits 0. The default resolution is 2560x1440 for the discrete target; the Mac acceptance harness explicitly supplies 1920x1080 and 2560x1440 runs. Benchmark runs must be release builds; a debug build records `passed: false` and identifies the profile.

The flythrough follows manifest waypoints through every required scene and changes active bands naturally. The carve storm submits public `WorldEditCommand` values through `WorldEditWrite` at scripted valid hillside targets, waits for each commit/surface-ready pair, then saves through `SaveWorldRequest`. The runner writes one self-contained JSON report atomically. Metric semantics and durations are in [benchmarks.md](benchmarks.md).

## Errors and observability

All commands/requests carry caller-selected `u64 request_id`. Every successfully enqueued edit later produces exactly one `EditCommitted` or `EditRejected`, with surface readiness as an additional lifecycle result for a commit; synchronous `SubmitError` means it was never enqueued. Every save/load request produces exactly one completed/failed terminal result, with `LoadWorldStarted` as a nonterminal acknowledgement. Errors are enums, not log-string contracts. Logs may add context but cannot be the only signal used by the demo or benchmark.

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

Counts exposed to GPU buffers are checked `u32`; host elapsed times and cumulative frame indices may be `u64`. Telemetry cannot return store references or edit data values.
