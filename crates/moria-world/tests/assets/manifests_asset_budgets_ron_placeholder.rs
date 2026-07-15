use std::{fs, path::Path};

use moria_world::presentation::{AssetId, AssetLoader, AssetMissingAction, RuntimeAssetProfile};
use serde::Deserialize;

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
struct AssetBudgetRegistry {
    schema_version: u16,
    entries: Vec<AssetBudgetEntry>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
struct AssetBudgetEntry {
    stable_id: String,
    path: String,
    content_sha256: String,
    max_file_bytes: u64,
    contract: AssetBudgetContract,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
enum AssetBudgetContract {
    Ron {
        schema_key: String,
    },
    Glb {
        max_triangles_per_primitive: u32,
        required_named_primitives: Vec<String>,
        required_animation_clips: Vec<String>,
        bounds_min_q8: [i32; 3],
        bounds_max_q8: [i32; 3],
        support_origin_q8: [i32; 3],
    },
    Ktx2 {
        width: u32,
        height: u32,
        layers: u16,
        mip_count: u8,
        color_space: TextureColorSpace,
        basis_payload: bool,
    },
    Wgsl {
        entry_points: Vec<String>,
        forbids_i64_atomics: bool,
    },
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
enum TextureColorSpace {
    Srgb,
    Linear,
}

#[test]
fn asset_budgets_placeholder_uses_the_declared_required_runtime_path() {
    let loader = AssetLoader::new();
    let declaration = loader.resolve_runtime_path("manifests/asset_budgets.ron");

    assert_eq!(declaration.unwrap().id, AssetId::AssetBudgets);
    assert_eq!(
        loader.missing_asset_action(AssetId::AssetBudgets, RuntimeAssetProfile::Development),
        AssetMissingAction::Fatal
    );

    let placeholder_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/manifests/asset_budgets.ron");
    let placeholder = fs::read_to_string(placeholder_path).unwrap();
    let registry: AssetBudgetRegistry = ron::de::from_str(&placeholder).unwrap();

    assert_eq!(registry.schema_version, 1);
    assert!(registry.entries.is_empty());
}
