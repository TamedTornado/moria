//! Validation for configuration loaded from RON.

use std::{collections::BTreeSet, error::Error, fmt};

use crate::config::{
    BandConfig, CollisionClass, InputAction, InputConfig, MaterialRegistry, PresentationConfig,
    RangeQ8, RegionConfig, SurfaceClass,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigValidationError {
    pub field: &'static str,
    pub reason: &'static str,
}

impl fmt::Display for ConfigValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid {}: {}", self.field, self.reason)
    }
}
impl Error for ConfigValidationError {}

type Result<T = ()> = std::result::Result<T, ConfigValidationError>;
fn invalid(field: &'static str, reason: &'static str) -> ConfigValidationError {
    ConfigValidationError { field, reason }
}
fn require(value: bool, field: &'static str, reason: &'static str) -> Result {
    value.then_some(()).ok_or_else(|| invalid(field, reason))
}
fn positive(value: u64, field: &'static str) -> Result {
    require(value > 0, field, "must be positive")
}
fn finite_positive(value: f32, field: &'static str) -> Result {
    require(
        value.is_finite() && value > 0.0,
        field,
        "must be finite and positive",
    )
}
fn ascending(range: [f32; 2], field: &'static str) -> Result {
    require(
        range[0].is_finite() && range[1].is_finite() && range[0] < range[1],
        field,
        "must be finite and ascending",
    )
}
fn range_q8(range: &RangeQ8, field: &'static str) -> Result {
    require(
        range.min_q8 > 0 && range.min_q8 <= range.max_q8,
        field,
        "must be positive and ascending",
    )
}

pub fn validate_region_config(config: &RegionConfig) -> Result {
    let b = &config.bounds;
    require(
        b.x_min_m < b.x_max_m && b.x_max_m - b.x_min_m == 1_000,
        "bounds.x",
        "must be an exact 1 km exclusive interval",
    )?;
    require(
        b.z_min_m < b.z_max_m && b.z_max_m - b.z_min_m == 1_000,
        "bounds.z",
        "must be an exact 1 km exclusive interval",
    )?;
    require(
        b.y_min_m < b.y_max_m && b.y_max_m - b.y_min_m == 256,
        "bounds.y",
        "must be an exact 256 m exclusive interval",
    )?;
    require(
        b.voxel_edge_q8 == 64,
        "bounds.voxel_edge_q8",
        "must be 64 for 0.25 m voxels",
    )?;
    require(
        b.brick_edge_voxels == 16,
        "bounds.brick_edge_voxels",
        "must be 16",
    )?;
    let t = &config.terrain;
    positive(u64::from(t.broad_scale_m), "terrain.broad_scale_m")?;
    positive(u64::from(t.meander_scale_m), "terrain.meander_scale_m")?;
    require(
        t.broad_scale_m > t.meander_scale_m,
        "terrain",
        "broad scale must exceed meander scale",
    )?;
    positive(u64::from(t.relief_m), "terrain.relief_m")?;
    require(
        t.topsoil_depth_q8 > 0 && t.topsoil_depth_q8 < t.subsoil_depth_q8,
        "terrain soil depths",
        "must be positive and ordered",
    )?;
    require(
        i32::from(b.y_max_m) - i32::from(t.typical_surface_m) >= 60,
        "terrain.typical_surface_m",
        "must leave at least 60 m of sky",
    )?;
    require(
        i32::from(t.typical_surface_m) - i32::from(b.y_min_m) >= 188,
        "terrain.typical_surface_m",
        "must leave at least 188 m of geology",
    )?;
    let g = &config.geology;
    require(
        g.stratum_thickness_m.iter().all(|&v| v > 0),
        "geology.stratum_thickness_m",
        "all strata must be positive",
    )?;
    require(
        g.tilt_degrees.unsigned_abs() <= 90,
        "geology.tilt_degrees",
        "must be within -90..=90",
    )?;
    positive(
        u64::from(g.aquifer_thickness_m),
        "geology.aquifer_thickness_m",
    )?;
    require(
        g.aquifer_material.0 == 5,
        "geology.aquifer_material",
        "must be gravel",
    )?;
    positive(
        u64::from(g.iron_vein_radius_q8),
        "geology.iron_vein_radius_q8",
    )?;
    let c = &config.cave;
    require(
        c.floor_elevation_m < c.entrance_elevation_m,
        "cave elevations",
        "floor must be below entrance",
    )?;
    require(
        c.entrance_tolerance_m > 0 && c.floor_tolerance_m > 0,
        "cave tolerances",
        "must be positive",
    )?;
    require(
        c.min_clear_width_q8 >= 768 && c.min_clear_height_q8 >= 768,
        "cave clearance",
        "must be at least 3 m",
    )?;
    require(
        (1..=90).contains(&c.max_route_slope_degrees),
        "cave.max_route_slope_degrees",
        "must be in 1..=90",
    )?;
    let w = &config.water;
    positive(u64::from(w.river_width_m), "water.river_width_m")?;
    positive(u64::from(w.river_depth_q8), "water.river_depth_q8")?;
    positive(
        u64::from(w.lake_min_diameter_m),
        "water.lake_min_diameter_m",
    )?;
    positive(u64::from(w.lake_depth_m), "water.lake_depth_m")?;
    let biome = &config.biome;
    positive(
        u64::from(biome.meadow_min_area_m2),
        "biome.meadow_min_area_m2",
    )?;
    positive(
        u64::from(biome.forest_min_area_m2),
        "biome.forest_min_area_m2",
    )?;
    positive(
        u64::from(biome.forest_tree_spacing_m),
        "biome.forest_tree_spacing_m",
    )?;
    require(
        u16::from(biome.tree_species_mix_percent[0]) + u16::from(biome.tree_species_mix_percent[1])
            == 100,
        "biome.tree_species_mix_percent",
        "must sum to 100",
    )?;
    positive(
        u64::from(biome.bushes_per_hectare),
        "biome.bushes_per_hectare",
    )?;
    let o = &config.objects;
    require(
        o.max_anchor_slope_degrees <= 90,
        "objects.max_anchor_slope_degrees",
        "must be at most 90",
    )?;
    positive(u64::from(o.route_clearance_m), "objects.route_clearance_m")?;
    require(
        o.index_cell_size_m == 32 && o.sample_index_cell_size_m == 4,
        "objects index cell sizes",
        "must be 32 m and 4 m",
    )?;
    require(
        o.max_index_cells_per_object > 0
            && o.max_sample_cells_per_object > 0
            && o.max_dependency_bricks_per_object > 0,
        "objects index caps",
        "must be positive",
    )?;
    require(
        o.max_index_entries_per_cell > 0
            && o.max_sample_entries_per_cell > 0
            && o.max_edit_dependency_candidates > 0
            && o.max_affected_objects_per_edit > 0,
        "objects query caps",
        "must be positive",
    )?;
    positive(
        u64::from(o.max_retained_index_bytes),
        "objects.max_retained_index_bytes",
    )?;
    for (range, field) in [
        (&o.birch_trunk_radius_q8, "objects.birch_trunk_radius_q8"),
        (&o.birch_trunk_height_q8, "objects.birch_trunk_height_q8"),
        (&o.pine_trunk_radius_q8, "objects.pine_trunk_radius_q8"),
        (&o.pine_trunk_height_q8, "objects.pine_trunk_height_q8"),
        (&o.canopy_radius_q8, "objects.canopy_radius_q8"),
        (&o.bush_radius_q8, "objects.bush_radius_q8"),
        (&o.boulder_radius_q8, "objects.boulder_radius_q8"),
        (&o.stump_radius_q8, "objects.stump_radius_q8"),
        (&o.stump_height_q8, "objects.stump_height_q8"),
        (&o.rock_radius_q8, "objects.rock_radius_q8"),
    ] {
        range_q8(range, field)?;
    }
    require(
        o.canopy_radius_q8.min_q8 >= 512 && o.canopy_radius_q8.max_q8 <= 1024,
        "objects.canopy_radius_q8",
        "must remain within 2..=4 m",
    )?;
    require(
        !config.ruin_stamp.is_empty(),
        "ruin_stamp",
        "must not be empty",
    )
}

pub fn validate_material_registry(registry: &MaterialRegistry) -> Result {
    const KEYS: [&str; 14] = [
        "air",
        "water",
        "topsoil",
        "subsoil",
        "sand",
        "gravel",
        "limestone",
        "sandstone",
        "shale",
        "granite",
        "iron_ore",
        "wood",
        "leaf",
        "cut_stone",
    ];
    require(
        registry.materials.len() == KEYS.len(),
        "materials",
        "must contain the 14 Product One materials",
    )?;
    for (expected_id, (material, key)) in registry.materials.iter().zip(KEYS).enumerate() {
        require(
            material.id.0 == expected_id as u8 && material.key == key,
            "materials",
            "IDs and keys must use Product One canonical order",
        )?;
        let expected_collision = match expected_id {
            0 => CollisionClass::Empty,
            1 => CollisionClass::Fluid,
            _ => CollisionClass::Solid,
        };
        require(
            material.collision_class == expected_collision,
            "materials.collision_class",
            "cannot override Product One collision classes",
        )?;
        require(
            (expected_id < 2 && material.hardness == 0)
                || (expected_id >= 2 && material.hardness > 0),
            "materials.hardness",
            "air/water must be zero and solids must be 1..=255",
        )?;
    }
    require(
        registry.materials[4].granular && registry.materials[5].granular,
        "materials.granular",
        "sand and gravel must be granular",
    )?;
    require(
        registry
            .materials
            .iter()
            .enumerate()
            .filter(|(id, _)| *id != 4 && *id != 5)
            .all(|(_, material)| !material.granular),
        "materials.granular",
        "only sand and gravel are granular",
    )?;
    let expected_surfaces = [
        SurfaceClass::Empty,
        SurfaceClass::Water,
        SurfaceClass::Organic,
        SurfaceClass::Organic,
        SurfaceClass::Granular,
        SurfaceClass::Granular,
        SurfaceClass::Rock,
        SurfaceClass::Rock,
        SurfaceClass::Rock,
        SurfaceClass::Rock,
        SurfaceClass::Ore,
        SurfaceClass::OrganicObject,
        SurfaceClass::OrganicObject,
        SurfaceClass::Masonry,
    ];
    require(
        registry
            .materials
            .iter()
            .zip(expected_surfaces)
            .all(|(m, surface)| m.surface_class == surface),
        "materials.surface_class",
        "must match Product One material classes",
    )
}

pub fn validate_presentation_config(config: &PresentationConfig) -> Result {
    let s = &config.streaming;
    for (index, band) in s.bands.iter().enumerate() {
        validate_band(
            band,
            index,
            if index == 0 {
                0
            } else {
                s.bands[index - 1].end_m
            },
        )?;
    }
    require(
        s.bands[3].end_m == 720,
        "streaming.bands",
        "horizon must end at 720 m",
    )?;
    require(
        s.hysteresis_m > 0 && u16::from(s.hysteresis_m) < s.bands[0].end_m,
        "streaming.hysteresis_m",
        "must fit inside Near band",
    )?;
    require(
        s.collision_radius_m > 0 && u16::from(s.collision_radius_m) <= s.bands[0].end_m,
        "streaming.collision_radius_m",
        "must fit inside Near band",
    )?;
    positive(
        u64::from(s.vertical_surface_window_m),
        "streaming.vertical_surface_window_m",
    )?;
    positive(
        u64::from(s.prefetch_seconds_q8),
        "streaming.prefetch_seconds_q8",
    )?;
    positive(
        u64::from(s.max_generation_jobs),
        "streaming.max_generation_jobs",
    )?;
    positive(u64::from(s.max_mesh_jobs), "streaming.max_mesh_jobs")?;
    positive(
        u64::from(s.max_install_bytes_per_frame),
        "streaming.max_install_bytes_per_frame",
    )?;
    require(
        s.edit_reserved_workers > 0 && s.edit_reserved_workers <= s.max_mesh_jobs as u8,
        "streaming.edit_reserved_workers",
        "must reserve a valid edit lane",
    )?;
    let m = &config.mutation;
    require(
        m.fixed_hz > 0
            && m.min_radius_q8 > 0
            && m.min_radius_q8 <= m.debug_radius_q8
            && m.debug_radius_q8 <= m.max_radius_q8,
        "mutation radii",
        "must be positive and ordered",
    )?;
    require(
        m.inner_full_strength_percent <= 100,
        "mutation.inner_full_strength_percent",
        "must be at most 100",
    )?;
    require(
        m.max_atomic_bricks > 0
            && m.max_commit_bricks_per_batch > 0
            && m.max_commit_bricks_per_batch <= m.max_atomic_bricks,
        "mutation atomic caps",
        "must be positive and ordered",
    )?;
    require(
        m.max_progressive_bricks >= u32::from(m.max_atomic_bricks),
        "mutation.max_progressive_bricks",
        "must cover atomic operations",
    )?;
    require(
        m.max_queued_edits > 0 && m.max_mutation_stage_ms_per_frame_q8 > 0,
        "mutation queue and stage caps",
        "must be positive",
    )?;
    validate_player(config)?;
    validate_rendering(config)?;
    validate_benchmark(config)
}

fn validate_band(band: &BandConfig, index: usize, expected_start: u16) -> Result {
    require(
        band.start_m == expected_start && band.end_m > band.start_m,
        "streaming.bands",
        "must be contiguous ascending ranges",
    )?;
    let expected_edge = [64, 128, 256, 1024][index];
    require(
        band.voxel_edge_q8 == expected_edge,
        "streaming.bands.voxel_edge_q8",
        "must use Product One LOD resolution",
    )
}
fn validate_player(config: &PresentationConfig) -> Result {
    let p = &config.player;
    finite_positive(p.body.radius_m, "player.body.radius_m")?;
    finite_positive(p.body.cylinder_height_m, "player.body.cylinder_height_m")?;
    for (value, field) in [
        (p.run_speed_mps, "player.run_speed_mps"),
        (p.sprint_speed_mps, "player.sprint_speed_mps"),
        (p.ground_accel_mps2, "player.ground_accel_mps2"),
        (p.air_accel_mps2, "player.air_accel_mps2"),
        (p.gravity_mps2, "player.gravity_mps2"),
        (p.jump_speed_mps, "player.jump_speed_mps"),
        (p.step_height_m, "player.step_height_m"),
        (p.paddle_speed_mps, "player.paddle_speed_mps"),
        (p.paddle_surface_offset_m, "player.paddle_surface_offset_m"),
    ] {
        finite_positive(value, field)?;
    }
    require(
        p.sprint_speed_mps > p.run_speed_mps,
        "player speeds",
        "sprint speed must exceed run speed",
    )?;
    require(
        p.max_walk_slope_deg.is_finite()
            && p.max_walk_slope_deg > 0.0
            && p.max_walk_slope_deg < 90.0,
        "player.max_walk_slope_deg",
        "must be within 0..90",
    )?;
    let c = &config.camera;
    finite_positive(c.distance_m, "camera.distance_m")?;
    ascending(c.distance_limits_m, "camera.distance_limits_m")?;
    require(
        c.distance_m >= c.distance_limits_m[0] && c.distance_m <= c.distance_limits_m[1],
        "camera.distance_m",
        "must fit distance limits",
    )?;
    ascending(c.pitch_limits_deg, "camera.pitch_limits_deg")?;
    require(
        c.pitch_limits_deg[0] >= -90.0 && c.pitch_limits_deg[1] <= 90.0,
        "camera.pitch_limits_deg",
        "must be within -90..=90",
    )?;
    finite_positive(c.probe_radius_m, "camera.probe_radius_m")?;
    finite_positive(c.collision_margin_m, "camera.collision_margin_m")?;
    finite_positive(config.light.depth_enable_m, "light.depth_enable_m")?;
    finite_positive(config.light.range_m, "light.range_m")?;
    finite_positive(config.light.intensity_lm, "light.intensity_lm")
}
fn validate_rendering(config: &PresentationConfig) -> Result {
    let r = &config.rendering;
    positive(u64::from(r.window.width), "rendering.window.width")?;
    positive(u64::from(r.window.height), "rendering.window.height")?;
    require(
        matches!(r.msaa_samples, 1 | 2 | 4 | 8),
        "rendering.msaa_samples",
        "must be a supported sample count",
    )?;
    positive(u64::from(r.shadow_map_size), "rendering.shadow_map_size")?;
    ascending(r.time_limits_hours, "rendering.time_limits_hours")?;
    require(
        r.time_of_day_hours >= r.time_limits_hours[0]
            && r.time_of_day_hours <= r.time_limits_hours[1],
        "rendering.time_of_day_hours",
        "must fit slider range",
    )?;
    finite_positive(
        r.time_keyboard_step_hours,
        "rendering.time_keyboard_step_hours",
    )?;
    require(
        r.grass_normal_min_y.is_finite() && (0.0..=1.0).contains(&r.grass_normal_min_y),
        "rendering.grass_normal_min_y",
        "must be in 0..=1",
    )?;
    finite_positive(
        r.grass_near_density_per_m2,
        "rendering.grass_near_density_per_m2",
    )?;
    require(
        r.grass_middle_density_scale.is_finite()
            && (0.0..=1.0).contains(&r.grass_middle_density_scale),
        "rendering.grass_middle_density_scale",
        "must be in 0..=1",
    )?;
    require(
        r.object_visibility_m <= r.cluster_visibility_m && r.cluster_visibility_m == 720,
        "rendering visibility",
        "must cover ordered Product One bands",
    )?;
    require(
        r.horizon_object_cell_size_m == 64,
        "rendering.horizon_object_cell_size_m",
        "must be 64 m",
    )?;
    finite_positive(r.horizon_derived_lod_m, "rendering.horizon_derived_lod_m")?;
    positive(
        u64::from(r.max_horizon_tree_members_per_cell),
        "rendering.max_horizon_tree_members_per_cell",
    )
}
fn validate_benchmark(config: &PresentationConfig) -> Result {
    let b = &config.benchmark;
    require(
        b.warmup_frames > 0
            && b.flythrough_duration_s > 0
            && b.colony_worker_streams > 0
            && b.colony_volume_m.iter().all(|&v| v > 0),
        "benchmark workload",
        "must be positive",
    )?;
    require(
        b.catastrophic_radius_q8 >= config.mutation.min_radius_q8
            && b.catastrophic_radius_q8 <= config.mutation.max_radius_q8,
        "benchmark.catastrophic_radius_q8",
        "must fit mutation radius bounds",
    )?;
    for (value, field) in [
        (b.fps_target, "benchmark.fps_target"),
        (b.max_mutation_frame_ms, "benchmark.max_mutation_frame_ms"),
        (b.max_admission_ms, "benchmark.max_admission_ms"),
        (
            b.carve_object_dependency_max_ms,
            "benchmark.carve_object_dependency_max_ms",
        ),
        (
            b.query_frame_critical_p99_ms,
            "benchmark.query_frame_critical_p99_ms",
        ),
        (
            b.query_normal_bundle_p99_ms,
            "benchmark.query_normal_bundle_p99_ms",
        ),
        (
            b.query_frame_critical_max_ms,
            "benchmark.query_frame_critical_max_ms",
        ),
        (
            b.query_cells_page_p99_ms,
            "benchmark.query_cells_page_p99_ms",
        ),
        (
            b.query_cells_page_max_ms,
            "benchmark.query_cells_page_max_ms",
        ),
    ] {
        finite_positive(value, field)?;
    }
    require(
        b.max_primary_ready_p95_ms <= b.max_primary_ready_ms
            && b.query_frame_critical_p99_ms <= b.query_frame_critical_max_ms
            && b.query_cells_page_p99_ms <= b.query_cells_page_max_ms,
        "benchmark percentile caps",
        "percentile targets must not exceed maxima",
    )?;
    require(
        b.max_first_commit_interactive_ms > 0
            && b.max_first_commit_colony_ms > 0
            && b.min_changed_bricks_per_second > 0
            && b.max_runnable_wait_ms > 0
            && b.max_reconciliation_interactive_ms > 0
            && b.max_reconciliation_volume_ms > 0
            && b.cold_start_max_ms > 0
            && b.graphics_memory_max_bytes > 0
            && b.save_max_bytes > 0
            && b.forest_object_validation_max_ms > 0
            && b.forest_object_index_build_max_ms > 0,
        "benchmark caps",
        "must be positive",
    )
}

pub fn validate_input_config(config: &InputConfig) -> Result {
    require(
        config.stick_dead_zone.is_finite() && (0.0..1.0).contains(&config.stick_dead_zone),
        "input.stick_dead_zone",
        "must be in 0..1",
    )?;
    finite_positive(
        config.mouse_sensitivity_degrees,
        "input.mouse_sensitivity_degrees",
    )?;
    finite_positive(
        config.gamepad_orbit_degrees_per_second,
        "input.gamepad_orbit_degrees_per_second",
    )?;
    let actions: BTreeSet<_> = config
        .bindings
        .iter()
        .map(|binding| binding.action)
        .collect();
    require(
        actions.len() == config.bindings.len(),
        "input.bindings",
        "actions must be unique",
    )?;
    require(
        actions.len() == 17,
        "input.bindings",
        "must bind every Product One action",
    )?;
    require(
        config
            .bindings
            .iter()
            .all(|binding| !binding.keyboard_mouse.is_empty() || !binding.gamepad.is_empty()),
        "input.bindings",
        "each action must have a physical binding",
    )?;
    let required = [
        InputAction::Move,
        InputAction::Sprint,
        InputAction::Jump,
        InputAction::Orbit,
        InputAction::Zoom,
        InputAction::Dig,
        InputAction::Place,
        InputAction::PreviousMaterial,
        InputAction::NextMaterial,
        InputAction::BrickBounds,
        InputAction::RawVoxels,
        InputAction::StreamingBands,
        InputAction::TimeDown,
        InputAction::TimeUp,
        InputAction::TimeSliderFocus,
        InputAction::Save,
        InputAction::Load,
    ];
    require(
        required.into_iter().all(|action| actions.contains(&action)),
        "input.bindings",
        "must bind every Product One action",
    )
}
