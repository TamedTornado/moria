# Public interfaces and state machines

This document specifies the consumer-visible Rust contract. Signatures are
normative shapes; implementation may add lifetimes and private fields but may
not weaken ownership, bounds, or outcomes. The facade is runtime-neutral:
progress is driven by Bevy schedules and observed with pollable receipts, not
by exposing an executor.

## World construction

### TECH-017 — Configuration and genesis

Implements: REQ-008, REQ-021, REQ-027, REQ-028, REQ-044

The Bevy entry point is:

```rust
pub struct MoriaPlugin {
    pub config: MoriaConfig,
}

pub struct MoriaConfig {
    pub canonical: CanonicalContract,
    pub budgets: ResourceBudgets,
    pub rollback: RollbackConfig,
    pub persistence: PersistenceConfig,
    pub presentation: PresentationConfig,
    pub qualification: QualificationPolicy,
}
```

`MoriaPlugin` installs one `MoriaClient` resource and feature plugins. A
consumer constructs exactly one world through `WorldBuilder`; multiple worlds
within one Bevy app are isolated by `WorldId`, queues, roots, and budgets.

```rust
impl MoriaClient {
    pub fn begin_world(&self, id: WorldId) -> Result<WorldBuilder, ConfigError>;
}

impl WorldBuilder {
    pub fn register_material(&mut self, def: MaterialDefinition)
        -> Result<(), ConfigError>;
    pub fn register_base_source(&mut self, source: Arc<dyn BaseContentSource>)
        -> Result<(), ConfigError>;
    pub fn register_volume(&mut self, def: GenesisVolume)
        -> Result<(), ConfigError>;
    pub fn register_participant(&mut self, adapter: ParticipantRegistration)
        -> Result<(), ConfigError>;
    pub fn publish_genesis(self) -> Result<GenesisReceipt, ConfigError>;
}
```

Builder calls only construct private configuration. `publish_genesis` freezes
registries, checks all IDs/domains/limits/content proofs/participant strategies,
verifies a current authority-backend qualification, materializes the configured
genesis-resident set, calculates canonical genesis bytes and root, and then
publishes tick zero. Any error leaves no usable world or partial registry.
There is no default content, material, participant strategy, RNG seed,
qualification, or empty-world substitution.

`GenesisReceipt` reaches `Ready { tick: 0, root_hash }` or
`Failed(ConfigError | ContentError | BackendError)` and then remains terminal.

### TECH-018 — Materials and volumes

Implements: REQ-003, REQ-008, REQ-013, REQ-019, REQ-020

```rust
pub struct MaterialDefinition {
    pub id: MaterialId,
    pub occupancy: OccupancyClass,
    pub presentation: MaterialPresentation,
}

pub enum SurfaceStyle {
    SmoothDensity,
    CrispCell,
}

pub enum OccupancyClass {
    SolidAbove { density_q8_8: i16 },
    Never,
}

pub struct GenesisVolume {
    pub requested_id: Option<VolumeId>,
    pub kind: VolumeKind,
    pub domain: LocalCellAabb,
    pub pivot: LocalCellPoint,
    pub placement: PlacementQ,
    pub base: BaseAuthorityId,
}
```

Material presentation contains only surface style and asset handles required
by Moria's rendering; arbitrary consumer metadata stays outside Moria.
`SolidAbove` requires a threshold from 1 through `i16::MAX`; `Never` is
inspectable material that does not contribute occupancy. Occupancy class is
canonical material metadata, while asset handles and surface style are
derived-presentation configuration. Neither is physics or damage data.

Static and dynamic volumes use identical query, mutation, checkpoint, and
presentation methods. Only dynamic volumes accept `SetPlacement`. World-space
overlap returns all volume encounters in stable `VolumeId` order. Moria never
merges volumes or selects a response.

## Tick admission and completion

### TECH-019 — Sealed tick batch and permit

Implements: REQ-005, REQ-011, REQ-017, REQ-027, REQ-033

```rust
pub struct TickBatchBuilder { /* bounded owned input */ }

impl MoriaClient {
    pub fn reserve_tick(
        &self,
        world: WorldId,
        tick: Tick,
        limits: TickReservation,
    ) -> Result<TickPermit, ReserveError>;
}

impl TickPermit {
    pub fn push(
        &mut self,
        input: CanonicalInput,
        correlation: Option<CorrelationMetadata>,
    ) -> Result<(), PushError>;
    pub fn seal(self) -> Result<SealedTickBatch, BatchError>;
}

impl MoriaClient {
    pub fn submit_tick(
        &self,
        batch: SealedTickBatch,
    ) -> Result<TickReceipt, SubmitError<SealedTickBatch>>;
}
```

Reservation atomically claims bounded queue bytes and one pending-tick slot;
dropping an unused permit releases them without input loss. `seal` canonical-
encodes, sorts, detects duplicate keys, verifies declared counts, and consumes
the builder. A sealed batch owns immutable canonical bytes, its BLAKE3 digest,
the unforgeable reservation token, and a separately bounded noncanonical
correlation sidecar keyed by the resulting `CanonicalOrder`.

Only the next tick is accepted; classifications are `WrongWorld`,
`BeforeNext`, `AfterNext`, `AlreadyPending`, `WorldNotReady`, `DependencyNotReady`,
`Full`, `Closed`, and `InvalidBatch`. Rejection returns the owned batch.
Accepted work gets one monotonically increasing noncanonical `ReceiptId`.
Dropping a receipt never cancels admitted work.

### TECH-020 — Canonical inputs and command outcomes

Implements: REQ-011, REQ-017, REQ-028, REQ-031, REQ-033

```rust
pub enum CanonicalInput {
    CreateVolume(CreateVolume),
    RetireVolume(RetireVolume),
    ActivateRegion(ActivateRegion),
    DeactivateRegion(DeactivateRegion),
    SetPlacement(SetPlacement),
    Erase(Erase),
    Place(Place),
    Patch(Patch),
    ParticipantInput(ParticipantInput),
}

pub struct InputHeader {
    pub source: InputSourceId,
    pub sequence: u32,
    pub expected_volume_revision: Option<VolumeRevision>,
    pub expected_source_hash: Option<CanonicalHash>,
}

pub struct CorrelationMetadata {
    pub id: CorrelationId,       // consumer-selected 128-bit value
    pub payload: BoundedBytes64, // 0..=64 uninterpreted bytes
}
```

Every variant has a closed `u8` wire tag and explicit maximum encoded size.
Convenience helpers only construct these values; they cannot submit or mutate.
No `Restore`, arbitrary callback, raw shader, raw buffer, or test mutation is a
canonical input. Correlation is explicitly not part of `InputHeader` or
canonical input bytes. At `seal`, it is associated with the input's unique
canonical key and then its sorted `CanonicalOrder`; it cannot affect sorting,
validation, outcomes, hashes, or participant input. Its bytes count against
the tick reservation and observation byte budgets.

```rust
pub enum CommandOutcome {
    Applied {
        tick: Tick,
        order: CanonicalOrder,
        affected: AffectedBounds,
        revision: Option<VolumeRevision>,
        root_hash: CanonicalHash,
    },
    Failed {
        tick: Tick,
        order: CanonicalOrder,
        reason: CanonicalFailure,
        state_unchanged: bool,
    },
    NoOp { tick: Tick, order: CanonicalOrder },
}
```

Failures such as absent IDs, wrong kind, stale revision/hash, invalid cells,
out-of-domain shape, arithmetic overflow, logical capacity, and participant
effect validation have stable wire tags. A command failure may coexist with
other command outcomes in a confirmed tick because it is itself deterministic;
a tick-global fault produces no confirmed outcome list.

Receipt and observation APIs return
`CommandOutcomeView { canonical: CommandOutcome, correlation:
Option<CorrelationMetadata> }`. The canonical outcome encoding contains no
correlation bytes. The sidecar lives until both the terminal receipt cache and
the corresponding observation-ring record expire. An observation gap may
therefore lose correlation and reports that fact; bounded resnapshot does not
reconstruct it. `moria-replay-v1` intentionally omits correlation, so replayed
outcomes carry `None`. Consumers that need correlation after replay retain
their own mapping keyed by `(tick, CanonicalOrder)`.

### TECH-021 — Receipt lifecycle and cancellation

Implements: REQ-005, REQ-011, REQ-015, REQ-017, REQ-021

All receipt types support nonblocking `poll(&self) -> ReceiptState<T, E>` and a
Bevy `MessageReader` notification. They are `Clone + Send + Sync`; polling is
idempotent and terminal results are retained until all receipt handles drop or
the configured terminal-receipt cache expires.

```text
Reserved -> Admitted -> Preparing -> Encoded -> Submitted
    -> GpuComplete -> Decoding -> Confirmed
    \-> FailedNoAdvance
```

`Confirmed` means the live root, revisions, outcome record, participant
commitments, rollback frontier, replay entry, and root hash were coordinated.
`Submitted` does not.

An admitted canonical tick cannot be consumer-cancelled. Before submission,
shutdown may explicitly abandon the whole unconfirmed tick and complete it as
`FailedNoAdvance(Shutdown)`; after GPU submission Moria drains lifetime
tracking and then discards the candidate without publication. Queries,
interest materialization, presentation, and checkpoint reads may be cancelled
before encoding. After submission, cancellation suppresses delivery but does
not return resource permits until GPU/map completion.

## Interest and lifecycle

### TECH-022 — Bounded interest

Implements: REQ-004, REQ-009, REQ-016, REQ-018, REQ-031

```rust
pub struct InterestRequest {
    pub id: InterestId,
    pub world: WorldId,
    pub volumes: VolumeSelector,
    pub bounds: WorldAabbQ,
    pub capabilities: InterestCapabilities,
    pub priority: u8,
    pub max_resident_bricks: u32,
}

pub enum InterestCapability {
    Inspect,
    Collision,
    Presentation,
    PreloadForActivation,
}
```

`upsert_interest` and `withdraw_interest` use a bounded noncanonical control
queue and return `InterestReceipt`. Interest IDs are consumer-owned and replace
prior requests atomically. Moria clips no request silently: the consumer either
uses an explicit `allow_partial` budget and receives the exact covered bounds,
or receives `InterestTooLarge`.

Per `(volume, brick-range, capability)` lifecycle follows:

```text
Cold -> Requested -> Materializing -> Ready -> Retiring -> Cold
                           \-> Failed -> Requested (explicit retry)
```

Withdrawal makes state eligible for `Retiring`; pins, unpersisted scars,
admitted work, snapshots, checkpoints, or observation recovery delay eviction.
Lifecycle is noncanonical and cannot change simulation-domain membership.
`PreloadForActivation` verifies and pins exact content but only
`ActivateRegion` in a tick changes canonical membership.

## Query and collision facade

### TECH-023 — Query contract

Implements: REQ-005, REQ-010, REQ-017, REQ-019, REQ-021

```rust
pub struct QueryRequest {
    pub world: WorldId,
    pub at: QueryFrontier,
    pub freshness: QueryFreshness,
    pub scope: QueryScope,
    pub kind: QueryKind,
    pub completeness: Completeness,
    pub limits: QueryLimits,
}

pub enum QueryFrontier {
    LatestCommitted,
    Retained { tick: Tick, root_hash: CanonicalHash },
}

pub struct QueryFreshness {
    pub minimum: BoundedVec<MinimumVolumeRevision>,
    pub if_unmet: MinimumRevisionPolicy,
}

pub struct MinimumVolumeRevision {
    pub volume: VolumeId,
    pub revision: VolumeRevision,
}

pub enum MinimumRevisionPolicy {
    Wait,
    ReturnStale,
}

pub enum QueryKind {
    Sample { point: WorldPointQ },
    Region { bounds: WorldAabbQ },
    Occupancy { shape: CollisionShapeQ },
    Trace { segment: SegmentQ, max_hits: u32 },
    Overlap { shape: CollisionShapeQ, max_hits: u32 },
    Sweep { shape: CollisionShapeQ, delta: WorldVectorQ, max_hits: u32 },
}
```

Query limits cap inspected bricks, returned cells/hits, encoded result bytes,
and workgroups. The default per request is 4,096 bricks, 65,536 records, and
4 MiB result bytes; configuration may lower but not exceed the compiled
portable maxima. A request larger than its limit is rejected before work.
`minimum` is limited by `QueryLimits.max_volume_revisions`, must contain unique
`VolumeId`s in increasing order after admission normalization, and every ID
must be selected by the query scope.

`Completeness::Complete` returns `Pending(ReadinessReason)` or
`Unavailable(Failure)` rather than partial data. `ExplicitPartial` returns
exact inspected bounds plus missing subranges and never describes them as
empty. Every ready result carries tick, world root hash, sorted per-volume
revisions, exact inspected bounds, completeness, and source commitment.

Ordinary results are noncanonical observations. A participant may affect a
later tick only by encoding the result or its commitment into a later
`ParticipantInput`. Tick-synchronous collision uses the participant artifact
contract rather than callback arrival.

### TECH-024 — Query result ordering and freshness

Implements: REQ-010, REQ-012, REQ-017, REQ-019

Matter samples are ordered by `(volume_id, local z, local y, local x)`.
Collision encounters are ordered by `(time_of_impact, volume_id, local z,
local y, local x, face_id)`. Equal fixed-point times retain the remaining key
order. Output capacity is precomputed; overflow returns
`ResultCapacityExceeded { required, supported }` and no silently truncated
complete result.

A query pins its root at admission. It may complete after later ticks and
still truthfully reports its older frontier. A `freshness.minimum` entry that
is not met returns pending or stale according to the request; Moria does not relabel.
Results from a reclaimed, device-lost, or hash-mismatched root finish
`Unavailable`, not empty. For `LatestCommitted`, `Wait` leaves the request
pending without pinning an older root until every minimum is met;
`ReturnStale` pins the current root and returns a result explicitly labeled
`stale` with each unmet pair. A retained frontier can never become newer, so
an unmet `Wait` condition returns `Unavailable(FrontierTooOld)` and
`ReturnStale` behaves as above.

## Observations and telemetry

### TECH-025 — Gap-aware observation stream

Implements: REQ-005, REQ-012, REQ-017, REQ-021

```rust
pub struct ObservationCursor {
    pub stream: ObservationStreamId,
    pub next_sequence: u64,
}

pub enum ObservationPoll {
    Items { next: ObservationCursor, items: Vec<Observation> },
    Gap {
        last_trustworthy: ObservationCursor,
        oldest_available: ObservationCursor,
        resnapshot_at: FrontierSummary,
    },
    Closed,
}
```

Each world has a fixed-capacity ring, default 8,192 records and configurable
only at genesis. Records have noncanonical stream sequence plus canonical tick,
within-tick order, root hash, relevant revisions, and contract version.
Coalescing is allowed only for lifecycle/presentation telemetry and retains
the covered sequence range. Canonical outcome observations are never
coalesced.

An outcome observation also carries the optional bounded correlation sidecar
from TECH-020. Correlation expiry follows ring expiry and is never synthesized
after a gap; all canonical fields remain independently usable.

Poll limits bound records and bytes. Overwrite advances `oldest_available` and
produces `Gap`; no API returns the newest cursor while hiding lost history.
Recovery uses an ordinary bounded query/frontier summary and a new cursor.
Delivery order cannot affect ticks.

### TECH-026 — Telemetry without storage exposure

Implements: REQ-004, REQ-018, REQ-022

`TelemetrySnapshot` reports configuration identity; adapter and qualification
identity; world/region/presentation states; logical and physical residency;
queue/pool current, capacity, and high-water marks; oldest operation age;
tick/frontier/replay depth; changed leaves/hash nodes; observation gaps;
checkpoint coverage; participant transfer/readback bytes; failures; and
timings/histograms named in the design.

It reports consumer-meaningful counts and bytes, never buffer handles, device
addresses, physical voxel slots, mutable ECS entities, or an iterator over
internal storage. Telemetry is bounded, sampled after the fact, and explicitly
noncanonical. Canonical hashes and outcomes are copied into it as labels; the
telemetry system does not calculate them.

## Error and world lifecycle

### TECH-027 — Typed failure taxonomy

Implements: REQ-005, REQ-015, REQ-021

Public failures retain actionable layers:

```rust
pub enum MoriaError {
    Config(ConfigError),
    Admission(AdmissionError),
    Content(ContentError),
    Canonical(CanonicalFailure),
    Participant(ParticipantError),
    Capacity(CapacityError),
    Backend(BackendError),
    DeviceLost(DeviceGeneration),
    Mapping(MapError),
    Decode(DecodeError),
    Persistence(PersistenceError),
    Replay(ReplayError),
    Qualification(QualificationError),
    Shutdown(ShutdownError),
}
```

Each variant exposes `scope()`, `retryability()`,
`committed_effect() -> None | Some(FrontierSummary)`, and a stable machine
code. Internal errors and uncaptured GPU validation errors never panic a
consumer process; they fail the affected candidate/world and preserve the last
trustworthy frontier. Decode, corruption, unsupported version, lineage
mismatch, device loss, and unqualified backend remain distinct.

Canonical failures have stable wire tags. Environmental failures do not enter
canonical hashes and cannot turn into a timing-dependent canonical outcome.

### TECH-028 — World lifecycle and shutdown

Implements: REQ-008, REQ-015, REQ-016, REQ-021

```text
Configuring -> VerifyingGenesis -> Ready <-> Replaying
                              \-> Failed
Ready/Replaying -> RecoveringParticipant -> Ready | Failed
Ready/Replaying/RecoveringParticipant/Failed -> ShuttingDown -> Closed
```

`Failed` exposes the last trustworthy frontier but rejects new ticks; bounded
queries against pinned readable roots may continue when their failure scope
allows. Shutdown:

1. closes permits and rejects new ticks, queries, interests, and checkpoints;
2. abandons any unsubmitted unconfirmed tick with explicit receipts;
3. drains submitted GPU work for lifetime safety without publishing it;
4. completes or explicitly fails the configured required checkpoint;
5. completes/gaps observation subscribers and participant leases;
6. releases derived resources, then canonical roots, then the device adapter.

Shutdown never creates a tick. Dirty scars remain reported as nondurable if the
checkpoint fails. `ShutdownReceipt` returns the last confirmed/durable
frontiers and every abandoned receipt ID.

## Deterministic participant API

### TECH-029 — Runtime-neutral participant adapter

Implements: REQ-005, REQ-006, REQ-030, REQ-035

```rust
pub trait CanonicalParticipant: Send + Sync + 'static {
    fn descriptor(&self) -> ParticipantDescriptor;
    fn prepare_genesis(
        &self,
        request: ParticipantGenesisRequest,
        sink: ParticipantStateSink,
    );
    fn prepare_tick(
        &self,
        source: ParticipantStateLease,
        request: ParticipantTickRequest,
        sink: ParticipantCompletionSink,
    );
    fn restore_snapshot(
        &self,
        request: ParticipantSnapshotRestoreRequest,
        snapshot: OwnedBytes,
        sink: ParticipantStateSink,
    );
    fn reconstruct(
        &self,
        request: ParticipantReconstructRequest,
        log: ParticipantReplayLease,
        sink: ParticipantStateSink,
    );
    fn export_snapshot(
        &self,
        source: ParticipantStateLease,
        request: ParticipantSnapshotExportRequest,
        sink: ParticipantSnapshotSink,
    );
}
```

The callback form avoids imposing Tokio or a `Send` future. Calls are
nonblocking; completion sinks accept exactly one result and reject wrong
participant/tick/source token/hash, duplicate completion, excessive
bytes/effects, or a closed device generation. `ParticipantStateSink` and
`ParticipantCompletionSink` accept an opaque immutable
`PreparedParticipantState` token bound as specified by TECH-016; the adapter
can inspect a later lease to its own token, but consumers and Moria cannot.
There is no `commit` callback: installing the immutable token is the
coordinator's infallible `FrontierBundle` pointer swap. Dropping an uninstalled
token is abort. The descriptor fixes contract/input versions, strategy,
maximum input/effect/snapshot/artifact/state-token bytes, canonical RNG
contracts, and failure policy at genesis.

```rust
pub struct ParticipantDescriptor {
    pub id: ParticipantId,
    pub contract: ContractDigest,
    pub input_schema: SchemaDigest,
    pub strategy: ParticipantRollbackStrategy,
    pub rng: BoundedVec<ParticipantRngContract>,
    pub limits: ParticipantLimits,
    pub failure: ParticipantFailurePolicy,
}

pub struct ParticipantRngContract {
    pub stream: RngStreamId,
    pub algorithm_id: [u8; 16],
    pub algorithm_version: u32,
    pub algorithm_contract: ContractDigest,
    pub state_schema: SchemaDigest,
    pub seed: BoundedBytes64,
}

pub enum ParticipantFailurePolicy {
    NoAdvanceExplicitRetry, // canonical descriptor tag 0
    FailWorld,              // canonical descriptor tag 1
}
```

RNG entries are sorted by stream ID and follow TECH-016. An adapter declaring
no entries attests that no randomness can affect its canonical state or
effects; Moria provides no implicit stream.

`ParticipantFailurePolicy` is a closed, genesis-bound choice. It applies only
after registration/admission; malformed descriptors always fail configuration.
There is no skip, stale-token reuse, empty-commitment, CPU-fallback, or
best-effort variant. Unknown wire tags are rejected during descriptor
decoding. Its exact behavior is:

| Failure site | `NoAdvanceExplicitRetry` | `FailWorld` |
| --- | --- | --- |
| Genesis preparation | Fail construction; no genesis is published | Same |
| Ordinary tick preparation/validation | Terminally fail that tick receipt as `NoAdvance(Participant(id, code))`; retain `State[t]` and `Ready`; only a new explicit submission may retry | Same `NoAdvance`, then enter `Failed` with `State[t]` as last trustworthy frontier |
| Retained rollback/correction | Fail the correction, drain private tokens, retain the original live bundle and `Ready` | Same atomic abort, then enter `Failed` |
| Durable restore/reconstruction | Fail construction; no restored bundle is published | Same |
| Device generation loss | Fail the affected attempt and enter `RecoveringParticipant`; reject ticks until an explicit bounded recovery recreates an equal-commitment token from a retained snapshot or durable replay bytes | Enter `Failed` immediately; drain old-generation work without publication |
| Snapshot export/checkpoint | Fail only the checkpoint because the installed token remains trustworthy; explicit checkpoint retry is allowed | Same; persistence failure does not corrupt canonical participant state |
| Shutdown | Close new participant permits, suppress publication, drain submitted uses, and report abandonment | Same |

Each explicit retry is one newly admitted, resource-reserved operation; Moria
performs no timer-driven or unbounded internal retry. Recovery installs a
replacement token only when participant ID, contract, tick, root, commitment,
RNG commitments, and new device generation exactly match the retained
frontier. It changes no canonical bytes and emits a lifecycle observation.
Recovery mismatch follows the same policy row again. A non-retryable adapter
contract violation or proven commitment divergence is reported distinctly;
the policy still selects operation-scoped versus world-terminal handling but
never permits publication.

CPU participants receive immutable canonical input bytes and a bounded collider
artifact already keyed to `State[t]`; they do not receive cell storage.
Participant effects are `Erase`, `Place`, `Patch`, or `SetPlacement` values
with normal preconditions. `prepare_tick` may read only its source lease and
must return a distinct token. Snapshot bytes remain opaque to Moria but their
size, digest, retention, export, durable storage, and staged restoration result
are coordinated. For the reconstructible strategy, `reconstruct` receives
bounded canonical replay-record bytes (from pinned memory for recent
correction or digest-verified checkpoint blobs after restart) and returns a
staged token whose per-tick commitments must match.

The Bevy GPU adapter has equivalent semantics and is specified in
[collision-presentation.md](collision-presentation.md). Registering both CPU
and GPU implementations for one participant ID is rejected; runtime fallback
cannot change which algorithm is authoritative.

### TECH-030 — Simulation-domain canonical union

Implements: REQ-009, REQ-014, REQ-018, REQ-031, REQ-040, REQ-043

An activation input names a live volume, a half-open brick-aligned local region,
base content root, optional per-brick manifest subtree digest, and an activity
class opaque `u32` owned by the consumer. Deactivation names the same canonical
key. Moria stores normalized disjoint intervals per `(volume, activity_class)`.
It computes union by stable endpoint sort (`start` before `end` is versioned)
and emits each covered brick exactly once.

Activation preflight requires all exact content and canonical collision inputs
pinned before batch admission. A missing/mismatched dependency returns
`DependencyNotReady` before admission; corruption detected after admission
causes tick `NoAdvance`. Interest, camera, presentation, I/O completion, and
cache eviction cannot add or remove a region. The normalized union, content
commitments, and activity classes are hashed and retained in rollback.
