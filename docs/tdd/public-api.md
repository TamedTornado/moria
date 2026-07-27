# Public API Contract

## 1. Facade shape

`moria::prelude` re-exports the stable consumer surface. All other exported
modules are grouped by concept: `config`, `content`, `command`, `query`,
`observation`, `persistence`, `presentation`, `extension`, and `telemetry`.
Internal modules are private.

The integration entry points are:

```rust
pub struct MoriaPlugin {
    pub config: MoriaConfig,
}

#[derive(Resource, Clone)]
pub struct MoriaClient { /* private bounded-channel handles */ }

impl MoriaClient {
    pub fn start_world(
        &self,
        spec: WorldSpec,
    ) -> Admission<OperationTicket<WorldStarted>>;
    pub fn restore_world(
        &self,
        spec: RestoreSpec,
    ) -> Admission<OperationTicket<WorldStarted>>;
    pub fn world(&self, id: WorldId) -> Option<WorldClient>;
}
```

`MoriaPlugin` installs main-world and render-world systems, creates bounded
channels, and publishes `MoriaClient`. `WorldClient` is a cloneable capability
for exactly one world. It contains an ID and channel sender, not world storage.
Methods may be called from Bevy systems or other threads and never block on GPU
or I/O.

No world exists until `start_world` completes. Startup validates the complete
material table, volume registry, source/store collaborators, and aggregate
budget before creating visible state. Startup failure leaves no addressable
partial world.

The complete callable facade is:

```rust
impl WorldClient {
    pub fn declare_interest(
        &self,
        spec: InterestSpec,
    ) -> Admission<InterestLease>;
    pub fn submit(
        &self,
        command: Command,
    ) -> Admission<OperationTicket<CommandResult>>;
    pub fn query(
        &self,
        request: QueryRequest,
    ) -> Admission<OperationTicket<QueryResult>>;
    pub fn subscribe(
        &self,
        spec: SubscriptionSpec,
    ) -> Admission<ObservationSubscription>;
    pub fn request_snapshot(
        &self,
        request: SnapshotRequest,
    ) -> Admission<OperationTicket<ObservationSnapshot>>;
    pub fn checkpoint(
        &self,
        request: CheckpointRequest,
    ) -> Admission<OperationTicket<CheckpointResult>>;
    pub fn retry_region(
        &self,
        volume: VolumeId,
        region: RegionCoord,
    ) -> Admission<OperationTicket<RegionStatus>>;
    pub fn retry_presentation(
        &self,
        volume: VolumeId,
        region: RegionCoord,
    ) -> Admission<OperationTicket<PresentationStatus>>;
    pub fn shutdown(
        &self,
        policy: ShutdownPolicy,
    ) -> Admission<OperationTicket<ShutdownResult>>;
    pub fn telemetry_snapshot(
        &self,
        request: TelemetryRequest,
    ) -> Result<TelemetrySnapshot, MoriaError>;
    pub fn prepare_discard_undurable(
        &self,
    ) -> Result<DiscardProposal, MoriaError>;
    pub fn confirm_discard_undurable(
        &self,
        token: DiscardToken,
        exact: RevisionSet,
    ) -> Admission<OperationTicket<DiscardResult>>;
}
```

`WorldStarted` contains the `WorldClient`, initial `RevisionSet`, and startup
diagnostics. The handle becomes discoverable through `world` in the same
main-world publication transaction that completes the start ticket.
`world` returns handles for `Ready`, `RecoveringDevice`, `Quiescing`, and
post-publication `Failed` worlds so diagnostics and safe shutdown remain
available; it returns `None` while configuring and after `Stopped`.
`RestoreSpec`, checkpoint DTOs, and discard/shutdown semantics are defined in
`persistence.md` and `runtime.md`. Methods return only public DTOs; the
signatures above are the required surface even if implementation groups them
into public modules.

## 2. Identity, coordinates, and revisions

Persistent world/volume IDs are UUID v7 newtypes:

```rust
WorldId, VolumeId
```

`MaterialId` is a stable world-local `u16` newtype. Value 0 is reserved for
empty; registered materials use `1..=65535`. Interest, subscription, and
operation IDs combine a process nonce and monotonic counter:

```rust
InterestId, SubscriptionId, ReceiptId, QueryId, CheckpointId, GpuExchangeId
```

The serialized form is stable. Runtime-generated IDs are returned to the
consumer before use; caller-supplied persistent IDs are collision-checked.
`OperationId` is a tagged public enum over the applicable receipt, query,
checkpoint, and exchange ID newtypes; it prevents a caller from polling or
correlating the wrong operation class.

A volume uses a discrete, right-handed local cell lattice:

```rust
pub struct LocalCell { pub x: i32, pub y: i32, pub z: i32 }
pub struct CellBounds { pub min: LocalCell, pub max_exclusive: LocalCell }
pub struct WorldPoint(pub DVec3);
pub struct LocalPoint {
    pub cell: LocalCell,
    pub fraction: Vec3, // each component in [0.0, 1.0)
}
pub struct VolumePlacement {
    pub translation: DVec3,
    pub rotation: Quat,
}
```

Bounds are half-open. Empty or inverted bounds are invalid. No API assumes an
up axis. `VolumeSpec::cell_edge_world_units` is a finite positive `f32` and is
immutable. Placement permits translation and rotation only; nonuniform scale
and shear are rejected because they make cell coverage and collision
ambiguous. Static volume placement is immutable. Dynamic placement changes
only through `SetPlacement`.

Local point construction canonicalizes carry/borrow into `cell` and rejects
nonfinite fractions. Large-world precision never casts an absolute `DVec3` or
large absolute local coordinate directly to `Vec3`. World-space query inputs
are transformed on the CPU into canonical `LocalPoint` values; GPU kernels
perform integer cell traversal plus brick-local floating math. Results
reconstruct `DVec3` on the CPU. Placement translations remain `f64`; rotations
are normalized finite `f32` quaternions promoted to `f64` for CPU
world-to-local origin transformation.

Each active volume owns one monotonic `Revision(u64)`. Creation starts at
revision 1. A committed matter edit or dynamic placement change increments it
exactly once with checked arithmetic. Exhaustion permanently fails further
mutations for that volume. A `CatalogRevision(u64)` advances on volume create
or final retirement. Catalog revision 1 names the atomically published initial
registry. Catalog revision exhaustion permanently rejects later create/final
retire commits without changing the catalog. Results spanning volumes contain:

```rust
pub struct RevisionSet {
    pub catalog: CatalogRevision,
    pub volumes: Vec<(VolumeId, Revision)>, // sorted by VolumeId
}
```

Revisions compare only within the same persistent world and volume. Restore
retains the persistent world ID, catalog revision, volume IDs, and volume
revisions.

A `RevisionSet` is a canonical registry snapshot, not merely a bag of
revisions. At its catalog revision it contains every active volume intersected
by the operation scope, including volumes that produced no material fact.
Volume entries are unique and sorted by the 16 serialized UUID bytes. A
world-scope operation captures the catalog and candidate volume descriptors
under one scheduler read barrier. A create, placement, or final-retirement
commit is ordered wholly before or after that capture. This prevents a no-hit
result from silently omitting a concurrently created or moved volume.

## 3. Configuration

`MoriaConfig` sets adapter requirements, queue and result capacities, GPU/CPU
budgets, retry policy, telemetry sampling, and maximum request bounds. Defaults
are the conservative values in `resources-and-portability.md`. All limits are
validated for nonzero values, cross-limit consistency, and GPU adapter support.

`WorldSpec` contains:

- caller-selected `WorldId`;
- a nonempty, unique material table;
- initial static/dynamic `VolumeSpec` values;
- one `Arc<dyn BaseContentSource>` per volume;
- the explicit `PersistencePolicy` defined in `persistence.md`;
- presentation registrations and budgets; and
- optional pre-publication GPU extension registrations; and
- correlation metadata limited to 256 bytes.

Materials and initial volumes are validated as a single unit. Material
definitions are immutable after startup. A consumer that changes material
meaning creates a new world contract and explicitly migrates persistence.

`MaterialDefinition` contains only substrate-owned facts:

```rust
pub struct MaterialDefinition {
    pub id: MaterialId,
    pub occupancy_threshold: u8,
    pub surface: SurfaceDefinition,
    pub presentation_key: PresentationKey,
    pub consumer_metadata: Option<BoundedBytes<1024>>,
}
```

Coverage lower than the threshold is nonoccupied for collision; coverage at or
above it is occupied. `MaterialId::EMPTY` always requires coverage zero and
cannot be registered. `occupancy_threshold` must be in `1..=255`.
`SurfaceDefinition` includes the pipeline parameters and boundary priority
defined in `presentation-and-extensions.md`. No hardness, health, force,
damage, wetness, or other behavior field is accepted.

## 4. Admission and operation status

Every submitted operation returns:

```rust
pub enum Admission<T> {
    Admitted(T),
    Rejected(MoriaError),
}
```

Admission performs all validation possible without materializing cold truth:
identity, structure, configured maxima, checked coordinate math, material
references, command kind, queue capacity, static/dynamic legality, revision
precondition against the current committed revision, and whether required
truth is materializable under the source/budget contract.

Admission reserves queue and worst-case transaction/result capacity. It does
not promise success. The ticket contains an ID and initial `Pending` state.
Polling is non-destructive and idempotent:

```rust
pub enum WorkStatus<T> {
    Pending(Progress),
    Complete(Arc<T>),
    Failed(MoriaError),
    Cancelled,
}

impl<T> OperationTicket<T> {
    pub fn id(&self) -> OperationId;
    pub fn poll(&self) -> WorkStatus<T>;
    pub fn request_cancel(&self) -> CancelRequestOutcome;
    pub fn acknowledge(&self) -> Result<(), MoriaError>;
}
```

Completed statuses are retained for the configured receipt-retention window or
until acknowledged. Eviction is observable as `ReceiptExpired`, never mapped
to success. Retention begins at terminal publication; holding a ticket does
not exempt its result from the configured bound. `poll` returns an `Arc` so a
successfully polled result remains valid after acknowledgment. `acknowledge`
releases Moria's retained copy and later polls report `ReceiptExpired`. A
command may be cancelled only before its `Committing` phase. Cancellation is a
request; `CancelRequestOutcome::{Requested, TooLate, AlreadyTerminal}` says
whether it can still win. Checkpoints and queries follow their own
cancellation rules in `runtime.md`.

`MoriaError` always contains:

```rust
pub struct MoriaError {
    pub code: ErrorCode,
    pub scope: ErrorScope,
    pub retry: RetryAdvice,
    pub committed: CommitEffect, // None or an identified revision change
    pub diagnostic: String,      // bounded to 1024 UTF-8 bytes
}
```

`ErrorScope` is the tagged union `Plugin`, `World(WorldId)`,
`Volume(WorldId, VolumeId)`, `Region(WorldId, VolumeId, RegionCoord)`,
`Operation(WorldId, OperationId)`, `Subscription(WorldId, SubscriptionId)`,
`Presentation(WorldId, VolumeId, RegionCoord)`,
`Checkpoint(WorldId, CheckpointId)`, or
`Extension(WorldId, GpuExchangeId)`. Scope IDs must refer to the most specific
known failing boundary.

The v1 machine-actionable code set is:

```rust
#[non_exhaustive]
pub enum ErrorCode {
    InvalidConfiguration,
    InvalidBounds,
    OutOfDomain,
    ArithmeticOverflow,
    UnknownWorld,
    UnknownVolume,
    UnknownMaterial,
    IdentityCollision,
    StaticPlacement,
    Retiring,
    StaleRevision,
    RevisionExhausted,
    NotReady,
    SourceUnavailable,
    SourceTimeout,
    InvalidSourceContent,
    BaseProofMismatch,
    BudgetExhausted,
    QueueFull,
    ResultCapacityExceeded,
    UnsupportedPlatform,
    AdapterCapability,
    ShaderOrLayout,
    TransactionFailed,
    QueryUnavailable,
    ObservationGap,
    PresentationFailed,
    PersistenceIo,
    CheckpointCorrupt,
    UnsupportedVersion,
    BaseMismatch,
    Shutdown,
    Cancellation,
    ReceiptExpired,
    ExtensionViolation,
    DeviceLost,
    InvariantViolation,
}
```

Serialized error codes use explicitly assigned stable `u16` discriminants
recorded by schema tests; declaration order is not the wire value. Adding a
code is backward-compatible because the Rust enum is non-exhaustive, while
renumbering or changing meaning is forbidden.

`CommitEffect` is exactly `None`, `Volume { volume, prior, current }`, or
`Catalog { prior, current, volumes }`. A failed matter mutation always carries
`None`. Other operations report a non-`None` effect only if their commit was
already published before the later failure.

## 5. Interest

```rust
pub struct InterestSpec {
    pub scope: InterestScope,
    pub capabilities: CapabilitySet,
    pub priority: Priority, // 0..=255
    pub deadline_hint: Option<Duration>,
    pub max_regions: u32,
}

pub struct InterestLease { /* cloneable lease ID and sender */ }
```

Capabilities are `MATTER`, `COLLISION`, `PRESENTATION`, and
`GPU_OBSERVATION`. Collision implies matter. Presentation does not imply that
queries may treat a region as ready.

Creating or updating an interest is admitted/rejected. The returned lease can
be updated explicitly:

```rust
pub enum InterestScope {
    Local { volume: VolumeId, bounds: CellBounds },
    World {
        volumes: VolumeSelector,
        bounds: WorldAabb,
    },
}

impl InterestLease {
    pub fn id(&self) -> InterestId;
    pub fn status(&self) -> InterestStatus;
    pub fn update(
        &self,
        spec: InterestSpec,
    ) -> Admission<OperationTicket<InterestUpdate>>;
    pub fn withdraw(
        &self,
    ) -> Admission<OperationTicket<InterestWithdrawal>>;
}
```

`VolumeSelector` is either one explicit bounded set of at most 256 volume IDs
or `AllCurrent`. A world scope is re-evaluated after catalog and placement
commits. If its intersecting region set would exceed `max_regions`, the lease
enters `OverCapacity`, emits an observation, admits no newly intersecting
regions, and cannot report ready until an explicit update fits. Existing pins
remain until that update or withdrawal, so capacity change never silently
substitutes a clipped ready set. Local scope does not move with placement.

Dropping the last clone sets a shared atomic withdrawal flag that the runtime
reconciles even when the intake queue is full; it cannot leave a permanent pin,
but it provides no completion fact. Deterministic consumers call `withdraw()`
and observe its ticket. Deadlines are priority hints, never permission to
return unknown truth. `InterestStatus` reports the exact resolved local
regions, readiness by capability, and any over-capacity/failure reason. The
directly selected regions plus implementation-required halos must both fit the
admission reservation described in `resources-and-portability.md`.

## 6. Queries and collision

All queries are bounded and asynchronous:

```rust
pub struct QueryRequest {
    pub target: QueryTarget,
    pub consistency: QueryConsistency,
    pub availability: AvailabilityPolicy,
    pub completion: CompletionPolicy,
    pub limits: QueryLimits,
}

pub enum QueryTarget {
    Local { volume: VolumeId, query: LocalQuery },
    World { query: WorldQuery },
}
```

`LocalQuery` carries only `LocalPoint`/`CellBounds`/local-space shape and ray
types. `WorldQuery` carries only `WorldPoint`/`WorldAabb`/world-space shape and
ray types. Both enums expose the same `Sample`, `Region`, `Occupancy`, `Trace`,
and `Sweep` intents described below; mixing coordinate spaces is impossible at
the type level. Local queries inspect exactly one named volume. World queries
first resolve every active volume whose transformed domain intersects the
finite requested geometry.

`QueryLimits` sets nonzero maxima for visited bricks/cells, returned facts,
hits, coverage spans, and result bytes, each no higher than the configured hard
limits. Admission reserves the result/readback side. GPU counters stop before
any visit/result bound would overflow. If a checked conservative bound on the
requested geometry already exceeds a limit, admission rejects `Complete` and
admits only an explicitly partial request. Data-dependent broad-phase work may
still reach the bound after admission; `Complete` then fails
`ResultCapacityExceeded`; an explicitly partial request returns exact coverage
and the exceeded limit as its reason.

`QueryConsistency` is `LatestCommitted`,
`AtLeast(RevisionSet)`, or `Exact(RevisionSet)`. `Exact` fails if a named
revision is no longer current; a cold region may materialize at that revision,
but Moria does not time-travel.

For a local target, `Exact`/`AtLeast` must name that volume and may omit catalog;
for a world target they must name a catalog lower bound and every explicit
volume precondition. `AtLeast` waits until the catalog and named active volume
revisions meet their lower bounds, then dispatches against one newly captured
current `RevisionSet`. It fails `UnknownIdentity` if a named volume retires and
fails `RevisionExhausted` if a bound cannot be reached. `Exact` dispatches only
when catalog and named revisions equal current state at the scheduler barrier;
for a world query its set must equal all volumes intersecting that query at the
named catalog revision. Missing, extra, or changed entries fail
`StaleRevision`. Extra unrelated entries are rejected for local queries.
`LatestCommitted` captures once when the query becomes dispatchable.

`AvailabilityPolicy` is:

- `RequireReady`: finish unavailable if any required truth is not ready;
- `Materialize`: create an operation-owned temporary interest and remain
  pending within budgets; or
- `UseInterest(InterestId)`: wait on the named declared interest.

`CompletionPolicy::Complete` rejects or fails unless every required volume and
cell is inspected. `Partial { max_facts }` permits an explicitly partial result
and returns covered/uncovered bounds plus the reason. Moria never silently
clips.

Partial coverage selection is deterministic in `(VolumeId, region, brick,
cell)` order; trace/sweep facts gathered from that coverage are then sorted by
distance/time as specified below. `QueryCoverage`
contains a canonical, bounded list of covered region/cell spans and an
`UncoveredReason`; it is not one misleading enclosing AABB. Admission reserves
the configured maximum coverage spans. If even that description would
overflow, the query fails `ResultCapacityExceeded` rather than losing coverage
information.

Required query kinds:

- `Sample { point }` returns material, coverage, occupancy, local/world
  position, volume, and revision facts for every overlapping volume.
- `Region { bounds, fields, max_cells }` returns a canonical x-major stream of
  requested material/coverage/occupancy facts.
- `Occupancy { shape }` accepts point, AABB, oriented box, sphere, capsule, or
  convex shape with at most 32 planes.
- `Trace { ray, max_distance, max_hits }` returns ordered entry/exit encounters.
- `Sweep { shape, delta, max_hits }` accepts sphere, capsule, AABB, or oriented
  box and returns time-of-impact intervals.

World-scope results preserve all overlapping volumes. Ordering is
`distance/time`, then `VolumeId`, then local cell lexicographic order. A
no-hit result is only `Complete` after all intersecting authoritative regions
were inspected. Each result includes requested bounds, actual covered bounds,
`RevisionSet`, completeness, and availability/failure details.

`QueryResult` is:

```rust
pub enum QueryResult {
    Complete { facts: QueryFacts, coverage: QueryCoverage, revisions: RevisionSet },
    Partial { facts: QueryFacts, coverage: QueryCoverage, revisions: RevisionSet },
    Unavailable { coverage: QueryCoverage, revisions: RevisionSet, reason: AvailabilityReason },
}
```

Only the first variant may encode a complete no-hit/no-occupied result.
`Unavailable` is a successful inspection outcome with no fabricated facts;
transport, device, validation, or invariant errors use `WorkStatus::Failed`.

Convex overlap uses conservative occupied-cell tests; sweeps use conservative
advancement plus bisection to configured precision. False negatives are
forbidden. Contact points/normals are query facts, not response policy.

## 7. Commands

Commands are value DTOs with bounded correlation metadata:

```rust
pub enum Command {
    Mutate(MatterMutation),
    CreateVolume(CreateVolume),
    RetireVolume(RetireVolume),
    SetPlacement(SetPlacement),
}
```

One `MatterMutation` names exactly one volume, one nonempty target bounds, an
optional exact revision precondition, bounded correlation bytes, and one
effect:

- `Remove { mask }`: set covered target samples to empty/zero;
- `Fill { material, coverage, mask, replace }`;
- `Patch(MaterialPatch)`: a dense or run-length-encoded bounded set of exact
  `(material, coverage)` samples;
- `Stamp(MaterialStamp)`: a consumer-supplied immutable patch plus a local
  integer translation and one of the 24 axis-aligned cube rotations.

`mask` is a bounded cell bitmask or analytic sphere/AABB/capsule evaluated in
local coordinates. `replace` is `Any`, `EmptyOnly`, or a bounded material set.
If a replacement predicate does not match a cell, that cell remains unchanged;
this is still one atomic command. A command that produces no change completes
successfully at the current revision with `changed = false`; it emits no matter
change observation and creates no scar.

Patch decompression size, target bounds, and stamp-transformed bounds are
verified before admission. Every target sample is validated before staging.
A successful changed mutation returns affected bounds, prior/new revision, and
correlation metadata. It becomes visible everywhere at the new revision once.

`CreateVolume` carries a `VolumeSpec`, `Arc<dyn BaseContentSource>`, and
presentation registrations. It validates and initializes a source-backed
volume at revision 1 and advances catalog revision. `RetireVolume` first
transitions to retiring, then commits only after pinned operations and required
persistence are safe. Retirement is itself persisted. `SetPlacement` is legal
only for dynamic volumes, accepts an optional revision precondition, and
advances the volume revision once without resampling local matter. All command
variants carry the same bounded correlation field outside their variant
payload. `CommandResult` identifies the operation, `changed`, prior/current
revision or catalog context, exact affected local/world bounds, and
correlation.

Create and retire admission reserve the next catalog revision before any
visible transition; revision exhaustion therefore rejects retirement before
the volume enters `retiring`.

There is no multi-volume command or transaction. Consumer coordination across
volumes is external and observes independent completion, as authorized by the
design's lack of cross-volume ordering.

## 8. Observations

```rust
pub struct ObservationFilter {
    pub volumes: VolumeFilter,
    pub world_bounds: Option<WorldAabb>,
    pub kinds: ObservationKinds,
}

pub enum ObservationItem {
    Event(Observation),
    Gap(ObservationGap),
}
```

A `SubscriptionSpec` contains this filter, a fixed capacity, and an idle
timeout no larger than configured maxima. Because a `WorldClient` is already
world-scoped, a filter cannot name other worlds. A subscription exposes
`poll(max_items)`, `state()`, `resume_after_snapshot(token)`, and `close()`;
`poll` is nonblocking and `max_items` is bounded by its capacity. Dropping the
last handle sets the same kind of runtime-reconciled close flag as an interest
lease, so a full intake queue cannot leak a subscription.

A subscription has a fixed capacity chosen at creation within configured
limits. Each event contains world-local `ObservationSequence`, affected
identity/bounds, revision context, and correlation metadata. Kinds are:
matter committed, volume created/retiring/retired/placement committed, region
lifecycle, presentation status, checkpoint status, resource pressure, and
world state.

Filters are checked before enqueue. When capacity would overflow, the queue is
replaced by one gap marker containing the last sequence definitely delivered,
the first unavailable sequence, and the latest known catalog/volume revision
summary. No post-gap event is delivered until the consumer calls
`resume_after_snapshot(snapshot_token)`.

`SnapshotRequest` names the gapped `SubscriptionId`, its exact filter,
requested metadata/matter fields, a query availability/completion policy, and
hard `max_regions`/`max_facts`/`max_bytes`. It returns catalog, lifecycle,
placement, presentation, and requested matter facts plus a resume token.
Snapshot capture and sequence-barrier installation are one scheduler
transaction: the token names the first sequence allocated after the captured
state, and matching events at or after that sequence queue behind the
snapshot. `resume_after_snapshot` rejects a token from another subscription,
filter generation, or expired snapshot. This closes the snapshot/subscribe
race without an unbounded log.

## 9. Telemetry access

`telemetry_snapshot(world, detail)` returns aggregate, bounded metrics only.
Per-region detail requires explicit bounds and a maximum row count. Telemetry
identifies adapter, backend, driver, Moria/Bevy versions, configuration digest,
budgets, queue use, lifecycle counts, authoritative/derived bytes, dirty scar
bytes, timings, revision lag, gaps, retries, and extension transfer pressure.
It never contains raw cell arrays or internal buffer handles.
