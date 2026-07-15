use std::fs;

use moria_world::presentation::{AssetId, AssetLoader, AssetMissingAction, RuntimeAssetProfile};
use serde_json::Value;

const PINE_NEAR_PATH: &str = "vegetation/pine_near.glb";
const GLB_MAGIC: u32 = 0x4654_6C67;
const GLB_VERSION: u32 = 2;
const JSON_CHUNK: u32 = 0x4E4F_534A;
const BIN_CHUNK: u32 = 0x004E_4942;

#[test]
fn pine_near_placeholder_is_a_shared_material_support_centered_glb() {
    let loader = AssetLoader::new();
    let declaration = loader.declaration(AssetId::PineNear);

    assert_eq!(declaration.id.stable_id(), "moria.vegetation.pine_near");
    assert_eq!(declaration.path, PINE_NEAR_PATH);
    assert_eq!(loader.resolve_runtime_path(PINE_NEAR_PATH), Ok(declaration));
    assert_eq!(
        loader.validation_fixture(AssetId::PineNear).key,
        AssetId::PineNear.stable_id()
    );
    assert!(matches!(
        loader.missing_asset_action(AssetId::PineNear, RuntimeAssetProfile::Development),
        AssetMissingAction::DevelopmentFallback { warning } if warning == AssetId::PineNear.stable_id()
    ));
    assert_eq!(
        loader.missing_asset_action(AssetId::PineNear, RuntimeAssetProfile::Release),
        AssetMissingAction::Fatal
    );

    let asset_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../assets/vegetation/pine_near.glb"
    );
    let bytes =
        fs::read(asset_path).expect("pine-near placeholder exists at its declared runtime path");
    let document = glb_json(&bytes);

    assert_eq!(document["asset"]["version"], "2.0");
    assert_eq!(document["asset"]["extras"]["units"], "metres");
    assert_eq!(document["asset"]["extras"]["up_axis"], "Y");
    assert_eq!(document["asset"]["extras"]["forward_axis"], "+Z");
    assert_eq!(document["asset"]["extras"]["origin"], "support_center");

    let materials = document["materials"].as_array().expect("materials array");
    assert_eq!(materials.len(), 1, "trunk and canopy share one material");

    let meshes = document["meshes"].as_array().expect("meshes array");
    assert_eq!(meshes.len(), 1, "near pine has one shared mesh");
    assert_eq!(meshes[0]["name"], "PineNear");

    let accessors = document["accessors"].as_array().expect("accessors array");
    let primitives = meshes[0]["primitives"].as_array().expect("mesh primitives");
    let triangles: usize = primitives
        .iter()
        .map(|primitive| {
            assert_eq!(primitive["material"], 0);
            assert!(primitive["attributes"].get("POSITION").is_some());
            assert!(primitive["attributes"].get("NORMAL").is_some());
            assert!(primitive["attributes"].get("TEXCOORD_0").is_some());
            let index_accessor = primitive["indices"].as_u64().expect("index accessor") as usize;
            let index_count = accessors[index_accessor]["count"]
                .as_u64()
                .expect("index count") as usize;
            assert_eq!(index_count % 3, 0, "indexed triangles");
            index_count / 3
        })
        .sum();
    assert!(triangles <= 12_000);

    let position_accessors: Vec<&Value> = primitives
        .iter()
        .map(|primitive| {
            let index = primitive["attributes"]["POSITION"]
                .as_u64()
                .expect("position accessor") as usize;
            &accessors[index]
        })
        .collect();
    let min_x = position_accessors
        .iter()
        .map(|accessor| accessor["min"][0].as_f64().unwrap())
        .fold(f64::INFINITY, f64::min);
    let min_y = position_accessors
        .iter()
        .map(|accessor| accessor["min"][1].as_f64().unwrap())
        .fold(f64::INFINITY, f64::min);
    let min_z = position_accessors
        .iter()
        .map(|accessor| accessor["min"][2].as_f64().unwrap())
        .fold(f64::INFINITY, f64::min);
    let max_x = position_accessors
        .iter()
        .map(|accessor| accessor["max"][0].as_f64().unwrap())
        .fold(f64::NEG_INFINITY, f64::max);
    let max_y = position_accessors
        .iter()
        .map(|accessor| accessor["max"][1].as_f64().unwrap())
        .fold(f64::NEG_INFINITY, f64::max);
    let max_z = position_accessors
        .iter()
        .map(|accessor| accessor["max"][2].as_f64().unwrap())
        .fold(f64::NEG_INFINITY, f64::max);

    assert_eq!([min_x, min_y, min_z], [-4.0, 0.0, -4.0]);
    assert_eq!([max_x, max_y, max_z], [4.0, 18.0, 4.0]);
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
