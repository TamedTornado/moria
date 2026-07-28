# Collision and Presentation

## Shared authority, separate products

Collision and presentation both read a captured material snapshot. Neither
feeds the other. A missing, stale, or failed mesh cannot change collision, and
collision acceleration cannot become a render surface.

For a snapshot, both record:

- device and directory generation;
- volume ID/key;
- volume revision and rigid placement;
- local bounds actually read.

Any derived output missing this provenance is invalid.

## Collision pipeline

The `collision` module is a private storage-level kernel. It accepts validated,
bounded POD traversal plans and returns private material/contact facts.
The `query` module owns every public collision descriptor, readiness/partial
policy, receipt, ordering codec, and `ContactFact`; it performs the only
translation across that boundary. Collision never imports `query`.

### Broad phase

World-space query bounds are transformed into each candidate volume's local
space. Candidate volume selection uses the committed volume-directory AABB.
Dynamic movement publishes a new directory AABB with placement. Overlapping
volumes all remain candidates.

Inside a volume, the exact/conservative coarse occupancy hierarchy skips known
empty groups, then page summaries skip empty bricks. Unknown/cold bricks make
the query pending or unavailable according to its readiness policy. They are
not skipped as empty.

The maximum candidate volume, brick, and cell counts are computed before
dispatch. Collision work is admitted only with the public
`TraversalAuthorization`; the checked conservative totals must fit both its
values and the fixed 8,192-brick/65,536-cell v1 maxima. Exceeding either bound
rejects the query. Partial coverage may omit unavailable spatial regions but
never buys more traversal work.

### Narrow phase

V1 authoritative geometry is the union of half-open occupied cell boxes under
the volume's rigid placement. `coverage >= 128` determines occupancy. Coverage
below threshold may influence a smooth render contour but is not collision.
This rule is stable and independently testable.

Supported operations:

- point/sample: exact containing-cell lookup;
- region occupancy: mask reduction over all intersecting cells;
- segment trace: 3D DDA in local cell coordinates;
- sphere overlap: closest point from sphere center to occupied cell box;
- AABB overlap: interval intersection;
- capsule overlap: squared segment-to-box distance;
- sphere/AABB/capsule sweep: conservative voxel traversal followed by analytic
  shape-vs-box time of impact.

For rotated dynamic volumes, shapes and sweeps are inverse-transformed into
volume local coordinates. An input AABB becomes an oriented box locally; the
narrow phase uses its eight points and separating axes rather than expanding it
to a loose local AABB. The broad phase may use a conservative enclosing AABB.

### Contact facts

```rust
pub struct ContactFact {
    pub volume: VolumeId,
    pub revision: VolumeRevision,
    pub material: MaterialId,
    pub cell: CellCoord,
    pub world_point: WorldPoint,
    pub world_normal: WorldVector,
    pub penetration_or_toi: f32,
}
```

Trace and sweep report normalized `toi` in `[0, 1]`. Overlap reports
nonnegative penetration estimate and a deterministic separating normal.
Normals are cell-box facts, not mesh normals. Ties choose axis X, then Y, then
Z in local space, then rotate to world space.

Initial overlap in a sweep returns `toi = 0`. Zero displacement is rejected for
sweep and should use overlap. Moria does not separate bodies, update velocity,
apply impulses, choose friction, or interpret contact.

The scheduled CPU behavior view reuses this private fact kernel over its
immutable exported cell set and debits the adapter's declared traversal
authorization; it does not submit a public query receipt. The scheduled GPU
view exports the same sample/occupancy truth in canonical cell records.
Neither path reads the derived surface mesh, and neither adds contact-response
or behavior state to collision.

### Ordering and overflow

Hits sort by:

1. increasing trace/sweep parameter (zero for overlap);
2. stable volume UUID bytes;
3. local cell Z, Y, X;
4. material ID.

Duplicate cell hits produced by adjacent traversal are removed. Exceeding the
authorized result cap always fails the receipt with `OutputOverflow` and
returns no result; partial coverage does not truncate hits from an inspected
brick. A partial collision result therefore omits only explicitly unavailable
spatial regions, carries those omissions, and cannot be reported as “no hit.”

## Presentation source field

Presentation uses material coverage as a scalar lattice field with isovalue
127.5. A brick job reads its 8³ cells plus a one-cell neighbor halo. Missing
neighbors inside the volume domain keep the artifact building/pending; outside
the finite domain the value is canonical empty.

Mutating a boundary cell marks its brick and every face/edge/corner neighbor
whose halo includes that cell dirty. One cell belongs to at most two artifact
read domains per axis, hence at most eight artifacts. Edits of cells on both
sides of one brick—such as all eight brick-corner cells—can have the full
3×3×3 union of 27 artifact keys. The 27 bound therefore applies to the union
for one local brick neighborhood, not to one cell or a whole command. A
legal command may affect 512 mutually separated bricks and therefore invalidate
up to `512 * 27 = 13,824` distinct artifact keys. The control plane enumerates
and deduplicates those keys during command validation with fixed maximum
scratch, but truth publication never waits for presentation job slots.

`presentation_dirty_records` is partitioned at startup: exactly
`live_volumes` slots are permanently reserved as one
`(live_slot, dirty_revision)` fallback marker each; the remaining slots hold
exact artifact keys. Configuration requires
`presentation_dirty_records >= live_volumes + presentation_jobs`. A created
volume inherits its directory slot's marker, and retirement clears it before
the slot can be reused.

At default limits all 13,824 exact keys fit. With a smaller exact partition or
concurrent invalidations, a volume atomically raises its reserved marker to the
newest dirty revision before discarding/coalescing its exact keys. No other
volume can consume that marker. The scheduler then enumerates only currently
interested artifacts in bounded `presentation_artifacts` pages and compares
their source revision. This can increase rebuild work but cannot lose a
stale/current transition, even when all live volumes commit concurrently. At most
`presentation_jobs` are submitted concurrently; the remainder stay as bounded
dirty/artifact state and drain fairly by priority then stable volume/brick
order. Telemetry distinguishes exact invalidations, volume coalescing, pending
dirty records, scheduled jobs, completed-current artifacts, and superseded
targets.

## Dual-contouring surface derivation

Each sign-changing cell cube emits at most one vertex:

1. locate edge crossings by linear interpolation of coverage;
2. estimate gradients by central difference in the halo;
3. gather crossing position/normal constraints;
4. solve a bounded QEF and clamp the vertex to the cell cube;
5. emit oriented quads/triangles for sign-changing grid edges;
6. assign material presentation inputs by the stable endpoint rule.

For `Organic`, normals are averaged and the regularized QEF favors a smooth
centroid. For `Constructed`, normals are clustered and the feature-preserving
QEF retains planes/creases. A mixed neighborhood chooses `Constructed` if any
surface-contributing material is constructed, preventing a masonry edge from
being rounded by an adjacent organic material.

Degenerate gradients use a stable axis normal. Non-finite QEF output,
out-of-cell vertices, invalid indices, and configured vertex/index overflow
fail the complete artifact. They are never truncated and labeled current.

Brick artifacts duplicate seam vertices deliberately. Position generation from
integer global cell coordinates and the same halo rules makes seam positions
identical. Rendering may use skirts only as a diagnostic fallback; a skirt is
never reported as the final current surface.

## Rendering ownership

Compute writes bounded vertex/index output plus an artifact header. After GPU
validation, Bevy render-world resources own device buffers. The main world
receives only artifact metadata and creates/updates tagged render entities
through the adapter.

No CPU readback is required for normal rendering. Diagnostic mesh export is a
separate bounded query and follows normal readback rules.

Each visible artifact tag contains source revision and placement revision.
Installation compares both with the current volume gate:

- match: install as `Current`;
- older valid artifact with `DisplayStale`: retain as `Stale`;
- mismatch under hide policy: remove;
- newer/impossible generation: invariant failure and quarantine.

Meshes are discarded and rebuilt after device recovery. Their absence cannot
make a region materially cold.

## Material presentation inputs

`SurfaceDescriptor` is embedded in `MaterialDefinition` and contains
`SurfaceClass`, one Bevy material handle or explicit neutral diagnostic input,
optional bounded triplanar handles/scale, and finite
roughness/metallic/tint values. Dressing is not an unresolved style-ID list on
this descriptor: the separate builder-time `DressingDescriptor` registry owns
a stable style key and an exact bounded material-key filter.

These inputs have no occupancy semantics. A missing asset fails presentation
for the affected material/artifact, not material registration or collision.
The diagnostic material is available only when the consumer explicitly selects
diagnostic fallback policy; it is not a hidden default product aesthetic.

## Derived dressing

Dressing is generated only on current surface artifacts:

1. enumerate candidate surface triangles;
2. seed a counter-based PRNG with volume key, brick coordinate, matter
   revision, and style key;
3. select points using descriptor density, coverage range, and exact material
   filter;
4. anchor each instance to source triangle coordinates plus revision;
5. write at most both the descriptor's 4,096-instance maximum and the
   configured global instance capacity.

Density, orientation, and scale are consumer presentation inputs, not
procedural world content. Instances have no material identity or occupancy.

On supporting matter change, all old anchors in affected artifacts become
stale with the surface and are replaced/removed under the same view policy.
Descriptor/style registration is callable only through
`MoriaBuilder::register_dressing_style`; duplicate/unknown style or material
keys, invalid ranges, and registry capacity fail validation explicitly.
Missing mesh/material assets fail the affected presentation artifact with a
typed asset error. Instance overflow fails the dressing artifact; it never
emits an unreported subset as current. A consumer requiring independent
query/mutation/collision registers a material volume instead of dressing.

## Presentation resource pressure

Presentation has independently bounded artifact metadata, dirty records,
vertex/index buffers, dressing instances, and in-flight job slots. LRU
eviction follows this order:

1. obsolete submitted output after GPU completion;
2. nonvisible stale artifacts;
3. background dressing;
4. background current meshes;
5. normal-priority artifacts outside current interest.

Evicting presentation sets `Absent`; it does not retire authoritative bricks
required for another capability. Presentation jobs can be coalesced to the
newest target revision before submission. Every skipped target appears in
telemetry.

There is no v1 distant LOD contract. Consumers request bounded presentation
interest within available budgets. Adding LOD later must preserve revision
provenance and may not weaken material/collision truth.

## Validation obligations

Collision correctness is exact relative to the occupied-cell specification.
The CPU oracle implements the same geometry independently without using GPU
page/storage code. Generated and fixed tests compare:

- all supported shapes in static, rotated dynamic, negative-coordinate, and
  overlapping-volume cases;
- empty/full/mixed/homogeneous/detailed bricks;
- boundary points and initial-overlap sweeps;
- results before/after edit, movement, eviction/rematerialization, and restore;
- results while presentation is absent, stale, failed, and current.

Presentation correctness validates topology indices, finite/clamped vertices,
seam agreement, material assignment, revision installation, and overflow. A
diagnostic visual fixture includes smooth organic matter, sharp constructed
matter, an edited cut, a placed patch, and revision-anchored dressing. Human
review judges visual coherence; it is not the oracle for occupancy.
