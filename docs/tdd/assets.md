# Asset Inventory and Pipeline

## Asset principles

All runtime assets live under repository-root `assets/`, Bevy's default lookup root. The base world is generated; assets provide reusable material appearance, registered-object visuals, one allowed sparse ruin stamp, the validation character, and diagnostic/config data. No terrain heightmap, cave mesh, river mesh, lake mesh, baked region scene, or hand-placed object scene is allowed. Generated `curated_manifest.ron` contains deterministic metadata, not expanded voxels or a replacement landscape.

Every binary loads assets through Bevy `AssetServer`. `moria-world` owns material/object/terrain asset collections; consumers receive readiness/errors but not mutable authoritative handles. Visual assets may be swapped without changing world identity unless they contain the ruin voxel stamp; only authoritative config and stamp content participate in the parameters digest.

## Directory and inventory

```text
assets/
  config/
    product_one_region.ron       # authoritative seed/generation parameters
    curated_manifest.ron         # generated/checked feature and route metadata
    presentation.ron             # streaming/render/player/camera defaults
    input.ron                    # physical-to-semantic bindings
  materials/
    materials.ron                # canonical material registry and layer mapping
    terrain_albedo.ktx2          # 14-layer array, unused air/water layers blank
    terrain_normal.ktx2          # matching layer/mip layout
    terrain_orm.ktx2             # occlusion/roughness/metallic array
    water_normal.ktx2
  stamps/
    ruin_p1.ron                  # sparse cut-stone/air voxel stamp and stair tags
  vegetation/
    birch_near.glb
    birch_mid.glb
    birch_far.glb
    pine_near.glb
    pine_mid.glb
    pine_far.glb
    bush_near.glb
    bush_far.glb
    grass_cluster.glb
    tree_horizon_cards.ktx2
  props/
    boulder.glb
    stump.glb
    rock.glb
  player/
    explorer.glb                 # mesh, skeleton, required locomotion clips
  shaders/
    terrain.wgsl
    water.wgsl
    vegetation.wgsl
    raw_voxel.wgsl
  manifests/
    asset_licenses.ron
    asset_budgets.ron
```

Birch and pine are the two concrete mixed-forest species. They are visual/species IDs in deterministic placement; adding more species is out of Product One scope. Boulders/stumps/rocks use a small set of mesh variants embedded as named glTF primitives if art needs variation, selected by stable placement hash. Variation never creates unique materials.

There are deliberately no combat, NPC, inventory, spell, mechanism, weather, dynamic-fluid, building-palette, sound/music, or narrative assets. The design does not require audio, so the implementation must not block startup on an invented audio set.

## Format specifications

### Meshes and animation

- Runtime object/player source is binary glTF 2.0 (`.glb`), metres, Y-up, +Z forward, transforms applied, origin at support/contact center.
- Object meshes use indexed triangles with `u32`-compatible counts, finite positions, normals, UV0, and tangents where normal maps require them. Vertex colors are optional presentation variation only.
- Near/mid/far LODs are separate files with identical origin/bounds conventions. No mesh file contains world-space placement.
- The explorer file contains named clips `Idle`, `Run`, `Sprint`, `Jump`, `Fall`, and `Paddle`. Missing optional transition clips blend between these; missing any named required clip is an asset validation error with a static-pose fallback during development.
- Collision meshes in glTF are ignored. Non-ruin registered-object collision uses voxel `solid_collision`; the ruin has no glTF asset and its presentation/collision both use stamped/current voxel truth; player collision uses the configured capsule.

Recommended initial triangle budgets, verified in `asset_budgets.ron`, are: near tree <= 12,000, mid <= 3,000, far <= 500; bush near <= 2,000/far <= 250; each prop variant <= 3,000; explorer <= 40,000. These are per shared asset and exist to support forest-scale/memory targets; acceptance performance, not triangle count alone, decides readiness.

### Textures

- Shipping textures use KTX2 with mipmaps and a cross-platform Basis Universal payload Bevy/wgpu can transcode on Metal, Vulkan, and DirectX targets.
- Terrain albedo/normal/ORM arrays have identical dimensions, layer ordering, and mip count. Initial layers are 1024 x 1024. Air/water registry slots contain neutral 1 x semantic layers in the array so material ID remains direct; water uses its own material texture.
- Albedo uses sRGB sampling; normal and ORM use linear sampling. ORM channels are R=ambient occlusion, G=roughness, B=metallic.
- Alpha-cut vegetation uses mip-aware cutout coverage. Horizon cards include color/normal/opacity and must avoid a species silhouette mismatch at transition distance.
- Source art files may live outside `assets/` or under a clearly ignored `art-source/`; the shipped asset tree contains only runtime-ready outputs.

### Shaders

WGSL files are Bevy material extensions with no backend-specific include. Terrain consumes position/normal plus four material IDs/weights and shared arrays; water consumes static surface geometry and presentation time for normal motion; vegetation consumes per-instance transform/variation; raw voxel consumes material ID. Shader time never affects world truth.

Shader validation compiles each pipeline in a headed smoke scene on Metal and Vulkan acceptance machines; DirectX-class compatibility is maintained through wgpu validation and CI shader parsing until a DirectX acceptance machine is available. No shader uses 64-bit atomics.

## Ruin stamp

`stamps/ruin_p1.ron` is the only hand-authored world shape allowed by the design. Its schema is:

```rust
pub struct SparseVoxelStamp {
    pub key: String,
    pub size_voxels: [u16; 3],
    pub pivot_voxel: [i16; 3],
    pub palette: Vec<MaterialId>,
    pub runs: Vec<StampRun>,
    pub tags: BTreeMap<String, VoxelCoord>,
}

pub struct StampRun {
    pub start_linear: u32,
    pub len: u16,
    pub palette_index: u8,
    pub density: u8,
}
```

Linear order matches brick voxel order generalized to stamp dimensions. Runs are sorted, non-overlapping, in bounds, and use only `cut_stone` or explicit air-carve palette values. Tags include `stair_bottom`, `stair_top`, and `entrance`. The stamp is sparse and small enough to validate/expand at placement; it has one intact staircase whose tread rise/run passes the configured capsule/step traversal test. Orientation is restricted to quarter turns around Y so voxel sampling is exact. The stamp's SHA-256 content digest participates in world identity and save compatibility.

The ruin's visible terrain mesh is extracted from stamped/current voxel truth on initial activation and on every affected world revision. There is no ruin GLB, separate ruin collision mesh, authored intact visual, or privileged staircase entity. Exact edit reversion produces a new voxel-derived mesh revision rather than restoring an authored root. All dirty ruin chunks and seams participate in the originating request's reconciliation accounting through render-extraction acknowledgement.

## Curated manifest

`curated_manifest.ron` is always generated by `moria-curate generate`, starts with a generated-file comment, and is never manually edited. It includes generated feature bounds, static water definitions, object placements, one ruin placement, tagged traversal/benchmark waypoints, and the seed/config digest. Coordinates are Q8 integers; collections use canonical sorted order.

`moria-curate check` regenerates and byte-compares canonical serialization, then runs:

- region/bounds and feature intersection checks;
- solid-collision-based route traversal with the configured capsule;
- meadow/forest eligible area; density-derived tree/bush/prop counts; 55/45 species minima; 5 m pairwise tree spacing; per-species lower/upper canopy-range bins; and 3 m registered-object clearance through the forest route, all on the same placement set;
- exact non-ruin raw solid-shape disjointness and non-ruin/ruin-stamp authored-coordinate disjointness, reporting stable IDs and the first conflicting voxel;
- both immutable index-table caps, complete retained-byte accounting, and deterministic worst-case radius-3 m forest/hillside edit candidate/affected-ID selection;
- carved river/lake bed checks;
- cliff gradient and visible tilted-strata checks;
- cave mouth/floor elevation, clearance, and connectivity checks;
- aquifer/iron cave-crossing checks; and
- ruin support/stair connectivity checks.

CI failure instructs the implementer to rerun generation only after an intentional authoritative config/stamp change. A new manifest changes `parameters_digest` and is incompatible with the one-version save, so the checked-in demo slot must be cleared for development; no migration is invented.

The additional `moria-curate prove-forest` command runs all checks above against the byte-identical checked-in manifest through the production runtime validator and adds M4 timing/memory evidence. It fails unless object validation is <=1,000 ms, index construction is <=250 ms, retained index memory including both grids is <=16 MiB, and dependency-coordinate allocation is zero. Passing density with undersized canopies, route obstruction, overlap, excessive index memory, or object-open overrun is not a partial success. The digest-matched headed F2 run separately proves that this same full manifest remains inside the <5,000 ms process-to-ready contract. These are blocking gates as specified in [implementation-plan.md](implementation-plan.md).

## Shared render asset resources

```rust
#[derive(Resource)]
struct WorldRenderAssets {
    terrain_material: Handle<TerrainMaterial>,
    water_material: Handle<WaterMaterial>,
    raw_voxel_mesh: Handle<Mesh>,
    object_lods: BTreeMap<ObjectVisualKey, ObjectLods>,
    dressing: DressingAssets,
    fallback_mesh: Handle<Mesh>,
    fallback_material: Handle<StandardMaterial>,
}
```

This resource is globally unique. Every repeated entity clones handles from it. Per-object transforms/IDs/LOD state are components. Loading code deduplicates texture/mesh handles by path and ledger ID; no spawn system calls `materials.add` or `meshes.add` for each instance.

Terrain meshes are dynamic per chunk and therefore unique `Mesh` handles, but every chunk shares one terrain material. Water patch meshes are dynamic and share one water material. Raw voxel instances share one cube mesh/material. Horizon base-card textures/geometry are immutable shared assets; the cell instance buffer is revision-filtered membership, and an edited tree's owner-filtered coarse mesh is a dynamic derived handle rather than a cached base card. Horizon cell eviction removes those instance/derived buffers after render extraction releases them. All dynamic handles are removed on eviction and the graphics ledger is decremented.

## Placeholder strategy

The implementation can start before final art is available, but placeholders preserve the same paths, bounds, LOD count, shared-handle rules, and material APIs:

- tree placeholders are procedural/trunk-and-canopy `.glb` files with clearly different birch/pine silhouettes;
- bush/prop placeholders are low-poly `.glb` primitives sized to final bounds;
- missing texture layers use keyed checker/flat-normal/roughness layers, not per-instance materials;
- explorer fallback is a capsule-proportioned mesh with static pose; movement/collision remain valid;
- missing horizon cards disable only Horizon tree presentation and emit an asset readiness warning; required acceptance art cannot ship with this fallback;
- shader load failure uses a shared magenta standard material in development and is fatal in benchmark/release acceptance.

Placeholders are acceptable for logic, streaming, mutation, and performance scaffolding, but the visual acceptance checklist in [rendering.md](rendering.md) must pass with final assets. A fallback use is included in benchmark report warnings so a run with materially incomplete presentation cannot pass silently.

Terrain, cave, water-body geometry, dressing anchors, and ruin placement never have hand-authored placeholder world meshes: their placeholder is the same procedural pipeline with simpler shared visual textures if necessary.

## Production asset registries

The two files under `assets/manifests/` are canonical RON documents with
`#![enable(implicit_some)]` disabled and `serde(deny_unknown_fields)` on every
record. They cover the 28 declared content assets exactly once: every stable ID
in the shared 30-entry declaration table except the two registry files
themselves. Entries are sorted by `stable_id`; paths must equal the immutable
declaration-table path; SHA-256 values are lowercase 64-character hexadecimal
digests of the exact installed bytes. Duplicate, missing, unknown, out-of-order,
path-mismatched, or digest-mismatched entries are fatal in every build profile.

```rust
pub struct AssetLicenseRegistry {
    pub schema_version: u16, // exactly 1
    pub entries: Vec<AssetLicenseEntry>,
}

pub struct AssetLicenseEntry {
    pub stable_id: String,
    pub path: String,
    pub content_sha256: String,
    pub provenance: AssetProvenance,
}

pub enum AssetProvenance {
    InHouseGenerated {
        generator_or_tool: String,
        author: String,
        source_path: Option<String>,
        modifications: Vec<String>,
    },
    External {
        source_url: String,
        author: String,
        license_spdx: String,
        license_text_path: String,
        modifications: Vec<String>,
    },
}
```

All provenance strings are non-empty after trimming. `source_url` is an
absolute `https` URL. `license_spdx` is one SPDX expression, and
`license_text_path` is a repository-relative existing file. External entries
may have an empty modifications list only when bytes are redistributed
unchanged. In-house entries may omit `source_path` only for deterministic
procedural output whose generator/tool is named. No `Unknown`, `TBD`, empty
license, or development-placeholder provenance is valid for release acceptance.

```rust
pub struct AssetBudgetRegistry {
    pub schema_version: u16, // exactly 1
    pub entries: Vec<AssetBudgetEntry>,
}

pub struct AssetBudgetEntry {
    pub stable_id: String,
    pub path: String,
    pub content_sha256: String,
    pub max_file_bytes: u64,
    pub contract: AssetBudgetContract,
}

pub enum AssetBudgetContract {
    Ron { schema_key: String },
    Glb {
        max_triangles_per_primitive: u32,
        required_named_primitives: Vec<String>,
        required_animation_clips: Vec<String>,
        bounds_min_q8: [i32; 3],
        bounds_max_q8: [i32; 3],
        support_origin_q8: [i32; 3],
    },
    Ktx2 {
        width: u32,
        height: u32,
        layers: u16,
        mip_count: u8,
        color_space: TextureColorSpace,
        basis_payload: bool,
    },
    Wgsl {
        entry_points: Vec<String>,
        forbids_i64_atomics: bool,
    },
}

pub enum TextureColorSpace { Srgb, Linear }
```

`max_file_bytes` is positive and enforced before decoding. Contract variants
must match the declared file extension. Bounds are finite Q8 metres with
`min < max` on every axis; the support origin lies on or inside the bounds.
Required-name vectors are sorted and duplicate-free. GLB triangle limits are
the per-asset limits in this document; the explorer records all six required
clips. KTX2 entries record the actual complete mip chain, layer count, colour
space, and require a Basis Universal payload. WGSL entries name every required
entry point and set `forbids_i64_atomics = true`. RON `schema_key` values name
the concrete loader schema rather than a generic `ron` bucket.

The license and budget entries for a stable ID must contain the same path and
content digest. The loader computes SHA-256 for both registry files themselves
and exposes `{ license_registry_sha256, budget_registry_sha256 }` in the
immutable `AssetValidationReport`; this avoids a self-referential registry
digest. Every content wire-in test records those two report digests and the
content digest it validated. Changing either registry or installed content
therefore invalidates stale wire-in evidence without creating a checksum cycle.

## Import and validation pipeline

At startup/asset test time, `AssetValidationPlugin` checks:

- all declared paths resolve and licenses are present;
- both production registries satisfy the exact schemas, cardinality, canonical
  ordering, cross-registry path/digest equality, and registry-report digest
  rules above;
- material registry order, texture array layer count/dimensions/formats/mips agree;
- mesh bounds, finite attributes, index range, LOD sequence, origins, and triangle budgets;
- vegetation materials use shared declared keys;
- player clips have required names;
- ruin stamp schema/materials/runs/tags/digest are valid; and
- shader asset definitions load without unsupported bindings.

Pure asset-schema tests run under `cargo test` without opening a window. GPU pipeline compilation and visual correctness run in the headed smoke/benchmark path.

`asset_licenses.ron` records source, author, license identifier/text path, and modifications for every external visual asset. Generated in-house assets are marked accordingly. An unlicensed/missing-entry asset fails release validation.

## Milestone outputs

The first terrain image, tunnel-carving clip, geology cutaway, dressed-world shot, playable-run capture, and benchmark results are produced from tagged manifest cameras/routes. They are written outside `assets/` (for example `target/captures/`) and are not loaded by the application or saved as world truth. Capture automation can set camera/time/debug toggles through the same consumer-facing presentation controls and issue mutation through `WorldEditWrite`; it cannot load a special terrain scene.
