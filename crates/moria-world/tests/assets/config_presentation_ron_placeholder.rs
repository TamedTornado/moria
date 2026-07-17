use std::{fs, path::Path};

use moria_world::presentation::{
    AssetId, AssetLoadPolicy, AssetLoader, AssetMissingAction, RuntimeAssetProfile,
};
use ron::Value;

const PRESENTATION_PATH: &str = "config/presentation.ron";

#[test]
fn presentation_config_placeholder_uses_its_declared_required_runtime_path() {
    let loader = AssetLoader::new();
    let declaration = loader.declaration(AssetId::PresentationConfig);

    assert_eq!(declaration.id.stable_id(), "moria.config.presentation");
    assert_eq!(declaration.path, PRESENTATION_PATH);
    assert_eq!(declaration.load_policy, AssetLoadPolicy::Required);
    assert_eq!(
        loader.resolve_runtime_path(PRESENTATION_PATH),
        Ok(declaration)
    );
    assert_eq!(
        loader.validation_fixture(AssetId::PresentationConfig).key,
        declaration.id.stable_id(),
    );
    assert_eq!(
        loader.missing_asset_action(
            AssetId::PresentationConfig,
            RuntimeAssetProfile::Development
        ),
        AssetMissingAction::Fatal,
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::PresentationConfig, RuntimeAssetProfile::Release),
        AssetMissingAction::Fatal,
    );
}

#[test]
fn presentation_config_placeholder_contains_the_product_one_defaults() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/config/presentation.ron");
    let source = fs::read_to_string(path)
        .expect("presentation placeholder exists at its declared runtime path");
    let value: Value = ron::de::from_str(&source).expect("presentation placeholder is valid RON");

    assert_eq!(value, expected_presentation_config());
}

fn expected_presentation_config() -> Value {
    ron::de::from_str(
        r#"(
            enabled: true,
            streaming: (
                bands: [
                    (band: Near, start_m: 0, end_m: 64, voxel_edge_q8: 64),
                    (band: Middle, start_m: 64, end_m: 160, voxel_edge_q8: 128),
                    (band: Far, start_m: 160, end_m: 320, voxel_edge_q8: 256),
                    (band: Horizon, start_m: 320, end_m: 720, voxel_edge_q8: 1024),
                ],
                hysteresis_m: 12,
                collision_radius_m: 12,
                vertical_surface_window_m: 12,
                prefetch_seconds_q8: 512,
                max_generation_jobs: 96,
                max_mesh_jobs: 64,
                max_install_bytes_per_frame: 16_777_216,
                edit_reserved_workers: 2,
            ),
            mutation: (
                fixed_hz: 60,
                debug_radius_q8: 768,
                min_radius_q8: 64,
                max_radius_q8: 4096,
                dig_strength: 255,
                place_strength: 255,
                inner_full_strength_percent: 70,
                max_atomic_bricks: 32,
                max_progressive_bricks: 8192,
                max_queued_edits: 32,
                max_commit_bricks_per_batch: 8,
                max_mutation_stage_ms_per_frame_q8: 1024,
            ),
            player: (
                body: (radius_m: 0.40, cylinder_height_m: 1.00),
                run_speed_mps: 5.0,
                sprint_speed_mps: 8.0,
                ground_accel_mps2: 35.0,
                air_accel_mps2: 10.0,
                gravity_mps2: 22.0,
                jump_speed_mps: 7.0,
                step_height_m: 0.30,
                max_walk_slope_deg: 48.0,
                paddle_speed_mps: 3.0,
                paddle_surface_offset_m: 0.55,
            ),
            camera: (
                distance_m: 5.5,
                min_distance_m: 2.0,
                max_distance_m: 9.0,
                min_pitch_deg: -65.0,
                max_pitch_deg: 75.0,
                probe_radius_m: 0.18,
                collision_margin_m: 0.12,
                light_depth_enable_m: 2.0,
                light_range_m: 18.0,
                light_intensity_lm: 1600.0,
            ),
            rendering: (
                window_width: 2560,
                window_height: 1440,
                msaa_samples: 4,
                shadow_map_size: 2048,
                time_of_day_hours: 14.0,
                time_min_hours: 6.0,
                time_max_hours: 20.0,
                time_keyboard_step_hours: 0.25,
                grass_normal_min_y: 0.75,
                grass_near_density_per_m2: 5.0,
                grass_middle_density_scale: 0.25,
                object_visibility_m: 320,
                cluster_visibility_m: 720,
                horizon_object_cell_size_m: 64,
                horizon_derived_lod_m: 4.0,
                max_horizon_tree_members_per_cell: 1024,
            ),
            benchmark: (
                warmup_frames: 300,
                flythrough_duration_s: 120,
                colony_worker_streams: 8,
                colony_volume_m: [32, 32, 16],
                catastrophic_radius_q8: 4096,
                watchdog_s: 300,
                fps_target: 60.0,
                max_mutation_frame_ms: 33.3,
                max_admission_ms: 2.0,
                max_first_commit_interactive_ms: 100,
                max_first_commit_colony_ms: 250,
                max_primary_ready_p95_ms: 250,
                max_primary_ready_max_ms: 500,
                min_changed_bricks_per_second: 32,
                max_runnable_wait_ms: 500,
                max_reconciliation_interactive_ms: 1000,
                max_reconciliation_volume_ms: 30000,
                cold_start_max_ms: 5000,
                graphics_memory_max_bytes: 2097152000,
                save_max_bytes: 50000000,
                forest_object_validation_max_ms: 1000,
                forest_object_index_build_max_ms: 250,
                carve_object_dependency_max_ms: 1.0,
                query_frame_critical_p99_ms: 1.0,
                query_normal_bundle_p99_ms: 2.0,
                query_frame_critical_max_ms: 4.0,
                query_cells_page_p99_ms: 4.0,
                query_cells_page_max_ms: 8.0,
            ),
        )"#,
    )
    .expect("the Product One presentation schema is valid RON")
}
