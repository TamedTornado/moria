# Adapter Substrate Contracts

## Purpose and authority

This file specifies three generic transports required by the scheduled
behavior boundary:

1. atomic extraction of existing authoritative matter into new child volumes;
2. bounded bulk placement publication for one persistent, multi-fidelity
   adapter simulation; and
3. optional bounded opaque GPU-to-CPU adapter egress.

They trace to the approved design's dynamic-volume, controlled-effect,
GPU-oriented behavior, persistence, and explicit-failure requirements.
They do not add physics, damage, weapons, fracture policy, body state,
significance thresholds, activity-region policy, scoring, audio, or gameplay
events to Moria.
An adapter may give the byte and proposal records those meanings; Moria sees
only bounded regions, existing samples, placements, identities, opaque bytes,
and publication outcomes.

These contracts extend the baseline mechanisms already selected for ordinary
matter, placement, directory generation, scars, staging readback, and the
six-binding Scheduled GPU ABI.
They do not replace ordinary single-volume commits or snapshots with a public
copy-on-write world-directory protocol, and they do not add a second group-0
binding set.

Scheduled GPU ABI v2 in this file supersedes the proposal-kind closed set of
Scheduled ABI v1 in T29, T30, T32, and T33 only by adding two GPU proposal
kinds and two optional sections inside existing bindings 0 and 1.
Unchanged v1 record layouts retain their `V1` type names inside v2.
There is no runtime v1/v2 negotiation in the initial implementation:
configuration accepts only scheduled ABI version 2.
Group 0 still has exactly six storage bindings.

## Public capability declarations

`BehaviorEngineDescriptor` gains the following fields:

```rust
pub struct BehaviorEngineDescriptor {
    // Existing fields remain.
    pub maximum_placement_updates: u32,
    pub maximum_component_extraction_proposals: u32,
    pub maximum_component_extraction_children: u32,
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
For a GPU adapter, the two component-extraction fields are either both zero or
both nonzero.
`maximum_component_extraction_children` is the aggregate candidate-child
identity capacity across all of that adapter's component-extraction proposals
in one tick.
`maximum_placement_updates` is the aggregate entry capacity of that adapter's
one placement-stream proposal per tick (zero disables the kind).

Extraction proposals, assignment cells, child bricks, transfer/payload bytes,
and directory-entry effects charge the existing
`maximum_proposals`, `maximum_proposal_bytes`, `maximum_effect_cells`,
`maximum_effect_bricks`, and `maximum_directory_effects` fields.
Placement-stream entries charge `maximum_proposals` (one proposal),
`maximum_proposal_bytes` (`64 * entry_count` payload), and
`maximum_directory_effects` (one directory effect per updated volume).
Registration fails when the checked products do not fit those descriptor
maxima and the world resource pools.

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
  extraction (existing proposal pool);
- payload, assignment-cell, child-brick, and directory-effect capacity under
  the existing per-tick proposal/payload/cell/brick/directory-effect pools;
- one live directory entry permit, permanent lifetime record, `VolumeId`,
  consumer-visible `VolumeKey`, revision slot, observation fact, presentation
  marker, and receipt child record for every possible child (the only new
  identity headroom beyond ordinary create);
- source residual and child page keys, page versions, detailed brick slots,
  full-brick scar records, occupancy summaries, and validation diagnostics
  under the existing page/brick/scar pools;
- the read-only child-reservation section of binding 0 and the extraction
  payload range of binding 1; and
- every byte of submission, feedback, and cleanup bookkeeping needed by the
  maximum operation under existing tick bookkeeping pools.

Reservation is one checked acquisition.
Moria never holds a subset while waiting for another pool.
Failure leaves the tick queued or rejects it under the behavior-tick overload
policy; no adapter executes.

Runtime IDs and stable keys are allocated on the CPU from already reserved
slots before GPU execution, but publication does not depend on a CPU readback.
`VolumeId` uses the ordinary pre-reserved generational live slot.

`VolumeKey` is derived, not allocated from a world-global ordinal namespace:

```text
UUIDv5(
  parent_volume_key as namespace,
  "moria-extract-v1"
    || source_revision_le_u64
    || proposal_slot_le_u32
    || piece_handle_le_u32
)
```

The parent key is the source volume's stable key at the pinned revision.
The proposal slot is the participant-local extraction-proposal index reserved
for that tick.
Piece handles are proposal-local and nonzero.
Exact registry collisions with an already live or tombstoned key fail identity
preflight before any adapter runs; there is no persisted candidate ordinal and
no world-owned derived-key namespace counter.
Unpublished stable keys are reservation candidates, not lifetime records or
tombstones.
They are released without becoming observable if unused or if the proposal
fails.
Once published, a key consumes its lifetime record permanently.

Binding 0's optional reservation section contains a dense mapping for each
`(extraction_proposal_slot, ComponentPieceHandle)`:

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
adapter cannot write the reservation section.
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
If no source sample remains, the same generation-swap publication installs its
retirement tombstone at `source_revision + 1`.
Each child begins at revision 1.

### Preparation and atomic publication

Publication reuses the baseline world-directory generation mechanism already
used for offline page-table rebuild and snapshot capture
(`directory_generation` on `SnapshotToken`), extended only for this closed
operation:

1. Before adapter execution, Moria installs prebuilt child directory entries
   that are invisible to readers of the current directory generation: they
   hold reserved IDs/keys/placement slots but are not members of the live
   generation set and cannot be resolved by queries, collision, presentation,
   or ordinary commands.
2. Portable WGSL phases, as ordered dispatches, validate proposal headers,
   source snapshot, labels, assignments, and dispositions; mark each assigned
   source cell and stable-compact assignments by piece; compute per-piece
   counts, bounds, and exact sample digests; and build new source residual and
   child bricks in unreferenced reserved slots.
3. Child matter is stored as complete full-brick scars against a canonical
   empty internal base (no consumer `BaseContentSource`). Empty bricks remain
   implicit empty against that base; every nonempty transferred brick is one
   ordinary 512-sample scar.
4. Validate cell conservation, page/slot generations, occupancy summaries,
   directory entry fields, and every counter/byte sentinel.
5. In a separate ordered publication dispatch after payload construction on
   the same queue, compare the source volume's still-pinned revision gate and
   the world's current directory generation, then perform one extraction-only
   directory generation swap that atomically:
   - publishes the source residual (or retirement tombstone);
   - promotes every `PublishChild` entry from invisible prebuilt to live under
     the new generation at revision 1 with its prepared scars and placement;
   - leaves unused/failed candidate entries unpublished.
6. After queue completion, emit observations and resolve the
   component-extraction receipt.

Construction dispatches never mutate a generation visible to readers.
The generation swap is the linearization point for the whole extraction.
Readers that acquired the old directory generation continue to resolve the
pre-extraction source only; readers that acquire the new generation see the
complete residual-or-tombstone plus every published child.
The coordinator does not submit any consumer reader behind a candidate
generation, append its observations, or resolve its receipt until queue
completion confirms that swap's submission.
A swap executed in a device generation that is lost before confirmation is
therefore never semantically committed or externally observable; recovery
reconstructs the retained old directory generation.
This is the same confirmed-publication boundary used by ordinary revision
gates, not a rollback of a visible commit.
Old page versions, slots, and the pre-swap directory generation remain pinned
until every old-generation reader and the publication submission complete.

No observable state contains duplicated, ownerless, or half-published matter.
The component-extraction proposal conflicts with every other proposal
addressing its source volume.
Two component-extraction proposals for one source in a tick are invalid.
Ordinary single-volume fills, patches, moves, and retires continue to use
their existing per-volume revision/placement gates and do not allocate
alternate world-directory roots.

### Failure and cleanup

- Cancellation before `Preparing` releases every candidate ID, directory
  permit, GPU range, and byte.
- Validation, assignment, conservation, or capacity sentinel failure leaves
  the old directory generation current and releases unpublished IDs/resources
  after their last GPU use.
- An unused child slot is released after validation establishes the exact
  published prefix.
- Renderer allocation failure before execution publishes nothing and releases
  the complete reservation.
- Device loss before swap submission or after submission but before queue
  confirmation produces typed no-publication device loss. No consumer reader
  can have acquired the candidate generation; old-generation resources are
  quarantined and candidate IDs never become visible.
- Device loss after confirmed publication preserves the applied extraction.
  Recovery then follows the dirty-scar rule below and may fail with
  `UnrecoverableDirtyState`; it never rolls back to the parent-only state.
- Shutdown cancels only before the ordinary `Preparing` boundary.
  A submitted component extraction drains to confirmed publication or
  terminal loss before its resources can be reused.

## Bulk placement publication

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
stream before any volume publishes.
Each GPU participant may emit at most one placement-stream proposal per tick;
its descriptor maximum is the aggregate entry capacity of that proposal.

Moria reserves the descriptor's complete maximum update records, proposal
payload bytes, one directory-effect slot per maximum entry, revision values,
observation facts, presentation markers, and outcomes before the adapter runs.
After batch validation, publication reuses the existing per-volume
revision/placement gates already used by ordinary `Move`: each valid entry
prepares one directory placement version and compare-exchanges that volume's
gate from the expected revision to `expected + 1`.
There is no alternate world-directory root and no requirement that unrelated
placement updates share one atomic multi-volume root.
The stream proposal is all-or-none for validation: if any entry is invalid, no
entry publishes.
On a valid stream, each volume retains the product's independent-publication
rule at its own gate, exactly as when several ordinary move proposals address
different volumes in one tick.
The result reports the exact updated `(VolumeId, VolumeRevision)` vector.
No ordinary per-volume host `VolumeCommand::Move` object, host body
enumeration, or authority-path CPU readback is created.

The placement stream is the selected bounded update mechanism for a
GPU-resident adapter's persistent coarse and full-fidelity objects.
It is still linear in the number of placements actually published, which is
unavoidable for fresh Moria placements, but removes per-object host
admission, proposal headers, queue operations, and receipts.
An adapter must not leave a moved persistent Moria volume's placement stale:
if its consumer-owned state changes the pose, it emits that pose in the current
stream or fails the participant/tick under its own policy.

When one tick selects ordinary single-volume proposals together with
component-extraction and/or placement streams, Moria orders work by
participant schedule order and proposal index.
Ordinary and placement-stream volumes publish through their per-volume gates.
Each component extraction publishes through its own directory generation
swap.
Device loss before queue confirmation of a tick submission exposes none of
that submission's candidate gates or generation swaps.
Every proposal receipt reports its own outcome after confirmation.

## CPU-authored activity regions and multi-fidelity simulation

Moria transports activity-region definitions only as the participant's opaque
current input.
The CPU/game layer, not Moria and not the GPU, chooses their count, shapes,
full-fidelity interiors, and halo widths.
The adapter validates its own schema and rejects invalid input through its
ordinary adapter failure.

A conforming multi-fidelity GPU adapter uses one persistent adapter
registration and one consumer-owned body table for the whole world.
Geographic regions are not separate Moria adapters or separate Moria worlds.
Each persistent body has one stable adapter-owned record and, when
matter-backed, one `VolumeId`.

Substrate obligations stop there:

- opaque current input already carries the CPU-authored regions;
- the placement stream and component extraction already publish the resulting
  poses and fracture outcomes;
- Moria does not define Full/Halo/Coarse classes, region union operators,
  promotion/demotion copies, or a fixed dispatch schedule as public
  architecture.

Adapter-owned classification, compaction, transition, and coarse continuation
are validated by the proof fixture described in
[validation.md](validation.md).
That fixture must demonstrate disconnected and overlapping CPU-defined
regions, one-time processing in their overlap, continuous boundary crossing
without transform/velocity discontinuity under the adapter's own fields,
continued coarse motion outside every full-physics region, and that fixed
maximum dispatch declared by the proof adapter remains within the existing
scheduled dispatch/workgroup limits under empty, sparse, half, and full active
lists.
Those details are proof-adapter evidence, not Moria vocabulary or a second
publication path.

Bodies outside every region continue through the adapter-owned coarse path.
Coarse motion and any remote destruction/debris policy remain adapter
semantics.
They may produce placement streams, component extraction, ordinary material
effects, handoffs, or opaque CPU egress.
Moria does not choose which outcome applies.

## Opaque GPU-to-CPU egress

### Transport API

An enabled GPU participant receives an optional egress section at the end of
binding 1 (the existing effect/output binding): a zero-initialized egress
header followed by `maximum_bytes`.
When egress is disabled, the section is a valid 80-byte disabled header and no
payload range is reserved.
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
`record_count * record_stride` initialized bytes through the existing
bounded staging pool and map machinery already used for outcome metadata and
checkpoint scar readback.
Effect validation/publication may continue in ordered GPU passes without
waiting for the map.

Queue completion first establishes the publication terminal decision.
Mapping completion later establishes CPU egress availability.
For each adapter, Moria releases egress receipts in increasing tick order;
with one active tick this requires no unbounded reorder queue.
A ready result includes tick, participant, request correlation, and schema, so
the owning consumer can decode it without inspecting solver state.

Working device bytes for the egress section are charged to the existing
proposal/output binding range while the adapter runs and are reusable only
after the ordered copy's last GPU use.
Staging maps charge the existing `staging_bytes` / map-slot pools and are
reusable only after successful/failed map completion, mapped-view drop, and
unmap.
Decoded host bytes are copied into one exact boxed slice.
The only capacity added beyond ordinary staging/map/receipt machinery is a
bounded retained-host-result pool that holds those decoded boxes until the
egress receipt is dropped or an undeliverable terminal result is discarded.
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

Group 0 still has exactly six storage bindings:

| Binding | Access | Contents |
| ---: | --- | --- |
| 0 | read-only | participant stable view, then optional child-reservation section |
| 1 | read-write | effect header, proposal records, payload bytes, then optional opaque egress section |
| 2 | read-only | incoming handoffs |
| 3 | read-write | outgoing handoffs |
| 4 | read-only | prior feedback |
| 5 | read-only | current consumer input |

The unchanged v1 view, volume, cell, input, handoff, and feedback layouts are
embedded exactly.
The effect header uses ABI version 2 and adds proposal kinds:
placement stream `5` and extract components `6`.
An ordinary create record remains unrepresentable.

When component extraction is enabled for the participant, binding 0 appends
after the cell records:

```text
ComponentReservationHeaderV2 (64 bytes)
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
The empty/disabled table is a valid header with zero records and
`total_bytes == 64`.
View `total_bytes` includes the section.

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

When CPU egress is enabled, binding 1 ends with:

```text
BehaviorEgressHeaderV2 (80 bytes)
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
Effect-header `payload_capacity` covers proposal payloads only; the egress
section is a distinct trailing range inside the same binding allocation and is
accounted in the checked binding total against
`max_storage_buffer_binding_size`.

All count-to-byte arithmetic is checked before allocation and again on GPU.
Every effective binding range fits
`max_storage_buffer_binding_size`.
Scheduled v2 still requires at least six storage bindings in one shader stage.

## Resource limits

Extraction and placement charge existing pools wherever those pools already
bound the same resource class:

| Charge | Existing limit / pool |
| --- | --- |
| Extraction and placement-stream proposals | `behavior_proposals` / descriptor `maximum_proposals` |
| Assignment records, placement entries, extraction headers | `behavior_proposal_bytes` / descriptor `maximum_proposal_bytes` |
| Assigned source cells | `behavior_effect_cells` / descriptor `maximum_effect_cells` |
| Source residual and child bricks | `behavior_effect_bricks` / descriptor `maximum_effect_bricks` |
| Source residual/tombstone and each child or placement directory update | `behavior_directory_effects` / descriptor `maximum_directory_effects` |
| Page versions, detailed slots, scars | existing page/brick/scar pools |
| Egress device copy, staging map, and map slots | existing staging/map pools used by outcome readback |
| Egress receipt object | existing receipt machinery plus retained-host-result capacity below |

Genuinely distinct added fields:

| Field | Default | Hard maximum / relationship |
| --- | ---: | --- |
| `behavior_component_extraction_children` | 256 | 4,096; aggregate descriptor child maxima fit; `live_volumes` and `volume_records` each cover initial live records plus this reserved maximum |
| `behavior_egress_host_bytes` | 16 MiB | 256 MiB; aggregate exact decoded egress deliveries retained until result drop |

These two pools do not alias proposal, handoff, generic staging, command, or
adapter factory-resource pools for the identity and retained-host roles they
add.
Tick admission reserves their complete descriptor maxima before execution
together with the existing proposal/payload/cell/brick/directory-effect and
staging/map charges those descriptors imply.
Telemetry reports current, high-water, waiting/rejected, unused
component-extraction child reservations, matter-conservation failures,
placement update count, egress retained-host bytes, and map/decode failure
count using the existing telemetry surfaces plus the two new kinds.

## Persistence and rematerialization

Every published child has minimal substrate provenance:

```text
DerivedExtractionProvenanceV2 {
    parent_volume_key,
    parent_revision,          // pinned revision before extraction
    proposal_slot,
    piece_handle,
    sample_count,
    sample_digest,
}
```

This is substrate provenance, not a behavior label.
The child does not have a consumer `BaseContentSource`.
Its publication creates matter as complete full-brick scars against the
canonical empty internal base, plus the provenance record.
Those scars are dirty authoritative persistence input under the ordinary scar
rules.
They remain pinned in retained GPU/scar capacity until a checkpoint is durable.
Before that checkpoint, device loss is `UnrecoverableDirtyState` under the
existing rule.

Checkpoint format v2 adds the provenance tag on the volume record.
It does **not** introduce a second complete sparse derived-base chunk class,
a world `derived_key_namespace`, or a next-candidate ordinal.
Child sample truth is the ordinary scar section already used for edited
external volumes; empty bricks need no record because the internal base is
canonical empty.
Restore recreates child identity by re-deriving the `VolumeKey` from
provenance (`parent_volume_key`, `parent_revision`, `proposal_slot`,
`piece_handle`), then installs domain, cell size, dynamic mode, placement,
provenance, and scar bytes without a consumer content source.
The parent need not still be live.
Cold rematerialization loads scars first against the empty internal base.
Digest/count mismatch, missing scar chunk, key/provenance mismatch, or an
attempt to attach a consumer source to the child fails restore.

Placement streams persist only the resulting committed per-volume placements
and revisions.
Activity-region input, adapter body/fidelity state, coarse/full state,
handoffs, and egress bytes are never persisted by Moria.

## Required validation

The validation plan must retain all prior scheduled-adapter evidence and add:

1. one real-GPU adapter labels at least three connected pieces in one source,
   publishes at least two children, retains a source remainder, and proves
   exact coordinate/sample conservation and tolerance-bounded world-box
   continuity across one extraction-only directory generation swap;
2. every candidate identity/resource is reserved before adapter execution,
   the binding-0 reservation section maps piece handles to final IDs, and the
   adapter updates its factory-owned body table without CPU authority-path
   readback;
3. cancellation, every validation sentinel, exhausted live/lifetime/page/
   brick/scar/byte pool, renderer OOM, device loss before and after the swap,
   shutdown, unused child slots, and old-reader reclamation leak no capacity;
4. checkpoint/restore and cold rematerialization reproduce child identity,
   provenance, placement, samples, occupancy, and later edits without a
   consumer source and without a global derived-key ordinal;
5. a proof adapter consumes CPU-authored disconnected and overlapping regions
   as opaque input, processes each body once in the overlap, preserves one
   body/volume identity across its own halo promotion/demotion, has no
   transform/velocity discontinuity in adapter-owned fields, and continues
   coarse motion/destruction-shaped generic effects outside all regions;
6. the placement stream publishes compacted full/coarse changes through
   existing per-volume placement gates without host body enumeration, stale
   Moria placement, duplicate volume entry, or per-object ordinary move
   proposals;
7. the proof adapter's declared fixed maximum dispatch stays within scheduled
   dispatch/workgroup limits at empty, sparse, half, and full active lists and
   passes P11 on every claimed backend family; and
8. an adapter-owned egress struct unknown to Moria round-trips byte-exactly for
   zero records, one record, exact capacity, and multiple ticks; one-over
   capacity, malformed header, cancellation, shutdown, map/decode failure, and
   device loss produce distinct terminal results with no silent bytes or
   leaked/reused-early buffers.

The adversarial reviewer must also demonstrate that none of these APIs adds a
Moria type for physics, damage, weapons, scoring, audio, region significance,
velocity, force, health, or debris policy.
