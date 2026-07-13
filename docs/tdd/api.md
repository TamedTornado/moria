# Public World API

## Consumer boundary

`moria-world` exposes a small Bevy-oriented facade. Internal `BaseRegion`, dense brick caches, edit maps, generators, and mesh jobs are private. The validation demo compiles against only these exports; a dependency rule test scans demo imports to reject internal module paths. There is no `world_mut`, raw voxel slice, mesh-as-collision, or privileged demo hook.

## Plugin and configuration

```rust
pub struct MoriaWorldPlugin {
    pub generation: GenerationConfig,
    pub queries: QueryConfig,
    pub streaming: StreamingConfig,
    pub mutation: MutationConfig,
    pub persistence: PersistenceConfig,
    pub render: WorldRenderConfig,
}

impl Plugin for MoriaWorldPlugin { /* registers API and owned systems */ }
```

The exhaustive public field/type/default/range schemas for all six plugin configs are in `config.md`; constructors use exactly the Product One defaults listed there and boot rejects invalid cross-field combinations. Adding the plugin starts deterministic manifest generation, save-index loading, material/asset loading, and spawn-area activation. It initializes `WorldLifecycle` to `Booting` and publishes readiness/failure events. Consumers may read lifecycle/diagnostics but cannot replace the base or delta store after boot.

## Queries

All query methods are read-only, observe one committed truth revision, and are callable from ordinary systems or pure tests:

```rust
pub trait WorldQuery {
    fn sample_voxel(&self, coord: VoxelCoord) -> Result<VoxelSample, QueryError>;
    fn sample_world(&self, pos: WorldPos) -> Result<VoxelSample, QueryError>;
    fn column(&self, xz_world: Vec2) -> Result<ColumnView, QueryError>;
    fn raycast(&self, ray: WorldRay, max_distance_m: f32, filter: QueryFilter)
        -> Result<Option<WorldHit>, QueryError>;
    fn overlap_aabb(&self, bounds: WorldAabb, filter: QueryFilter)
        -> Result<Vec<OccupiedCell>, QueryError>;
    fn sphere_contacts(&self, sphere: WorldSphere, filter: QueryFilter)
        -> Result<Vec<VoxelContact>, QueryError>;
    fn water_surface(&self, xz_world: Vec2) -> Result<Option<WaterSurfaceHit>, QueryError>;
    fn feature_at(&self, pos: WorldPos) -> Result<Vec<FeatureView>, QueryError>;
    fn landmark(&self, id: LandmarkId) -> Result<LandmarkView, QueryError>;
    fn route(&self, id: RouteId) -> Result<RouteView, QueryError>;
}
```

`QueryFilter` is `Solid`, `Fluid`, `AnyMaterial`, or a set of material IDs. Results include voxel coordinate, exact `VoxelValue`, optional material/feature/object IDs, and queried truth revision. Raycast uses voxel DDA in coordinate order with density/phase filtering and returns the first occupied cell, entry distance, world point, and outward face normal. Equal-boundary ties resolve X then Y then Z for deterministic targeting. `landmark` and `route` expose immutable named metadata/waypoints from the generated manifest so external games and the benchmark can find curated features without internal generator access. A query never requires a brick already active; cache miss invokes the procedural base sampler plus delta lookup.

Bulk enumeration is bounded before sampling. `overlap_aabb` computes the exact half-open voxel AABB and `sphere_contacts` computes its enclosing voxel AABB; either returns `QueryError::QueryTooLarge { requested_cells, max_cells: 262_144 }` without visiting a voxel when the enclosing count exceeds `QueryConfig::max_bulk_scan_cells`. Both stop and return `QueryError::ResultCapacityExceeded { max_results: 65_536 }` if a filter would emit a 65,537th result; an error carries no partial `Vec`. The implementation reserves at most `min(enclosing_cells, max_bulk_results)` result slots, uses a per-call scratch column cache bounded by the same scan limit, and does not activate a brick, allocate a dense brick, or populate a persistent streaming cache. Thus an all-region or adversarial inactive-space request has constant preflight work and bounded allocation; only accepted local queries procedurally sample truth. Collision subdivides a larger sweep, if ever needed, into deterministic adjacent requests that each satisfy these limits and merges contacts in coordinate order.

Errors are `OutOfBounds`, `WorldNotReady`, `InvalidInput` (non-finite position, negative range/radius), `QueryTooLarge`, or `ResultCapacityExceeded`. Ordinary bounded inactive-space queries do not return “not loaded.” Tests issue the complete-region AABB, a sphere enclosing the region, a just-over-limit slab, a maximum accepted empty volume, and a maximum-result checkerboard fixture; they assert preflight rejection performs zero samples/cache inserts and all accepted calls stay within both configured bounds.

## Activation requests

```rust
pub struct ActivationRequest {
    pub owner: ActivationOwner,
    pub focus: WorldPos,
    pub radius_m: f32,
    pub need: ActivationNeed, // Traverse | Display | Inspect | Mutate
    pub ttl_frames: Option<u32>,
}

pub struct ActivationToken(/* opaque */);

pub trait WorldActivation {
    fn request(&mut self, request: ActivationRequest) -> Result<ActivationToken, ActivationError>;
    fn update(&mut self, token: &ActivationToken, request: ActivationRequest)
        -> Result<(), ActivationError>;
    fn release(&mut self, token: ActivationToken);
}
```

Consumers submit/update/drop tokens through `WorldActivation`. The player/camera maintain long-lived traversal/display tokens; a debug target receives a short inspect/mutate token. Requests prioritize caches and representation only and cannot change sampled truth. Radius is capped by the configured far band and invalid/out-of-bounds requests fail explicitly.

## Mutation commands

```rust
pub struct CommandId(pub u64);

pub enum WorldCommand {
    DigSphere {
        id: CommandId,
        center: QuantizedWorldPos,
        radius_voxels: u16,
        strength: u8,
    },
    PlaceSphere {
        id: CommandId,
        center: QuantizedWorldPos,
        radius_voxels: u16,
        strength: u8,
        material: MaterialId,
    },
}

pub trait WorldCommandSink {
    fn submit(&mut self, command: WorldCommand) -> Result<(), CommandError>;
}
```

Submission validates unique command ID, bounds, radius, material phase/placeability, lifecycle readiness, and interactive in-flight limit. Invalid commands change nothing and return `DuplicateId`, `OutOfBounds`, `InvalidRadius`, `InvalidMaterial`, `WorldNotReady`, or `Busy`.

An accepted sphere is one atomic truth transaction. It samples every voxel whose cell overlaps the sphere, applies the phase-specific density formula in `config.md`, updates deltas, increments the world revision once if at least one voxel changes, invalidates affected brick/object/dressing caches and neighbour halos, and emits:

```rust
pub enum WorldEvent {
    MutationAccepted { id: CommandId, revision: u32, accepted_frame: u64,
                       changed_voxels: u32, affected_bricks: Vec<BrickCoord> },
    CollisionReady { id: CommandId, revision: u32, fixed_tick: u64 },
    SurfaceCommitted { id: CommandId, revision: u32, presented_frame: u64,
                       artifacts: u32 },
    MutationRejected { id: CommandId, error: CommandError },
}
```

`CollisionReady` occurs no later than the first fixed tick after acceptance because collision queries use the delta store directly. `SurfaceCommitted` means all currently visible terrain, edited-object, water/raw-view, and dressing-removal artifacts affected by that command match the revision; it must occur at frame delta <= 2 for the representative carve. Offscreen artifacts are marked dirty but do not block the visible commit event. If an artifact becomes visible later, the streaming system must build the latest revision before showing it.

Digging object-backed solid matter uses the same command and absolute world-coordinate deltas. A dig sample whose current phase is `Fluid` or `Empty` is byte-identical before/after, creates no delta, and contributes neither to `changed_voxels` nor artifact invalidation. A sphere containing soil, air, and water therefore changes only its solid samples. A command with zero changed samples still emits `MutationAccepted` and `SurfaceCommitted` for one revision-neutral no-op, with `changed_voxels = 0` and `artifacts = 0`; it does not increment truth revision. There is intentionally no object deletion/felling API and no fluid edit command, and Product One commands cannot create a water delta.

## Persistence commands

```rust
pub enum PersistenceCommand {
    SaveSingleSlot { request_id: u64 },
    LoadSingleSlot { request_id: u64 },
}

pub enum PersistenceEvent {
    SaveCompleted { request_id: u64, path: PathBuf, bytes: u64, revision: u32 },
    LoadCompleted { request_id: u64, changed_bricks: u32, revision: u32 },
    Failed { request_id: u64, error: PersistenceError },
}
```

Save snapshots one complete delta revision and writes atomically in a worker. Edits accepted after the snapshot remain in memory and are not falsely reported saved. Load is allowed only when no mutation is in flight; it atomically replaces the delta set after full validation, invalidates derived artifacts, installs collision truth, rebuilds the current visible set through the revisioned priority path, and only then emits `LoadCompleted`. Player/debug mutation remains suppressed during that interval. Format/error details are in `persistence.md`.

## Diagnostics and inspection

```rust
pub enum DiagnosticMode { BrickBounds, RawVoxels, StreamingBands }

pub struct DiagnosticCommand {
    pub mode: DiagnosticMode,
    pub enabled: bool,
}

pub struct WorldDiagnosticsSnapshot {
    pub truth_revision: u32,
    pub active_by_band: [u32; 4],
    pub uniform_bricks: u32,
    pub dense_bricks: u32,
    pub dirty_bricks: u32,
    pub queued_jobs: u32,
    pub tracked_gpu_bytes: u64,
}
```

Diagnostic toggles are supported world presentation commands, not direct render/storage edits. Brick bounds show 4 m boundaries. Raw mode replaces smooth terrain in the near band with occupied 25 cm cubes colored by material and shares cached cube mesh/material handles. Streaming visualization tints derived representations by active band and shows band radii. Modes can be combined; their overlays are disposable and excluded from persistence/collision.

## Lifecycle and readiness

```rust
pub enum WorldLifecycle {
    Booting,
    Ready { manifest: ManifestSummary },
    Failed(WorldBootError),
}

pub enum WorldBootEvent {
    ManifestReady,
    ControlReady { elapsed_ms: u32 },
    Failed(WorldBootError),
}
```

`ControlReady` requires generation/manifest validation complete, save index/deltas installed if present, player collision neighborhood active, spawn ground query valid, and a near-field presentable surface. Far bands may continue refining after control. `Failed` is terminal and renders a plain error panel; the demo must not spawn a controllable player over incomplete collision truth.

## API property contracts

- Query invariance: activation, camera location, diagnostic mode, and render LOD never alter a query result at a fixed seed/delta revision.
- Base determinism: equal seed/config/coordinate yields byte-identical `VoxelValue` and metadata on supported targets.
- Mutation locality: a sphere command changes no voxel whose cell does not overlap the sphere; only containing bricks and one-halo neighbours become dirty.
- Fluid immutability: for every Product One command sequence, every coordinate whose current phase is fluid before a dig remains byte-identical afterward; place rejects fluid material and can only replace a sample when the formula raises solid density.
- Delta minimality: after any command, every stored entry differs from base and every changed queried coordinate has one stored absolute entry.
- Consumer parity: the demo’s imports and runtime operations are a subset of the exported facade; benchmark scripts also submit public commands and read public events/snapshots.
- Presentation revision: no result older than a brick/object’s requested revision may replace a newer derived artifact.
- Render partition: the `SurfaceOwner` rule in `data-model.md` assigns every visible solid/empty boundary primitive to exactly one terrain brick or base object ID; mutation events dirty every owner whose extraction stencil changed.
