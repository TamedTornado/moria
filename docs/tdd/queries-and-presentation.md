# Queries, collision, and presentation

## 1. Snapshot capture and volume indexing

World-space queries first capture `TopologyRevision`, the immutable volume
registry snapshot, and each intersecting volume's placement/root/revision.
The CPU registry uses a dynamic bounding-volume hierarchy over conservative
world AABBs. The BVH is acceleration metadata only: candidates are transformed
into volume-local coordinates and resolved against GPU matter. A BVH miss in
oracle comparison is an invariant failure, not an empty result.

Volume overlap is ordinary. Candidates are sorted by `VolumeId`; results
retain volume and material identity for every hit. Moria never unions cells,
chooses a foreground volume, or computes response.

## 2. Query primitives

The version-1 exact primitive set is:

- sample a local or world point;
- enumerate matter runs in a local AABB;
- any/all occupied in an AABB;
- ray trace;
- overlap with sphere, capsule, AABB, OBB, or convex polytope of at most
  32 planes;
- translational sweep of those shapes along a finite vector; and
- bounded observation-recovery snapshot.

Rotational sweeps are not a primitive. A consumer can submit bounded
intermediate poses or use a behavior system, but Moria does not claim a
continuous rotational result.

Broad phase traverses volume BVH, sparse summaries, and brick summaries.
Narrow phase samples authoritative cell coverage. Rays use 3-D DDA in each
volume local space. Shape overlap conservatively enumerates intersecting cells,
then tests the thresholded cell box; organic coverage additionally refines the
contact point/normal from the trilinear scalar gradient. A zero gradient
returns the stable cell-face normal selected by greatest penetration and
axis-order X, Y, Z tie-breaking.

Sweeps use conservative advancement against candidate cell boxes with a
maximum iteration count in `MoriaLimits`. If convergence is not proved, the
result is `Indeterminate`, not no-hit. Contacts contain volume, material,
local/world point, world normal, time of impact for sweeps, cell, coverage,
and captured volume revision.

`max_hits`/`max_contacts` is mandatory. Ordered traces sort by distance then
`VolumeId` then cell coordinates. If more results exist, `truncated=true`;
truncation is a complete occupancy fact only up to the last returned distance,
not a claim about the remainder.

## 3. Query consistency and availability

Once captured, a query holds snapshot generation pins until GPU completion.
Placement and matter changes after capture do not alter it. Results state the
captured revision vector, so a consumer can reject stale facts.

A minimum revision condition remains pending only under `Materialize`; under
`ReadyOnly` it returns unavailable. A requested minimum revision greater than
the current committed revision is `PendingRevision`, not a stale query.
Retired volumes captured in an older topology snapshot remain inspectable only
while pinned; new queries use the new topology.

World queries are complete only if every volume intersecting the query bounds
at captured topology was ready and inspected. The absence of a registered
volume is not itself material, but a failed/cold intersecting volume makes the
query unavailable or explicitly partial.

## 4. Presentation pipeline

Presentation is a consumer-requested derivative keyed by:

```text
(WorldId, VolumeId, brick region, VolumeRevision, PresentationPolicyHash)
```

Its state machine is:

```text
Absent -> Building(target_revision) -> Current(target_revision)
Current(R) + truth R+n -> Stale(R, target=R+n) -> Current(R+n)
Building | Stale -> Failed(target_revision, error)
Failed -> Building (explicit retry or later revision)
any -> Absent (interest withdrawal/eviction)
```

An older valid mesh may remain visible only when the consumer selected
`ShowStale`; `HideStale` removes it and `DiagnosticFallback` displays a clearly
derived error proxy. Status and telemetry still say `Stale` or `Failed`.
Collision never binds mesh buffers.

Meshing uses one-cell halo snapshots:

- `SurfaceMode::Crisp`: culled/greedy axis-aligned faces at occupancy
  thresholds, with material boundaries retained.
- `SurfaceMode::Organic`: surface nets over the trilinear coverage field,
  clamped to source cells, with normals from scalar gradients.
- mixed boundaries are owned by the lower stable `MaterialId`; crisp material
  forces a retained boundary face so constructed edges are not smoothed away.

Brick meshes share deterministic boundary vertices derived from global local
cell coordinates and fixed-point 16.16 interpolation. This prevents cracks
without sharing mutable mesh state. Level of detail is not in contract version
1: the implementation builds full-resolution interested presentation and
reports budget pressure rather than substituting an unvalidated distant truth.

Meshes store volume-local vertices and use the current admitted placement at
draw time. Moving a volume therefore rebuilds neither matter nor mesh.
When placement advances the volume revision without changing the matter root,
the draw record binds the new placement and presentation status advances to
that volume revision without remeshing. This is not relabeling an obsolete
view: the composed view consists of unchanged local geometry plus the newly
committed placement. Status reports both the local matter root generation and
the current volume revision so consumers can audit that distinction.

## 5. Derived dressing and matter-backed assemblies

`DressingProvider` receives a bounded, read-only `SurfaceSnapshot` containing
surface samples, material IDs, stable local anchors, and revision. It returns a
bounded instance stream charged to derived bytes. It cannot query storage
internals or submit changes through the provider callback.

Each instance key contains `(VolumeId, anchor cell/feature, VolumeRevision,
provider key)`. When supporting matter changes, old instances become stale and
are removed or regenerated according to presentation policy. Dressing has no
occupancy and is never returned by truth queries.

Anything that must be occupied, queried, edited, or persisted is registered as
matter in an ordinary material volume. `CreateVolume` is the matter-backed
assembly seam; there is no special prop authority.

## 6. Derived resource behavior

Presentation jobs reserve maximum vertex/index/instance bytes before dispatch.
If actual output would overflow, the entire artifact fails and no partial mesh
is labeled current. Under derived-budget pressure Moria evicts unreferenced
artifacts, preserves stale artifacts only if policy and budget allow, and
otherwise reports `Absent` or `Failed(BudgetExceeded)`.

Raw-cell, region-boundary, lifecycle, and revision visualizers are implemented
in `moria-validation` from public query/telemetry results. They are not
compiled into a storage backdoor.
