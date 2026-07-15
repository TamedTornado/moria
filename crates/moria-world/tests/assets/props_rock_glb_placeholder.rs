use std::{fs, path::PathBuf};

use moria_world::presentation::{AssetId, AssetLoader, AssetMissingAction, RuntimeAssetProfile};

const GLB_MAGIC: u32 = 0x4654_6C67;
const JSON_CHUNK_TYPE: u32 = 0x4E4F_534A;
const ROCK_PATH: &str = "props/rock.glb";

#[test]
fn rock_placeholder_uses_its_declared_path_and_fallback_contract() {
    let loader = AssetLoader::new();
    let declaration = loader.declaration(AssetId::Rock);

    assert_eq!(declaration.id.stable_id(), "moria.props.rock");
    assert_eq!(declaration.path, ROCK_PATH);
    assert_eq!(
        loader.resolve_runtime_path(ROCK_PATH),
        Ok(declaration),
        "the fixture must exercise the immutable shared loader path"
    );
    assert!(matches!(
        loader.missing_asset_action(AssetId::Rock, RuntimeAssetProfile::Development),
        AssetMissingAction::DevelopmentFallback {
            warning: "moria.props.rock"
        }
    ));
    assert_eq!(
        loader.missing_asset_action(AssetId::Rock, RuntimeAssetProfile::Release),
        AssetMissingAction::Fatal
    );
}

#[test]
fn rock_placeholder_is_a_support_centered_shared_material_binary_gltf() {
    let bytes = fs::read(asset_path()).expect("the rock placeholder must exist at its final path");
    assert!(
        bytes.len() >= 20,
        "GLB header and JSON chunk header are required"
    );
    assert_eq!(u32_at(&bytes, 0), GLB_MAGIC);
    assert_eq!(u32_at(&bytes, 4), 2, "the placeholder uses glTF 2.0");
    assert_eq!(u32_at(&bytes, 8) as usize, bytes.len());
    assert_eq!(u32_at(&bytes, 16), JSON_CHUNK_TYPE);

    let json_length = u32_at(&bytes, 12) as usize;
    let json = std::str::from_utf8(&bytes[20..20 + json_length])
        .expect("the GLB JSON chunk must be UTF-8")
        .trim_end();

    assert!(json.contains("\"asset\":{\"version\":\"2.0\""));
    assert!(json.contains("\"name\":\"RockVariantA\""));
    assert!(json.contains("\"name\":\"RockVariantB\""));
    assert_eq!(json.matches("\"materials\":[").count(), 1);
    assert_eq!(json.matches("\"material\":0").count(), 2);
    assert!(json.contains("\"min\":[-0.6,0,-0.6]"));
    assert!(json.contains("\"max\":[0.6,0.6,0.6]"));
    assert_eq!(
        json.matches("\"extras\":{\"triangle_count\":8}").count(),
        2,
        "each named variant uses eight indexed triangles, within the 3000-triangle prop budget"
    );
}

fn asset_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("assets")
        .join(ROCK_PATH)
}

fn u32_at(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(
        bytes[offset..offset + 4]
            .try_into()
            .expect("four-byte field"),
    )
}
