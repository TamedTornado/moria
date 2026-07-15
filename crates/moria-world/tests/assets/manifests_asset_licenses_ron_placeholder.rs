use std::{fs, path::Path};

use moria_world::presentation::{AssetId, AssetLoader, AssetMissingAction, RuntimeAssetProfile};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct AssetLicenseRegistry {
    schema_version: u16,
    entries: Vec<AssetLicenseEntry>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct AssetLicenseEntry {
    stable_id: String,
    path: String,
    content_sha256: String,
    provenance: AssetProvenance,
}

#[derive(Debug, Deserialize, PartialEq)]
enum AssetProvenance {
    InHouseGenerated {
        generator_or_tool: String,
        author: String,
        source_path: Option<String>,
        modifications: Vec<String>,
    },
    External {
        source_url: String,
        author: String,
        license_spdx: String,
        license_text_path: String,
        modifications: Vec<String>,
    },
}

#[test]
fn asset_licenses_placeholder_uses_the_declared_schema_and_runtime_path() {
    let loader = AssetLoader::new();
    let declaration = loader.declaration(AssetId::AssetLicenses);

    assert_eq!(declaration.id.stable_id(), "moria.manifests.asset_licenses");
    assert_eq!(declaration.path, "manifests/asset_licenses.ron");
    assert_eq!(
        loader.resolve_runtime_path(declaration.path),
        Ok(declaration)
    );
    assert_eq!(
        loader.validation_fixture(AssetId::AssetLicenses).key,
        "moria.manifests.asset_licenses"
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::AssetLicenses, RuntimeAssetProfile::Development),
        AssetMissingAction::Fatal
    );

    let asset_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets")
        .join(declaration.path);
    let registry: AssetLicenseRegistry = ron::from_str(
        &fs::read_to_string(asset_path).expect("declared registry placeholder exists"),
    )
    .expect("placeholder uses the canonical license registry schema");

    assert_eq!(registry.schema_version, 1);
    assert!(registry.entries.is_empty());
}
