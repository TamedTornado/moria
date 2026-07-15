use moria_world::presentation::{
    AssetId, AssetLoadPolicy, AssetLoader, AssetMissingAction, RuntimeAssetProfile,
};

const VEGETATION_SHADER: &str = include_str!("../../../../assets/shaders/vegetation.wgsl");

#[test]
fn vegetation_shader_placeholder_uses_its_declared_path_and_observable_fallback() {
    let loader = AssetLoader::new();
    let declaration = loader.declaration(AssetId::VegetationShader);

    assert_eq!(declaration.id.stable_id(), "moria.shaders.vegetation");
    assert_eq!(declaration.path, "shaders/vegetation.wgsl");
    assert_eq!(declaration.load_policy, AssetLoadPolicy::ReleaseFatal);
    assert_eq!(
        loader.resolve_runtime_path(declaration.path),
        Ok(declaration),
    );
    assert_eq!(
        loader.validation_fixture(AssetId::VegetationShader).key,
        declaration.id.stable_id(),
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::VegetationShader, RuntimeAssetProfile::Development),
        AssetMissingAction::DevelopmentFallback {
            warning: "moria.shaders.vegetation",
        },
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::VegetationShader, RuntimeAssetProfile::Release),
        AssetMissingAction::Fatal,
    );
}

#[test]
fn vegetation_shader_placeholder_declares_portable_instance_inputs() {
    for required in [
        "#import bevy_pbr",
        "mesh_functions::get_world_from_local",
        "vertex.instance_index",
        "instance_transform",
        "variation",
        "@vertex",
        "@fragment",
    ] {
        assert!(
            VEGETATION_SHADER.contains(required),
            "vegetation shader is missing {required:?}",
        );
    }

    for forbidden in ["i64", "u64", "atomic<i64>", "atomic<u64>"] {
        assert!(
            !VEGETATION_SHADER.contains(forbidden),
            "vegetation shader must remain portable and 32-bit-only: found {forbidden:?}",
        );
    }
}
