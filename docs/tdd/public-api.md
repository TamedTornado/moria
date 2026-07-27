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
    pub fn start_world(&self, spec: WorldSpec) -> Admission<WorldStartTicket>;
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

A volume uses a discrete, right-handed local cell lattice:

```rust
pub struct LocalCell { pub x: i32, pub y: i32, pub z: i32 }
pub struct CellBounds { pub min: LocalCell, pub max_exclusive: LocalCell }
pub struct WorldPoint(pub DVec3);
pub struct LocalPoint(pub Vec3);
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

Large-world precision never casts an absolute `DVec3` directly to `Vec3`.
World-space query inputs are transformed on the CPU into `(LocalCell, Vec3
fraction)` pairs; GPU kernels perform integer cell traversal plus brick-local
floating math. Results reconstruct `DVec3` on the CPU. Placement translations
remain `f64`; rotations are normalized finite `f32` quaternions.

Each active volume owns one monotonic `Revision(u64)`. Creation starts at
revision 1. A committed matter edit or dynamic placement change increments it
exactly once with checked arithmetic. Exhaustion permanently fails further
mutations for that volume. A `CatalogRevision(u64)` advances on volume create
or retire. Results spanning volumes contain:

```rust
pub struct RevisionSet {
    pub catalog: CatalogRevision,
    pub volumes: Vec<(VolumeId, Revision)>, // sorted by VolumeId
}
```

Revisions compare only within the same persistent world and volume. Restore
retains the persistent world ID, catalog revision, volume IDs, and volume
revisions.

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
- one optional required `Arc<dyn CheckpointStore>`;
- presentation registrations and budgets; and
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
cannot be registered. No hardness, health, force, damage, wetness, or other
behavior field is accepted.

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
    Complete(T),
    Failed(MoriaError),
    Cancelled,
}
```

Completed statuses are retained for the configured receipt-retention window or
until acknowledged. Eviction is observable as `ReceiptExpired`, never mapped
to success. A command may be cancelled only before its `Committing` phase.
Cancellation is a request; the returned status says whether cancellation won
the race. Checkpoints and queries follow their own cancellation rules in
`runtime.md`.

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

Stable `ErrorCode` categories include invalid configuration/bounds/domain,
unknown identity, static-placement change, stale revision, not ready,
unavailable source, invalid source content/proof, budget/queue exhaustion,
adapter capability, transaction failure, query unavailable, observation gap,
presentation failure, persistence I/O/corruption/version/base mismatch,
shutdown, cancellation, receipt expiry, and internal invariant violation.

## 5. Interest

```rust
pub struct InterestSpec {
    pub volumes: VolumeSelector,
    pub world_bounds: WorldAabb,
    pub capabilities: CapabilitySet,
    pub priority: Priority, // 0..=255
    pub deadline_hint: Option<Duration>,
}

pub struct InterestLease { /* cloneable lease ID and sender */ }
```

Capabilities are `MATTER`, `COLLISION`, `PRESENTATION`, and
`GPU_OBSERVATION`. Collision implies matter. Presentation does not imply that
queries may treat a region as ready.

Creating or updating an interest is admitted/rejected. The returned lease can
be updated explicitly. Dropping the last clone sends a best-effort withdrawal;
deterministic consumers call `withdraw()` and observe its receipt. Deadlines
are priority hints, never permission to return unknown truth. Actual local
regions and capabilities becoming ready are reported through lifecycle
observations and `interest_status`.

## 6. Queries and collision

All queries are bounded and asynchronous:

```rust
pub struct QueryRequest {
    pub scope: QueryScope,
    pub kind: QueryKind,
    pub consistency: QueryConsistency,
    pub availability: AvailabilityPolicy,
    pub completion: CompletionPolicy,
}
```

`QueryScope` is one volume in local coordinates or world bounds across all
intersecting volumes. `QueryConsistency` is `LatestCommitted`,
`AtLeast(RevisionSet)`, or `Exact(RevisionSet)`. `Exact` fails if a named
revision is no longer current; a cold region may materialize at that revision,
but Moria does not time-travel.

`AvailabilityPolicy` is:

- `RequireReady`: finish unavailable if any required truth is not ready;
- `Materialize`: create an operation-owned temporary interest and remain
  pending within budgets; or
- `UseInterest(InterestId)`: wait on the named declared interest.

`CompletionPolicy::Complete` rejects or fails unless every required volume and
cell is inspected. `Partial { max_facts }` permits an explicitly partial result
and returns covered/uncovered bounds plus the reason. Moria never silently
clips.

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

Convex overlap uses conservative voxel coverage; sweeps use conservative
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
optional exact revision precondition, and one effect:

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
advances the volume revision once without resampling local matter.

There is no multi-volume command or transaction. Consumer coordination across
volumes is external and observes independent completion, as authorized by the
design's lack of cross-volume ordering.

## 8. Observations

```rust
pub struct ObservationFilter {
    pub worlds: BoundedSet<WorldId>,
    pub volumes: VolumeFilter,
    pub world_bounds: Option<WorldAabb>,
    pub kinds: ObservationKinds,
}

pub enum ObservationItem {
    Event(Observation),
    Gap(ObservationGap),
}
```

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

`request_snapshot(filter)` is a bounded query-like operation. It returns
catalog, lifecycle, placement, presentation, and requested matter facts plus a
resume token. The token atomically establishes the next observation sequence;
events after that point queue behind the snapshot. This closes the
snapshot/subscribe race without an unbounded log.

## 9. Telemetry access

`telemetry_snapshot(world, detail)` returns aggregate, bounded metrics only.
Per-region detail requires explicit bounds and a maximum row count. Telemetry
identifies adapter, backend, driver, Moria/Bevy versions, configuration digest,
budgets, queue use, lifecycle counts, authoritative/derived bytes, dirty scar
bytes, timings, revision lag, gaps, retries, and extension transfer pressure.
It never contains raw cell arrays or internal buffer handles.
