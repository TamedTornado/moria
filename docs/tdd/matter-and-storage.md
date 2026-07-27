# Matter, Storage, Mutation, and Collision

## 1. Authoritative cell model

A logical cell is:

```rust
#[repr(C)]
struct PackedCell(u32);
// bits 0..=15 material ID, bits 16..=23 coverage, bits 24..=31 reserved = 0
```

Coverage is an unsigned scalar from 0 (no matter) to 255 (full coverage at the
cell sample). Empty is exactly material 0 and coverage 0. Nonempty material
with zero coverage is invalid. Reserved bits must be zero in sources,
commands, persistence, and GPU output.

Coverage provides shape information for smooth surface reconstruction without
changing collision identity. Collision occupancy uses the registered
material's threshold. Material identity at a mixed boundary is the nonempty
sample selected by deterministic nearest-surface rules; no blended gameplay
material is invented. More elaborate visual blending remains derived
presentation data.

Logical truth is the exact `PackedCell` lattice plus volume placement at a
revision. Compression, page location, mesh vertices, and acceleration data are
not truth.

## 2. Spatial partition

Each finite `VolumeSpec::domain` is partitioned into aligned 8×8×8 leaf bricks
(512 cells). Edge bricks have a validity mask implied by the domain. A volume
domain may span at most `2^21` cells per axis and must fit signed `i32`
coordinates; checked arithmetic rejects configurations outside both rules.

Bricks are grouped into 8×8×8-brick regions (64 cells per axis). Region is the
unit of interest lifecycle, source requests, persistence indexing, and
telemetry. Query and mutation bounds remain cell-exact and are not expanded in
reported results.

The 8-cell leaf was selected to keep a full cell payload at 2 KiB, allow
bounded copy-on-write transactions, and avoid excessive page-table fanout.
Changing either partition constant is a persistence and shader-layout change
requiring golden migration tests and benchmark evidence.

## 3. Sparse representation

The GPU owns:

1. a per-world volume descriptor table;
2. a two-level sparse region/brick page table;
3. descriptor pools for `Empty`, `Homogeneous`, and `Mixed` bricks;
4. a fixed-capacity mixed-brick pool;
5. transaction page pools;
6. per-ready-region occupancy min/max hierarchy;
7. revision-tagged query/result rings; and
8. revision-tagged presentation buffers described elsewhere.

A brick descriptor is one of:

- `Empty`: implicit empty, no payload;
- `Homogeneous(PackedCell)`: one payload word;
- `Mixed(PageIndex)`: exactly 512 packed cells in the private pool; or
- `Unavailable`: an internal sentinel that is never interpreted as empty.

Untouched homogeneous regions therefore cost region/brick descriptors, not
raw cells. Fully cold regions have no detailed GPU descriptor and retain only
CPU lifecycle/source/scar metadata. Sparse page-table entries are allocated
only for requested regions, scars, and nonhomogeneous source results.

The CPU may retain:

- immutable world/material/volume/source descriptors;
- region lifecycle and revision metadata;
- compact `Empty`/`Homogeneous` source answers;
- canonical sparse scars and dirty flags;
- bounded in-flight patch/source staging; and
- bounded result/readback buffers.

It must not retain every mixed brick after upload. Debugging and validation
obtain bounded public snapshots through normal query paths.

## 4. Base materialization

`BaseContentSource` is an object-safe, asynchronous consumer trait:

```rust
pub trait BaseContentSource: Send + Sync + 'static {
    fn descriptor(&self) -> BaseDescriptor;
    fn request_region(
        &self,
        request: BaseRegionRequest,
    ) -> BoxFuture<'static, Result<BaseRegion, SourceError>>;
}
```

A request names world, volume, exact aligned region bounds, expected
`BaseIdentity`, maximum encoded/decoded bytes, and cancellation token. It never
asks the source for an unbounded stream. The result consists of 512 canonical
brick answers (`Empty`, `Homogeneous`, or mixed cell payload), edge validity,
and content proofs described in `persistence.md`.

Materialization pipeline:

1. Admission pins a cold/requested region and reserves source, GPU, and
   readback capacity.
2. Runtime transitions `cold -> requested -> materializing`.
3. The source future runs on Bevy's I/O task pool with a timeout/retry policy.
4. CPU validation checks identity proof, exact bounds, payload lengths,
   reserved bits, material IDs, empty semantics, and digest.
5. Valid payloads upload to staging; a compute pass expands only mixed bricks,
   applies canonical scars, and builds occupancy aggregates.
6. A validation pass checks pool indices, cell domains, and scar application.
7. After the GPU fence and compact completion record, runtime publishes
   `ready(revision)` once.

Source timeout or retryable unavailability retains `requested` until retry
policy exhausts or the operation deadline is cancelled. Invalid data/proof is
nonretryable for that source identity and transitions the region to `failed`.
No descriptor becomes queryable before step 7.

Concurrent requests for the same region and required revision coalesce. Their
interest/operation pins remain independent, and one cancellation cannot cancel
work still needed by another.

## 5. Atomic matter mutation

One admitted mutation targets one volume and at most the configured cells and
bricks. It follows a copy-on-write transaction:

1. **Admission:** validate structure, current revision precondition, material
   IDs, target domain, queue/pool worst case, and source materializability.
2. **Pin/materialize:** pin every intersecting region and make its committed
   revision ready. If the revision changed before staging, an exact
   precondition fails as conflict; an unconditioned command is restamped to the
   latest revision before staging.
3. **Reserve:** reserve transaction descriptors and enough pages for every
   brick that could become mixed. Failure here terminates with no effect.
4. **Stage:** GPU kernels copy current descriptors/pages to transaction-owned
   pages and apply the effect only there. Live page-table entries are unchanged.
5. **Validate:** a separate pass validates every staged descriptor, material,
   index, affected-bounds reduction, scar capacity, and command checksum. A
   fault bit can force failure here in conformance builds.
6. **Prepare scar:** GPU compares staged cells with base cells and emits a
   bounded canonical delta for the affected bricks. The CPU verifies the
   compact delta and reserves its scar index entry before commit.
7. **Commit:** one ordered compute dispatch swaps all affected page-table
   descriptors, updates occupancy aggregates, and writes one completion record
   containing prior/new revision and checksum. Public query dispatches for that
   volume are ordered either before or after this dispatch. There is no public
   dispatch interleaving inside it.
8. **Publish:** only after the completion fence does runtime advance CPU
   revision metadata, install the scar delta, emit one observation, and mark
   presentation stale/building.
9. **Reclaim:** old pages are retired after all earlier GPU readers complete;
   transaction pages are discarded on any failure before commit.

The commit dispatch does no fallible allocation or content validation. CPU
publication after the completion fence is the public commit point. A GPU device
loss before publication invalidates the entire private GPU allocation, so the
unpublished command fails with no committed effect; no later query result from
that device is published. A device loss after CPU publication retains the
already installed scar/revision and reports that known committed effect.
Runtime rebuilds GPU authority from base plus CPU scars before becoming ready
again. Thus even device loss never invents an ambiguous partial commit.

If a command changes no cells, steps 6–8 return `changed = false`, retain the
revision, and avoid scar/presentation/observation work.

## 6. Placement and registry commits

Placement changes use the same ordered control queue but stage only a new
volume transform and top-level spatial-index entry. The commit updates both
atomically and increments the volume revision. In-flight world-space queries
are ordered entirely before or after the new placement.

Create reserves identity, source, registry, budget, and persistence entries
before its catalog commit. It becomes visible at volume revision 1 and the new
catalog revision. Retire enters `retiring` visibly but the catalog removal
commits only after:

- no operation pins the volume;
- dirty scars and the retirement record satisfy the configured durability
  policy; and
- no checkpoint cut refers to work not yet copied.

The retiring state rejects new commands and interests but allows existing
queries to finish at their captured revision. The final removal advances
catalog revision once and emits a retired observation.

## 7. GPU query execution

World-space queries first traverse a CPU-maintained volume AABB index containing
only identity, transform, domain AABB, and committed revision. It is not an
occupancy index. Candidate volumes are sorted by ID and their transforms/bounds
are uploaded in the query packet.

GPU query kernels:

- transform shapes/rays into each candidate's local coordinates;
- traverse region and brick occupancy summaries;
- read authoritative mixed/homogeneous cells;
- write fixed-size fact records to a per-query slice; and
- set explicit overflow, unavailable, stale-revision, and completion flags.

An overflow flag makes a complete query fail with `ResultCapacityExceeded`.
For explicitly partial queries, it produces a partial result and exact
uncovered reason. Result buffers are copied to a bounded map-read ring and
decoded only after the fence. Every completion echoes the dispatch revision
set; a mismatch is an internal invariant failure, never silently upgraded.

Region query payload size is bounded at admission. Large consumer reads are
implemented by explicit pagination: each page is a separate query with its own
revision. Moria does not promise a coherent unbounded snapshot across pages.

## 8. Collision algorithms

Collision reads the same cell representation as sample/region queries:

- Point and region occupancy inspect exact occupied samples.
- Ray trace uses 3D DDA through local cells and reports ordered occupied
  intervals; rotated volume hits are transformed back to world space.
- Shape overlap prunes with region/brick occupancy and tests conservative
  cell coverage surfaces.
- Sweeps use broad-phase swept AABBs, local-space conservative advancement,
  and at most 16 bisection iterations to reach configured tolerance.
- Surface normals derive from the local coverage gradient when available and
  fall back to the entered cell face. They are transformed by placement
  rotation.

World overlap is the union of all volume facts. Overlapping cells are not
merged, prioritized, or resolved. Results include volume/material identity,
cell, world contact/interval, normal when defined, and exact volume revision.

For occupancy and sweep, conservative means Moria may report a possible contact
near a partially covered cell but must not miss coverage at or above the
occupancy threshold. The result includes `ContactCertainty::Exact` or
`Conservative`. Consumers own any follow-up policy.

Collision correctness is checked against the deterministic reference model
for bounded fixtures. The reference model is `#[cfg(test)]`/conformance-only
and cannot be selected as a product runtime backend.

## 9. Determinism and shader interfaces

Truth results are deterministic for the same ordered requests, source bytes,
and adapter-conformant integer operations. GPU mutation and occupancy kernels
use integer cell math and deterministic prefix sums; they do not use
floating-point atomics. Floating collision facts are tolerance-tested and
carry precision metadata.

Every host/shader shared struct has:

- explicit `#[repr(C)]` host layout using POD-compatible fields;
- a WGSL counterpart with recorded size/alignment/offsets;
- compile-time host assertions;
- shader reflection/schema tests; and
- a versioned `LayoutId` included in persistence/evidence configuration.

Shader compilation errors fail startup. A missing optional presentation
pipeline may fail presentation only; a missing matter/query/mutation pipeline
fails the world before use.
