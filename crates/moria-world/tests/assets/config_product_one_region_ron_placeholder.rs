use std::{fs, path::PathBuf};

use moria_world::{
    MaterialId, RegionConfig,
    presentation::{AssetId, AssetLoader, AssetMissingAction, RuntimeAssetProfile},
    validate_region_config,
};

const REGION_CONFIG_PATH: &str = "config/product_one_region.ron";

#[test]
fn product_one_region_placeholder_uses_the_predeclared_loader_route_and_required_policy() {
    let loader = AssetLoader::new();
    let declaration = loader.declaration(AssetId::ProductOneRegion);

    assert_eq!(
        declaration.id.stable_id(),
        "moria.config.product_one_region"
    );
    assert_eq!(declaration.path, REGION_CONFIG_PATH);
    assert_eq!(
        loader.resolve_runtime_path(REGION_CONFIG_PATH),
        Ok(declaration),
        "the fixture must exercise the immutable shared loader path"
    );
    assert_eq!(
        loader.validation_fixture(AssetId::ProductOneRegion).key,
        declaration.id.stable_id()
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::ProductOneRegion, RuntimeAssetProfile::Development),
        AssetMissingAction::Fatal,
        "authoritative region configuration cannot silently fall back"
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::ProductOneRegion, RuntimeAssetProfile::Release),
        AssetMissingAction::Fatal
    );
}

#[test]
fn product_one_region_placeholder_has_normative_generation_and_object_constraints() {
    let config = parse_config();

    assert_eq!(config.seed, 0x4D4F_5249_415F_5031);
    assert_eq!(
        (
            config.bounds.x_min_m,
            config.bounds.x_max_m,
            config.bounds.y_min_m,
            config.bounds.y_max_m,
            config.bounds.z_min_m,
            config.bounds.z_max_m,
        ),
        (-500, 500, -128, 128, -500, 500)
    );
    assert_eq!(config.bounds.voxel_edge_q8, 64);
    assert_eq!(config.bounds.brick_edge_voxels, 16);

    assert_eq!(config.terrain.typical_surface_m, 64);
    assert_eq!(config.terrain.broad_scale_m, 220);
    assert_eq!(config.terrain.meander_scale_m, 72);
    assert_eq!(config.terrain.relief_m, 34);
    assert_eq!(config.terrain.topsoil_depth_q8, 256);
    assert_eq!(config.terrain.subsoil_depth_q8, 768);
    assert_eq!(config.geology.stratum_thickness_m, [8, 12, 10, 18]);
    assert_eq!(config.geology.tilt_degrees, 18);
    assert_eq!(config.geology.aquifer_thickness_m, 6);
    assert_eq!(config.geology.aquifer_material, MaterialId(5));
    assert_eq!(config.geology.iron_vein_radius_q8, 320);
    assert_eq!(config.cave.entrance_elevation_m, 0);
    assert_eq!(config.cave.entrance_tolerance_m, 2);
    assert_eq!(config.cave.floor_elevation_m, -40);
    assert_eq!(config.cave.floor_tolerance_m, 2);
    assert_eq!(config.cave.min_clear_width_q8, 768);
    assert_eq!(config.cave.min_clear_height_q8, 768);
    assert_eq!(config.cave.max_route_slope_degrees, 35);
    assert_eq!(config.water.river_width_m, 10);
    assert_eq!(config.water.river_depth_q8, 512);
    assert_eq!(config.water.lake_min_diameter_m, 80);
    assert_eq!(config.water.lake_depth_m, 6);

    assert_eq!(config.biome.meadow_min_area_m2, 40_000);
    assert_eq!(config.biome.forest_min_area_m2, 120_000);
    assert_eq!(config.biome.forest_tree_spacing_m, 5);
    assert_eq!(config.biome.tree_species_mix_percent, [55, 45]);
    assert_eq!(config.biome.bushes_per_hectare, 450);
    assert_eq!(config.objects.boulders_per_hectare, 24);
    assert_eq!(config.objects.stumps_per_hectare, 14);
    assert_eq!(config.objects.rocks_per_hectare, 90);
    assert_eq!(config.objects.max_anchor_slope_degrees, 32);
    assert_eq!(config.objects.route_clearance_m, 3);
    assert_eq!(config.objects.index_cell_size_m, 32);
    assert_eq!(config.objects.max_index_cells_per_object, 16);
    assert_eq!(config.objects.max_index_entries_per_cell, 1_024);
    assert_eq!(config.objects.sample_index_cell_size_m, 4);
    assert_eq!(config.objects.max_sample_cells_per_object, 16);
    assert_eq!(config.objects.max_sample_entries_per_cell, 64);
    assert_eq!(config.objects.max_edit_dependency_candidates, 256);
    assert_eq!(config.objects.max_affected_objects_per_edit, 64);
    assert_eq!(config.objects.max_dependency_bricks_per_object, 128);
    assert_eq!(config.objects.max_retained_index_bytes, 16_777_216);
    assert_eq!(config.objects.birch_trunk_radius_q8.min_q8, 51);
    assert_eq!(config.objects.birch_trunk_radius_q8.max_q8, 90);
    assert_eq!(config.objects.birch_trunk_height_q8.min_q8, 2_048);
    assert_eq!(config.objects.birch_trunk_height_q8.max_q8, 3_584);
    assert_eq!(config.objects.pine_trunk_radius_q8.min_q8, 64);
    assert_eq!(config.objects.pine_trunk_radius_q8.max_q8, 115);
    assert_eq!(config.objects.pine_trunk_height_q8.min_q8, 2_560);
    assert_eq!(config.objects.pine_trunk_height_q8.max_q8, 4_608);
    assert_eq!(config.objects.canopy_radius_q8.min_q8, 512);
    assert_eq!(config.objects.canopy_radius_q8.max_q8, 1_024);
    assert_eq!(config.objects.bush_radius_q8.min_q8, 128);
    assert_eq!(config.objects.bush_radius_q8.max_q8, 307);
    assert_eq!(config.objects.boulder_radius_q8.min_q8, 128);
    assert_eq!(config.objects.boulder_radius_q8.max_q8, 461);
    assert_eq!(config.objects.stump_radius_q8.min_q8, 64);
    assert_eq!(config.objects.stump_radius_q8.max_q8, 141);
    assert_eq!(config.objects.stump_height_q8.min_q8, 64);
    assert_eq!(config.objects.stump_height_q8.max_q8, 192);
    assert_eq!(config.objects.rock_radius_q8.min_q8, 38);
    assert_eq!(config.objects.rock_radius_q8.max_q8, 154);
    assert_eq!(config.ruin_stamp, "stamps/ruin_p1.ron");
    assert!(validate_region_config(&config).is_ok());
}

#[test]
fn product_one_region_placeholder_rejects_unknown_fields() {
    let invalid = fs::read_to_string(asset_path())
        .expect("the region config placeholder exists at its final path")
        .replacen("    seed:", "    unexpected_field: 0,\n    seed:", 1);

    assert!(
        ron::de::from_str::<RegionConfig>(&invalid).is_err(),
        "the authoritative config schema must deny unknown fields"
    );
}

fn parse_config() -> RegionConfig {
    ron::de::from_str(
        &fs::read_to_string(asset_path())
            .expect("the region config placeholder exists at its final path"),
    )
    .expect("the region config placeholder uses the documented RON schema")
}

fn asset_path() -> PathBuf {
    let declaration = AssetLoader::new()
        .resolve_runtime_path(REGION_CONFIG_PATH)
        .expect("the region config uses its immutable shared loader route");
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("assets")
        .join(declaration.path)
}
