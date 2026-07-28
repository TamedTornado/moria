# Adapter Substrate Contracts

## Purpose and authority

This file specifies three generic transports required by the scheduled
behavior boundary:

1. atomic extraction of existing authoritative matter into new child volumes;
2. a bounded GPU batch of placement updates for adapters that keep persistent
   multi-fidelity simulation outside Moria; and
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
    pub cpu_egress: BehaviorCpuEgressDescriptor,
}

pub struct BehaviorCpuEgressDescriptor {
    pub maximum_records: u32,
    pub maximum_bytes: u64,
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

CPU egress is disabled exactly when `maximum_records == 0` and
`maximum_bytes == 0`.
When enabled, both fields are nonzero, `maximum_bytes` is at most the
configured host/staging ceilings that also serve ordinary handoff and
readback work, and the pair is checked in `u64`.
Moria does not declare, store, or enforce a record stride or schema
identifier; the adapter owns any record layout inside the opaque initialized
prefix.

The descriptor does not declare activity regions.
Region definitions remain current-tick consumer input under the existing
opaque input contract.
A multi-fidelity adapter defines and validates its own input schema within its
declared input byte maximum.

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
- one live directory entry, permanent lifetime record, `VolumeId`,
  consumer-visible `VolumeKey`, revision slot, observation fact, presentation
  marker, and receipt child record for every possible child;
- source and child page keys, page versions, detailed brick slots, scar
  records, occupancy summaries, transfer records, assignment records, and
  validation diagnostics;
- the complete binding-6 reservation table and component-extraction payload
  bytes; and
- every byte of submission, feedback, persistence provenance, and cleanup
  bookkeeping needed by the maximum operation.

Reservation is one checked acquisition against the existing live-volume,
lifetime-volume, page, brick, scar, proposal, and staging pools.
Moria never holds a subset while waiting for another pool.
Failure leaves the tick queued or rejects it under the behavior-tick overload
policy; no adapter executes.

Runtime IDs and stable keys are allocated on the CPU from already reserved
slots before GPU execution, but publication does not depend on a CPU readback.
`VolumeId` uses the ordinary pre-reserved generational live slot.
`VolumeKey` uses the ordinary lifetime-record allocation path used by create:
each reserved child receives a distinct key that cannot collide with a live
key or retained tombstone, and once published that key permanently consumes
its lifetime record.
Unpublished candidate keys and IDs are reservation holders only; they are
released without becoming observable if unused or if the proposal fails.
No separate derived-key namespace, candidate ordinal, or UUIDv5 derivation
lifecycle is introduced.

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
It may publish later placement updates through the generic placement-batch
proposal.

The source remains live with one incremented revision when any remainder
exists.
If no source sample remains, the same extraction transaction publishes its
retirement tombstone at `source_revision + 1`.
Each child begins at revision 1.

### Preparation and atomic publication

Portable WGSL phases are ordered dispatches:

1. validate proposal headers, source snapshot, labels, assignments, and
   dispositions;
2. mark each assigned source cell and stable-compact assignments by piece;
3. compute per-piece counts, bounds, and exact sample digests;
4. build new source remainder and child bricks in unreferenced reserved slots;
5. validate cell conservation, page/slot generations, occupancy summaries,
   directory entries, and every counter/byte sentinel;
6. install the source change and every child directory entry as one extraction
   transaction using the existing directory-generation / per-volume gate
   machinery ordered so no consumer reader can observe a partial result; and
7. after queue completion, emit observations and resolve the
   component-extraction receipt.

Construction never mutates a directory entry or volume revision visible to
readers.
Publication is ordered after payload construction on the same queue and is the
linearization point for the whole extraction: readers and later proposals see
either the complete pre-extraction source or the complete post-extraction
source remainder/tombstone plus every published child.
Ordinary single-volume matter, move, and retire proposals continue to use
their existing per-volume gates; extraction does not replace those paths and
does not expose a public world-directory epoch type.

The coordinator does not submit any consumer reader behind the candidate
install, append its observations, or resolve its receipt until queue
completion confirms the publication submission.
A gate executed in a device generation that is lost before confirmation is
never semantically committed or externally observable; recovery reconstructs
the retained pre-extraction source.
This is the same confirmed-publication boundary used by ordinary revision
gates, not a rollback of a visible commit.
Old source pages and slots remain pinned until every pre-extraction reader and
the publication submission complete.

No observable state contains duplicated, ownerless, or half-published matter.
The component-extraction proposal conflicts with every other proposal
addressing its source volume.
Two component-extraction proposals for one source in a tick are invalid.

### Failure and cleanup

- Cancellation before `Preparing` releases every candidate ID, directory
  permit, GPU range, and byte.
- Validation, assignment, conservation, or capacity sentinel failure leaves
  the pre-extraction source current and releases unpublished IDs/resources
  after their last GPU use.
- An unused child slot is released after validation establishes the exact
  published prefix.
- Renderer allocation failure before execution publishes nothing and releases
  the complete reservation.
- Device loss before gate submission or after submission but before queue
  confirmation produces typed no-publication device loss. No consumer reader
  can have acquired a candidate child or modified source; old-generation
  resources are quarantined and candidate IDs never become visible.
- Device loss after confirmed publication preserves the applied extraction.
  Recovery then follows the dirty-scar rule below and may fail with
  `UnrecoverableDirtyState`; it never rolls back to the parent-only state.
- Shutdown cancels only before the ordinary `Preparing` boundary.
  A submitted component extraction drains to confirmed publication or
  terminal loss before its resources can be reused.

## Bounded placement-update batch

`PlacementBatch` is one bounded scheduled GPU proposal whose payload is a
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
batch before any entry publishes.
Each GPU participant may emit at most one placement-batch proposal per tick;
its descriptor maximum is the aggregate entry capacity of that proposal.

Moria reserves the descriptor's complete maximum update records, ordinary
per-volume move directory versions, revision values, observation facts,
presentation markers, outcomes, and GPU scratch before the adapter runs.
Publication reuses the existing per-volume move gate for each entry: each
addressed volume independently advances its revision by one when its gate
succeeds.
Cross-volume placement atomicity is not required and is not provided.
The result reports the exact updated `(VolumeId, VolumeRevision)` vector
through the ordinary tick/proposal receipt path; those pairs are the same
revision vector already carried by `BehaviorTickCompleted::published`.
No host body enumeration or authority-path CPU readback is required.

The placement batch is the selected bounded update mechanism for a
GPU-resident adapter's changed persistent poses.
It is still linear in the number of placements actually published, which is
unavoidable for fresh Moria placements, but removes per-object host
admission headers and separate ordinary `VolumeCommand::Move` objects for the
batch.
An adapter must not leave a moved persistent Moria volume's placement stale:
if its consumer-owned state changes the pose, it emits that pose in the current
batch or fails the participant/tick under its own policy.

## CPU-authored activity regions and multi-fidelity ownership

Moria transports activity-region definitions only as the participant's opaque
current input.
The CPU/game layer, not Moria and not the GPU, chooses their count, shapes,
full-fidelity interiors, and halo widths.
The adapter validates its own schema and rejects invalid input through its
ordinary adapter failure.

Multi-fidelity simulation policy lives entirely in the external proof
adapter:

- body-table organization and persistence across ticks;
- deterministic region-union classification;
- full / halo / coarse list construction;
- promotion and demotion continuity;
- continued coarse motion outside every full-physics region; and
- dispatch strategy and workgroup sizing.

Moria does not name fidelity classes, own body tables, or select indirect
versus fixed dispatch.
It supplies the existing opaque input binding, stable views, proposal sinks,
placement-batch publication through ordinary move gates, and the optional
egress channel.
A conforming multi-fidelity proof adapter must still demonstrate the product
requirements—disconnected and overlapping CPU regions, one-time processing in
overlap, continuous boundary crossing, and continued coarse motion outside
every full region—using those hooks and its own oracle.
Those obligations do not expand Moria's public vocabulary.

## Opaque GPU-to-CPU egress

### Transport API

An enabled GPU participant receives binding 7 as a zero-initialized egress
header followed by `maximum_bytes`.
Moria supplies a WGSL helper that atomically reserves a byte range and
increments a record counter only while both stay within the declared maxima;
otherwise the helper sets overflow and preserves the full required counts.
The adapter must use that helper for every record it intends to deliver.
Record layout and allocation order are adapter-defined; Moria preserves the
initialized prefix byte-for-byte and does not sort, stride-check, or decode
it.

```rust
pub type BehaviorEgressReceipt = Receipt<BehaviorEgressCompleted>;

pub struct BehaviorEgressCompleted {
    pub tick: BehaviorTickId,
    pub engine: BehaviorEngineId,
    pub correlation: Correlation,
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
    Overflow { required_records: u32, required_bytes: u64, capacity_records: u32, capacity_bytes: u64 },
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

Zero records is a successful
`BehaviorEgressCompleted { record_count: 0, bytes: Box::new([]) }`.
It is distinct from every unavailable/failure variant.
At exact capacity, all initialized bytes are delivered.
When required records or bytes exceed the declared maxima or the overflow flag
is set, no prefix is delivered and the receipt fails with the exact required
and capacity values.
Silent truncation is forbidden.

The egress receipt uses
`OperationScope::BehaviorEgress { tick, engine }`.
Every transport failure maps without loss to
`OperationErrorKind::BehaviorEgress(the_exact_failure_above)`;
the outer `device_generation` repeats the generation for `DeviceLost`.

`revision_changed` on this transport error is the independently confirmed
tick publication value; it never implies that the bytes were delivered.

### Ordering and lifetime

Egress reuses the existing GPU-to-CPU handoff and staging path:

1. the adapter writes the opaque working range;
2. Moria validates the header and, on a valid nonoverflow result, copies the
   initialized prefix to a pre-reserved staging range from the ordinary
   staging pool;
3. effect validation/publication continues without waiting for the map;
4. queue completion establishes the publication terminal decision;
5. map completion, mapped-view drop, unmap, and host-byte handoff establish
   CPU egress availability using the same milestones as GPU-to-CPU handoffs.

For each adapter, Moria releases egress receipts in increasing tick order;
with one active tick this requires no unbounded reorder queue.
A ready result includes tick, participant, and request correlation, so the
owning consumer can decode it without inspecting solver state.

Working, staging, and host bytes charge the existing
`behavior_handoff_bytes`, `behavior_handoff_maps`, and generic
`staging_bytes` / `staging_maps` / host-result capacity that already serve
mixed-processor handoffs and ordinary readback.
They do not introduce dedicated egress device, staging, host, map, or receipt
pools.
Reservation still happens before any adapter runs: every enabled egress
descriptor's maxima are included in the tick's atomic admission reservation
alongside handoff and proposal maxima.
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
| 7 | read-write | opaque CPU-egress header/payload or a valid disabled header |

The unchanged v1 view, volume, cell, input, handoff, and feedback layouts are
embedded exactly.
The effect header uses ABI version 2 and adds proposal kinds:
placement batch `5` and extract components `6`.
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

`BehaviorEgressHeaderV2` is 64 bytes:

```text
0  magic:u32 = MORO
4  version:u32 = 2
8  engine:u32
12 status:u32              // Disabled=0, Enabled=1
16 record_capacity:u32
20 required_records:atomic<u32>
24 payload_capacity:u32
28 required_bytes:atomic<u32>
32 overflow:atomic<u32>
36 payload_offset:u32
40 total_bytes:u32
44 tick_low:u32
48 tick_high:u32
52 generation_low:u32
56 generation_high:u32
60 reserved:u32
```

Moria initializes counters, overflow, payload, padding, and reserved words to
zero.
Disabled requires zero capacities/counters and `total_bytes == 64`.
Enabled requires descriptor equality and
`total_bytes == align4(64 + maximum_bytes)`.
After execution, `overflow != 0`, `required_records > record_capacity`, or
`required_bytes > payload_capacity` produces the explicit overflow failure and
no payload delivery.

All count-to-byte arithmetic is checked before allocation and again on GPU.
Every effective binding range fits
`max_storage_buffer_binding_size`.
Scheduled v2 requires at least eight storage bindings in one shader stage.

## Resource limits

`ResourceLimits` and `ResourceKind` add only the extraction and placement
fields that cannot already be expressed by live/lifetime volume, brick, scar,
proposal, and handoff/staging ceilings:

| Field | Default | Hard maximum / relationship |
| --- | ---: | --- |
| `behavior_placement_updates` | 65,536 | 1,048,576; aggregate descriptor maxima fit |
| `behavior_placement_bytes` | 4 MiB | 64 MiB; `>= 64 * behavior_placement_updates` |
| `behavior_component_extraction_proposals` | 16 | 1,024 |
| `behavior_component_extraction_children` | 256 | 4,096; `live_volumes` and `volume_records` each cover initial live records plus this reserved maximum |
| `behavior_component_extraction_assignment_cells` | 262,144 | 1,048,576 |
| `behavior_component_extraction_child_bricks` | 4,096 | 65,536 |
| `behavior_component_extraction_bytes` | 32 MiB | 256 MiB; covers mapping, payload, transfer, and validation records |

Egress does not add independent resource kinds.
Its declared record/byte maxima charge the existing handoff, staging, map, and
host-result pools at registration and again at tick admission.
Tick admission reserves the complete descriptor maxima for placement updates
and component extraction before execution, including worst-case child
directory entries, scars, and ordinary move-gate versions for every placement
entry.
Telemetry reports current, high-water, waiting/rejected, last-use-delayed
bytes, overflow count, map/decode failure count, unused component-extraction
reservations, matter-conservation failures, and placement update count.

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
Its publication installs an internal empty base plus ordinary full-brick scars
for every nonempty child brick transferred from the source.
Those scars are dirty authoritative persistence input under the existing scar
machinery.
They remain pinned in retained GPU/scar capacity until a checkpoint is durable.
Before that checkpoint, device loss is `UnrecoverableDirtyState` under the
existing rule.

Checkpoint format v2 adds the provenance tag for derived-extraction children.
It does not introduce a parallel sparse derived-base chunk store or a derived
key-namespace lifecycle: child matter is the ordinary base-plus-scar path with
an empty internal base, and child keys are ordinary lifetime keys.
Later edits are ordinary full-brick scars relative to that empty base plus any
earlier scars.
Restore recreates child identity, domain, cell size, dynamic mode, placement,
provenance, and sample bytes without a consumer content source by loading the
empty base and applying scars.
The parent need not still be live.
Digest/count mismatch, missing scar chunk, or an attempt to attach a consumer
source to the child fails restore.

Placement batches persist only the resulting committed per-volume placements
and revisions.
Activity-region input, adapter body/fidelity state, coarse/full state,
handoffs, and egress bytes are never persisted by Moria.

## Required validation

The validation plan must retain all prior scheduled-adapter evidence and add:

1. one real-GPU adapter labels at least three connected pieces in one source,
   publishes at least two children, retains a source remainder, and proves
   exact coordinate/sample conservation and tolerance-bounded world-box
   continuity across one atomic extraction transaction;
2. every candidate identity/resource is reserved before adapter execution,
   binding 6 maps piece handles to final IDs, and the adapter updates its
   factory-owned body table without CPU authority-path readback;
3. cancellation, every validation sentinel, exhausted live/lifetime/page/
   brick/scar/byte pool, renderer OOM, device loss before and after the gate,
   shutdown, unused child slots, and old-reader reclamation leak no capacity;
4. checkpoint/restore and cold rematerialization reproduce child identity,
   provenance, placement, samples, occupancy, and later edits without a
   consumer source, using empty-base plus full-brick scars;
5. a proof adapter consumes CPU-authored disconnected and overlapping regions
   through opaque input, processes each body once in the overlap, preserves one
   body/volume identity across halo promotion/demotion, has no
   transform/velocity discontinuity, and continues coarse motion/destruction-
   shaped generic effects outside all regions—while Moria supplies only the
   transport and placement hooks;
6. the placement batch publishes compacted full/coarse changes without host
   body enumeration, stale Moria placement, or per-object ordinary host move
   commands, and each entry advances through the ordinary per-volume move gate
   into the tick revision vector; and
7. an adapter-owned egress layout unknown to Moria round-trips byte-exactly for
   zero records, one record, exact capacity, and multiple ticks; one-over
   capacity, malformed header, cancellation, shutdown, map/decode failure, and
   device loss produce distinct terminal results with no silent bytes or
   leaked/reused-early buffers.

The adversarial reviewer must also demonstrate that none of these APIs adds a
Moria type for physics, damage, weapons, scoring, audio, region significance,
velocity, force, health, or debris policy.
