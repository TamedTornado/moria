use std::{fs, path::PathBuf};

use moria_world::presentation::{AssetId, AssetLoader, AssetMissingAction, RuntimeAssetProfile};
use serde_json::Value;

const BUSH_NEAR_PATH: &str = "vegetation/bush_near.glb";
const GLB_MAGIC: u32 = 0x4654_6C67;
const GLB_VERSION: u32 = 2;
const JSON_CHUNK: u32 = 0x4E4F_534A;
const BIN_CHUNK: u32 = 0x004E_4942;

#[test]
fn bush_near_placeholder_is_a_shared_material_support_centered_glb() {
    let loader = AssetLoader::new();
    let declaration = loader.declaration(AssetId::BushNear);
    assert_eq!(declaration.id.stable_id(), "moria.vegetation.bush_near");
    assert_eq!(declaration.path, BUSH_NEAR_PATH);
    assert_eq!(
        loader.resolve_runtime_path(BUSH_NEAR_PATH),
        Ok(declaration),
        "the fixture must exercise the immutable shared loader path"
    );
    assert_eq!(
        loader.validation_fixture(AssetId::BushNear).key,
        AssetId::BushNear.stable_id()
    );
    assert!(matches!(
        loader.missing_asset_action(AssetId::BushNear, RuntimeAssetProfile::Development),
        AssetMissingAction::DevelopmentFallback { warning }
            if warning == AssetId::BushNear.stable_id()
    ));
    assert_eq!(
        loader.missing_asset_action(AssetId::BushNear, RuntimeAssetProfile::Release),
        AssetMissingAction::Fatal
    );

    let bytes = fs::read(asset_path()).expect("bush-near placeholder exists at its declared path");
    let document = glb_json(&bytes);

    assert_eq!(document["asset"]["version"], "2.0");
    assert_eq!(document["asset"]["extras"]["units"], "metres");
    assert_eq!(document["asset"]["extras"]["up_axis"], "Y");
    assert_eq!(document["asset"]["extras"]["forward_axis"], "+Z");
    assert_eq!(document["asset"]["extras"]["origin"], "support_center");

    let materials = document["materials"].as_array().expect("materials array");
    assert_eq!(materials.len(), 1, "the bush uses one shared material");

    let meshes = document["meshes"].as_array().expect("meshes array");
    assert_eq!(meshes.len(), 1, "the placeholder is one shared mesh");
    assert_eq!(meshes[0]["name"], "BushNear");
    let primitive = &meshes[0]["primitives"].as_array().expect("mesh primitives")[0];
    assert_eq!(primitive["material"], 0);
    assert!(
        primitive.get("indices").is_some(),
        "bush uses indexed triangles"
    );
    for attribute in ["POSITION", "NORMAL", "TEXCOORD_0", "TANGENT"] {
        assert!(
            primitive["attributes"].get(attribute).is_some(),
            "bush supplies {attribute}"
        );
    }

    let accessors = document["accessors"].as_array().expect("accessors array");
    let index_accessor = primitive["indices"].as_u64().expect("index accessor") as usize;
    let index_count = accessors[index_accessor]["count"]
        .as_u64()
        .expect("index count");
    assert_eq!(index_count % 3, 0, "indexed triangles");
    assert!(index_count / 3 <= 2_000, "bush remains within its budget");

    let position_accessor = primitive["attributes"]["POSITION"]
        .as_u64()
        .expect("position accessor") as usize;
    let position = &accessors[position_accessor];
    assert_eq!(numeric_array(&position["min"]), Some(vec![-1.2, 0.0, -1.2]));
    assert_eq!(numeric_array(&position["max"]), Some(vec![1.2, 2.4, 1.2]));
}

fn asset_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("assets")
        .join(BUSH_NEAR_PATH)
}

fn numeric_array(value: &Value) -> Option<Vec<f64>> {
    value.as_array()?.iter().map(Value::as_f64).collect()
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
