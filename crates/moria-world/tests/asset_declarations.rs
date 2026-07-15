use std::collections::BTreeSet;

use moria_world::presentation::{
    ASSET_COUNT, ASSET_DECLARATIONS, AssetDeclarationError, AssetId, AssetLoader,
};

#[path = "assets/shaders_terrain_wgsl_placeholder.rs"]
mod shaders_terrain_wgsl_placeholder;

#[test]
fn declaration_table_is_the_complete_unique_runtime_inventory() {
    let expected_paths = BTreeSet::from([
        "config/product_one_region.ron",
        "config/curated_manifest.ron",
        "config/presentation.ron",
        "config/input.ron",
        "materials/materials.ron",
        "materials/terrain_albedo.ktx2",
        "materials/terrain_normal.ktx2",
        "materials/terrain_orm.ktx2",
        "materials/water_normal.ktx2",
        "stamps/ruin_p1.ron",
        "vegetation/birch_near.glb",
        "vegetation/birch_mid.glb",
        "vegetation/birch_far.glb",
        "vegetation/pine_near.glb",
        "vegetation/pine_mid.glb",
        "vegetation/pine_far.glb",
        "vegetation/bush_near.glb",
        "vegetation/bush_far.glb",
        "vegetation/grass_cluster.glb",
        "vegetation/tree_horizon_cards.ktx2",
        "props/boulder.glb",
        "props/stump.glb",
        "props/rock.glb",
        "player/explorer.glb",
        "shaders/terrain.wgsl",
        "shaders/water.wgsl",
        "shaders/vegetation.wgsl",
        "shaders/raw_voxel.wgsl",
        "manifests/asset_licenses.ron",
        "manifests/asset_budgets.ron",
    ]);
    let declared_paths: BTreeSet<_> = ASSET_DECLARATIONS
        .iter()
        .map(|declaration| declaration.path)
        .collect();

    assert_eq!(ASSET_DECLARATIONS.len(), ASSET_COUNT);
    assert_eq!(ASSET_COUNT, 30);
    assert_eq!(declared_paths, expected_paths);
    assert_eq!(
        ASSET_DECLARATIONS
            .iter()
            .filter(|declaration| declaration.id == AssetId::ProductOneRegion)
            .count(),
        1
    );
    assert_eq!(
        ASSET_DECLARATIONS
            .iter()
            .filter(|declaration| declaration.path == "config/product_one_region.ron")
            .count(),
        1
    );
}

#[test]
fn loader_rejects_runtime_paths_not_in_the_immutable_table() {
    let loader = AssetLoader::new();

    assert_eq!(
        loader.resolve_runtime_path("invented/audio.ogg"),
        Err(AssetDeclarationError::UndeclaredRuntimePath {
            path: "invented/audio.ogg".to_owned(),
        })
    );
}
