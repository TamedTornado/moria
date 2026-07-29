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
clear; RNG-stream IDs are nonzero `u32` values. Each is unique in its registry.
`VolumeId` is a nonzero `u64`. Genesis volumes may claim explicit unique IDs;
canonical `next_volume_serial` starts one above their maximum. Sorted
post-genesis create commands allocate and increment it. IDs are never reused,
including after retirement or rollback-window reclamation.

Duplicate genesis IDs, zero reserved IDs, exhausted counters, and a reference
to an absent or retired ID are typed validation failures. Physical node,
buffer, slot, entity, task, and submission IDs never appear in canonical
encoding, persistence, replay, observations, or the public identity types.

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

Orientation is a canonical unit quaternion `QuatQ14Wire([i16; 4])`, component
order `(x,y,z,w)`, scale 16,384. Registration and composition:

1. calculate products and norm with checked signed 64-bit integer arithmetic;
2. normalize using integer square root and ties-to-even division;
3. choose the sign whose first nonzero component in `(w,x,y,z)` is positive;
4. reject a zero norm or a component outside `i16`;
5. apply the same procedure after every composition.

Inverse negates `(x,y,z)` and repeats sign canonicalization. The algorithm is
closed over `QuatQ14Wire` and cannot accumulate backend-dependent drift. The
declared 4,095-cell maximum radius makes the worst representable one-step
orientation quantization displacement less than one cell; a generated
exhaustive-bound proof is retained with the arithmetic tests. Float transforms
are one-way derived presentation values and are never accepted back as a
canonical placement.

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

Human labels and correlation metadata are noncanonical bounded UTF-8 and do
not appear in replay identity or hashes. Contract, schema, arithmetic, shader,
and hash-domain versions are fixed digests in genesis. CPU encoding and WGSL
wire layout have byte-for-byte fixtures for every record.

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
participant commitments, declared participant RNG commitments, current tick,
and contract identities. Runtime residency and readiness are a separate cache
indexed by `(base digest, volume, brick)`.

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
read-before-write rule is part of the transition version.

All revision and source-hash preconditions are evaluated against `State[t]`.
Eligible successful commands then compose in canonical order on a staged
state; for overlapping writes the later canonical command sees the earlier
staged cell. A failed command contributes a canonical outcome at its order
position but no writes or revision advance. Tick-global inability to bind
content, participants, arithmetic contract, or canonical resources yields
`NoAdvance`; there is no partial tick publication.

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
12. atomically swap the host-visible live root handle, confirm participant
    commitments, append replay records, and emit observations.

Dependent GPU phases are ordered dispatches on one queue. No shader uses a
cross-workgroup spin protocol. Before step 12, only private slots refer to the
candidate root. Any failure or device-generation mismatch discards/retire-
queues those slots and leaves `State[t]`, its revisions, snapshots, and hash
live. Step 12 occurs in one exclusive coordinator system; readers acquire
either the old or new root, never a mixture.

## Snapshot sharing and rollback

### TECH-014 — Persistent roots and bounded rollback window

Implements: REQ-018, REQ-029, REQ-035, REQ-037, REQ-043

`RollbackConfig.capacity_ticks` defaults to 32, must be at least 20, and is
bounded above by the configured canonical-memory budget. Each confirmed
frontier retains:

```text
tick, world_root_hash, GPU root handle, immutable metadata root,
tick-batch digest, outcome digest, participant commitments
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
For every attempted tick it receives the source root hash, its bounded input
slice, and canonical artifact leases. It returns a bounded ordered effect list
and a 32-byte commitment. Effects are normal commands and have no privileged
mutation path.

A tick confirms only after every participant accepts the same frontier and its
commitment is included in the world root. Snapshot participants durably own
their opaque snapshot bytes; Moria pins a receipt/handle and verifies its
digest and size. Reconstructible participants receive replay bytes and must
reproduce commitments. Missing, oversized, late-generation, or divergent
participant products cause `NoAdvance` or rollback failure; Moria never
interprets or repairs the participant state.

Participant completion order is irrelevant. Products occupy preassigned
participant-ID slots and are combined in ID order.
