use std::{fs, path::Path};

use bevy::prelude::*;
use moria_world::presentation::{
    AssetId, AssetValidationError, AssetValidationPlugin, AssetValidationStatus,
    RuntimeAssetProfile, WorldRenderAssets, validate_asset_directory,
};
use serde_json::Value;
use sha2::{Digest, Sha256};

#[test]
fn validation_plugin_installs_one_shared_render_asset_resource() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(AssetValidationPlugin::for_development());

    assert!(app.world().contains_resource::<WorldRenderAssets>());
    assert!(matches!(
        app.world().resource::<AssetValidationStatus>(),
        AssetValidationStatus::Failed { errors }
            if errors.contains(&AssetValidationError::RegistryInventory { registry: "licenses" })
    ));
}

#[test]
fn repeated_placements_clone_the_same_shared_handles() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(AssetValidationPlugin::for_development());

    let assets = app.world().resource::<WorldRenderAssets>();
    let expected = assets.object_handles(AssetId::BirchNear).unwrap();
    for _ in 0..1_000 {
        assert_eq!(assets.object_handles(AssetId::BirchNear), Some(expected));
    }
}

#[test]
fn validation_rejects_glb_with_an_invalid_declared_attribute_reference() {
    let directory = temporary_asset_directory("invalid-glb-reference");
    fs::create_dir_all(directory.join("manifests")).unwrap();
    fs::create_dir_all(directory.join("vegetation")).unwrap();

    let mut glb = fs::read(source_assets().join("vegetation/pine_near.glb")).unwrap();
    let json_length = u32::from_le_bytes(glb[12..16].try_into().unwrap()) as usize;
    let binary_start = 20 + json_length + 8;
    let mut document: Value = serde_json::from_slice(&glb[20..20 + json_length]).unwrap();
    document["meshes"][0]["primitives"][0]["attributes"]["COLOR_0"] = Value::from(999);
    glb = glb_bytes(document, glb[binary_start..].to_vec());
    fs::write(directory.join("vegetation/pine_near.glb"), &glb).unwrap();

    let digest = hex_digest(&glb);
    let licenses = format!(
        "(schema_version: 1, entries: [(stable_id: \"moria.vegetation.pine_near\", path: \"vegetation/pine_near.glb\", content_sha256: \"{digest}\", provenance: InHouseGenerated(generator_or_tool: \"test\", author: \"test\", source_path: None, modifications: []))])"
    );
    fs::write(directory.join("manifests/asset_licenses.ron"), licenses).unwrap();
    let budgets = format!(
        "(schema_version: 1, entries: [(stable_id: \"moria.vegetation.pine_near\", path: \"vegetation/pine_near.glb\", content_sha256: \"{digest}\", max_file_bytes: 10000, contract: Glb(max_triangles_per_primitive: 12000, required_named_primitives: [\"PineNear\"], required_animation_clips: [], bounds_min_q8: (-1024, 0, -1024), bounds_max_q8: (1024, 4608, 1024), support_origin_q8: (0, 0, 0)))])"
    );
    fs::write(directory.join("manifests/asset_budgets.ron"), budgets).unwrap();

    let errors =
        validate_asset_directory(&directory, RuntimeAssetProfile::Development).unwrap_err();
    assert!(
        errors.contains(&AssetValidationError::Format {
            stable_id: "moria.vegetation.pine_near".to_owned(),
        }),
        "{errors:?}"
    );

    fs::remove_dir_all(directory).unwrap();
}

fn source_assets() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets")
}

fn temporary_asset_directory(name: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!("moria-world-{name}-{}", std::process::id()))
}

fn glb_bytes(document: Value, binary: Vec<u8>) -> Vec<u8> {
    let mut json = serde_json::to_vec(&document).unwrap();
    while !json.len().is_multiple_of(4) {
        json.push(b' ');
    }
    let mut bytes = Vec::with_capacity(20 + json.len() + 8 + binary.len());
    bytes.extend_from_slice(b"glTF");
    bytes.extend_from_slice(&2_u32.to_le_bytes());
    bytes.extend_from_slice(&0_u32.to_le_bytes());
    bytes.extend_from_slice(&(json.len() as u32).to_le_bytes());
    bytes.extend_from_slice(b"JSON");
    bytes.extend_from_slice(&json);
    bytes.extend_from_slice(&(binary.len() as u32).to_le_bytes());
    bytes.extend_from_slice(b"BIN\0");
    bytes.extend_from_slice(&binary);
    let length = bytes.len() as u32;
    bytes[8..12].copy_from_slice(&length.to_le_bytes());
    bytes
}

fn hex_digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
