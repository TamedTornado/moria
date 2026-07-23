# Compact generation, indexing, persistence, and evidence data

## Scope

This file specifies the data changes required by Design R3-R12. Existing voxel
bytes, material predicates, sparse brick-delta behavior, query result types,
and presentation ownership remain unchanged unless explicitly listed.

## Coordinate and bounded-generation model

Existing scalar constants remain:

```rust
pub const Q8_UNITS_PER_METER: i32 = 256;
pub const VOXEL_EDGE_Q8: i32 = 64;          // 0.25 m
pub const BRICK_EDGE_VOXELS: i32 = 16;      // 4 m
pub const GENERATION_CELL_EDGE_BRICKS: i32 = 16;
pub const GENERATION_CELL_EDGE_VOXELS: i32 = 256;
pub const GENERATION_CELL_EDGE_Q8: i32 = 16_384; // 64 m
pub const GENERATION_CONTEXT_CELLS: i32 = 1;
pub const MAX_OWNER_CELLS_PER_REQUEST: u8 = 27;
```

`VoxelCoord`, `WorldPointQ8`, and `GenerationCellCoord` use signed `i32`
components. `BrickCoord` changes from fixed-region `i16` components to signed
`i32` absolute brick coordinates. Conversion checks the opened
`WorldBounds`; coordinate types do not carry implicit Product One bounds.
`GenerationCellCoord` is a public fixed-width value used by bounded activation
and diagnostics; `GenerationCellRequest` and `GeneratedCell` below are
crate-private runtime values and are not exported as an alternate read path.

```rust
pub struct GenerationCellCoord {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub struct GenerationCellRequest {
    pub cell: GenerationCellCoord,
}

pub struct GeneratedCell {
    pub identity_digest: [u8; 32],
    pub cell: GenerationCellCoord,
    pub procedural: CellProceduralDescriptors,
    pub objects: Vec<RegisteredObject>,
    pub object_index: CellObjectIndex,
    pub accounting: CellGenerationAccounting,
    pub result_digest: [u8; 32],
}
```

`WorldBounds` remains min-inclusive/max-exclusive. Validation requires:

1. all axes are nonempty;
2. min/max are aligned to `GENERATION_CELL_EDGE_Q8`;
3. Q8-to-voxel and voxel-to-brick/cell conversions fit `i32`;
4. the total generation-cell count fits in 48 bits; and
5. the bounds contain every authored anchor and its raw/dependency extent.

The cell ordinal is lexicographic x-fastest, then z, then y, relative to the
validated bounds:

```text
ordinal = local_x
        + cells_x * (local_z + cells_z * local_y)
```

All arithmetic is checked in `u64`; overflow is `WorldDefinitionError`, never
wrapping.

A core cell result owns only candidates anchored inside that cell. When
sampling/indexing the core, the runtime may request owner cells with coordinate
offsets `[-1, 1]` on each axis, clipped to world bounds. No generator call may
read an active-cell cache or request a second context ring. The generator uses
global integer coordinates, so a shared face/edge/corner evaluates identically
from either adjacent request.

Procedural overlap acceptance is also order-independent. Generation evaluates
the fixed candidate slots in the same clipped 27-cell neighborhood; when two
raw solid shapes conflict, the candidate with the lower
`(owner_cell_ordinal, candidate_ordinal)` wins and the other is rejected.
Authored-anchor conflicts are definition errors rather than silently rejected.
The validated context-extent cap guarantees that every possible cross-cell
conflict is visible in this neighborhood. A cell retains only its own accepted
objects, even though it evaluates bounded neighboring candidates to decide
them.

[Design R4, AC3-AC4; Analysis: hard-coded coordinate seam]

## Compact world definition and identity

The active checked-in input is `assets/config/substrate_world.ron`:

```rust
pub struct WorldDefinition {
    pub schema_version: u32,
    pub schema_digest: [u8; 32],
    pub generator_version: u32,
    pub generator_digest: [u8; 32],
    pub seed: u64,
    pub bounds: WorldBoundsConfig,
    pub terrain: TerrainParameters,
    pub materials_digest: [u8; 32],
    pub registered_objects: RegisteredObjectParameters,
    pub anchors: Vec<AuthoredAnchor>,
}
```

For reproducible repository tests, this definition uses the cell-aligned
half-open bounds `[-256 m, 256 m)` on x, y, and z: exactly 8 cells per axis and
512 possible cells. This is compact fixture/stress input, not a required world
extent or content population. Consumers may provide any validated finite
bounds.

`TerrainParameters` reuses the existing fixed-width integer terrain, geology,
cave, water, and material-generation inputs after removing ecology/route
constraints. `RegisteredObjectParameters` contains only algorithmic candidate,
shape, per-cell capacity, index, and context limits. It contains no species,
forest area, density ratio, canopy, route, or shipped-population count.

`AuthoredAnchor` is explicit bounded fixture truth:

```rust
pub struct AuthoredAnchor {
    pub key: String,
    pub owner_cell: GenerationCellCoord,
    pub transform: QuantizedTransform,
    pub shape: RegisteredObjectShape,
}
```

Anchor keys are nonempty ASCII kebab-case and strictly sorted/unique.
Transforms and shapes are integer/fixed-point. At most 64 anchors and at most
1 MiB of decoded anchor/stamp data are accepted in a product definition. The
substrate definition exercised by the smoke fixture includes one neutral
diagnostic arch/stamp; neither the cap nor the fixture creates a required
scenery category.

`assets/fixtures/substrate_smoke.ron` is a consumer-owned proof/viewer script,
not authoritative world truth:

```rust
pub struct BoundedFixture {
    pub schema_version: u32,
    pub fixture_digest: [u8; 32],
    pub expected_world_digest: [u8; 32],
    pub activation_steps: Vec<FixtureActivationStep>,
    pub probes: Vec<FixtureProbe>,
    pub edits: Vec<WorldEditCommand>,
    pub anchor_keys: Vec<String>,
}
```

Its lists are bounded, canonically sorted where order is not itself the test,
and domain-digested. `anchor_keys` may only reference anchors already present
in `WorldDefinition`; the fixture cannot add world truth. The fixture affects
workload/proof identity, not `WorldIdentity`.

Identity is explicit:

```rust
pub struct WorldIdentity {
    pub seed: u64,
    pub bounds: WorldBounds,
    pub schema_version: u32,
    pub schema_digest: [u8; 32],
    pub generator_version: u32,
    pub generator_digest: [u8; 32],
    pub parameters_digest: [u8; 32],
    pub anchors_digest: [u8; 32],
    pub world_digest: [u8; 32],
}
```

Digest rules are domain-separated SHA-256:

- `schema_digest = SHA256("moria/substrate-world-schema/v1")`;
- `generator_digest = SHA256("moria/substrate-generator/v1")`; any change that
  changes authoritative generated bytes or candidate ordering must increment
  `generator_version` and change this domain value;
- `parameters_digest` hashes seed-excluding bounds, terrain, material digest,
  object parameters, and all capacity values;
- `anchors_digest` hashes the strictly sorted anchors and the decoded
  authoritative bytes of referenced stamps; and
- `world_digest` hashes the literal domain
  `"moria/substrate-world-identity/v1"` followed by the seed, bounds, every
  version/digest above, in the declared order.

Canonical digest encoding is implemented once in
`generation::definition`; it writes unsigned integers big-endian, signed
integers as their two's-complement fixed-width bytes, length-prefixes UTF-8 and
vectors with `u32`, writes enum discriminants as documented `u8`, and rejects
values that exceed their fixed-width/capacity. RON whitespace, field order,
comments, file path, machine, presentation, viewer bindings, benchmark
arguments, and save deltas do not affect identity.

`moria-curate check` parses with `deny_unknown_fields`, validates canonical
sorting and all caps, recomputes all digests, and fails on mismatch. It does
not emit an object population. [Design R3, AC2]

## Stable registered-object identity

Generic public types replace scenery-specific types:

```rust
pub struct ObjectId {
    pub world_digest: [u8; 32],
    pub local: u64,
}

pub enum RegisteredObjectKind {
    Ellipsoid,
    Column,
    SparseStamp,
}

pub enum RegisteredObjectShape {
    Ellipsoid { radii_q8: [u16; 3], perturbation_key: u64 },
    Column { radius_q8: u16, height_q8: u16 },
    SparseStamp { asset_key: String },
}

pub struct RegisteredObject {
    pub id: ObjectId,
    pub kind: RegisteredObjectKind,
    pub transform_q: QuantizedTransform,
    pub shape: RegisteredObjectShape,
    pub anchor: VoxelCoord,
}
```

There is no `SpeciesId`, special ruin ID, route tag, or tree/canopy variant in
the generic contract. Optional presentation metadata is keyed separately by
`RegisteredObjectKind` and is non-authoritative.

Each owner cell produces candidate records in a deterministic fixed candidate
scan of at most 65,535 slots. Rejected candidates still occupy their
deterministic candidate ordinal; acceptance of an earlier candidate must not
renumber a later candidate. Accepted candidates are sorted by
`candidate_ordinal` and must fit:

```rust
pub const MAX_OBJECT_CANDIDATES_PER_CELL: u32 = 65_535;
```

The local part of the stable ID is collision-free within validated bounds:

```text
slot = candidate_ordinal + 1                 // 1..=65_535
local = (owner_cell_ordinal << 16) | slot
ObjectId = (world_digest, local)
```

`owner_cell_ordinal < 2^48`; local zero remains invalid/reserved. Authored
anchors use slots descending from 65,535 in sorted anchor-key order;
definition validation ensures the procedural scan and authored slot ranges do
not overlap. An ID is therefore a collision-free function of complete world
identity, bounds-relative owner cell, generator version, and fixed candidate
ordinal. IDs compare lexicographically by `world_digest` then `local`; a live
world rejects IDs with another world digest. A changed generator version
changes the world identity; IDs are stable only within an identical world
identity, as required for deterministic save/reload and activation.

Tests must prove no duplicates over every cell in the small fixture and
generated stress workload, and exact equality after arbitrary request
permutations, eviction, and reactivation. Hash truncation is forbidden as the
uniqueness mechanism. [Design R4, R6, AC3, AC5]

## Per-cell spatial indexes and overlap safety

`ObjectSpatialIndex<'a>` becomes an owned `CellObjectIndex`. It retains:

- sorted `RegisteredObject` values for one owner cell;
- fixed-size raw/dependency bounds records;
- sorted dependency-grid and sample-grid cells;
- exact retained-capacity byte accounting; and
- zero retained dependency-coordinate arrays.

The current 32 m dependency and 4 m sample grid sizes remain unless a measured
TDD revision changes them. Each query determines the bounded owner-cell
neighborhood from its query AABB, obtains at most 27 indexes, merges candidate
IDs, sorts/deduplicates them, and exact-filters analytic bounds/shapes. Output
is ascending `ObjectId`.

`RegisteredObjectParameters` carries the existing safety capacities after
generic renaming:

- maximum dependency/sample cells per object;
- maximum members per dependency/sample cell;
- maximum exact candidates and affected objects per edit;
- maximum dependency bricks per object;
- maximum retained bytes per generated owner cell; and
- maximum raw/dependency extent of one generation-cell context ring.

Exceeding a capacity is a typed generation/definition error and a failed proof.
It is not resolved by truncating candidates. The per-cell retained-byte limit,
active/context cell counts, cache bytes, and total retained index bytes are all
reported; there is no fixed whole-world 16 MiB claim.

The independent oracle does not call `CellObjectIndex`, reuse its grid-key
helpers, or inspect its candidate output. For a bounded fixture/stress cell it
iterates the source candidate list and analytic shapes directly, then sorts
IDs. It verifies:

- point sampling;
- AABB placement queries;
- dependency queries for edited coordinates;
- object/object and object/stamp overlap safety; and
- exact affected-object attribution.

An object whose raw/dependency bound crosses a cell face remains owned and
indexed only by its anchor cell; the querying core includes it by visiting
context indexes. Duplicate ownership is invalid. [Design R6-R7, AC5-AC6]

## Runtime cache, sparse truth, and accounting

`WorldStore` owns:

```rust
struct WorldStore {
    identity: WorldIdentity,
    generated_cells: BTreeMap<GenerationCellCoord, GeneratedCellRecord>,
    active_bricks: HashMap<BrickCoord, BrickRecord>,
    deltas: BTreeMap<BrickCoord, BrickDelta>,
    revision: u64,
}
```

`GeneratedCellRecord` tracks core references, context references, pins,
generation token, immutable generated payload, and retained-byte categories.
The cache key includes `world_digest` and cell coordinate. A record with zero
references/pins is evictable even when the cell has deltas; deltas remain in
the separate sorted map and overlay a regenerated base later.

For a query, base truth is evaluated from the bounded cell descriptors and at
most 27 cell object indexes, then the sparse delta overrides it. Inactive
in-bounds queries are permitted and bounded; they may synchronously run pure
descriptor evaluation but may not retain or enumerate the full world as a
side effect.

Accounting distinguishes:

- requested core cells;
- generated context cells;
- retained generated cells;
- active bricks and materialized voxel arrays;
- delta bricks and delta cells;
- object/index records and retained capacities;
- task queues and immutable task payload bytes; and
- derived CPU/GPU presentation bytes.

`retained_bytes` means live heap capacities owned by the named category plus
inline payload bytes, not serialized lengths or theoretical element sizes.
The proof records category sums and a reconciliation check that their sum
equals the declared total. OS/allocator overhead is explicitly outside this
counter and may be separately measured.

## Mutation attribution and derived fingerprints

Each committed batch publishes exact, ascending vectors:

```rust
pub struct EditBatchCommitted {
    pub request_id: u64,
    pub batch_index: u32,
    pub revision: u64,
    pub changed_bricks: Vec<BrickCoord>,
    pub affected_objects: Vec<ObjectId>,
}
```

`changed_bricks` contains only bricks with at least one base-relative delta
change in the batch. `affected_objects` is the exact set whose extraction
dependency intersects a changed coordinate, obtained through the production
index and verified by the independent oracle in F1. Both vectors are
deduplicated/sorted and bounded by mutation/index configuration. Net no-op
commands publish an explicit zero-work terminal record without incrementing
revision.

For proof and reconciliation, each registered object's derived state has a
fingerprint:

```text
SHA256(world_digest || object_id || authoritative_dependency_samples ||
       derived_payload_revision_and_bytes)
```

The production renderer need not expose mesh bytes publicly. A bounded
telemetry/proof hook reports `(ObjectId, revision, fingerprint, phase)`.
Objects outside the exact affected set must retain the same fingerprint and
revision; affected objects must either install a current-revision fingerprint
or an explicit current-revision tombstone.

## Sparse save format

The new accepted format is a zstd-compressed canonical JSON document with file
extension `.moria-delta.zst`. It uses only existing dependencies.

```rust
pub struct WorldDeltaFileV1 {
    pub schema: String,                 // "moria-substrate-delta"
    pub format_version: u32,            // 1
    pub world: WorldIdentity,
    pub saved_revision: u64,
    pub bricks: Vec<SavedBrickDelta>,
    pub delta_content_sha256: [u8; 32],
}

pub struct SavedBrickDelta {
    pub brick: BrickCoord,
    pub cells: Vec<SavedVoxelDelta>,
}

pub struct SavedVoxelDelta {
    pub local_index: u16,
    pub voxel: Voxel,
}
```

Bricks are strictly ascending; cells are strictly ascending and in
`0..4096`. A cell exists only when its current voxel differs byte-for-byte from
the regenerated base for the file's identity. `delta_content_sha256` uses the
same fixed-width domain-separated encoding rules over the sorted bricks/cells.
JSON uses struct field order, no pretty printing, no optional omitted fields,
and rejects unknown fields. zstd uses one documented fixed compression level
and no timestamp, so identical snapshots are byte-identical.

Save snapshots the delta map at one revision, encodes/writes to a sibling
temporary file, flushes/syncs, and atomically renames. A failed write leaves
the previous slot intact and reports failure. Load:

1. reads/decompresses with explicit compressed and decoded byte caps;
2. parses and validates schema, ordering, identity, checksum, bounds, local
   indices, materials, and base-relative non-no-op cells;
3. stages a complete `BTreeMap` without changing live truth;
4. swaps the map at one `FixedUpdate` boundary and increments revision once;
5. invalidates the symmetric difference of old/new deltas; and
6. completes only after active query/collision and derived reconciliation
   reach the new revision.

Unsupported format/schema/generator/world identity returns a typed error.
Derived mesh/object/water/debug bytes, generated cell caches, stress output,
machine data, and viewer state are forbidden. [Design R8, AC7]

## Substrate proof and benchmark evidence

Forest report types are replaced by:

```rust
pub struct SubstrateProofReport {
    pub schema: String, // "moria-substrate-proof"
    pub report_version: u32,
    pub status: EvidenceStatus,
    pub failure_reasons: Vec<String>,
    pub build: BuildProfile,
    pub world: WorldIdentity,
    pub fixture: WorkloadIdentity,
    pub stages: Vec<ProofStageEvidence>,
    pub correctness_digest_sha256: [u8; 32],
    pub accounting: ResourceAccountingEvidence,
    pub measurements: MeasurementEvidence,
    pub machine: MachineProfile,
}

pub enum EvidenceStatus {
    NotStarted,
    Partial,
    Failed,
    Passed,
}
```

`WorkloadIdentity` records fixture schema/version/digest and every request,
cell-count, object-count, distribution, edit, save, and oracle parameter.
`ProofStageEvidence` has a stable stage ID, status, input/output counts,
result digest, optional failure detail, and timing. Required F1 stages are:

1. `identity-and-definition`;
2. `bounded-generation-repeat`;
3. `activation-order-and-eviction`;
4. `stable-object-identities`;
5. `bounded-query-oracle`;
6. `broad-phase-oracle`;
7. `mutation-attribution`;
8. `unaffected-object-fingerprints`;
9. `derived-reconciliation`;
10. `save-reload`;
11. `resource-accounting`; and
12. `evidence-self-validation`.

`correctness_digest_sha256` hashes only normalized stage IDs, fixture/world
identity, deterministic inputs, exact result digests/counts, and pass/failure
outcomes. It excludes timestamps, elapsed times, host paths, and machine
identity. Matching inputs on Linux/macOS must match this digest.

`MachineProfile` keeps complete OS name/version/architecture, CPU, logical
cores, physical memory, Rust version, Cargo profile, and git commit. GPU,
backend, driver, resolution, and resident measurement are nullable with an
explicit availability flag because F1 is headless. No acceptance label or
specific machine is validated as functional correctness.

Report invariants:

- `Passed` iff every required stage is present and passed and
  `failure_reasons` is empty;
- `NotStarted`, `Partial`, and `Failed` always have sorted unique reasons;
- missing/null/contradictory/digest-mismatched evidence cannot deserialize or
  validate as `Passed`;
- stage IDs are unique and in required canonical order;
- workload/population totals reconcile with generated cell/object counts;
- accounting categories sum exactly; and
- serialization/writes preserve the existing atomic-output rules.

Benchmark reports use schema `moria-substrate-benchmark`, the same
`EvidenceStatus`, build/world/workload/machine identity, and nullable
scenario-specific measurements. Scenario status is never derived solely from
timing. Generated workload output belongs under `target/`, never `assets/`.

[Design R9-R12, AC8-AC11]
