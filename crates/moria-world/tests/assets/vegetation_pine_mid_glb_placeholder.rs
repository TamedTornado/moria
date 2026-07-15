use std::convert::TryInto;

use moria_world::presentation::{AssetId, AssetLoader, AssetMissingAction, RuntimeAssetProfile};
use serde_json::Value;

const PINE_MID_PATH: &str = "vegetation/pine_mid.glb";
const GLB_MAGIC: u32 = 0x4654_6C67;
const GLB_VERSION: u32 = 2;
const JSON_CHUNK_TYPE: u32 = 0x4E4F_534A;
const BIN_CHUNK_TYPE: u32 = 0x004E_4942;

#[test]
fn pine_mid_placeholder_is_a_shared_material_binary_gltf_at_its_declared_path() {
    let loader = AssetLoader::new();
    let declaration = loader.declaration(AssetId::PineMid);

    assert_eq!(declaration.id.stable_id(), "moria.vegetation.pine_mid");
    assert_eq!(declaration.path, PINE_MID_PATH);
    assert_eq!(loader.resolve_runtime_path(PINE_MID_PATH), Ok(declaration));
    assert_eq!(
        loader.validation_fixture(AssetId::PineMid).asset_id,
        AssetId::PineMid
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::PineMid, RuntimeAssetProfile::Development),
        AssetMissingAction::DevelopmentFallback {
            warning: "moria.vegetation.pine_mid",
        }
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::PineMid, RuntimeAssetProfile::Release),
        AssetMissingAction::Fatal
    );

    let asset = include_bytes!("../../../../assets/vegetation/pine_mid.glb");
    let (document, binary_chunk) = parse_glb(asset);

    assert_eq!(document["asset"]["version"], "2.0");
    assert_eq!(document["asset"]["extras"]["units"], "metres");
    assert_eq!(document["asset"]["extras"]["up_axis"], "Y");
    assert_eq!(document["asset"]["extras"]["forward_axis"], "+Z");
    assert_eq!(document["asset"]["extras"]["origin"], "support_center");
    assert_eq!(document["materials"].as_array().map(Vec::len), Some(1));
    assert_eq!(
        document["materials"][0]["name"],
        "PinePlaceholderSharedMaterial"
    );

    let primitives: Vec<&Value> = document["meshes"]
        .as_array()
        .expect("glTF includes meshes")
        .iter()
        .flat_map(|mesh| mesh["primitives"].as_array().into_iter().flatten())
        .collect();
    assert!(!primitives.is_empty());
    assert!(primitives.iter().all(|primitive| {
        primitive["mode"] == 4
            && primitive["indices"].is_u64()
            && primitive["material"] == 0
            && primitive["attributes"]["POSITION"].is_u64()
            && primitive["attributes"]["NORMAL"].is_u64()
            && primitive["attributes"]["TEXCOORD_0"].is_u64()
    }));

    let accessors = document["accessors"].as_array().expect("glTF accessors");
    let triangles: u64 = primitives
        .iter()
        .map(|primitive| {
            let position = primitive["attributes"]["POSITION"].as_u64().unwrap() as usize;
            let position_accessor = &accessors[position];
            assert_eq!(position_accessor["min"], serde_json::json!([-4, 0, -4]));
            assert_eq!(position_accessor["max"], serde_json::json!([4, 18, 4]));
            assert_eq!(position_accessor["type"], "VEC3");

            let index = primitive["indices"].as_u64().unwrap() as usize;
            accessors[index]["count"]
                .as_u64()
                .expect("index accessor count")
                / 3
        })
        .sum();
    assert!(triangles <= 3_000, "mid LOD has {triangles} triangles");
    assert!(!binary_chunk.is_empty());
}

fn parse_glb(bytes: &[u8]) -> (Value, &[u8]) {
    assert!(bytes.len() >= 20, "GLB contains a header and JSON chunk");
    assert_eq!(read_u32(bytes, 0), GLB_MAGIC);
    assert_eq!(read_u32(bytes, 4), GLB_VERSION);
    assert_eq!(read_u32(bytes, 8) as usize, bytes.len());

    let json_length = read_u32(bytes, 12) as usize;
    assert_eq!(read_u32(bytes, 16), JSON_CHUNK_TYPE);
    let json_start = 20;
    let json_end = json_start + json_length;
    let document = serde_json::from_slice(&bytes[json_start..json_end]).expect("valid glTF JSON");

    assert!(bytes.len() >= json_end + 8, "GLB contains a BIN chunk");
    let bin_length = read_u32(bytes, json_end) as usize;
    assert_eq!(read_u32(bytes, json_end + 4), BIN_CHUNK_TYPE);
    let bin_start = json_end + 8;
    let bin_end = bin_start + bin_length;
    assert_eq!(bin_end, bytes.len());

    (document, &bytes[bin_start..bin_end])
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().expect("four bytes"))
}
