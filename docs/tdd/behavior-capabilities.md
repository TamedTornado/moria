# Behavior Adapter Capabilities

## Scope

This file extends the scheduled behavior contract with three generic
capabilities required by the committed architecture-review authority in
[`docs/evals/tdd-simplicity/overengineered-adapter-amendment.md`](../evals/tdd-simplicity/overengineered-adapter-amendment.md):

1. atomic extraction of source matter into independently placed child volumes;
2. adapter-owned multi-fidelity simulation driven by CPU-authored activity
   data; and
3. bounded opaque asynchronous GPU-to-CPU egress.

The implementation reuses the existing tick permit, stable view, proposal
buffer, directory generation, move effects, handoff staging, mapping, and
receipt machinery.
It does not add a second adapter framework, an activity-region registry, a
placement stream, an event vocabulary, or raw GPU authority.

Moria understands only source samples, child reservations, labels, placements,
opaque input bytes, opaque egress bytes, capacities, revisions, and terminal
transport outcomes.
Connectivity, significance, coarse/full fidelity, bodies, velocity, fracture,
damage, weapons, events, and every other behavior meaning remain
adapter-owned.

## Scheduled ABI v2

Scheduled ABI v2 is the only scheduled ABI accepted by the initial
implementation.
It preserves v1's six group-0 bindings and all v1 records except where this
file explicitly assigns formerly reserved words or adds a proposal kind.

| Binding | Access | v2 contents |
| ---: | --- | --- |
| 0 | read-only | participant stable-view header, volume records, cell records, then child-reservation records |
| 1 | read-write | effect header, proposal records, and payload bytes |
| 2 | read-only | incoming adapter handoffs |
| 3 | read-write | outgoing adapter handoffs plus the optional consumer-egress lane |
| 4 | read-only | prior terminal feedback |
| 5 | read-only | current opaque consumer input |

No new binding, raw resource, indirect dispatch, or submission method is
introduced.
All logical 64-bit values retain the v1 low/high `u32` representation.

`BehaviorViewHeaderV2` remains 64 bytes.
The four words at bytes 48..63 are:

```text
48 child_reservation_count:u32
52 child_reservation_offset:u32
56 child_label_capacity:u32
60 reserved:u32 = 0
```

When no registered participant can extract components, count and capacity are
zero and offset equals `total_bytes`.
Otherwise the reservation records immediately follow the cell records.
`child_label_capacity` is the participant descriptor's checked
`maximum_effect_cells`, expressed as a count of `u32` labels, and
`total_bytes == child_reservation_offset + 32 * child_reservation_count`.

```rust
#[repr(C)]
pub struct BehaviorChildReservationV2 {
    pub slot: u32,
    pub reserved: u32,
    pub volume: ScheduledU64LeV1,
    pub key: [u8; 16],
}
```

The record is exactly 32 bytes at offsets `0, 4, 8, 16`.
`slot` is dense from zero, `reserved` is zero, and both the runtime ID and UUID
are final if that slot publishes.
They are provisional and invalid for ordinary facade calls before publication.
The adapter may copy them into adapter-owned GPU state without a CPU readback.

The 32-byte outgoing handoff descriptor keeps its v1 fields.
For an ordinary edge, all three reserved words remain zero.
An optional consumer-egress lane uses `peer_engine == 0` and assigns the words
at bytes 20, 24, and 28 to `record_stride`, `record_capacity`, and
`written_records`.
The ordinary `capacity` and `written_bytes` fields remain at bytes 8 and 12,
and `status` remains at byte 16.
No registered engine receives ID zero.

The effect proposal adds kind `5 = extract-components`.
All v1 kinds and offsets remain unchanged.
For kind 5:

- the snapshot index and expected revision name one pinned source volume;
- target min/max name an authorized half-open source-local cell box;
- the field at byte 72 is `child_count`, not a material sample;
- placement fields are zero and correlation retains its ordinary opaque value;
- payload length is exactly four bytes times the checked cell count; and
- each little-endian label is `0 = keep in source`,
  `1..=child_count = transfer to reservation slot label - 1`, or
  `u32::MAX = explicitly remove`.

No other label is valid.
An empty source sample may only carry label zero.
Each named child must receive at least one nonempty material sample.
Every transferred sample preserves its exact four-byte material value.

## Atomic component extraction

### Admission and identity

`BehaviorEngineDescriptor::maximum_component_children` is zero for adapters
that cannot extract and otherwise is `1..=256`.
One scheduled tick permits at most one extract-components proposal.
`ResourceLimits::behavior_component_children` bounds the checked aggregate
maximum, defaults to 64, and has a hard v2 maximum of 1,024.

Before the tick enters `Preparing`, `BehaviorTickPermit` atomically reserves:

- the declared child count from `behavior_component_children`;
- the same number of free live-volume and lifetime-volume records;
- provisional runtime IDs and stable UUID keys;
- one never-reused `ComponentExtractionKey` shared by the source and children;
- child directory entries and one replacement directory root;
- the descriptor's declared `maximum_effect_bricks` for the union of changed
  source bricks and distinct canonical child bricks, plus their page keys,
  page versions, detailed slots, and dirty derived-base records;
- source copy-on-write, scar, observation, outcome, and receipt records; and
- the label bytes inside the existing proposal-byte and affected-cell limits.

Failure reserves nothing and invokes no adapter.
Unused child reservations return after proposal validation.
They do not consume lifetime keys because only published identities enter the
world directory and tombstone history.
Their provisional runtime generations become stale and their UUID values are
never reused, so an adapter-retained failed reservation can never alias a
later live child; burning that never-valid UUID consumes no `volume_records`
entry or tombstone.

### Preparation and publication

Moria validates the complete label array on the GPU against the pinned source
snapshot.
The combined distinct changed-source and canonical-child brick count must fit
the participant's declared `maximum_effect_bricks`; overflow rejects the
complete proposal before publication.
It computes each child's source-coordinate bounding box, then selects a
canonical child frame:

```text
child local cell = source local cell - component_min
child domain     = [0, component_max - component_min)
child rotation   = source rotation
child translation =
    source placement applied to (component_min * source cell_size)
```

Every child is `Dynamic`, inherits the source cell size and world material
registry, and begins at revision one.
Its diagnostic name is exactly `extracted-` followed by the child's lowercase
32-hex-digit UUID without hyphens (42 ASCII bytes); no adapter string crosses
the scheduled ABI.
The source advances by one revision.
Moria builds the changed source pages and all child pages in unreferenced
storage.

The existing versioned world-directory root is the publication gate.
One ordered compare-and-swap installs a replacement root containing the source
revision and every child entry.
Readers pinned to the earlier directory generation see the complete old
source; later readers see the changed source and all children.
No reader can see a child before source removal, source removal without every
child, or a partially initialized child.

For every nonempty source sample inside the target, validation proves exactly
one outcome: retained in the source, transferred to one child, or explicitly
removed.
Duplicate transfer, out-of-target transfer, an empty child, capacity overflow,
invalid label, source mismatch, or any failed allocation rejects the complete
proposal and leaves the directory root unchanged.

This is the only multi-volume atomic scheduled operation in v2.
It cannot invent material, use a consumer `BaseContentSource`, create a child
from another source, or perform arbitrary volume creation.
It conflicts with every other proposal addressing the source.
Other unrelated per-volume proposals retain their existing independent
publication semantics.

### Completion and reclamation

The proposal outcome is:

```rust
pub struct ComponentExtractionApplied {
    pub extraction: ComponentExtractionKey,
    pub source: VolumeId,
    pub source_revision: VolumeRevision,
    pub children: Box<[ExtractedChild]>,
    pub correlation: Correlation,
}

pub struct ExtractedChild {
    pub slot: u32,
    pub volume: VolumeId,
    pub key: VolumeKey,
    pub revision: VolumeRevision, // always one
    pub domain: CellAabb,
    pub placement: RigidPlacement,
}
```

The child vector is exact and bounded by the proposal's pre-reserved child
records.
It is also encoded into the normal observation ring as one bounded
`ComponentsExtracted` fact carrying the extraction key, source revision, child
identities, and placements.
The fact and append-time filter envelope are reserved before execution.

Cancellation is possible only before the behavior family's existing
`Preparing` transition.
After preparation begins, validation/allocation failure produces no
publication and releases every provisional ID, slot, page, and root after its
last GPU use.
Released provisional runtime generations and UUIDs remain permanently invalid
as specified at admission.
Device loss before confirmed root publication produces typed no-publication
and quarantines the generation.
Confirmed root publication remains committed and follows the existing
dirty-state recovery rule.
Old source pages and the old root remain pinned until their snapshot readers
and last submissions complete.

## Persistence for extracted children

Checkpoint format v2 distinguishes:

```text
ExternalBase {
    lineage,
    reconstruction_fingerprint,
}

ExtractedBase {
    parent_volume_key,
    extraction_key,
    parent_revision,
    complete_nonempty_brick_refs,
}
```

`extraction_key` is the exact `ComponentExtractionKey` UUID shared by that
publication's children, and `parent_revision` is the pinned source revision
before extraction.
An extracted base stores every full brick containing a nonempty sample at
creation; all omitted bricks in the finite child domain are canonical empty.
Later child edits remain ordinary full-brick scars relative to that immutable
base.
This is sparse reconstruction state, not derived geometry and not a raw dump
of the source volume.

Until a checkpoint durably contains an extracted base, its complete
dirty-derived-base bricks pin authoritative residency.
Retirement or pressure cannot discard them.
Device loss during that interval is `UnrecoverableDirtyState`, exactly like
another committed GPU-only scar.
After durability, cold materialization reads the extracted base and later
scars through the existing bounded checkpoint reader and staging pools; it
never calls a consumer content source for that child.

Restore still requires exact registration of every externally sourced live
volume.
It reconstructs extracted descendants named by the manifest without a builder
registration, after validating their parent provenance, stable-key
uniqueness, domain, material IDs, complete derived-base chunks, and tombstone
status.
The parent key may name a live manifest record or a retained tombstone whose
terminal revision is at least the recorded extraction parent revision; the
parent's current liveness is not required.
Nested extracted descendants are validated in parent-before-child
topological order, and a missing or cyclic provenance chain is an extracted-
provenance restore mismatch.
An extra builder registration using an extracted key is a restore mismatch.

## CPU-authored multi-fidelity adapter pattern

Moria adds no activity-region type or fidelity state.
A CPU/game layer encodes one or more regions and any transition parameters in
the participant's existing bounded opaque current input.
The same immutable bytes reach the planner/CPU callback and scheduled binding
5.

A conforming GPU adapter may keep one persistent adapter-owned body table,
classify each body against the deterministic union defined by its private
schema, and run coarse, transition, or full work in its own kernels.
Disconnected and overlapping regions therefore remain one adapter and one
simulation state, not separate Moria worlds or participants.
The adapter is responsible for processing a body once, retaining identity and
state through transitions, and continuing its coarse policy outside every
region.

Moria's only placement responsibility is to validate and publish the adapter's
ordinary bounded move proposals.
Each changed dynamic volume consumes one existing directory-effect and proposal
record, and all moves for one volume still coalesce into that volume's one tick
revision.
The descriptor's `maximum_directory_effects`, the configured proposal and
directory-effect limits, and the tick permit bound the maximum moved bodies.
An adapter that needs a larger population selects larger legal limits and must
pass the fixed feasibility gate; Moria does not silently freeze omitted bodies
or claim their placements advanced.

This reuse is deliberate.
A separate activity registry, Moria-owned fidelity enum, bulk placement ABI,
or indirect-dispatch authority is not required to expose the capability.

## Opaque GPU-to-CPU egress

### Declaration and admission

A GPU descriptor may declare:

```rust
pub struct BehaviorEgressDescriptor {
    pub record_stride: NonZeroU32,   // 1..=4,096
    pub maximum_records: u32,        // 1..=65,536
}
```

CPU descriptors and GPU descriptors without egress use `None`.
The checked product must fit 64 MiB, one storage binding, and the descriptor's
handoff maximum.
The builder charges an egress lane as one synthetic outgoing handoff:
one device range, one staging range, one host result range, one map, and one
terminal receipt.
These use the existing `behavior_handoff_bytes` and
`behavior_handoff_maps` pools and appear separately in telemetry as handoff
subkind `ConsumerEgress`.

The complete maximum is reserved by `BehaviorTickPermit` before any adapter
runs.
There is no hidden egress allocation and no readback route for adapters that
declared `None`.

### Production and delivery

The egress payload is zero-initialized.
Moria initializes status `Empty` and both counts to zero.
The adapter writes a fixed-stride initialized prefix, the descriptor's
`written_records`/`written_bytes`, and status `Ready`; it must write `Ready`
even for zero records.
Moria validates:

```text
written_records <= maximum_records
written_bytes == written_records * record_stride
written_bytes <= capacity
```

An overflow or inconsistent count returns no prefix.
Leaving status `Empty`, using an unknown status, or writing `Failed` terminates
only the egress receipt with
`OperationErrorKind::Behavior(BehaviorEngineFailure::GpuValidation)`.
Moria copies only the validated initialized prefix to the dedicated staging
range after the adapter's last write, waits for queue and map completion,
copies it into an exact `Arc<[u8]>`, drops the mapped view, and unmaps before
reusing the range.
Moria never decodes the payload.

`BehaviorTickCompleted` carries one `BehaviorEgressReceipt` for each declared
lane that reached dispatch:

```rust
pub type BehaviorEgressReceipt = Receipt<BehaviorEgressReady>;

pub struct BehaviorEgressReady {
    pub tick: BehaviorTickId,
    pub engine: BehaviorEngineId,
    pub correlation: Correlation,
    pub record_stride: NonZeroU32,
    pub records: u32,
    pub bytes: Arc<[u8]>,
}
```

Zero records is successful `Ready` with an empty exact slice.
It is distinct from pending or failure.
Exact capacity succeeds.
One-over count, count/byte mismatch, or shader overflow is
`OutputOverflow` with no delivered bytes.
Mapping, decode-of-transport-header, shutdown, and device loss retain their
existing distinct operation errors.

Tick publication and the tick receipt do not wait for egress mapping.
The tick result contains the egress receipt after copy submission is known.
Publication never depends on CPU interpretation of egress.
Dropping an egress receipt does not cancel submitted GPU work.
Because the receipt is exposed only after copy submission,
`request_cancel()` returns `TooLate { stage: Submitted | AwaitingReadback }`.
The device/staging slots remain charged through last GPU/map use, and the host
result charge remains until the last receipt/result clone drops.
That backpressure is bounded and may delay a later tick permit.

Cancellation before `Preparing` creates no egress receipt.
A participant not run creates no egress receipt and its participant outcome
explains why.
Shutdown drains submitted egress or resolves it with typed terminal loss before
releasing the adapter generation.
GPU-to-GPU data continues to use ordinary handoffs and is never forced through
this CPU lane.

## Required evidence

The following are blocking:

- exact source-sample conservation across old/new directory snapshots;
- usable child IDs in adapter GPU state without authority-path CPU readback;
- all-or-none failure after child pages are prepared;
- first-checkpoint and later-checkpoint restore/rematerialization of extracted
  children;
- disconnected and overlapping CPU-authored region bytes, one-time body
  processing, continuous adapter-owned state across promotion/demotion, and
  continuing coarse move proposals outside every region;
- zero, exact-capacity, and one-over opaque egress;
- publication completing while egress remains pending;
- map failure, cancellation, shutdown, and device-loss cleanup; and
- host/WGSL v2 layout, tag, offset, count, label, egress, and reserved-word
  negative fixtures on every claimed backend family.

No evidence may inspect authoritative storage, infer correctness from a render
mesh, use a second GPU device, or add behavior-specific fields to Moria.
