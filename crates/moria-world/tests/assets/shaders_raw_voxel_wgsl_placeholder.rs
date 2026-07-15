use moria_world::presentation::{AssetId, AssetLoader, AssetMissingAction, RuntimeAssetProfile};

const RAW_VOXEL_SHADER: &str = include_str!("../../../../assets/shaders/raw_voxel.wgsl");

#[test]
fn raw_voxel_shader_placeholder_uses_the_declared_material_id_diagnostic_contract() {
    let loader = AssetLoader::new();
    let declaration = loader
        .resolve_runtime_path("shaders/raw_voxel.wgsl")
        .expect("the raw-voxel shader path must remain declared");

    assert_eq!(declaration.id, AssetId::RawVoxelShader);
    assert_eq!(declaration.path, "shaders/raw_voxel.wgsl");
    assert_eq!(
        loader.validation_fixture(AssetId::RawVoxelShader).key,
        "moria.shaders.raw_voxel"
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::RawVoxelShader, RuntimeAssetProfile::Development),
        AssetMissingAction::DevelopmentFallback {
            warning: "moria.shaders.raw_voxel",
        }
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::RawVoxelShader, RuntimeAssetProfile::Release),
        AssetMissingAction::Fatal
    );

    assert!(RAW_VOXEL_SHADER.contains("material_id: u32"));
    assert!(RAW_VOXEL_SHADER.contains("input.material_id"));
    assert!(RAW_VOXEL_SHADER.contains("@vertex"));
    assert!(RAW_VOXEL_SHADER.contains("@fragment"));
    assert!(!RAW_VOXEL_SHADER.contains("i64"));
    assert!(!RAW_VOXEL_SHADER.contains("u64"));
    assert!(!RAW_VOXEL_SHADER.contains("atomic"));
}
