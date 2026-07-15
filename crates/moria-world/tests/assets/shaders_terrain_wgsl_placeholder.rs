use moria_world::presentation::{AssetId, AssetLoader, AssetMissingAction, RuntimeAssetProfile};

const TERRAIN_SHADER_SOURCE: &str = include_str!("../../../../assets/shaders/terrain.wgsl");

#[test]
fn terrain_shader_placeholder_uses_its_declared_path_and_portable_material_contract() {
    let loader = AssetLoader::new();
    let declaration = loader.declaration(AssetId::TerrainShader);

    assert_eq!(declaration.id.stable_id(), "moria.shaders.terrain");
    assert_eq!(declaration.path, "shaders/terrain.wgsl");
    assert_eq!(
        loader.resolve_runtime_path(declaration.path),
        Ok(declaration)
    );
    assert_eq!(
        loader.validation_fixture(AssetId::TerrainShader),
        declaration.validation_fixture
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::TerrainShader, RuntimeAssetProfile::Development),
        AssetMissingAction::DevelopmentFallback {
            warning: "moria.shaders.terrain",
        }
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::TerrainShader, RuntimeAssetProfile::Release),
        AssetMissingAction::Fatal,
    );

    for required_declaration in [
        "@location(0) position: vec3<f32>",
        "@location(1) normal: vec3<f32>",
        "@location(2) @interpolate(flat) material_ids: vec4<u32>",
        "@location(3) material_weights: vec4<f32>",
        "var terrain_albedo_layers: texture_2d_array<f32>",
        "var terrain_normal_layers: texture_2d_array<f32>",
        "var terrain_orm_layers: texture_2d_array<f32>",
        "fn vertex",
        "fn fragment",
    ] {
        assert!(
            TERRAIN_SHADER_SOURCE.contains(required_declaration),
            "terrain shader must declare {required_declaration}"
        );
    }

    for forbidden_declaration in ["i64", "u64", "atomic<"] {
        assert!(
            !TERRAIN_SHADER_SOURCE.contains(forbidden_declaration),
            "terrain shader must remain portable and 32-bit only: {forbidden_declaration}"
        );
    }
}
