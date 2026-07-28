# Adapter Substrate Contracts

## Purpose and authority

This file specifies three generic transports required by the scheduled
behavior boundary:

1. atomic extraction of existing authoritative matter into new child volumes;
2. bounded bulk placement publication for GPU-resident persistent objects; and
3. optional bounded opaque GPU-to-CPU adapter egress.

They trace to the approved design's dynamic-volume, controlled-effect,
GPU-oriented behavior, persistence, and explicit-failure requirements.
They do not add physics, damage, weapons, fracture policy, body state,
significance thresholds, activity-region policy, scoring, audio, or gameplay
events to Moria.
An adapter may give the byte and proposal records those meanings; Moria sees
only bounded regions, existing samples, placements, identities, opaque bytes,
and publication outcomes.

Scheduled GPU ABI v2 in this file supersedes the six-binding Scheduled ABI v1
selection in T29, T30, T32, and T33.
Unchanged v1 record layouts retain their `V1` type names inside v2.
There is no runtime v1/v2 negotiation in the initial implementation:
configuration accepts only scheduled ABI version 2.

## Public capability declarations

`BehaviorEngineDescriptor` gains the following fields:

```rust
pub struct BehaviorEngineDescriptor {
    // Existing fields remain.
    pub maximum_placement_updates: u32,
    pub maximum_component_extraction_proposals: u32,
    pub maximum_component_extraction_children: u32,
    pub maximum_component_extraction_assignment_cells: u32,
    pub maximum_component_extraction_child_bricks: u32,
    pub maximum_component_extraction_payload_bytes: u64,
    pub maximum_cpu_egress_bytes: u64,
}
```

All new maxima are zero for a CPU adapter.
For a GPU adapter, component-extraction fields are either all zero or all
nonzero.
`maximum_component_extraction_children` is the aggregate candidate-child
capacity across all of that adapter's component-extraction proposals in one
tick.
The checked products and sums must fit the adapter maxima and the world
resource pools before registration succeeds.

CPU egress is disabled exactly when `maximum_cpu_egress_bytes == 0`.
When enabled, the maximum is a checked four-byte-aligned bound in
`4..=16_777_216` and fits the world egress/handoff transport pools.
Payload bytes and any optional consumer-visible schema metadata are opaque to
Moria; the transport does not interpret them.

The descriptor does not declare activity regions or fidelity classes.
Region definitions remain current-tick consumer input under the existing
opaque input contract.
A multi-fidelity adapter defines and validates its own input schema, body
tables, classification, and coarse/full kernels within its declared input
bytes, factory state, and counted dispatch limits.
Those choices are adapter and validation concerns, not Moria substrate
vocabulary.

## Atomic component extraction

### Operation boundary

`ExtractComponents` is a scheduled GPU substrate effect, not ordinary
`VolumeCommand::Create` and not arbitrary scheduled creation.
It can only redistribute samples that are occupied in one pinned source-volume
revision.
It cannot introduce a material sample, obtain a content source, clone a cell,
address another source volume, or create a child with consumer-authored base
content.

The adapter supplies bounded piece labels and a generic disposition:

```rust
pub struct ComponentPieceHandle(NonZeroU32); // proposal-local

pub enum ComponentPieceDisposition {
    PublishChild,
    RemoveFromMatter,
}

pub struct ComponentExtractionApplied {
    pub command: CommandId,
    pub source: VolumeId,
    pub source_revision: Option<VolumeRevision>,
    pub children: Vec<ComponentChildApplied>,
    pub removed_cells: u32,
    pub removed_digest: [u8; 32],
    pub directory_generation: DirectoryGeneration,
    pub correlation: Correlation,
}

pub struct ComponentChildApplied {
    pub piece: ComponentPieceHandle,
    pub volume: VolumeId,
    pub key: VolumeKey,
    pub revision: VolumeRevision, // always 1 at publication
    pub local_domain: CellAabb,
    pub placement: RigidPlacement,
    pub sample_count: u32,
    pub sample_digest: [u8; 32],
}
```

Cells with no piece assignment remain in the source.
`PublishChild` transfers the assigned samples to a new persistent Moria
volume.
`RemoveFromMatter` is the ordinary substrate effect of explicit removal;
Moria reports its exact cell count and digest but creates no debris, effect, or
gameplay event.
An adapter may interpret that disposition as transient debris or a visual
effect and may carry its own data through a handoff or egress.
Moria does not make that policy choice.

Conservation is over canonical source-space records
`(source_z, source_y, source_x, packed_sample)`, not over material values alone:

```text
source occupied coordinate/sample multiset before
  = source remainder coordinate/sample multiset after
  + every child record mapped back by its origin
  + explicitly removed coordinate/sample multiset
```

Every source sample appears in exactly one term.
The required validation proof uses zero explicitly removed cells, so the
before multiset equals source remainder plus children exactly.
An unassigned sample is never inferred to be removed.
GPU validation uses assignment marks and exact counts for this proof, not a
digest as a substitute. Reported BLAKE3 digests are canonical
coordinate-plus-packed-sample evidence; child digests use child-local
coordinates and removal digests use source coordinates.

Moria validates piece-handle uniqueness per assigned source cell and exact
source membership.
The adapter owns how it discovered components and which ones are significant.
Moria does not certify or define connectivity, but the fracture-shaped proof
adapter must produce six-neighbor connected pieces and the independent oracle
verifies that property.

### Pre-reservation and identity

Before the tick enters `Preparing`, Moria atomically reserves the worst case
declared by every enabled GPU participant:

- one proposal and fixed outcome record for every possible component
  extraction;
- one live directory entry, permanent lifetime-record *slot*, `VolumeId`,
  provisional `VolumeKey`, revision slot, observation fact, presentation
  marker, and receipt child record for every possible child;
- prepared directory-entry slots sufficient for the source update/tombstone
  and every possible child under one directory-generation swap;
- source and child page keys, page versions, detailed brick slots, scar
  records, occupancy summaries, transfer records, assignment records, and
  validation diagnostics;
- the complete binding-6 reservation table and component-extraction payload
  bytes; and
- every byte of submission, feedback, persistence provenance, and cleanup
  bookkeeping needed by the maximum operation.

Reservation is one checked acquisition.
Moria never holds a subset while waiting for another pool.
Failure leaves the tick queued or rejects it under the behavior-tick overload
policy; no adapter executes.

Runtime IDs and stable keys are allocated on the CPU from already reserved
slots before GPU execution, but publication does not depend on a CPU readback.
`VolumeId` uses the ordinary pre-reserved generational live slot.
`VolumeKey` is an ordinary unique stable key allocated into a *provisional*
lifetime-candidate slot that is not yet a permanent lifetime record or
tombstone.
Preflight draws unique keys (process-local random UUIDs), checks them against
every live key and retained tombstone in `volume_records`, and retries on the
vanishingly rare collision until a free key is found or a fixed
`volume_records + 1` attempt bound fails identity preflight.
Unpublished provisional keys never become lifetime records or tombstones;
they release without becoming observable if unused or if the proposal fails.
Once published, a key consumes its lifetime record permanently under the
existing `volume_records` rule.
No separate derived-key namespace, UUIDv5 ordinal allocator, unused-ordinal
gap, or `ImportAs` namespace special case is required: published keys are
ordinary `VolumeKey` values, and only those keys are persisted.

Binding 6 contains a dense mapping for each
`(participant, extraction_proposal_slot, ComponentPieceHandle)`:

```text
ComponentReservationRecordV2 (48 bytes)
  0  proposal_slot:u32
  4  piece_handle:u32
  8  volume_id_low:u32
 12  volume_id_high:u32
 16  volume_key:[u8;16]
 32  state:u32       // Reserved=1, Published=2, Unused=3, Failed=4
 36  reserved:[u32;3]
```

Every possible handle therefore has its final `VolumeId` before the adapter
dispatch.
The adapter may copy that mapping into its factory-created GPU body state in
the same ordered dispatch that writes piece assignments.
The proposal is all-or-none, so prior feedback's admitted/rejected outcome is
sufficient to reconcile those associations without an authority-path CPU
readback.
The ID is provisional until that feedback says published: it is the child's
final ID on success and is permanently invalid on failure/unused disposition.
No Moria view or effect can resolve a provisional candidate as a live volume.
Moria changes `state` only in ordered validation/publication passes; the
adapter cannot write binding 6.
The Rust mirror spells the 16 key bytes as `[u8; 16]`; WGSL spells the same
wire bytes as four `u32` words because portable host-shareable WGSL has no
storage-buffer `u8` scalar.

### Child frame, placement, and inherited facts

For each `PublishChild` piece, Moria computes the source cell with the
lexicographically smallest `(z, y, x)` key and uses its ordinary `(x, y, z)`
coordinate as the child's integer origin `o`.
The child axes equal the source local axes.
Each transferred source coordinate `c` becomes child coordinate `c - o`.
The child domain is the tight half-open AABB of those coordinates.
The child inherits the source `cell_size`, material sample bytes, and
occupancy rule exactly.
It is published as a dynamic volume.

The initial child placement is:

```text
child_placement = source_placement * translate(o * source.cell_size)
```

Rotation is exactly the source rotation.
Thus every transferred cell occupies the same world-space box immediately
before and after publication.
That equality is mathematical; GPU qualification compares each transformed
corner with tolerance
`max(8 * f32::EPSILON * max(1, abs(coordinate)), 1e-6 * cell_size)`.
Integer coordinate/sample ownership remains byte-exact.
The adapter cannot supply a discontinuous initial transform through the
component-extraction proposal.
It may publish later placement updates through the generic placement stream.

The source remains live with one incremented revision when any remainder
exists.
If no source sample remains, the same directory-generation transaction
publishes its retirement tombstone at `source_revision + 1`.
Each child begins at revision 1.

### Preparation and atomic publication

Portable WGSL phases are ordered dispatches:

1. validate proposal headers, source snapshot, labels, assignments, and
   dispositions;
2. mark each assigned source cell and stable-compact assignments by piece;
3. compute per-piece counts, bounds, and exact sample digests;
4. build new source and child bricks in unreferenced reserved slots;
5. validate cell conservation, page/slot generations, occupancy summaries,
   directory entries, and every counter/byte sentinel;
6. prepare unreferenced directory entries for the source remainder or
   tombstone and every `PublishChild` child, with their revision/placement
   gates already holding the post-extraction values; and
7. in a separate ordered publication dispatch, validate the still-current
   source revision and `directory_generation`, then perform one
   directory-generation swap that installs the complete prepared entry set;
8. after queue completion, emit observations and resolve the
   component-extraction receipt.

The construction dispatches never mutate a directory entry visible under the
old generation.
The publication pass is ordered after payload construction on the same queue.
Readers that pin `directory_generation` before the swap observe the complete
old source; readers that pin after the swap observe the complete new
source/children set.
The coordinator does not submit any consumer reader behind a candidate
generation, append its observations, or resolve its receipt until queue
completion confirms that swap's submission. A swap executed in a device
generation that is lost before confirmation is therefore never semantically
committed or externally observable; recovery reconstructs the retained old
directory. This is the same confirmed-publication boundary used by ordinary
revision gates, not a rollback of a visible commit.
Old directory entries, source pages, and slots remain pinned until every
old-generation reader and the publication submission complete.

No observable state contains duplicated, ownerless, or half-published matter.
The component-extraction proposal conflicts with every other proposal
addressing its source volume.
Two component-extraction proposals for one source in a tick are invalid.

This reuse of the existing per-volume revision/placement model and pinned
`directory_generation` snapshot is intentional.
Atomic source-to-child extraction needs one prepared multi-entry membership
swap; it does not require replacing ordinary single-volume publication with a
global epoch type, four-level immutable radix tree, or a second authority
version system for every query path.

### Failure and cleanup

- Cancellation before `Preparing` releases every candidate ID, provisional
  key, directory permit, GPU range, and byte.
- Validation, assignment, conservation, or capacity sentinel failure leaves
  the old directory generation current and releases unpublished IDs/resources
  after their last GPU use.
- An unused child slot is released after validation establishes the exact
  published prefix.
- Renderer allocation failure before execution publishes nothing and releases
  the complete reservation.
- Device loss before swap submission or after submission but before queue
  confirmation produces typed no-publication device loss. No consumer reader
  can have acquired the candidate membership; old-generation resources are
  quarantined and candidate IDs never become visible.
- Device loss after confirmed publication preserves the applied extraction.
  Recovery then follows the dirty-derived-content rule below and may fail with
  `UnrecoverableDirtyState`; it never rolls back to the parent-only state.
- Shutdown cancels only before the ordinary `Preparing` boundary.
  A submitted component extraction drains to confirmed publication or
  terminal loss before its resources can be reused.

## Directory generation and bulk placement

### Directory snapshot authority

Baseline snapshots already pin `directory_generation` before resolving volume
revisions and placements.
Component extraction is the closed exception that prepares several directory
entries and installs them with one generation swap so readers never observe a
half-split source.
Ordinary single-volume matter and placement commits continue to publish
through their existing per-volume revision/placement gates.
Directory-generation exhaustion closes the world to new multi-entry swaps;
per-volume revision exhaustion remains per-volume terminal failure under the
existing rule.

Per-volume revisions remain the freshness identity exposed to consumers.
The generation swap is an atomic visibility mechanism for membership, not a
cross-volume gameplay clock.

### Placement stream

`PlacementStream` is one bounded scheduled GPU proposal whose payload is a
stable-compacted array of placement updates:

```text
PlacementUpdateV2 (64 bytes)
  0  snapshot_index:u32
  4  flags:u32                 // v2 zero
  8  expected_revision_low:u32
 12  expected_revision_high:u32
 16  translation:[f32;4]
 32  rotation_xyzw:[f32;4]
 48  reserved:[u32;4]
```

Every entry must address a distinct dynamic volume in the participant's
filtered view.
Placements are finite and normalized under the ordinary rigid-placement
rules.
The array is sorted by snapshot index after stable GPU compaction.
Duplicate, stale, static, malformed, or over-capacity entries fail the whole
stream before publication.
Each GPU participant may emit at most one placement-stream proposal per tick;
its descriptor maximum is the aggregate entry capacity of that proposal.

Moria reserves the descriptor's complete maximum update records, revision
values, observation facts, presentation markers, outcomes, and GPU scratch
before the adapter runs.
After GPU validation, publication reuses the ordinary per-volume placement
move gates: one ordered compute pass may advance many addressed volumes, but
each volume independently compares its expected revision and writes its new
revision/placement pair.
The stream is all-or-none at the proposal level (invalid entry rejects the
whole stream before any gate writes), yet it does not claim a multi-volume
atomic membership transaction.
The result reports the exact updated `(VolumeId, VolumeRevision)` vector.
No ordinary per-volume host `VolumeCommand::Move` object, host body
enumeration, or authority-path CPU readback is created.

When one tick selects several component-extraction proposals and/or participant
placement streams, Moria orders those proposal transactions by participant
schedule order and proposal index.
Extraction proposals publish through prepared directory-generation swaps;
placement streams publish through per-volume gates.
Each proposal is all-or-none under its own rule, and unrelated proposals retain
the approved independent-publication rule: validation/preparation failure can
omit one proposal without discarding another under the selected participant
policy.
The ordered gate set is queue-confirmed as one tick submission, so device loss
before that confirmation exposes none of its candidate publications.
Every proposal receipt reports its own outcome after confirmation.

The placement stream is the selected bounded update mechanism for a
GPU-resident adapter's persistent objects.
It is still linear in the number of placements actually published, which is
unavoidable for fresh Moria placements, but removes per-object host
admission, proposal headers, queue operations, and receipts.
The GPU compacts only changed placements and Moria performs one validation
pipeline and one batched per-volume gate pass.
An adapter must not leave a moved persistent Moria volume's placement stale:
if its consumer-owned state changes the pose, it emits that pose in the current
stream or fails the participant/tick under its own policy.

The authority does not require all placement updates in a stream to become
visible as one cross-volume atomic membership change.
Reusing ordinary move gates therefore preserves independent-volume revision
semantics while still supplying the bulk GPU path the multi-fidelity proof
needs.

## CPU-authored activity regions and multi-fidelity simulation

### Substrate boundary

Moria transports activity-region definitions only as the participant's opaque
current input.
The CPU/game layer, not Moria and not the GPU, chooses their count, shapes,
full-fidelity interiors, and halo widths.
The adapter validates its own schema and rejects invalid input through its
ordinary adapter failure.

The substrate boundary for multi-fidelity participation is exactly:

1. opaque current-tick consumer input (region bytes and any other adapter
   stimuli);
2. counted GPU-adapter execution under the existing dispatch/workgroup and
   factory-resource limits; and
3. the bounded `PlacementStream` (and ordinary matter/extract effects) for
   publishing substrate outcomes.

Moria does not mandate one adapter registration/body table shape, a closed
`Full | Halo | Coarse` public vocabulary, a hierarchical compaction algorithm,
an exact dispatch count, or a fixed proof body capacity as substrate law.
Those are properties of a conforming proof adapter used as validation
evidence.

### Integration obligations (adapter-owned)

A conforming multi-fidelity proof must still demonstrate the required
capabilities without teaching Moria physics vocabulary:

- Geographic regions are not separate Moria adapters or separate Moria worlds.
  One persistent consumer-owned body table (or equivalent) classifies each
  body once against the CPU-supplied region union so overlapping regions never
  process the same body twice.
- Bodies outside every full-fidelity region continue coarse motion; they do
  not freeze or disappear solely because no activity region covers them.
- A transition halo may promote or demote fidelity without changing the body's
  stable adapter identity or its matter-backed `VolumeId`, and without a
  transform/velocity discontinuity attributable to geography alone.
- Disconnected regions use the same adapter and do not require cross-adapter
  migration.
- Changed Moria placements publish through `PlacementStream`; the adapter must
  not leave poses stale.

The class names, region schema, classification priority, list compaction, and
kernel schedule belong to the proof adapter, not Moria's public vocabulary.
The participant's Moria cell records are exported once independently of region
count.
Coarse motion and any remote destruction/debris policy remain adapter
semantics; they may produce placement streams, component extraction, ordinary
material effects, handoffs, or opaque CPU egress.

### Dispatch bound

V2 retains fixed maximum dispatch rather than exposing indirect dispatch.
Tick admission charges each adapter's declared dispatch and workgroup maxima
through the existing counted encoder.
The multi-fidelity proof adapter used for validation must fit those declared
maxima and the world defaults; P11 measures fixed-overdispatch cost at empty,
sparse, half, and full active sets for that proof.
Failure of P11 blocks the fixed-dispatch selection for that proof class and
requires a later controlled indirect-dispatch revision; implementation may not
silently expose raw `INDIRECT` buffers.
Exact dispatch counts and body capacities used by the proof are validation
evidence, not public substrate constants.

## Opaque GPU-to-CPU egress

### Transport API

Optional GPU-to-CPU egress reuses the existing bounded opaque handoff
transport shape rather than inventing a second fixed-stride record protocol.

An enabled GPU participant receives binding 7 as a Moria-initialized handoff-
shaped terminal buffer:

```text
BehaviorCpuEgressHeaderV2 (64 bytes)
  0  magic:u32 = MORC
  4  version:u32 = 2
  8  engine:u32
 12  status:u32              // Disabled=0, Empty=1, Ready=2, Overflow=3, Failed=4
 16  capacity:u32            // maximum payload bytes
 20  written_bytes:u32       // adapter-written initialized prefix length
 24  payload_offset:u32
 28  total_bytes:u32
 32  tick_low:u32
 36  tick_high:u32
 40  generation_low:u32
 44  generation_high:u32
 48  reserved:[u32;4]
```

Moria initializes `status` to `Empty`, `written_bytes` to zero, and the
payload range to zero.
The adapter writes an initialized prefix of at most `capacity` bytes and sets
`written_bytes` and `status` accordingly.
Overflow is explicit: either `status == Overflow` or
`written_bytes > capacity` fails the receipt and delivers no prefix.
Silent truncation is forbidden.
Record stride, schema identifiers, and atomic record allocators are not part
of the substrate; adapters that want fixed records pack them inside the opaque
prefix.

```rust
pub type BehaviorEgressReceipt = Receipt<BehaviorEgressCompleted>;

pub struct BehaviorEgressCompleted {
    pub tick: BehaviorTickId,
    pub engine: BehaviorEngineId,
    pub correlation: Correlation,
    pub bytes: Box<[u8]>,
}

pub enum BehaviorEgressTerminal {
    Disabled,
    Pending { receipt: BehaviorEgressReceipt },
    Unavailable { reason: BehaviorEgressFailure },
}

pub enum BehaviorEgressFailure {
    ParticipantUnavailable { reason: BehaviorEgressParticipantUnavailable },
    Overflow { written_or_required_bytes: u32, capacity: u32 },
    InvalidHeader,
    GpuValidation,
    ReadbackMap,
    Decode,
    CancelledBeforePreparation,
    Shutdown,
    DeviceLost { generation: DeviceGeneration },
}

pub enum BehaviorEgressParticipantUnavailable {
    Skipped(BehaviorEngineFailure),
    NotRun(BehaviorParticipantNotRunReason),
}
```

`BehaviorParticipantOutcome` gains `egress: BehaviorEgressTerminal`.
An enabled participant receives a receipt at tick admission.
The tick receipt may become ready after publication while the egress receipt
is still pending.
Publication selection and authority never wait for CPU interpretation of
egress bytes.
GPU header validation may detect overflow or malformed transport, but that
failure changes only the egress receipt unless the adapter independently
failed its proposal output.
Conversely, a whole-proposal rejection, another participant's tick abort, or
`NoPublication` does not discard a valid egress prefix from a participant that
executed. The owning consumer receives the tick disposition and decides what
the adapter-owned bytes mean. `BehaviorEgressFailure::GpuValidation` is used
only when dispatch/API failure makes that participant's egress writes
untrustworthy, not merely because one independently parsed proposal was
rejected.

Zero written bytes with `status == Empty` or `Ready` is a successful
`BehaviorEgressCompleted { bytes: Box::new([]) }`.
It is distinct from every unavailable/failure variant.
At exact capacity, all written bytes are delivered when `status == Ready` and
`written_bytes == capacity`.
When overflow is indicated, no prefix is delivered.
The existing asynchronous handoff map/copy/unmap lifecycle stages apply:
ordered GPU copy to staging, map after queue completion, exact host box, and
permit release on map/view-drop/unmap and result drop.

The egress receipt uses
`OperationScope::BehaviorEgress { tick, engine }`.
Every transport failure maps without loss to
`OperationErrorKind::BehaviorEgress(the_exact_failure_above)`;
the outer `device_generation` repeats the generation for `DeviceLost`.

`revision_changed` on this transport error is the independently confirmed
tick publication value; it never implies that the bytes were delivered.

### Ordering and lifetime

The GPU pass writes effects and egress in adapter order.
After the adapter dispatch, Moria validates the egress header and, on a valid
nonoverflow result, copies exactly `written_bytes` initialized bytes to its
pre-reserved staging range (the same ordered staging path used by GPU-to-CPU
handoffs).
Effect validation/publication may continue in ordered GPU passes without
waiting for the map.

Queue completion first establishes the publication terminal decision.
Mapping completion later establishes CPU egress availability.
For each adapter, Moria releases egress receipts in increasing tick order;
with one active tick this requires no unbounded reorder queue.
A ready result includes tick, participant, and request correlation so the
owning consumer can decode adapter-owned bytes without inspecting solver
state.

The working egress range is reusable only after its copy's last GPU use.
The staging range is reusable only after successful/failed map completion,
mapped-view drop, and unmap.
Decoded host bytes are copied into one exact boxed slice whose reserved host
byte permit remains charged until the result is dropped or undeliverable
delivery is discarded.
Dropping the public receipt does not cancel submitted readback; Moria still
maps or fails it and reclaims every permit.

Cancellation before `Preparing` fails the egress receipt as cancelled and
releases all capacity.
After `Preparing`, cancellation is too late.
Shutdown drains submitted egress; if terminal device/shutdown handling makes
mapping impossible, the receipt receives the explicit failure before the
shutdown report completes.
Device loss before map completion yields `DeviceLost`, even when publication
was already confirmed; the tick outcome still truthfully reports that
publication.
A map or decode failure yields its own terminal status and quarantines the
staging slot until safe unmap/destruction.

No public egress type contains a raw device, queue, mapped view, Moria
authority resource, or solver buffer.
GPU-to-GPU consumers continue to use ordered handoffs or adapter-owned
factory buffers.
Moria never routes a GPU handoff through this CPU channel.

## Scheduled GPU ABI v2

Group 0 has exactly eight storage bindings:

| Binding | Access | Contents |
| ---: | --- | --- |
| 0 | read-only | participant stable view |
| 1 | read-write | effect header, proposal records, placement/component-extraction payload |
| 2 | read-only | incoming handoffs |
| 3 | read-write | outgoing handoffs |
| 4 | read-only | prior feedback |
| 5 | read-only | current consumer input |
| 6 | read-only | component-extraction reservation/identity table or a valid empty header |
| 7 | read-write | optional opaque CPU-egress terminal header/payload or a valid disabled header |

The unchanged v1 view, volume, cell, input, handoff, and feedback layouts are
embedded exactly.
The effect header uses ABI version 2 and adds proposal kinds:
placement stream `5` and extract components `6`.
An ordinary create record remains unrepresentable.

`ComponentReservationHeaderV2` is 64 bytes:

```text
0 magic:u32 = MORR
4 version:u32 = 2
8 engine:u32
12 record_count:u32
16 record_offset:u32
20 total_bytes:u32
24 tick_low:u32
28 tick_high:u32
32 generation_low:u32
36 generation_high:u32
40 reserved:[u32;6]
```

The header is followed by 48-byte reservation records in proposal-slot then
piece-handle order.
The empty table is a valid header with zero records and `total_bytes == 64`.

An extract-components proposal's payload begins with:

```text
ComponentExtractionPayloadHeaderV2 (64 bytes)
  0 piece_count:u32
  4 assignment_count:u32
  8 piece_offset:u32
 12 assignment_offset:u32
 16 total_bytes:u32
 20 reserved:[u32;11]

ComponentPieceRecordV2 (32 bytes)
  0 piece_handle:u32
  4 disposition:u32          // PublishChild=1, RemoveFromMatter=2
  8 declared_cell_count:u32
 12 reserved0:u32
 16 reserved:[u32;4]

ComponentAssignmentRecordV2 (24 bytes)
  0 source_x:i32
  4 source_y:i32
  8 source_z:i32
 12 piece_handle:u32
 16 reserved:[u32;2]
```

Records are sorted by piece handle, then source Z/Y/X.
Counts, ranges, reserved words, duplicate cells, empty-source assignments, and
unreserved handles are validated.

Disabled egress requires zero capacity/written bytes and `total_bytes == 64`.
Enabled requires `total_bytes == align4(64 + maximum_cpu_egress_bytes)` and
descriptor equality for capacity.
After execution, overflow or invalid header/status produces the explicit
failure and no payload delivery.

All count-to-byte arithmetic is checked before allocation and again on GPU.
Every effective binding range fits
`max_storage_buffer_binding_size`.
Scheduled v2 requires at least eight storage bindings in one shader stage.

## Resource limits

`ResourceLimits` and `ResourceKind` add independent fields:

| Field | Default | Hard maximum / relationship |
| --- | ---: | --- |
| `behavior_placement_updates` | 65,536 | 1,048,576; aggregate descriptor maxima fit |
| `behavior_placement_bytes` | 4 MiB | 64 MiB; `>= 64 * behavior_placement_updates` |
| `behavior_component_extraction_proposals` | 16 | 1,024 |
| `behavior_component_extraction_children` | 256 | 4,096; `live_volumes` and `volume_records` each cover initial live records plus this reserved maximum |
| `behavior_component_extraction_assignment_cells` | 262,144 | 1,048,576 |
| `behavior_component_extraction_child_bricks` | 4,096 | 65,536 |
| `behavior_component_extraction_bytes` | 32 MiB | 256 MiB; covers mapping, payload, transfer, validation, and prepared directory-entry scratch |
| `behavior_egress_bytes` | 16 MiB desired / 1 MiB minimum as `GpuCapacityLimit` host companion | 256 MiB and adapter allocation; covers device, staging, and exact decoded host deliveries for enabled descriptors |
| `behavior_egress_maps` | 16 | 256; at least enabled GPU participant count; reuses the handoff map lifecycle |
| `behavior_egress_receipts` | 64 | 4,096; bounds pending and retained terminal egress results |

These pools do not alias proposal, generic staging, command, observation, or
adapter factory-resource pools.
Egress accounting is independent of adapter-to-adapter handoff edge maxima but
reuses the same staging/map/unmap machinery.
Tick admission reserves complete descriptor maxima before execution.
For each selected component-extraction proposal it reserves one prepared
directory-generation swap, directory entries for source and every child, and
the associated page/brick/scar records; path sharing only releases unused
capacity after construction.
Telemetry reports current, high-water, waiting/rejected, last-use-delayed
bytes, overflow count, map/decode failure count, unused component-extraction
reservations, matter-conservation failures, placement update count, and
directory-generation swap latency.

## Persistence and rematerialization

Every child has:

```text
DerivedExtractionProvenanceV2 {
    parent_volume_key,
    parent_revision,          // pinned revision before extraction
    extraction_command_id,
    piece_handle,
    sample_count,
    sample_digest,
}
```

This is substrate provenance, not a behavior label.
The child does not have a consumer `BaseContentSource`.
Its publication creates an internal derived-content base consisting of every
nonempty child brick plus the provenance record.
Those bricks are dirty authoritative persistence input.
They remain pinned in retained GPU/scar capacity until a checkpoint is durable.
Before that checkpoint, device loss is `UnrecoverableDirtyState` under the
existing rule.

Checkpoint format v2 adds the provenance tag and stores the complete sparse
derived base for each child under its ordinary published `VolumeKey`.
Later edits are ordinary full-brick scars relative to that stored derived
base.
Restore recreates child identity, domain, cell size, dynamic mode, placement,
provenance, and sample bytes without a consumer content source.
The parent need not still be live.
Cold rematerialization loads the persisted derived base first, then later
scars.
Digest/count mismatch, missing derived chunk, or an attempt to attach a
consumer source to the child fails restore.

Placement streams persist only the resulting committed per-volume placements
and revisions.
Activity-region input, adapter body/fidelity state, coarse/full state,
handoffs, and egress bytes are never persisted by Moria.

## Required validation

The validation plan must retain all prior scheduled-adapter evidence and add:

1. one real-GPU adapter labels at least three connected pieces in one source,
   publishes at least two children, retains a source remainder, and proves
   exact coordinate/sample conservation and tolerance-bounded world-box
   continuity across one prepared directory-generation swap;
2. every candidate identity/resource is reserved before adapter execution,
   binding 6 maps piece handles to final `VolumeId`s and provisional ordinary
   `VolumeKey`s, only published keys become lifetime records, and the adapter
   updates its factory-owned body table without CPU authority-path readback;
3. cancellation, every validation sentinel, exhausted live/lifetime/page/
   brick/scar/byte pool, renderer OOM, device loss before and after the swap,
   shutdown, unused child slots, and old-reader reclamation leak no capacity;
4. checkpoint/restore and cold rematerialization reproduce child identity,
   provenance, placement, samples, occupancy, and later edits without a
   consumer source or a separate derived-key namespace;
5. a proof adapter consumes CPU-authored disconnected and overlapping regions,
   processes each body once in the overlap, preserves one body/volume identity
   across halo promotion/demotion, has no transform/velocity discontinuity,
   and continues coarse motion/destruction-shaped generic effects outside all
   regions—while using only opaque input, counted dispatches, and
   `PlacementStream` as substrate hooks;
6. the placement stream publishes compacted changes through ordinary
   per-volume move gates without host body enumeration, stale Moria placement,
   duplicate volume entry, or per-object ordinary host move proposals;
7. fixed maximum dispatch for the multi-fidelity proof is measured at empty,
   sparse, half, and full active sets and passes P11 on every claimed backend
   family; and
8. an adapter-owned egress byte layout unknown to Moria round-trips
   byte-exactly for zero bytes, one payload, exact capacity, and multiple
   ticks; overflow, malformed header, cancellation, shutdown, map/decode
   failure, and device loss produce distinct terminal results with no silent
   bytes or leaked/reused-early buffers.

The adversarial reviewer must also demonstrate that none of these APIs adds a
Moria type for physics, damage, weapons, scoring, audio, region significance,
velocity, force, health, or debris policy.
