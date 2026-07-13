# Configuration

All product parameters are typed, serialized, hashable in a stable field order, and split by ownership. `GenerationConfig` and its seed are immutable after boot because changing either would invalidate save deltas. Demo and render settings may change at runtime where stated. Defaults below are the Product One acceptance preset; benchmark reports include the exact `acceptance_config_fingerprint` defined below.

## Coordinate and storage constants

| Constant | Type | Default | Contract |
|---|---:|---:|---|
| `VOXEL_SIZE_M` | `f32` public / fixed-point internally | `0.25` | Four voxels per metre. |
| `BRICK_EDGE_VOXELS` | `u32` | `16` | 4 m brick edge; never runtime-variable. |
| `REGION_XZ_M` | `u32` | `1000` | X and Z half-open extent. |
| `REGION_MIN_Y_M` | `i32` | `-128` | Bottom, inclusive. |
| `REGION_MAX_Y_M` | `i32` | `128` | Top, exclusive. |
| `DENSITY_EMPTY` | `u8` | `0` | Fully empty. |
| `DENSITY_ISO` | `u8` | `128` | Solid occupancy and surface threshold for solid materials. |
| `DENSITY_FULL` | `u8` | `255` | Fully filled. |
| `GENERATION_FIXED_SCALE` | `i32` | `4096` | Fixed-point units per metre in base generation. |

Voxel bounds are X/Z `0..4000` and Y `-512..512`. Brick bounds are X/Z `0..250` and Y `-32..32`. Conversions use Euclidean floor division, including negative Y. Public floating positions are range-checked before conversion; out-of-range queries return `QueryError::OutOfBounds`.

## `GenerationConfig`

```rust
pub struct GenerationConfig {
    pub seed: WorldSeed,
    pub region: RegionBounds,
    pub terrain: TerrainParams,
    pub geology: GeologyParams,
    pub hydrology: HydrologyParams,
    pub biomes: BiomeParams,
    pub feature_constraints: FeatureConstraints,
    pub object_placement: ObjectPlacementParams,
    pub vegetation_templates: VegetationTemplateRef,
    pub ruin_stamp: RuinStampRef,
}
```

The nested fields below are exhaustive. Distances/elevations are metres unless suffixed otherwise; angles are degrees. Inclusive validity ranges are decoder/boot validation contracts.

| Type.field | Type | Product One default | Valid range/meaning |
|---|---:|---:|---|
| `GenerationConfig.seed` | `WorldSeed(u64)` | `0x4D4F_5249_4150_3031` | Fixed authored seed; all `u64` valid, but comparable Product One runs require this value. |
| `RegionBounds.min_m` | `[i32; 3]` | `[0, -128, 0]` | Fixed for Product One. |
| `RegionBounds.extent_m` | `[u32; 3]` | `[1000, 256, 1000]` | Fixed for Product One and divisible by voxel/brick scale. |
| `TerrainParams.base_surface_m` | `i32` | `64` | `48..80` |
| `TerrainParams.continent_frequency_per_m` | `FixedI32` | `3/1000` | `1/2000..1/100` |
| `TerrainParams.continent_amplitude_m` | `i32` | `22` | `0..48` |
| `TerrainParams.hill_frequency_per_m` | `FixedI32` | `1/96` | `1/512..1/16` |
| `TerrainParams.hill_amplitude_m` | `i32` | `11` | `0..32` |
| `TerrainParams.detail_frequency_per_m` | `FixedI32` | `1/24` | `1/128..1/4` |
| `TerrainParams.detail_amplitude_m` | `i32` | `2` | `0..8` |
| `TerrainParams.topsoil_depth_m` | `FixedI32` | `3/2` | `1/2..4` |
| `TerrainParams.subsoil_depth_m` | `FixedI32` | `4` | `1..12` |
| `TerrainParams.cliff_slope_threshold` | `FixedI32` | `7/10` | `0..1` normalized gradient |
| `TerrainParams.noise_salt` | `u64` | `0x5445_5252_4149_4E01` | Stable hash domain. |
| `GeologyParams.strata_period_m` | `FixedI32` | `18` | `4..64` |
| `GeologyParams.strata_tilt_x_deg` | `i16` | `17` | `-45..45` |
| `GeologyParams.strata_tilt_z_deg` | `i16` | `-11` | `-45..45` |
| `GeologyParams.strata_warp_amplitude_m` | `FixedI32` | `3/2` | `0..6` |
| `GeologyParams.cave_radius_m` | `FixedI32` | `3` | `2..8` |
| `GeologyParams.cave_warp_m` | `FixedI32` | `3/2` | `0..4` |
| `GeologyParams.aquifer_y_m` | `RangeInclusive<i32>` | `20..=28` | Inside region Y. |
| `GeologyParams.ore_radius_m` | `FixedI32` | `3/2` | `1/2..4` |
| `GeologyParams.noise_salt` | `u64` | `0x4745_4F4C_4F47_5901` | Stable hash domain. |
| `HydrologyParams.river_half_width_m` | `FixedI32` | `6` | `2..20` |
| `HydrologyParams.river_depth_m` | `FixedI32` | `3` | `1..8` |
| `HydrologyParams.river_surface_drop_per_m` | `FixedI32` | `1/500` | `0..1/50` |
| `HydrologyParams.lake_radius_m` | `FixedI32` | `52` | `20..120` |
| `HydrologyParams.lake_depth_m` | `FixedI32` | `6` | `2..20` |
| `HydrologyParams.lake_surface_y_m` | `FixedI32` | `59` | `40..80` |
| `HydrologyParams.bank_blend_m` | `FixedI32` | `4` | `1..12` |
| `BiomeParams.meadow_grass_per_m2` | `FixedI32` | `3` | `0..12` |
| `BiomeParams.forest_tree_species_a_per_ha` | `u16` | `420` | `1..1000` |
| `BiomeParams.forest_tree_species_b_per_ha` | `u16` | `280` | `1..1000` |
| `BiomeParams.forest_bushes_per_ha` | `u16` | `360` | `0..1200` |
| `BiomeParams.forest_threshold` | `FixedI32` | `11/20` | `0..1` biome field |
| `BiomeParams.noise_frequency_per_m` | `FixedI32` | `1/180` | `1/1000..1/16` |
| `BiomeParams.noise_salt` | `u64` | `0x4249_4F4D_4553_0001` | Stable hash domain. |
| `ObjectPlacementParams.cell_edge_m` | `u16` | `16` | `4..64`, power of two |
| `ObjectPlacementParams.tree_min_spacing_m` | `FixedI32` | `5/2` | `1..12` |
| `ObjectPlacementParams.bush_min_spacing_m` | `FixedI32` | `1` | `1/2..6` |
| `ObjectPlacementParams.rock_density_per_ha` | `u16` | `90` | `0..1000` |
| `ObjectPlacementParams.stump_density_per_ha` | `u16` | `24` | `0..500` |
| `ObjectPlacementParams.max_objects_per_mutation_dependency` | `u8` | `24` | `1..24`; boot validator checks every eligible 3 m mutation volume. |
| `ObjectPlacementParams.salt` | `u64` | `0x4F42_4A45_4354_0001` | Stable hash domain. |
| `FeatureConstraints.cave_local_depth_m` | `RangeInclusive<u16>` | `38..=42` | Required route depth. |
| `FeatureConstraints.min_outcrop_tilt_deg` | `u8` | `12` | `1..45` |
| `FeatureConstraints.route_clearance_radius_m` | `FixedI32` | `2/5` | `0.35..1` |
| `FeatureConstraints.min_forest_route_length_m` | `u16` | `120` | `50..400` |
| `FeatureConstraints.required_ruins` | `u8` | `1` | Must equal 1. |
| `VegetationTemplateRef.logical_path` | `AssetPath` | `vegetation/vegetation_templates.ron` | Relative root-asset path only. |
| `VegetationTemplateRef.expected_hash` | `[u8; 32]` | checked-in BLAKE3 digest literal in `product_one.ron` | Must match the exact loaded template bytes before base sampling. |
| `RuinStampRef.logical_path` | `AssetPath` | `stamps/ruin.ron` | Relative root-asset path only. |
| `RuinStampRef.expected_hash` | `[u8; 32]` | checked-in BLAKE3 digest literal in `product_one.ron` | Must match loaded stamp bytes. |

`FixedI32` is a serialized numerator/denominator pair reduced on load and converted to `GENERATION_FIXED_SCALE`; zero denominators and out-of-range conversions fail boot. Rock stratum material order is the fixed registry sequence `[limestone, sandstone, shale, granite]`; the two tree species are stable IDs `pine = 1` and `birch = 2`. The checked-in `ProductOne` preset is the only supported runtime generation preset. Its file contains exactly these values and both base-truth asset hashes, not unspecified tuning fields. Values can be changed during implementation tuning only by committing a new preset and expected manifest/sample fingerprint; the committed preset used by acceptance is concrete truth, not a runtime auto-tune.

## Fingerprint registry and compatibility

Every fingerprint is BLAKE3-256 over its ASCII domain tag followed by the canonical encoding described here. Unsigned/signed integers and IEEE-754 float bit patterns are little-endian; booleans are one byte; strings and byte sequences are `u64`-length-prefixed; enums use their documented stable `u8` discriminant; arrays/structs are encoded in the listed field order without field names; rationals encode reduced signed numerator then positive denominator. Paths are normalized relative UTF-8 asset paths. RON whitespace and map ordering never enter the stream. A referenced base-truth asset encodes both normalized logical path and the BLAKE3-256 digest of the exact loaded bytes; its parsed values are not substituted for that byte digest.

`BASE_SAMPLER_REVISION: u32 = 1` is a compiled compatibility constant, not a user setting or a field read from `product_one.ron`. It identifies the complete `sample_base` algorithm and input-schema semantics, including fixed-point/noise evaluation, operation precedence, manifest and object-placement interpretation, vegetation-template and ruin-stamp interpretation, and `BaseSample` provenance assignment. Any code or schema change that can alter `sample_base`'s value or provenance at any coordinate for otherwise identical seed, config, material registry, and asset bytes must increment this constant in the same change. Refactors proven byte-for-byte and provenance-for-provenance equivalent by the checked-in conformance corpus do not increment it.

| Digest name | Domain tag | Canonical ordered membership |
|---|---|---|
| `generation_fingerprint` | `MORIA-P1-GENERATION\0` | `BASE_SAMPLER_REVISION` first, then coordinate/storage constants in table order, followed exactly by: seed; region min/extent; all terrain fields; all geology fields; all hydrology fields; all biome fields; all feature-constraint fields; all object-placement fields; vegetation-template logical path/hash; ruin-stamp logical path/hash. Within each named group, use its table order. The two expected hashes must first equal the exact loaded bytes. Thus the base-sampler semantics and every asset capable of changing `sample_base` are part of this digest. |
| `material_fingerprint` | `MORIA-P1-MATERIALS\0` | The ordered 14-entry material registry; per entry encode `id`, canonical `name`, `hardness`, `granular`, `phase`, `debug_placeable`, and `surface_class` in that order, followed by `state_must_be_zero = true`. `render` is excluded because it cannot change voxel/query truth; visual settings are represented by `render_fingerprint` and run-level placeholder/assets metadata. |
| `query_config_fingerprint` | `MORIA-P1-QUERY\0` | Every `QueryConfig` field in table order. |
| `streaming_config_fingerprint` | `MORIA-P1-STREAMING\0` | Every `StreamingConfig` field in table order. |
| `mutation_config_fingerprint` | `MORIA-P1-MUTATION\0` | Every `MutationConfig` field in table order, followed by the fixed place-material cycle as stable material IDs. |
| `player_config_fingerprint` | `MORIA-P1-PLAYER\0` | Every `PlayerConfig` field in table order. Physical bindings are excluded because benchmark scripts submit semantic actions and bindings do not change the measured world behavior. |
| `camera_light_config_fingerprint` | `MORIA-P1-CAMERA-LIGHT\0` | Every `CameraLightConfig` field in table order. |
| `render_fingerprint` | `MORIA-P1-RENDER\0` | Every `WorldRenderConfig` field in table order. |
| `persistence_fingerprint` | `MORIA-P1-PERSISTENCE\0` | Every `PersistenceConfig` field in table order, followed by save magic, envelope field layout, brick codec threshold/tags, absolute-value entry layout, CRC variant, Zstandard flags, and locked Zstandard crate version from `persistence.md`. The resolved platform directory and test-injected root are excluded; the configured path components remain included. |
| `acceptance_limits_fingerprint` | `MORIA-P1-ACCEPTANCE-LIMITS\0` | Every acceptance constant in table order, encoding strict/inclusive comparator discriminant before its numeric threshold. |

The aggregate named everywhere as `acceptance_config_fingerprint` is exactly:

```text
BLAKE3-256(
  "MORIA-P1-ACCEPTANCE-CONFIG\0" ||
  generation_fingerprint || material_fingerprint ||
  query_config_fingerprint || streaming_config_fingerprint ||
  mutation_config_fingerprint || player_config_fingerprint ||
  camera_light_config_fingerprint || render_fingerprint ||
  persistence_fingerprint || acceptance_limits_fingerprint
)
```

This order is normative. Benchmark scenario/schema ID, resolution, present mode, placeholder-assets flag, build profile, and machine profile remain separate comparison keys because they are run metadata rather than configuration. Golden literals for every component digest and the aggregate are checked in beside `product_one.ron` after authored asset bytes exist. Boot uses the compiled `BASE_SAMPLER_REVISION`, hashes the exact vegetation-template and ruin bytes, rejects either asset/hash mismatch, recomputes all component digests and the aggregate, and rejects any mismatch before building the manifest or reaching `ControlReady`.

Compatibility uses are intentionally distinct and use these exact names: the save envelope stores and load validates `generation_fingerprint`, `material_fingerprint`, and `persistence_fingerprint` before sampling a base coordinate or applying deltas; distributable bundle metadata records those three plus `acceptance_config_fingerprint`; cache keys and worker results carry `generation_fingerprint`; benchmark reports record all component digest names above plus the aggregate, and comparison requires aggregate equality. All of these consumers therefore inherit `BASE_SAMPLER_REVISION` through the one normative generation digest rather than carrying a second revision field. Render/query/player tuning invalidates benchmark comparability but not an otherwise byte-compatible world save, while a base-sampler revision, vegetation-template, ruin, material-truth, or codec change rejects that save.

A checked-in base-sampler conformance corpus records its revision header plus representative and boundary `BaseSample` value/provenance outputs for the Product One preset. CI requires the corpus header to equal the compiled `BASE_SAMPLER_REVISION` and recomputes every output; an output change without an intentional revision/header/golden-digest update fails. A separate compatibility test invokes the canonical fingerprint encoder with otherwise identical inputs and revision values `1` and `2`, asserts unequal `generation_fingerprint` and aggregate values, and exercises the pre-sampling save rejection specified in `persistence.md`.

`FeatureConstraints` requires exactly one connected route manifest containing, in order or by named branches: meadow spawn; dense mixed forest; river/lake viewpoints; canopy-level cliff; jumpable shelf sequence; intact ruin stairs; cave mouth; aquifer crossing; ore crossing; cave floor with `local_depth_m` in `[38, 42]`. Generation fails boot with a diagnostic if the preset does not satisfy every constraint; it must never silently launch a deficient seed.

## `QueryConfig`

| Field | Type | Default | Contract |
|---|---:|---:|---|
| `max_bulk_scan_cells` | `u32` | `262_144` | `1..=262_144`; checked enclosing voxel count before AABB/sphere traversal. |
| `max_bulk_results` | `u32` | `65_536` | `1..=65_536` and `<= max_bulk_scan_cells`; 65,537th match fails without a partial result. |
| `max_raycast_distance_m` | `f32` | `1024.0` | finite `0.25..=1024`; longer rays reject `QueryTooLarge`. |

These are hard public service limits, not hints. They bound local collision/camera/inspection calls while rejecting all-region eager enumeration in constant preflight work.

## `StreamingConfig`

Distances are horizontal from the player/camera focus, with vertical/frustum checks applied after band selection.

| Field | Type | Default | Representation |
|---|---:|---:|---|
| `interaction_radius_m` | `f32` | `24` | Full-resolution sampled/dense truth required for mutation and immediate traversal. |
| `near_radius_m` | `f32` | `96` | Full 25 cm visible surface, nearby cave portals, full object/dressing density. |
| `mid_radius_m` | `f32` | `256` | 50 cm derived terrain LOD, shared object LOD, reduced dressing. |
| `far_radius_m` | `f32` | `720` | 1 m surface-only terrain through every possible corner, tree impostor/low LOD, no underground volume except visible connected portals. |
| `retain_hysteresis_m` | `f32` | `24` | Prevents load/unload churn at each boundary. |
| `prefetch_seconds` | `f32` | `1.0` | Extends selection along player velocity and camera look direction. |
| `max_normal_commits_per_frame` | `u16` | `12` | Caps non-mutation mesh uploads; mutation uses its separately bounded reserved packets/range table below. |
| `normal_apply_budget_ms` | `f32` | `2.0` | Main-thread normal streaming budget. |
| `mutation_apply_budget_ms` | `f32` | `4.0` | Main-thread carve apply ceiling. |
| `mutation_truth_budget_ms` | `f32` | `4.0` | Acceptance-frame sampling/delta/dirty-set ceiling; no apply runs in that frame. |
| `normal_reserved_logical_cores` | `u8` | `3` | Normal worker count is `max(1, logical_cores - 3)`, leaving main plus two mutation workers unsaturated. |
| `max_normal_jobs_per_worker` | `u8` | `2` | Bounds queued plus running normal work; excess plans remain undispatched. |
| `normal_job_sample_cells` | `u16` | `5_832` | One 16³ brick plus one-cell halo; larger work is split. |
| `normal_cancel_check_cells` | `u16` | `256` | Running normal work checks the mutation epoch at least this often and abandons stale results. |
| `mutation_worker_threads` | `u8` | `2` | Exclusive project task pool; never runs normal streaming jobs. Acceptance mode requires at least four logical cores. |
| `mutation_prepare_deadline_ms` | `f32` | `12.0` | Wall time from immutable snapshot availability to both prepared packets on acceptance hardware. |
| `mutation_max_changed_bricks` | `u8` | `27` | Exact maximum for a quantized radius-3 m sphere at 25 cm cells. |
| `mutation_max_dependency_bricks` | `u8` | `125` | Changed-brick box plus one-brick extraction halo. |
| `mutation_commit_packet_bytes` | `u32` | `50_331_648` | Maximum prepared upload bytes admitted in each of two frames (48 MiB). |
| `mutation_commit_packets` | `u8` | `2` | Exactly two preallocated packets; no allocation growth on commit. |
| `mutation_max_range_swaps` | `u16` | `224` | 125 composite-surface partitions + 24 objects + 27 water + 27 raw + 21 reserved descriptors, batched across two packets. |
| `mutation_max_water_bytes` | `u32` | `4_000_000` | Local refreshed static-water vertex/index payload cap. |
| `mutation_max_descriptor_bytes` | `u32` | `1_000_000` | Range tables, alignment, and commit metadata cap. |

Uniform bricks and procedural column summaries may be represented in any band without dense voxel arrays. Edited bricks are never discarded from the delta store; only their expanded caches and render artifacts unload. Defaults are benchmark-tunable, but any changed acceptance preset must be fingerprinted in the report.

Validation requires finite nonnegative distances; `4 <= interaction <= 48`, `32 <= near <= 160`, `128 <= mid <= 384`, `512 <= far <= 720`, strictly increasing band radii, `0 <= hysteresis <= 64`, `0 <= prefetch <= 3`, normal commits `1..64`, budgets `0.25..4 ms`, reserved cores `3`, jobs/worker `1..2`, sample/cancel values equal the table defaults, mutation workers `2`, prepare deadline `1..12 ms`, and all mutation geometry/packet/range limits equal the table defaults in Product One. Cross-field packet capacity must cover the compiled extractor upper bound before boot succeeds.

## `MutationConfig`

| Field | Type | Default | Contract |
|---|---:|---:|---|
| `default_radius_m` | `f32` | `3.0` | Representative debug sphere radius. |
| `min_radius_m` / `max_radius_m` | `f32` | `0.25 / 3.0` | Radius keyboard adjustment range. |
| `center_quantum_voxels` | `u8` | `16` | Target centres quantize to 1/16 voxel for deterministic replay. |
| `default_strength` | `u8` | `255` | Full centre removal/addition with antialiased boundary falloff. |
| `target_range_m` | `f32` | `12.0` | Camera-centre voxel raycast range. |
| `max_in_flight_mutations` | `u8` | `1` | Interactive tool accepts another operation after prior surface commit; carve storm submits on its scripted cadence. |

Dig/place uses integer squared distance to the quantized centre. For radius `r`, inner voxels receive full strength and the outer one-voxel shell receives a deterministic linear coverage value. Dig is phase-specific: if `old.material.phase == Solid`, it computes `new_density = old_density.saturating_sub(coverage * strength / 255)` and canonicalizes density 0 to air/state 0; if phase is `Fluid` or `Empty`, it returns the exact old three bytes. Place rejects a non-solid selected material, returns fluid samples byte-identically, and otherwise computes `new_density = max(old_density, coverage * strength / 255)`; only where the operation raises density does it assign the selected solid material and state 0. A delta is removed when the result equals the base composite sample. `changed_voxels` counts only byte-different results, so mixed land/water/air spheres neither count nor dirty unchanged water/air samples.

The place-material cycle is `topsoil, subsoil, sand, gravel, limestone, sandstone, shale, granite, iron_ore, wood, leaf, cut_stone`. Air is dig-only and generated water is immutable through this Product One operation.

Validation requires finite radii/range, `0.25 <= min_radius <= default_radius <= max_radius <= 3.0`, centre denominator exactly `16`, target range `0.25..24 m`, and in-flight count exactly `1`. Strength accepts `1..=255`; zero-strength commands reject `InvalidInput` rather than producing a misleading accepted operation.

## `PlayerConfig`

| Field | Type | Default |
|---|---:|---:|
| `fixed_timestep` | `Duration` | `1/60 s` |
| `capsule_radius_m` | `f32` | `0.35` |
| `capsule_half_height_m` | `f32` | `0.55` |
| `run_speed_mps` | `f32` | `4.5` |
| `sprint_speed_mps` | `f32` | `7.5` |
| `ground_acceleration_mps2` | `f32` | `30` |
| `air_acceleration_mps2` | `f32` | `8` |
| `gravity_mps2` | `f32` | `24` |
| `jump_speed_mps` | `f32` | `8` |
| `max_step_height_m` | `f32` | `0.50` |
| `max_walkable_slope_deg` | `f32` | `50` |
| `paddle_speed_mps` | `f32` | `3.0` |
| `water_surface_offset_m` | `f32` | `0.55` below capsule centre |

These are validation-character parameters, not stats or progression. The curated shelf and stair validators use this exact collision volume, step height, jump speed, and gravity; a route is invalid if the headless traversal fixture cannot clear it.

`PlayerConfig` has exactly the fields in the table. Values must be finite; timestep is fixed at 60 Hz, radius `0.2..0.6`, half-height `0.4..1.0`, speeds/accelerations/gravity/jump are positive and at most `50`, step `0..0.5`, slope `0..60°`, and water offset must keep the capsule intersecting the surface. Product One comparable runs require the listed defaults.

Controls map centrally to semantic actions:

| Physical input | Action |
|---|---|
| W/A/S/D | planar `Move` axis relative to camera |
| Left Shift | held `Sprint` |
| Space | edge-buffered `Jump` |
| mouse delta while captured | `OrbitCamera` |
| Escape | `ReleaseCursor` / `PauseDiagnostics` (simulation is not paused) |
| G | `Dig` |
| P | `Place` |
| `[` / `]` | previous/next place material |
| `-` / `=` | decrease/increase sphere radius |
| B | `ToggleBrickBounds` |
| V | `ToggleRawVoxels` |
| N | `ToggleStreamingBands` |
| T | `ToggleTimeSlider` |
| Left/Right when time slider active | decrease/increase fixed solar time |
| F5 | `SaveSlot` |
| F9 | `LoadSlot` |

Debug action edges remain buffered until one fixed/application consumer acknowledges them, so a rendered frame with zero fixed ticks cannot lose an operation.

## Camera and lighting

| Field | Type | Default |
|---|---:|---:|
| `orbit_distance_m` | `f32` | `5.0` |
| `min_collision_distance_m` | `f32` | `0.6` |
| `pitch_limits_deg` | `(f32, f32)` | `(-70, 65)` |
| `collision_radius_m` | `f32` | `0.20` |
| `recovery_half_life_s` | `f32` | `0.08` |
| `player_light_range_m` | `f32` | `18` |
| `player_light_intensity_lm` | `f32` | `2200` |
| `underground_sky_cover_m` | `f32` | `2.0` |
| `fixed_solar_time_h` | `f32` | `14.0` |
| `time_slider_range_h` | `(f32, f32)` | `(6.0, 20.0)` |

The player light enables when the base/current surface above the player occludes the sky for at least 2 m or the named cave route volume contains the player. It is presentation-only. The time slider changes sun direction and ambient intensity but never advances automatically and never changes generation or world truth.

`CameraLightConfig` has exactly the fields in the table. All floats must be finite; orbit is `1..12 m`, minimum distance `0.2..orbit`, pitch lies within `-89..89°`, collision radius `0.05..0.5 m`, half-life `0.01..0.5 s`, light range `1..30 m`, intensity `0..10,000 lm`, sky cover `0..16 m`, solar time/range within `0..24 h`, and the default solar time must lie inside the slider range.

## `WorldRenderConfig`

| Field | Type | Default | Validity/contract |
|---|---:|---:|---|
| `near_plane_m` | `f32` | `0.05` | `0.01..0.20` |
| `vertical_fov_deg` | `f32` | `65.0` | `45..90` |
| `terrain_crease_angle_deg` | `f32` | `38.0` | `0..90`; preserves rock/material creases at or above this gradient angle. |
| `dressing_min_normal_y` | `f32` | `0.82` | `0..1`; eligible upward surface threshold. |
| `lod_transition_overlap_m` | `f32` | `8.0` | `4..16` |
| `sun_illuminance_lux_at_noon` | `f32` | `100_000.0` | `1_000..150_000` |
| `ambient_brightness` | `f32` | `80.0` | `0..1000`, Bevy environment-map light units. |
| `distance_fog_start_m` | `f32` | `480.0` | `near_radius..far_radius` |
| `distance_fog_end_m` | `f32` | `720.0` | `fog_start..far_radius`; never hides missing in-band geometry. |
| `water_normal_scroll_m_per_s` | `[f32; 2]` | `[0.018, 0.011]` | each `0..0.1`; visual UV motion only. |
| `water_roughness` | `f32` | `0.18` | `0..1` |
| `shadow_map_size` | `u32` | `4096` | `1024 | 2048 | 4096`; fingerprinted target setting. |

## `PersistenceConfig`

| Field | Type | Default | Validity/contract |
|---|---:|---:|---|
| `application_qualifier` | `String` | `org` | Fixed platform-path qualifier. |
| `application_organization` | `String` | `moria` | Non-empty path component. |
| `application_name` | `String` | `product-one` | Non-empty path component. |
| `slot_file_name` | `String` | `product-one.delta.zst` | Single plain filename, no separators. |
| `zstd_level` | `i32` | `9` | `1..=19`; fixed for canonical acceptance size. |
| `max_uncompressed_bytes` | `u64` | `196_608_000_000` | Finite-region worst-case defensive ceiling; decoder never preallocates it and also enforces record/count limits. |
| `max_changed_bricks` | `u32` | `4_000_000` | Exact region brick count `250 * 64 * 250`. |
| `max_changed_voxels` | `u64` | `16_384_000_000` | Exact region voxel count. |
| `fsync_parent_directory` | `bool` | `true` | Required where the platform API supports directory sync. |

Tests/benchmarks may replace only the resolved root/path with an isolated temporary directory; codec fields remain acceptance defaults. The save magic, little-endian field layout, CRC32, and absolute-value codec are fixed in `persistence.md` and included in the persistence fingerprint.

## Acceptance constants

| Target | Contract |
|---|---|
| control ready | `< 5.0 s` from app start for clean seed and acceptance save on target machines |
| frame rate | `>= 60.0` mean FPS at the required target resolution; report 1% low additionally |
| mutation presentation | every affected currently visible artifact committed within `<= 2` presented frames |
| hitch ceiling | no frame `> 25.0 ms` during representative carve; main-thread mutation work `<= 4 ms/frame` |
| active graphics bytes | tracked allocation `< 2,000,000,000` everywhere; discrete Linux resident high-water also `< 2,000,000,000` |
| per-scenario delta save | `< 50,000,000` bytes for both the flythrough one-carve slot and the defined carve-storm slot |

The Linux 3060-class result is marked `provisional` in reports until measured on the named test machine and explicitly re-baselined. M4 results identify unified memory rather than claiming dedicated VRAM.
