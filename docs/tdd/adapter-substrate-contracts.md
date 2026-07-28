# Adapter Substrate Contracts

## Purpose and authority

This file specifies the three substrate hooks required by human review of the
scheduled behavior boundary:

1. atomic extraction of existing authoritative matter into new child volumes;
2. bounded bulk placement publication so one persistent multi-fidelity adapter
   can keep Moria placements current without per-object host moves; and
3. optional bounded opaque GPU-to-CPU adapter egress.

They do not add physics, damage, weapons, fracture policy, body state,
significance thresholds, activity-region policy, scoring, audio, or gameplay
events to Moria. An adapter may give records those meanings; Moria sees only
bounded regions, existing samples, placements, identities, opaque bytes, and
publication outcomes.

Scheduled GPU ABI v2 supersedes the six-binding Scheduled ABI v1 selection in
T29, T30, T32, and T33 for group-0 layout and accepted proposal kinds.
Unchanged v1 record layouts retain their `V1` type names inside v2.
There is no runtime v1/v2 negotiation: configuration accepts only scheduled
ABI version 2.

## Public capability declarations

`BehaviorEngineDescriptor` gains:

```rust
pub struct BehaviorEngineDescriptor {
    // Existing fields remain.
    pub maximum_placement_updates: u32,              // zero for CPU
    pub maximum_component_extraction_children: u32,  // zero for CPU
    pub maximum_component_extraction_bytes: u64,     // zero for CPU
    pub cpu_egress: BehaviorCpuEgressDescriptor,
}

pub struct BehaviorCpuEgressDescriptor {
    pub schema: [u8; 16],
    pub record_stride: u32,
    pub maximum_records: u32,
    pub maximum_bytes: u64,
}
```

`maximum_placement_updates` is the aggregate placement-stream entry capacity
for that participant in one tick (zero disables the stream).
`maximum_component_extraction_children` is the aggregate candidate-child
capacity across that participant's extract-components proposals in one tick.
`maximum_component_extraction_bytes` bounds assignment, piece, transfer, and
validation payload for those proposals. For a GPU adapter the two extraction
fields are either both zero or both nonzero. Checked products and sums must
fit the adapter maxima and world resource pools before registration succeeds.

CPU egress is disabled exactly when `maximum_records == 0`,
`maximum_bytes == 0`, `record_stride == 0`, and `schema == [0; 16]`.
When enabled, `record_stride` is a multiple of four in `4..=65_536`,
`maximum_records * record_stride == maximum_bytes`, and the product is checked
in `u64`. Schema and record bytes are opaque to Moria.

The descriptor does not declare activity regions. Region definitions remain
current-tick consumer input under the existing opaque input contract. A
multi-fidelity adapter defines and validates its own input schema within its
declared input byte maximum.

## Atomic component extraction

### Operation boundary

`ExtractComponents` is a scheduled GPU substrate effect, not ordinary
`VolumeCommand::Create` and not arbitrary scheduled creation. It redistributes
samples that are occupied in one pinned source-volume revision. It cannot
introduce a material sample, obtain a content source, clone a cell, address
another source volume, or create a child with consumer-authored base content.

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
    pub directory_generation: u64,
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

Cells with no piece assignment remain in the source. `PublishChild` transfers
the assigned samples to a new persistent Moria volume. `RemoveFromMatter` is
ordinary explicit removal: Moria reports exact cell count and digest but
creates no debris, effect, or gameplay event. An adapter may interpret that
disposition as transient debris or a visual effect through its own state or
egress. Moria does not make that policy choice.

Conservation is over canonical source-space records
`(source_z, source_y, source_x, packed_sample)`, not material values alone:

```text
source occupied coordinate/sample multiset before
  = source remainder coordinate/sample multiset after
  + every child record mapped back by its origin
  + explicitly removed coordinate/sample multiset
```

Every source sample appears in exactly one term. Unassigned samples are never
inferred as removed. GPU validation uses assignment marks and exact counts for
this proof; reported BLAKE3 digests are canonical coordinate-plus-sample
evidence. Child digests use child-local coordinates; removal digests use
source coordinates.

Moria validates piece-handle uniqueness per assigned source cell and exact
source membership. The adapter owns component discovery and significance.
Moria does not define connectivity. The fracture-shaped proof adapter must
produce six-neighbor connected pieces; the independent oracle verifies that.

### Pre-reservation and identity

Before the tick enters `Preparing`, Moria atomically reserves the worst case
declared by every enabled GPU participant:

- proposal and outcome capacity for extract-components up to the participant's
  proposal maxima;
- one live directory entry, permanent lifetime record, `VolumeId`,
  consumer-visible `VolumeKey`, revision/placement slot, observation fact,
  presentation marker, and receipt child record for every possible child;
- source and child page keys, page versions, detailed brick slots, scar
  records, occupancy summaries, transfer and assignment scratch, and
  validation diagnostics within `maximum_component_extraction_bytes` and the
  ordinary page/brick/scar pools; and
- every submission, feedback, persistence provenance, and cleanup bookkeeping
  byte needed by that maximum.

Reservation is one checked acquisition. Failure leaves the tick queued or
rejects it under the behavior-tick overload policy; no adapter executes.

Runtime IDs and stable keys are allocated on the CPU from already reserved
slots before GPU execution. Publication does not depend on a CPU authority-path
readback of matter. `VolumeId` uses the ordinary pre-reserved generational live
slot. `VolumeKey` is UUIDv5 in the world's persistent
`derived_key_namespace` over `"moria-derived-v2" || candidate_ordinal_le`;
the world owns a checked monotonic `u64` candidate ordinal. Fresh worlds
initialize that namespace from `WorldKey`; checkpoint/restore preserves it
unchanged, including `ImportAs`. Exact registry collisions advance the ordinal
and retry at most `volume_records + 1` times per child; exhausting that bound
or the ordinal fails identity preflight before any adapter runs.

Unpublished stable keys are reservation candidates, not lifetime records or
tombstones. They release without becoming observable if unused or if the
proposal fails. Once published, a key consumes its lifetime record permanently.
Candidate ordinals are never reused even when keys remain unpublished.

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

Every possible handle therefore has its final `VolumeId` before adapter
dispatch. The adapter may copy that mapping into factory-owned GPU body state
in the same ordered dispatch that writes piece assignments. Prior feedback's
admitted/rejected outcome is sufficient to reconcile associations without an
authority-path CPU readback. The ID is provisional until that feedback says
published. No Moria view or effect can resolve a provisional candidate as a
live volume. Moria changes `state` only in ordered validation/publication
passes; the adapter cannot write binding 6.

### Child frame, placement, and inherited facts

For each `PublishChild` piece, Moria computes the source cell with the
lexicographically smallest `(z, y, x)` key and uses its ordinary `(x, y, z)`
coordinate as the child's integer origin `o`. Child axes equal the source local
axes. Each transferred source coordinate `c` becomes child coordinate `c - o`.
The child domain is the tight half-open AABB of those coordinates. The child
inherits the source `cell_size`, material sample bytes, and occupancy rule
exactly. It is published as a dynamic volume.

The initial child placement is:

```text
child_placement = source_placement * translate(o * source.cell_size)
```

Rotation is exactly the source rotation. Every transferred cell occupies the
same world-space box immediately before and after publication. That equality
is mathematical; GPU qualification compares each transformed corner with
tolerance
`max(8 * f32::EPSILON * max(1, abs(coordinate)), 1e-6 * cell_size)`.
Integer coordinate/sample ownership remains byte-exact. The adapter cannot
supply a discontinuous initial transform through the extract-components
proposal. It may publish later placement updates through the placement stream.

The source remains live with one incremented revision when any remainder
exists. If no source sample remains, the same multi-entry publication installs
its retirement tombstone at `source_revision + 1`. Each child begins at
revision 1.

### Atomic multi-entry publication

Extract-components reuses the product's existing directory-generation and
per-volume publication model rather than inventing a parallel directory ABI.

Portable ordered phases:

1. validate proposal headers, source snapshot, labels, assignments, and
   dispositions;
2. mark assigned source cells and compact by piece;
3. compute per-piece counts, bounds, and exact sample digests;
4. build new source and child bricks in unreferenced reserved slots;
5. validate conservation, page/slot generations, occupancy summaries, directory
   entries, and counters;
6. prepare every new directory entry, tombstone, and authority version in
   unreferenced storage; and
7. in a separate ordered publication pass, install those entries together under
   one advanced `directory_generation`, using the same confirmed-publication
   boundary as ordinary revision gates (prepare offline, gate after payload
   construction, withhold consumer readers/observations/receipts until queue
   confirmation).

Readers that acquire the old directory generation see the complete old source.
Readers that acquire the new generation see the complete new source/children
set. No observable state contains duplicated, ownerless, or half-published
matter. The extract-components proposal conflicts with every other proposal
addressing its source volume. Two extract-components proposals for one source
in a tick are invalid.

When one tick selects several extract-components and/or placement-stream
proposals, Moria orders them by participant schedule order and proposal index.
Each proposal is all-or-none. Unrelated proposals retain independent
publication: validation failure may omit one without discarding another under
the selected participant policy. Device loss before the tick's submission is
queue-confirmed exposes none of its candidate multi-entry installs.

### Failure and cleanup

- Cancellation before `Preparing` releases every candidate ID, directory
  permit, GPU range, and byte.
- Validation, assignment, conservation, or capacity failure leaves the old
  directory generation current and releases unpublished IDs/resources after
  their last GPU use.
- An unused child slot is released after validation establishes the exact
  published set.
- Renderer allocation failure before execution publishes nothing and releases
  the complete reservation.
- Device loss before confirmed publication produces typed no-publication device
  loss; candidate IDs never become visible and the old generation remains.
- Device loss after confirmed publication preserves the applied extraction.
  Recovery follows the dirty-derived-content rule and may fail with
  `UnrecoverableDirtyState`; it never rolls back to the parent-only state.
- Shutdown cancels only before the ordinary `Preparing` boundary. A submitted
  extraction drains to confirmed publication or terminal loss before reuse.

## Bulk placement stream

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
filtered view. Placements are finite and normalized under ordinary
rigid-placement rules. The array is sorted by snapshot index after stable GPU
compaction. Duplicate, stale, static, malformed, or over-capacity entries fail
the whole stream before publication. Each GPU participant may emit at most one
placement-stream proposal per tick; its descriptor maximum is that proposal's
aggregate entry capacity.

Moria reserves the descriptor's complete maximum update records, directory
entry/authority versions, observation facts, presentation markers, outcomes,
and GPU scratch before the adapter runs. Publication installs all valid
placement entries under one advanced `directory_generation`; each addressed
volume independently advances its revision by one. The result reports the
exact updated `(VolumeId, VolumeRevision)` vector and the new directory
generation. No ordinary per-volume `VolumeCommand::Move` object, host body
enumeration, or authority-path CPU readback is created.

The placement stream is the selected bounded update mechanism for a
GPU-resident adapter's persistent coarse and full-fidelity objects. It is
linear in the number of placements actually published—unavoidable for fresh
Moria placements—but removes per-object host admission, proposal headers,
queue operations, and receipts. An adapter must not leave a moved persistent
Moria volume's placement stale: if consumer-owned state changes the pose, it
emits that pose in the current stream or fails the participant/tick under its
own policy.

```rust
pub struct PlacementStreamApplied {
    pub command: CommandId,
    pub updated: Vec<(VolumeId, VolumeRevision)>,
    pub directory_generation: u64,
    pub correlation: Correlation,
}
```

## CPU-authored activity regions and multi-fidelity simulation

Moria transports activity-region definitions only as the participant's opaque
current input. The CPU/game layer, not Moria and not the GPU, chooses their
count, shapes, full-fidelity interiors, and halo widths. The adapter validates
its own schema and rejects invalid input through ordinary adapter failure.

A conforming multi-fidelity GPU adapter uses one persistent adapter
registration and one consumer-owned body table for the whole world.
Geographic regions are not separate Moria adapters or separate Moria worlds.
Each persistent body has one stable adapter-owned record and, when
matter-backed, one `VolumeId`.

Classification, fidelity classes, coarse/full kernels, transition/halo policy,
velocity fields, remote destruction, and debris outcomes are adapter semantics.
Substrate obligations are only:

- accept CPU-authored region bytes through the existing opaque input path;
- allow one persistent GPU adapter to keep bodies continuously owned across
  geography without Moria migration protocols;
- publish changed poses through the placement stream so Moria placements stay
  current without per-object host move admission; and
- retain fixed maximum dispatch for the proof workload and make that choice
  falsifiable (P11).

The proof adapter used for validation must demonstrate:

- disconnected and overlapping CPU-authored regions;
- each body processed once in an overlap (deterministic union owned by the
  adapter);
- continuous ownership and no adapter-oracle transform/velocity discontinuity
  across promotion/demotion; and
- continued coarse motion and destruction-shaped generic effects outside every
  full-fidelity region.

Moria acquires no region, fidelity-class, velocity, or simulation vocabulary.

### Dispatch bound

V2 retains fixed maximum dispatch rather than exposing indirect dispatch.
The multi-fidelity proof adapter declares dispatch and workgroup maxima that
cover classification, simulation passes, and placement compaction at the
65,536-body proof capacity. Tick admission charges those declared maxima.
P11 measures empty, sparse, half, and full active lists on every claimed
backend family. Failure of P11 blocks the fixed-dispatch selection and requires
a later controlled revision; implementation may not silently expose raw
`INDIRECT` buffers.

The exact kernel schedule, compaction algorithm, and fidelity class names are
proof-adapter internals documented in the validation harness, not public Moria
types or ABI.

## Opaque GPU-to-CPU egress

### Transport API

An enabled GPU participant receives binding 7 as a zero-initialized egress
header followed by `maximum_bytes`. Moria supplies a WGSL helper that
atomically reserves one fixed-stride record: the returned index is written only
when it is below `maximum_records`; otherwise the helper sets overflow and
preserves the full required count. The adapter must use that helper for every
record. Allocation order is adapter-defined; Moria preserves the initialized
prefix byte-for-byte and does not sort or decode it.

```rust
pub type BehaviorEgressReceipt = Receipt<BehaviorEgressCompleted>;

pub struct BehaviorEgressCompleted {
    pub tick: BehaviorTickId,
    pub engine: BehaviorEngineId,
    pub correlation: Correlation,
    pub schema: [u8; 16],
    pub record_stride: u32,
    pub record_count: u32,
    pub bytes: Box<[u8]>,
}

pub enum BehaviorEgressTerminal {
    Disabled,
    Pending { receipt: BehaviorEgressReceipt },
    Unavailable { reason: BehaviorEgressFailure },
}

pub enum BehaviorEgressFailure {
    ParticipantUnavailable { reason: BehaviorEgressParticipantUnavailable },
    Overflow { required_records: u32, capacity: u32 },
    CounterOverflow,
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

`BehaviorParticipantOutcome` gains `egress: BehaviorEgressTerminal`. An enabled
participant receives a receipt at tick admission. The tick receipt may become
ready after publication while the egress receipt is still pending. Publication
selection and authority never wait for CPU interpretation of egress bytes.

GPU header validation may detect overflow or malformed transport; that failure
changes only the egress receipt unless the adapter independently failed its
proposal output. Conversely, a whole-proposal rejection, another participant's
tick abort, or `NoPublication` does not discard a valid egress prefix from a
participant that executed. The owning consumer receives the tick disposition
and decides what the adapter-owned bytes mean.

Zero records is a successful
`BehaviorEgressCompleted { record_count: 0, bytes: Box::new([]) }`.
It is distinct from every unavailable/failure variant. At exact capacity, all
records are delivered. When `required_records > maximum_records` or the
overflow flag is set, no prefix is delivered and the receipt fails with the
exact required/capacity values. Silent truncation is forbidden. An attempt
beyond `u32::MAX` sets `CounterOverflow` and delivers no prefix.

The egress receipt uses
`OperationScope::BehaviorEgress { tick, engine }`.
Every transport failure maps without loss to
`OperationErrorKind::BehaviorEgress(the_exact_failure_above)`.

### Ordering and lifetime

After adapter dispatch, Moria validates the egress header and, on a valid
nonoverflow result, copies exactly `record_count * record_stride` initialized
bytes to pre-reserved staging. Effect validation/publication may continue in
ordered GPU passes without waiting for the map.

Queue completion first establishes the publication terminal decision. Mapping
completion later establishes CPU egress availability. For each adapter, Moria
releases egress receipts in increasing tick order. A ready result includes
tick, participant, request correlation, and schema.

Working egress ranges are reusable only after their copy's last GPU use.
Staging is reusable only after successful/failed map completion, mapped-view
drop, and unmap. Decoded host bytes are copied into one exact boxed slice
whose host permit remains charged until the result is dropped or undeliverable
delivery is discarded. Dropping the public receipt does not cancel submitted
readback.

Cancellation before `Preparing` fails the egress receipt as cancelled and
releases all capacity. After `Preparing`, cancellation is too late. Shutdown
drains submitted egress; if mapping is impossible, the receipt receives the
explicit failure before the shutdown report completes. Device loss before map
completion yields `DeviceLost`, even when publication was already confirmed.
A map or decode failure yields its own terminal status.

No public egress type contains a raw device, queue, mapped view, Moria
authority resource, or solver buffer. GPU-to-GPU consumers continue to use
ordered handoffs or adapter-owned factory buffers. Moria never routes a GPU
handoff through this CPU channel.

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
| 7 | read-write | opaque CPU-egress header/payload or a valid disabled header |

Unchanged v1 view, volume, cell, input, handoff, and feedback layouts are
embedded exactly. The effect header uses ABI version 2 and adds proposal kinds:
placement stream `5` and extract components `6`. An ordinary create record
remains unrepresentable.

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
piece-handle order. The empty table is a valid header with zero records and
`total_bytes == 64`.

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

Records are sorted by piece handle, then source Z/Y/X. Counts, ranges,
reserved words, duplicate cells, empty-source assignments, and unreserved
handles are validated.

`BehaviorEgressHeaderV2` is 80 bytes:

```text
0  magic:u32 = MORO
4  version:u32 = 2
8  engine:u32
12 status:u32              // Disabled=0, Enabled=1
16 record_stride:u32
20 record_capacity:u32
24 required_records:atomic<u32>
28 overflow:atomic<u32>
32 payload_offset:u32
36 total_bytes:u32
40 tick_low:u32
44 tick_high:u32
48 generation_low:u32
52 generation_high:u32
56 schema:[u8;16]           // WGSL mirror: array<u32,4>
72 reserved:[u32;2]
```

Moria initializes counters, overflow, payload, padding, and reserved words to
zero. Disabled requires zero stride/capacity/counters and `total_bytes == 80`.
Enabled requires descriptor equality and
`total_bytes == align4(80 + maximum_bytes)`.
After execution, `overflow != 0` or `required_records > record_capacity`
produces the explicit overflow failure and no payload delivery.

All count-to-byte arithmetic is checked before allocation and again on GPU.
Every effective binding range fits `max_storage_buffer_binding_size`.
Scheduled v2 requires at least eight storage bindings in one shader stage.

## Resource limits

`ResourceLimits` and `ResourceKind` add independent fields charged at
registration and tick admission:

| Field | Default | Hard maximum / relationship |
| --- | ---: | --- |
| `behavior_placement_updates` | 65,536 | 1,048,576; aggregate descriptor maxima fit |
| `behavior_placement_bytes` | 4 MiB | 64 MiB; `>= 64 * behavior_placement_updates` |
| `behavior_component_extraction_children` | 256 | 4,096; live/lifetime volume records cover initial live records plus this reserved maximum |
| `behavior_component_extraction_bytes` | 32 MiB | 256 MiB; mapping, payload, transfer, and validation records |
| `behavior_egress_maps` | 16 | 256; at least enabled GPU participant count |
| `behavior_egress_receipts` | 64 | 4,096; bounds pending and retained terminal egress results |
| `behavior_egress_records` | 16,384 | 1,048,576; aggregate descriptor maxima fit |
| `behavior_egress_bytes` | 16 MiB desired / 1 MiB minimum | 256 MiB and adapter allocation; covers `align4(80 + maximum_bytes)` device ranges, staging payloads, and exact decoded host deliveries |

These pools do not alias proposal, handoff, generic staging, command, or
observation pools. Tick admission reserves complete descriptor maxima before
execution. Multi-entry publication charges ordinary directory/page/brick/scar
capacity for every changed entry; no separate public radix-node or
directory-root pool is introduced. Telemetry reports current and high-water
use, waiting/rejected counts, unused component-extraction reservations,
matter-conservation failures, placement update counts, and egress
overflow/map/decode failures.

## Persistence and rematerialization

Every published child has substrate provenance:

```text
DerivedExtractionProvenanceV2 {
    parent_volume_key,
    parent_revision,          // pinned revision before extraction
    extraction_command_id,
    candidate_ordinal,
    piece_handle,
    sample_count,
    sample_digest,
}
```

The child does not have a consumer `BaseContentSource`. Publication creates an
internal derived-content base of every nonempty child brick plus the provenance
record. Those bricks are dirty authoritative persistence input and remain
pinned until a checkpoint is durable. Before that checkpoint, device loss is
`UnrecoverableDirtyState` under the existing rule.

Checkpoint format v2 adds the provenance tag and stores the complete sparse
derived base for each child. The manifest also stores the world's next derived
candidate ordinal; restore requires it to be greater than every saved child's
provenance ordinal and continues from it without key reuse. Later edits are
ordinary full-brick scars relative to that stored derived base. Restore
recreates child identity, domain, cell size, dynamic mode, placement,
provenance, and sample bytes without a consumer content source. The parent
need not still be live. Cold rematerialization loads the persisted derived base
first, then later scars. Digest/count mismatch, missing derived chunk, or an
attempt to attach a consumer source to the child fails restore.

Placement streams persist only resulting committed per-volume placements and
revisions. Activity-region input, adapter body/fidelity state, handoffs, and
egress bytes are never persisted by Moria.

## Required validation

The validation plan must retain all prior scheduled-adapter evidence and add:

1. one real-GPU adapter labels at least three connected pieces in one source,
   publishes at least two children, retains a source remainder, and proves
   exact coordinate/sample conservation and tolerance-bounded world-box
   continuity across one multi-entry directory-generation publication;
2. every candidate identity/resource is reserved before adapter execution,
   binding 6 maps piece handles to final IDs, and the adapter updates its
   factory-owned body table without CPU authority-path readback;
3. cancellation, validation failure, exhausted live/lifetime/page/brick/scar/
   byte pools, renderer OOM, device loss before and after publication,
   shutdown, unused child slots, and old-reader reclamation leak no capacity;
4. checkpoint/restore and cold rematerialization reproduce child identity,
   provenance, placement, samples, occupancy, and later edits without a
   consumer source;
5. a proof adapter consumes CPU-authored disconnected and overlapping regions,
   processes each body once in the overlap, preserves one body/volume identity
   across fidelity transitions, and continues coarse motion/destruction-shaped
   generic effects outside all regions;
6. the placement stream publishes compacted changes without host body
   enumeration, stale Moria placement, duplicate volume entry, or per-object
   ordinary move proposals;
7. fixed maximum dispatch is measured at empty, sparse, half, and full active
   lists and passes P11 on every claimed backend family; and
8. an adapter-owned egress struct unknown to Moria round-trips byte-exactly for
   zero records, one record, exact capacity, and multiple ticks; one-over
   capacity, malformed header, cancellation, shutdown, map/decode failure, and
   device loss produce distinct terminal results with no silent bytes or
   leaked/reused-early buffers.

The adversarial reviewer must also demonstrate that none of these APIs adds a
Moria type for physics, damage, weapons, scoring, audio, region significance,
velocity, force, health, or debris policy.
