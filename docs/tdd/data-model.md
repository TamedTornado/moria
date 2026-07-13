# Data Model

## Coordinate types and invariants

Distinct newtypes prevent metre/voxel/brick confusion:

```rust
pub struct WorldPos(pub Vec3);
pub struct VoxelCoord(pub IVec3);
pub struct BrickCoord(pub IVec3);
pub struct LocalVoxelIndex(pub u16); // x + 16 * (y + 16 * z), 0..4095
pub struct WorldSeed(pub u64);
```

`WorldPos -> VoxelCoord` floors each coordinate after dividing by 0.25 m. `VoxelCoord -> BrickCoord` uses Euclidean division by 16. All persisted maps and deterministic operations sort by `(brick.x, brick.y, brick.z, local_index)`; hash-map iteration is never observable.

## Material and voxel truth

```rust
pub struct MaterialId(pub u8);

pub struct MaterialDef {
    pub id: MaterialId,
    pub name: String,
    pub hardness: u8,
    pub granular: bool,
    pub phase: MaterialPhase,       // Empty | Fluid | Solid
    pub debug_placeable: bool,
    pub surface_class: SurfaceClass, // Organic | Rock | Masonry | Vegetation | Water
    pub render: MaterialRenderDef,
}

#[repr(C)]
pub struct VoxelValue {
    pub material: MaterialId,
    pub density: u8,
    pub state: u8,
}
```

The registry assigns stable IDs in this order: air, water, topsoil, subsoil, sand, gravel, limestone, sandstone, shale, granite, iron ore, wood, leaf, cut stone. Save files encode IDs and include the exact `generation_fingerprint`, `material_fingerprint`, and `persistence_fingerprint` defined in `config.md`. `state` is always zero in Product One. No system writes nonzero state, performs granular settling, or advances material behavior.

Solid occupancy is `material.phase == Solid && density >= 128`. Water is full-density fluid and therefore occupied material but not solid collision. Density 0 is canonicalized to air/state 0. Material hardness is exposed metadata reserved for consumers; Product One's proof dig uses the explicit operation strength and does not run a mining/progression rule. Render properties never decide truth.

## Procedural base region

`BaseRegion` is an immutable resource containing the `GenerationConfig`, `RegionManifest`, `MaterialRegistry`, boot-verified `generation_fingerprint` (including the compiled `BASE_SAMPLER_REVISION` and exact vegetation-template and ruin-stamp byte digests), and read-only spatial indices for feature/object metadata. It does not contain a 4000 x 1024 x 4000 voxel array.

### Pure sample pipeline

`sample_base(VoxelCoord, &BaseRegion) -> BaseSample` is a pure function with this deterministic precedence:

1. Reject outside region bounds.
2. Evaluate the column summary: surface elevation, soil thicknesses, biome, lake/river containment and water level.
3. Evaluate 3D geological strata at the sample centre using tilted fixed-point planes and stable seeded noise.
4. Subtract cave signed-distance volumes.
5. Apply aquifer-band and iron-vein volumes where their host/depth constraints match.
6. Apply generated water in carved lake/river empty volume up to its static surface.
7. Apply voxel object/stamp occupancy from the object spatial index. Object solid matter wins over empty/fluid and non-overlapping placement validation prevents ambiguous solid/solid precedence. The ruin stamp may deliberately replace terrain with cut stone at its foundation.

`BaseSample` returns `VoxelValue` plus optional `FeatureId`, `ObjectId`, and column metadata used by diagnostics. `ObjectId` is present exactly when step 7 supplied the winning base solid sample; it is absent for terrain/geology/water/air, including empty template cells inside an object's AABB. Generation uses project-owned stable hash salts and signed fixed-point noise. No result depends on thread count, map order, floating transcendental behavior, or frame time.

The precedence and all algorithms in this section are covered by `BASE_SAMPLER_REVISION` as defined in `config.md`. An implementation change that can alter either returned voxel bytes or provenance/metadata for identical inputs is a compatibility change and must bump that revision before its conformance golden is updated.

### `ColumnSummary`

```rust
pub struct ColumnSummary {
    pub xz_voxel: IVec2,
    pub surface_y_fixed: i32,
    pub topsoil_bottom_y_fixed: i32,
    pub subsoil_bottom_y_fixed: i32,
    pub biome: BiomeId,
    pub water: Option<WaterColumn>,
    pub feature_refs: SmallVec<[FeatureId; 4]>,
    pub placement_cell: PlacementCellId,
}
```

Columns are calculated on demand and cached in bounded tiles. They summarize vertical composition but caves/ore/aquifer remain 3D evaluations; a column run cache may accelerate sampling without becoming saved truth.

`column_runs(xz)` exposes a lazily derived, ordered bottom-to-top iterator of `ColumnRun { y_start_voxel, y_end_voxel_exclusive, material_or_void, feature_refs }`. It combines surface layers, stone strata, cave gaps, water, ore, and aquifer results for that column without storing every voxel. Adjacent equal runs are canonicalized. This is the technical representation of the design's column composition and is cacheable/discardable.

### Geological features

```rust
pub struct GeologicalFeatureRecord {
    pub id: FeatureId,
    pub kind: GeologicalFeatureKind,
    pub extent: QuantizedVolume,
    pub host_material: Option<MaterialId>,
    pub depth_range_fixed: RangeInclusive<i32>,
    pub orientation: QuantizedOrientation,
}
```

Kinds are topsoil layer, subsoil layer, tilted stratum, karst cave, aquifer band, and iron-ore vein. The aquifer is represented without wetness state: deterministic porous sand/gravel host cells surround static water-filled gaps/fractures, and the cave intersection exposes that material pattern. No saturation, pressure, propagation, or draining system exists. The ore volume replaces eligible host rock with iron ore. Cave volume produces air except where its named aquifer intersection deliberately exposes static water.

### `RegionManifest`

The generator returns stable metadata for:

- meadow and dense mixed forest polygons;
- river channel spline and lake basin/surface;
- cliff/outcrop bounds and exposed tilted-strata viewpoint;
- connected cave spline/volume, mouth, floor, aquifer crossing, and ore crossing;
- boulder, stump, rock, bush, and two-species tree placements;
- ruin point metadata and stamp transform;
- named traversal and flythrough waypoints with world elevation and local surface depth.

An offline/boot validator proves cardinality, intersections, route connectivity, slope/clearance, stair traversability, forest density, and local cave depth before the preset is accepted. Feature locations are generated from the authored seed and curated constraints; the terrain itself is not serialized or hand-shaped.

### Water

`WaterBody` has `WaterBodyId`, `Lake | River`, fixed surface Y or deterministic river segment levels, carved occupied polygon/volume, and static state. The river generator first lowers the terrain channel, then fills only resulting empty volume; the lake generator creates a basin before fill. There is no flow graph, pressure, drain/fill command, or per-frame fluid system.

### Voxel objects and ruin

```rust
pub struct VoxelObjectRecord {
    pub id: ObjectId,
    pub kind: ObjectKind,
    pub template: VoxelTemplateId,
    pub transform: QuantizedTransform,
    pub species: Option<SpeciesId>,
    pub bounds_voxel: VoxelAabb,
}
```

Object kinds are tree, bush, boulder, stump, scattered rock, and ruin. Two stable tree species IDs are required. Trees/bushes/rocks use deterministic procedural voxel templates/variants; the ruin uses the one sparse checked-in cut-stone stamp and intact stair metadata. Records are indexed into placement cells and overlapping bricks, so a base query samples only nearby candidates.

Objects participate in the composite voxel query and dig/place like other matter. An edit does not trigger felling, falling, splitting, growth, fire, or a rigid body: unsupported remnants remain static. Render ownership is based on immutable base provenance, not edit history: `sample_base(coord).ObjectId` owns that lattice sample for presentation even after it is dug to air; every coordinate without such an ID belongs to terrain, including material placed into base air beside a tree or ruin. Placing back into a dug object coordinate therefore remains object-owned, while adjacent new construction remains terrain-owned. The delta does not need a second owner field because seed/config/coordinate reproduce this provenance exactly.

### Surface ownership and object render lifecycle

Surface extraction first enumerates crossings from the full current composite field; it never extracts terrain and objects as if the other were empty. A crossing exists only between a current solid sample and a current non-solid sample. Its `SurfaceOwner` is the immutable base-provenance owner of the solid-side lattice sample: `Object(ObjectId)` when present, otherwise `Terrain(lexicographically_lower_emitting_brick)`. Solid/solid transitions between terrain and an object are material transitions, not exterior boundaries, and emit no coincident cap. Dual-contouring cells containing crossings from more than one owner use the common QEF vertex and deterministic topology once, then partition primitives by the lexicographically smallest `(crossing_axis, solid_coord)` crossing contributing to that primitive. Brick seam ownership is applied after this rule. Consequently each exterior solid boundary primitive has exactly one owner and all owners use identical seam vertices.

The extraction product is partitioned into terrain-brick fragments and object fragments. Unedited repeated objects use a shared base mesh keyed by `(template, orientation_variant, support_contact_signature)`. Placement validation permits only the template's checked-in support-contact signatures, so tree/bush/boulder/stump/rock instances with an equal key really have byte-identical object-owned base fragments; the one ruin may have its own shared key. A command makes an instance unique whenever any changed sample lies in its one-cell extraction stencil or changes a crossing currently/previously owned by that object, including a terrain edit beside its boundary. It returns to the shared base handle only when every sample in that stencil again equals `sample_base` and the recomputed key matches the record's base key. “Edited” therefore means extraction dependency changed, not merely “a delta coordinate carried this object ID.”

For each mutation, dirty detection enumerates before-and-after crossing keys in the changed-cell box plus one lattice-cell halo and takes the union of their owners. It dirties those terrain bricks, those object IDs, and both adjacent brick seam owners; water/raw/dressing invalidation remains phase-specific. A sphere spanning terrain and a tree can therefore dirty both artifacts without assigning the entire command to either. Tests compare the union of all owner-filtered outputs with a one-pass unpartitioned extraction and assert identical primitive keys, no duplicates, and no omissions for: an unedited embedded tree, the ruin foundation/stairs, a dig crossing terrain/object, place into adjacent base air, refill of a dug object coordinate, full revert to base/shared handle, and brick-corner seams.

## Brick representations

```rust
pub enum BrickCache {
    Uniform { value: VoxelValue, revision: u32 },
    Dense { voxels: Box<[VoxelValue; 4096]>, revision: u32 },
}

pub struct BrickRuntime {
    pub coord: BrickCoord,
    pub cache: BrickCache,
    pub dirty: DirtyFlags,
    pub band: ActiveBand,
    pub requested_revision: u32,
    pub committed_revision: u32,
}
```

A uniform cache stores one value plus metadata. Dense arrays exist only for requested surface/cave/inspection/mutation bricks. A dense cache may collapse back to uniform on unload if all samples match, while edits remain in `EditDeltaStore`. Meshing reads a one-voxel halo from adjacent base/delta queries, not duplicated authoritative border voxels.

`DirtyFlags` independently tracks truth cache, terrain mesh, water surface, object instance, dressing, raw-voxel diagnostic, and collision broad-phase cache. A mutation marks every brick containing a changed voxel plus any face/edge/corner neighbour whose one-voxel meshing halo can observe the change. Detection only records affected artifacts; response/job dispatch is separate.

`revision` is a wrapping `u32`. Equality uses the current requested revision; jobs older than it are stale. A session is restarted before 2^32 mutations, so wrap ambiguity is not a supported runtime case.

## Edit delta model

```rust
pub struct EditDeltaStore {
    by_brick: BTreeMap<BrickCoord, BrickDelta>,
    revision: u32,
}

pub enum BrickDelta {
    Sparse(Vec<VoxelDelta>), // sorted unique local indices
    DenseRle(Vec<DeltaRun>),
}

pub struct VoxelDelta {
    pub index: LocalVoxelIndex,
    pub value: VoxelValue, // absolute current composite value
}
```

The store contains exactly the positions whose current value differs from `sample_base`. It chooses sparse encoding below 25% changed voxels and deterministic run encoding at or above that threshold; this affects storage only, not semantics. Applying a mutation compares each result to the base, inserting/replacing a delta or deleting it when equal. Artifact ownership is recomputed from base provenance and is intentionally absent from the persisted delta. One write lock/command transaction covers the entire sphere, so queries never observe a half-applied operation.

Absolute values make load exact and prevent replay-order dependence. The base seed/config is never copied into the payload. Derived surfaces, dressing, active bands, player pose, camera, and time of day are not persisted.

## Surface dressing

`DressingInstance` contains deterministic `DressingId`, kind (grass blade cluster or small clutter), anchor world position/normal, source brick/revision, source material, and shared render variant. It is derived only for eligible upward-facing topsoil/grass-like material surfaces with normal Y `>= WorldRenderConfig.dressing_min_normal_y` (`0.82`), using a stable surface-cell hash and biome density. It has no voxel occupancy and no save record.

When a mutation intersects an anchor’s source cells, the old instance is hidden immediately. After the new surface commits, dressing for the changed brick is regenerated: it disappears if no eligible support exists or obtains a new deterministic anchor on the changed surface. Trees, bushes, boulders, stumps, rocks, and the ruin are never represented as dressing.

## Runtime/demo entities and resources

- `Player` entity components: `Player`, `KinematicBody`, `Velocity`, `GroundContact`, `WaterContact`, `PlayerIntent`, `ActionState`, and `Transform`. No stats, inventory, health, or combat components exist.
- `CameraRig` entity components: orbit yaw/pitch/distance, collision radius, target player, and camera child transform.
- `UndergroundLight` is a player child point-light with presentation-only enable state.
- Global resources: immutable configs, `BaseRegion`, `MaterialRegistry`, `WorldQueryService`, `EditDeltaStore`, `StreamingFocuses`, task queues, diagnostic modes, fixed solar time, and benchmark recorder. Per-brick/per-object/per-player state remains components or keyed storage entries, not singleton resources.
- `DebugOperation` is the semantic action/command record containing operation kind, quantized target, radius, strength, and selected place material where applicable. View toggles/time adjustment are `DiagnosticCommand`/`FixedSolarTime` updates rather than fake voxel edits.
- `BenchmarkRun` is the report aggregate containing scenario ID, full machine/run profile, resolution, frame samples/summary, cold-start sample, mutation event correlations, graphics-memory samples, save metrics, and pass/fail reasons as specified in `benchmarks.md`.

## Relationships and authority

`CurrentWorld(coord) = EditDeltaStore.get(coord).unwrap_or(BaseRegion.sample(coord))`. Collision, raycasts, meshing, raw voxel display, mutation, save comparison, and object editing call this same composite query. `SurfaceOwner(coord) = sample_base(coord).ObjectId.map(Object).unwrap_or(Terrain)` is a separate reproducible presentation attribution and never changes the value returned by `CurrentWorld`. Streaming may cache either answer but cannot change it. Render entities point back to source brick/object IDs and revision; they never own material truth.
