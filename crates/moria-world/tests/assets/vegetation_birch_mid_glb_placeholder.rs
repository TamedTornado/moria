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
    assert_eq!(declaration.id.stable_id(), "moria.vegetation.birch_mid");
    assert_eq!(declaration.path, BIRCH_MID_PATH);
    assert_eq!(
        loader.resolve_runtime_path(BIRCH_MID_PATH),
        Ok(declaration),
        "the fixture must exercise the immutable shared loader path"
    );
    assert_eq!(
        loader.validation_fixture(AssetId::BirchMid).key,
        AssetId::BirchMid.stable_id()
    );
    assert!(matches!(
        loader.missing_asset_action(AssetId::BirchMid, RuntimeAssetProfile::Development),
        AssetMissingAction::DevelopmentFallback { warning }
            if warning == AssetId::BirchMid.stable_id()
    ));
    assert_eq!(
        loader.missing_asset_action(AssetId::BirchMid, RuntimeAssetProfile::Release),
        AssetMissingAction::Fatal
    );

    let bytes = fs::read(asset_path()).expect("birch-mid placeholder exists at its declared path");
    let document = glb_json(&bytes);

    assert_eq!(document["asset"]["version"], "2.0");
    assert_eq!(document["asset"]["extras"]["units"], "metres");
    assert_eq!(document["asset"]["extras"]["up_axis"], "Y");
    assert_eq!(document["asset"]["extras"]["forward_axis"], "+Z");
    assert_eq!(document["asset"]["extras"]["origin"], "support_center");

    let materials = document["materials"].as_array().expect("materials array");
    assert_eq!(materials.len(), 1, "the mid birch uses one shared material");

    let meshes = document["meshes"].as_array().expect("meshes array");
    assert_eq!(meshes.len(), 1, "the placeholder is one shared mesh");
    assert_eq!(meshes[0]["name"], "BirchMid");
    let primitive = &meshes[0]["primitives"].as_array().expect("mesh primitives")[0];
    assert_eq!(primitive["material"], 0);
    assert!(
        primitive.get("indices").is_some(),
        "birch uses indexed triangles"
    );
    for attribute in ["POSITION", "NORMAL", "TEXCOORD_0", "TANGENT"] {
        assert!(
            primitive["attributes"].get(attribute).is_some(),
            "birch supplies {attribute}"
        );
    }

    let accessors = document["accessors"].as_array().expect("accessors array");
    let index_accessor = primitive["indices"].as_u64().expect("index accessor") as usize;
    let index_count = accessors[index_accessor]["count"]
        .as_u64()
        .expect("index count");
    assert_eq!(index_count % 3, 0, "indexed triangles");
    assert!(
        index_count / 3 <= 3_000,
        "mid birch remains within its budget"
    );

    let position_accessor = primitive["attributes"]["POSITION"]
        .as_u64()
        .expect("position accessor") as usize;
    let position = &accessors[position_accessor];
    assert_eq!(
        position["min"]
            .as_array()
            .expect("position minimum")
            .iter()
            .map(Value::as_f64)
            .collect::<Option<Vec<_>>>(),
        Some(vec![-4.0, 0.0, -4.0])
    );
    assert_eq!(
        position["max"]
            .as_array()
            .expect("position maximum")
            .iter()
            .map(Value::as_f64)
            .collect::<Option<Vec<_>>>(),
        Some(vec![4.0, 14.0, 4.0])
    );
}

fn asset_path() -> String {
    format!(
        "{}/../../assets/{BIRCH_MID_PATH}",
        env!("CARGO_MANIFEST_DIR")
    )
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
