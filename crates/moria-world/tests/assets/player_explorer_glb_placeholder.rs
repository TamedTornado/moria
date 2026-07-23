use std::fs;

use moria_world::presentation::{AssetId, AssetLoader, AssetMissingAction, RuntimeAssetProfile};
use serde_json::Value;

const EXPLORER_PATH: &str = "player/explorer.glb";
const GLB_MAGIC: u32 = 0x4654_6C67;
const GLB_VERSION: u32 = 2;
const JSON_CHUNK: u32 = 0x4E4F_534A;
const BIN_CHUNK: u32 = 0x004E_4942;
const REQUIRED_CLIPS: [&str; 6] = ["Idle", "Run", "Sprint", "Jump", "Fall", "Paddle"];

#[test]
fn explorer_placeholder_is_a_skeletal_capsule_glb_with_required_clips() {
    let loader = AssetLoader::new();
    let declaration = loader.declaration(AssetId::Explorer);
    assert_eq!(declaration.id.stable_id(), "moria.player.explorer");
    assert_eq!(declaration.path, EXPLORER_PATH);
    assert_eq!(loader.resolve_runtime_path(EXPLORER_PATH), Ok(declaration));
    assert!(matches!(
        loader.missing_asset_action(AssetId::Explorer, RuntimeAssetProfile::Development),
        AssetMissingAction::DevelopmentFallback { warning }
            if warning == AssetId::Explorer.stable_id()
    ));
    assert_eq!(
        loader.missing_asset_action(AssetId::Explorer, RuntimeAssetProfile::Release),
        AssetMissingAction::Fatal
    );

    let bytes = fs::read(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../assets/player/explorer.glb"
    ))
    .expect("explorer placeholder exists at its declared runtime path");
    let document = glb_json(&bytes);

    assert_eq!(document["asset"]["version"], "2.0");
    assert_eq!(document["asset"]["extras"]["units"], "metres");
    assert_eq!(document["asset"]["extras"]["up_axis"], "Y");
    assert_eq!(document["asset"]["extras"]["forward_axis"], "+Z");
    assert_eq!(document["asset"]["extras"]["origin"], "support_center");

    assert_eq!(document["materials"].as_array().map(Vec::len), Some(1));
    let primitive = &document["meshes"].as_array().expect("one explorer mesh")[0]["primitives"]
        .as_array()
        .expect("explorer mesh primitive")[0];
    assert_eq!(primitive["material"], 0);
    for attribute in [
        "POSITION",
        "NORMAL",
        "TEXCOORD_0",
        "TANGENT",
        "JOINTS_0",
        "WEIGHTS_0",
    ] {
        assert!(primitive["attributes"].get(attribute).is_some());
    }

    let accessors = document["accessors"].as_array().expect("accessors array");
    let index_count =
        accessors[primitive["indices"].as_u64().expect("index accessor") as usize]["count"]
            .as_u64()
            .expect("index count");
    assert_eq!(index_count % 3, 0, "explorer uses indexed triangles");
    assert!(index_count / 3 <= 40_000, "explorer triangle budget");
    let position = &accessors[primitive["attributes"]["POSITION"]
        .as_u64()
        .expect("position accessor") as usize];
    assert_eq!(position["min"][1].as_f64(), Some(0.0));
    assert!(position["min"][0].as_f64().is_some_and(|value| value < 0.0));
    assert!(position["max"][0].as_f64().is_some_and(|value| value > 0.0));
    assert!(
        position["max"][1]
            .as_f64()
            .is_some_and(|value| value >= 1.5)
    );
    assert!(position["min"][2].as_f64().is_some_and(|value| value < 0.0));
    assert!(position["max"][2].as_f64().is_some_and(|value| value > 0.0));

    let skin = &document["skins"].as_array().expect("explorer skeleton")[0];
    assert!(
        skin["joints"]
            .as_array()
            .is_some_and(|joints| !joints.is_empty())
    );
    assert!(skin.get("inverseBindMatrices").is_some());

    let scene_index = document["scene"].as_u64().expect("default scene") as usize;
    let scene = &document["scenes"].as_array().expect("scenes array")[scene_index];
    let skeleton_node = skin["skeleton"].as_u64().expect("skeleton root");
    assert!(
        scene["nodes"].as_array().is_some_and(|nodes| nodes
            .iter()
            .any(|node| node.as_u64() == Some(skeleton_node))),
        "the instantiated scene includes the skeleton root"
    );
    let mesh_node = document["nodes"]
        .as_array()
        .expect("explorer nodes")
        .iter()
        .position(|node| node["mesh"].as_u64() == Some(0))
        .expect("skinned explorer mesh node") as u64;
    assert_eq!(document["nodes"][mesh_node as usize]["skin"], 0);
    assert!(
        document["nodes"][skeleton_node as usize]["children"]
            .as_array()
            .is_some_and(|children| children.iter().any(|node| node.as_u64() == Some(mesh_node))),
        "the skinned mesh is parented under the instantiated skeleton root"
    );

    let clips: Vec<_> = document["animations"]
        .as_array()
        .expect("required animation clips")
        .iter()
        .map(|animation| animation["name"].as_str().expect("named animation"))
        .collect();
    assert_eq!(clips, REQUIRED_CLIPS);
    for animation in document["animations"].as_array().expect("animations") {
        assert!(
            !animation["channels"]
                .as_array()
                .expect("animation channels")
                .is_empty()
        );
        assert!(
            !animation["samplers"]
                .as_array()
                .expect("animation samplers")
                .is_empty()
        );
    }
}

fn glb_json(bytes: &[u8]) -> Value {
    assert!(bytes.len() >= 20, "GLB header and JSON chunk header");
    assert_eq!(u32_at(bytes, 0), GLB_MAGIC);
    assert_eq!(u32_at(bytes, 4), GLB_VERSION);
    assert_eq!(u32_at(bytes, 8) as usize, bytes.len());
    let json_length = u32_at(bytes, 12) as usize;
    assert_eq!(u32_at(bytes, 16), JSON_CHUNK);
    let binary_offset = 20 + json_length;
    assert!(bytes.len() >= binary_offset + 8, "GLB binary chunk header");
    let binary_length = u32_at(bytes, binary_offset) as usize;
    assert_eq!(u32_at(bytes, binary_offset + 4), BIN_CHUNK);
    assert_eq!(binary_offset + 8 + binary_length, bytes.len());
    serde_json::from_slice(&bytes[20..binary_offset]).expect("valid GLB JSON chunk")
}

fn u32_at(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(
        bytes[offset..offset + 4]
            .try_into()
            .expect("four-byte field"),
    )
}
