# Presentation and Behavior Extensions

## 1. Presentation authority boundary

Presentation consumes immutable revision-tagged matter snapshots from private
GPU storage. It can be destroyed wholesale without changing world revisions,
scars, occupancy, or collision. No query API accepts a presentation handle.

Each region has independent status:

```rust
pub enum PresentationStatus {
    Absent,
    Building { target: Revision },
    Current { revision: Revision },
    Stale { shown: Revision, target: Revision },
    Failed { target: Revision, error: PresentationError },
}
```

A changed matter commit marks intersecting presentation stale or building in
the same publication cycle. An unaffected region may be relabeled to the new
volume revision without rebuilding because its matter is provably unchanged.
Placement changes update the volume transform without rebuilding local meshes;
their status relabels to the new volume revision after the transform commit.

The consumer configures `StaleViewPolicy::{Show, Hide, Diagnostic}`. This
affects visibility only. Status always reports the actual shown/target
revisions.

## 2. Surface derivation

`SurfaceDefinition` chooses one of two technical pipelines:

- `Smooth { iso_level, feature_weight }`: constrained dual contouring over the
  coverage field, with QEF vertices clamped to their cell and deterministic
  fallback to the cell center.
- `Constructed`: greedy coplanar face merging on occupied/empty boundaries,
  preserving sharp orthogonal cell features.

Both consume the same authoritative cells and share crack-free brick border
samples. Material boundaries carry stable material IDs for shader lookup.
Each surface definition contains a `boundary_priority: u8`. Transitions between
smooth and constructed materials use a deterministic boundary-owner rule:
higher boundary priority owns the interface geometry and selects its pipeline;
equal priority breaks ties by lower material ID.

Mesh generation stages:

1. capture region/neighbor descriptors and target revision;
2. classify occupied boundaries in compute;
3. prefix-sum exact vertex/index counts;
4. reserve from bounded geometry pools;
5. emit positions, normals, material IDs, and revision;
6. validate counts/ranges and border checksum;
7. publish allocation only if target revision is still current.

If the revision changed, generated buffers are discarded and status remains
stale/building for the newer revision. Pool/compilation/device errors set
`Failed` without changing matter. Retry is explicit or pressure-policy driven.

Normals are derived presentation. Honest cut faces follow changed coverage and
material after rebuild. Raw voxel/debug views are separate public queries plus
diagnostic rendering and never become a fallback authority.

## 3. Derived dressing

Consumers may register bounded `DressingDefinition` values keyed by material
and presentation key. A definition supplies a Bevy mesh/material asset key,
density/filter parameters, size/orientation ranges, and deterministic salt. It
cannot claim occupancy or a material ID.

Moria generates a revision-tagged `SurfaceAnchor` buffer from current surface
samples. Anchor positions are deterministic from `(world, volume, region,
cell, revision, definition salt)`. Indirect instances reference consumer
assets but remain owned by the presentation allocation.

When supporting matter changes:

- intersecting anchors immediately become stale with their source revision;
- `Hide` policy removes them before the next view extraction;
- `Show` may retain them only as visibly stale presentation;
- rebuild emits anchors only on current supporting occupied surfaces; and
- retirement releases them.

No dressing entity is inserted into collision, persistence, or matter queries.
If a consumer wants an object to have independent occupancy/mutation identity,
it creates a static or dynamic material volume through the ordinary command.

## 4. Matter-backed assemblies

There is no separate object storage. An assembly is a normal material volume,
usually with a bounded patch/stamp base source and caller-selected persistent
ID. It uses ordinary creation, interest, query, mutation, placement (if
dynamic), presentation, scar, checkpoint, and restore paths.

This choice prevents a vegetation/clutter object API from becoming a
privileged second occupancy model.

## 5. CPU behavior-extension path

No special CPU behavior interface is required. A CPU plug-in uses bounded
observations, queries, and commands via `MoriaClient`. Its correlation bytes
link external decisions to receipts and observations. Removing it removes the
behavior and leaves Moria's material schema unchanged.

## 6. Bounded GPU observation

The optional `gpu-extension` crate feature exposes an authorized exchange, not
storage:

```rust
pub struct GpuObservationRequest {
    pub interest: InterestId,
    pub local_bounds: BoundedVolumeBounds,
    pub fields: GpuObservationFields,
    pub minimum_revision: Option<Revision>,
    pub max_cells: u32,
    pub max_effects: u32,
}

pub trait MoriaGpuExtension: Send + Sync + 'static {
    fn manifest(&self) -> GpuExtensionManifest;
    fn prepare(
        &mut self,
        context: &mut GpuExchangeContext<'_>,
        snapshot: GpuMatterSnapshot<'_>,
        effects: GpuEffectSink<'_>,
    ) -> Result<(), ExtensionError>;
}
```

The implementation runs only inside a registered Bevy RenderApp system at
Moria's declared render-graph slot. The request must be covered by a live
`GPU_OBSERVATION` interest. Moria copies/compacts selected committed fields
GPU-to-GPU into an immutable, exchange-owned snapshot buffer. It is tagged with
exact world/volume/bounds/revision/layout and has a hard cell count. It is not
the sparse page table or brick pool.

`GpuMatterSnapshot` does not expose the underlying authoritative `wgpu::Buffer`.
It provides only methods that bind the read-only snapshot at the documented
bind-group index and report layout/count metadata. It expires at the exchange
fence and cannot be retained. Rust lifetimes prevent ordinary retention; debug
validation detects use outside its graph node.

This path avoids CPU readback but may perform bounded GPU-to-GPU compaction.
Telemetry reports copied bytes, cells, latency, queue delay, and any readback
caused by terminal result publication.

## 7. GPU effect sink and admission

`GpuEffectSink` binds a Moria-owned append buffer with fixed records:

```text
effect kind (patch cell or placement proposal)
world/volume ID table index
local cell or proposed placement
material + coverage
expected revision
extension correlation (64 bits)
```

The manifest declares maximum records, target volumes, allowed effect kinds,
and material IDs. Overflow sets a flag and rejects the entire exchange output.
The sink cannot write page-table indices or raw addresses.

After extension dispatch:

1. Moria validates record count/overflow/checksum on GPU.
2. Records are grouped into bounded proposed commands; compact headers and
   records are read back only as required for main-world admission.
3. Main-world admission revalidates extension authorization, IDs, bounds,
   material definitions, expected revisions, command size, and budget exactly
   like consumer-submitted commands.
4. Each accepted group receives an ordinary `ReceiptId` and follows the same
   copy-on-write transaction path. Rejected groups receive stable errors.
5. Exchange completion returns the list of admission outcomes and correlation
   values through normal bounded polling/observations.

Extension dispatch never mutates truth directly. To preserve atomicity, all
patch records with one group ID form one matter command and either all commit
or none. Cross-volume groups are rejected.

## 8. Extension failure and isolation

Shader compilation, manifest violation, overflow, timeout, device validation
error, or malformed effect output fails the exchange. It affects no truth
unless a previous output group had already completed ordinary admission; those
receipts proceed independently and are reported.

An extension receives no persistence hook, cell schema extension, behavior
fields, collision-response callback, or private entity access. Its own state
and save data remain external. Device loss follows the world recovery behavior
in `matter-and-storage.md`.

## 9. Visual acceptance

Automated checks prove revision/status/geometry invariants. Human captures
prove only the configured fixture's visual result. Required captures show:

- smooth deep-volume surfaces and honest edit cuts;
- crisp constructed surfaces;
- current/stale/failed presentation status overlays;
- derived dressing disappearing/rebuilding with support; and
- raw matter diagnostic aligned with presentation.

Captures record adapter/config/revision and are invalid if presentation has not
reported the target edit revision `Current`.
