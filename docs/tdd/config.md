# Configuration and Defaults

## Configuration ownership

Authoritative curated values live in `assets/config/product_one_region.ron`; material presentation values live in `assets/materials/materials.ron`; render/runtime tuning lives in `assets/config/presentation.ron`; and physical action bindings live in `assets/config/input.ron`. The generated `assets/config/curated_manifest.ron` records feature instances and route metadata derived from the seed/config. Values affecting generated truth participate in `parameters_digest`; presentation, input, player, camera, streaming budget, and benchmark window values do not.

RON files deserialize with `deny_unknown_fields`. Startup validates all ranges and cross-field invariants before world creation. Production binaries do not silently substitute defaults for a present but invalid file. Rust `Default` implementations exist for tests and for generating the initial files; the checked-in values below are normative.

## Authoritative region configuration

```rust
pub struct RegionConfig {
    pub seed: u64,
    pub bounds: BoundsConfig,
    pub terrain: TerrainGenConfig,
    pub geology: GeologyConfig,
    pub cave: CaveConfig,
    pub water: WaterGenConfig,
    pub biome: BiomeConfig,
    pub objects: ObjectGenConfig,
    pub ruin_stamp: String,
}
```

| Field | Type | Default | Contract/motivation |
|---|---:|---:|---|
| `seed` | `u64` | `0x4D4F_5249_415F_5031` | Stable authored Product One seed (`MORIA_P1`). |
| `bounds.x_min/max_m` | `i16` | `-500 / 500` | Exact 1 km region width. Max is exclusive. |
| `bounds.z_min/max_m` | `i16` | `-500 / 500` | Exact 1 km region depth. Max is exclusive. |
| `bounds.y_min/max_m` | `i16` | `-128 / 128` | Exact 256 m vertical extent. Max is exclusive. |
| `bounds.voxel_edge_q8` | `u16` | `64` | 0.25 m voxels. |
| `bounds.brick_edge_voxels` | `u8` | `16` | 16-cubed voxel bricks / 4 m cubes. |
| `terrain.typical_surface_m` | `i16` | `64` | Region-wide typical surface target. |
| `terrain.broad_scale_m` | `u16` | `220` | Produces kilometre-scale rolling elevation. |
| `terrain.meander_scale_m` | `u16` | `72` | Adds hills/meadow variation without cube-like terrain. |
| `terrain.relief_m` | `u8` | `34` | Supports rolling hills and cliff candidates within bounds. |
| `terrain.topsoil_depth_q8` | `u16` | `256` | 1.0 m topsoil band for grass anchors/cut earth. |
| `terrain.subsoil_depth_q8` | `u16` | `768` | 3.0 m subsoil below topsoil for credible cuts. |

The generation constants are fixed-point frequencies/amplitudes internally derived from these metre fields; the implementation must not add undocumented magic scale values. The curator checks that generated playable sky above the typical surface is at least 60 m and geology below it is at least 188 m, allowing voxel rounding around the design's “about 64/190 m” language.

### Geology and feature constraints

| Field | Type | Default | Contract |
|---|---:|---:|---|
| `geology.stratum_thickness_m` | `[u8; 4]` | `[8, 12, 10, 18]` | Alternating limestone, sandstone, shale, granite bands. |
| `geology.tilt_degrees` | `i8` | `18` | Visibly tilted exposed strata. Stored/evaluated fixed point. |
| `geology.aquifer_thickness_m` | `u8` | `6` | Finite aquifer band intersecting cave route. |
| `geology.aquifer_material` | `MaterialId` | `gravel` | A distinct water-bearing geological band without wetness simulation. |
| `geology.iron_vein_radius_q8` | `u16` | `320` | 1.25 m vein radius visible at cave crossing. |
| `cave.entrance_elevation_m` | `i16` | `0` | Curator target; tolerance below. |
| `cave.entrance_tolerance_m` | `u8` | `2` | Keeps entrance on generated surface without hand carving. |
| `cave.floor_elevation_m` | `i16` | `-40` | Required continuous-depth target. |
| `cave.floor_tolerance_m` | `u8` | `2` | Acceptance range for generated route. |
| `cave.min_clear_width_q8` | `u16` | `768` | 3 m minimum route width for player/camera traversal. |
| `cave.min_clear_height_q8` | `u16` | `768` | 3 m minimum route height. |
| `cave.max_route_slope_degrees` | `u8` | `35` | Keeps the route walkable with shelves/stairs. |
| `water.river_width_m` | `u8` | `10` | Visible channel and paddle area. |
| `water.river_depth_q8` | `u16` | `512` | 2 m carved channel. |
| `water.lake_min_diameter_m` | `u8` | `80` | Landscape-scale lake, not a puddle prop. |
| `water.lake_depth_m` | `u8` | `6` | Genuine carved basin. |

The listed values implement the required feature character; visual/geometric acceptance is enforced by curator queries rather than exact handcrafted coordinates. Candidate search is deterministic: enumerate keyed candidates in ascending index, accept the first set satisfying all constraints, and store its generated metadata. Runtime revalidation must find the same set.

### Biome, object, and route constraints

| Field | Type | Default | Contract |
|---|---:|---:|---|
| `biome.meadow_min_area_m2` | `u32` | `40_000` | Ensures a readable rolling meadow. |
| `biome.forest_min_area_m2` | `u32` | `120_000` | Ensures forest-scale density. |
| `biome.forest_tree_spacing_m` | `u8` | `5` | Dense while retaining a walkable route. |
| `biome.tree_species_mix_percent` | `[u8; 2]` | `[55, 45]` | Both required species occur at scale; sum must be 100. |
| `biome.bushes_per_hectare` | `u16` | `450` | Dense forest understory. |
| `objects.boulders_per_hectare` | `u16` | `24` | Required boulder scatter. |
| `objects.stumps_per_hectare` | `u16` | `14` | Required stumps. |
| `objects.rocks_per_hectare` | `u16` | `90` | Required scattered rocks. |
| `objects.max_anchor_slope_degrees` | `u8` | `32` | Prevents unsupported registered objects. |
| `objects.route_clearance_m` | `u8` | `3` | Keeps route traversable through dense forest. |
| `objects.index_cell_size_m` | `u8` | `32` | Immutable runtime placement-index cell edge. |
| `objects.max_index_cells_per_object` | `u8` | `16` | Bound for the raw-shape plus surface-dependency index AABB. |
| `objects.max_index_entries_per_cell` | `u16` | `1024` | Bound for deterministic activation and edit-dependency queries. |
| `objects.sample_index_cell_size_m` | `u8` | `4` | Fine horizontal cell used by synchronous base-voxel queries; matches a brick width. |
| `objects.max_sample_cells_per_object` | `u8` | `16` | Bound for raw-shape insertion into the fine sample index. |
| `objects.max_sample_entries_per_cell` | `u8` | `64` | Hard per-voxel object-candidate bound used by public query complexity. |
| `objects.max_edit_dependency_candidates` | `u16` | `256` | Maximum broad-phase placements for any legal radius-3 m edit center in the curated forest/hillside proof domain. |
| `objects.max_affected_objects_per_edit` | `u8` | `64` | Maximum exact dependency hits/readiness-barrier object roots for one supported edit. |
| `objects.max_dependency_bricks_per_object` | `u16` | `128` | Bound for sparse delta probes by one lazy dependency footprint. |
| `objects.max_retained_index_bytes` | `u32` | `16_777_216` | 16 MiB cap including compact records, grid storage/capacity, keys, and allocator padding. |
| `ruin_stamp` | UTF-8 path | `stamps/ruin_p1.ron` | One sparse cut-stone stamp with staircase. |

`ObjectGenConfig` also contains Q8 voxel-shape ranges: birch trunk radius 0.20–0.35 m and height 8–14 m, pine trunk radius 0.25–0.45 m and height 10–18 m, canopy radii 2–4 m, bush radii 0.5–1.2 m, boulder radii 0.5–1.8 m, stump radius 0.25–0.55 m/height 0.25–0.75 m, and rock radii 0.15–0.6 m. Stable placement hashes choose within ranges; all values quantize to Q8. These analytic shapes make the non-ruin objects material and solid-collision truth while shared GLB assets provide their intact high-detail presentation. The ruin instead uses the configured sparse stamp for truth and always-voxel-derived presentation.

Tree/bush/object counts are generated from eligible biome area, not spawned as disconnected decorations. Let `forest_area_m2` and `eligible_land_area_m2` be the exact raster-cell counts reported by curation. The manifest requires `tree_count >= ceil(forest_area_m2 / 25)`, `bush_count >= ceil(forest_area_m2 * 450 / 10_000)`, and each prop count at least `ceil(eligible_land_area_m2 * configured_per_hectare / 10_000)`. Birch and pine counts are each at least the floor of the minimum tree count times their configured 55/45 percentage. Tree anchor X/Z distance is at least 5 m for every pair; this spacing does not waive voxel-shape disjointness.

Every tree canopy radius must be in 2–4 m. Each species must include at least 16 lower-range placements with maximum horizontal canopy radius in `[2.0, 2.5]` m and at least 16 upper-range placements in `[3.5, 4.0]` m, ensuring the checked forest proves the stated range rather than selecting only the easiest small canopy. The forest route-clearance volume is the configured player capsule swept along the piecewise-linear forest route with its horizontal radius expanded to exactly 3 m; no registered-object solid voxel may intersect that volume anywhere on the tagged forest segment. The route must still traverse the qualifying forest area rather than skirting its boundary.

Placement generation walks a seed-keyed candidate order and rejects a candidate whose raw fixed-point solid shape shares any voxel with an already accepted non-ruin shape or intersects any authored coordinate of the transformed ruin stamp; it continues until all area, count, species, spacing, canopy-bin, route-clearance, index, and edit-candidate contracts pass together or rejects the seed as incuratable. This preserves the required dense forest without allowing an intact shared GLB to cover voxels attributed to another placement. The manifest validator independently repeats the exact disjointness check. It computes only fixed-size raw/dependency bounds per placement; dependency coordinates are never expanded or retained. Deterministic radius-3 m center enumeration over tagged forest/hillside surface cells must also prove the 256 broad-candidate and 64 exact-hit edit caps and record the maximum-candidate target for the carve feasibility gate. The validator also requires exactly one ruin, at least one cliff/outcrop tagged with visible strata, one cave route, one aquifer crossing, one ore crossing, one lake, and one river, plus placements of both tree species, bushes, boulders, stumps, and rocks inside the flythrough's visible corridor.

## Material configuration

Each `MaterialDef` validates a stable `u8 id`, key, hardness `1..=255` for solid matter (0 for air/water), granular flag, collision class, surface class, and texture layers. Air is `CollisionClass::Empty`, water is `CollisionClass::Fluid`, and IDs 2–13 are `CollisionClass::Solid`; no authored config may override those Product One classes.

| ID | Key | Hardness | Granular | Surface class |
|---:|---|---:|:---:|---|
| 0 | `air` | 0 | no | Empty |
| 1 | `water` | 0 | no | Water |
| 2 | `topsoil` | 28 | no | Organic |
| 3 | `subsoil` | 42 | no | Organic |
| 4 | `sand` | 18 | yes | Granular |
| 5 | `gravel` | 35 | yes | Granular |
| 6 | `limestone` | 82 | no | Rock |
| 7 | `sandstone` | 68 | no | Rock |
| 8 | `shale` | 60 | no | Rock |
| 9 | `granite` | 120 | no | Rock |
| 10 | `iron_ore` | 135 | no | Ore |
| 11 | `wood` | 55 | no | OrganicObject |
| 12 | `leaf` | 8 | no | OrganicObject |
| 13 | `cut_stone` | 100 | no | Masonry |

Hardness is a relative falloff/partial-strength debug erosion divisor with reference value 64, not a mining/stat system; a full-strength kernel core still removes any material in one operation. Surface class selects extraction normal policy and material shading only. State defaults to zero for every material. `placeable` is derived as solid matter excluding wood/leaf registered-object internals; the Product One place cycle is topsoil, subsoil, sand, gravel, limestone, sandstone, shale, granite, iron ore, and cut stone.

## Streaming and task configuration

```rust
pub struct StreamingConfig {
    pub bands: [BandConfig; 4],
    pub hysteresis_m: u8,
    pub collision_radius_m: u8,
    pub vertical_surface_window_m: u8,
    pub prefetch_seconds_q8: u16,
    pub max_generation_jobs: u16,
    pub max_mesh_jobs: u16,
    pub max_install_bytes_per_frame: u32,
    pub edit_reserved_workers: u8,
}
```

| Band | Radial range | Representation | Object policy |
|---|---|---|---|
| `Near` | 0–64 m | 0.25 m surface/cave mesh; detailed only at boundaries/collision/edits | Full trees/bushes/props/dressing |
| `Middle` | 64–160 m | 0.5 m sampled surface mesh; cave interiors omitted unless focus is underground | Full registered objects, reduced dressing |
| `Far` | 160–320 m | 1 m surface mesh | Tree/object LOD, no small dressing |
| `Horizon` | 320–720 m | 4 m column-derived terrain/water mesh | Revisioned 64 m tree cells: intact cards plus edited per-ID 4 m payloads |

The 720 m horizon covers the farthest corner from any point in a 1 km square. LOD distances use camera/player focus and frustum visibility; physics/mutation focus always requests authoritative 0.25 m samples independent of visual LOD. Persisted edits are resampled into coarse distant meshes; cave voids are intentionally not rendered at far distance because they are occluded by the surface.

Defaults: `hysteresis_m = 12`, `collision_radius_m = 12`, `vertical_surface_window_m = 12`, `prefetch_seconds_q8 = 512` (2 s), `max_generation_jobs = 96`, `max_mesh_jobs = 64`, `max_install_bytes_per_frame = 16_777_216` (16 MiB), and `edit_reserved_workers = 2`. At runtime worker counts are capped by available Bevy task-pool threads; at least one worker lane is reserved for edit mesh work. If fewer than three worker threads exist, edit jobs preempt ordinary work rather than reserving idle lanes.

Budgets are starting acceptance values, not excuses to miss the two-frame contract. The benchmark may justify tuning checked-in presentation values without changing authoritative generation/digest. Memory categories have hard pre-upload guards: one allocation and one GPU-visible count/index must fit `u32`; total host sums use checked `u64`.

## Mutation configuration

| Field | Type | Default | Contract |
|---|---:|---:|---|
| `fixed_hz` | `u16` | `60` | Deterministic simulation/edit cadence. |
| `debug_radius_q8` | `u16` | `768` | 3 m signature dig/place sphere. |
| `min_radius_q8` | `u16` | `64` | One voxel minimum. |
| `max_radius_q8` | `u16` | `768` | Product One debug/API capacity with two-frame scheduling. |
| `dig_strength` | `u8` | `255` | Full-strength proof carve, hardness-scaled with falloff. |
| `place_strength` | `u8` | `255` | Full-strength proof placement. |
| `inner_full_strength_percent` | `u8` | `70` | Solid core plus smooth boundary. |
| `max_queued_edits` | `u8` | `2` | Prevents unbounded carve backlog; overflow is explicit `Busy`. |

The 3 m maximum is the Product One supported operation envelope tied to its stated representative carve and acceptance latency. It is not presented as a future substrate limit; supporting larger atomic edits would require a new latency/memory contract. Radius is still carried in every API request so smaller proof operations are supported.

## Player and camera configuration

| Field | Type | Default | Motivation/contract |
|---|---:|---:|---|
| `body.radius_m` | `f32` | `0.40` | Fits a walkable cave while visibly third person. |
| `body.cylinder_height_m` | `f32` | `1.00` | Total capsule height 1.8 m. |
| `run_speed_mps` | `f32` | `5.0` | Traversal run. |
| `sprint_speed_mps` | `f32` | `8.0` | Distinct sprint for 1 km demo route. |
| `ground_accel_mps2` | `f32` | `35.0` | Responsive validation movement. |
| `air_accel_mps2` | `f32` | `10.0` | Limited airborne steering. |
| `gravity_mps2` | `f32` | `22.0` | Stable 60 Hz voxel traversal. |
| `jump_speed_mps` | `f32` | `7.0` | Clears configured rock shelves after curator validation. |
| `step_height_m` | `f32` | `0.30` | Clears one 0.25 m voxel stair without climbing. |
| `max_walk_slope_deg` | `f32` | `48.0` | Supports rock shelves but rejects cliff faces. |
| `paddle_speed_mps` | `f32` | `3.0` | Surface-only water travel. |
| `paddle_surface_offset_m` | `f32` | `0.55` | Keeps upper body above static surface. |
| `camera.distance_m` | `f32` | `5.5` | Default third-person view. |
| `camera.min/max_distance_m` | `[f32; 2]` | `[2.0, 9.0]` | Zoom limits. |
| `camera.pitch_deg` | `[f32; 2]` | `[-65, 75]` | Free orbit without inversion. |
| `camera.probe_radius_m` | `f32` | `0.18` | Terrain avoidance including near plane. |
| `camera.collision_margin_m` | `f32` | `0.12` | Prevents wall clipping. |
| `light.depth_enable_m` | `f32` | `2.0` | Enables below local surface/low sky exposure. |
| `light.range_m` | `f32` | `18.0` | Makes cave route legible. |
| `light.intensity_lm` | `f32` | `1_600.0` | Simple player-attached cave light. |

Movement floats are ephemeral integration values; every solid-collision query quantizes to Q8 before authoritative sampling. Curator traversal validation uses this exact body and movement configuration and scripted intent, so the cliff shelves, ruin staircase, cave mouth, and cave floor must be continuously reachable without privileged motion.

## Input defaults

| Action | Keyboard/mouse | Gamepad |
|---|---|---|
| Move | W/A/S/D | Left stick |
| Sprint | Left Shift | Left stick press |
| Jump | Space | South face button |
| Orbit | Mouse motion while captured | Right stick |
| Zoom | Mouse wheel | Triggers difference |
| Dig | G or left mouse | Right shoulder |
| Place | P or right mouse | Left shoulder |
| Previous/next material | `[` / `]` | D-pad left/right |
| Brick bounds | F1 | — |
| Raw voxels | F2 | — |
| Streaming bands | F3 | — |
| Time down/up | `-` / `=` | D-pad down/up while debug modifier held |
| Time slider focus | Tab | — |
| Save/load | F5 / F9 | — |

Diagnostics remain keyboard-driven as required, while standard gamepad movement/camera actions exercise the action abstraction. Default stick dead zone is 0.15; mouse sensitivity is 0.12 degrees per input unit; gamepad orbit is 150 degrees/s at full deflection. Bindings are fixed for Product One; the RON mapping centralizes them but no rebinding UI is included.

## Rendering and environment configuration

| Field | Type | Default | Contract |
|---|---:|---:|---|
| `window.width/height` | `u32` | `2560 / 1440` | Discrete acceptance default. Mac harness also supplies 1080p. |
| `msaa_samples` | `u8` | `4` | Smooth conventional presentation; may be re-baselined only with evidence. |
| `shadow_map_size` | `u32` | `2048` | Portable directional shadows within memory target. |
| `time_of_day_hours` | `f32` | `14.0` | Fixed readable daylight default. |
| `time_min/max_hours` | `[f32; 2]` | `[6.0, 20.0]` | Slider inspection range. |
| `time_keyboard_step_hours` | `f32` | `0.25` | Exact per-edge keyboard/gamepad adjustment; clamped to slider range. |
| `grass_normal_min_y` | `f32` | `0.75` | Upward-facing topsoil anchors only. |
| `grass_near_density_per_m2` | `f32` | `5.0` | Meadow/forest floor dressing. |
| `grass_middle_density_scale` | `f32` | `0.25` | Distance reduction. |
| `object_visibility_m` | `u16` | `320` | Full objects through far band. |
| `cluster_visibility_m` | `u16` | `720` | Horizon tree clusters. |
| `horizon_object_cell_size_m` | `u8` | `64` | Aligned aggregate ownership cell; equals the Horizon terrain tile width. |
| `horizon_derived_lod_m` | `f32` | `4.0` | Current-truth owner-filtered LOD for dependency-edited Horizon trees. |
| `max_horizon_tree_members_per_cell` | `u16` | `1024` | Validation cap on sorted base-plus-derived IDs in one cell; overflow rejects the manifest and is never truncated. |

Terrain uses CPU dual contouring with material-aware feature constraints, shared texture arrays, triplanar projection, distance LOD, frustum culling, and transition skirts. The Horizon member cap bounds aggregate repartition/install work and GPU-visible instance counts; it does not cap edits or silently omit placements. Detailed policy is in [rendering.md](rendering.md). The renderer must use Bevy/wgpu portable shader features only and compile for Metal, Vulkan, and DirectX-class backends. No shader buffer uses a 64-bit atomic type.

## Benchmark configuration

| Field | Default | Meaning |
|---|---:|---|
| `warmup_frames` | 300 | Excluded from FPS/frame distribution. |
| `flythrough_duration_s` | 120 | Continuous route coverage. |
| `carve_count` | 128 | Repeated public 3 m edits and a meaningful heavy-delta save workload. |
| `post_edit_settle_frames` | 2 | Deadline, not an arbitrary wait; scenario fails if not ready. |
| `watchdog_s` | 300 | Fails a stuck run. |
| `fps_target` | 60.0 | Acceptance average and one-percent-low target. |
| `max_edit_ready_frames` | 2 | Mutation contract. |
| `max_carve_frame_ms` | 33.3 | No-visible-hitch threshold. |
| `cold_start_max_ms` | 5000 | Process to control ready. |
| `graphics_memory_max_bytes` | `2_097_152_000` | Product resident-graphics-memory target; the portable application ledger is a proxy only pending product approval or resident measurement. |
| `save_max_bytes` | `50_000_000` | Heavily defaced compressed slot. |
| `forest_object_validation_max_ms` | `1000` | M4 F1 budget for production object validation plus both index tables. |
| `forest_object_index_build_max_ms` | `250` | M4 F1 subset budget for index construction. |
| `carve_object_dependency_max_ms` | `1.0` | M4 F2 combined dirty-discovery and dependency-eligibility budget. |
| `query_frame_critical_p99_ms` | `1.0` | Per-call M4 bound for synchronous frame-critical public queries. |
| `query_normal_bundle_p99_ms` | `2.0` | M4 bound for one representative movement/camera/debug query bundle. |
| `query_frame_critical_max_ms` | `4.0` | No measured frame-critical public query may exceed this. |
| `query_cells_page_p99/max_ms` | `4.0 / 8.0` | M4 inspection-only maximum diagnostic page bounds. |

The first three timing fields are feasibility gates, not user-tunable quality settings: a failing report requires implementation/TDD review and cannot rewrite them. The carve storm alternates dig and place in a deterministic sequence but finishes with heavy visible defacement; reverting an edit exactly to base is omitted from the measured save-size set because it would correctly remove that delta. Exact paths and metric aggregation are in [benchmarks.md](benchmarks.md) and gate sequencing is in [implementation-plan.md](implementation-plan.md).

## Cargo profiles

The workspace root owns profiles:

```toml
[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3

[profile.release]
codegen-units = 1
lto = "thin"
```

No package overrides these without benchmark evidence. The dev loop remains `cargo run -p moria-demo`; only shipping/acceptance benchmarks require release.
