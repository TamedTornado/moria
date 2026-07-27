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
dispatch. Exceeding a bound rejects the query unless explicit partial coverage
was requested.

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

### Ordering and overflow

Hits sort by:

1. increasing trace/sweep parameter (zero for overlap);
2. stable volume UUID bytes;
3. local cell Z, Y, X;
4. material ID.

Duplicate cell hits produced by adjacent traversal are removed. Exceeding the
authorized result cap is `OutputOverflow` unless partial was explicitly
requested. A partial collision result carries omitted coverage and cannot be
reported as “no hit.”

## Presentation source field

Presentation uses material coverage as a scalar lattice field with isovalue
127.5. A brick job reads its 8³ cells plus a one-cell neighbor halo. Missing
neighbors inside the volume domain keep the artifact building/pending; outside
the finite domain the value is canonical empty.

Mutating a boundary cell marks its brick and every face/edge/corner neighbor
whose halo includes that cell dirty. This is at most 27 artifact jobs. The
affected job set is reserved before the mutation publishes, or presentation
work records explicit pressure and remains stale/absent; truth publication does
not depend on mesh capacity.

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

`SurfaceDescriptor` contains:

- `SurfaceClass`;
- Bevy material asset handle or a Moria neutral diagnostic material;
- optional bounded triplanar texture handles and scale;
- roughness/metallic/tint values used only for rendering;
- zero or more dressing style IDs.

These inputs have no occupancy semantics. A missing asset fails presentation
for the affected material/artifact, not material registration or collision.
The diagnostic material is available only when the consumer explicitly selects
diagnostic fallback policy; it is not a hidden default product aesthetic.

## Derived dressing

Dressing is generated only on current surface artifacts:

1. enumerate candidate surface triangles;
2. seed a counter-based PRNG with volume key, brick coordinate, matter
   revision, and style key;
3. select points using descriptor density and material filter;
4. anchor each instance to source triangle coordinates plus revision;
5. write at most the configured instance capacity.

Density, orientation, and scale are consumer presentation inputs, not
procedural world content. Instances have no material identity or occupancy.

On supporting matter change, all old anchors in affected artifacts become
stale with the surface and are replaced/removed under the same view policy.
Overflow fails the dressing artifact; it never emits an unreported subset as
current. A consumer requiring independent query/mutation/collision registers a
material volume instead of dressing.

## Presentation resource pressure

Presentation has independent vertex/index/instance pools and LRU eviction.
Eviction order is:

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
