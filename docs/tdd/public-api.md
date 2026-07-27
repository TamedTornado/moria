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

pub struct WorldKey(uuid::Uuid);
pub struct VolumeKey(uuid::Uuid);
pub struct MaterialKey(uuid::Uuid);

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
    pub fn new(config: MoriaConfig) -> Self;
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
    pub fn validate(&self) -> Result<ValidatedMoria, ConfigurationErrors>;
}
```

`ValidatedMoria` is consumed by `MoriaPlugin` or a test host exactly once.
Registration does not allocate GPU storage or invoke a source. Duplicate
stable keys, invalid domains, missing material references, invalid
fingerprints, and impossible limits are registration/configuration failures.

```rust
pub struct MaterialDefinition {
    pub key: MaterialKey,
    pub debug_name: String,              // 1..=96 UTF-8 bytes
    pub presentation: SurfaceDescriptor,
    pub opaque_metadata: Vec<u8>,        // <= config.max_material_metadata_bytes
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
    Closed,
    PayloadTooLarge { requested: usize, limit: usize },
}

pub enum TryReserveError {
    Full { available_records: u32, available_bytes: usize },
    Closed,
    PayloadTooLarge { requested: usize, limit: usize },
}

impl WorldHandle {
    pub fn try_reserve_command(
        &self,
        payload_bytes: usize,
    ) -> Result<CommandPermit, TryReserveError>;

    pub fn reserve_command(
        &self,
        payload_bytes: usize,
    ) -> ReserveFuture<CommandPermit>;

    pub fn try_reserve_query(
        &self,
        result_budget_bytes: usize,
    ) -> Result<QueryPermit, TryReserveError>;
}
```

A permit reserves both one queue record and the declared bytes. Dropping an
unused permit releases capacity. Submission consumes the permit and command.
Structural rejection returns the command unchanged where possible.

```rust
pub enum SubmitError<T> {
    Invalid { command: T, violations: Vec<Violation> },
    StaleHandle { command: T },
    WorldNotAccepting { command: T, state: WorldState },
    PermitMismatch { command: T },
}

pub struct Receipt<T> { /* Future<Output = Result<T, OperationError>> */ }

impl<T> Receipt<T> {
    pub fn id(&self) -> OperationId;
    pub fn try_status(&self) -> ReceiptStatus<&T>;
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
```

`declare_interest` validates and returns a lease plus the accepted bounded
scope. Cloning the lease retains interest. `update` atomically replaces the
request after validation. Dropping the last clone withdraws it. Withdrawal
does not cancel commands, invalidate completed results, or discard dirty scars.

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
```

`commit_manifest` is the atomic durability point. After it succeeds, every
referenced chunk must be durable and readable. A store incapable of this
contract is rejected. Moria includes a native filesystem store using sibling
temporary files, file sync, atomic rename, and parent-directory sync where the
platform supports it.

```rust
pub struct CheckpointRequest {
    pub key: CheckpointKey,
    pub scope: CheckpointScope,
}

pub struct CheckpointApplied {
    pub key: CheckpointKey,
    pub durable: Vec<(VolumeKey, VolumeRevision)>,
    pub manifest: ChunkDigest,
}
```

The checkpoint frontier is captured when the request is admitted. Later
commits remain dirty and are excluded. Restore is a startup mode, validates all
registrations and base fingerprints before publishing any volume, and returns
one `RestoreReceipt`.

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
}

pub struct GpuExtensionRequest {
    pub extension: ExtensionId,
    pub query: GpuInspectionQuery,
    pub opaque_state: GpuStateInput,
    pub effect_permit: CommandPermit,
}
```

Registration validates WGSL and the fixed ABI. A request first captures a
committed bounded inspection snapshot into an extension-owned packet:
header/revisions, requested material samples or occupancy records, lifecycle
deltas, and opaque consumer state. The shader may write only its private state,
diagnostics, and candidate `Fill`/bounded-run `Patch`/`Move` effect records.

Moria checks output count, coordinates, material IDs, revision preconditions,
and permit bytes on GPU, copies only compact outcome metadata to the control
plane, and routes valid candidates into ordinary per-volume command
transactions. Each candidate has a normal command ID/receipt and can conflict
or fail. Invalid output fails the extension request and commits no candidate
from that dispatch. The fixed output capacity cannot be partially accepted.

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
