use std::{collections::BTreeSet, fs};

use moria_world::presentation::{AssetId, AssetLoader, AssetMissingAction, RuntimeAssetProfile};
use serde_json::{Value, json};

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

    assert_scene_reaches_skin_and_animation_nodes(&document);

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

#[test]
fn skin_hierarchy_allows_a_common_scene_root_with_skeleton_and_mesh_siblings() {
    assert_scene_reaches_skin_and_animation_nodes(&scene_graph_document(
        &[0],
        &[&[1, 2], &[3], &[], &[]],
        &[1, 3],
        2,
        &[3],
    ));
}

#[test]
#[should_panic(expected = "skin joint 3 is reachable from the default scene")]
fn skin_hierarchy_rejects_an_unreachable_skin_joint() {
    assert_scene_reaches_skin_and_animation_nodes(&scene_graph_document(
        &[0],
        &[&[1, 2], &[], &[], &[]],
        &[1, 3],
        2,
        &[1],
    ));
}

#[test]
#[should_panic(expected = "animation target 4 is reachable from the default scene")]
fn skin_hierarchy_rejects_an_unreachable_animation_target() {
    assert_scene_reaches_skin_and_animation_nodes(&scene_graph_document(
        &[0],
        &[&[1, 2], &[3], &[], &[], &[]],
        &[1, 3],
        2,
        &[4],
    ));
}

fn scene_graph_document(
    roots: &[u64],
    children: &[&[u64]],
    joints: &[u64],
    skinned_mesh: usize,
    animation_targets: &[u64],
) -> Value {
    let nodes: Vec<_> = children
        .iter()
        .enumerate()
        .map(|(index, children)| {
            let mut node = json!({ "children": children });
            if index == skinned_mesh {
                node["mesh"] = json!(0);
                node["skin"] = json!(0);
            }
            node
        })
        .collect();
    let channels: Vec<_> = animation_targets
        .iter()
        .map(|target| json!({ "target": { "node": target } }))
        .collect();

    json!({
        "scene": 0,
        "scenes": [{ "nodes": roots }],
        "nodes": nodes,
        "skins": [{ "skeleton": 1, "joints": joints }],
        "animations": [{ "channels": channels }],
    })
}

fn assert_scene_reaches_skin_and_animation_nodes(document: &Value) {
    let nodes = document["nodes"].as_array().expect("explorer nodes");
    let reachable = reachable_scene_nodes(document);
    let skins = document["skins"].as_array().expect("explorer skins");

    for (skin_index, skin) in skins.iter().enumerate() {
        let skeleton = skin["skeleton"].as_u64().expect("skeleton root");
        assert!(
            reachable.contains(&skeleton),
            "skeleton root {skeleton} is reachable from the default scene"
        );
        for joint in skin["joints"].as_array().expect("skin joints") {
            let joint = joint.as_u64().expect("skin joint node");
            assert!(
                reachable.contains(&joint),
                "skin joint {joint} is reachable from the default scene"
            );
        }

        let skin_index = u64::try_from(skin_index).expect("skin index fits in u64");
        let skinned_mesh_nodes: Vec<_> = nodes
            .iter()
            .enumerate()
            .filter_map(|(node_index, node)| {
                (node["skin"].as_u64() == Some(skin_index)).then_some((node_index, node))
            })
            .collect();
        assert!(
            !skinned_mesh_nodes.is_empty(),
            "skin {skin_index} has a skinned mesh node"
        );
        for (node_index, node) in skinned_mesh_nodes {
            assert!(node["mesh"].is_number(), "skinned node has a mesh");
            let node_index = u64::try_from(node_index).expect("node index fits in u64");
            assert!(
                reachable.contains(&node_index),
                "skinned mesh node {node_index} is reachable from the default scene"
            );
        }
    }

    for animation in document["animations"].as_array().expect("animations") {
        for channel in animation["channels"].as_array().expect("animation channels") {
            let target = channel["target"]["node"]
                .as_u64()
                .expect("animation target node");
            assert!(
                reachable.contains(&target),
                "animation target {target} is reachable from the default scene"
            );
        }
    }
}

fn reachable_scene_nodes(document: &Value) -> BTreeSet<u64> {
    let nodes = document["nodes"].as_array().expect("explorer nodes");
    let scene_index = document["scene"].as_u64().expect("default scene") as usize;
    let roots = document["scenes"].as_array().expect("scenes array")[scene_index]["nodes"]
        .as_array()
        .expect("default scene roots");
    let mut reachable = BTreeSet::new();
    let mut pending: Vec<_> = roots
        .iter()
        .map(|node| node.as_u64().expect("scene root node"))
        .collect();

    while let Some(node_index) = pending.pop() {
        if !reachable.insert(node_index) {
            continue;
        }
        let node_index = usize::try_from(node_index).expect("node index fits in usize");
        let node = nodes.get(node_index).expect("scene node index is valid");
        if let Some(children) = node["children"].as_array() {
            pending.extend(
                children
                    .iter()
                    .map(|child| child.as_u64().expect("scene child node")),
            );
        }
    }

    reachable
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
