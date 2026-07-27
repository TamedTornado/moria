# Public consumer contract

## 1. Facade shape

Consumers install `MoriaPlugin`, insert one `MoriaConfig`, and access a
`MoriaWorlds` Bevy resource. The resource is a cloneable command/query handle;
it does not expose ECS entities, render-world buffers, internal schedules, or
storage. A non-Bevy host may use the same contract types through a future
adapter, but only the Bevy adapter is current scope.

Startup has two phases:

```rust
let registration = MoriaWorldRegistration {
    materials,
    volumes,
    limits,
    presentation_policy,
    persistence_policy,
};
let startup: StartupTicket = worlds.start(registration, content_sources, sink)?;
```

`start` performs synchronous structural validation. It returns
`Err(ConfigurationError)` without creating a world if IDs, bounds, limits,
lineage descriptors, required sources, material definitions, or GPU
capabilities are invalid. A valid registration returns a ticket whose terminal
outcome is `Ready(WorldHandle)` or `Failed(MoriaError)`. No command or query is
accepted before `Ready`.

## 2. Identity, coordinates, and revisions

All IDs are nonzero, opaque 128-bit values serialized in network byte order:

- `WorldId`, `VolumeId`, `MaterialId`, `InterestId`, `SubscriptionId`
- `ReceiptId`, `QueryId`, `CheckpointId`, and consumer `CorrelationId`

The consumer supplies stable world, volume, and material IDs. Moria allocates
ephemeral request IDs. IDs are never recycled within a process. Restore rejects
duplicate or mismatched stable IDs.

Public coordinate types prevent accidental space mixing:

- `LocalCell { x: i32, y: i32, z: i32 }`
- `LocalCellAabb { min, max_exclusive }`
- `LocalPoint(DVec3)` in cell units
- `WorldPoint(DVec3)` and `WorldVector(DVec3)` in consumer world units
- `Placement { translation: DVec3, rotation: DQuat }`

`Placement` must contain finite values and a normalized quaternion within
`1e-9`; it is an isometry with no scale or shear. Each volume has one positive,
finite `cell_size: f64`. There is no distinguished axis.

Revision types are nonzero `u64` values:

- `VolumeRevision`: local matter and placement history for one volume.
- `TopologyRevision`: creation/retirement history for the world's volume set.
- `ObservationSequence`: delivery order within one subscription.

Registration starts every volume at revision 1 and topology at 1. A successful
matter mutation or placement command increments that volume revision exactly
once. Create/retire increments topology exactly once; creation starts the new
volume at revision 1. Overflow is a permanent `RevisionExhausted` failure and
the affected world stops new admissions. A multi-volume query returns a sorted
`RevisionVector` of `(VolumeId, VolumeRevision)` plus `TopologyRevision`; no
scalar global snapshot is implied.

## 3. Material and volume registration

`MaterialDefinition` contains only substrate-owned fields:

```rust
pub struct MaterialDefinition {
    pub id: MaterialId,
    pub occupancy_threshold: u8,
    pub surface_mode: SurfaceMode,       // Organic | Crisp
    pub presentation: PresentationKey,  // opaque consumer asset key
}
```

Material ID zero is reserved for empty. Empty always has coverage zero and is
never registered. Nonempty matter has coverage `1..=255`; a point/cell counts
as occupied when coverage is at least its material's threshold. Material
registration is immutable for a running world. Consumer behavior metadata is
keyed by `MaterialId` and stored outside Moria. A running world supports at
most 65,535 registered nonempty materials because its GPU table slot is 16
bits; a lower configured `max_materials` is enforced at registration.
`occupancy_threshold` must be in `1..=255`.

`VolumeDefinition` contains `id`, `kind: Static | Dynamic`, finite
`LocalCellAabb`, `cell_size`, initial `Placement`, content source ID, verified
base lineage descriptor, and presentation policy. Static placement cannot be
changed. Dynamic placement is changed only by command.

Every axis length must be positive, a multiple of the 8-cell brick edge, and
no greater than `2^31 - 8`. The brick-count product must fit `u64`. Bounds are
finite even if mostly uniform; multiple volumes compose larger spaces.

## 4. Interest

An `InterestRequest` names:

- one world;
- one or more `(VolumeId, LocalCellAabb)` entries;
- capability bits `INSPECT`, `COLLIDE`, `PRESENT`, and `GPU_OBSERVE`;
- `Priority` in `0..=255` (255 is highest);
- an optional deadline used for scheduling only; and
- a caller-owned `InterestId`.

Bounds are brick-aligned by Moria for materialization and reported back as
`effective_bounds`. They are never clipped to a volume unless
`allow_partial=true`; otherwise out-of-domain interest is rejected.

`declare_interest` returns an `InterestTicket`. Acknowledgment means the
request is tracked, not ready. `InterestStatus` reports each region's lifecycle,
available capabilities, revision, pressure reason, and retry hint. Updating an
existing ID atomically replaces its desired bounds/capabilities. Withdrawal
makes regions retirement candidates but preserves issued immutable results and
dirty scars.

Interest is not required for a command. Admission may create an internal,
lower-level pin for its bounded target if resources can be reserved. Queries
may either require pre-existing readiness or request materialization according
to `AvailabilityPolicy`.

## 5. Commands and receipts

The public command enum is closed for the initial contract:

```rust
pub enum Command {
    Remove(RemoveMatter),
    Place(PlaceMatter),
    Patch(ApplyPatch),
    CreateVolume(CreateVolume),
    RetireVolume(RetireVolume),
    SetPlacement(SetDynamicPlacement),
}
```

Matter commands target exactly one volume and one local AABB:

- `Remove` writes empty cells selected by a bounded stamp.
- `Place` writes one registered material and coverage through a bounded stamp.
- `Patch` supplies a brick-major run-length stream of final
  `(material, coverage)` values for cells inside the target.

Built-in stamps are `Aabb`, `Sphere`, and `Capsule` in local coordinates.
They are effects, not game verbs. Patch decoding has a declared cell count and
byte length and must cover each addressed cell at most once. Unmentioned cells
remain unchanged. Empty patches and writes that would change no cell are
structurally valid; they apply and advance no revision, returning
`Applied { changed: false, revision: current }`.

`CreateVolume` contains a `VolumeDefinition` whose source ID and lineage were
pre-registered at startup, plus an optional bounded initial patch.
`RetireVolume` names one existing volume and preserves its persistent scar
record until a checkpoint covers the retirement. New content-source code
cannot be smuggled through a command; a consumer that needs another source
starts another world registration.

Every command carries:

- a `CorrelationId` returned unchanged in outcomes/observations;
- optional `RevisionPrecondition`;
- `CommandPriority`;
- a declared target and encoded byte size; and
- an optional cancellation token.

Preconditions are `VolumeIs(VolumeRevision)` and, for create/retire,
`TopologyIs(TopologyRevision)`. Matter commands cannot span volumes. A caller
requiring independent success submits commands separately; cross-volume
transactions are not supplied.

Submission has a strict boundary:

- `Rejected(CommandRejection)`: synchronous validation or reservation failed;
  no work or effect exists.
- `Admitted(CommandReceipt)`: all worst-case transaction, scar-capture,
  receipt, and output capacity is reserved.

An admitted receipt progresses monotonically:

```text
Queued -> Preparing -> Executing -> Publishing -> Applied
                   \-> Failed
```

Cancellation before `Executing` produces `Failed(Cancelled)` with no change.
Cancellation after execution begins is a request only; atomic work finishes
as applied or failed. Matter mutation is not public until its COW root and
revision are published together. Any pre-publication error returns `Failed`
with `committed_revision_changed=false`. Create, retire, and placement failures
also report explicitly whether a revision changed. A receipt is retained until
acknowledged or the configured retention limit forces a reported receipt gap.

## 6. Queries

Query submission returns `Rejected(QueryRejection)` or `QueryTicket`.
Admission reserves the maximum result bytes and captures an availability
policy:

- `ReadyOnly`: return `Unavailable` if required matter is not ready.
- `Materialize`: acquire temporary interest and remain pending within budget.
- `AllowPartial`: inspect ready subregions and list every omitted subregion
  with its reason.

The query enum is:

- `Sample { world_point | (volume, local_point) }`
- `Region { scope, local/world bounds, fields }`
- `Occupancy { scope, CollisionShape }`
- `Trace { world_ray, max_distance, max_hits }`
- `Overlap { CollisionShape, max_contacts }`
- `Sweep { CollisionShape, translation, max_contacts }`
- `Snapshot { subscription filter bounds }`

Results always include requested/effective bounds, topology revision, sorted
volume revision vector, completeness, and byte/contact truncation status.
`Complete` means every intersecting registered volume at the captured topology
revision was ready and inspected. `Partial` exists only for `AllowPartial`.
`Unavailable` names cold, pending, failed, retired, or budget-blocked scopes.
No-hit and empty are result facts only after complete authoritative inspection.

The CPU result is a bounded immutable value copied from GPU output. Consumers
that run in the render world may instead request the read-only GPU result lease
specified in `matter-and-gpu.md`; its semantic result header is still copied
to the receipt channel.

## 7. Observations

A subscription contains a bounded filter over world, volumes, local/world
bounds, and event kinds. Supported observations are:

- matter committed;
- volume created, retired, or placement changed;
- region lifecycle changed;
- presentation status changed;
- checkpoint completed or failed;
- resource pressure decision; and
- receipt terminal outcome.

Every record contains subscription sequence, topology/revision context,
affected bounds, correlation ID when present, and stable error/reason codes.
Events describe committed facts only. They are not commands and carry no
behavior interpretation.

Polling returns `Events`, `CaughtUp`, or
`Gap { last_trustworthy_sequence, last_trustworthy_revisions }`. On a gap the
subscription remains paused. The consumer issues a bounded `Snapshot` query,
then calls `resume(subscription, snapshot_token)`; using an unrelated or stale
token is rejected. Silent dropping and auto-resume are forbidden.

## 8. Errors and diagnostics

All public errors have:

```rust
pub struct MoriaError {
    pub code: ErrorCode,
    pub scope: ErrorScope,
    pub retry: RetryDisposition,
    pub committed_revision_changed: bool,
    pub message: String,
}
```

Stable top-level codes include `InvalidConfiguration`, `InvalidBounds`,
`MissingWorld`, `MissingVolume`, `MissingMaterial`, `WrongVolumeKind`,
`StaleRevision`, `NotReady`, `ContentUnavailable`, `InvalidContent`,
`LineageMismatch`, `ReconstructionUnverified`, `BudgetExceeded`,
`QueueFull`, `OutputTooLarge`, `ObservationGap`, `ReceiptGap`,
`PersistenceIo`, `CorruptCheckpoint`, `UnsupportedFormat`, `DeviceLost`,
`RevisionUnavailable`, `Cancelled`, `RevisionExhausted`, and
`InternalInvariant`.

`RetryDisposition` is `Never`, `AfterInterest`, `AfterPressureRelief`,
`AfterSourceRecovery`, `AfterDeviceRecovery`, or `ConsumerAction`. Messages
may improve without a semver change; codes and field meanings may not.

Telemetry is snapshot/counter data, never an internal handle. It reports
configured/used/reserved limits, lifecycle counts, queue depths, latency
histograms, revision lag, checkpoint coverage, observation gaps, GPU extension
bytes, CPU readback bytes, adapter/driver identity, and all pressure decisions.
