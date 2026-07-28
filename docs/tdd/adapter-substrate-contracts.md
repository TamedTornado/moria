# Adapter Substrate Contracts

## Purpose and authority

This file specifies three generic substrate hooks required by the scheduled
behavior boundary:

1. atomic extraction of existing authoritative matter into new child volumes;
2. bounded bulk placement publication that a multi-fidelity adapter can use
   without per-object host move admission; and
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
    pub schema: [u8; 16],
    pub record_stride: u32,
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

CPU egress is disabled exactly when `maximum_records == 0`,
`maximum_bytes == 0`, `record_stride == 0`, and `schema == [0; 16]`.
When enabled, `record_stride` is a multiple of four in `4..=65_536`,
`maximum_records * record_stride == maximum_bytes`, and the product is checked
in `u64`.
The schema identifier and record bytes are opaque to Moria.

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
    pub directory_epoch: WorldDirectoryEpoch,
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

Reservation is one checked acquisition.
Moria never holds a subset while waiting for another pool.
Failure leaves the tick queued or rejects it under the behavior-tick overload
policy; no adapter executes.

Runtime IDs and stable keys are allocated on the CPU from already reserved
slots before GPU execution, but publication does not depend on a CPU readback.
`VolumeId` uses the ordinary pre-reserved generational live slot.
`VolumeKey` is UUIDv5 in the world's persistent `derived_key_namespace` over
`"moria-derived-v2" || candidate_ordinal_le`; the world owns a checked
monotonic `u64` candidate ordinal.
Fresh worlds initialize that namespace from `WorldKey`; checkpoint/restore
preserves it unchanged, including `ImportAs`, so changing the containing world
identity cannot change or reuse derived child keys.
Exact registry collisions advance the ordinal and retry at most
`volume_records + 1` times per child; exhausting that bound or the ordinal
fails identity preflight before any adapter runs.
Unpublished stable keys are reservation candidates, not lifetime records or
tombstones.
They are released without becoming observable if unused or if the proposal
fails.
Once published, a key consumes its lifetime record permanently.
Candidate ordinals are never reused even when their keys remain unpublished;
this bounded gap owns no volume/tombstone record and only advances the
persisted next-ordinal counter.

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
If no source sample remains, the same multi-volume publication installs its
retirement tombstone at `source_revision + 1`.
Each child begins at revision 1.

### Multi-volume publication (reuse, not a new directory architecture)

Ordinary single-volume matter, move, and retire commits keep the baseline
per-volume revision-gate publication model in
[state-and-storage.md](state-and-storage.md).
This amendment does **not** replace that path with a world-wide radix
directory, alternate root pools, or a new authority-index ABI for every
command.

`ExtractComponents` and `PlacementStream` are the only closed exceptions that
must install several volume-directory facts together.
They reuse the existing volume directory and the
`directory_generation` already captured by snapshots:

1. validate proposal headers, source snapshot, labels, assignments, and
   dispositions;
2. build new source and child bricks, page versions, and directory entries only
   in unreferenced reserved slots;
3. validate cell conservation, occupancy summaries, directory entries, and
   every counter/byte sentinel;
4. in a separate ordered publication dispatch, install every prepared
   volume-directory fact for that proposal (source revision or tombstone, each
   child live entry, each placement/revision advance) and advance the world's
   `directory_generation` once as the multi-volume linearization point; and
5. after queue completion, emit observations and resolve the proposal receipt.

`WorldDirectoryEpoch` is the consumer-visible token for that multi-volume
linearization: a checked nonzero `u64` equal to the published
`directory_generation`.
It is not a cross-volume gameplay clock and does not introduce a second
directory data structure.
Readers that capture a snapshot acquire `directory_generation` before resolving
volume entries (already required by the snapshot token), so they observe either
the complete pre-proposal directory or the complete post-proposal set for that
proposal.
The coordinator does not submit any consumer reader behind a candidate
multi-volume publication, append its observations, or resolve its receipt until
queue completion confirms that publication.
A gate executed in a device generation that is lost before confirmation is
therefore never semantically committed or externally observable; recovery
reconstructs the retained pre-proposal directory.
This is the same confirmed-publication boundary used by ordinary revision
gates, not a rollback of a visible commit.
Old source pages, slots, and directory entries remain pinned until every
old-generation reader and the publication submission complete.

No observable state contains duplicated, ownerless, or half-published matter.
The component-extraction proposal conflicts with every other proposal
addressing its source volume.
Two component-extraction proposals for one source in a tick are invalid.

When one tick selects several component-extraction proposals and/or participant
placement streams, Moria orders those multi-volume proposals by participant
schedule order and proposal index and publishes each with its own
`directory_generation` advance and independent receipt.
Each proposal is all-or-none, but unrelated proposals retain the approved
independent-publication rule: validation/preparation failure can omit one
proposal without discarding another under the selected participant policy.
The ordered multi-volume set is queue-confirmed as one tick submission, so
device loss before that confirmation exposes none of its candidate directory
facts.

### Failure and cleanup

- Cancellation before `Preparing` releases every candidate ID, directory
  permit, GPU range, and byte.
- Validation, assignment, conservation, or capacity sentinel failure leaves
  the pre-proposal directory current and releases unpublished IDs/resources
  after their last GPU use.
- An unused child slot is released after validation establishes the exact
  published prefix.
- Renderer allocation failure before execution publishes nothing and releases
  the complete reservation.
- Device loss before gate submission or after submission but before queue
  confirmation produces typed no-publication device loss. No consumer reader
  can have acquired the candidate multi-volume set; old-generation resources
  are quarantined and candidate IDs never become visible.
- Device loss after confirmed publication preserves the applied extraction.
  Recovery then follows the dirty-derived-content rule below and may fail with
  `UnrecoverableDirtyState`; it never rolls back to the parent-only state.
- Shutdown cancels only before the ordinary `Preparing` boundary.
  A submitted component extraction drains to confirmed publication or
  terminal loss before its resources can be reused.

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
Publication installs all valid placement entries under one multi-volume
`directory_generation` advance (one `WorldDirectoryEpoch` per stream); each
addressed volume independently advances its revision by one under that same
linearization.
The result reports the exact updated `(VolumeId, VolumeRevision)` vector and
the epoch token.
No ordinary per-volume `VolumeCommand::Move` object, host body enumeration, or
authority-path CPU readback is created.

The placement stream is the selected bounded update mechanism for a
GPU-resident adapter's persistent coarse and full-fidelity objects.
It is still linear in the number of placements actually published, which is
unavoidable for fresh Moria placements, but removes per-object host
admission, proposal headers, queue operations, and receipts.
The GPU compacts only changed placements and Moria performs one validation
pipeline and one multi-volume publication.
An adapter must not leave a moved persistent Moria volume's placement stale:
if its consumer-owned state changes the pose, it emits that pose in the current
stream or fails the participant/tick under its own policy.

## CPU-authored activity regions and multi-fidelity integration

Moria's substrate obligation for multi-fidelity simulation is deliberately
small:

1. transport CPU-authored activity-region definitions only as the participant's
   existing opaque current input (binding 5);
2. provide the placement stream above so a persistent adapter can publish many
   pose updates without host body enumeration; and
3. retain fixed maximum dispatch, with blocking P11 evidence that a
   multi-fidelity proof adapter can classify bodies, run coarse/full work, and
   compact placements under its declared dispatch/workgroup maxima.

Moria does not define region geometry, fidelity classes, body tables, halo
width, promotion/demotion policy, coarse destruction, or classification
algorithms.
Those remain adapter-owned semantics demonstrated by the proof adapter and
validation plan.

A conforming multi-fidelity GPU adapter uses one persistent adapter
registration for the whole world: geographic regions are not separate Moria
adapters or separate Moria worlds.
Region bytes arrive through current-tick consumer input; the GPU does not
choose where important regions are.
The adapter classifies each persistent body at most once against the
CPU-supplied regions (deterministic union under overlapping regions), continues
coarse simulation outside every region, and publishes changed Moria placements
through one `PlacementStream`.
Disconnected regions use the same adapter-owned table and do not require
cross-adapter migration.
A body crossing between regions remains one continuously owned adapter body and
one `VolumeId` when matter-backed.

The portable baseline keeps fixed maximum dispatch rather than exposing
indirect dispatch.
P11 measures a proof adapter that declares maxima of 16 dispatches and 8,192
workgroups for a 65,536-body / 16,384 matter-backed volume workload at empty,
1%, 50%, and 100% active lists.
Tick admission charges those declared maxima, not only the expected counts.
Failure of P11 blocks the fixed-dispatch selection and requires a later
controlled indirect-dispatch revision; implementation may not silently expose
raw `INDIRECT` buffers.

## Opaque GPU-to-CPU egress

### Transport API

An enabled GPU participant receives binding 7 as a zero-initialized egress
header followed by `maximum_bytes`.
Moria supplies a WGSL helper that atomically reserves one fixed-stride record:
the returned index is written only when it is below `maximum_records`;
otherwise the helper sets overflow and preserves the full required count.
The adapter must use that helper for every record.
Record allocation order is adapter-defined; Moria preserves the initialized
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
At exact capacity, all records are delivered.
When `required_records > maximum_records` or the overflow flag is set, no
prefix is delivered and the receipt fails with the exact required/capacity
values.
The helper uses a saturating compare-exchange loop rather than wrapping
`required_records`; an attempt beyond `u32::MAX` sets a distinct
`CounterOverflow` failure and delivers no prefix.
Silent truncation is forbidden.

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
nonoverflow result, copies exactly
`record_count * record_stride` initialized bytes to its pre-reserved staging
range.
Effect validation/publication may continue in ordered GPU passes without
waiting for the map.

Queue completion first establishes the publication terminal decision.
Mapping completion later establishes CPU egress availability.
For each adapter, Moria releases egress receipts in increasing tick order;
with one active tick this requires no unbounded reorder queue.
A ready result includes tick, participant, request correlation, and schema, so
the owning consumer can decode it without inspecting solver state.

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
zero.
Disabled requires zero stride/capacity/counters and `total_bytes == 80`.
Enabled requires descriptor equality and
`total_bytes == align4(80 + maximum_bytes)`.
After execution, `overflow != 0` or `required_records > record_capacity`
produces the explicit overflow failure and no payload delivery.

All count-to-byte arithmetic is checked before allocation and again on GPU.
Every effective binding range fits
`max_storage_buffer_binding_size`.
Scheduled v2 requires at least eight storage bindings in one shader stage.

## Resource limits

`ResourceLimits` and `ResourceKind` add only the pools required by the three
hooks:

| Field | Default | Hard maximum / relationship |
| --- | ---: | --- |
| `behavior_placement_updates` | 65,536 | 1,048,576; aggregate descriptor maxima fit |
| `behavior_placement_bytes` | 4 MiB | 64 MiB; `>= 64 * behavior_placement_updates` |
| `behavior_component_extraction_proposals` | 16 | 1,024 |
| `behavior_component_extraction_children` | 256 | 4,096; `live_volumes` and `volume_records` each cover initial live records plus this reserved maximum |
| `behavior_component_extraction_assignment_cells` | 262,144 | 1,048,576 |
| `behavior_component_extraction_child_bricks` | 4,096 | 65,536 |
| `behavior_component_extraction_bytes` | 32 MiB | 256 MiB; covers mapping, payload, transfer, and validation records |
| `behavior_egress_maps` | 16 | 256; at least enabled GPU participant count |
| `behavior_egress_receipts` | 64 | 4,096; bounds pending and retained terminal egress results |
| `behavior_egress_records` | 16,384 | 1,048,576; aggregate descriptor maxima fit |
| `behavior_egress_device_bytes` | 16 MiB desired / 1 MiB minimum | 256 MiB and adapter allocation; aggregate `align4(80 + maximum_bytes)` ranges fit |
| `behavior_egress_staging_bytes` | 16 MiB desired / 1 MiB minimum | 256 MiB and adapter allocation; aggregate payload maxima fit |
| `behavior_egress_host_bytes` | 16 MiB | 256 MiB; aggregate exact decoded deliveries fit |

These pools do not alias proposal, handoff, generic staging, command,
observation, or adapter factory-resource pools.
They also do not invent separate world-directory root/node/entry/authority
pools: multi-volume proposals charge the existing live/lifetime volume records,
page/brick/scar pools, and the placement/extraction/egress pools above.
Tick admission reserves their complete descriptor maxima before execution.
Telemetry reports current, high-water, waiting/rejected, last-use-delayed
bytes, overflow count, map/decode failure count, unused component-extraction
reservations, matter-conservation failures, placement update count, and
multi-volume publication latency.

## Persistence and rematerialization

Every child has:

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

This is substrate provenance, not a behavior label.
The child does not have a consumer `BaseContentSource`.
Its publication creates an internal derived-content base consisting of every
nonempty child brick plus the provenance record.
Those bricks are dirty authoritative persistence input.
They remain pinned in retained GPU/scar capacity until a checkpoint is durable.
Before that checkpoint, device loss is `UnrecoverableDirtyState` under the
existing rule.

Checkpoint format v2 adds the provenance tag and stores the complete sparse
derived base for each child.
The manifest also stores the world's next derived candidate ordinal; restore
requires it to be greater than every saved child's provenance ordinal and
continues from it without key reuse.
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
   continuity across one multi-volume `directory_generation` publication;
2. every candidate identity/resource is reserved before adapter execution,
   binding 6 maps piece handles to final IDs, and the adapter updates its
   factory-owned body table without CPU authority-path readback;
3. cancellation, every validation sentinel, exhausted live/lifetime/page/
   brick/scar/byte pool, renderer OOM, device loss before and after the gate,
   shutdown, unused child slots, and old-reader reclamation leak no capacity;
4. checkpoint/restore and cold rematerialization reproduce child identity,
   provenance, placement, samples, occupancy, and later edits without a
   consumer source;
5. a proof adapter consumes CPU-authored disconnected and overlapping regions
   through opaque input, processes each body once in the overlap, preserves one
   body/volume identity across region boundaries, continues coarse
   motion/destruction-shaped generic effects outside all regions, and never
   freezes or drops objects solely because they left every region;
6. the placement stream publishes compacted full/coarse changes without host
   body enumeration, stale Moria placement, duplicate volume entry, or
   per-object ordinary move proposals;
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
