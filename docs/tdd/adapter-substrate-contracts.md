# Adapter Substrate Contracts

## Purpose and authority

This file adds three capabilities to the scheduled behavior contract:

1. atomically redistribute matter from one existing volume into independently
   placed child volumes;
2. publish a bounded batch of dynamic-volume placements; and
3. return bounded opaque GPU records to the owning CPU consumer.

These are small extensions of the existing scheduled tick, not separate
subsystems.
They reuse `BehaviorTickPermit`, the participant effect allocation, Moria's
copy-on-write storage transactions, ordinary receipts and observations, and
the configured GPU/staging pools.
The public Rust types are normative in [public-api.md](public-api.md), tick
ordering is normative in [behavior-scheduling.md](behavior-scheduling.md), and
persistence is normative in [persistence.md](persistence.md).

Moria owns bounds, identity reservation, validation, publication, lifetime,
failure, and transport.
The adapter owns component discovery and significance, activity-region
meaning, fidelity classes, coarse/full simulation, and the schema and meaning
of egress bytes.
No Moria type represents physics, damage, weapons, velocity, forces, debris,
scoring, audio, or gameplay events.

Scheduled ABI v2 is the only scheduled ABI accepted by the initial
implementation.
It retains v1 record layouts where those layouts did not change.

## Capability declaration and admission

`BehaviorEngineDescriptor` declares:

- maximum placement updates;
- maximum extraction proposals, candidate children, assigned source cells,
  child bricks, and extraction payload bytes; and
- an optional egress schema, stride, record count, and byte maximum.

The complete field-level schema and configuration maxima are in
[public-api.md](public-api.md).
CPU adapters declare all three capabilities as zero.
For a GPU adapter, all extraction maxima are zero together or nonzero
together.
Egress is disabled only when all egress fields are zero.
When enabled, its stride is four-byte aligned and
`maximum_records * record_stride == maximum_bytes` under checked arithmetic.
Moria does not interpret the schema identifier or records.

Registration checked-sums each descriptor against the corresponding world
limits.
Tick admission atomically reserves the participant effect range and all
ordinary resources that its declared worst case can consume.
This includes child identities and volume records, copy-on-write page/brick
and scar capacity, one directory publication transaction per root-changing
proposal, placement records, egress working/staging/host bytes, map slots,
receipts, observations, and cleanup records.
Admission never holds a partial reservation while waiting for another pool.
If the complete reservation is unavailable, the tick waits or is rejected
under its existing overload policy and no adapter executes.
Unused capacity is released only after validation establishes the used prefix
and the last GPU or mapped use is complete.

## Atomic component extraction

### Input and output

`ExtractComponents` addresses exactly one pinned source-volume snapshot.
Its payload contains:

- a bounded set of nonzero proposal-local piece handles;
- for each piece, `PublishChild` or `RemoveFromMatter`; and
- a sorted set of source-cell coordinates assigned to those handles.

Unassigned source cells remain in the source.
`PublishChild` transfers the assigned samples to a new persistent dynamic
volume.
`RemoveFromMatter` explicitly removes the assigned samples and reports their
count; Moria gives that removal no debris or gameplay meaning.

The operation cannot introduce a sample, copy one source cell to two owners,
address a second source, attach consumer-authored content, or transport a
`BaseContentSource`.
The adapter decides which discovered pieces are published or removed.
Moria validates assignments and conservation but does not define or validate
the adapter's notion of connectivity.

On success, `ComponentExtractionApplied` reports:

- the source remainder revision or source retirement revision;
- published children ordered by piece handle, including final `VolumeId`,
  stable key, revision 1, local domain, placement, and sample count;
- the explicit removed-cell count; and
- the committed `WorldDirectoryEpoch`.

Unused candidate identities never appear in the result.

### Pre-reserved child identities

For every declared proposal slot and legal piece handle, Moria reserves a
candidate runtime ID and stable key before the adapter dispatch.
The participant sees the complete dense
`(proposal_slot, piece_handle) -> VolumeId, VolumeKey` table as read-only
scheduled input.
This lets adapter-owned GPU state associate a discovered body with its final
Moria identity without authority-path readback.

Candidates are not live and cannot resolve through a view or effect until the
extraction publishes.
Successful children retain their reserved identities.
Unused or failed candidate handles become permanently invalid; their backing
slots return to the pools without creating tombstones.
Moria rejects the participant if it modifies the canonical reservation bytes.

The compact wire record is:

```text
ComponentReservationRecordV2 (48 bytes)
  proposal_slot:u32
  piece_handle:u32
  volume_id_low:u32
  volume_id_high:u32
  volume_key:[u32;4]
  state:u32                 // Reserved=1, Published=2, Unused=3, Failed=4
  reserved:[u32;3]          // zero
```

The reservation section contains exactly the descriptor-declared
`maximum_component_extraction_proposals *
maximum_component_extraction_children` records in proposal-slot then
piece-handle order; its byte length is therefore `48 * record_count` under
checked arithmetic.
It has no second header because the scheduled effect header already supplies
the section offset and length and the descriptor supplies the dimensions.

Stable keys are derived deterministically from the world key, tick,
participant, proposal slot, candidate slot, and a collision-retry salt.
The complete candidate set is checked against live keys, retained tombstones,
and itself before dispatch.
Only published keys become durable lifetime records.

### Conservation, frames, and atomic publication

For every occupied source coordinate/sample pair, validation requires exactly
one post-operation disposition:

```text
source_before = source_remainder + all_children + explicitly_removed
```

The equality is an exact multiset equality over source coordinates and packed
samples.
A digest is not a substitute for this check.
Validation rejects duplicate assignments, empty-source assignments,
out-of-range handles, count/byte overflow, or any missing/duplicated sample.

For a published child, the lexicographically smallest assigned source cell is
its local origin.
Each transferred coordinate becomes `source_cell - origin`; the tight
half-open AABB is the child domain.
The child inherits cell size, axes, samples, occupancy rules, and material
registry interpretation.
Its initial placement is:

```text
source_placement * translate(origin * source_cell_size)
```

Therefore each transferred cell occupies the same world-space box immediately
before and after publication.

Moria builds the source remainder and every child in unreferenced reserved
storage, validates the complete candidate state, and then installs one
immutable directory root through one checked `WorldDirectoryEpoch` gate.
Readers pin a directory epoch before resolving volume revisions, so they see
either the old source or the complete new source/children directory, never an
intermediate mixture.
Old roots and storage remain pinned until every old-epoch reader and the
publication submission complete.
Per-volume revisions remain the public freshness identities; the directory
epoch is only the atomic multi-volume visibility gate.

### Failure and persistence

Cancellation before `Preparing`, pre-dispatch allocation failure, validation
failure, stale source revision, or device loss before confirmed publication
leaves the old directory current and eventually releases every candidate and
reserved resource.
After `Preparing`, cancellation is too late and cleanup continues even if the
consumer drops receipts.
A device generation lost after confirmed publication retains the committed
outcome and follows the ordinary dirty-state recovery rule.

Each child persists its identity, domain, placement, source-extraction
provenance, and complete sparse derived base.
It has no consumer `BaseContentSource`.
Until its first checkpoint is durable, those derived-base bricks are dirty and
pinned; loss of the only authoritative copy is
`UnrecoverableDirtyState`.
Restore and cold rematerialization load the derived base and then later scars.
Activity-region input, adapter body state, and egress bytes are never
checkpointed by Moria.

## Placement stream for persistent adapters

`PlacementStream` is one scheduled proposal containing a bounded,
stable-compacted array of rigid placement updates.
Every entry names a distinct dynamic volume in the participant's pinned
`VolumeRecords` view and includes its expected revision.
Moria rejects the complete stream for a duplicate volume, stale revision,
static volume, malformed transform, or count/byte overflow.

The compact record is:

```text
PlacementUpdateV2 (64 bytes)
  snapshot_index:u32
  flags:u32                 // zero
  expected_revision_low:u32
  expected_revision_high:u32
  translation:[f32;4]
  rotation_xyzw:[f32;4]
  reserved:[u32;4]          // zero
```

One validated stream publishes all addressed directory entries through one
directory epoch and advances each addressed volume revision once.
`PlacementStreamApplied` reports the updated volume/revision pairs in
ascending snapshot-index order.
The mechanism avoids host enumeration and one ordinary move command per body;
it does not choose which objects move or how their poses are calculated.

CPU-defined activity regions remain opaque participant input.
A conforming multi-fidelity adapter uses one persistent body table across all
regions, classifies a body once against the deterministic union of overlapping
regions, preserves body and volume identity through promotion/demotion, and
continues its adapter-owned coarse work outside all full-fidelity regions.
Those are adapter proof obligations, not Moria data models.
The initial portable path uses the descriptor's existing fixed dispatch and
workgroup maxima with guarded inactive lanes; it exposes no raw indirect
dispatch buffer.
P11 must prove the declared maximum is viable before this integration claim is
accepted.

## Opaque GPU-to-CPU egress

An enabled GPU participant receives one zero-initialized fixed-stride egress
range.
The supplied WGSL helper reserves record indices atomically, preserves the
full required count, and sets overflow rather than writing beyond capacity.
Moria copies only a valid initialized prefix to its pre-reserved staging range
and delivers it through `BehaviorEgressReceipt`.
Records remain byte-for-byte adapter data.

The header is:

```text
BehaviorEgressHeaderV2 (80 bytes)
  magic:u32                 // MORO
  version:u32               // 2
  engine:u32
  status:u32                // Disabled=0, Enabled=1
  record_stride:u32
  record_capacity:u32
  required_records:atomic<u32>
  overflow:atomic<u32>
  payload_offset:u32
  total_bytes:u32
  tick_low:u32
  tick_high:u32
  generation_low:u32
  generation_high:u32
  schema:[u32;4]
  reserved:[u32;2]          // zero
```

The public result carries tick, participant, correlation, schema, stride,
record count, and an exact shared byte slice.
Zero records is successful ready-empty.
Exact capacity succeeds.
Overflow or counter saturation fails with no delivered prefix; truncation is
forbidden.
Mapping, envelope decode, cancellation, shutdown, participant-not-run, and
device loss have distinct terminal outcomes listed in
[public-api.md](public-api.md).

Effect publication never waits for CPU decoding and may complete while egress
is pending.
The working range is reusable after its copy's last GPU use; staging is
reusable only after map completion, view drop, and unmap; host bytes remain
charged until the last shared result handle is dropped.
Dropping a receipt does not cancel a submitted copy or release its resources
early.
GPU-to-GPU data continues to use scheduled handoffs and is never routed
through this CPU lane.

## Scheduled ABI v2 integration

Group 0 remains exactly six storage bindings:

| Binding | Access | Contents |
| ---: | --- | --- |
| 0 | read-only | participant stable view |
| 1 | read-write | effects, child reservations, and optional egress |
| 2 | read-only | incoming handoffs |
| 3 | read-write | outgoing handoffs |
| 4 | read-only | prior feedback |
| 5 | read-only | current consumer input |

The 64-byte effect header keeps its existing fields and uses its former four
reserved words for aligned
`reservation_offset`, `reservation_bytes`, `egress_offset`, and
`egress_bytes`.
Sections are nonoverlapping and their checked aligned sum must fit one effective
storage binding.
Proposal kind 5 is `PlacementStream`; kind 6 is `ExtractComponents`.
Arbitrary create remains unrepresentable.

The extraction payload uses a 64-byte header, 32-byte piece records, and
24-byte sorted assignment records.

```text
ComponentExtractionPayloadHeaderV2 (64 bytes)
  piece_count:u32
  assignment_count:u32
  piece_offset:u32
  assignment_offset:u32
  total_bytes:u32
  reserved:[u32;11]         // zero

ComponentPieceRecordV2 (32 bytes)
  piece_handle:u32
  disposition:u32           // PublishChild=1, RemoveFromMatter=2
  declared_cell_count:u32
  reserved0:u32             // zero
  reserved:[u32;4]          // zero

ComponentAssignmentRecordV2 (24 bytes)
  source_x:i32
  source_y:i32
  source_z:i32
  piece_handle:u32
  reserved:[u32;2]          // zero
```

Counts, ranges, reserved words, duplicate cells, source membership, and
candidate handles are validated before publication.
All logical 64-bit values use the scheduled low/high `u32` representation.
The effect allocation has `STORAGE | COPY_SRC | COPY_DST`; shader access and
the restricted encoder/factory rules remain those of
[behavior-scheduling.md](behavior-scheduling.md).

## Evidence obligations

The existing validation layers must prove:

1. exact extraction conservation, child frame continuity, GPU-visible final
   IDs, old-or-new directory visibility, and query/collision/observation
   agreement;
2. cancellation, every malformed count/range/reserved word, pool exhaustion,
   stale revision, device loss on both sides of publication, old-reader
   reclamation, and unused-candidate cleanup;
3. checkpoint/restore and cold rematerialization of derived children without a
   consumer source;
4. disconnected and overlapping CPU regions, one-time classification,
   continuous identity/transform through transitions, and continued
   adapter-owned coarse work;
5. unique bulk placement publication without host body enumeration or stale
   Moria placement;
6. fixed-dispatch feasibility at the declared maximum on every claimed backend
   family;
7. byte-exact egress for zero, one, exact capacity, multiple ticks, and
   adapter-unknown schemas; and
8. explicit overflow, mapping/decode failure, cancellation, shutdown, device
   loss, ordered delivery, and no early resource reuse.

C11-C13 and P11-P13 in [validation.md](validation.md) are the retained
correctness and blocking performance receipts.
None of those proofs may use raw Moria storage, a CPU authority-path component
readback, or a behavior-specific Moria type.
