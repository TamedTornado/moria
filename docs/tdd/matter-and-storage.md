# Matter, Storage, Mutation, and Collision

## 1. Authoritative cell model

A logical cell is:

```rust
#[repr(C)]
struct PackedCell(u32);
// bits 0..=15 material ID, bits 16..=23 coverage, bits 24..=31 reserved = 0
```

Coverage is an unsigned scalar from 0 (no matter) to 255 (full coverage) at the
center of the addressed cell. Empty is exactly material 0 and coverage 0.
Nonempty material with zero coverage is invalid. Reserved bits must be zero in
sources, commands, persistence, and GPU output.

The authoritative occupied set is defined exactly: a cell contributes its
axis-aligned unit cell box when its coverage is greater than or equal to its
material's `occupancy_threshold`, and contributes no occupied space otherwise.
Point queries use floor-to-cell with the volume domain half-open at its maximum;
overlap, trace, and sweep use the closed boundary of the union of those boxes
for contact. Faces shared by two occupied cells are interior and not contacts;
when several boundary cells coincide at an edge/corner, the lower
lexicographic cell owns the fact. This definition is the reference model for
collision and removes any dependency on a generated mesh.

Coverage additionally provides a scalar field for smooth presentation.
Presentation trilinearly interpolates center samples, treats the known finite
domain exterior as a virtual zero only while closing a boundary surface, and
uses the configured iso-level. This virtual presentation sample is not a query
answer and cannot make cold or failed in-domain matter appear empty. Material
identity at a visual mixed boundary is the nonempty sample selected by
deterministic nearest-surface rules; no blended gameplay material is invented.
Collision remains the thresholded occupied-cell set above, so smoothing never
changes occupancy truth. More elaborate visual blending remains derived data.

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
3. immutable base and committed-current descriptors for each ready brick;
4. descriptor pools for `Empty`, `Homogeneous`, and `Mixed` bricks;
5. fixed-capacity immutable-base and committed-current mixed-brick pools;
6. transaction page pools;
7. per-ready-region occupancy hierarchy;
8. revision-tagged query/result rings; and
9. revision-tagged presentation buffers described elsewhere.

A brick descriptor is one of:

- `Empty`: implicit empty, no payload;
- `Homogeneous(PackedCell)`: one payload word;
- `Mixed(PageIndex)`: exactly 512 packed cells in the private pool; or
- `Unavailable`: an internal sentinel that is never interpreted as empty.

For a ready brick, the immutable base descriptor names the proof-verified base
leaf and the current descriptor names base-plus-scar truth. They share the same
descriptor/page when no scar exists. A changed mixed brick may therefore pay
for both a base page and a current page; that cost is charged to authoritative
residency and is released when the region becomes cold. Retaining the base
view is required to canonicalize later scars without a CPU mirror or a second
source fetch during commit.

Untouched homogeneous regions therefore cost region/brick descriptors, not raw
cells. Fully cold regions have no detailed GPU descriptor and retain only CPU
lifecycle/source/scar metadata. Sparse page-table entries are allocated only
for requested regions, scars, and nonhomogeneous source results.

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

`descriptor()` is captured once during configuration and must remain byte-equal
for the source registration's lifetime. `SourceError` contains a stable
consumer-selected diagnostic code, bounded message, and
`SourceRetry::{Transient, Permanent}`; Moria maps it to its own scoped
`MoriaError` without trusting retryability for malformed bytes or proofs.
Cancellation is cooperative. A future that ignores cancellation remains
charged to the source concurrency/buffer reservation until it resolves or the
configured source timeout expires; after timeout its eventual output is
discarded by operation generation.

Materialization pipeline:

1. Admission pins a cold/requested region and reserves source, GPU, and
   readback capacity.
2. Runtime transitions `cold -> requested -> materializing`.
3. The source future runs on Bevy's I/O task pool with a timeout/retry policy.
4. CPU validation checks identity proof, exact bounds, payload lengths,
   reserved bits, material IDs, empty semantics, and digest.
5. Valid payloads upload to staging; a compute pass installs immutable base
   descriptors/pages, derives sharing current descriptors, applies canonical
   scars into distinct current pages only where needed, and builds occupancy
   aggregates.
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
6. **Prepare scar:** GPU compares staged cells with the retained immutable base
   descriptors/pages and emits a bounded canonical delta for the affected
   bricks. The CPU verifies the compact delta and installs it in a reserved
   unpublished scar version before commit.
7. **Commit:** one ordered compute dispatch swaps all affected page-table
   descriptors, updates occupancy aggregates, and writes one completion record
   containing prior/new revision and checksum. Public query dispatches for that
   volume are ordered either before or after this dispatch. There is no public
   dispatch interleaving inside it.
8. **Publish:** only after the completion fence does runtime advance CPU
   revision metadata, install the scar delta, emit one observation, and mark
   presentation stale/building.
9. **Reclaim:** old pages are retired after all earlier GPU readers complete;
   transaction pages and the unpublished scar version are discarded on any
   failure before commit.

The commit dispatch does no fallible allocation or content validation. From
dispatch through CPU publication, the scheduler places a per-volume commit
barrier: no later query, command, checkpoint cut, presentation build, or
extension snapshot for that volume can capture or dispatch. Earlier readers
are already ordered before the swap. CPU publication after the completion
fence installs the prepared scar, revision, receipt fact, observation, and
presentation invalidation in one main-world transaction and is the public
commit point. A GPU device
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
volume transform and top-level spatial-index entry. The same per-volume barrier
orders the GPU descriptor update before one CPU publication that swaps the AABB
index snapshot and increments the volume revision. In-flight world-space
queries are ordered entirely before or after that publication.

Create reserves identity, immutable source descriptor, registry/tombstone
capacity, budget, and persistence entries before its catalog commit. It becomes
visible at volume revision 1 and the new catalog revision. Retire enters
`retiring` visibly but the catalog removal
commits only after:

- no operation pins the volume;
- dirty scars and the retirement record satisfy the configured durability
  policy; and
- no checkpoint cut refers to work not yet copied.

The retiring state rejects new local commands, interests, and queries but
allows existing queries to finish at their captured revision. A newly submitted
world-scope query whose bounds intersect a retiring volume waits behind the
retirement control operation rather than omitting it or extending retirement
with a new pin. The final removal advances catalog revision once and emits a
retired observation. Its persistent volume record becomes a tombstone holding
identity, last revision, and retirement state; its source/scars may be released
only after the configured durability rule is met. A `VolumeId` is never reused
within one persistent world.

Cancellation or failure before final removal atomically leaves the catalog
unchanged and returns the volume from `retiring` to its prior lifecycle states;
waiting world queries are then dispatchable against it. Retirement has no
post-catalog-removal failure stage.

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
- Shape overlap prunes with region/brick occupancy and tests the thresholded
  occupied-cell boxes defined in §1.
- Sweeps use broad-phase swept AABBs, local-space conservative advancement,
  and at most 16 bisection iterations to reach configured tolerance.
- Contact normals are the entered occupied-cell face normals, transformed by
  placement rotation. Coverage-gradient normals are presentation facts and
  cannot alter collision.

World overlap is the union of all volume facts. Overlapping cells are not
merged, prioritized, or resolved. Results include volume/material identity,
cell, world contact/interval, normal when defined, and exact volume revision.

Point, AABB, and ray results are exact against the occupied-cell union.
Oriented/curved/convex overlap and sweep may return
`ContactCertainty::Conservative` when bounded iteration stops before a
separating proof; they must not miss an occupied cell. A conservative hit is a
fact about algorithmic certainty, not a behavior response. Consumers own any
follow-up policy.

`CollisionPrecision` is expressed as a positive cell-edge fraction, defaults
to `1/256`, and is validated within `[1/4096, 1/16]`. Results report the
configured tolerance and iterations used. Reaching 16 iterations without that
tolerance returns a conservative fact rather than a falsely exact one.

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
