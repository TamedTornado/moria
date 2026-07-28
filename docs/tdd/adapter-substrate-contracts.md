# Adapter Substrate Contracts

## Scope

Scheduled GPU behavior exposes three additional substrate capabilities:

1. source-bound extraction of existing matter into independently placed child
   volumes;
2. bounded bulk placement updates for adapters that keep coarse simulation
   active outside CPU-selected full-fidelity regions; and
3. optional bounded opaque GPU-to-CPU egress.

These are transport and publication contracts. Moria does not define physics,
damage, weapons, connectivity significance, activity-region policy, fidelity
state, debris, or an event vocabulary. Those meanings and all simulation state
remain adapter-owned.

The initial Scheduled ABI v1 is revised in place because it has not shipped.
Bindings 0 through 5 and their record layouts remain unchanged. Binding 6 adds
the read-only child reservation table and binding 7 adds the optional
read-write egress target. There is no compatibility negotiation or parallel
directory architecture.

## Declared capacity

`BehaviorEngineDescriptor` adds:

```rust
pub struct BehaviorEngineDescriptor {
    // Existing fields remain.
    pub maximum_placement_updates: u32,
    pub maximum_component_children: u32,
    pub cpu_egress: BehaviorCpuEgressDescriptor,
}

pub struct BehaviorCpuEgressDescriptor {
    pub schema: [u8; 16],
    pub record_stride: u32,
    pub maximum_records: u32,
}
```

The new fields are zero for CPU adapters. A GPU adapter may use any subset.
Egress is disabled when all egress fields are zero. When enabled,
`record_stride` is four-byte aligned in `4..=65_536`, and its checked product with
`maximum_records` fits both `maximum_proposal_bytes` and the configured
behavior-egress byte limit. Registration checked-sums three times that product
across enabled adapters for the distinct device, staging, and retained-host
ranges.

The existing descriptor maxima do the rest of the accounting:

- extraction assignments and piece records count against
  `maximum_effect_cells`, `maximum_effect_bricks`, `maximum_proposals`, and
  `maximum_proposal_bytes`;
- each possible child counts against `maximum_component_children`,
  `maximum_directory_effects`, live-volume and lifetime-volume capacity;
- placement records count against `maximum_placement_updates`,
  `maximum_directory_effects`, and `maximum_proposal_bytes`; and
- egress reserves its exact device, staging, and eventual host bytes, one
  staging map, and its result record before execution.

Tick admission acquires these permits together with the existing proposal,
transaction, page, brick, scar, observation, presentation, completion, and
feedback permits. It never executes an adapter after a partial reservation.
The two new configured pools are `behavior_egress_records` and
`behavior_egress_bytes`; the latter charges the three distinct device,
staging, and retained-host ranges. Existing staging-map capacity owns the map.
No directory-root, directory-node, authority-version, egress-map, or
egress-receipt pool is added.

## Source-bound component extraction

### Proposal and identity

`ExtractComponents` is one GPU-only scheduled proposal against exactly one
volume and its pinned revision. Its payload contains:

```text
Piece {
    handle:u32,                    // nonzero, proposal-local
    disposition:u32,               // Child=1, Remove=2
}

Assignment {
    source_x:i32,
    source_y:i32,
    source_z:i32,
    piece_handle:u32,
}
```

Pieces are unique and sorted by handle. Assignments are sorted by handle then
source Z/Y/X. Every assignment must name an occupied source cell visible in
the pinned view, and no cell may occur twice. An unassigned source cell remains
in the source. `Child` transfers assigned samples to a persistent dynamic
volume. `Remove` explicitly removes them without giving Moria a debris or
gameplay meaning. The proposal cannot introduce or clone a sample, refer to a
second source, or carry a `BaseContentSource`.

Before any adapter runs, Moria reserves one candidate `VolumeId`, generated
stable `VolumeKey`, live entry, lifetime entry, and result slot for every
possible child. Each key is one CPU-generated UUIDv4; generation failure or a
collision with a live/tombstoned key fails preflight rather than retrying
without a bound. Unused or failed candidates never become
directory or tombstone records and are reusable only after their last GPU use;
a published stable key is permanent in the ordinary lifetime registry.

Binding 6 is an immutable dense table keyed by participant, proposal slot, and
piece handle:

```text
ComponentReservationV1 {
    proposal:u32,
    piece_handle:u32,
    volume_id_low:u32,
    volume_id_high:u32,
    volume_key:[u8;16],            // WGSL: four u32 words
    reserved:[u32;4],
}
```

Its exact 64-byte header is:

```text
ComponentReservationHeaderV1 {
     0 magic:u32,                 // MORR
     4 version:u32,               // 1
     8 engine:u32,
    12 record_count:u32,
    16 record_offset:u32,         // 64
    20 total_bytes:u32,
    24 tick_low:u32,
    28 tick_high:u32,
    32 generation_low:u32,
    36 generation_high:u32,
    40 reserved:[u32;6],
}
```

An adapter that does not
extract components receives a valid empty table. The adapter may copy final
IDs into its factory-owned body state during its dispatch. They resolve as
volumes only if next-tick feedback reports that the proposal published; unused
or failed candidate IDs are stale forever. No authority-path CPU readback is
required.

### Frames and matter ownership

For each child, Moria chooses the lexicographically smallest assigned source
cell `(z, y, x)` as integer origin `o`. Child axes and cell size equal the
source's. Source cell `c` becomes child-local `c - o`, and the child's domain
is the tight half-open bound of its assigned cells. Its initial placement is:

```text
source_placement * translate(o * source.cell_size)
```

The child therefore initially occupies the same world-space cell boxes as its
source records. Rotation is inherited exactly. Every child starts at revision
1 and may later move through the ordinary move or bulk-placement contract.
The source advances once if a remainder exists; otherwise the same transaction
retires it at its next revision.

Conservation is exact over `(source coordinate, packed sample)` records:

```text
source before = source remainder + child records mapped through each origin
                + explicitly removed records
```

Moria validates membership, uniqueness, counts, bounds, samples, and that every
input record occurs in exactly one term. Digests may be reported as evidence
but do not replace this proof. Moria does not validate connectivity or decide
which pieces deserve child volumes.

### Atomic publication by reusing transactions

Extraction extends the existing prepared matter transaction rather than adding
a versioned world directory. Preparation writes the new source pages, child
pages, directory records, scars, and observations only into reserved,
unreferenced slots. Each prepared source/child authority head contains its
previous head and the index of one `CompoundVolumeTransaction`:

```text
CompoundVolumeTransaction {
    expected_source_revision,
    state: atomic<u32>,            // Prepared, Committed, Failed
}
```

Directory and snapshot lookup ignores a prepared head whose transaction is not
`Committed`, following the previous head; a new child has no previous head and
is absent. Ordered preparation installs every head and validates all
sentinels. One separate publication dispatch compares the pinned source
revision and changes the transaction state from `Prepared` to `Committed`.
This word is the linearization point. Because all heads and payloads were
installed by earlier ordered dispatches, readers see either the old source
alone or the complete source remainder plus all children. They cannot see a
half-created child set, duplicated matter, or an ownerless interval.

The transaction conflicts with every selected proposal addressing the source.
Its child entries also reserve their identities, so no ordinary operation can
address them before commit. Existing per-volume queues serialize later work;
unrelated proposals keep the existing independent-publication behavior.
Old heads and pages remain pinned until their readers and the publication
submission finish.

Validation or allocation failure leaves the transaction uncommitted and
reclaims all candidate resources after last GPU use. Cancellation wins only
before the existing `Preparing` boundary. Device loss before confirmed commit
reports no publication and quarantines old-generation candidates before
release. Confirmed commit remains applied; later loss follows the existing
dirty-authority recovery rule. Shutdown drains work that crossed
`Preparing`. A rejected proposal's feedback is sufficient for the adapter to
discard provisional body associations.

### Persistence

An extracted child has a substrate-owned `DerivedExtraction` source record:
parent stable key and revision, extraction command, piece handle, sample count,
and sample digest. Its complete nonempty extracted bricks enter the existing
dirty-scar path; absent bricks are canonical empty. Until a checkpoint makes
them durable they remain pinned and device loss may produce the existing
`UnrecoverableDirtyState`.

Persistence format v2 includes an `ExternalBase |
DerivedExtraction` source tag in each volume record. External volumes retain
their lineage/fingerprint contract. A derived child stores its provenance and
complete sparse initial brick references, then ordinary later scars.
Restore accepts registrations only for external volumes, reconstructs derived
children from those durable bricks, and validates identity, provenance, domain,
cell size, placement, sample count, and digest before directory publication.
Cold rematerialization loads the derived brick or canonical empty rather than
calling a consumer source. Import mode preserves the stored child keys.

## CPU activity regions and persistent multi-fidelity adapters

Activity regions use existing current-tick opaque consumer input on binding 5.
The CPU/game layer chooses their schema, shapes, number, and halo parameters;
Moria neither parses them nor chooses important regions. One adapter owns one
persistent body table for the world. It must deterministically classify each
body once against the union of all supplied regions into its own mutually
exclusive full, halo, or coarse work. Overlap cannot duplicate processing, and
crossing a boundary cannot change the body or volume identity.

Full physics, coarse integration, transform/velocity state, transition copy,
and remote destruction policy remain adapter-owned. The adapter must continue
its coarse work outside every region and preserve its transform/velocity
continuity through promotion and demotion; Moria merely transports input and
publishes generic effects.

`PlacementBatch` is one scheduled proposal whose payload is a packed array of
ordinary move records:

```text
PlacementUpdateV1 {
    snapshot_index:u32,
    reserved:u32,
    expected_revision_low:u32,
    expected_revision_high:u32,
    translation:[f32;4],
    rotation_xyzw:[f32;4],
}
```

Entries must be unique, in snapshot-index order, finite, normalized, dynamic,
and bound to the pinned revision. The batch is all-valid-or-rejected; after
validation, each entry uses the existing placement transaction and advances
that volume once. Entries do not need atomic cross-volume visibility.
Preparation and submission are batched, and one proposal/result replaces
per-object host commands and receipts. Only changed poses need be compacted.
If adapter-owned motion changes a matter-backed volume's pose, the adapter
must include it in that tick's batch or fail its tick; Moria placement may not
silently become stale.

The existing counted encoder's fixed dispatch/workgroup maxima are sufficient:
classification, active-list compaction, full/halo/coarse work, and changed-pose
compaction dispatch over declared maximum ranges while every kernel guards its
compacted logical count. The adapter declares a finite upper bound at
registration, and tick admission charges that bound. No indirect buffer or raw
encoder is exposed. Qualification measures empty, sparse, overlapping, and
full lists; failure blocks this fixed-dispatch selection rather than permitting
an undocumented mechanism.

## Opaque GPU-to-CPU egress

Binding 7 is zero-initialized and contains this exact 80-byte transport header
followed immediately by the declared fixed-stride capacity:

```text
BehaviorEgressHeaderV1 {
     0 magic:u32,                 // MORO
     4 version:u32,               // 1
     8 engine:u32,
    12 enabled:u32,
    16 record_stride:u32,
    20 record_capacity:u32,
    24 required_records:atomic<u32>,
    28 overflow:atomic<u32>,
    32 payload_offset:u32,        // 80
    36 total_bytes:u32,
    40 tick_low:u32,
    44 tick_high:u32,
    48 generation_low:u32,
    52 generation_high:u32,
    56 schema:[u8;16],            // WGSL: four u32 words
    72 reserved:[u32;2],
}
```

Disabled requires zero capacity/stride/schema/counters and
`total_bytes == 80`. Enabled fields must exactly match the descriptor and
`total_bytes == 80 + record_stride * record_capacity`. Moria provides an
atomic reservation helper.
Each call increments the required record count without wrap and returns a
writable index only below capacity; attempts beyond capacity set overflow.
Moria treats the initialized prefix as bytes and never decodes or sorts it.

After adapter dispatch, Moria validates the header and copies exactly
`record_count * record_stride` bytes to the pre-reserved staging range. A
malformed header, counter overflow, or capacity overflow delivers no prefix.
Publication validation and submission continue without waiting for mapping or
CPU interpretation. Queue completion establishes publication first; mapping,
exact host copy, view drop, and unmap then establish the egress result.

The existing tick receipt resolves after every enabled participant's egress
has reached a terminal result, so a second receipt family and pending-result
pool are unnecessary:

```rust
pub enum BehaviorEgressOutcome {
    Disabled,
    Ready {
        schema: [u8; 16],
        record_stride: u32,
        record_count: u32,
        bytes: Box<[u8]>,
    },
    Failed(BehaviorEgressFailure),
}

pub enum BehaviorEgressFailure {
    ParticipantNotRun,
    Overflow { required_records: u32, capacity: u32 },
    CounterOverflow,
    InvalidHeader,
    Map,
    DeviceLost { generation: DeviceGeneration },
    Shutdown,
}
```

`BehaviorParticipantOutcome` adds `egress: BehaviorEgressOutcome`; the enclosing
tick already supplies tick ID, participant ID, and request correlation.
`Ready` with zero records and empty bytes means no events. It is distinct from
every failure. Exact capacity succeeds; overflow never silently truncates.
The tick's publication disposition and `revision_changed` remain truthful when
egress later fails.

Cancellation before `Preparing` means no adapter ran and the tick receipt's
existing cancellation error is the terminal egress answer. After `Preparing`,
cancellation is too late. A skipped/not-run participant receives
`ParticipantNotRun`. Shutdown drains submitted maps or returns `Shutdown`.
Device loss before mapping returns `DeviceLost` even if publication committed.
Working bytes release after their copy's last GPU use; staging bytes release
after map completion, view drop, and unmap; host bytes remain charged until
the tick result is dropped. Dropping the tick receipt never cancels submitted
cleanup.

One active behavior tick and one enclosing receipt already guarantee increasing
tick delivery order, so no reorder queue is added. GPU-to-GPU traffic continues
to use handoffs. The egress surface exposes no device, queue, mapped view,
authority buffer, or adapter solver state.

## Validation obligations

The existing scheduled-adapter suite additionally proves:

1. a GPU adapter labels at least three connected pieces, publishes at least two
   children with a source remainder, copies binding-6 IDs into adapter-owned
   GPU state, and preserves exact coordinate/sample ownership and initial
   world-space cell boxes without authority-path CPU readback;
2. old readers see the parent state and new readers see the complete committed
   source/children state; every reservation, malformed assignment, exhausted
   pool, cancellation, renderer allocation failure, shutdown, and device-loss
   path leaks no candidate ID, page, brick, scar, result, or byte;
3. checkpoint/restore and cold rematerialization reproduce derived identity,
   placement, provenance, samples, and later edits without a consumer source;
4. disconnected and overlapping CPU regions process each persistent body once,
   a crossing retains identity and continuous adapter-owned transform/velocity,
   coarse motion continues outside every region, and compact placement batches
   keep Moria placements current;
5. fixed maximum dispatch remains within declared dispatch/workgroup limits at
   empty, sparse, overlapping, and full activity, with a blocking physical-GPU
   performance receipt; and
6. an adapter-owned egress record unknown to Moria round-trips byte-exactly for
   zero, one, and exact capacity across multiple ticks. Overflow, malformed
   header, cancellation, participant failure, shutdown, map failure, and
   device loss are distinct and reclaim buffers only after their defined last
   use.

The adversarial review also proves that none of these APIs introduces a Moria
type for physics, damage, weapons, force, velocity, health, scoring, audio,
region significance, or gameplay events.
