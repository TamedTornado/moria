use std::fs;

use moria_world::presentation::{AssetId, AssetLoader, AssetMissingAction, RuntimeAssetProfile};
use serde_json::Value;

const GRASS_CLUSTER_PATH: &str = "vegetation/grass_cluster.glb";
const GLB_MAGIC: u32 = 0x4654_6C67;
const GLB_VERSION: u32 = 2;
const JSON_CHUNK: u32 = 0x4E4F_534A;
const BIN_CHUNK: u32 = 0x004E_4942;

#[test]
fn grass_cluster_placeholder_is_a_shared_material_support_anchored_glb() {
    let loader = AssetLoader::new();
    let declaration = loader.declaration(AssetId::GrassCluster);
    assert_eq!(declaration.path, GRASS_CLUSTER_PATH);
    assert_eq!(
        loader.resolve_runtime_path(GRASS_CLUSTER_PATH),
        Ok(declaration)
    );
    assert!(matches!(
        loader.missing_asset_action(AssetId::GrassCluster, RuntimeAssetProfile::Development),
        AssetMissingAction::DevelopmentFallback { warning }
            if warning == AssetId::GrassCluster.stable_id()
    ));

    let asset_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../assets/vegetation/grass_cluster.glb"
    );
    let bytes = fs::read(asset_path)
        .expect("grass-cluster placeholder exists at its declared runtime path");
    let document = glb_json(&bytes);

    assert_eq!(document["asset"]["version"], "2.0");
    assert_eq!(document["asset"]["extras"]["units"], "metres");
    assert_eq!(document["asset"]["extras"]["up_axis"], "Y");
    assert_eq!(document["asset"]["extras"]["forward_axis"], "+Z");
    assert_eq!(document["asset"]["extras"]["origin"], "support_center");

    let materials = document["materials"].as_array().expect("materials array");
    assert_eq!(materials.len(), 1, "the cluster uses one shared material");

    let meshes = document["meshes"].as_array().expect("meshes array");
    assert_eq!(meshes.len(), 1, "the cluster is one shared batched mesh");
    let primitive = &meshes[0]["primitives"].as_array().expect("mesh primitives")[0];
    assert_eq!(primitive["material"], 0);
    assert!(
        primitive.get("indices").is_some(),
        "grass uses indexed triangles"
    );
    for attribute in ["POSITION", "NORMAL", "TEXCOORD_0", "TANGENT"] {
        assert!(
            primitive["attributes"].get(attribute).is_some(),
            "grass supplies {attribute}"
        );
    }

    let accessors = document["accessors"].as_array().expect("accessors array");
    let index_accessor = primitive["indices"].as_u64().expect("index accessor") as usize;
    let index_count = accessors[index_accessor]["count"]
        .as_u64()
        .expect("index count");
    assert_eq!(index_count % 3, 0, "indexed triangles");
    let position_accessor = primitive["attributes"]["POSITION"]
        .as_u64()
        .expect("position accessor") as usize;
    let position = &accessors[position_accessor];
    assert_eq!(position["min"][1].as_f64(), Some(0.0));
    assert!(position["max"][1].as_f64().unwrap() > 0.0);
    assert!(position["min"][0].as_f64().unwrap() < 0.0);
    assert!(position["max"][0].as_f64().unwrap() > 0.0);
    assert!(position["min"][2].as_f64().unwrap() < 0.0);
    assert!(position["max"][2].as_f64().unwrap() > 0.0);
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
