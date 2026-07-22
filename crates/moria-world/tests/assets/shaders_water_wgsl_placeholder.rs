use std::{fs, path::PathBuf};

use moria_world::presentation::{
    AssetId, AssetLoadPolicy, AssetLoader, AssetMissingAction, RuntimeAssetProfile,
};

fn runtime_asset_path(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("assets")
        .join(path)
}

fn function_body<'a>(source: &'a str, function_name: &str) -> &'a str {
    let signature = format!("fn {function_name}(");
    let function_start = source
        .find(&signature)
        .unwrap_or_else(|| panic!("water shader declares conventional `{signature}` entry point"));
    let body_start = source[function_start..]
        .find('{')
        .map(|offset| function_start + offset)
        .expect("water shader entry point has a body");

    let mut depth = 0_u32;
    for (offset, character) in source[body_start..].char_indices() {
        match character {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return &source[body_start + 1..body_start + offset];
                }
            }
            _ => {}
        }
    }

    panic!("water shader entry point `{function_name}` has a closing brace");
}

#[test]
fn water_shader_placeholder_uses_the_predeclared_loader_route() {
    let loader = AssetLoader::new();
    let declaration = loader.declaration(AssetId::WaterShader);

    assert_eq!(declaration.id, AssetId::WaterShader);
    assert_eq!(declaration.id.stable_id(), "moria.shaders.water");
    assert_eq!(declaration.path, "shaders/water.wgsl");
    assert_eq!(declaration.load_policy, AssetLoadPolicy::ReleaseFatal);
    assert_eq!(
        declaration.validation_fixture.asset_id,
        AssetId::WaterShader
    );
    assert_eq!(declaration.validation_fixture.key, "moria.shaders.water");
    assert_eq!(
        loader.validation_fixture(AssetId::WaterShader),
        declaration.validation_fixture
    );
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
    let vertex_body = function_body(&source, "vertex");
    assert!(source.contains("@fragment"));
    let fragment_body = function_body(&source, "fragment");
    assert!(source.contains("@location(0) position: vec3<f32>"));
    assert!(source.contains("@location(1) normal: vec3<f32>"));
    assert!(vertex_body.contains("vec4<f32>(input.position, 1.0)"));
    assert!(vertex_body.contains("normalize(input.normal)"));
    assert!(!vertex_body.contains("water_time"));
    assert!(source.contains("var<uniform> water_time: WaterTime"));
    assert!(fragment_body.contains("water_time.seconds"));
    assert!(!source.contains("i64"));
    assert!(!source.contains("u64"));
    assert!(!source.contains("atomic<"));
}
