# Public Consumer Contract

The signatures below are normative API shapes. Exact module paths may be
refined during implementation, but ownership, milestones, errors, and bounds
must remain.

## Identities and revisions

```rust
pub struct MoriaHandle { /* Send + Sync */ }
pub struct WorldHandle { /* Send + Sync, opaque generation */ }

pub struct WorldId(u64);
pub struct VolumeId(u64);
pub struct MaterialId(u16);
pub struct InterestId(u64);
pub struct SubscriberId(u64);
pub struct CommandId(u64);
pub struct QueryId(u64);
pub struct ExtensionId(u64);
pub struct OperationId(u64);

pub struct WorldKey(uuid::Uuid);
pub struct VolumeKey(uuid::Uuid);
pub struct MaterialKey(uuid::Uuid);
pub struct ExtensionKey(uuid::Uuid);
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

pub enum SurfaceClass {
    Organic,
    Constructed,
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

### Configuration schema

Every field below is public and constructible. `MoriaConfig::default()` supplies
the stated defaults; zero is invalid except where a capability is disabled.
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
    pub persistence: bool,               // default false; enabled by store/restore
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

pub struct ResourceLimits {
    pub nonempty_materials: u32,
    pub max_material_metadata_bytes: u32,
    pub live_volumes: u32,
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
    pub subscribers: u32,
    pub volumes_per_filter: u32,
    pub staging_maps: u32,
    pub staging_bytes: GpuCapacityLimit, // bytes; v1 hard max fits u32
    pub content_requests: u32,
    pub content_response_bytes: u64,
    pub persistence_requests: u32,
    pub persistence_staged_bytes: u64,
    pub presentation_jobs: u32,
    pub mesh_vertices: u32,
    pub mesh_indices: u32,
    pub extension_jobs: u32,
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
| `max_material_metadata_bytes` | 4 KiB | 1 MiB per registration |
| `live_volumes` | 1,024 | 65,535 |
| `interest_leases` / `bricks_per_interest` | 64 / 4,096 | 4,096 / 65,536 |
| `detailed_bricks: GpuCapacityLimit` | 32,768 / 8,192 | `min(u32::MAX, adapter allocation/2,048)`; segmented by binding limit |
| `page_keys: GpuCapacityLimit` | 131,072 / 32,768 | largest power of two within adapter allocation and `u32`; live load <=70% |
| `page_versions: GpuCapacityLimit` | 262,144 / 65,536 | adapter allocation/entry size and `u32`; `>= page_keys`; covers command reservations |
| `versions_per_brick` | 8 | 64 |
| `dirty_scar_bricks: GpuCapacityLimit` | 32,768 / 8,192 | adapter allocation/2,048 and `u32`; `>= max_command_bricks` |
| `command_records` / `command_payload_bytes` | 1,024 / 64 MiB | 65,536 / 1 GiB; records `>= extension_candidate_effects` when enabled; bytes >= maximum patch |
| `query_records` / `query_result_bytes` | 256 / 32 MiB | 16,384 / 1 GiB; bytes >= largest enabled query result |
| `observation_facts` | 4,096 | 1,048,576 |
| `subscribers` / `volumes_per_filter` | 64 / 256 | 4,096 / `live_volumes` |
| `staging_maps` / `staging_bytes: GpuCapacityLimit` | 8 / 32 MiB desired, 8 MiB minimum | maps 1..=256; bytes <=1 GiB and adapter allocation; covers largest enabled readback chunk |
| `content_requests` / `content_response_bytes` | 64 / 32 MiB | 4,096 / 1 GiB; bytes >= one detailed brick |
| `persistence_requests` / `persistence_staged_bytes` | 8 / 64 MiB | 256 / 1 GiB; staged bytes >= 8 MiB chunk decode bound when enabled |
| `presentation_jobs` | 1,024 | 65,536; zero only when presentation disabled |
| `mesh_vertices` / `mesh_indices` | 2,097,152 / 12,582,912 | `u32` and adapter allocation bound; each covers one maximum artifact when enabled |
| `extension_jobs` | 64 | 4,096; zero only when extensions disabled |
| `extension_packet_bytes` / `extension_state_bytes` | 16 MiB / 1 MiB | fixed v1 maxima 64 MiB / 4 MiB |
| `extension_candidate_effects` | 256 | fixed v1 maximum 256 and `<= command_records` |

The fixed request maxima remain: 32,768 cells and 512 bricks per matter
command, 16 MiB patch payload, 262,144 cells per region read, 4,096 collision
hits, 256 world-scope volumes, 2,048 vertices/12,288 indices per brick artifact,
and 256 candidate effects. They are exported in `contract_limits`; they are
not independently configurable.

`PresentationConfig` defaults to `stale_view_policy = DisplayStale`,
`retry_count = 1` (legal `0..=3`), and
`diagnostic_fallback = false`. It contains no camera or content policy.

Supplying a checkpoint store or restore request requires
`capabilities.persistence = true`; enabling persistence without a store is a
configuration error. Enabling GPU extensions requires the Cargo feature,
nonzero extension limits, and a command queue capable of reserving the
configured worst-case batch. Disabling presentation requires all presentation
pool/job fields to be zero and makes presentation interest an explicit
`CapabilityDisabled` error.

`EffectiveConfig` mirrors every requested field and records each exact
effective value plus `Exact | AdapterClamped { requested, adapter_max }`.
It is returned by startup, available through
`MoriaHandle::effective_config()`, and embedded in telemetry/evidence. Values
not marked adapter-negotiated must equal their request. No clamp can weaken an
enabled operation below its fixed public maximum; such an adapter fails
startup instead.

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

`RigidPlacement` contains translation and a unit quaternion. Construction
normalizes only within a small documented tolerance; zero, non-finite, scale,
and shear inputs are rejected. Static volumes reject placement commands.

Every region method takes explicit bounds. It either accepts the complete
request, rejects it with `SupportedBounds`, or honors an explicit
`PartialPolicy::Allow { max_items }`. Silent clipping is forbidden.

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
        packet_bytes: u64,
    ) -> Result<ExtensionPermit, TryReserveError>;

    pub fn reserve_extension(
        &self,
        packet_bytes: u64,
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

A command/query/checkpoint/extension permit reserves one record and the
declared bytes in that operation's queue. An `EffectBatchPermit` reserves
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
```

Receipts are cloneable observers of one shared terminal state. Dropping every
observer does not cancel work. `request_cancel` returns `Requested` only; the
terminal receipt distinguishes `CancelledBeforeSubmission` from an operation
that was already submitted and therefore continues.

Stages are `Queued`, `WaitingForMatter`, `Preparing`, `Submitted`,
`AwaitingReadback`, `AwaitingPersistence`, and `Recovering`. A stage is
diagnostic, not a latency promise.

`OperationError` is a non-exhaustive top-level category carrying operation ID,
scope, retryability, device generation, whether any revision changed, and one
of:

- `Validation`
- `Conflict { expected, current }`
- `Unavailable`
- `BudgetExhausted`
- `OutputOverflow`
- `Content`
- `GpuValidation`
- `OutOfMemory`
- `DeviceLost`
- `Readback`
- `Decode`
- `Persistence`
- `Cancelled`
- `ShuttingDown`
- `InternalInvariant`

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
    ExceedsConfiguredLimit,
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

`RegistryPageRequest { after: Option<MaterialKey>, max_records, max_bytes }`
is bounded by `nonempty_materials`, the `QueryPermit`, and
`query_result_bytes`.
`MaterialRegistryPage` is a stable-key-sorted owned vector of
`MaterialRegistration { id, key, debug_name, presentation, opaque_metadata }`,
the registry digest, and `next_after`. It never splits a record: if the first
eligible registration exceeds `max_bytes`, it returns
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
    pub scope: InterestScope,            // one volume-local or bounded world AABB
    pub capabilities: InterestCapabilities,
    pub priority: InterestPriority,      // Background | Normal | Urgent
    pub max_bricks: u32,
}

pub struct InterestLease { /* Send + Sync */ }

impl InterestLease {
    pub fn id(&self) -> InterestId;
    pub fn accepted(&self) -> &AcceptedInterest;
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

Interest declarations use the configured `interest_leases` slots and return
`InterestError::Full` synchronously when exhausted; they have no payload queue
and therefore no wait policy. A consumer may retry after receiving a resource-
pressure observation.

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
```

A request contains one volume key, its lineage/fingerprint, sorted unique brick
coordinates, the intersected domain, material registry digest, and maximum
encoded bytes. The callback runs on a Moria worker, never a render or Bevy main
thread.

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
    pub encoding: PatchEncoding, // dense row-major or sorted non-overlapping runs
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
Runs must be sorted, non-overlapping, inside bounds, and collectively within
the configured cell/byte limits.

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
    pub scope: QueryScope,                // one volume or bounded world scope
    pub minimum: MinimumRevision,
    pub readiness: ReadinessPolicy,       // Pending | Materialize
    pub partial: PartialPolicy,           // Deny by default
    pub max_results: u32,
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
        omitted: Vec<QueryScope>,
    },
}

pub enum QueryResult {
    Samples(Vec<SampleFact>),
    Region(RegionSamples),               // row-major or homogeneous runs
    Occupancy(OccupancyFact),
    Trace(Vec<ContactFact>),
    Overlap(Vec<ContactFact>),
    Sweep(Vec<ContactFact>),
    Snapshot(WorldSnapshot),
}
```

All query results include query ID, actual inspected bounds, device generation,
a sorted vector of `(VolumeId, VolumeRevision, RigidPlacement)`, completeness,
and result-specific facts.

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
availability/error. `Allow` returns an explicit coverage mask, omitted scopes,
and `Complete | PartialRequested`; hitting a result cap without prior partial
authorization is `OutputOverflow`, not success.

## Observation

```rust
pub struct Subscription {
    pub volumes: BoundedVolumeFilter,
    pub local_or_world_bounds: Option<QueryAabb>,
    pub kinds: ObservationKinds,
}

pub enum ObservationItem {
    Fact(Observation),
    Gap(ObservationGap),
}

pub enum SubscriptionStart {
    CurrentHead,
    Retained(ObservationSequence),
}

pub struct ObservationSubscriber { /* Send + Sync, bounded cursor */ }

impl ObservationSubscriber {
    pub fn id(&self) -> SubscriberId;
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
```

The Bevy adapter owns render entities and mesh assets and tags them with opaque
volume/brick/revision components for diagnostics. These components do not grant
query access. Consumers may supply Bevy material handles through presentation
registration, but Moria validates that every runtime material has a rendering
input before presentation interest becomes ready.

Derived dressing uses a bounded `DressingDescriptor`: stable style key,
material filter, density, scale/orientation ranges, and consumer mesh/material
handles. It has no occupancy. Matter-backed objects use ordinary volume
creation instead.

## Persistence API

```rust
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
    pub wgsl: String,
    pub entry_point: String,
    pub max_invocations: u32,
    pub max_observations: u32,
    pub max_candidate_effects: u32,
    pub max_effect_payload_bytes: u32,   // aggregate encoded child commands
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
    pub effects: Vec<AdmittedEffect>,
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

Registration validates WGSL and the fixed ABI. A request first captures a
committed bounded inspection snapshot into an extension-owned packet:
header/revisions, requested material samples or occupancy records, lifecycle
deltas, and opaque consumer state. The shader may write only its private state,
diagnostics, and candidate `Fill`/bounded-run `Patch`/`Move` effect records.

The request's batch permit must reserve at least the descriptor's declared
`max_candidate_effects`, not merely an expected count, and enough aggregate
encoded command payload bytes for the worst-case records permitted by the
descriptor. This reservation happens before packet capture or shader dispatch.
The extension queue permit independently bounds packet/state/diagnostic work.
Registration rejects a descriptor whose candidate count exceeds
`extension_candidate_effects`, whose aggregate effect bytes exceed
`command_payload_bytes`, or whose worst record exceeds the fixed matter-command
limits.

Moria checks output count, coordinates, material IDs, revision preconditions,
record lengths, and aggregate bytes on GPU, copies only compact outcome
metadata to the control plane, then validates the entire candidate array on the
host. Any invalid record, overflow, duplicate effect slot, or mismatch with the
batch reservation fails the extension receipt and admits zero child commands.
No command ID is assigned before whole-array validation succeeds.

After successful validation, Moria converts every candidate into an ordinary
`MatterCommand` or `VolumeCommand`, consumes the matching reserved record/byte
slice, assigns a normal command ID, and returns every child receipt in
`GpuExtensionDispatched.effects` in shader output order. Unused record, byte,
and completion capacity is released immediately after the produced count and
encoded sizes are validated. The outer extension receipt completes at this
all-children-admitted milestone; it does not wait for child completion. Each
child can then apply, conflict, be cancelled before submission, or fail
independently under the normal per-volume queue. Thus validation/admission is
all-or-none while terminal effects are deliberately independent. Cross-volume
atomicity is not implied.

The packet/effect buffers are not Moria storage and contain only the explicitly
requested bounded snapshot. No extension receives page-table, brick-pool,
scar-pool, presentation, or renderer buffer handles. CPU-oriented behaviors
use the ordinary query/observation/command APIs.

## Telemetry

`WorldHandle::telemetry()` returns an immutable aggregate snapshot containing:

- world/device state and adapter capability context;
- lifecycle region counts and active interest by priority/capability;
- configured versus used detail/scar/page/mesh/staging capacity;
- queue records/bytes, high-water marks, rejection, and latency histograms;
- command/query stages and terminal outcomes;
- observation ring use/gaps;
- presentation state and truth-to-view revision lag;
- checkpoint frontier/progress/dirty coverage;
- extension packet/effect bytes and readback bytes;
- resource-pressure decisions.

Coordinates, raw samples, shader buffers, and consumer opaque metadata are not
telemetry. Histograms have fixed buckets defined in the evidence schema.

## Shutdown

```rust
pub enum ShutdownPolicy {
    Drain { require_checkpoint: Option<CheckpointKey> },
    CancelUnsubmitted { require_checkpoint: Option<CheckpointKey> },
}

pub struct ShutdownReport {
    pub final_revisions: Vec<(VolumeKey, VolumeRevision)>,
    pub durable_revisions: Vec<(VolumeKey, VolumeRevision)>,
    pub cancelled: Vec<OperationId>,
    pub failed: Vec<(OperationId, OperationErrorKind)>,
    pub clean: bool,
}
```

Shutdown atomically closes permits/admission, applies the queued-work policy,
waits for submitted GPU work or device terminal state, completes required
checkpointing, emits the report, then releases resources. A failed required
checkpoint yields `clean = false`; dirty data is not described as durable.
The application may still terminate, but must make that loss decision outside
Moria.
