# Public Consumer Contract

The signatures below are normative API shapes. Exact module paths may be
refined during implementation, but ownership, milestones, errors, and bounds
must remain.

## Identities and revisions

```rust
pub struct MoriaHandle { /* Send + Sync */ }
pub struct WorldHandle { /* Send + Sync, opaque generation */ }
pub struct ValidatedMoria { /* consumed installation plan */ }
pub struct MoriaPlugin { /* Bevy Plugin */ }
pub struct CommandPermit { /* owned queue reservation */ }
pub struct QueryPermit { /* owned queue reservation */ }
pub struct CheckpointPermit { /* owned queue reservation */ }
pub struct ExtensionPermit { /* owned queue/job reservation */ }
pub struct EffectBatchPermit { /* owned child-command batch reservation */ }

pub struct WorldId(u64);
pub struct VolumeId(u64);
#[repr(transparent)]
pub struct MaterialId(u16);
pub struct InterestId(u64);
pub struct SubscriberId(u64);
pub struct CommandId(u64);
pub struct QueryId(u64);
pub struct ExtensionId(u64);
pub struct GpuStateId(u64);
pub struct DressingStyleId(u32);
pub struct OperationId(u64);

pub struct WorldKey(uuid::Uuid);
pub struct VolumeKey(uuid::Uuid);
pub struct MaterialKey(uuid::Uuid);
pub struct ExtensionKey(uuid::Uuid);
pub struct DressingStyleKey(uuid::Uuid);
pub struct CheckpointKey(uuid::Uuid);

pub struct VolumeRevision(NonZeroU64);
pub struct ObservationSequence(NonZeroU64);
pub struct DeviceGeneration(NonZeroU64);
```

Runtime IDs are process-local generational handles. A stale ID is rejected
before GPU work. Stable keys are consumer-supplied and persisted. Numeric
runtime IDs and physical slot numbers are never durable.

## Configuration

```rust
pub struct MoriaBuilder { /* registrations */ }

impl MoriaBuilder {
    pub fn new(world: WorldDefinition, config: MoriaConfig) -> Self;
    pub fn register_material(
        &mut self,
        definition: MaterialDefinition,
    ) -> Result<MaterialId, RegistrationError>;
    pub fn register_dressing_style(
        &mut self,
        descriptor: DressingDescriptor,
    ) -> Result<DressingStyleId, RegistrationError>;
    pub fn register_volume(
        &mut self,
        definition: VolumeDefinition,
        source: Arc<dyn BaseContentSource>,
    ) -> Result<VolumeId, RegistrationError>;
    pub fn checkpoint_store(
        &mut self,
        store: Arc<dyn CheckpointStore>,
    ) -> &mut Self;
    pub fn restore_from(
        &mut self,
        request: RestoreRequest,
    ) -> Result<&mut Self, RegistrationError>;
    pub fn validate(&self) -> Result<ValidatedMoria, ConfigurationErrors>;
}

pub struct WorldDefinition {
    pub key: WorldKey,
    pub debug_name: String,              // 1..=96 UTF-8 bytes
}

pub struct BevyInstallation {
    pub plugin: MoriaPlugin,
    pub moria: MoriaHandle,
    pub world: WorldHandle,
    pub startup: Receipt<StartupApplied>,
}

impl ValidatedMoria {
    pub fn into_bevy(self) -> BevyInstallation;
}

pub struct StartupApplied {
    pub world: WorldId,
    pub key: WorldKey,
    pub effective_config: EffectiveConfig,
    pub adapter: AdapterCapabilityReport,
    pub mode: StartupModeApplied,
}

pub enum StartupModeApplied {
    Fresh,
    Restored(RestoreApplied),
}
```

The installation handles exist in `Configured` state so they can be inserted
into consumer resources before `App::add_plugins(installation.plugin)`.
`startup` becomes ready only after the plugin is installed and startup or
restore reaches `Ready`; submitting through the world earlier returns
`WorldNotAccepting`. `ValidatedMoria` is consumed exactly once. The
`test-support` driver consumes it through the same internal installation
routine and exposes no additional consumer operation.

Registration does not allocate GPU storage or invoke a source. Duplicate
stable keys, invalid domains, missing material references, invalid
fingerprints, and impossible limits are registration/configuration failures.

```rust
pub struct MaterialDefinition {
    pub key: MaterialKey,
    pub debug_name: String,              // 1..=96 UTF-8 bytes
    pub presentation: SurfaceDescriptor,
    pub opaque_metadata: Vec<u8>,        // <= config.limits.max_material_metadata_bytes
}

pub struct SurfaceDescriptor {
    pub class: SurfaceClass,
    pub material: SurfaceMaterialInput,
    pub triplanar: Option<TriplanarTextures>,
    pub tint_linear_rgba: [f32; 4],
    pub roughness: f32,                   // finite, 0..=1
    pub metallic: f32,                    // finite, 0..=1
}

pub enum SurfaceMaterialInput {
    Bevy(bevy::asset::Handle<bevy::pbr::StandardMaterial>),
    NeutralDiagnostic,                   // only used when fallback policy permits
}

pub struct TriplanarTextures {
    pub albedo: bevy::asset::Handle<bevy::image::Image>,
    pub normal: Option<bevy::asset::Handle<bevy::image::Image>>,
    pub meters_per_repeat: f32,           // finite and > 0
}

pub enum SurfaceClass {
    Organic,
    Constructed,
}

pub struct DressingDescriptor {
    pub key: DressingStyleKey,
    pub debug_name: String,              // 1..=96 UTF-8 bytes
    pub materials: Vec<MaterialKey>,      // sorted unique, 1..=64
    pub mesh: bevy::asset::Handle<bevy::mesh::Mesh>,
    pub material: bevy::asset::Handle<bevy::pbr::StandardMaterial>,
    pub density_per_square_meter: f32,   // finite, 0..=4,096
    pub coverage_range: [u8; 2],         // inclusive, 128 <= min <= max
    pub scale_range: [f32; 2],           // finite, 0 < min <= max <= 64
    pub yaw_range_radians: [f32; 2],     // finite, min <= max
    pub normal_offset: f32,              // finite local meters
    pub max_instances_per_artifact: u32, // 1..=4,096
}

pub struct VolumeDefinition {
    pub key: VolumeKey,
    pub debug_name: String,
    pub domain: CellAabb,                // finite, min inclusive/max exclusive
    pub cell_size: f32,                  // finite and > 0
    pub mode: VolumeMode,                // Static | Dynamic
    pub initial_placement: RigidPlacement,
    pub lineage: ContentLineage,
    pub reconstruction: ReconstructionFingerprint,
}
```

Opaque metadata is returned only through material registry inspection. Moria
does not interpret it and does not upload it to occupancy kernels.

Surface inputs are embedded by value in each material registration. Dressing
uses a separate builder-time registry because one style may filter several
materials and owns independent mesh/material assets. `materials` is the exact
material filter; every key must resolve by `validate`, and no material outside
that list receives the style. Duplicate style keys, exhausted
`dressing_styles`, invalid ranges, unknown material keys, or a descriptor whose
per-artifact maximum exceeds the configured instance pool are structured
`RegistrationError`/`ConfigurationErrors`. V1 has no runtime presentation
registration or mutation.

Asset handles may be registered before their assets finish loading. A missing
or removed asset makes the affected artifact
`PresentationError::AssetUnavailable { style, asset_kind }`; it does not fail
material truth or collision. `NeutralDiagnostic` is rendered only when the
consumer selected diagnostic fallback.

```rust
pub enum RegistrationError {
    DuplicateMaterial(MaterialKey),
    DuplicateVolume(VolumeKey),
    DuplicateDressingStyle(DressingStyleKey),
    MaterialCapacity { limit: u32 },
    MaterialMetadataCapacity { requested: u64, available: u64 },
    LiveVolumeCapacity { limit: u32 },
    VolumeRecordCapacity { limit: u32 },
    DressingStyleCapacity { limit: u32 },
    InvalidDefinition(Vec<Violation>),
}

pub struct ConfigurationErrors {
    pub violations: Vec<Violation>,      // deterministic field-path order
}

pub struct StaleHandleError {
    pub kind: StaleHandleKind,
}

pub enum StaleHandleKind {
    World,
    Volume,
    Material,
    Subscriber,
    Extension,
    GpuState,
}

pub struct ShaderDiagnostic {
    pub stage: ShaderStage,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub message: String,                 // <=1,024 UTF-8 bytes
}

pub enum ShaderStage {
    Parse,
    Validate,
    Pipeline,
}
```

### Configuration schema

Every field below is public and constructible. `MoriaConfig::default()` supplies
the stated defaults; zero is invalid except where a capability or the optional
dressing subfeature is explicitly disabled below.
Count fields are `u32`, byte fields are `u64`, and conversion to platform
`usize` is checked.

```rust
pub struct MoriaConfig {
    pub capabilities: CapabilityConfig,
    pub limits: ResourceLimits,
    pub overload: OverloadPolicies,
    pub workers: WorkerConfig,
    pub presentation: PresentationConfig,
}

pub struct CapabilityConfig {
    pub presentation: bool,              // default true
    pub persistence: bool,               // default false; must be explicitly enabled
    pub gpu_extensions: bool,             // default false; also needs feature
}

pub enum OverloadPolicy {
    Reject,
    WaitForPermit,
}

pub struct OverloadPolicies {
    pub commands: OverloadPolicy,         // default WaitForPermit
    pub queries: OverloadPolicy,          // default WaitForPermit
    pub checkpoints: OverloadPolicy,      // default Reject
    pub extensions: OverloadPolicy,       // default Reject
}

pub struct WorkerConfig {
    pub content_threads: NonZeroU8,       // default 2, legal 1..=8
    pub persistence_threads: NonZeroU8,   // default 1, legal 1..=8
}

pub struct GpuCapacityLimit {
    pub desired: u32,
    pub minimum: u32,
}

pub struct EffectiveConfig {
    pub requested: MoriaConfig,
    pub capabilities: CapabilityConfig,
    pub overload: OverloadPolicies,
    pub workers: WorkerConfig,
    pub presentation: PresentationConfig,
    pub limits: Vec<EffectiveLimit>,     // exactly one per ResourceKind
}

pub struct EffectiveLimit {
    pub resource: ResourceKind,
    pub requested: u64,
    pub minimum: Option<u64>,
    pub effective: u64,
    pub disposition: LimitDisposition,
}

pub enum LimitDisposition {
    Exact,
    AdapterClamped { adapter_max: u64 },
}

pub struct AdapterCapabilityReport {
    pub backend: String,
    pub adapter: String,
    pub driver: String,
    pub device_type: String,
    pub features: Vec<String>,
    pub downlevel_flags: Vec<String>,
    pub numeric_limits: Vec<(String, u64)>,
    pub software_fallback: bool,
}

pub struct ResourceLimits {
    pub nonempty_materials: u32,
    pub max_material_metadata_bytes: u32,
    pub material_metadata_bytes: u64,
    pub live_volumes: u32,
    pub volume_records: u32,
    pub interest_leases: u32,
    pub bricks_per_interest: u32,
    pub detailed_bricks: GpuCapacityLimit,
    pub page_keys: GpuCapacityLimit,
    pub page_versions: GpuCapacityLimit,
    pub versions_per_brick: u32,
    pub dirty_scar_bricks: GpuCapacityLimit,
    pub command_records: u32,
    pub command_payload_bytes: u64,
    pub query_records: u32,
    pub query_result_bytes: u64,
    pub observation_facts: u32,
    pub observation_payload_bytes: u64,
    pub subscribers: u32,
    pub volumes_per_filter: u32,
    pub staging_maps: u32,
    pub staging_bytes: GpuCapacityLimit, // bytes; v1 hard max fits u32
    pub content_requests: u32,
    pub content_bricks_per_request: u32,
    pub content_response_bytes: u64,
    pub persistence_requests: u32,
    pub persistence_staged_bytes: u64,
    pub extraction_records: u32,
    pub extraction_bytes: u64,
    pub presentation_jobs: u32,
    pub presentation_artifacts: u32,
    pub presentation_dirty_records: u32,
    pub mesh_vertices: u32,
    pub mesh_indices: u32,
    pub dressing_styles: u32,
    pub dressing_instances: GpuCapacityLimit,
    pub extension_jobs: u32,
    pub extension_registrations: u32,
    pub extension_registry_bytes: u64,
    pub extension_packet_bytes: u64,
    pub extension_state_bytes: u64,
    pub extension_candidate_effects: u32,
}

pub struct PresentationConfig {
    pub stale_view_policy: StaleViewPolicy,
    pub retry_count: u8,
    pub diagnostic_fallback: bool,
}
```

`ResourceLimits` has the following fields and relationships. “Hard maximum”
means validation rejects a larger request before startup. An
adapter-negotiated field uses `GpuCapacityLimit`: startup chooses
`effective = min(desired, adapter_legal)` and fails with
`UnsupportedCapabilities` if effective is below `minimum`, below one enabled
maximum legal operation, or violates a cross-limit.

| Field | Default | Hard maximum / relationship |
| --- | ---: | --- |
| `nonempty_materials` | 4,096 | 65,535; empty ID 0 is additional |
| `max_material_metadata_bytes` | 4 KiB | 1 MiB per registration; per-record gauge, not aggregate capacity |
| `material_metadata_bytes` | 16 MiB | 1 GiB retained aggregate; `>= max_material_metadata_bytes` |
| `live_volumes` | 1,024 | 65,535 |
| `volume_records` | 4,096 | 65,535; `>= live_volumes`; counts every live key and retained tombstone for the world's lifetime |
| `interest_leases` / `bricks_per_interest` | 64 / 4,096 | 4,096 / 65,536 |
| `detailed_bricks: GpuCapacityLimit` | 32,768 / 8,192 | `min(u32::MAX, adapter allocation/2,048)`; segmented by binding limit |
| `page_keys: GpuCapacityLimit` | 131,072 / 32,768 | largest power of two within adapter allocation and `u32`; live load <=70% |
| `page_versions: GpuCapacityLimit` | 262,144 / 65,536 | adapter allocation/entry size and `u32`; `>= page_keys`; covers command reservations |
| `versions_per_brick` | 8 | 64 |
| `dirty_scar_bricks: GpuCapacityLimit` | 32,768 / 8,192 | adapter allocation/2,048 and `u32`; `>= max_command_bricks` |
| `command_records` / `command_payload_bytes` | 1,024 / 64 MiB | 65,536 / 1 GiB; records `>= extension_candidate_effects` when enabled; bytes >= maximum patch |
| `query_records` / `query_result_bytes` | 256 / 32 MiB | 16,384 / 1 GiB; bytes >= largest enabled query result |
| `observation_facts` | 4,096 | 1,048,576 |
| `observation_payload_bytes` | 32 MiB | 1 GiB; `>= 64 + 32 * volume_records` so one maximum checkpoint fact fits |
| `subscribers` / `volumes_per_filter` | 64 / 256 | 4,096 / `min(live_volumes, 256)` |
| `staging_maps` / `staging_bytes: GpuCapacityLimit` | 8 / 32 MiB desired, 8 MiB minimum | maps 1..=256; bytes <=1 GiB and adapter allocation; covers largest enabled readback chunk |
| `content_requests` / `content_bricks_per_request` / `content_response_bytes` | 64 / 512 / 32 MiB | 4,096 requests / 4,096 bricks per request / 1 GiB; bytes `>= 2,048 * content_bricks_per_request + 256` |
| `persistence_requests` / `persistence_staged_bytes` | 8 / 64 MiB | 256 / 1 GiB; staged bytes >= 8 MiB chunk decode bound when enabled |
| `extraction_records` / `extraction_bytes` | 2,048 / 32 MiB | 65,536 / 1 GiB; bytes cover one maximum enabled patch or extension input packet plus inline state and records are at least 1 |
| `presentation_jobs` | 1,024 | 65,536; zero only when presentation disabled |
| `presentation_artifacts` / `presentation_dirty_records` | 16,384 / 16,384 | 1,048,576 each; artifacts `>= presentation_jobs`; dirty records `>= presentation_jobs + live_volumes` and reserve one marker per live-volume slot; both zero only when presentation is disabled |
| `mesh_vertices` / `mesh_indices` | 2,097,152 / 12,582,912 | `u32` and adapter allocation bound; each covers one maximum artifact when enabled |
| `dressing_styles` / `dressing_instances: GpuCapacityLimit` | 256 / 1,048,576 desired, 65,536 minimum | 4,096 styles / adapter allocation and `u32` instances; instances cover one descriptor's `max_instances_per_artifact`; both may be zero together to disable dressing only |
| `extension_jobs` | 64 | 4,096; zero only when extensions disabled |
| `extension_registrations` / `extension_registry_bytes` | 32 / 4 MiB | 1,024 / 64 MiB; owns all registered WGSL and entry-point bytes |
| `extension_packet_bytes` / `extension_state_bytes` | 16 MiB / 1 MiB | fixed v1 pool maxima 64 MiB / 4 MiB; state pool holds at least one prior+next pair |
| `extension_candidate_effects` | 256 | fixed v1 maximum 256 and `<= command_records` |

The fixed request maxima remain: 32,768 cells and 512 bricks per matter
command, 16 MiB patch payload, 262,144 cells per region read, 8,192 candidate
bricks and 65,536 candidate cells per collision traversal, 4,096 collision
hits, 256 world-scope volumes, 2,048 vertices/12,288 indices per brick artifact,
13,824 unique halo invalidations per matter command, 4,096 dressing instances
per artifact, 1 MiB WGSL and 128 UTF-8 bytes for one extension entry point, and
256 candidate effects. They are exported in
`contract_limits`; they are not independently configurable.

`PresentationConfig` defaults to `stale_view_policy = DisplayStale`,
`retry_count = 1` (legal `0..=3`), and
`diagnostic_fallback = false`. It contains no camera or content policy.

Supplying a checkpoint store or restore request requires the consumer to set
`capabilities.persistence = true`; neither call mutates the capability flag.
Enabling persistence without a store is a configuration error. Enabling GPU
extensions requires the Cargo feature, nonzero extension limits, extraction
bytes large enough for one maximum configured packet, and a command queue
capable of reserving the configured worst-case batch. Disabling presentation
requires all presentation artifact, dirty, mesh, dressing, and job pool fields
to be zero and makes presentation interest an explicit `CapabilityDisabled`
error.

With presentation enabled, `dressing_styles == 0` is legal only when
`dressing_instances.desired == minimum == 0`; registering a style then returns
`DressingStyleCapacity { limit: 0 }`. With dressing enabled, registry count and
instance desired/minimum are nonzero and the effective instance pool holds at
least one descriptor's `max_instances_per_artifact`. Extension registry bytes
must be at least 1 MiB + 128 bytes when extensions are enabled, so one maximum
legal descriptor is representable.

`EffectiveConfig.requested` mirrors every requested field. Its `limits` vector
records each numeric `ResourceLimits` field in `ResourceKind` order plus
`Exact | AdapterClamped { adapter_max }`; the remaining public config records
are repeated as their exact effective values.
It is returned by startup, available through
`MoriaHandle::effective_config()`, and embedded in telemetry/evidence. Values
not marked adapter-negotiated must equal their request. No clamp can weaken an
enabled operation below its fixed public maximum; such an adapter fails
startup instead.

Metadata registration reserves both one material record and its exact retained
metadata bytes. Exhausting `material_metadata_bytes` returns
`RegistrationError::MaterialMetadataCapacity` without retaining the
definition. `max_material_metadata_bytes` is enforced first. Its telemetry
usage is the largest current record and high-water is the largest record ever
accepted; `MaterialMetadataBytes` reports the aggregate retained pool.

The observation ring owns independent fact-slot and encoded-payload capacities.
Append evicts oldest whole facts until both fit and never splits a fact.
Checkpoint revision vectors encode as 32 bytes per entry plus a 64-byte fact
header, so one maximum legal fact always fits. Subscriber cursor/revision
arrays are fixed-capacity allocations derived from
`subscribers * volumes_per_filter` at startup and do not grow with history.

`content_bricks_per_request` is the exact count bound for each source callback.
Moria partitions larger materialization demand into stable brick-order batches,
with at most `content_requests` callbacks in flight.

```rust
impl MoriaHandle {
    pub fn effective_config(&self) -> Option<EffectiveConfig>;
}
```

## Bounds and coordinates

`CellCoord` and `BrickCoord` are signed local coordinates. `CellAabb` is
half-open and validated with checked arithmetic. `WorldPoint` and
`WorldVector` contain finite `f32` values. World queries transform into each
volume's local address space with its committed placement.

```rust
pub struct CellCoord { pub x: i32, pub y: i32, pub z: i32 }
pub struct BrickCoord { pub x: i32, pub y: i32, pub z: i32 }
pub struct CellAabb { pub min: CellCoord, pub max: CellCoord }
pub struct WorldPoint { pub x: f32, pub y: f32, pub z: f32 }
pub struct WorldVector { pub x: f32, pub y: f32, pub z: f32 }
pub struct WorldAabb { pub min: WorldPoint, pub max: WorldPoint }

pub struct RigidPlacement {
    pub translation: WorldVector,
    pub rotation_xyzw: [f32; 4],
}

pub enum VolumeMode { Static, Dynamic }

pub enum RevisionPrecondition {
    AnyCommitted,
    Exact(VolumeRevision),
}

pub enum MinimumRevision {
    AnyCommitted,
    AtLeast { volume: VolumeId, revision: VolumeRevision },
    Exact { volume: VolumeId, revision: VolumeRevision },
}

#[repr(C)]
pub struct MaterialSample {
    pub material: MaterialId,
    pub coverage: u8,
    pub flags: u8,
}

#[repr(transparent)]
pub struct Correlation([u8; 16]);
```

`Correlation::NONE` is all zeroes; `Correlation::from_bytes` and
`Correlation::as_bytes` construct/inspect the fixed 16-byte value. It is
copied into command, observation, checkpoint diagnostic, and candidate-effect
records. It has no separately allocated payload and is never interpreted by
Moria.

`RigidPlacement` contains translation and a unit quaternion. Construction
normalizes only within a small documented tolerance; zero, non-finite, scale,
and shear inputs are rejected. Static volumes reject placement commands.

Every region method takes explicit bounds. It either accepts the complete
request, rejects it with `SupportedBounds`, or honors an explicit
`PartialPolicy::Allow { max_omitted_regions }`. Silent clipping is forbidden.

## Admission, permits, and receipts

Ingress is bounded independently by record count and owned payload bytes.

```rust
pub enum ReserveError {
    Full { available_records: u32, available_bytes: u64 },
    Closed,
    PayloadTooLarge { requested: u64, limit: u64 },
}

pub enum TryReserveError {
    Full { available_records: u32, available_bytes: u64 },
    Closed,
    PayloadTooLarge { requested: u64, limit: u64 },
}

pub struct ReserveFuture<P> {
    /* Future<Output = Result<P, ReserveError>> + Send */
}

impl WorldHandle {
    pub fn try_reserve_command(
        &self,
        payload_bytes: u64,
    ) -> Result<CommandPermit, TryReserveError>;

    pub fn reserve_command(
        &self,
        payload_bytes: u64,
    ) -> ReserveFuture<CommandPermit>;

    pub fn try_reserve_query(
        &self,
        result_budget_bytes: u64,
    ) -> Result<QueryPermit, TryReserveError>;

    pub fn reserve_query(
        &self,
        result_budget_bytes: u64,
    ) -> ReserveFuture<QueryPermit>;

    pub fn try_reserve_checkpoint(
        &self,
        staged_bytes: u64,
    ) -> Result<CheckpointPermit, TryReserveError>;

    pub fn reserve_checkpoint(
        &self,
        staged_bytes: u64,
    ) -> ReserveFuture<CheckpointPermit>;

    pub fn try_reserve_extension(
        &self,
        job_bytes: u64,
    ) -> Result<ExtensionPermit, TryReserveError>;

    pub fn reserve_extension(
        &self,
        job_bytes: u64,
    ) -> ReserveFuture<ExtensionPermit>;

    pub fn try_reserve_effect_batch(
        &self,
        max_effects: u16,
        command_payload_bytes: u64,
    ) -> Result<EffectBatchPermit, TryReserveError>;

    pub fn reserve_effect_batch(
        &self,
        max_effects: u16,
        command_payload_bytes: u64,
    ) -> ReserveFuture<EffectBatchPermit>;
}
```

A command/query/checkpoint permit reserves one record and the declared bytes in
that operation's queue. An extension permit reserves one job and the complete
job allocation (header, packet, two state ranges, candidates, diagnostics, and
effect payload); it must fit the configured packet/state and descriptor effect
bounds. An `EffectBatchPermit` reserves
`max_effects` ordinary command records, their aggregate encoded payload bytes,
and the same number of child receipt/completion slots. Dropping an unused
permit releases all capacity.

`ReserveFuture<P>` has output `Result<P, ReserveError>`. With the queue's
configured `WaitForPermit`, it waits in bounded FIFO waiter storage; with
`Reject`, it immediately resolves to `ReserveError::Full`. Every `try_` method
is always immediate regardless of policy. Dropping the future removes its
waiter. Each queue has at most its configured record count in waiter slots; an
additional waiter resolves `Full` rather than allocating. Queue close resolves
every waiter as `Closed`. Effect-batch reservation uses the command queue's
overload policy because it reserves ordinary child command capacity.

Submission consumes its permit and owned input. Structural rejection returns
the input unchanged and releases the submitted operation permit's capacity. A
rejected `GpuExtensionRequest` still owns its nested `EffectBatchPermit`, so
the caller may correct/resubmit it or drop it to release child capacity.
Declared bytes are an upper bound; admission rejects an input whose encoded
size exceeds them and releases unused bytes immediately after successful
encoding.

```rust
pub enum SubmitError<T> {
    Invalid { command: T, violations: Vec<Violation> },
    StaleHandle { command: T },
    WorldNotAccepting { command: T, state: WorldState },
    PermitMismatch { command: T },
}

pub struct Receipt<T: Clone> {
    /* Clone + Future<Output = Result<T, OperationError>> + Send + Sync */
}

impl<T: Clone> Receipt<T> {
    pub fn id(&self) -> OperationId;
    pub fn try_status(&self) -> ReceiptStatus<T>;
    pub fn request_cancel(&self) -> CancelRequest;
}

pub enum ReceiptStatus<T> {
    Pending(OperationStage),
    Ready(T),
    Failed(OperationError),
}

pub enum CancelRequest {
    Accepted,
    TooLate { stage: OperationStage },
    NotCancellable,
    AlreadyTerminal,
}

pub enum OperationStage {
    Queued,
    WaitingForMatter,
    Preparing,
    Submitted,
    AwaitingReadback,
    AwaitingPersistence,
    Recovering,
}

pub enum WorldState {
    Configured,
    Starting,
    Ready,
    Recovering,
    ShuttingDown,
    Stopped,
    Failed,
}

pub enum Retryability {
    Never,
    AfterInputChange,
    AfterPressureRelief,
    AfterRecovery,
    Immediate,
}

pub struct OperationError {
    pub operation: OperationId,
    pub scope: OperationScope,
    pub retryability: Retryability,
    pub device_generation: Option<DeviceGeneration>,
    pub revision_changed: bool,
    pub kind: OperationErrorKind,
    pub diagnostic: String,
}

pub enum OperationScope {
    World(WorldId),
    Volume(VolumeId),
    Region { volume: VolumeId, bounds: CellAabb },
    Checkpoint(CheckpointKey),
    Extension(ExtensionId),
}

pub enum OperationErrorKind {
    Validation,
    Conflict { expected: VolumeRevision, current: VolumeRevision },
    Unavailable,
    BudgetExhausted(ResourceKind),
    OutputOverflow { required: u64, limit: u64 },
    Content(ContentErrorKind),
    GpuValidation,
    OutOfMemory,
    DeviceLost,
    Readback,
    Decode,
    Persistence(PersistenceErrorKind),
    Startup(StartupFailure),
    CancelledBeforePreparation,
    ShuttingDown,
    InternalInvariant,
}

pub struct StartupFailure {
    pub stage: StartupStage,
    pub causes: Vec<StartupCause>,       // sorted, <= config fields + required features
}

pub enum StartupStage {
    RendererLookup,
    AdapterQualification,
    DeviceResources,
    PersistenceOpen,
    RestoreRead,
    DirectoryPublication,
}

pub enum StartupCause {
    RendererUnavailable,
    UnsupportedCapabilities {
        adapter: Option<AdapterCapabilityReport>,
        missing_features: Vec<CapabilityRequirement>,
        unmet_limits: Vec<LimitRequirement>,
    },
    Shader(ShaderDiagnostic),
    OutOfMemory,
    Persistence(PersistenceErrorKind),
    Restore(PersistenceErrorKind),
    InternalInvariant,
}

pub struct CapabilityRequirement {
    pub capability: RequiredCapability,
    pub available: bool,
}

pub enum RequiredCapability {
    ComputeShaders,
    StorageBuffers,
    BufferToBufferCopy,
    FourWritableStorageBindings,
}

pub struct LimitRequirement {
    pub resource: ResourceKind,
    pub requested_minimum: u64,
    pub adapter_maximum: Option<u64>,
}

pub struct Violation {
    pub path: String,                    // <=128 UTF-8 bytes
    pub code: ViolationCode,
    pub supported: Option<SupportedBounds>,
    pub diagnostic: String,              // <=512 UTF-8 bytes
}

pub enum ViolationCode {
    InvalidValue,
    InvalidBounds,
    TooLarge,
    MissingIdentity,
    DuplicateIdentity,
    StaleHandle,
    LiveVolumeCapacity,
    VolumeRecordCapacity,
    RetiredVolumeKey,
    CapabilityDisabled,
    CrossLimit,
}

pub struct SupportedBounds {
    pub maximum_records: Option<u32>,
    pub maximum_bytes: Option<u64>,
    pub maximum_cells: Option<u32>,
    pub maximum_bricks: Option<u32>,
    pub maximum_candidate_cells: Option<u32>,
    pub maximum_candidate_bricks: Option<u32>,
    pub maximum_volumes: Option<u16>,
}
```

Receipts are cloneable observers of one shared terminal state. Dropping every
observer does not cancel work. For command, query, checkpoint, and extension
operations, the cancellation linearization point is the atomic transition from
`Queued | WaitingForMatter` to `Preparing`. `request_cancel` wins that race by
installing terminal `CancelledBeforePreparation` and returns `Accepted`; if
preparation won, it returns `TooLate` with the observed stage and the operation
continues. `Accepted` means the terminal state and capacity release are visible
before the method returns. Startup and shutdown receipts return
`NotCancellable`; terminal receipts return `AlreadyTerminal`.

Every cancellable accepted operation starts in `Queued`, may enter
`WaitingForMatter`, and must win exactly one transition to `Preparing` before
it can acquire transaction slots, pin checkpoint versions, freeze a query
snapshot, allocate an extension job, or enter any later family-specific stage.
This rule applies even when an operation needs no GPU dispatch; no family has
an alternate cancellation boundary.

An operation stage is diagnostic, not a latency promise.
`OperationErrorKind` is `#[non_exhaustive]` for source compatibility, while all
v1 variants and their fields are listed above. `Violation` vectors are sorted
by `path` then `code` and bounded by the submitted record's field count; they
never contain arbitrary consumer payloads.

Startup failures use `OperationErrorKind::Startup`; the outer operation error
still supplies world scope, retryability, and diagnostic. Causes are aggregated
rather than fail-fast, sorted by capability/`ResourceKind`/shader stage, and
bounded by the closed required-feature list plus the numeric config fields.
`UnsupportedCapabilities` carries the queried adapter report when one exists
and one `LimitRequirement` for every unmet minimum, so startup never collapses
renderer absence, adapter insufficiency, and restore failure into
`Unavailable`.

A failed matter mutation always reports `revision_changed = false`.

### Facade operations

The following methods are the only admission and inspection facade. `submit_*`
does no hidden waiting; callers acquire the matching permit first.

```rust
impl MoriaHandle {
    pub fn world(&self, id: WorldId) -> Result<WorldHandle, StaleHandleError>;
}

impl WorldHandle {
    pub fn submit_material_registry(
        &self,
        permit: QueryPermit,
        page: RegistryPageRequest,
    ) -> Result<Receipt<MaterialRegistryPage>, SubmitError<RegistryPageRequest>>;
    pub fn material(
        &self,
        id: MaterialId,
    ) -> Result<Arc<MaterialRegistration>, StaleHandleError>;

    pub fn submit_matter(
        &self,
        permit: CommandPermit,
        command: MatterCommand,
    ) -> Result<Receipt<MatterApplied>, SubmitError<MatterCommand>>;

    pub fn submit_volume(
        &self,
        permit: CommandPermit,
        command: VolumeCommand,
    ) -> Result<Receipt<VolumeApplied>, SubmitError<VolumeCommand>>;

    pub fn submit_query(
        &self,
        permit: QueryPermit,
        query: Query,
    ) -> Result<Receipt<QueryOutcome>, SubmitError<Query>>;

    pub fn declare_interest(
        &self,
        request: InterestRequest,
    ) -> Result<InterestLease, InterestError>;

    pub fn subscribe(
        &self,
        subscription: Subscription,
        start: SubscriptionStart,
    ) -> Result<ObservationSubscriber, SubscriptionError>;

    pub fn request_checkpoint(
        &self,
        permit: CheckpointPermit,
        request: CheckpointRequest,
    ) -> Result<Receipt<CheckpointApplied>, SubmitError<CheckpointRequest>>;

    pub fn register_gpu_extension(
        &self,
        descriptor: GpuExtensionDescriptor,
    ) -> Result<ExtensionId, ExtensionRegistrationError>;

    pub fn submit_gpu_extension(
        &self,
        permit: ExtensionPermit,
        request: GpuExtensionRequest,
    ) -> Result<Receipt<GpuExtensionDispatched>, SubmitError<GpuExtensionRequest>>;

    pub fn telemetry(&self) -> TelemetrySnapshot;

    pub fn shutdown(
        &self,
        policy: ShutdownPolicy,
    ) -> Result<Receipt<ShutdownReport>, ShutdownStartError>;

    pub fn shutdown_receipt(&self) -> Option<Receipt<ShutdownReport>>;
}
```

`register_gpu_extension` is available only when the Cargo feature and
configured capability are enabled. `shutdown` is the only accepted operation
that does not use an ordinary queue permit: the world preallocates exactly one
shutdown record during startup, and the first call atomically consumes it.
Later calls return `AlreadyShuttingDown` and may obtain the same receipt
through `WorldHandle::shutdown_receipt()`.

Synchronous facade errors are closed enums:

```rust
pub enum InterestError {
    Invalid(Vec<Violation>),
    Full { limit: u32 },
    StaleHandle,
    WorldNotAccepting(WorldState),
    CapabilityDisabled(InterestCapabilities),
}

pub enum SubscriptionError {
    Invalid(Vec<Violation>),
    Full { limit: u32 },
    StartNotRetained { requested: ObservationSequence, oldest: ObservationSequence },
    WorldNotAccepting(WorldState),
}

pub enum ObservationError {
    Closed,
    StaleSubscriber,
}

pub enum ResumeError {
    SnapshotScopeMismatch,
    SnapshotOlderThanGap,
    NotWaitingForSnapshot,
    StaleSubscriber,
}

pub enum ExtensionRegistrationError {
    CapabilityDisabled,
    DuplicateKey,
    InvalidDescriptor(Vec<Violation>),
    ShaderValidation(ShaderDiagnostic),
    RegistryFull { registrations: u32, limit: u32 },
    RegistryBytes {
        requested: u64,
        available: u64,
        per_descriptor_limit: u64,
    },
    WorldNotAccepting(WorldState),
}

pub enum ShutdownStartError {
    AlreadyShuttingDown,
    StaleWorld,
}
```

Registration/configuration and asynchronous operation errors remain the
structured types specified above and in the failure table; none is replaced by
a string. Every error exposes its stable category plus a human-readable
diagnostic generated from the same fields.

```rust
pub struct RegistryPageRequest {
    pub after: Option<MaterialKey>,
    pub max_records: u32,
    pub max_bytes: u64,
}

pub struct MaterialRegistryPage {
    pub records: Vec<MaterialRegistration>,
    pub registry_digest: [u8; 32],
    pub next_after: Option<MaterialKey>,
}

pub struct MaterialRegistration {
    pub id: MaterialId,
    pub key: MaterialKey,
    pub debug_name: String,
    pub presentation: SurfaceDescriptor,
    pub opaque_metadata: Vec<u8>,
}
```

The request is bounded by `nonempty_materials`, the `QueryPermit`, and
`query_result_bytes`. The page is stable-key sorted and owned. It never splits
a record: if the first eligible registration exceeds `max_bytes`, it returns
terminal `OperationError::OutputOverflow` with the required one-record size.
Repeated pages therefore provide the stated opaque-metadata inspection path
without an unbounded registry allocation or an unreserved concurrent copy.

## Interest

```rust
bitflags! {
    pub struct InterestCapabilities: u8 {
        const INSPECTION = 0b0001;
        const COLLISION  = 0b0010;
        const PRESENTATION = 0b0100;
    }
}

pub struct InterestRequest {
    pub scope: InterestScope,
    pub capabilities: InterestCapabilities,
    pub priority: InterestPriority,      // Background | Normal | Urgent
    pub max_bricks: u32,
}

pub enum InterestScope {
    VolumeLocal { volume: VolumeId, bounds: CellAabb },
    World {
        bounds: WorldAabb,
        volumes: BoundedVolumeFilter,
    },
}

pub enum InterestPriority {
    Background,
    Normal,
    Urgent,
}

pub enum BoundedVolumeFilter {
    Include(Vec<VolumeId>),              // sorted unique, <= volumes_per_filter
    All { max_volumes: u16 },            // 1..=min(256, volumes_per_filter)
}

pub struct AcceptedInterest {
    pub id: InterestId,
    pub scope: InterestScope,
    pub capabilities: InterestCapabilities,
    pub priority: InterestPriority,
    pub resolved_bricks: u32,
    pub regions: Vec<ResolvedInterestRegion>,
}

pub struct ResolvedInterestRegion {
    pub volume: VolumeSnapshotRef,
    pub bricks: Vec<BrickCoord>,         // sorted unique; total <= request.max_bricks
}

pub struct InterestState {
    pub capabilities: Vec<CapabilityReadiness>, // one record per requested bit
}

pub struct CapabilityReadiness {
    pub capability: InterestCapability,
    pub ready_bricks: u32,
    pub total_bricks: u32,
    pub state: CapabilityState,
}

pub enum InterestCapability {
    Inspection,
    Collision,
    Presentation,
}

pub enum CapabilityState {
    Pending,
    Ready { snapshots: Vec<VolumeSnapshotRef> },
    Failed { regions: Vec<UnavailableRegion> },
}

pub struct InterestLease { /* Send + Sync */ }

impl InterestLease {
    pub fn id(&self) -> InterestId;
    pub fn accepted(&self) -> AcceptedInterest;
    pub fn state(&self) -> InterestState;
    pub fn update(
        &self,
        replacement: InterestRequest,
    ) -> Result<AcceptedInterest, InterestError>;
}
```

`declare_interest` validates and returns a lease plus the accepted bounded
scope. Cloning the lease retains interest. `update` atomically replaces the
request after validation. Dropping the last clone withdraws it. Withdrawal
does not cancel commands, invalidate completed results, or discard dirty scars.

Long-lived interest filters are snapshots, not live queries.
`Include` resolves the named live handles and `All` resolves every volume live
at acceptance, sorted by stable key; exceeding `max_volumes` rejects instead of
clipping. A world-scope interest then freezes the exact local brick set
intersecting its world bound at the captured placements. `AcceptedInterest`
exposes those IDs, placements/revisions, and bricks. Later create, retire, or
move commands do not add, substitute, or spatially recompute this set. A
retired member reports `RegionFailureKind::Retired`; it is not replaced by a
new volume. The consumer calls `update` to take a new bounded membership and
placement snapshot.

Interest declarations use the configured `interest_leases` slots and return
`InterestError::Full` synchronously when exhausted; they have no payload queue
and therefore no wait policy. A consumer may retry after receiving a resource-
pressure observation.

Every vector above is bounded by `volumes_per_filter`, `max_bricks`, and the
three-bit capability set. `AcceptedInterest.scope` is the normalized,
fully accepted request (including sorted filters), not a silently clipped
scope. `CapabilityReadiness.ready_bricks == total_bricks` is required for
`Ready`; a failed region is never counted ready.

`Urgent` changes ordering only; it cannot exceed budgets or preempt an admitted
transaction.

## Base content source

```rust
pub trait BaseContentSource: Send + Sync + 'static {
    fn descriptor(&self) -> SourceDescriptor;
    fn load_bricks(
        &self,
        request: BaseBrickRequest,
        cancel: &CancellationToken,
    ) -> Result<BaseBrickBatch, ContentError>;
}

pub struct ContentLineage {
    pub family: uuid::Uuid,
    pub version: u32,
    pub opaque: Vec<u8>,                 // <=256 bytes
}

pub struct ReconstructionFingerprint(pub [u8; 32]);

pub struct SourceDescriptor {
    pub lineage: ContentLineage,
    pub reconstruction: ReconstructionFingerprint,
}

pub struct BaseBrickRequest {
    pub volume: VolumeKey,
    pub lineage: ContentLineage,
    pub reconstruction: ReconstructionFingerprint,
    pub bricks: Vec<BrickCoord>,         // sorted unique, <= content_bricks_per_request
    pub intersections: Vec<CellAabb>,    // one per brick
    pub material_registry_digest: [u8; 32],
    pub maximum_encoded_bytes: u64,
}

pub struct BaseBrickBatch {
    pub descriptor: SourceDescriptor,
    pub bricks: Vec<BaseBrickResult>,    // exact request order/count
    pub encoded_bytes: u64,
}

pub enum BaseBrickResult {
    Homogeneous(MaterialSample),
    Detailed(Box<[MaterialSample; 512]>),
}

pub struct ContentError {
    pub kind: ContentErrorKind,
    pub retryability: Retryability,
    pub diagnostic: String,
}

pub enum ContentErrorKind {
    Unavailable,
    InvalidBatch,
    Cancelled,
    Panicked,
}

pub struct CancellationToken { /* Clone + Send + Sync */ }

impl CancellationToken {
    pub fn is_cancelled(&self) -> bool;
}
```

A request contains one volume key, its lineage/fingerprint, at most
`content_bricks_per_request` sorted unique brick coordinates, the intersected
domain, material registry digest, and maximum encoded bytes. Larger demand is
partitioned in stable coordinate order; one callback never sees a hidden
larger batch. Its worst-case detailed response must fit
`content_response_bytes`. The callback runs on a Moria worker, never a render
or Bevy main thread.

A result has exactly one response for every requested coordinate:
`Homogeneous(MaterialSample)` or `Detailed([MaterialSample; 512])`. Results
outside the domain must be canonical empty. Unknown material IDs, nonzero v1
flags, omitted/duplicate bricks, excess bytes, and descriptor mismatch fail the
whole batch. Failed content is never installed partially.

The source descriptor supplies both:

- `ContentLineage`: stable family/version identity used for migration policy;
- `ReconstructionFingerprint([u8; 32])`: digest identifying the exact base
  inputs/algorithm needed to reproduce unscarred matter.

Restore requires both to match the checkpoint. A source cannot merely assert
lineage compatibility.

## Matter commands

```rust
pub enum MatterCommand {
    Fill {
        volume: VolumeId,
        target: CellAabb,
        sample: MaterialSample,
        precondition: RevisionPrecondition,
        correlation: Correlation,
    },
    Patch {
        volume: VolumeId,
        patch: MaterialPatch,
        precondition: RevisionPrecondition,
        correlation: Correlation,
    },
}

pub struct MaterialPatch {
    pub bounds: CellAabb,
    pub encoding: PatchEncoding,
}

pub enum PatchEncoding {
    Dense(Vec<MaterialSample>),
    Runs(Vec<MaterialRun>),
}

pub struct MaterialRun {
    pub start_index: u32,
    pub length: NonZeroU32,
    pub sample: MaterialSample,
}

pub struct MatterApplied {
    pub command: CommandId,
    pub volume: VolumeId,
    pub affected: CellAabb,
    pub revision: VolumeRevision,
    pub correlation: Correlation,
}
```

Canonical removal is `Fill` with `MaterialSample::EMPTY`; place/replace is
`Fill` with a registered nonempty sample. `Patch` covers consumer stamps.
Patch coordinates are volume-local. Dense order is X-fastest, then Y, then Z.
Dense length must equal the checked cell count. Run indices use that same
linear order; runs must be sorted, non-overlapping, inside the dense length,
and collectively within the configured cell/byte limits. Cells not covered by
a run are unchanged.

One command targets exactly one volume. Admission verifies identity, bounds,
materials, static structure, permit, and current precondition. Cold target
bricks may be admitted and materialized. Immediately before prepare, the
precondition is checked again; an intervening commit produces terminal
`Conflict` with no effect. Per-volume admitted commands prepare in FIFO order.

After all affected current bricks are available, Moria reserves every required
new slot, page node, scar record, and completion record. It writes new versions,
validates the transaction, then publishes one new revision. Failure before
publication releases reservations. Device loss after submission makes that
device generation unavailable and never reports a partial success.

`affected` is the requested intersection after complete-validation; commands
outside the volume are rejected rather than clipped. No-op writes are valid and
still commit a revision because the accepted command and correlation are
observable; telemetry marks `changed_samples = 0`.

## Volume commands

```rust
pub enum VolumeCommand {
    Create {
        definition: VolumeDefinition,
        source: Arc<dyn BaseContentSource>,
        correlation: Correlation,
    },
    Move {
        volume: VolumeId,
        placement: RigidPlacement,
        precondition: RevisionPrecondition,
        correlation: Correlation,
    },
    Retire {
        volume: VolumeId,
        precondition: RevisionPrecondition,
        correlation: Correlation,
    },
}

pub enum VolumeApplied {
    Created {
        command: CommandId,
        volume: VolumeId,
        key: VolumeKey,
        revision: VolumeRevision,
        correlation: Correlation,
    },
    Moved {
        command: CommandId,
        volume: VolumeId,
        placement: RigidPlacement,
        revision: VolumeRevision,
        correlation: Correlation,
    },
    Retired {
        command: CommandId,
        key: VolumeKey,
        terminal_revision: VolumeRevision,
        correlation: Correlation,
    },
}
```

Create reserves a stable/runtime identity and becomes applied only after its
directory entry is committed; content remains cold until interest. Move is
valid only for `Dynamic` and commits the placement at one new volume revision.
It does not resample local cells. Retire rejects new work, waits for admitted
work and checkpoint obligations, commits a tombstone revision, emits an
observation, and invalidates the runtime handle. Durable tombstones prevent a
saved key from being accidentally reused.

`live_volumes` bounds concurrently live directory entries.
`volume_records` separately bounds all stable keys ever accepted by the world,
including current live volumes and retained retirement tombstones. Retiring a
volume releases one live slot but never releases its lifetime record. A create
at the live limit is synchronously invalid with
`ViolationCode::LiveVolumeCapacity`; a create after lifetime-key exhaustion is
`ViolationCode::VolumeRecordCapacity` with the supported limit. Neither enters
GPU work, and a retired key is always `ViolationCode::RetiredVolumeKey`, even
if live capacity is available. The whole-world manifest therefore contains at most
`volume_records` live-plus-tombstone entries.

Create, move, and retire do not share atomicity with matter commands or another
volume. Their receipts state whether a revision changed on failure.

## Queries

```rust
pub enum Query {
    Sample(SampleQuery),
    Region(RegionQuery),
    Occupancy(OccupancyQuery),
    Trace(TraceQuery),
    Overlap(OverlapQuery),
    Sweep(SweepQuery),
    Snapshot(SnapshotQuery),
}

pub struct QueryOptions {
    pub minimum: MinimumRevision,
    pub readiness: ReadinessPolicy,
    pub partial: PartialPolicy,
    pub max_results: u32,
    pub traversal: Option<TraversalAuthorization>,
}

pub struct TraversalAuthorization {
    pub max_candidate_bricks: NonZeroU32, // <= 8,192
    pub max_candidate_cells: NonZeroU32,  // <= 65,536
}

pub enum ReadinessPolicy {
    Pending,
    Materialize,
}

pub enum PartialPolicy {
    Deny,
    Allow { max_omitted_regions: u16 },
}

pub enum QueryScope {
    VolumeCells { volume: VolumeId, bounds: CellAabb },
    WorldBounds {
        bounds: WorldAabb,
        volumes: BoundedVolumeFilter,
    },
}

pub enum SampleAddress {
    VolumeCell { volume: VolumeId, cell: CellCoord },
    WorldPoint {
        point: WorldPoint,
        volumes: BoundedVolumeFilter,
    },
}

pub struct SampleQuery {
    pub address: SampleAddress,
    pub options: QueryOptions,
}

pub enum RegionEncoding {
    Dense,
    HomogeneousRuns,
}

pub struct RegionQuery {
    pub scope: QueryScope,
    pub encoding: RegionEncoding,
    pub options: QueryOptions,
}

pub enum OccupancyTarget {
    Point(SampleAddress),
    Region(QueryScope),
    Shape {
        shape: CollisionShape,
        volumes: BoundedVolumeFilter,
    },
}

pub struct OccupancyQuery {
    pub target: OccupancyTarget,
    pub include_first: u16,              // 0..=min(max_results, 4,096)
    pub options: QueryOptions,
}

pub struct WorldSegment {
    pub start: WorldPoint,
    pub end: WorldPoint,
}

pub struct TraceQuery {
    pub segment: WorldSegment,
    pub volumes: BoundedVolumeFilter,
    pub options: QueryOptions,
}

pub enum CollisionShape {
    Sphere { center: WorldPoint, radius: f32 },
    Aabb(WorldAabb),
    Capsule {
        a: WorldPoint,
        b: WorldPoint,
        radius: f32,
    },
}

pub struct OverlapQuery {
    pub shape: CollisionShape,
    pub volumes: BoundedVolumeFilter,
    pub options: QueryOptions,
}

pub struct SweepQuery {
    pub shape_at_start: CollisionShape,
    pub displacement: WorldVector,
    pub volumes: BoundedVolumeFilter,
    pub options: QueryOptions,
}

bitflags! {
    pub struct SnapshotContents: u8 {
        const VOLUME_STATE = 0b0001;
        const REGION_LIFECYCLE = 0b0010;
        const MATERIAL_SAMPLES = 0b0100;
        const OCCUPANCY = 0b1000;
    }
}

pub enum SnapshotScope {
    SubscriptionGap { subscriber: SubscriberId, max_bricks: u32 },
    Regions(Vec<QueryScope>),            // sorted, non-overlapping, <=256
}

pub struct SnapshotQuery {
    pub scope: SnapshotScope,
    pub contents: SnapshotContents,
    pub options: QueryOptions,
}

pub enum QueryAvailability {
    Ready(QueryResult),
    Pending {
        required: Vec<RequiredRegion>,
        retryability: Retryability,
    },
    Unavailable {
        scopes: Vec<UnavailableRegion>,
        retryability: Retryability,
    },
}

pub struct QueryOutcome {
    pub query: QueryId,
    pub inspected: Vec<InspectedRegion>,
    pub snapshots: Vec<VolumeSnapshotRef>,
    pub device_generation: DeviceGeneration,
    pub completeness: QueryCompleteness,
    pub availability: QueryAvailability,
}

pub enum QueryCompleteness {
    Complete,
    PartialRequested {
        coverage: CoverageMask,
        omitted: Vec<UnavailableRegion>,
    },
}

pub struct CoverageMask {
    pub bricks: Vec<BrickCoverage>,
}

pub struct BrickCoverage {
    pub volume: VolumeId,
    pub brick: BrickCoord,
    pub inspected: bool,
}

pub struct RequiredRegion {
    pub scope: QueryScope,
    pub reason: RequiredReason,
}

pub enum RequiredReason {
    Cold,
    Materializing,
    MinimumRevision,
}

pub struct UnavailableRegion {
    pub scope: QueryScope,
    pub reason: UnavailableReason,
}

pub enum UnavailableReason {
    FailedContent,
    RetiredVolume,
    RevisionUnavailable,
    CapabilityDisabled,
    Budget,
    DeviceUnavailable,
}

pub struct InspectedRegion {
    pub scope: QueryScope,
    pub revision: VolumeRevision,
}

pub struct VolumeSnapshotRef {
    pub volume: VolumeId,
    pub key: VolumeKey,
    pub revision: VolumeRevision,
    pub placement: RigidPlacement,
}

pub enum QueryResult {
    Samples(Vec<SampleFact>),
    Region(RegionSamples),
    Occupancy(OccupancyFact),
    Trace(Vec<ContactFact>),
    Overlap(Vec<ContactFact>),
    Sweep(Vec<ContactFact>),
    Snapshot(WorldSnapshot),
}

pub struct SampleFact {
    pub volume: VolumeId,
    pub revision: VolumeRevision,
    pub cell: CellCoord,
    pub sample: MaterialSample,
    pub world_center: WorldPoint,
}

pub struct RegionSamples {
    pub volumes: Vec<VolumeRegionSamples>,
}

pub struct VolumeRegionSamples {
    pub volume: VolumeId,
    pub revision: VolumeRevision,
    pub bounds: CellAabb,
    pub encoding: RegionSampleEncoding,
}

pub enum RegionSampleEncoding {
    Dense(Vec<MaterialSample>),
    HomogeneousRuns(Vec<RegionRun>),
}

pub struct RegionRun {
    pub start_index: u32,
    pub length: NonZeroU32,
    pub sample: MaterialSample,
}

pub struct OccupancyFact {
    pub occupied: bool,
    pub first: Vec<OccupiedCellFact>,
}

pub struct OccupiedCellFact {
    pub volume: VolumeId,
    pub revision: VolumeRevision,
    pub cell: CellCoord,
    pub sample: MaterialSample,
}

pub struct ContactFact {
    pub volume: VolumeId,
    pub revision: VolumeRevision,
    pub material: MaterialId,
    pub cell: CellCoord,
    pub world_point: WorldPoint,
    pub world_normal: WorldVector,
    pub penetration_or_toi: f32,
}

pub struct WorldSnapshot {
    pub scope: SnapshotScope,
    pub resolved_subscription: Option<AcceptedSubscription>,
    pub observation_head: ObservationSequence,
    pub volumes: Vec<VolumeStateSnapshot>,
    pub regions: Vec<RegionStateSnapshot>,
    pub samples: Vec<SampleFact>,
    pub occupancy: Vec<OccupiedCellFact>,
    pub resume: Option<GapResumeToken>,
}

pub struct VolumeStateSnapshot {
    pub volume: VolumeId,
    pub key: VolumeKey,
    pub revision: VolumeRevision,
    pub placement: RigidPlacement,
    pub mode: VolumeMode,
}

pub struct RegionStateSnapshot {
    pub volume: VolumeId,
    pub bounds: CellAabb,
    pub state: RegionLifecycleState,
}

pub enum RegionLifecycleState {
    Cold,
    Requested,
    Materializing,
    Ready { revision: VolumeRevision },
    Retiring,
    Failed { failure: RegionFailure },
}

pub struct RegionFailure {
    pub kind: RegionFailureKind,
    pub retryability: Retryability,
    pub device_generation: Option<DeviceGeneration>,
    pub diagnostic: String,              // <=512 UTF-8 bytes
}

pub enum RegionFailureKind {
    Content(ContentErrorKind),
    InvalidContent,
    BudgetExhausted(ResourceKind),
    DeviceLost,
    RevisionUnavailable,
    Retired,
    InternalInvariant,
}

pub struct GapResumeToken { /* opaque subscriber/scope/head digest */ }
```

Every concrete query embeds exactly one `QueryOptions`; there is no implicit
default attached by `submit_query`. `max_results` must be nonzero except for an
occupancy request with `include_first = 0`, and cannot exceed the fixed limit
for that query family or its permit bytes. World filters, query scopes, and
snapshot region vectors are normalized and validated before admission.
Collision inputs require finite values, positive radii, a nonzero trace segment
or sweep displacement, and complete shape bounds inside the supported checked
coordinate range.

Shape/region occupancy, trace, overlap, and sweep require
`Some(TraversalAuthorization)`; point occupancy and noncollision query
families require `None`. Before admission, the control plane transforms the
finite world bounds into each selected volume, intersects its domain, and
checked-sums the conservative brick-AABB and cell-AABB counts across volumes.
Both totals must fit the request authorization and the fixed 8,192-brick /
65,536-cell maxima. The authorization therefore bounds worst-case traversal
work even when occupancy is sparse or `max_results` is small. An excess is a
synchronous `ViolationCode::TooLarge` whose `SupportedBounds` reports both
candidate maxima; partial coverage never authorizes extra traversal.

All query outcomes include query ID, actual inspected bounds, device
generation, a sorted `VolumeSnapshotRef` vector, completeness, and
result-specific facts.

- `Sample` returns every volume sample covering a world point, or the one
  addressed local sample. Overlapping volumes are preserved.
- `Region` returns row-major samples or homogeneous runs for a bounded
  volume-local region. World-region queries return records grouped by volume;
  they never merge materials.
- `Occupancy` returns occupied/unoccupied only when every required sample is
  ready. It can optionally return the first occupied facts.
- `Trace` returns ordered cell encounters along a finite segment.
- `Overlap` tests a supported shape at one placement.
- `Sweep` tests a supported shape along a finite displacement and reports
  time-of-impact facts without moving anything.
- `Snapshot` returns lifecycle/revision/placement summaries and may include
  bounded material data for observation-gap recovery.

`SnapshotScope::SubscriptionGap` is accepted only while that subscriber is in
`NeedsSnapshot`. Its result has `resolved_subscription = Some` containing the
exact accepted subscription and pinned volume IDs, and `GapResumeToken` binds
subscriber ID, resolved-scope digest, gap head, captured observation head, and
captured revisions. Explicit region snapshots have
`resolved_subscription = None` and `resume = None`. `resume_after` rejects a token from another subscriber,
scope, or an older gap; this is the complete race-closing contract and does not
depend on caller-provided sequence arithmetic.

Supported collision shapes are sphere, axis-aligned box, and capsule. Inputs
are finite and nondegenerate. Trace/sweep results sort by parametric distance,
then stable volume key, then local cell coordinate. Coincident overlaps are all
retained up to the explicit result cap.

If readiness is `Pending`, cold data yields `QueryAvailability::Pending` with
the required regions and no fabricated facts. `Materialize` creates internal
query interest bounded by the query permit. It does not evade interest or
residency budgets.

The `Query` variant and `QueryResult` variant must match as follows:
`Sample -> Samples`, `Region -> Region`, `Occupancy -> Occupancy`,
`Trace -> Trace`, `Overlap -> Overlap`, `Sweep -> Sweep`, and
`Snapshot -> Snapshot`. Pending/unavailable outcomes contain no result variant.
All vectors are bounded by the permit, `max_results`, and fixed request
maxima; decoding an excess count is `OutputOverflow`.

`PartialPolicy::Deny` either returns complete facts or a non-success
availability/error. `Allow` returns one `BrickCoverage` for every resolved
brick, explicit unavailable omitted regions, and
`Complete | PartialRequested`; the coverage vector is bounded by the request's
resolved brick maximum. Hitting a result cap without prior partial
authorization is `OutputOverflow`, not success. In v1, partial means only that
explicitly named unavailable spatial regions were omitted. It never permits
hit truncation inside an inspected brick: if collision output exceeds
`max_results`, the receipt fails `OutputOverflow { required, limit }` under
both partial policies and returns no `QueryResult`.

## Observation

```rust
pub struct Subscription {
    pub volumes: BoundedVolumeFilter,
    pub bounds: Option<ObservationBounds>,
    pub kinds: ObservationKinds,
}

pub enum ObservationBounds {
    VolumeLocal { volume: VolumeId, bounds: CellAabb },
    World(WorldAabb),
}

bitflags! {
    pub struct ObservationKinds: u16 {
        const MATTER = 0x0001;
        const VOLUME = 0x0002;
        const LIFECYCLE = 0x0004;
        const PRESENTATION = 0x0008;
        const CHECKPOINT = 0x0010;
        const PRESSURE = 0x0020;
        const DEVICE = 0x0040;
    }
}

pub enum ObservationItem {
    Fact(Observation),
    Gap(ObservationGap),
}

pub struct Observation {
    pub sequence: ObservationSequence,
    pub fact: ObservationFact,
}

pub enum ObservationFact {
    MatterCommitted {
        command: CommandId,
        volume: VolumeId,
        affected: CellAabb,
        revision: VolumeRevision,
        correlation: Correlation,
    },
    VolumeCreated {
        command: CommandId,
        volume: VolumeId,
        key: VolumeKey,
        revision: VolumeRevision,
        correlation: Correlation,
    },
    VolumeMoved {
        command: CommandId,
        volume: VolumeId,
        revision: VolumeRevision,
        placement: RigidPlacement,
        correlation: Correlation,
    },
    VolumeRetired {
        command: CommandId,
        key: VolumeKey,
        terminal_revision: VolumeRevision,
        correlation: Correlation,
    },
    RegionLifecycle(RegionStateSnapshot),
    Presentation {
        volume: VolumeId,
        brick: BrickCoord,
        state: PresentationState,
    },
    Checkpoint {
        key: CheckpointKey,
        outcome: CheckpointObservation,
    },
    ResourcePressure(ResourcePressureFact),
    Device {
        generation: DeviceGeneration,
        state: DeviceObservationState,
    },
}

pub enum CheckpointObservation {
    Durable { revisions: Vec<(VolumeKey, VolumeRevision)> },
    Failed {
        kind: PersistenceErrorKind,
        retryability: Retryability,
    },
}

pub struct ResourcePressureFact {
    pub resource: ResourceKind,
    pub used: u64,
    pub limit: u64,
    pub action: PressureAction,
}

pub enum PressureAction {
    Deferred,
    Rejected,
    Coalesced,
    EvictedDerived,
    EvictedAuthority,
}

pub enum ResourceKind {
    Materials,
    MaterialMetadataPerRegistration,
    MaterialMetadataBytes,
    LiveVolumes,
    VolumeRecords,
    InterestLeases,
    BricksPerInterest,
    DetailedBricks,
    PageKeys,
    PageVersions,
    VersionsPerBrick,
    DirtyScars,
    CommandRecords,
    CommandPayloadBytes,
    QueryRecords,
    QueryResultBytes,
    ObservationFacts,
    ObservationPayloadBytes,
    Subscribers,
    VolumesPerFilter,
    StagingMaps,
    StagingBytes,
    ContentRequests,
    ContentBricksPerRequest,
    ContentResponseBytes,
    PersistenceRequests,
    PersistenceStagedBytes,
    ExtractionRecords,
    ExtractionBytes,
    PresentationJobs,
    PresentationArtifacts,
    PresentationDirtyRecords,
    MeshVertices,
    MeshIndices,
    DressingStyles,
    DressingInstances,
    ExtensionJobs,
    ExtensionRegistrations,
    ExtensionRegistryBytes,
    ExtensionPacketBytes,
    ExtensionStateBytes,
    ExtensionCandidateEffects,
}

pub enum DeviceObservationState {
    Lost,
    Recovering,
    Ready,
    Failed,
}

pub struct ObservationGap {
    pub last_delivered: Option<ObservationSequence>,
    pub oldest_retained: ObservationSequence,
    pub current_head: ObservationSequence,
    pub scope: AcceptedSubscription,
    pub trustworthy_revisions: Vec<(VolumeId, VolumeRevision)>,
}

pub struct AcceptedSubscription {
    pub request: Subscription,
    pub volumes: Vec<VolumeSnapshotRef>, // stable-key sorted snapshot
}

pub enum SubscriptionStart {
    CurrentHead,
    Retained(ObservationSequence),
}

pub struct ObservationSubscriber { /* Send + Sync, bounded cursor */ }

impl ObservationSubscriber {
    pub fn id(&self) -> SubscriberId;
    pub fn accepted(&self) -> AcceptedSubscription;
    pub fn try_next(&self) -> Result<Option<ObservationItem>, ObservationError>;
    pub fn resume_after(
        &self,
        snapshot: &WorldSnapshot,
    ) -> Result<(), ResumeError>;
}
```

Facts cover committed matter, volume create/move/retire, lifecycle changes,
presentation status, checkpoint completion/failure, resource pressure, and
device recovery. Every fact has a world sequence; change facts also carry the
relevant volume revision and correlation.

Subscriptions are bounded by volume count and one optional spatial bound. Each
subscriber has a cursor into a shared configured ring. Polling never blocks a
commit. If overwritten, the next item is exactly one `Gap` containing the last
delivered sequence, current oldest sequence, current head, affected subscription
scope, and last trustworthy revisions known at the cursor. The subscriber must
obtain a bounded `Snapshot` and call `resume_after(snapshot)`; no later facts
are delivered before that.

Subscription volume membership is snapshotted at `subscribe`. `Include`
captures the named live handles and `All` captures every then-live volume,
stable-key sorted; `max_volumes` is checked against that complete set. Later
creates are excluded, retirement of a captured volume is delivered when
`VOLUME` is selected and then leaves that pinned member terminal, and no new
volume substitutes for it. `accepted()` and every gap expose the resolved
membership.

Unlike interest brick residency, an optional subscription spatial bound is an
event predicate over that pinned membership: matter/lifecycle/presentation
facts match their affected world bounds at the fact revision, and a move fact
matches when either its prior or new placed domain intersects the bound. Thus a
captured dynamic volume can move into or out of the bound without changing the
finite ID set. To include later-created volumes, the consumer creates a new
subscription and closes/drops the old subscriber.

Checkpoint revision vectors and gap revision vectors are stable-key/runtime-ID
sorted and bounded by `volume_records` and `volumes_per_filter` respectively.
Checkpoint vectors and all other variable retained fact payloads are charged
to `observation_payload_bytes`; overwrite advances on whole facts until both
ring count and byte capacity are available. Gap vectors are materialized from
the subscriber's fixed-capacity revision array reserved at subscription time,
not from an unbounded allocation.
`ResourceKind` is the closed set of every field in `ResourceLimits`, so
pressure on extraction, lifetime volume records, presentation artifacts/dirty
records/instances, or extension registry storage is observable through the
same fact.

Observations are not a command bus. A subscriber receives no storage or
mutation privilege.

## Presentation API

Presentation is requested through interest. Consumers register material
surface inputs and choose:

```rust
pub enum StaleViewPolicy {
    DisplayStale,
    HideUntilCurrent,
    DiagnosticBounds,
}

pub enum PresentationState {
    Absent,
    Building { target: VolumeRevision },
    Current { source: VolumeRevision },
    Stale { visible: VolumeRevision, target: VolumeRevision },
    Failed { target: VolumeRevision, error: PresentationError },
}

pub enum PresentationError {
    AssetUnavailable {
        style: Option<DressingStyleKey>,
        asset_kind: PresentationAssetKind,
    },
    OutputOverflow { resource: ResourceKind, required: u64, limit: u64 },
    InvalidGeometry,
    GpuValidation,
    DeviceLost,
}

pub enum PresentationAssetKind {
    SurfaceMaterial,
    TriplanarAlbedo,
    TriplanarNormal,
    DressingMesh,
    DressingMaterial,
}
```

The Bevy adapter owns render entities and mesh assets and tags them with opaque
volume/brick/revision components for diagnostics. These components do not grant
query access. Surface registration is the `MaterialDefinition.presentation`
value, and dressing registration is
`MoriaBuilder::register_dressing_style`; there is no unnamed presentation
registration path. When presentation is enabled, validation requires every
style filter key to resolve and every material to have a valid surface
descriptor before presentation interest can become ready.

Each dressing descriptor is fixed-size except its 1..=64 material-key filter
and 96-byte name. Descriptor count is bounded by `dressing_styles`; generated
instances are bounded by both `max_instances_per_artifact` and the global
`dressing_instances` pool. Dressing has no occupancy. Matter-backed objects use
ordinary volume creation instead.

## Persistence API

```rust
pub struct ChunkDigest(pub [u8; 32]);

pub struct PersistenceError {
    pub kind: PersistenceErrorKind,
    pub retryability: Retryability,
    pub diagnostic: String,
}

pub enum PersistenceErrorKind {
    NotFound,
    Io,
    UnsupportedDurability,
    UnexpectedEof,
    SizeChanged,
    Bounds,
    SizeLimit,
    Corrupt,
    UnsupportedVersion { saved: u16, supported: u16 },
    RestoreMismatch(RestoreMismatch),
    Panicked,
}

pub enum RestoreMismatch {
    WorldKey,
    MaterialMissing(MaterialKey),
    MaterialDefinition(MaterialKey),
    VolumeMembership,
    TombstonedVolume(VolumeKey),
    VolumeDefinition(VolumeKey),
    Lineage(VolumeKey),
    ReconstructionFingerprint(VolumeKey),
}

pub trait CheckpointStore: Send + Sync + 'static {
    fn begin(&self, checkpoint: CheckpointKey)
        -> Result<Box<dyn CheckpointWriter>, PersistenceError>;
    fn open(&self, checkpoint: CheckpointKey)
        -> Result<Box<dyn CheckpointReader>, PersistenceError>;
}

pub trait CheckpointWriter: Send {
    fn put_chunk(&mut self, id: ChunkDigest, bytes: &[u8])
        -> Result<(), PersistenceError>;
    fn commit_manifest(self: Box<Self>, bytes: &[u8])
        -> Result<(), PersistenceError>;
    fn abort(self: Box<Self>) -> Result<(), PersistenceError>;
}

pub trait CheckpointReader: Send {
    fn manifest_len(&mut self) -> Result<u64, PersistenceError>;
    fn read_manifest(
        &mut self,
        offset: u64,
        destination: &mut [u8],
    ) -> Result<(), PersistenceError>;
    fn chunk_len(&mut self, id: ChunkDigest) -> Result<u64, PersistenceError>;
    fn read_chunk(
        &mut self,
        id: ChunkDigest,
        offset: u64,
        destination: &mut [u8],
    ) -> Result<(), PersistenceError>;
}
```

`commit_manifest` is the atomic durability point. After it succeeds, every
referenced chunk must be durable and readable. A store incapable of this
contract is rejected. Moria includes a native filesystem store using sibling
temporary files, file sync, atomic rename, and parent-directory sync where the
platform supports it.

Reader lengths are authoritative size discovery and may be queried more than
once. Reads must fill the complete destination or return
`PersistenceError::UnexpectedEof`; `offset + destination.len()` is checked and
must not exceed the discovered length. The reader retains ownership of store
handles, while Moria owns each bounded destination buffer from its persistence
staging pool. A manifest over `max_manifest_bytes` (64 MiB v1), a chunk over
4 MiB encoded, a changing reported length, missing data, or backend I/O failure
is returned as a distinct persistence error before allocation/decoding. Reader
methods execute only on persistence workers and never receive a Moria storage
handle.

```rust
pub enum CheckpointScope {
    WholeWorld,
}

pub struct CheckpointRequest {
    pub key: CheckpointKey,
    pub scope: CheckpointScope,
}

pub struct CheckpointApplied {
    pub key: CheckpointKey,
    pub durable: Vec<(VolumeKey, VolumeRevision)>,
    pub manifest: ChunkDigest,
}

pub struct RestoreRequest {
    pub checkpoint: CheckpointKey,
    pub world: RestoreWorldMode,
}

pub enum RestoreWorldMode {
    RequireSameKey,
    ImportAs(WorldKey),
}

pub struct RestoreApplied {
    pub checkpoint: CheckpointKey,
    pub saved_world: WorldKey,
    pub active_world: WorldKey,
    pub imported: bool,
    pub revisions: Vec<(VolumeKey, VolumeRevision, RigidPlacement)>,
    pub manifest: ChunkDigest,
}
```

The checkpoint frontier is captured when the request is admitted. Later
commits remain dirty and are excluded. V1 checkpoints are whole-world only:
the manifest contains every live volume at the captured frontier plus every
known retirement tombstone. It cannot omit a live volume, and no partial-scope
variant is reserved.

`restore_from` selects the builder's startup mode and may be called once.
`RequireSameKey` requires manifest and `WorldDefinition` keys to match.
`ImportAs(k)` requires the builder world key to equal `k`, preserves material
and volume keys, and changes only the containing world identity. Restore
validates all registrations and base fingerprints before publishing any
volume. Its output is `StartupApplied::mode =
StartupModeApplied::Restored(RestoreApplied)`; startup failure uses the normal
receipt error with a restore-specific stage and publishes no world directory.

The current live volume registration set must exactly equal the manifest's
live volume key set. Missing and extra current volumes are both
`RestoreMismatch::VolumeMembership`; tombstoned keys may not be registered.
Every persisted material must have a matching current key and
occupancy-relevant definition. Extra current materials are allowed regardless
of presentation inputs, because no persisted sample refers to them; they must
have distinct keys and valid ordinary definitions. There is no
“presentation-only material” category in v1.

## GPU behavior extension

The optional `gpu-extension` feature is deliberately descriptor based:

```rust
pub struct GpuExtensionDescriptor {
    pub key: ExtensionKey,
    pub wgsl: String,                    // 1..=1 MiB UTF-8 bytes
    pub entry_point: String,             // 1..=128 UTF-8 bytes
    pub max_invocations: u32,            // 1..=1,048,576
    pub max_inspection_records: u32,     // 1..=262,144
    pub max_candidate_effects: u32,      // 0..=256
    pub max_effect_payload_bytes: u32,   // 0..=command_payload_bytes
    pub state_bytes: u32,                // 0..=min(4 MiB, extension_state_bytes / 2)
}

pub enum GpuInspectionQuery {
    Samples {
        scope: QueryScope,
        maximum_records: u32,
        minimum: MinimumRevision,
        readiness: ReadinessPolicy,
    },
    Occupancy {
        scope: QueryScope,
        maximum_records: u32,
        traversal: TraversalAuthorization,
        minimum: MinimumRevision,
        readiness: ReadinessPolicy,
    },
    Lifecycle {
        scope: QueryScope,
        maximum_records: u32,
    },
    ObservationDeltas {
        subscriber: SubscriberId,
        after: ObservationSequence,
        maximum_records: u32,
    },
}

pub enum GpuStateInput {
    Zeroed,
    Inline(Vec<u8>),
    Previous(GpuStateOutput),
}

pub struct GpuExtensionRequest {
    pub extension: ExtensionId,
    pub query: GpuInspectionQuery,
    pub opaque_state: GpuStateInput,
    pub effect_batch: EffectBatchPermit,
}

pub struct GpuExtensionDispatched {
    pub extension: ExtensionId,
    pub snapshot: Vec<VolumeSnapshotRef>,
    pub diagnostics: ExtensionDiagnostics,
    pub state: Option<GpuStateOutput>,
    pub effects: Vec<AdmittedEffect>,
}

pub struct ExtensionDiagnostics {
    pub flags: u32,
    pub counters: [u32; 8],
    pub inspection_records: u32,
    pub produced_effects: u32,
    pub effect_payload_bytes: u32,
}

pub struct GpuStateOutput {
    /* Clone + Send + Sync lease */
    pub id: GpuStateId,
    pub extension: ExtensionId,
    pub device_generation: DeviceGeneration,
    pub bytes: u32,
}

pub enum AdmittedEffect {
    Matter {
        command: CommandId,
        receipt: Receipt<MatterApplied>,
    },
    Volume {
        command: CommandId,
        receipt: Receipt<VolumeApplied>,
    },
}
```

Registration is world-lifetime and consumes one `extension_registrations`
record plus `wgsl.len() + entry_point.len()` from
`extension_registry_bytes`; there is no unbounded pipeline or descriptor
cache. Reaching either limit returns the corresponding
`ExtensionRegistrationError` without compiling a pipeline. Registration
validates WGSL and the fixed ABI. A request captures exactly one closed
`GpuInspectionQuery` into an extension-owned packet. `maximum_records` must be
nonzero and fit the packet permit, descriptor maximum, and fixed public query
limit; unsupported partial coverage is rejected rather than clipped.

When `descriptor.state_bytes > 0`, `GpuStateInput::Inline` must contain exactly
that many bytes; `Zeroed` creates that many zero bytes, and `Previous` must name the same
extension, byte count, world, and current device generation. Shader-written
state remains in an extension-owned GPU buffer and returns as an opaque
`Some(GpuStateOutput)`; it is accepted by a later request but grants no buffer
access. A state ID becomes stale on device loss or world shutdown. State is
external-behavior working data, is not checkpointed, and never affects Moria
truth unless a candidate effect is admitted. For `state_bytes == 0`, only
`Zeroed` is valid and the output state is `None`.

`Previous` owns a cloned immutable state lease until submission; each dispatch
writes a distinct next-state allocation, so concurrent requests may safely
branch from one prior state. Dropping the last `GpuStateOutput` clone makes its
bytes reclaimable after the last GPU reader completes. Live prior/next states
and in-flight state ranges together may not exceed `extension_state_bytes`;
pressure rejects/waits through the extension permit policy and never allocates
an unbounded state history. For a descriptor with zero candidate capacity,
`try_reserve_effect_batch(0, 0)` returns a zero-capacity permit without
consuming command records; any nonzero candidate output is invalid.

### Extension ABI v1

All ABI words are 32-bit little-endian values, every offset is from extension
job-allocation byte zero and divisible by four, and all reserved words must be
zero on input and output. Rust mirrors are
`#[repr(C)] + Pod + Zeroable`; layout tests assert each offset and total size
against WGSL constants. An extension declares only Moria bind group 0:
read/write control header, read-only packet records/input state, read/write
next-state, write-only candidate records, and write-only effect payload. These
are nonoverlapping bounded ranges in an extension-owned job allocation; no
range aliases Moria storage. Any other group/binding is rejected by Naga
validation.

The 128-byte packet header is:

| Byte | Field |
| ---: | --- |
| 0 | magic `0x4d4f5249` |
| 4 | ABI version `1` |
| 8 | inspection kind: samples `1`, occupancy `2`, lifecycle `3`, observation delta `4` |
| 12 | flags; v1 input is zero |
| 16, 20 | snapshot count, inspection record count |
| 24, 28 | state byte count, candidate capacity |
| 32, 36 | output candidate count (initially zero), diagnostic word count (`8`) |
| 40, 44 | effect-payload capacity, output payload bytes (initially zero) |
| 48 | total packet bytes |
| 52..63 | reserved zero |
| 64, 68 | snapshot-record offset, inspection-record offset |
| 72, 76 | input-state offset, output-state offset |
| 80, 84 | candidate-record offset, effect-payload offset |
| 88, 92 | device generation low/high words |
| 96, 100 | operation ID low/high words |
| 104..127 | reserved zero |

Snapshot records are 64 bytes: runtime volume ID at 0, revision at 8,
translation `[f32; 4]` at 16, quaternion `[f32; 4]` at 32, and stable
`VolumeKey` bytes at 48. Sample and occupancy records are 32 bytes: snapshot
index at 0, signed local cell XYZ at 4/8/12, packed
`material:u16|coverage:u8|flags:u8` at 16, occupancy `0|1` at 20, and reserved
zero through 31. Lifecycle records are 32 bytes: snapshot index at 0, signed
brick XYZ at 4/8/12, lifecycle tag at 16, retryability tag at 20, and reserved
failure-kind tag at 24 (`0` for nonfailed, otherwise the closed
`RegionFailureKind` tags: content `1`, invalid content `2`, budget `3`, device
lost `4`, revision unavailable `5`, retired `6`, internal invariant `7`) with
the closed `ContentErrorKind` or `ResourceKind` tag at 28 where applicable,
zero otherwise. Observation-delta records are 64 bytes and contain sequence at 0,
closed observation-kind tag at 8, runtime volume ID at 16, revision at 24,
correlation bytes at 32, and kind-specific scalar data at 48; observations
whose complete fact does not fit this fixed record are rejected for this GPU
inspection variant and remain available through the CPU observation API.

Candidate records are fixed 128-byte slots:

| Byte | Field |
| ---: | --- |
| 0 | kind: unused `0`, fill `1`, patch-runs `2`, move `3` |
| 4 | flags; v1 zero |
| 8 | runtime volume ID `u64` |
| 16 | mandatory exact expected `VolumeRevision` `u64`; zero is invalid |
| 24 | 16-byte `Correlation` |
| 40, 44, 48 | target min XYZ `i32` |
| 52, 56, 60 | target max XYZ `i32` |
| 64 | packed material sample |
| 68, 72 | effect-payload offset and byte length |
| 76 | reserved zero |
| 80 | placement translation `[f32; 4]` |
| 96 | placement quaternion `[f32; 4]` |
| 112..127 | reserved zero |

Fill uses target/sample and has zero payload. Patch-runs uses target plus a
payload slice of 20-byte records
`{ start_index:u32, length:u32, sample:u32, reserved:[u32;2] }`; runs use the
same X-fastest rules as `MaterialPatch`, and both reserved words are zero.
Move uses placement and requires target/sample/payload fields to be zero.
Every kind carries the mandatory exact revision precondition from the captured
snapshot. Create, retire, dense patches, and any unknown kind are not extension
ABI effects; consumers submit them through the ordinary CPU facade.

The shader may write only next-state bytes, eight consumer-defined diagnostic
counter words/flags,
candidate count/records, and effect payload. It cannot change the snapshot,
inspection records, capacities, IDs, or offsets. Diagnostics are copied as the
fixed `ExtensionDiagnostics`; next state remains GPU-owned.

The request's batch permit must reserve at least the descriptor's declared
`max_candidate_effects`, not merely an expected count, and enough aggregate
encoded command payload bytes for the worst-case records permitted by the
descriptor. This reservation happens before packet capture or shader dispatch.
The extension queue permit independently bounds packet/state/diagnostic work.
Registration rejects a descriptor whose candidate count exceeds
`extension_candidate_effects`, whose aggregate effect bytes exceed
`command_payload_bytes`, or whose worst record exceeds the fixed matter-command
limits. It also rejects WGSL/entry-point sizes above their per-descriptor
limits, zero/excess invocation or observation counts, state above
`extension_state_bytes`, or a descriptor that cannot fit one extraction batch.

Moria checks output count, offsets/alignment, reserved words, coordinates,
material IDs/flags, mandatory revision preconditions, record lengths, and
aggregate bytes on GPU. It then copies the 64-byte outcome/diagnostic block and
exactly the produced fixed candidate records/effect payload through the bounded
staging pool; host validation decodes that same bounded transport
representation into ordinary owned commands. Readback is therefore at most
`64 + 128 * produced_count + produced_payload_bytes`, never the inspection
packet or material samples. Any invalid record, overflow, duplicate effect
slot, or mismatch with the batch reservation fails the extension receipt and
admits zero child commands. No command ID is assigned before whole-array
validation succeeds.

After successful validation, Moria converts every candidate into an ordinary
`MatterCommand` or `VolumeCommand`, consumes the matching reserved record/byte
slice, assigns a normal command ID, and returns every child receipt in
`GpuExtensionDispatched.effects` in shader output order. Unused record, byte,
and completion capacity is released immediately after the produced count and
encoded sizes are validated. The outer extension receipt completes at this
all-children-admitted milestone; it does not wait for child completion. Each
child can then apply, conflict, be cancelled before preparation, or fail
independently under the normal per-volume queue. Thus validation/admission is
all-or-none while terminal effects are deliberately independent. Cross-volume
atomicity is not implied.

The packet/effect buffers are not Moria storage and contain only the explicitly
requested bounded snapshot. No extension receives page-table, brick-pool,
scar-pool, presentation, or renderer buffer handles. CPU-oriented behaviors
use the ordinary query/observation/command APIs.

## Telemetry

```rust
pub struct TelemetrySnapshot {
    pub world: WorldId,
    pub state: WorldState,
    pub device_generation: Option<DeviceGeneration>,
    pub effective_config: Option<EffectiveConfig>,
    pub resources: Vec<ResourceUsage>,   // exactly one per ResourceKind
    pub lifecycle_regions: [u64; 6],     // Cold..Failed enum order
    pub active_interests: u32,
    pub operation_stages: [u64; 7],      // OperationStage enum order
    pub observation_backlog: u32,
    pub observation_gaps: u64,
    pub presentation_states: [u64; 5],   // Absent..Failed enum order
    pub maximum_truth_view_revision_lag: u64,
    pub dirty_scar_bricks: u32,
    pub checkpoint_frontier_lag: u64,
    pub extension_packet_bytes: u64,
    pub extension_effect_readback_bytes: u64,
}

pub struct ResourceUsage {
    pub resource: ResourceKind,
    pub requested: u64,
    pub effective: u64,
    pub used: u64,
    pub high_water: u64,
    pub waiting_or_deferred: u64,
    pub rejected: u64,
    pub coalesced: u64,
}
```

Additional latency counters use fixed buckets versioned in the evidence schema.
`ResourceUsage` is the normative capacity/gauge accounting shape, and its
vector must contain every `ResourceKind` in enum order even when the value is
zero or its capability is disabled. Pool kinds report aggregate use.
Per-operation/per-record kinds (`MaterialMetadataPerRegistration`,
`BricksPerInterest`, `VersionsPerBrick`, and
`ContentBricksPerRequest`) report the largest current value in `used` and the
largest observed value in `high_water`; they are never added as though they
were aggregate pools.

`WorldHandle::telemetry()` returns an immutable aggregate snapshot containing:

- world/device state and adapter capability context;
- lifecycle region counts and active interest by priority/capability;
- configured versus used detail/scar/page/mesh/staging capacity;
- live versus lifetime volume records; extraction records/bytes; presentation
  artifact, dirty-record, dressing-style, and instance capacity;
- queue records/bytes, high-water marks, rejection, and latency histograms;
- command/query stages and terminal outcomes;
- observation ring use/gaps;
- presentation state and truth-to-view revision lag;
- checkpoint frontier/progress/dirty coverage;
- extension registration/registry bytes, packet/state/effect bytes, stale state
  IDs, and candidate/diagnostic readback bytes;
- resource-pressure decisions.

Coordinates, raw samples, shader buffers, and consumer opaque metadata are not
telemetry. Histograms have fixed buckets defined in the evidence schema.

## Shutdown

```rust
pub enum ShutdownPolicy {
    Drain { require_checkpoint: Option<CheckpointKey> },
    CancelNotPrepared { require_checkpoint: Option<CheckpointKey> },
}

pub struct ShutdownReport {
    pub final_revisions: Vec<(VolumeKey, VolumeRevision)>,
    pub durable_revisions: Vec<(VolumeKey, VolumeRevision)>,
    pub cancelled: Vec<OperationId>,
    pub failed: Vec<(OperationId, OperationErrorKind)>,
    pub clean: bool,
}
```

Shutdown atomically closes permits/admission. `CancelNotPrepared` installs
`CancelledBeforePreparation` for every operation still in
`Queued | WaitingForMatter` using the same transition race as explicit
cancellation; `Preparing` and later operations drain. It then waits for
submitted GPU work or device terminal state, completes required
checkpointing, emits the report, then releases resources. A failed required
checkpoint yields `clean = false`; dirty data is not described as durable.
The application may still terminate, but must make that loss decision outside
Moria.
