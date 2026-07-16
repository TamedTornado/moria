use moria_world::presentation::{
    AssetId, AssetLoadPolicy, AssetLoader, AssetMissingAction, RuntimeAssetProfile,
};

const TERRAIN_SHADER: &str = include_str!("../../../../assets/shaders/terrain.wgsl");

#[test]
fn terrain_shader_placeholder_uses_its_declared_path_and_observable_fallback() {
    let loader = AssetLoader::new();
    let declaration = loader.declaration(AssetId::TerrainShader);

    assert_eq!(declaration.id.stable_id(), "moria.shaders.terrain");
    assert_eq!(declaration.path, "shaders/terrain.wgsl");
    assert_eq!(declaration.load_policy, AssetLoadPolicy::ReleaseFatal);
    assert_eq!(
        loader.resolve_runtime_path(declaration.path),
        Ok(declaration),
    );
    assert_eq!(
        loader.validation_fixture(AssetId::TerrainShader).key,
        declaration.id.stable_id(),
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::TerrainShader, RuntimeAssetProfile::Development),
        AssetMissingAction::DevelopmentFallback {
            warning: "moria.shaders.terrain",
        },
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::TerrainShader, RuntimeAssetProfile::Release),
        AssetMissingAction::Fatal,
    );
}

#[test]
fn terrain_shader_placeholder_declares_four_layer_shared_array_material_inputs() {
    for required in [
        "@location(0) position: vec3<f32>",
        "@location(1) normal: vec3<f32>",
        "@location(2) material_ids: vec4<u32>",
        "@location(3) material_weights: vec4<f32>",
        "texture_2d_array<f32>",
        "terrain_albedo_layers",
        "terrain_normal_layers",
        "terrain_orm_layers",
        "textureSample",
        "input.material_ids",
        "input.material_weights",
        "@vertex",
        "@fragment",
    ] {
        assert!(
            TERRAIN_SHADER.contains(required),
            "terrain shader is missing {required:?}",
        );
    }

    for forbidden in ["i64", "u64", "atomic<i64>", "atomic<u64>"] {
        assert!(
            !TERRAIN_SHADER.contains(forbidden),
            "terrain shader must remain portable and 32-bit-only: found {forbidden:?}",
        );
    }
}
