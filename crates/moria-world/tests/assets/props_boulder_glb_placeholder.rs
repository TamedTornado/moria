use std::fs;

use moria_world::presentation::{AssetId, AssetLoader, AssetMissingAction, RuntimeAssetProfile};
use serde_json::Value;

const BOULDER_PATH: &str = "props/boulder.glb";
const GLB_MAGIC: u32 = 0x4654_6C67;
const GLB_VERSION: u32 = 2;
const JSON_CHUNK: u32 = 0x4E4F_534A;
const BIN_CHUNK: u32 = 0x004E_4942;

#[test]
fn boulder_placeholder_is_a_shared_material_support_centered_glb() {
    let loader = AssetLoader::new();
    let declaration = loader.declaration(AssetId::Boulder);
    assert_eq!(declaration.path, BOULDER_PATH);
    assert_eq!(loader.resolve_runtime_path(BOULDER_PATH), Ok(declaration));
    assert!(matches!(
        loader.missing_asset_action(AssetId::Boulder, RuntimeAssetProfile::Development),
        AssetMissingAction::DevelopmentFallback { warning } if warning == AssetId::Boulder.stable_id()
    ));

    let asset_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../assets/props/boulder.glb"
    );
    let bytes =
        fs::read(asset_path).expect("boulder placeholder exists at its declared runtime path");
    let document = glb_json(&bytes);

    assert_eq!(document["asset"]["version"], "2.0");
    assert_eq!(document["asset"]["extras"]["units"], "metres");
    assert_eq!(document["asset"]["extras"]["up_axis"], "Y");
    assert_eq!(document["asset"]["extras"]["forward_axis"], "+Z");
    assert_eq!(document["asset"]["extras"]["origin"], "support_center");

    let materials = document["materials"].as_array().expect("materials array");
    assert_eq!(materials.len(), 1, "all variants share one material");

    let meshes = document["meshes"].as_array().expect("meshes array");
    let accessors = document["accessors"].as_array().expect("accessors array");
    assert_eq!(
        meshes.len(),
        2,
        "placeholder supplies named boulder variants"
    );
    for mesh in meshes {
        assert!(
            mesh["name"]
                .as_str()
                .is_some_and(|name| name.starts_with("BoulderVariant"))
        );
        let primitives = mesh["primitives"].as_array().expect("mesh primitives");
        let triangles: usize = primitives
            .iter()
            .map(|primitive| {
                assert_eq!(primitive["material"], 0);
                assert!(primitive["attributes"]["NORMAL"].is_u64());
                assert!(primitive["attributes"]["TEXCOORD_0"].is_u64());
                let index_accessor =
                    primitive["indices"].as_u64().expect("index accessor") as usize;
                let index_count = accessors[index_accessor]["count"]
                    .as_u64()
                    .expect("index count") as usize;
                assert_eq!(index_count % 3, 0, "indexed triangles");
                index_count / 3
            })
            .sum();
        assert!(triangles <= 3_000);
    }

    for mesh in meshes {
        for primitive in mesh["primitives"].as_array().expect("mesh primitives") {
            let position_accessor = primitive["attributes"]["POSITION"]
                .as_u64()
                .expect("position accessor") as usize;
            let accessor = &accessors[position_accessor];
            let min = accessor["min"].as_array().expect("position min");
            let max = accessor["max"].as_array().expect("position max");
            assert!(min[0].as_f64().unwrap() >= -1.8 && max[0].as_f64().unwrap() <= 1.8);
            assert_eq!(min[1].as_f64(), Some(0.0));
            assert!(max[1].as_f64().unwrap() <= 3.6);
            assert!(min[2].as_f64().unwrap() >= -1.8 && max[2].as_f64().unwrap() <= 1.8);
        }
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
