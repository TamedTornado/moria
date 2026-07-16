//! Semantic validation for RON configuration contracts.

use std::{collections::BTreeSet, error::Error, fmt};

use crate::config::{CollisionClass, ConfigSet, MaterialRegistryConfig, RegionConfig};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConfigValidationError {
    Invalid { field: &'static str },
}

impl fmt::Display for ConfigValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid { field } => write!(formatter, "invalid configuration field: {field}"),
        }
    }
}
impl Error for ConfigValidationError {}

fn invalid(field: &'static str) -> ConfigValidationError {
    ConfigValidationError::Invalid { field }
}
fn positive(value: f32) -> bool {
    value.is_finite() && value > 0.0
}
fn finite_unit(value: f32) -> bool {
    value.is_finite() && (0.0..=1.0).contains(&value)
}
fn range<T: PartialOrd>(values: [T; 2]) -> bool {
    values[0] <= values[1]
}

pub fn validate_region_config(config: &RegionConfig) -> Result<(), ConfigValidationError> {
    let bounds = &config.bounds;
    if !(bounds.x_min_m < bounds.x_max_m
        && bounds.z_min_m < bounds.z_max_m
        && bounds.y_min_m < bounds.y_max_m)
    {
        return Err(invalid("bounds"));
    }
    if bounds.x_min_m != -500
        || bounds.x_max_m != 500
        || bounds.z_min_m != -500
        || bounds.z_max_m != 500
        || bounds.y_min_m != -128
        || bounds.y_max_m != 128
        || bounds.voxel_edge_q8 != 64
        || bounds.brick_edge_voxels != 16
    {
        return Err(invalid("bounds.product_one_grid"));
    }
    let terrain = &config.terrain;
    if terrain.broad_scale_m == 0
        || terrain.meander_scale_m == 0
        || terrain.relief_m == 0
        || terrain.topsoil_depth_q8 == 0
        || terrain.subsoil_depth_q8 < terrain.topsoil_depth_q8
    {
        return Err(invalid("terrain"));
    }
    let geology = &config.geology;
    if geology.stratum_thickness_m.contains(&0)
        || geology.tilt_degrees.unsigned_abs() > 89
        || geology.aquifer_thickness_m == 0
        || geology.aquifer_material.0 != 5
        || geology.iron_vein_radius_q8 == 0
    {
        return Err(invalid("geology"));
    }
    let cave = &config.cave;
    if cave.entrance_tolerance_m == 0
        || cave.floor_tolerance_m == 0
        || cave.min_clear_width_q8 == 0
        || cave.min_clear_height_q8 == 0
        || cave.max_route_slope_degrees > 89
        || cave.floor_elevation_m >= cave.entrance_elevation_m
    {
        return Err(invalid("cave"));
    }
    let water = &config.water;
    if water.river_width_m == 0
        || water.river_depth_q8 == 0
        || water.lake_min_diameter_m == 0
        || water.lake_depth_m == 0
    {
        return Err(invalid("water"));
    }
    let biome = &config.biome;
    if biome.meadow_min_area_m2 == 0
        || biome.forest_min_area_m2 == 0
        || biome.forest_tree_spacing_m == 0
        || biome
            .tree_species_mix_percent
            .into_iter()
            .map(u16::from)
            .sum::<u16>()
            != 100
        || biome.tree_species_mix_percent.contains(&0)
        || biome.bushes_per_hectare == 0
    {
        return Err(invalid("biome"));
    }
    let objects = &config.objects;
    if objects.max_anchor_slope_degrees > 89
        || objects.route_clearance_m == 0
        || objects.index_cell_size_m == 0
        || objects.max_index_cells_per_object == 0
        || objects.max_index_entries_per_cell == 0
        || objects.sample_index_cell_size_m == 0
        || objects.max_sample_cells_per_object == 0
        || objects.max_sample_entries_per_cell == 0
        || objects.max_edit_dependency_candidates == 0
        || objects.max_affected_objects_per_edit == 0
        || objects.max_dependency_bricks_per_object == 0
        || objects.max_retained_index_bytes == 0
        || !range(objects.birch_trunk_radius_q8)
        || !range(objects.birch_height_m)
        || !range(objects.pine_trunk_radius_q8)
        || !range(objects.pine_height_m)
        || !range(objects.canopy_radius_q8)
        || !range(objects.bush_radius_q8)
        || !range(objects.boulder_radius_q8)
        || !range(objects.stump_radius_q8)
        || !range(objects.stump_height_q8)
        || !range(objects.rock_radius_q8)
    {
        return Err(invalid("objects"));
    }
    if config.ruin_stamp.is_empty() {
        return Err(invalid("ruin_stamp"));
    }
    Ok(())
}

fn validate_materials(materials: &MaterialRegistryConfig) -> Result<(), ConfigValidationError> {
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
    if materials.definitions.len() != KEYS.len() {
        return Err(invalid("materials.definitions"));
    }
    for (expected_id, (definition, key)) in materials.definitions.iter().zip(KEYS).enumerate() {
        let expected_collision = match expected_id {
            0 => CollisionClass::Empty,
            1 => CollisionClass::Fluid,
            _ => CollisionClass::Solid,
        };
        if definition.id.0 != expected_id as u8
            || definition.key != key
            || definition.collision != expected_collision
            || (expected_id < 2 && definition.hardness != 0)
            || (expected_id >= 2 && definition.hardness == 0)
            || definition.granular != matches!(expected_id, 4 | 5)
        {
            return Err(invalid("materials.canonical"));
        }
    }
    Ok(())
}

fn validate_presentation(config: &ConfigSet) -> Result<(), ConfigValidationError> {
    let presentation = &config.presentation;
    let streaming = &presentation.streaming;
    let mut end = 0;
    for band in &streaming.bands {
        if band.min_distance_m != end || band.max_distance_m <= band.min_distance_m {
            return Err(invalid("streaming.bands"));
        }
        end = band.max_distance_m;
    }
    if end != 720
        || streaming.hysteresis_m == 0
        || streaming.collision_radius_m == 0
        || streaming.vertical_surface_window_m == 0
        || streaming.prefetch_seconds_q8 == 0
        || streaming.max_generation_jobs == 0
        || streaming.max_mesh_jobs == 0
        || streaming.max_install_bytes_per_frame == 0
    {
        return Err(invalid("streaming"));
    }
    let mutation = &presentation.mutation;
    if mutation.fixed_hz == 0
        || mutation.min_radius_q8 == 0
        || mutation.debug_radius_q8 < mutation.min_radius_q8
        || mutation.debug_radius_q8 > mutation.max_radius_q8
        || mutation.max_radius_q8 < mutation.min_radius_q8
        || mutation.dig_strength == 0
        || mutation.place_strength == 0
        || mutation.inner_full_strength_percent == 0
        || mutation.inner_full_strength_percent > 100
        || mutation.max_atomic_bricks == 0
        || mutation.max_progressive_bricks < u32::from(mutation.max_atomic_bricks)
        || mutation.max_queued_edits == 0
        || mutation.max_commit_bricks_per_batch == 0
        || mutation.max_mutation_stage_ms_per_frame_q8 == 0
    {
        return Err(invalid("mutation"));
    }
    let player = &presentation.player;
    if !positive(player.body.radius_m)
        || !positive(player.body.cylinder_height_m)
        || !positive(player.run_speed_mps)
        || player.sprint_speed_mps < player.run_speed_mps
        || !positive(player.ground_accel_mps2)
        || !positive(player.air_accel_mps2)
        || !positive(player.gravity_mps2)
        || !positive(player.jump_speed_mps)
        || !positive(player.step_height_m)
        || !(0.0..90.0).contains(&player.max_walk_slope_deg)
        || !positive(player.paddle_speed_mps)
        || !positive(player.paddle_surface_offset_m)
    {
        return Err(invalid("player"));
    }
    let camera = &presentation.camera;
    if !positive(camera.min_distance_m)
        || camera.min_distance_m > camera.distance_m
        || camera.distance_m > camera.max_distance_m
        || camera.min_pitch_deg >= camera.max_pitch_deg
        || camera.min_pitch_deg < -90.0
        || camera.max_pitch_deg > 90.0
        || !positive(camera.probe_radius_m)
        || !positive(camera.collision_margin_m)
    {
        return Err(invalid("camera"));
    }
    if !positive(presentation.light.depth_enable_m)
        || !positive(presentation.light.range_m)
        || !positive(presentation.light.intensity_lm)
    {
        return Err(invalid("light"));
    }
    let rendering = &presentation.rendering;
    if rendering.window_width == 0
        || rendering.window_height == 0
        || rendering.msaa_samples == 0
        || rendering.shadow_map_size == 0
        || !rendering.time_of_day_hours.is_finite()
        || rendering.time_min_hours >= rendering.time_max_hours
        || !(rendering.time_min_hours..=rendering.time_max_hours)
            .contains(&rendering.time_of_day_hours)
        || !positive(rendering.time_keyboard_step_hours)
        || !finite_unit(rendering.grass_normal_min_y)
        || !positive(rendering.grass_near_density_per_m2)
        || !finite_unit(rendering.grass_middle_density_scale)
        || rendering.object_visibility_m == 0
        || rendering.cluster_visibility_m < rendering.object_visibility_m
        || rendering.horizon_object_cell_size_m == 0
        || !positive(rendering.horizon_derived_lod_m)
        || rendering.max_horizon_tree_members_per_cell == 0
    {
        return Err(invalid("rendering"));
    }
    let benchmark = &presentation.benchmark;
    if benchmark.warmup_frames == 0
        || benchmark.flythrough_duration_s == 0
        || benchmark.colony_worker_streams == 0
        || benchmark.colony_volume_m.contains(&0)
        || benchmark.catastrophic_radius_q8 != mutation.max_radius_q8
        || benchmark.watchdog_s == 0
        || !positive(benchmark.fps_target)
        || !positive(benchmark.max_mutation_frame_ms)
        || !positive(benchmark.max_admission_ms)
        || benchmark.max_first_commit_interactive_ms == 0
        || benchmark.max_first_commit_colony_ms == 0
        || benchmark.max_primary_ready_p95_ms > benchmark.max_primary_ready_ms
        || benchmark.min_changed_bricks_per_second == 0
        || benchmark.max_runnable_wait_ms == 0
        || benchmark.max_reconciliation_interactive_ms == 0
        || benchmark.max_reconciliation_volume_ms == 0
        || benchmark.cold_start_max_ms == 0
        || benchmark.graphics_memory_max_bytes == 0
    {
        return Err(invalid("benchmark"));
    }
    Ok(())
}

fn validate_input(config: &ConfigSet) -> Result<(), ConfigValidationError> {
    let input = &config.input;
    if !finite_unit(input.stick_dead_zone)
        || input.stick_dead_zone == 0.0
        || !positive(input.mouse_sensitivity_degrees)
        || !positive(input.gamepad_orbit_degrees_per_second)
    {
        return Err(invalid("input.tuning"));
    }
    let mut actions = BTreeSet::new();
    for binding in &input.bindings {
        if binding.keyboard_mouse.is_empty() || !actions.insert(binding.action as u8) {
            return Err(invalid("input.bindings"));
        }
    }
    if actions.len() != 17 {
        return Err(invalid("input.bindings"));
    }
    Ok(())
}

pub fn validate_config_set(config: &ConfigSet) -> Result<(), ConfigValidationError> {
    validate_region_config(&config.region)?;
    validate_materials(&config.materials)?;
    validate_presentation(config)?;
    validate_input(config)
}

#[cfg(test)]
mod tests {
    use crate::config::{ConfigSet, MaterialId, RegionConfig};

    #[test]
    fn product_one_defaults_validate() {
        assert!(ConfigSet::default().validate().is_ok());
    }

    #[test]
    fn region_validation_rejects_non_product_one_grid() {
        let mut config = RegionConfig::default();
        config.bounds.voxel_edge_q8 = 63;

        assert!(config.validate().is_err());
    }

    #[test]
    fn material_registry_requires_the_canonical_product_one_entries() {
        let mut config = ConfigSet::default();
        config.materials.definitions[5].id = MaterialId(99);

        assert!(config.validate().is_err());
    }

    #[test]
    fn region_ron_rejects_unknown_fields() {
        let mut ron = ron::ser::to_string(&RegionConfig::default()).unwrap();
        let closing = ron.pop().unwrap();
        assert_eq!(closing, ')');
        ron.push_str(", unexpected: 1)");

        assert!(ron::from_str::<RegionConfig>(&ron).is_err());
    }

    #[test]
    fn digest_boundary_excludes_runtime_configuration() {
        let config = ConfigSet::default();
        let authoritative = config.authoritative_ron().unwrap();

        let mut presentation_changed = config.clone();
        presentation_changed.presentation.rendering.window_width = 1920;
        presentation_changed.input.stick_dead_zone = 0.2;
        assert_eq!(
            presentation_changed.authoritative_ron().unwrap(),
            authoritative
        );

        let mut region_changed = config;
        region_changed.region.seed += 1;
        assert_ne!(region_changed.authoritative_ron().unwrap(), authoritative);
    }
}
