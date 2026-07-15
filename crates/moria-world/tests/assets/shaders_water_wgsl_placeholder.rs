use std::{fs, path::PathBuf};

use moria_world::presentation::{AssetId, AssetLoader, AssetMissingAction, RuntimeAssetProfile};

fn runtime_asset_path(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("assets")
        .join(path)
}

#[test]
fn water_shader_placeholder_uses_the_predeclared_loader_route() {
    let loader = AssetLoader::new();
    let declaration = loader.declaration(AssetId::WaterShader);

    assert_eq!(declaration.id.stable_id(), "moria.shaders.water");
    assert_eq!(declaration.path, "shaders/water.wgsl");
    assert_eq!(
        loader.resolve_runtime_path(declaration.path),
        Ok(declaration)
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::WaterShader, RuntimeAssetProfile::Development),
        AssetMissingAction::DevelopmentFallback {
            warning: "moria.shaders.water",
        }
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::WaterShader, RuntimeAssetProfile::Release),
        AssetMissingAction::Fatal
    );

    let source = fs::read_to_string(runtime_asset_path(declaration.path))
        .expect("the water shader placeholder exists at its declared runtime path");
    assert!(source.contains("@vertex"));
    assert!(source.contains("@fragment"));
    assert!(source.contains("@location(0) position: vec3<f32>"));
    assert!(source.contains("@location(1) normal: vec3<f32>"));
    assert!(source.contains("var<uniform> water_time: WaterTime"));
    assert!(source.contains("water_time.seconds"));
    assert!(!source.contains("i64"));
    assert!(!source.contains("u64"));
    assert!(!source.contains("atomic<"));
}
