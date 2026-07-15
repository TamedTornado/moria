use std::{fs, path::PathBuf};

use moria_world::presentation::{AssetId, AssetLoader, AssetMissingAction, RuntimeAssetProfile};
use serde_json::Value;

const BUSH_FAR_PATH: &str = "vegetation/bush_far.glb";
const GLB_MAGIC: u32 = 0x4654_6C67;
const GLB_VERSION: u32 = 2;
const JSON_CHUNK: u32 = 0x4E4F_534A;
const BIN_CHUNK: u32 = 0x004E_4942;

#[test]
fn bush_far_placeholder_uses_the_predeclared_loader_route_and_fallback_contract() {
    let loader = AssetLoader::new();
    let declaration = loader.declaration(AssetId::BushFar);

    assert_eq!(declaration.id.stable_id(), "moria.vegetation.bush_far");
    assert_eq!(declaration.path, BUSH_FAR_PATH);
    assert_eq!(loader.resolve_runtime_path(BUSH_FAR_PATH), Ok(declaration));
    assert_eq!(
        loader.missing_asset_action(AssetId::BushFar, RuntimeAssetProfile::Development),
        AssetMissingAction::DevelopmentFallback {
            warning: "moria.vegetation.bush_far",
        }
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::BushFar, RuntimeAssetProfile::Release),
        AssetMissingAction::Fatal
    );
}

#[test]
fn bush_far_placeholder_is_a_support_anchored_shared_material_binary_gltf() {
    let bytes = fs::read(asset_path()).expect("the bush placeholder exists at its declared path");
    let json = glb_json(&bytes);

    assert_eq!(json["asset"]["version"], "2.0");
    assert_eq!(json["asset"]["extras"]["units"], "metres");
    assert_eq!(json["asset"]["extras"]["up_axis"], "Y");
    assert_eq!(json["asset"]["extras"]["forward_axis"], "+Z");
    assert_eq!(json["asset"]["extras"]["origin"], "support_center");

    let materials = json["materials"].as_array().expect("materials array");
    assert_eq!(
        materials.len(),
        1,
        "the placeholder has one shared material"
    );
    let primitive = &json["meshes"][0]["primitives"][0];
    assert_eq!(primitive["material"], 0);
    assert_eq!(primitive["mode"], 4, "primitive is triangles");
    assert_eq!(primitive["attributes"]["NORMAL"], 1);
    assert_eq!(primitive["attributes"]["TEXCOORD_0"], 2);

    let accessors = json["accessors"].as_array().expect("accessors array");
    let positions = &accessors[primitive["attributes"]["POSITION"]
        .as_u64()
        .expect("position accessor") as usize];
    assert_eq!(positions["min"], serde_json::json!([-1.2, 0, -1.2]));
    assert_eq!(positions["max"], serde_json::json!([1.2, 1.2, 1.2]));
    let index_count =
        accessors[primitive["indices"].as_u64().expect("index accessor") as usize]["count"]
            .as_u64()
            .expect("index count");
    assert_eq!(index_count % 3, 0, "indices define triangles");
    assert_eq!(
        index_count / 3,
        8,
        "placeholder retains its eight-triangle budget"
    );
    assert!(index_count / 3 <= 250, "bush far triangle budget");
}

fn asset_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("assets")
        .join(BUSH_FAR_PATH)
}

fn glb_json(bytes: &[u8]) -> Value {
    assert!(bytes.len() >= 28, "GLB header and chunks are required");
    assert_eq!(u32_at(bytes, 0), GLB_MAGIC);
    assert_eq!(u32_at(bytes, 4), GLB_VERSION);
    assert_eq!(u32_at(bytes, 8) as usize, bytes.len());
    let json_length = u32_at(bytes, 12) as usize;
    assert_eq!(u32_at(bytes, 16), JSON_CHUNK);
    let binary_offset = 20 + json_length;
    assert!(
        bytes.len() >= binary_offset + 8,
        "binary data chunk is required"
    );
    let binary_length = u32_at(bytes, binary_offset) as usize;
    assert_eq!(u32_at(bytes, binary_offset + 4), BIN_CHUNK);
    assert_eq!(binary_offset + 8 + binary_length, bytes.len());
    serde_json::from_slice(&bytes[20..binary_offset]).expect("the GLB JSON chunk must be valid")
}

fn u32_at(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(
        bytes[offset..offset + 4]
            .try_into()
            .expect("four-byte field"),
    )
}
