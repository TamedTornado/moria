use std::fs;

use moria_world::presentation::{AssetId, AssetLoader, AssetMissingAction, RuntimeAssetProfile};
use serde_json::Value;

const BIRCH_MID_PATH: &str = "vegetation/birch_mid.glb";
const GLB_MAGIC: u32 = 0x4654_6C67;
const GLB_VERSION: u32 = 2;
const JSON_CHUNK: u32 = 0x4E4F_534A;
const BIN_CHUNK: u32 = 0x004E_4942;

#[test]
fn birch_mid_placeholder_is_a_shared_material_support_centered_glb() {
    let loader = AssetLoader::new();
    let declaration = loader.declaration(AssetId::BirchMid);
    assert_eq!(declaration.path, BIRCH_MID_PATH);
    assert_eq!(loader.resolve_runtime_path(BIRCH_MID_PATH), Ok(declaration));
    assert!(matches!(
        loader.missing_asset_action(AssetId::BirchMid, RuntimeAssetProfile::Development),
        AssetMissingAction::DevelopmentFallback { warning } if warning == AssetId::BirchMid.stable_id()
    ));

    let asset_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../assets/vegetation/birch_mid.glb"
    );
    let bytes =
        fs::read(asset_path).expect("birch mid placeholder exists at its declared runtime path");
    let document = glb_json(&bytes);

    assert_eq!(document["asset"]["version"], "2.0");
    assert_eq!(document["asset"]["extras"]["units"], "metres");
    assert_eq!(document["asset"]["extras"]["up_axis"], "Y");
    assert_eq!(document["asset"]["extras"]["forward_axis"], "+Z");
    assert_eq!(document["asset"]["extras"]["origin"], "support_center");

    let materials = document["materials"].as_array().expect("materials array");
    assert_eq!(
        materials.len(),
        1,
        "the shared birch material has one declaration"
    );

    let meshes = document["meshes"].as_array().expect("meshes array");
    assert_eq!(meshes.len(), 1, "mid LOD has one shared mesh");
    assert_eq!(meshes[0]["name"], "BirchMid");

    let primitive = &meshes[0]["primitives"].as_array().expect("mesh primitives")[0];
    assert_eq!(primitive["material"], 0);
    assert!(primitive["attributes"]["POSITION"].is_u64());
    assert!(primitive["attributes"]["NORMAL"].is_u64());
    assert!(primitive["attributes"]["TEXCOORD_0"].is_u64());

    let accessors = document["accessors"].as_array().expect("accessors array");
    let index_accessor = primitive["indices"].as_u64().expect("index accessor") as usize;
    let index_count = accessors[index_accessor]["count"]
        .as_u64()
        .expect("index count") as usize;
    assert_eq!(index_count % 3, 0, "indexed triangles");
    assert!(index_count / 3 <= 3_000);

    let position_accessor = primitive["attributes"]["POSITION"]
        .as_u64()
        .expect("position accessor") as usize;
    let position = &accessors[position_accessor];
    assert_eq!(number_array(&position["min"]), [-4.0, 0.0, -4.0]);
    assert_eq!(number_array(&position["max"]), [4.0, 14.0, 4.0]);
}

fn number_array(value: &Value) -> [f64; 3] {
    let values = value.as_array().expect("three-dimensional numeric array");
    assert_eq!(values.len(), 3, "three array elements");
    [
        values[0].as_f64().expect("finite numeric element"),
        values[1].as_f64().expect("finite numeric element"),
        values[2].as_f64().expect("finite numeric element"),
    ]
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
