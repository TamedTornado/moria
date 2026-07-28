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

Scheduled GPU ABI v2 in this file supersedes the proposal-kind set in T29,
T30, T32, and T33 by adding two closed multi-volume substrate effects while
retaining the six group-0 storage bindings of Scheduled ABI v1.
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
When enabled, the value is a multiple of four in `4..=16_777_216` and must fit
the participant's handoff and staging budgets at registration.
The byte contents are opaque to Moria; any record schema lives inside the
adapter-owned payload.

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
- one fracture-scoped prepared directory delta large enough for the source
  entry or tombstone plus every pre-reserved child entry;
- source and child page keys, page versions, detailed brick slots, scar
  records, occupancy summaries, transfer records, assignment records, and
  validation diagnostics;
- component-extraction payload bytes; and
- every byte of submission, feedback, persistence provenance, and cleanup
  bookkeeping needed by the maximum operation.

Reservation is one checked acquisition.
Moria never holds a subset while waiting for another pool.
Failure leaves the tick queued or rejects it under the behavior-tick overload
policy; no adapter executes.

Runtime IDs are allocated on the CPU from already reserved generational live
slots before GPU execution so the publication pass can install them without a
CPU readback on the authority path.
Those IDs are provisional until terminal feedback: they are the children's
final IDs on success and are permanently invalid on failure or unused
disposition.
No Moria view or effect can resolve a provisional candidate as a live volume.

Stable keys are deterministic UUIDv5 values in the world's persistent
`derived_key_namespace` over

```text
"moria-derived-v2"
  || source_volume_key_bytes
  || source_revision_le
  || proposal_slot_le
  || piece_handle_le
```

The namespace is initialized from `WorldKey` for a fresh world and is preserved
unchanged across checkpoint/restore, including `ImportAs`, so changing the
containing world identity cannot change or reuse derived child keys.
There is no world-wide candidate ordinal and no collision-retry allocator:
the source key, pinned source revision, proposal slot, and piece handle already
distinguish every child of a fracture.
An exact collision with an existing live or lifetime key fails identity
preflight for that proposal before any adapter runs; the checked product of
pre-reserved children and the lifetime table makes that failure explicit.

Unpublished keys and IDs are reservation candidates, not lifetime records or
tombstones.
They are released without becoming observable if unused or if the proposal
fails.
Once published, a key consumes its lifetime record permanently.

The adapter does **not** receive final `VolumeId` values before dispatch.
Proposal-local `ComponentPieceHandle` values remain the only in-dispatch child
labels.
After queue-confirmed publication (or terminal rejection), Moria writes the
piece-handle-to-`VolumeId` / key / outcome mapping into the participant's
ordinary prior-feedback records (binding 4 on the next ordered dispatch).
That mapping is GPU-visible without an authority-path CPU readback and is the
sole substrate path for binding factory-owned body state to published children.
Provisional IDs remain unusable for any Moria view until that feedback reports
`Published`.

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
If no source sample remains, the same fracture delta publishes its retirement
tombstone at `source_revision + 1`.
Each child begins at revision 1.

### Fracture-scoped prepared directory delta

Atomic old-or-new visibility for one `ExtractComponents` publication does **not**
require a new world-wide directory authority model, immutable radix topology,
or a directory-epoch gate on every ordinary query, collision traversal,
scheduled view, checkpoint, presentation job, or single-volume commit.

The baseline already provides:

- `SnapshotToken.directory_generation` captured before volume gates;
- offline prepare of unreferenced page/brick versions;
- per-volume revision and placement publication gates; and
- coordinator-withheld consumer readers, observations, and receipts until a
  publication submission is queue-confirmed.

`ExtractComponents` adds only a **fracture-scoped prepared directory delta**
that gates the source and its pre-reserved children:

```text
FractureDirectoryDelta {
    expected_directory_generation: u64,
    expected_source_revision: VolumeRevision,
    source: PreparedSourceEntry,          // live remainder or tombstone
    children: [PreparedChildEntry; N],    // pre-reserved slots only
    transfer_and_validation_sentinels...
}
```

Portable WGSL phases are ordered dispatches:

1. validate proposal headers, source snapshot, labels, assignments, and
   dispositions;
2. mark each assigned source cell and stable-compact assignments by piece;
3. compute per-piece counts, bounds, and exact sample digests;
4. build new source and child bricks in unreferenced reserved slots;
5. validate cell conservation, page/slot generations, occupancy summaries,
   directory delta entries, and every counter/byte sentinel;
6. fill the unreferenced `FractureDirectoryDelta` with the source entry or
   tombstone and every child entry (IDs, keys, domains, placements, content
   roots already selected);
7. in a separate ordered publication dispatch, verify
   `directory_generation` and source revision, install every prepared child
   directory slot, publish the source revision/placement or tombstone through
   the ordinary per-volume gates, then advance `directory_generation` by one
   checked step; and
8. after queue completion, emit observations, resolve the
   component-extraction receipt, and write piece-handle identity outcomes into
   prior feedback.

The construction dispatches never mutate a directory entry visible to readers.
The publication pass is ordered after payload construction on the same queue.
Readers acquire `directory_generation` before resolving volume entries, so a
snapshot taken under the pre-publication generation cannot observe any child
or source remainder from the delta, and a snapshot taken under the
post-publication generation observes the complete source/children set.
The coordinator does not submit any consumer reader behind the fracture
publication, append its observations, or resolve its receipt until queue
completion confirms that submission.
A publication executed in a device generation that is lost before confirmation
is therefore never semantically committed or externally observable; recovery
reconstructs the retained pre-delta directory.
This is the same confirmed-publication boundary used by ordinary revision
gates, not a rollback of a visible commit.
Old source pages, slots, and the previous generation remain pinned until every
old-generation reader and the publication submission complete.

No observable state contains duplicated, ownerless, or half-published matter.
The component-extraction proposal conflicts with every other proposal
addressing its source volume.
Two component-extraction proposals for one source in a tick are invalid.
Unrelated volumes keep the approved independent-publication rule (T4).

Ordinary single-volume commits continue to use the existing per-volume
revision/placement gates and do not allocate fracture deltas, directory roots,
or epoch counters.

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
- Device loss before gate submission or after submission but before queue
  confirmation produces typed no-publication device loss. No consumer reader
  can have acquired the candidate generation; old-generation resources are
  quarantined and candidate IDs never become visible.
- Device loss after confirmed publication preserves the applied extraction.
  Recovery then follows the dirty-derived-content rule below and may fail with
  `UnrecoverableDirtyState`; it never rolls back to the parent-only state.
- Shutdown cancels only before the ordinary `Preparing` boundary.
  A submitted component extraction drains to confirmed publication or
  terminal loss before its resources can be reused.

## Placement stream

### Boundary

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
stream **before any publication**.
Each GPU participant may emit at most one placement-stream proposal per tick;
its descriptor maximum is the aggregate entry capacity of that proposal.

Moria reserves the descriptor's complete maximum update records, revision
values, observation facts, presentation markers, outcomes, and GPU scratch
before the adapter runs.
No alternate world-directory root and no cross-volume directory epoch are
allocated for the stream.

### Publication and freshness

After whole-stream validation, Moria publishes each accepted entry through the
**existing per-volume placement/revision gates** already used by ordinary
moves.
Each addressed volume independently advances its revision by one.
The result reports the exact updated `(VolumeId, VolumeRevision)` vector.
No ordinary per-volume `VolumeCommand::Move` object, host body enumeration, or
authority-path CPU readback is created.

This matches T4: matter, placement, creation readiness, and retirement facts
use one monotonic revision stream per volume; a world observation sequence
orders delivery but is not a cross-volume truth revision.
The cited multi-fidelity requirement asks for **bounded, scalable placement
updates** that keep coarse-object Moria placements fresh, not for all-or-none
cross-volume atomicity of an entire stream.
A GPU-compacted array validated once and published through those per-volume
gates is therefore the selected mechanism.

The stream remains linear in the number of placements actually published,
which is unavoidable for fresh Moria placements, but removes per-object host
admission, proposal headers, queue operations, and receipts.
An adapter must not leave a moved persistent Moria volume's placement stale:
if its consumer-owned state changes the pose, it emits that pose in the current
stream or fails the participant/tick under its own policy.

When one tick selects several component-extraction proposals and/or participant
placement streams, Moria orders those proposals by participant schedule order
and proposal index.
Each proposal is all-or-none at validation, but unrelated proposals retain
independent publication under the selected participant policy: validation or
preparation failure can omit one proposal without discarding another.
Device loss before queue confirmation of a proposal's publication submission
exposes none of that proposal's candidate state.
Every proposal receipt reports its own outcome after confirmation.

## CPU-authored activity regions and persistent multi-fidelity simulation

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

The selected proof algorithm is:

1. upload the CPU-supplied region bytes through binding 5;
2. dispatch one invocation per persistent body;
3. test that body against every declared region in stable input order and
   reduce the matches to one closed adapter-owned class
   `Full | Halo | Coarse`, with proof-adapter priority
   `any Full > any Halo > Coarse`;
4. mark and hierarchical-scan the mutually exclusive predicates;
5. scatter each body index exactly once into one compact list;
6. run the adapter's full, transition, or coarse kernels against those lists;
   and
7. compact changed Moria placements into one `PlacementStream`.

The class names and region schema belong to the proof adapter, not Moria's
public vocabulary.
Overlapping regions form a deterministic union because classification writes
one result per body before compaction.
A body matching two regions is never emitted twice.
The participant's Moria cell records are exported once independently of region
count; proof kernels visit a matter-backed body's cell range only from that
body's one compact-list entry.
Disconnected regions use the same table and passes.

The halo is processed by the adapter's transition path.
Promotion initializes any full-fidelity representation from the same
persistent transform and velocity fields used by the coarse representation;
demotion writes the last full-fidelity state back to those same fields.
The proof adapter performs these transition copies without numeric conversion
and asserts bit-identical transform/velocity fields immediately before and
after the transition step; later integration is a separate adapter operation.
The body record and `VolumeId` never migrate or change because of geography.
Consequently a crossing cannot introduce a transform or velocity jump unless
the adapter itself violates its validated proof oracle.

Bodies outside every region continue through the adapter-owned coarse pass.
Coarse motion and any remote destruction/debris policy remain adapter
semantics.
They may produce placement streams, component extraction, ordinary material
effects, handoffs, or opaque CPU egress.
Moria does not choose which outcome applies.

### Dispatch bound after compaction

V2 retains fixed maximum dispatch rather than exposing indirect dispatch.
For the selected proof capacity of 65,536 persistent bodies and workgroup width
128, one maximum-list pass dispatches 512 workgroups.
The portable proof adapter uses exactly:

1. four class-compaction dispatches: local scan/tile totals (`512`
   workgroups), scan three classes' level-1 totals (`12`), scan level 2 (`3`),
   then add offsets/scatter (`512`);
2. three fixed simulation dispatches, one each for the compact full, halo, and
   coarse lists (`3 * 512`); and
3. four changed-placement compaction dispatches: local scan/tile totals
   (`512`), level 1 (`4`), level 2 (`1`), and scatter (`512`).

That is exactly 11 adapter dispatches and at most 3,604 workgroups per tick.
The proof descriptor declares maxima of 16 dispatches and 8,192 workgroups;
tick admission charges those declared maxima, not only the expected counts.
Every pass receives its compacted logical count and guards
`global_invocation_id < count`.
Inactive lanes still participate in any workgroup barrier.

Those descriptor maxima fit the default scheduled limits of 256 dispatches and
1,048,576 workgroups with substantial headroom.
P11 measures the fixed-overdispatch cost at empty, 1%, 50%, and 100% active
lists.
Failure of P11 blocks the fixed-dispatch selection and requires a later
controlled indirect-dispatch revision; implementation may not silently expose
raw `INDIRECT` buffers.

## Opaque GPU-to-CPU egress

### Transport API

An enabled GPU participant reuses the existing bounded **outgoing-handoff**
buffer and the existing **GPU-to-CPU staging/map lifecycle** already used for
processor handoffs.
No additional group-0 binding, fixed-stride record ABI, or independent
egress map/receipt/record/device/staging/host pool family is introduced.

When `maximum_cpu_egress_bytes > 0`, tick admission reserves one terminal
consumer egress edge for that participant from `behavior_handoff_bytes` and
`behavior_handoff_maps`, plus the ordinary staging map capacity needed to
deliver an exact host copy.
The edge is not a peer-engine handoff: its only consumer is the owning host
receipt path.
Moria initializes the participant's outgoing handoff table with that terminal
edge's capacity; the adapter writes an initialized opaque prefix using the
same handoff header/`written_bytes` rules already used for GPU-to-CPU
successor handoffs.
Moria does not sort, decode, or interpret the payload bytes.

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
    Overflow { required_bytes: u64, capacity: u64 },
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

Zero bytes is a successful
`BehaviorEgressCompleted { bytes: Box::new([]) }`.
It is distinct from every unavailable/failure variant.
At exact capacity, the complete initialized prefix is delivered.
When `written_bytes > maximum_cpu_egress_bytes` or the handoff overflow /
capacity sentinel is set, no prefix is delivered and the receipt fails with
the exact required/capacity values.
Silent truncation is forbidden.

The egress receipt uses
`OperationScope::BehaviorEgress { tick, engine }`.
Every transport failure maps without loss to
`OperationErrorKind::BehaviorEgress(the_exact_failure_above)`;
the outer `device_generation` repeats the generation for `DeviceLost`.

`revision_changed` on this transport error is the independently confirmed
tick publication value; it never implies that the bytes were delivered.

### Ordering and lifetime

The GPU pass writes effects and the terminal egress edge in adapter order.
After the adapter dispatch, Moria validates the handoff header and, on a valid
nonoverflow result, copies exactly `written_bytes` initialized bytes through
the existing GPU-to-CPU handoff staging path.
Effect validation/publication may continue in ordered GPU passes without
waiting for the map.

Queue completion first establishes the publication terminal decision.
Mapping completion later establishes CPU egress availability.
For each adapter, Moria releases egress receipts in increasing tick order;
with one active tick this requires no unbounded reorder queue.
A ready result includes tick, participant, and request correlation, so the
owning consumer can decode it without inspecting solver state.

Working handoff ranges are reusable only after their copy's last GPU use.
Staging ranges are reusable only after successful/failed map completion,
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
GPU-to-GPU consumers continue to use ordered peer handoffs or adapter-owned
factory buffers.
Moria never routes a GPU peer handoff through this CPU channel; the terminal
egress edge is a host delivery, not a successor adapter input.

No ordering, overflow, lifecycle, or publication-independence requirement
above needs a separate binding-7 schema/count ABI or independent egress pool
family: those obligations are already met by the handoff header, the
GPU-to-CPU staging/map path, and a host `Receipt<Box<[u8]>>`.

## Scheduled GPU ABI v2

Group 0 retains exactly **six** storage bindings:

| Binding | Access | Contents |
| ---: | --- | --- |
| 0 | read-only | participant stable view |
| 1 | read-write | effect header, proposal records, placement/component-extraction payload |
| 2 | read-only | incoming handoffs |
| 3 | read-write | outgoing handoffs, including the optional terminal consumer-egress edge |
| 4 | read-only | prior feedback (includes piece-handle → child identity outcomes) |
| 5 | read-only | current consumer input |

The unchanged v1 view, volume, cell, input, handoff, and feedback layouts are
embedded exactly.
The effect header uses ABI version 2 and adds proposal kinds:
placement stream `5` and extract components `6`.
An ordinary create record remains unrepresentable.

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
unknown handles are validated.

Prior-feedback records for a published extraction include, for each piece that
requested `PublishChild`:

```text
ComponentIdentityFeedbackV2 (48 bytes)
  0 proposal_slot:u32
  4 piece_handle:u32
  8 volume_id_low:u32
 12 volume_id_high:u32
 16 volume_key:[u8;16]
 32 state:u32       // Published=2, Unused=3, Failed=4
 36 reserved:[u32;3]
```

Rejected or cancelled proposals write only the failed/unused states; no live
directory resolution is possible from those records.

All count-to-byte arithmetic is checked before allocation and again on GPU.
Every effective binding range fits
`max_storage_buffer_binding_size`.
Scheduled v2 requires the same six storage bindings in one shader stage as
v1.

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
| `behavior_component_extraction_bytes` | 32 MiB | 256 MiB; covers mapping, payload, transfer, fracture-delta, and validation records |
| `behavior_cpu_egress_bytes` | 1 MiB | 16 MiB; aggregate enabled `maximum_cpu_egress_bytes` fit inside handoff/staging budgets |

These pools do not alias proposal, generic staging, command, observation, or
adapter factory-resource pools, except that terminal egress deliberately
reuses the handoff and staging accounting already required for GPU-to-CPU
handoffs.
Tick admission reserves their complete descriptor maxima before execution.
For every selected extract-components proposal it reserves one fracture
directory delta large enough for the source and every pre-reserved child;
path sharing only releases unused capacity after construction.
Telemetry reports current, high-water, waiting/rejected, last-use-delayed
bytes, overflow count, map/decode failure count, unused component-extraction
reservations, matter-conservation failures, placement update count, and
fracture-delta publication bytes/latency.

## Persistence and rematerialization

Every child has optional substrate provenance for audit and restore checks:

```text
DerivedExtractionProvenance {
    parent_volume_key,
    parent_revision,          // pinned revision before extraction
    extraction_command_id,
    proposal_slot,
    piece_handle,
    sample_count,
    sample_digest,
}
```

This is substrate provenance, not a behavior label.
The child does not have a consumer `BaseContentSource`.

Child rematerialization does **not** require a separate checkpoint-v2 complete
sparse derived-base category or provenance rematerialization path.
Publication installs a closed internal **empty-base** descriptor for the child
and records every transferred nonempty brick as an ordinary T8 full-brick
scar relative to that empty base.
Those scars are dirty authoritative persistence input and remain pinned in
retained GPU/scar capacity until a checkpoint is durable.
Before that checkpoint, device loss is `UnrecoverableDirtyState` under the
existing rule.
Later edits are ordinary full-brick scars relative to the same empty base
(or compacted equal-to-base removals).

Checkpoint format continues to store identity, domain, cell size, dynamic
mode, placement, optional provenance, and the scar set.
The world's `derived_key_namespace` is stored so restore can re-derive and
verify each child key from parent key, parent revision, proposal slot, and
piece handle; there is no next-candidate ordinal.
Restore recreates child identity, domain, cell size, dynamic mode, placement,
provenance, and sample bytes without a consumer content source by applying
empty base plus scars.
The parent need not still be live.
Cold rematerialization loads scars over the empty base.
Digest/count mismatch, missing scar chunk, or an attempt to attach a consumer
source to the child fails restore.

What reconstruction requirement cannot be met by source-plus-scar? None for
the substrate contract: once transferred samples are full-brick scars against
an empty base, the child's complete contents are represented by the existing
T8 checkpoint/restore path without a parallel derived-base store.

Placement streams persist only the resulting committed per-volume placements
and revisions.
Activity-region input, adapter body/fidelity state, coarse/full state,
handoffs, and egress bytes are never persisted by Moria.

## Required validation

The validation plan must retain all prior scheduled-adapter evidence and add:

1. one real-GPU adapter labels at least three connected pieces in one source,
   publishes at least two children, retains a source remainder, and proves
   exact coordinate/sample conservation and tolerance-bounded world-box
   continuity across one fracture-scoped directory-generation gate;
2. every candidate identity/resource is reserved before adapter execution,
   piece handles remain the only in-dispatch labels, and prior feedback
   returns final `VolumeId`/key mappings without CPU authority-path readback;
3. cancellation, every validation sentinel, exhausted live/lifetime/page/
   brick/scar/byte pool, renderer OOM, device loss before and after the gate,
   shutdown, unused child slots, and old-reader reclamation leak no capacity;
4. checkpoint/restore and cold rematerialization reproduce child identity,
   provenance, placement, samples, occupancy, and later edits from empty base
   plus T8 scars without a consumer source;
5. a proof adapter consumes CPU-authored disconnected and overlapping regions,
   processes each body once in the overlap, preserves one body/volume identity
   across halo promotion/demotion, has no transform/velocity discontinuity,
   and continues coarse motion/destruction-shaped generic effects outside all
   regions;
6. the placement stream publishes compacted full/coarse changes without host
   body enumeration, stale Moria placement, duplicate volume entry, or
   per-object ordinary move proposals, and without cross-volume atomic
   directory roots;
7. fixed maximum dispatch is measured at empty, sparse, half, and full active
   lists and passes P11 on every claimed backend family; and
8. an adapter-owned egress payload unknown to Moria round-trips byte-exactly
   for zero bytes, one prefix, exact capacity, and multiple ticks; one-over
   capacity, malformed header, cancellation, shutdown, map/decode failure, and
   device loss produce distinct terminal results with no silent bytes or
   leaked/reused-early buffers, using the terminal handoff/staging path.

The adversarial reviewer must also demonstrate that none of these APIs adds a
Moria type for physics, damage, weapons, scoring, audio, region significance,
velocity, force, health, or debris policy.
