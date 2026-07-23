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
    assert_eq!(
        loader.resolve_runtime_path(PINE_NEAR_PATH),
        Ok(declaration),
        "the fixture must exercise the immutable shared loader path"
    );
    assert_eq!(
        loader.validation_fixture(AssetId::PineNear).key,
        AssetId::PineNear.stable_id()
    );
    assert!(matches!(
        loader.missing_asset_action(AssetId::PineNear, RuntimeAssetProfile::Development),
        AssetMissingAction::DevelopmentFallback { warning }
            if warning == AssetId::PineNear.stable_id()
    ));
    assert_eq!(
        loader.missing_asset_action(AssetId::PineNear, RuntimeAssetProfile::Release),
        AssetMissingAction::Fatal
    );

    let bytes = fs::read(asset_path()).expect("pine-near placeholder exists at its declared path");
    let (document, binary_length) = glb_document(&bytes);

    assert_eq!(document["asset"]["version"], "2.0");
    assert_eq!(
        document["asset"]["generator"],
        "moria procedural pine-near placeholder"
    );
    assert_eq!(document["asset"]["extras"]["units"], "metres");
    assert_eq!(document["asset"]["extras"]["up_axis"], "Y");
    assert_eq!(document["asset"]["extras"]["forward_axis"], "+Z");
    assert_eq!(document["asset"]["extras"]["origin"], "support_center");

    let materials = document["materials"].as_array().expect("materials array");
    assert_eq!(materials.len(), 1, "the near pine uses one shared material");

    let meshes = document["meshes"].as_array().expect("meshes array");
    assert_eq!(meshes.len(), 1, "the placeholder is one shared mesh");
    assert_eq!(meshes[0]["name"], "PineNear");
    let primitive = &meshes[0]["primitives"].as_array().expect("mesh primitives")[0];
    assert_eq!(primitive["material"], 0);
    assert!(
        primitive.get("indices").is_some(),
        "pine uses indexed triangles"
    );
    for attribute in ["POSITION", "NORMAL", "TEXCOORD_0", "TANGENT"] {
        assert!(
            primitive["attributes"].get(attribute).is_some(),
            "pine supplies {attribute}"
        );
    }

    let accessors = document["accessors"].as_array().expect("accessors array");
    let buffer_views = document["bufferViews"]
        .as_array()
        .expect("buffer views array");
    for accessor in accessors {
        let buffer_view = &buffer_views[accessor["bufferView"]
            .as_u64()
            .expect("accessor buffer view") as usize];
        let component_bytes = match accessor["componentType"]
            .as_u64()
            .expect("accessor component type")
        {
            5123 => 2,
            5126 => 4,
            component_type => panic!("unsupported component type {component_type}"),
        };
        let component_count = match accessor["type"].as_str().expect("accessor type") {
            "SCALAR" => 1,
            "VEC2" => 2,
            "VEC3" => 3,
            "VEC4" => 4,
            accessor_type => panic!("unsupported accessor type {accessor_type}"),
        };
        let accessor_bytes =
            accessor["count"].as_u64().expect("accessor count") * component_bytes * component_count;
        let accessor_offset = accessor
            .get("byteOffset")
            .and_then(Value::as_u64)
            .unwrap_or(0);
        assert!(
            accessor_offset + accessor_bytes
                <= buffer_view["byteLength"].as_u64().expect("view length"),
            "every accessor must fit inside its buffer view"
        );
        assert!(
            buffer_view
                .get("byteOffset")
                .and_then(Value::as_u64)
                .unwrap_or(0)
                + buffer_view["byteLength"].as_u64().expect("view length")
                <= binary_length as u64,
            "every buffer view must fit inside the GLB binary chunk"
        );
    }
    let index_accessor = primitive["indices"].as_u64().expect("index accessor") as usize;
    let index_count = accessors[index_accessor]["count"]
        .as_u64()
        .expect("index count");
    assert_eq!(index_count % 3, 0, "indexed triangles");
    assert!(
        index_count / 3 <= 12_000,
        "near pine remains within its budget"
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
        Some(vec![4.0, 18.0, 4.0])
    );
}

fn asset_path() -> String {
    format!(
        "{}/../../assets/{PINE_NEAR_PATH}",
        env!("CARGO_MANIFEST_DIR")
    )
}

fn glb_document(bytes: &[u8]) -> (Value, usize) {
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
    (
        serde_json::from_slice(&bytes[20..binary_offset]).expect("valid GLB JSON chunk"),
        binary_length,
    )
}

fn u32_at(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(
        bytes[offset..offset + 4]
            .try_into()
            .expect("four-byte field"),
    )
}
