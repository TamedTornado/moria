# Canonical architecture and data model

This document defines state that can affect future authoritative results and
the mechanisms that publish it. Public Rust shapes are in
[interfaces.md](interfaces.md); concrete GPU scheduling is in
[gpu-runtime.md](gpu-runtime.md).

## Canonical identity and coordinates

### TECH-005 — Stable identity model

Implements: REQ-003, REQ-017, REQ-028, REQ-033

`WorldId`, `MaterialId`, `VolumeId`, `ParticipantId`, `InputSourceId`, and
`RngStreamId` are fixed-width newtypes. `WorldId` is a consumer-supplied 128-bit
value committed at genesis. Material IDs are consumer-supplied nonzero `u16`
values. Participant IDs are nonzero `u32` values no greater than
`0x7fff_ffff`; input-source IDs are nonzero `u32` values with their high bit
clear. RNG-stream IDs are nonzero `u32` values scoped to and unique within one
participant descriptor; Moria has no global RNG registry and consumes no
randomness of its own. Each other ID is unique in its world registry.
`VolumeId` is a nonzero `u64`. Genesis volumes may claim explicit unique IDs;
canonical `next_volume_serial` starts one above their maximum. Sorted
post-genesis create commands allocate and increment it. IDs are never reused,
including after retirement or rollback-window reclamation.

Duplicate genesis IDs, zero reserved IDs, exhausted counters, and a reference
to an absent or retired ID are typed validation failures. Physical node,
buffer, slot, entity, task, and submission IDs never appear in canonical
encoding, persistence, replay, observations, or the public identity types.

```rust
pub struct WorldId(pub [u8; 16]);
pub struct MaterialId(pub u16);
pub struct VolumeId(pub u64);
pub struct ParticipantId(pub u32);
pub struct InputSourceId(pub u32);
pub struct RngStreamId(pub u32);
pub struct Tick(pub u64);
pub struct VolumeRevision(pub u64);
pub struct CanonicalOrder(pub u32);
pub struct DeviceGeneration(pub u64);
pub struct ReceiptId(pub u64);

pub struct CanonicalHash(pub [u8; 32]);
pub struct ContentDigest(pub [u8; 32]);
pub struct ContractDigest(pub [u8; 32]);
pub struct SchemaDigest(pub [u8; 32]);
pub struct BlobDigest(pub [u8; 32]);
pub struct EvidenceDigest(pub [u8; 32]);
```

Public constructors validate the nonzero/range rules above. Digest types are
not implicitly interchangeable even though their wire widths match.

### TECH-006 — Material cells, bricks, and logical domains

Implements: REQ-001, REQ-003, REQ-004, REQ-020, REQ-028

The addressable unit is a cubic cell on a volume-local integer lattice. One
canonical `CellWire` is four bytes:

```text
offset  size  field
0       2     material_id: u16 little-endian
2       2     density_q8_8: i16 little-endian
```

`material_id == 0` is empty and requires `density_q8_8 <= 0`.
`material_id != 0` requires a registered material. Occupancy is determined by
that material's immutable genesis `OccupancyClass` and density threshold;
`Never` material remains inspectable matter but contributes no collision.
Density supplies signed coverage for honest interpolated boundaries; material
presentation style does not change occupancy. Invalid combinations are
rejected at content or patch validation rather than normalized silently.

A brick contains 8×8×8 cells in x-major, then y, then z order and has an exact
2,048-byte canonical payload. A brick may instead be represented by the same
four-byte uniform cell. Floor division and remainder for negative cell
coordinates are explicitly Euclidean: `q = floor(a/8)`, `r = a - 8q`,
`0 <= r < 8`.

A volume domain is a nonempty, half-open local-cell AABB whose coordinates are
`i32`, each side is at most 8,191 cells, and every corner is within 4,095 cells
of the declared placement pivot. Large worlds use multiple sparse volumes;
their theoretical cell count is not a residency promise. Bounds, not an up
axis, define the volume.

```rust
pub struct CellWire {
    pub material_id: u16,
    pub density_q8_8: i16,
}

pub struct LocalCellPoint(pub [i32; 3]);
pub struct LocalCellAabb {
    pub min: LocalCellPoint,
    pub max: LocalCellPoint,
}
pub struct BrickCoord(pub [i32; 3]);
pub struct BrickAabb {
    pub min: BrickCoord,
    pub max: BrickCoord,
}
```

Both AABBs are half-open and require `min < max` on every axis.

### TECH-007 — Canonical fixed-point arithmetic and placement

Implements: REQ-003, REQ-019, REQ-021, REQ-028, REQ-036, REQ-043

Simulation-facing world positions and translations are signed Q23.8 values
stored as `i32` (1/256 cell resolution). Inputs outside
`[-8_388_608, 8_388_607 + 255/256]` cells are unrepresentable. Addition,
subtraction, multiplication, dot products, and squared distances use checked
signed 64-bit semantics. WGSL implements the required 64-bit intermediate with
reviewed two-word `u32` helpers; native shader `i64` is not a baseline feature.

Division truncates toward negative infinity. Right shift of signed values is
defined as floor division by the corresponding power of two. Reduction to a
narrower fixed-point form uses round-to-nearest, ties-to-even. Overflow,
division by zero, invalid shift, and a nonrepresentable result return stable
canonical failure tags; saturation is used only by an input verb that names
it explicitly.

Products, sums, dot products, squared lengths, and integer square roots whose
operands are stored Q23.8 or Q1.14 values use checked signed 64-bit
intermediates. Exact rational comparison in collision interval clipping and
SAT depth comparison uses checked signed 128-bit two's-complement semantics.
The WGSL baseline implements these as reviewed two-word and four-word `u32`
helpers respectively. Overflow at either width is
`CanonicalFailure::ArithmeticOverflow`; an implementation may not reassociate,
reduce early, saturate, or use floating point to avoid it.

Orientation is a canonical quantized unit quaternion
`QuatQ14Wire([i16; 4])`, component order `(x,y,z,w)`, scale
`S = 16,384`. Registration and composition:

1. registration treats the four input components as one integer vector;
   composition computes the raw Q2.28 Hamilton product in this exact order:
   `x=aw*bx+ax*bw+ay*bz-az*by`,
   `y=aw*by-ax*bz+ay*bw+az*bx`,
   `z=aw*bz+ax*by-ay*bx+az*bw`,
   `w=aw*bw-ax*bx-ay*by-az*bz`, evaluating terms left-to-right;
2. calculate the exact positive integer squared norm
   `N = x*x+y*y+z*z+w*w` with checked signed 64-bit products/sums; reject
   `N == 0`;
3. for each component magnitude `a`, compute the exact
   `round_ties_even(a*S/sqrt(N))` without first truncating `sqrt(N)`: a
   15-step binary search finds the largest `q in 0..=S` for which
   `q*q*N <= a*a*S*S`; compare `4*a*a*S*S` with
   `(2*q+1)*(2*q+1)*N` in checked `i128`, choosing `q`, `q+1`, or the even
   one on equality (`q == S` cannot increment), then restore the component
   sign;
4. require every result component to fit `i16` and require the quantized-unit
   shell
   `abs(rx*rx+ry*ry+rz*rz+rw*rw - S*S) <= 32,769`; failure of this
   postcondition is `CanonicalFailure::InvalidOrientation`;
5. choose the sign whose first nonzero component in `(w,x,y,z)` is positive;
6. apply the same procedure after every composition.

This comparison is the normative square-root rounding algorithm; an
implementation may not substitute division by `isqrt(N)`. In particular,
registration input `(1,1,0,0)` normalizes to
`(11585,11585,0,0)` rather than remaining length `sqrt(2)`. The shell bound
follows from rounding four exact normalized components by at most one half each.
Inverse negates `(x,y,z)` and repeats sign canonicalization; it does not
renormalize because the squared norm is unchanged. The algorithm is closed
over `QuatQ14Wire` and cannot accumulate backend-dependent drift.

Canonical vector rotation does not assume that the quantized components have
an exactly representable Euclidean length. From the stored components it
recomputes `D = x*x+y*y+z*z+w*w` and builds this signed rational rotation
numerator:

```text
[ D-2(yy+zz)   2(xy-wz)    2(xz+wy)   ]
[ 2(xy+wz)     D-2(xx+zz)  2(yz-wx)   ]
[ 2(xz-wy)     2(yz+wx)    D-2(xx+yy) ]
```

The denominator of every entry is the same exact positive `D`. This is the
scale-independent quaternion rotation formula, so before the final
fixed-point rounding the rational transform is orthogonal even when the
stored quaternion lies anywhere in the permitted quantized-unit shell. Each
numerator term is calculated exactly in `i64`. For each output component, the
three numerator×vector products are checked and summed left-to-right in the
displayed order, then divided once by `D` with round-to-nearest, ties-to-even.
Inverse rotation uses the transpose of this same numerator and denominator; it
does not rebuild a second matrix from a rounded inverse. Placement is exactly
`world = translation + rotate(orientation, local - pivot)` and its inverse is
`local = pivot + rotate_transpose(orientation, world - translation)`, with a
checked operation at every subtraction and addition. Collision, CPU oracle,
WGSL, persistence verification, and replay all use this sequence.

The declared 4,095-cell maximum radius makes the worst representable
one-component orientation quantization step, including final Q23.8 transform
rounding, less than one cell. Retained generated proofs cover the unit-shell
postcondition, rational orthogonality, transpose inverse, composition closure,
and this displacement bound. Float transforms are one-way derived
presentation values and are never accepted back as canonical placement.

```rust
pub struct Q23_8(pub i32);
pub struct WorldPointQ(pub [Q23_8; 3]);
pub struct WorldVectorQ(pub [Q23_8; 3]);
pub struct WorldAabbQ {
    pub min: WorldPointQ,
    pub max: WorldPointQ,
}
pub struct SegmentQ {
    pub start: WorldPointQ,
    pub end: WorldPointQ,
}
pub struct QuatQ14(pub [i16; 4]); // x, y, z, w
pub struct PlacementQ {
    pub translation: WorldPointQ,
    pub orientation: QuatQ14,
}
```

`WorldAabbQ` is half-open. Each constructor applies the arithmetic and
orientation validation above rather than accepting raw unchecked fields.

## Canonical bytes and commitments

### TECH-008 — Canonical wire encoding

Implements: REQ-017, REQ-028, REQ-032, REQ-034, REQ-038

Canonical encoding version `moria-canonical-v1` is a hand-written,
schema-tested binary format:

- unsigned and signed integers are fixed-width little-endian two's complement;
- booleans and enums are `u8` tags with rejected unknown values;
- digests are exactly 32 bytes;
- sequences use a `u32` element count and elements in their specified order;
- optional fields use a `u8` presence tag followed by the value;
- no platform-sized integer, float, string, map, implicit padding, or
  serde-derived layout is allowed in canonical bytes;
- every decoder rejects trailing bytes, nonminimal variants, excessive
  lengths, invalid tags, and arithmetic overflow.

Human labels are bounded UTF-8; correlation metadata is a bounded ID and opaque
byte payload. Both are noncanonical and do not appear in replay identity or
hashes. Contract, schema, arithmetic, shader, and hash-domain versions are
fixed digests in genesis. CPU encoding and WGSL wire layout have byte-for-byte
fixtures for every record.

### TECH-009 — Merkle commitment

Implements: REQ-001, REQ-017, REQ-028, REQ-032, REQ-034, REQ-038

BLAKE3-256 is the canonical hash algorithm, implemented with 32-bit operations
on CPU and WGSL. Every node hashes:

```text
"moria/v1/<domain>" || canonical_length || canonical_payload
```

Distinct domains cover genesis, material registry, base source, brick,
scar-leaf, radix-node, volume metadata, simulation domain, allocator state,
participant commitment, outcome list, tick batch, tick state, and world root.
The world root combines child commitments in stable ID/key order. Derived
presentation, lifecycle cache state, physical slots, receipt IDs, timings, and
telemetry are excluded.

The canonical material-registry payload contains material ID and occupancy
class/threshold. Surface style and asset handles are in the separately
versioned derived-presentation registry and are excluded from the world root.

Hashing is incremental. A changed brick recomputes its leaf and 26 four-bit
radix ancestors; a changed volume recomputes its volume leaf and the world
registry path. Unchanged node hashes are retained. A tick reports changed leaf
and node counts, and a test fails if a one-brick fixture schedules unrelated
volume hashes.

## Sparse authoritative state

### TECH-010 — Logical sparse representation

Implements: REQ-001, REQ-003, REQ-004, REQ-014, REQ-018, REQ-029

Each volume is the immutable tuple:

```text
VolumeState {
  id, kind, domain, base_authority, placement, revision,
  scar_root, simulation_regions, retired
}
```

`kind` is `Static` or `Dynamic`; static placement cannot change after genesis.
The base authority describes homogeneous content or a content-addressed brick
manifest. The scar root is a persistent 4-bit radix tree keyed by:

```text
VolumeId:u64 || zigzag(bx):u32 || zigzag(by):u32 || zigzag(bz):u32
```

The 104-bit key has exactly 26 radix levels. A leaf is either a uniform-cell
override or a complete canonical brick. Absence means “obtain this brick from
the exact base authority,” never “empty.” A scar leaf is omitted only when its
payload is byte-identical to verified base content.

The world root includes sorted immutable registries, volume roots and
placements, canonical simulation-domain union, `next_volume_serial`,
participant commitments (including their ordered canonical RNG-state
commitments), current tick, and contract identities. Runtime residency and
readiness are a separate cache indexed by
`(base digest, volume, brick)`.

### TECH-011 — Deterministic tick ordering and conflict rules

Implements: REQ-011, REQ-017, REQ-027, REQ-031, REQ-033, REQ-036, REQ-043

Only `current_tick + 1` is eligible. A sealed `TickBatch` contains bounded
inputs whose unique key is `(phase:u8, source_id:u32, source_sequence:u32)`.
The fixed phase order is:

0. opaque participant input delivery;
1. volume create/retire and simulation-domain activation/deactivation;
2. placement changes;
3. direct matter commands;
4. participant-proposed ordinary placement or matter commands.

Inputs are sorted lexicographically by this key. Duplicate keys reject the
whole batch before admission. Source sequence is explicit consumer data; queue
arrival never supplies it. Direct inputs use their registered high-bit-clear
`InputSourceId`. Participant effects use
`0x8000_0000 | ParticipantId` as the order source in a disjoint namespace and
a bounded local sequence. Participant commitments are derived products, not
batch inputs; they combine separately in `ParticipantId` order.

Participant preparation reads `State[t]` plus its phase-zero input. It does not
observe same-tick lifecycle, placement, or direct-matter effects; all such
effects, including its own proposals, compose into `State[t + 1]`. This
read-before-write rule is part of the transition version. It also cannot
observe another participant's same-tick state, effects, or opaque events.
Registration rejects a dependency declaration or adapter requiring a
same-tick predecessor. V1 has no participant DAG, handoff pass, prior-feedback
buffer, or conflict callback.

All revision and source-hash preconditions are evaluated against `State[t]`.
Eligible successful commands then compose in canonical order on a staged
state; for overlapping writes the later canonical command sees the earlier
staged cell. A failed command contributes a canonical outcome at its order
position but no writes or revision advance. Tick-global inability to bind
content, participants, arithmetic contract, or canonical resources yields
`NoAdvance`; there is no partial tick publication.

This is also the complete participant-effect conflict policy. Participant
effects occupy phase 4 in `(ParticipantId, local_sequence)` order. Overlap is
legal and composes by the rule above; stale or otherwise unmet preconditions
fail only that effect. V1 adds no conflict graph, ownership lock, handoff
buffer, arbitration callback, or automatic retry.

### TECH-012 — Atomic mutation and revision rules

Implements: REQ-011, REQ-017, REQ-025, REQ-033

The canonical matter commands are `Erase`, `Place`, and `Patch`. Each targets
one live volume and at most 64 bricks / 32,768 cells.

- `Erase` applies a bounded local AABB, sphere, or stamp mask and either sets
  selected cells to canonical empty or subtracts an explicit Q8.8 density
  amount with specified saturation at empty.
- `Place` applies the same shapes and replaces selected cells with one valid
  `CellWire`.
- `Patch` supplies sorted unique `(local_cell, CellWire)` pairs.

Shapes are discretized by the Q23.8 integer rules. Empty target sets are valid
no-op outcomes and do not advance a revision. All nonempty targeted cells,
base bricks, destination slots, and output sizes are resolved before any new
root is constructed. A command-level validation or capacity failure marks the
command failed and writes nothing. A successful nonempty command advances its
volume revision exactly once, even across many bricks. Create, retire, and a
dynamic placement change likewise advance only their named volume lifecycle
revision. `u64` revision exhaustion is a typed failure.

### TECH-013 — Multi-phase construction and atomic publication

Implements: REQ-001, REQ-005, REQ-011, REQ-033, REQ-035

A tick attempt has these ordered phases:

1. decode and structural validation on CPU before ownership transfers;
2. pin `State[t]`, exact base chunks, participant resources, and pool permit;
3. run participant preparation against the pinned source commitment;
4. stable-sort and validate direct and proposed effects;
5. mark touched bricks and compute exact capacities with checked prefix scans;
6. materialize complete old brick values into unreferenced work slots;
7. apply each brick's commands in canonical order;
8. hash changed leaves and copy-on-write radix ancestors;
9. validate all diagnostic, overflow, participant, and output records;
10. encode outcomes and calculate the new world root;
11. await GPU completion and bounded outcome/hash mapping;
12. deliver one generation-tagged candidate envelope through the reserved
    render-to-main completion bridge, then atomically swap the main-world
    `FrontierBundle` containing the GPU root token and participant state
    tokens, confirm the receipt, append replay records, and emit observations.

Dependent GPU phases are ordered dispatches on one queue. No shader uses a
cross-workgroup spin protocol. Before step 12, only private slots refer to the
candidate root. Any failure, missing bridge reservation, or device-generation
mismatch discards/retire-queues those slots and leaves `State[t]`, its
revisions, participant state, snapshots, and hash live. Step 12 occurs in the
exclusive main-world publication system specified by TECH-032. The root token
names already completed device objects retained in the render world, so
subsequent extracted work can use it without a second render-world “live”
swap. Readers acquire either the old or new immutable `FrontierBundle`, never
a mixture.

## Snapshot sharing and rollback

### TECH-014 — Persistent roots and bounded rollback window

Implements: REQ-018, REQ-029, REQ-035, REQ-037, REQ-043

`RollbackConfig.capacity_ticks` defaults to 32, must be at least 20, and is
bounded above by the configured canonical-memory budget. Each confirmed
frontier retains:

```text
tick, world_root_hash, GPU root handle, immutable metadata root,
tick-batch digest, outcome digest, participant commitments,
opaque participant state tokens and snapshot metadata
```

The frontier is O(changed bricks × radix depth) additional logical state.
Installing one is an O(number of registries + participants) root-handle swap;
it does not enumerate material bricks. Active live, retained, replay, query,
checkpoint, and participant leases pin roots. A root leaves the window only
after capacity eviction and no pin remains.

If 20 frontiers cannot be retained under the declared canonical-state budget,
genesis fails. Once running, deterministic logical-budget preflight prevents a
tick from confirming if it would violate the minimum retained window. It never
evicts a reachable frontier based on completion timing.

### TECH-015 — Stable compaction and physical reclamation

Implements: REQ-004, REQ-018, REQ-029, REQ-033, REQ-037

Candidate keys and keep predicates use fixed slots followed by portable
`mark -> hierarchical exclusive scan -> scatter`. Tile width is 128
invocations with two elements per lane. Tile totals are recursively scanned in
separate ordered dispatches. Inactive lanes participate in every barrier with
the identity value.

Every result reports `total`, `written`, and `overflowed`; overflow aborts the
candidate rather than truncating it. Sorting is stable LSD radix sort over
fixed-width keys, four bits per pass. No atomic append order enters canonical
state.

Physical nodes and bricks have `(slot, generation)` handles. Reclamation first
removes all new references, then waits for root pins and the queue completion
of every prior reader, then increments the generation and returns the slot.
Generation wrap permanently retires the slot. A stale handle is rejected
before encoding. Physical free-list order is deliberately noncanonical; all
canonical resource outcomes are decided against logical configured budgets
before physical allocation.

### TECH-016 — Coordinated participant frontier

Implements: REQ-006, REQ-029, REQ-030, REQ-033, REQ-035, REQ-037, REQ-043

Every participant registers exactly one strategy:
`PerTickSnapshot { max_bytes }` or
`ReconstructibleFromCanonicalStateAndLog { max_replay_ticks }`.
Its canonical state is participant-owned in meaning and representation, but it
must be encapsulated in an immutable opaque state token whose lifetime Moria
can pin. There is no adapter-global mutable canonical state. A token is bound
to `(participant, contract, tick, world_root, commitment,
device_generation?)`, and only the originating adapter may inspect it.

For every attempted tick the adapter receives a lease to the source token,
source root hash, bounded input slice, and canonical artifact leases. It
constructs a new uninstalled token and returns a bounded ordered effect list,
a bounded ordered opaque event list, a 32-byte participant commitment, and,
for each declared RNG stream, the canonical RNG-state commitment specified
below. Effects are ordinary commands and have no privileged mutation path.
Events are participant-owned output carried in canonical participant records
and the confirmed tick receipt; they never enter Moria's observation stream or
feed another participant in the same tick.
Preparing a token may not mutate the source token. A tick confirms only when
the exclusive coordinator installs one immutable `FrontierBundle` containing
the candidate root and every prepared participant token. Before that swap all
tokens are private; after it the old bundle remains pinned for readers and
rollback. Thus participant installation cannot partially commit independently
of substrate publication.

A rollback or correction creates a private `CorrectionContext` containing
tokens restored from the target frontier. Each replayed tick produces the next
private token in that context. Success installs only the final bundle; failure
or cancellation drops every private token after its CPU/GPU leases drain and
leaves the original live bundle untouched. Snapshot restore and reconstruct
operations therefore return staged tokens; they never mutate an in-place
participant. Device-generation loss terminally invalidates staged GPU tokens
from that generation. Old-generation completion may release resources but
cannot enter a live or correction bundle.

Every CPU and GPU participant operation uses the same bounded lifecycle:

```text
Reserved(source pin + destination bytes)
  -> Preparing | RestoringSnapshot | Reconstructing
  -> PreparedPrivate
  -> InstalledInFrontier
  \-> Failed -> Aborting -> DrainingLastUse -> Reclaimed
```

Only `PreparedPrivate` may enter a bundle, and installation is the host pointer
swap rather than an adapter callback. A sink completion moves to
`PreparedPrivate`; duplicate completion is rejected. Cancellation is accepted
only before preparation is submitted. After submission it suppresses
installation, drains the token, and returns its fixed permit. Descriptor maxima
for source state, destination state, effects, opaque events, snapshot bytes,
replay ticks, and artifact leases are reserved before the operation; no
callback may grow them. Effects and events use separate Moria-owned fixed-slot
sinks with exact aggregate byte counters. A completion cannot return a
consumer-owned `Vec`, map, diagnostic string, or allocation; it fills those
sinks and one bounded diagnostic record.

For `PerTickSnapshot`, each prepared token also exposes immutable
`SnapshotMetadata { uncompressed_bytes, digest }` and a bounded asynchronous
export operation. Moria pins the token for the rollback window. Participant
code owns the snapshot schema, while Moria owns the lifecycle of the handle and
copies verified snapshot bytes into `CheckpointStore` for durable checkpoints
as specified by TECH-045. The participant is not allowed to substitute a
durable external locator. `ReconstructibleFromCanonicalStateAndLog` tokens
declare their maximum replay prefix and must reproduce every intermediate
commitment from canonical genesis/frontier plus exact log bytes. A durable
checkpoint owns content-addressed copies of the required replay records as
specified by TECH-044 through TECH-049; an in-memory range or digest without
those bytes is not a reconstruction source.

Moria itself has no RNG algorithm or RNG state. A participant using randomness
that can affect canonical output must list every stream in its genesis
descriptor:

```text
RngContract {
  stream_id: nonzero u32,
  algorithm_id: 16 bytes,
  algorithm_version: u32,
  algorithm_contract_digest: 32 bytes,
  state_schema_digest: 32 bytes,
  seed_bytes: 0..=64 canonical bytes
}
```

The referenced algorithm contract must completely specify seed decoding,
state bytes, next-state/output transition, rejection sampling, and exhaustion.
Each participant commitment contains, in `stream_id` order,
`(stream_id, state_byte_len:u32, BLAKE3(state_bytes))`. Snapshot bytes contain
the complete state bytes for every stream. A reconstructible participant must
derive them from the descriptor seed and canonical log and reproduce those
digests. OS entropy, wall clock, thread identity, and undeclared streams are
conformance failures. These descriptors and state commitments are included in
genesis, world hashing, retained frontiers, checkpoints, replay, restore, and
qualification evidence; a 32-byte participant commitment alone is not treated
as an RNG specification.

Missing, oversized, wrong-source, duplicate, late-generation, or divergent
products cause `NoAdvance` or rollback failure. Participant completion order
is irrelevant: products occupy preassigned `ParticipantId` slots and are
combined in ID order. Moria never interprets participant behavior or RNG
meaning, but it validates every declared bound, identity, digest, and lifecycle
transition. The descriptor's closed `ParticipantFailurePolicy` controls
whether such a failed canonical operation leaves the world retryable at its
last frontier or terminally fails it; TECH-029 defines the complete matrix.
No policy can omit the participant, reuse its prior token as the next tick's
state, synthesize an empty commitment, or publish a partial bundle.
