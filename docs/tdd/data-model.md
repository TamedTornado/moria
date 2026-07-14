# Data Model and Persistence

## Coordinate and scalar conventions

Authoritative coordinates are independent of Bevy transforms:

```rust
pub struct VoxelCoord { pub x: i32, pub y: i32, pub z: i32 }
pub struct BrickCoord { pub x: i16, pub y: i16, pub z: i16 }
pub struct ColumnCoord { pub x: i32, pub z: i32 }
pub struct WorldPointQ8 { pub x: i32, pub y: i32, pub z: i32 }
```

`WorldPointQ8` is metres multiplied by 256. The voxel edge is exactly 0.25 m, so one voxel is 64 Q8 units. Conversion uses floor division for negative values; tests must cover every face around the origin. Region world bounds are `x,z in [-500, 500)` metres and `y in [-128, 128)` metres. Therefore the addressable grid is 4,000 x 1,024 x 4,000 voxels and 250 x 64 x 250 bricks. The typical generated surface is near world elevation +64 m. The curated cave entrance is near 0 m and its floor reaches `-40 +/- 2 m`, reconciling the requested route elevation with the region-wide typical surface.

`BrickCoord` is relative to the region minimum and cannot be constructed publicly without bounds validation. A brick contains 16 x 16 x 16 voxels, spans exactly 4 m, and uses local linear order `x + 16 * (z + 16 * y)`. Stable save ordering is lexicographic `(x, y, z)`, followed by local index.

Authoritative density is `u8`: `0` is empty and `255` is fully filled. The word “occupied” is not used as a technical predicate because matter, collision, and water have different semantics. These exact predicates are authoritative:

```rust
fn material_present(v: Voxel) -> bool {
    v.material != AIR && v.density > 0
}
fn water_volume(v: Voxel) -> bool {
    v.material == WATER && v.density > 0
}
fn solid_collision(v: Voxel, materials: &MaterialRegistry) -> bool {
    materials[v.material].collision_class == CollisionClass::Solid && v.density >= 128
}
```

`air` has `CollisionClass::Empty`, `water` has `CollisionClass::Fluid`, and every other Product One material has `CollisionClass::Solid`. Collision and `QueryMask::SOLID` use only `solid_collision`; paddling and `QueryMask::WATER` use only `water_volume`; raw diagnostics use `material_present` and label each cell as solid or water. Surface extraction evaluates the same solid-material scalar crossing at density 128 and never extracts water as solid geometry, but its output is partitioned by the provenance-aware `solid_presentation_owner` contract below rather than sending every crossing to terrain. Surface extraction linearly interpolates that crossing for presentation. A `MaterialId(u8)` and `ObjectId(u64)` are stable registry/instance identifiers. IDs are never array positions exposed to consumers.

## Design entity mapping

| Design entity | Technical representation | Ownership |
|---|---|---|
| World Seed | `WorldSeed` / `WorldIdentity` | Immutable world library value/resource |
| Base Region | private `BaseRegion { identity, config, manifest }` | Globally unique `moria-world` resource |
| Column | on-demand `ColumnSample` | Pure generated value / bounded cache |
| Voxel Brick | `BrickCoord`, private `BrickRecord`, `BrickDelta` | Sparse private store, not an ECS entity |
| Voxel | four-byte `Voxel` | Detailed brick/delta value |
| Material | `MaterialId`, `MaterialDef`, registry | Immutable library resource |
| Geological Feature | `FeatureInstance` plus pure signed-field evaluators | Manifest/base generation |
| Water Body | `WaterBodyDef` plus derived water volume/mesh | Manifest/base generation |
| Voxel Object | `ObjectPlacement`, `VoxelObjectShape`, per-placement render root or revisioned `HorizonObjectCell` payload | Base generation and presentation |
| Surface Dressing | `DressingInstance`, `SurfaceAnchor` | Revision-derived presentation only |
| Ruin Point of Interest | `RuinPoi`, `SparseVoxelStamp` | Generated placement plus one authored stamp |
| Player | `Player`, `CharacterBody`, `PlayerIntent`, `WaterContact` | Demo ECS components |
| Camera | `OrbitCamera` | Demo ECS component |
| Debug Operation | `DebugToolState`, `WorldEditCommand`, diagnostic flags | Demo state plus public command/results |
| Edit Delta Set | private sorted `BTreeMap<BrickCoord, BrickDelta>` | Authoritative mutable overlay/single save |
| Benchmark Run | `BenchmarkReport` and scenario FSM | Benchmark consumer/output |

The current World State is not a second copy: it is the pure composition `BaseRegion + Edit Delta Set`, observed at a monotonically increasing revision. Presentation and consumer entities are downstream of that composition.

## Seed, region, and generated metadata

```rust
pub struct WorldSeed {
    pub value: u64,
    pub parameters_digest: [u8; 32],
}

pub struct WorldIdentity {
    pub seed: u64,
    pub parameters_digest: [u8; 32],
    pub bounds: WorldBounds,
}

struct BaseRegion {
    identity: WorldIdentity,
    config: Arc<RegionConfig>,
    manifest: Arc<CuratedManifest>,
}

pub struct RegionConfig { /* typed fields in config.md */ }

pub struct CuratedManifest {
    pub seed: u64,
    pub parameters_digest: [u8; 32],
    pub generated_by: String,
    pub features: Vec<FeatureInstance>,
    pub water_bodies: Vec<WaterBodyDef>,
    pub objects: Vec<ObjectPlacement>,
    pub ruin: RuinPoi,
    pub route: TraversalRoute,
}
```

`objects` is the canonical serialized order, sorted by `ObjectId`. Runtime constructs one private immutable `ObjectSpatialIndex` during manifest validation and retains it only on success. It has one fixed-size `ObjectIndexRecord { raw_bounds, dependency_bounds }` per placement and two sorted horizontal tables:

- a 32 m `DependencyGridCell { key, members: Vec<u32> }` table indexes `dependency_bounds` for activation and edit dirty discovery; and
- a 4 m `SampleGridCell { key, members: Vec<u32> }` table indexes `raw_bounds` so a synchronous voxel evaluation tests at most 64 analytic object shapes rather than scanning a dense 32 m cell.

Members are manifest indices sorted by `ObjectId`. Both tables exact-filter their respective bounds/predicates; the fine table does not change base precedence. No dependency coordinate vector, bitmap, per-voxel shape expansion, or duplicate ID lookup map is retained. Validation rejects an object touching more than 16 cells in either table, a dependency footprint touching more than 128 voxel bricks, a dependency cell containing more than 1,024 placements, a sample cell containing more than 64 placements, any supported radius-3 m edit broad phase exceeding 256 placements or exact dependency result exceeding 64 placements, or total retained index capacity above 16 MiB. These limits are configuration contracts, not silent truncation.

Checked memory accounting is `record_capacity * size_of::<ObjectIndexRecord>() + sum(table_cell_capacity * size_of::<TableCell>()) + sum(all_member_capacities * size_of::<u32>()) + 16 * (2 + dependency_occupied_cells + sample_occupied_cells)`. The allocator term covers both outer vectors and every occupied-cell member vector. `retained_index_bytes` therefore includes both grids, fixed records, dependency metadata, reserved capacity, keys, and allocator padding and is reported in startup telemetry. Runtime validation/build timing includes count/spacing/canopy/route/disjointness validation plus construction of both tables; F1 caps the combined phase at 1,000 ms and the table-build subset at 250 ms on the M4.

The same validation rejects `ObjectShapeOverlap { lower_id, higher_id, first_voxel }` when two non-ruin raw shape evaluators are both solid at any voxel coordinate, and `ObjectRuinOverlap { object_id, first_voxel }` when a non-ruin raw shape is solid at any transformed authored ruin-stamp coordinate, including an explicit air-carve coordinate. Candidate pairs come from the dependency grid, exact intersection scans use lexicographic voxel order, and the first reported conflict is ordered by `(lower_id, higher_id, first_voxel)`. It also emits exact `ForestContractViolation` variants for area/count/species, spacing, canopy range-bin, route clearance, sample/dependency/Horizon cell capacity, edit-candidate capacity, validation time, and retained bytes. Thus every accepted non-ruin placement owns its complete raw voxel shape and may safely use an unmasked authored root. ID lookup binary-searches the already sorted manifest vector rather than retaining a companion map. The compact index records are derived memory and are neither saved nor exposed as mutable state.

`WorldSeed` plus `RegionConfig` is the base-world identity. `parameters_digest` is SHA-256 over canonical RON config bytes and the ruin-stamp content; the implementation uses a small SHA-256 crate only for this identity check. The digest is not a save-version mechanism. `moria-curate generate` deterministically searches feature candidates from the seed and writes `assets/config/curated_manifest.ron`; `check` regenerates it in memory, compares canonical values, and validates all feature contracts. The manifest is generated metadata, not hand-sculpted terrain. Runtime rejects a seed/config/manifest mismatch before opening the world.

The manifest contains at most 16 generated `FeatureInstance` records/evaluator dispatches; repeated strata are represented parametrically by one stratum evaluator rather than an unbounded record list. This cap is validated before `WorldReady` and is the `F <= 16` term in public query costs.

`TraversalRoute` is an ordered list of generated waypoints with semantic tags: meadow, forest, river, lake, cliff_top, rock_shelves, ruin_stair_bottom/top, cave_mouth, aquifer, ore_vein, cave_floor, and `signature_carve_hillside`. It contains at most 64 waypoints. The signature tag identifies an in-bounds solid hillside at which a radius-3 m dig creates a capsule-clear through-route; it is metadata, not a voxel override. Route data is public observation used by demo spawn and benchmark scripts. The curator accepts a seed only if the route is connected by `solid_collision` queries, the entrance/floor elevation contract passes, the river/lake occupy carved basins, both tree species occur in the forest, the aquifer and iron feature intersect the cave route, and the signature carve target passes its before/after pure occupancy oracle.

### Column

`ColumnSample` is a value returned on demand, not one of 16 million permanently allocated entities:

```rust
pub struct ColumnSample {
    pub coord: ColumnCoord,
    pub surface_y_q8: i32,
    pub runs: Vec<ColumnRun>,
    pub biome: BiomeId,
    pub feature_mask: u32,
}

pub struct ColumnRun {
    pub y_min_voxel: i16,
    pub y_max_voxel_exclusive: i16,
    pub material: MaterialId,
    pub kind: RunKind, // Matter, Air, Water, or CaveGap
}
```

`ColumnEvaluator` combines broad fixed-point terrain, biome, strata, cave, ore, aquifer, water, and POI evaluators. A bounded tile cache may retain active column samples, but eviction cannot affect output.

### Geological feature

```rust
pub enum FeatureKind {
    Topsoil, Subsoil, Stratum, KarstCave, Aquifer, IronVein,
}

pub struct FeatureInstance {
    pub id: u32,
    pub kind: FeatureKind,
    pub bounds: AabbQ8,
    pub host_material: MaterialId,
    pub depth_q8: i32,
    pub orientation_q16: [i32; 4],
    pub generator_key: u64,
}
```

The orientation is a normalized fixed-point quaternion/plane representation selected by feature kind. Strata use tilted planes with bounded perturbation; the cave uses a connected spline-and-chamber signed field; the aquifer uses a finite band volume; the vein uses a swept branching volume. `FeatureInstance` contains parameters, not expanded voxels. At a coordinate, feature precedence is: ruin, non-ruin object, cave void, ore vein, aquifer material, host stratum, subsoil, topsoil, water, air. The unique ruin wins an overlap with another object and the lowest `ObjectId` wins among overlapping non-ruin candidate shapes so curator candidate evaluation is total and deterministic. Such object conflicts are invalid in a persisted/accepted manifest, however: world opening performs the exact overlap validation above before exposing a `BaseRegion`. Therefore an opened world's base evaluation has exactly one `VoxelSource` without masking part of any accepted non-ruin shape. Cave void wins over geology so the route remains open; exposed walls still reveal the intersecting aquifer and ore on their surface.

### Water body

```rust
pub enum WaterKind { River, Lake }
pub struct WaterBodyDef {
    pub id: u32,
    pub kind: WaterKind,
    pub surface_y_q8: i32,
    pub footprint: Vec<WorldPointQ8>,
    pub bed_profile_key: u64,
}
```

The footprint and bed profile contribute to terrain generation before water volume is assigned, proving that the channel/basin is real. Water voxels have water material and nonzero density, so `material_present` and `water_volume` are true while `solid_collision` is false. No velocity, pressure, volume transfer, or update state exists.

## Material and voxel truth

```rust
#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Voxel {
    pub material: MaterialId, // u8
    pub density: u8,
    pub state: u8,
    pub flags: u8,
}

pub struct MaterialDef {
    pub id: MaterialId,
    pub key: String,
    pub hardness: u8,
    pub granular: bool,
    pub collision_class: CollisionClass, // Empty, Fluid, or Solid
    pub surface_class: SurfaceClass,
    pub albedo_layer: u16,
    pub normal_layer: u16,
    pub roughness: u8,
}
```

`Voxel` is exactly four bytes and has no padding. `state` is always zero in Product One and is round-tripped by edits/saves; no system reads it to produce behavior. `flags` is reserved for representation-local flags and must serialize as zero until a documented Product One use exists. Material registry keys are exactly: `air`, `water`, `topsoil`, `subsoil`, `sand`, `gravel`, `limestone`, `sandstone`, `shale`, `granite`, `iron_ore`, `wood`, `leaf`, and `cut_stone`. IDs are assigned in that canonical order and validated at load. Sand and gravel set `granular = true`, but no settling system is registered.

Hardness scales partial/falloff debug erosion. The deterministic spherical kernel computes a Q8 distance and an integer weight `w`: the inner 70% has `w = 255`, then weight decreases linearly to zero at the radius. A full-strength dig always subtracts 255 density in the inner core so the signature action opens matter of any material. Outside the core or at lower tool strength, erosion is `ceil(strength * w * hardness_reference / (255 * hardness))`, capped at 255; `hardness_reference` is 64. When density reaches zero, material becomes air and state/flags become zero. Place increases density in available space with the same fixed-point spatial weight (hardness does not resist placement) and sets the selected material wherever it contributes density; water can be displaced because placement is a proof operation. Place rejects `air` and `water` as selected construction materials. Material defaults are in [config.md](config.md).

## Sparse brick store

The private authoritative resource is conceptually:

```rust
struct WorldStore {
    identity: WorldIdentity,
    active: HashMap<BrickCoord, BrickRecord>,
    deltas: BTreeMap<BrickCoord, BrickDelta>,
    revision: u64,
}

enum BrickBase {
    Procedural(ProceduralClass),
    Uniform(Voxel),
    Detailed(Box<[Voxel; 4096]>),
}

struct BrickRecord {
    base: BrickBase,
    revision: u64,
    dirty: DirtyFlags,
    band: ActiveBand,
    pin_count: u16,
}
```

`WorldStore` and all fields are private to `moria-world`. A procedural classifier uses conservative density bounds to mark bricks wholly air, water, or one geology material without evaluating all voxels. A brick crossing any density/material boundary becomes detailed only while required for collision, mutation, raw inspection, or mesh production. A mutated brick's current detail can be evicted after its `BrickDelta` is retained; reactivation regenerates base and reapplies that delta.

`DirtyFlags` distinguishes collision/query truth, mesh, seam, water, registered-object visuals, Horizon-cell aggregate membership, surface anchors, and dressing. An edit increments the global revision once, assigns that revision to every changed brick, and dirties changed bricks plus face-neighbor mesh seams. The object index batch-queries the sorted changed coordinates and returns placements whose lazy `ObjectSurfaceDependency` contains at least one of them; every returned non-ruin ID sets `registered-object visuals`, including when a matching coordinate's own base source is terrain or ruin. A returned Horizon-visible tree also dirties its anchor-owned `HorizonCellKey`, whether eligibility changed or only its derived payload changed. This lookup and the mesh snapshot builder use the same extraction stencil defined below, so a boundary input cannot affect an object primitive without dirtying both its per-placement and Horizon consumers. `Ruin(ObjectId)` does not set those flags because ruin presentation is owned by the always-voxel-derived terrain chunk path already dirtied for the changed brick and seams. A changed support surface sets `surface anchors`. The registered-object flag drives the intact/voxel-derived vegetation/prop transition or rebuilds an already-derived payload, while the latter regenerates dressing and never invents object physics. Edge/corner sampling comes from the immutable snapshot and does not require dirtying chunks whose outputs cannot depend on the changed sample. Background results carry their source key, request token, desired LOD, and relevant content revision and are discarded if any no longer match.

Bricks are not modeled as ECS entities in the authoritative store. Derived render chunks are entities with `TerrainChunk { key: RenderChunkKey, lod, revision }`; a key covers one near brick or an aligned group of horizontal bricks at coarser surface LOD. This distinction prevents Bevy queries—including Bevy 0.19 resource-backed entities—from becoming a mutation channel.

## Objects, ruin, and dressing

```rust
pub enum ObjectKind { TreeA, TreeB, Bush, Boulder, Stump, Rock, Ruin }

pub struct ObjectPlacement {
    pub id: ObjectId,
    pub kind: ObjectKind,
    pub transform_q: QuantizedTransform,
    pub species: Option<SpeciesId>,
    pub shape: VoxelObjectShape,
    pub anchor: VoxelCoord,
}

pub struct QuantizedTransform {
    pub translation: WorldPointQ8,
    pub yaw_quarter_turns: u8,
    pub uniform_scale_q8: u16,
}

pub enum VoxelObjectShape {
    Tree { trunk_radius_q8: u16, trunk_height_q8: u16, canopy_radii_q8: [u16; 3] },
    Bush { radii_q8: [u16; 3] },
    Boulder { radii_q8: [u16; 3], perturbation_key: u64 },
    Stump { radius_q8: u16, height_q8: u16 },
    Rock { radii_q8: [u16; 3], perturbation_key: u64 },
    SparseStamp { asset_key: String },
}

pub struct RuinPoi {
    pub placement: ObjectPlacement,
    pub stair_bottom: WorldPointQ8,
    pub stair_top: WorldPointQ8,
}

pub struct DressingInstance {
    pub key: DressingKey,
    pub kind: DressingKind,
    pub anchor: SurfaceAnchor,
    pub transform: Transform,
}

pub struct SurfaceAnchor {
    pub brick: BrickCoord,
    pub cell: [u8; 3],
    pub source_revision: u64,
    pub material: MaterialId,
}
```

Tree, bush, boulder, stump, rock, and ruin placements are registered generated objects with stable IDs. Trees/bushes/stumps use fixed-point trunk/canopy/ellipsoid primitives; boulders/rocks use perturbed fixed-point ellipsoids; and the ruin uses its sparse stamp. Every shape is sampled into base voxel truth on demand. An accepted manifest guarantees that no two non-ruin raw shapes share a solid voxel and no non-ruin raw shape intersects an authored ruin-stamp coordinate; this is a presentation-safety invariant, not collision precedence. Tree, bush, boulder, stump, and rock presentation uses shared authored GLB meshes while intact; the ruin is the exception and is always displayed by the same revisioned, material-aware voxel mesh extraction used for stamped/current cut-stone truth. It has no GLB or intact-authored presentation state. Wood/leaf primitives assign their matching materials, rocks inherit the placement's generated host rock, and the ruin assigns cut stone. No object has velocity, health, growth, fire, or lifecycle simulation components.

Base evaluation internally retains `VoxelSource::Terrain | Object(ObjectId) | Ruin(ObjectId)` alongside the resulting base `Voxel`; this provenance is regenerated and is not serialized. A delta replaces only the `Voxel`, never its base provenance: a placed/digged cell whose base source is an object remains attributed to that object, a delta in ordinary terrain remains terrain-attributed, and a delta in the ruin stamp (including an authored air-carve cell) remains ruin-attributed. Delta removal on exact reversion therefore cannot change attribution.

Normal solid-world presentation uses one pure partition predicate:

```rust
enum SolidPresentationOwner {
    TerrainChunk,          // VoxelSource::Terrain or VoxelSource::Ruin(_)
    NonRuinObject(ObjectId),
}

fn solid_presentation_owner(
    current: Voxel,
    base_source: VoxelSource,
    materials: &MaterialRegistry,
) -> Option<SolidPresentationOwner> {
    if !solid_collision(current, materials) { return None; }
    Some(match base_source {
        VoxelSource::Terrain | VoxelSource::Ruin(_) => SolidPresentationOwner::TerrainChunk,
        VoxelSource::Object(id) => SolidPresentationOwner::NonRuinObject(id),
    })
}
```

This predicate is carried in every solid-mesh snapshot and is the only routing rule used by terrain, ruin, and derived-object extraction. It does not affect collision/query results. A current solid cell has exactly one normal presentation owner; a non-solid cell has none. Diagnostic raw-voxel overlay is intentionally outside this normal-world partition and is shown only when the user enables that debug mode.

For a non-ruin placement `id`, `ObjectSurfaceDependency(id)` is a mathematical set, not a stored collection. Each supported object LOD declares a finite, compile-time extraction stencil containing every relative density, material, provenance, downsampling, and gradient input it can read when emitting a primitive from an owner cell. The union stencil is capped at 512 unique offsets and tested against the extractor's instrumented read trace. Given the placement's analytic raw-shape predicate, exact membership is evaluated lazily:

```rust
fn dependency_contains(placement: &ObjectPlacement, coord: VoxelCoord) -> bool {
    OBJECT_EXTRACTION_STENCIL.iter().any(|offset| {
        raw_shape_contains(placement, coord.checked_sub(*offset))
    })
}
```

Checked subtraction outside the region is false. This is exactly the union of every extractor input associated with a base owner cell of `NonRuinObject(id)` at every supported object LOD; it includes object cells and the adjacent terrain/ruin halo that can affect the object's position, normal, material weights, emission, or occlusion. `dependency_bounds` is the raw voxel AABB expanded by the minimum/maximum union-stencil offsets and clipped to the region, so it is a conservative constant-size broad phase. Dirty discovery, delta intersection, and snapshot construction use the same stencil definition and `dependency_contains`; changing a coordinate outside this mathematical set cannot change the owner-filtered payload. An explicit sorted-set enumerator exists only under `cfg(test)` as a small-shape oracle and is never used during manifest loading, startup, streaming, or normal extraction.

Only a non-ruin `Object(ObjectId)` has dependency eligibility `Intact | VoxelDerived { revision }`; this is independent of its current distance band. It is `Intact` exactly when `delta_intersects_dependency(id)` is false. That query enumerates only the at-most-128 brick coordinates overlapped by `dependency_bounds`, probes the sparse `BTreeMap<BrickCoord, BrickDelta>`, and tests only present `VoxelDelta` coordinates with `dependency_contains`, returning on the first match. With `b <= 128`, `D` delta bricks, `m` stored deltas in those bricks, and stencil size `s <= 512`, its worst-case work is `O(b log D + m*s)`, allocation is `O(1)`, and inactive objects are never queried merely to establish eligibility. This condition guarantees that every extraction input equals base, for which an authored GLB or base Horizon card is the intentional presentation of the raw analytic object shape. Any object-, terrain-, or ruin-attributed dependency delta excludes both authored forms and selects an owner-filtered current-truth payload at the active LOD. Authored presentation returns only after all such deltas exactly revert to base; reverting merely the object-attributed cells while an adjacent boundary delta remains cannot restore it. Because overlap validation guarantees that every accepted raw shape cell is routed to its own ID, terrain chunks emit none of the object's primitives in either state.

Horizon-visible trees are assigned by anchor position to one 64 m `HorizonCellKey` aligned relative to the region minimum; assignment never depends on activation or edits. A resident Horizon cell is one logical revisioned payload:

```rust
struct HorizonObjectCell {
    key: HorizonCellKey,
    token: u64,
    source_revision: u64,
    base_card_ids: Vec<ObjectId>,
    derived: Vec<HorizonDerivedObject>,
}

struct HorizonDerivedObject {
    id: ObjectId,
    revision: u64,
    // The mesh may be empty when current truth contains no owned surface.
    mesh: Option<DerivedMeshKey>,
}
```

Both vectors are sorted by `ObjectId`, disjoint, and contain every Horizon-visible tree assigned to the cell exactly once: an `Intact` tree contributes one base card ID, while a `VoxelDerived` tree is absent from the cluster and contributes one owner-filtered 4 m derived record, including an empty tombstone for a fully removed tree. Non-tree objects are intentionally culled beyond `object_visibility_m` and therefore are not members of the Horizon-visible set. `source_revision` is the world revision whose sparse deltas were snapshotted for all members. A task may install only if its token and cell's desired source revision still match; the entire cluster buffer, derived records, and removals swap as one deferred batch. Immutable base card descriptors may be cached by `(WorldIdentity, HorizonCellKey)`, but filtered membership, derived payloads, installed entities, and GPU buffers may not be reused by cell key alone. They are discarded on eviction or keyed by a digest of the exact relevant deltas, and activation always re-evaluates eligibility from the current delta map. Thus eviction, reactivation, and load cannot resurrect a base card from stale aggregate state.

A `Ruin(ObjectId)` instead has `RuinVisualState::VoxelDerived { revision }` from initial activation onward, while its cells route to `TerrainChunk`. Editing or exactly reverting stamped ruin cells merely advances that voxel-derived chunk mesh to current world revision; it never adds or restores a GLB. Edits spanning an object boundary retain the base partition per coordinate: object-attributed parts belong to that object's root and terrain/ruin-attributed parts belong to terrain chunks. These are presentation states only; collision/material queries always use current voxel values. Edits to support terrain do not cause felling or physics—the registered placement remains static as required—but any support edit in its surface dependency is visually represented by the derived root until that boundary returns to base.

Dressing is deterministically derived from a hash of seed, surface cell, biome density, and dressing kind. An anchor is eligible only when its current density surface is upward-facing above the configured normal threshold and its material is topsoil. On a mesh revision, dressing for the changed chunk is destroyed and regenerated. Thus removing or moving the surface necessarily removes or relocates grass; dressing is neither in `WorldStore.deltas` nor the save file.

## Demo ECS data

- `Player` is a marker component. `CharacterBody` stores capsule radius/height, grounded state, and velocity; `PlayerIntent` stores semantic move, sprint, jump-edge latch, and look input until consumed; `WaterContact` stores dry/paddling and sampled surface level; `UndergroundLight` marks the child light.
- `OrbitCamera` stores yaw, pitch, desired distance, collision-adjusted distance, and target offset. It is per camera, not a resource.
- `ActionMap` is a singleton resource mapping physical controls to `PlayerAction`, `DebugAction`, and `UiAction`. `ActionSnapshot` is per-player/per-frame input state; fixed-step `PlayerIntent` is a component so the boundary can later support more than one controlled entity without redesign.
- `DebugToolState` is a singleton because Product One has one local diagnostic tool. It stores operation, radius, selected material, and visualization toggles. It does not store voxel references.
- `SunState` is a singleton containing fixed time-of-day; it never advances automatically.

## Edit delta set and save file

`BrickDelta` is the sorted set of voxel values that differ from fresh base evaluation:

```rust
struct VoxelDelta { local_index: u16, value: Voxel }
struct BrickDelta { brick: BrickCoord, voxels: Vec<VoxelDelta> }
```

After each edit, changed cells equal to base are removed from the delta. Empty brick deltas are removed. This means filling a prior dig back to the exact base value shrinks the save and that the save always represents current differences, not operation history.

The single slot defaults to the platform user-data directory as `moria/product-one/world.delta`; tests and CLI may inject a path. Saving writes a temporary sibling, flushes it, then atomically renames it. File layout before zstd compression is:

```text
8 bytes  magic = "MORIADEL"
8 bytes  seed, little-endian
32 bytes parameters digest
4 bytes  brick count, little-endian u32
repeated sorted bricks:
  6 bytes brick x/y/z as little-endian i16
  2 bytes changed voxel count as u16 (1..4096)
  repeated: 2-byte local index + 4-byte Voxel
32 bytes SHA-256 of all preceding uncompressed bytes
```

The outer zstd stream uses level 3 to keep interactive saves fast. No version field or migration dispatcher exists, honoring the single-version scope. A wrong magic, seed/digest mismatch, out-of-bounds coordinate, duplicate/unsorted index, invalid material ID, checksum failure, or decompression failure returns `LoadError` and leaves the current world unchanged. On successful load, base world identity is established first, all deltas are validated in a staging map, and the map is swapped atomically into `WorldStore` before nearby derived content is rebuilt.

`u32` counts cap the format and satisfy the 32-bit counter constraint. The heavy-defacement benchmark must remain below 50,000,000 compressed bytes. Save size is measured from the final file, not an estimate.

## Feasibility evidence

The pre-implementation gates use two serializable reports separate from the final `BenchmarkReport`. Both share `BuildProfile`, `WorldIdentity`, and `MachineProfile`, use RFC 3339 timestamps, store sorted failure reasons, and validate `passed == failure_reasons.is_empty()`.

```rust
pub struct ForestFeasibilityReport {
    pub schema: String, // "moria-product-one-forest-feasibility"
    pub timestamp_utc: String,
    pub passed: bool,
    pub failure_reasons: Vec<String>,
    pub build: BuildProfile,
    pub world: WorldIdentity,
    pub manifest_sha256: String,
    pub machine: MachineProfile,
    pub forest_area_m2: u32,
    pub eligible_land_area_m2: u32,
    pub object_counts: BTreeMap<String, u32>,
    pub required_object_counts: BTreeMap<String, u32>,
    pub species_counts: BTreeMap<String, u32>,
    pub minimum_tree_spacing_q8: u32,
    pub canopy_min_q8: u16,
    pub canopy_max_q8: u16,
    pub canopy_range_bins: BTreeMap<String, u32>,
    pub minimum_route_clearance_q8: u32,
    pub overlap_conflicts: u32,
    pub first_conflict: Option<String>,
    pub object_index: ObjectIndexEvidence,
    pub worst_edit_target: WorstEditTargetEvidence,
}

pub struct WorstEditTargetEvidence {
    pub center: WorldPointQ8,
    pub broad_candidates: u16,
    pub exact_dependency_ids: u16,
    pub dependency_bricks: u16,
    pub tie_break_rank: u32,
}

pub struct MutationFeasibilityReport {
    pub schema: String, // "moria-product-one-mutation-feasibility"
    pub timestamp_utc: String,
    pub passed: bool,
    pub failure_reasons: Vec<String>,
    pub build: BuildProfile,
    pub world: WorldIdentity,
    pub manifest_sha256: String,
    pub forest_report_sha256: String,
    pub machine: MachineProfile,
    pub resolution: [u32; 2],
    pub backend: String,
    pub cold_start_ms: f64,
    pub workloads: Vec<MutationWorkloadEvidence>,
    pub query_costs: QueryCostEvidence,
}

pub struct MutationWorkloadEvidence {
    pub role: MutationWorkloadRole,
    pub request_count: u32,
    pub submitted_frame: u64,
    pub first_committed_frame: u64,
    pub final_reconciled_frame: u64,
    pub admission_ms: Distribution,
    pub first_commit_ms: Distribution,
    pub primary_ready_ms: Distribution,
    pub reconciliation_ms: Distribution,
    pub changed_bricks_per_second: f64,
    pub maximum_runnable_wait_ms: f64,
    pub maximum_frame_ms: f64,
    pub traversable: bool,
    pub changed_voxels: u64,
    pub changed_bricks: u32,
    pub committed_batches: u32,
    pub stage_timings_ms: BTreeMap<String, f64>,
    pub stage_counts: BTreeMap<String, u64>,
    pub barrier_expected_items: u32,
    pub barrier_renderer_ready_items: u32,
    pub horizon_partition_checked: bool,
    pub horizon_excluded_base_cards: u16,
    pub horizon_derived_records: u16,
}

pub enum MutationWorkloadRole { InteractiveCarve, ColonyVolume, CatastrophicCarve }

pub struct QueryCostEvidence {
    pub sample_counts: BTreeMap<String, u32>,
    pub cold_inactive_calls: BTreeMap<String, Distribution>,
    pub frame_critical_calls: BTreeMap<String, Distribution>,
    pub normal_bundle_ms: Distribution,
    pub column_ms: Distribution,
    pub diagnostic_metadata_page_ms: Distribution,
    pub diagnostic_cells_page_ms: Distribution,
    pub observed_work_maxima: BTreeMap<String, u64>,
}
```

Required mutation stage keys are `admission`, `schedule`, `edit-stage`, `commit`, `dirty-discovery`, `dependency-eligibility`, `snapshot`, `terrain-mesh`, `object-mesh`, `seams`, `dressing-remove`, `dressing-install`, `bevy-install`, `primary-ready`, `render-extract`, `gpu-upload`, `render-queue`, and `reconciliation`. A legitimate no-work branch records count `0` and elapsed time rather than omitting its key. `dirty-discovery + dependency-eligibility <= 1.0 ms` is evaluated at the catastrophic stress target, not from rounded distributions. Report validators reject non-finite times, wrong M4/Metal/resolution/profile identity, manifest-digest mismatch, missing keys, count/cap violations, starvation, and expected/renderer-ready reconciliation inequality. Failed artifacts are immutable inputs to review and are never rewritten as passing reports.

## Benchmark data

```rust
pub struct BenchmarkReport {
    pub schema: String, // literal "moria-product-one-benchmark"
    pub timestamp_utc: String, // RFC 3339 UTC
    pub scenario: ScenarioName,
    pub passed: bool,
    pub failure_reasons: Vec<String>,
    pub baseline_status: BaselineStatus,
    pub build: BuildProfile,
    pub world: Option<WorldIdentity>,
    pub assets: Option<AssetEvidence>,
    pub machine: Option<MachineProfile>,
    pub resolution: Option<[u32; 2]>,
    pub cold_start_ms: Option<f64>,
    pub frame_rate: Option<FrameRateMetrics>,
    pub frame_time_ms: Option<Distribution>,
    pub mutation_latency: Option<MutationLatencyMetrics>,
    pub graphics_memory: Option<GraphicsMemoryEvidence>,
    pub save: SaveEvidence,
    pub coverage: Option<CoverageEvidence>,
    pub streaming: Option<StreamingEvidence>,
}

pub struct MutationLatencyMetrics {
    pub sample_count: u32,
    pub admission_ms: Distribution,
    pub accepted_to_first_commit_ms: Distribution,
    pub commit_to_primary_ready_ms: Distribution,
    pub accepted_to_reconciliation_ms: Distribution,
    pub changed_bricks_per_second: f64,
    pub maximum_runnable_wait_ms: f64,
    pub representative_max_frame_ms: f64,
}

pub struct GraphicsMemoryEvidence {
    pub application_ledger: GraphicsMemoryEstimate,
    pub resident_measurement: Option<ResidentGraphicsMeasurement>,
    pub product_target_proven: bool,
    pub estimate_substitution_approval_id: Option<String>,
}

pub struct SaveEvidence {
    pub attempted: bool,
    pub completed: bool,
    pub size_bytes: Option<u64>,
    pub changed_voxels: Option<u32>,
    pub changed_bricks: Option<u32>,
    pub round_trip: Option<RoundTripEvidence>,
}

pub enum ScenarioName { Flythrough, CarveStorm }
pub enum BaselineStatus { Provisional, Verified }
pub struct BuildProfile {
    pub cargo_profile: String,
    pub git_commit: String,
    pub rustc_version: String,
}
pub struct AssetEvidence {
    pub manifest_sha256: String,
    pub fallbacks: Vec<String>,
    pub warnings: Vec<String>,
}
pub struct MachineProfile {
    pub profile_id_sha256: String,
    pub os_name: String,
    pub os_version: String,
    pub architecture: String,
    pub cpu_model: String,
    pub logical_cores: u32,
    pub total_physical_memory_bytes: u64,
    pub gpu_adapter_name: String,
    pub gpu_vendor: u32,
    pub gpu_device: u32,
    pub gpu_device_class: String,
    pub wgpu_backend: String,
    pub driver: Option<String>,
    pub driver_metadata_available: bool,
    pub memory_architecture: String, // "unified" or "discrete"
    pub acceptance_label: String,
}
pub struct FrameRateMetrics {
    pub sample_count: u64,
    pub measured_seconds: f64,
    pub arithmetic_fps: f64,
    pub one_percent_low_fps: f64,
}
pub struct Distribution {
    pub min: f64, pub p50: f64, pub p95: f64, pub p99: f64, pub max: f64,
}
pub struct DistributionU64 {
    pub min: u64, pub p50: u64, pub p95: u64, pub p99: u64, pub max: u64,
}
pub struct GraphicsMemoryEstimate {
    pub peak_bytes: u64,
    pub end_bytes: u64,
    pub categories: BTreeMap<String, u64>,
    pub untracked_driver_overhead: bool, // always true
}
pub struct ResidentGraphicsMeasurement {
    pub provider: String,
    pub scope: String,
    pub sampling_interval_ms: u32,
    pub peak_bytes: u64,
    pub artifact_sha256: String,
    pub artifact_path: String,
}
pub struct RoundTripEvidence {
    pub passed: bool,
    pub delta_voxels_compared: u32,
    pub base_samples_compared: u32,
    pub identity_match: bool,
    pub derived_bytes_found: bool,
}
pub struct CoverageEvidence {
    pub route_tags_visited: Vec<String>,
    pub active_bands_entered: Vec<ActiveBand>,
    pub edited_material_counts: BTreeMap<String, u32>,
    pub final_changed_spheres: u32,
    pub final_changed_region_cells: u32,
    pub workload_minimum_met: bool,
}
pub struct StreamingEvidence {
    pub peak_active_counts: ActiveCounts,
    pub peak_queue_depths: QueueDepths,
    pub first_steady_derived_bytes: u64,
    pub return_steady_derived_bytes: u64,
    pub monotonic_growth_check_passed: bool,
    pub object_index: ObjectIndexEvidence,
}
pub struct ObjectIndexEvidence {
    pub validation_ms: f64,
    pub build_ms: f64,
    pub retained_bytes: u64,
    pub retained_byte_categories: BTreeMap<String, u64>,
    pub placement_records: u32,
    pub dependency_grid_entries: u32,
    pub sample_grid_entries: u32,
    pub max_dependency_cell_entries: u16,
    pub max_sample_cell_entries: u8,
    pub max_horizon_tree_members_per_cell: u16,
    pub max_edit_candidates: u16,
    pub max_edit_affected_objects: u8,
    pub max_dependency_bricks: u16,
    pub dependency_coordinate_allocation_bytes: u64,
}
```

Fallback/warning/tag/band vectors are sorted and duplicate-free; category maps serialize with lexicographically sorted keys. Missing driver metadata is explicit `null`, never omitted, and requires `driver_metadata_available == false`. `untracked_driver_overhead` must be true. Exact required/null rules and failure serialization are in [benchmarks.md](benchmarks.md); this is the sole report representation used by the runner and JSON contract.

## Relationship invariants

- `current_voxel(coord) == delta(coord).unwrap_or(base(seed, config, manifest, coord))` for every in-bounds coordinate.
- A render chunk, Horizon object-cell payload, dressing instance, water patch, and collision response names the world/content revision from which it was derived; stale task output can never replace a newer relevant revision.
- For each resident Horizon cell, sorted `base_card_ids` and `derived.id` values are disjoint and their union equals exactly the Horizon-visible tree IDs assigned to that cell; any ID with a dependency delta occurs only in `derived`, even when its mesh is empty.
- Regenerated base provenance is total and unique at every coordinate; `solid_presentation_owner` maps every current solid cell to exactly one normal-world presentation path without changing `solid_collision`, and exact delta reversion preserves that routing.
- Every `ObjectPlacement.anchor` is supported by generated solid collision, and every displayed registered object has exactly one placement ID. A ruin placement additionally has a valid connected stair path.
- Every dressing instance has one eligible current surface anchor; changing the anchor revision removes it before regeneration.
- Water exists only inside a `WaterBodyDef` footprint and between its carved bed and static surface. `water_volume` is true there and `solid_collision` is always false for water.
- A save contains only non-base voxel values, never columns, manifests, meshes, dressing, render assets, player state, camera state, time of day, or benchmark data.
