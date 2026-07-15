use std::{fs, path::PathBuf};

use moria_world::presentation::{
    AssetId, AssetLoader, AssetMissingAction, RuntimeAssetProfile,
};

const EXPLORER_PATH: &str = "player/explorer.glb";
const REQUIRED_CLIPS: [&str; 6] = ["Idle", "Run", "Sprint", "Jump", "Fall", "Paddle"];

fn explorer_asset_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("assets")
        .join(EXPLORER_PATH)
}

fn glb_json(bytes: &[u8]) -> &str {
    assert!(bytes.len() >= 20, "GLB must contain a header and JSON chunk");
    assert_eq!(&bytes[..4], b"glTF");
    assert_eq!(u32::from_le_bytes(bytes[4..8].try_into().unwrap()), 2);
    assert_eq!(u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize, bytes.len());

    let json_length = u32::from_le_bytes(bytes[12..16].try_into().unwrap()) as usize;
    assert_eq!(&bytes[16..20], b"JSON");
    let json_end = 20 + json_length;
    std::str::from_utf8(&bytes[20..json_end]).unwrap().trim_end()
}

#[test]
fn explorer_placeholder_resolves_through_the_declared_loader_and_exposes_fallbacks() {
    let loader = AssetLoader::new();
    let declaration = loader.declaration(AssetId::Explorer);

    assert_eq!(declaration.id, AssetId::Explorer);
    assert_eq!(declaration.path, EXPLORER_PATH);
    assert_eq!(loader.resolve_runtime_path(EXPLORER_PATH), Ok(declaration));
    assert_eq!(loader.validation_fixture(AssetId::Explorer).key, "moria.player.explorer");
    assert_eq!(
        loader.missing_asset_action(AssetId::Explorer, RuntimeAssetProfile::Development),
        AssetMissingAction::DevelopmentFallback {
            warning: "moria.player.explorer",
        }
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::Explorer, RuntimeAssetProfile::Release),
        AssetMissingAction::Fatal
    );
}

#[test]
fn explorer_placeholder_is_a_capsule_proportioned_skeletal_binary_gltf() {
    let json = glb_json(&fs::read(explorer_asset_path()).unwrap());

    assert!(json.contains("\"asset\":{\"version\":\"2.0\""));
    assert!(json.contains("\"name\":\"ExplorerCapsule\""));
    assert!(json.contains("\"name\":\"ExplorerSkeleton\""));
    assert!(json.contains("\"joints\":[1]"));
    assert!(json.contains("\"min\":[-0.5,0,-0.5]"));
    assert!(json.contains("\"max\":[0.5,2,0.5]"));
    for clip in REQUIRED_CLIPS {
        assert!(json.contains(&format!("\"name\":\"{clip}\"")));
    }
}
