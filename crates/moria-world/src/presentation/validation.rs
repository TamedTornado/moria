//! Startup validation for the immutable runtime asset inventory.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Component, Path, PathBuf},
};

use bevy::prelude::*;
use serde::Deserialize;
use sha2::{Digest, Sha256};

use crate::{MaterialRegistry, SparseVoxelStamp};

use super::{AssetDeclaration, AssetId, AssetLoadPolicy, RuntimeAssetProfile, ASSET_DECLARATIONS};

const CONTENT_ASSET_COUNT: usize = 28;

/// The color space recorded by a KTX2 budget declaration.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
pub enum TextureColorSpace {
    Srgb,
    Linear,
}

/// Canonical asset-license registry.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AssetLicenseRegistry {
    pub schema_version: u16,
    pub entries: Vec<AssetLicenseEntry>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AssetLicenseEntry {
    pub stable_id: String,
    pub path: String,
    pub content_sha256: String,
    pub provenance: AssetProvenance,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum AssetProvenance {
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

/// Canonical per-file format and byte-budget registry.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AssetBudgetRegistry {
    pub schema_version: u16,
    pub entries: Vec<AssetBudgetEntry>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AssetBudgetEntry {
    pub stable_id: String,
    pub path: String,
    pub content_sha256: String,
    pub max_file_bytes: u64,
    pub contract: AssetBudgetContract,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum AssetBudgetContract {
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

/// A stable ID representing one shared mesh or material allocation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RenderAssetHandleId(u64);

/// Handles cloned by every repeated object instance.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObjectRenderHandles {
    pub mesh: RenderAssetHandleId,
    pub material: RenderAssetHandleId,
}

/// The only global collection of reusable object render allocations.
#[derive(Resource, Debug)]
pub struct WorldRenderAssets {
    pub terrain_material: RenderAssetHandleId,
    pub water_material: RenderAssetHandleId,
    pub raw_voxel_mesh: RenderAssetHandleId,
    pub fallback_mesh: RenderAssetHandleId,
    pub fallback_material: RenderAssetHandleId,
    object_handles: BTreeMap<AssetId, ObjectRenderHandles>,
}

impl WorldRenderAssets {
    /// Returns cloned handles from the shared allocation; it never allocates.
    #[must_use]
    pub fn object_handles(&self, id: AssetId) -> Option<ObjectRenderHandles> {
        self.object_handles.get(&id).copied()
    }

    fn from_declarations() -> Self {
        let object_handles = ASSET_DECLARATIONS
            .iter()
            .filter(|declaration| declaration.path.ends_with(".glb"))
            .map(|declaration| (declaration.id, handles_for(declaration)))
            .collect();
        Self {
            terrain_material: handles_for(declaration_for(AssetId::TerrainAlbedo)).material,
            water_material: handles_for(declaration_for(AssetId::WaterNormal)).material,
            raw_voxel_mesh: handles_for(declaration_for(AssetId::RawVoxelShader)).mesh,
            fallback_mesh: handles_for(declaration_for(AssetId::Explorer)).mesh,
            fallback_material: handles_for(declaration_for(AssetId::TerrainShader)).material,
            object_handles,
        }
    }
}

impl Default for WorldRenderAssets {
    fn default() -> Self {
        Self::from_declarations()
    }
}

/// Immutable evidence emitted by validation before runtime presentation is ready.
#[derive(Clone, Debug, Resource, Eq, PartialEq)]
pub struct AssetValidationReport {
    pub license_registry_sha256: String,
    pub budget_registry_sha256: String,
    pub development_fallbacks: BTreeSet<String>,
}

/// Startup validation lifecycle; failures intentionally expose no partial assets.
#[derive(Clone, Debug, Resource, Eq, PartialEq)]
pub enum AssetValidationStatus {
    Ready,
    Failed { errors: Vec<AssetValidationError> },
}

/// A deterministic, public validation failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AssetValidationError {
    Read { path: String },
    RegistrySchema { registry: &'static str },
    RegistryInventory { registry: &'static str },
    RegistryEntry { stable_id: String },
    Digest { stable_id: String },
    Budget { stable_id: String },
    Format { stable_id: String },
    Stamp { stable_id: String },
    Shader { stable_id: String },
    MissingAsset { stable_id: String },
}

/// Validates all declared runtime files and installs only shared object handles.
pub struct AssetValidationPlugin {
    root: PathBuf,
    profile: RuntimeAssetProfile,
}

impl AssetValidationPlugin {
    #[must_use]
    pub fn new(root: impl Into<PathBuf>, profile: RuntimeAssetProfile) -> Self {
        Self {
            root: root.into(),
            profile,
        }
    }

    #[must_use]
    pub fn for_development() -> Self {
        Self::new(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets"),
            RuntimeAssetProfile::Development,
        )
    }
}

impl Plugin for AssetValidationPlugin {
    fn build(&self, app: &mut App) {
        let outcome = validate_asset_directory(&self.root, self.profile);
        app.insert_resource(WorldRenderAssets::from_declarations());
        match outcome {
            Ok(report) => {
                app.insert_resource(report);
                app.insert_resource(AssetValidationStatus::Ready);
            }
            Err(errors) => {
                app.insert_resource(AssetValidationStatus::Failed { errors });
            }
        }
    }
}

/// Validates the production registries, bytes, and declared format contracts.
pub fn validate_asset_directory(
    root: &Path,
    profile: RuntimeAssetProfile,
) -> Result<AssetValidationReport, Vec<AssetValidationError>> {
    let license_bytes = read(root, "manifests/asset_licenses.ron")?;
    let budget_bytes = read(root, "manifests/asset_budgets.ron")?;
    let licenses = parse_registry::<AssetLicenseRegistry>(&license_bytes, "licenses")?;
    let budgets = parse_registry::<AssetBudgetRegistry>(&budget_bytes, "budgets")?;
    let mut errors = validate_registries(root, &licenses, &budgets);
    let mut fallbacks = BTreeSet::new();

    for declaration in ASSET_DECLARATIONS
        .iter()
        .filter(|entry| !is_registry(entry.id))
    {
        let Some(budget) = budgets
            .entries
            .iter()
            .find(|entry| entry.stable_id == declaration.id.stable_id())
        else {
            continue;
        };
        match fs::read(root.join(declaration.path)) {
            Ok(bytes) => {
                if bytes.len() as u64 > budget.max_file_bytes {
                    errors.push(AssetValidationError::Budget {
                        stable_id: budget.stable_id.clone(),
                    });
                }
                if digest(&bytes) != budget.content_sha256 {
                    errors.push(AssetValidationError::Digest {
                        stable_id: budget.stable_id.clone(),
                    });
                    continue;
                }
                if !valid_content(declaration.id, &bytes, &budget.contract) {
                    errors.push(AssetValidationError::Format {
                        stable_id: budget.stable_id.clone(),
                    });
                }
            }
            Err(_)
                if profile == RuntimeAssetProfile::Development
                    && declaration.load_policy != AssetLoadPolicy::Required =>
            {
                fallbacks.insert(declaration.id.stable_id().to_owned());
            }
            Err(_) => errors.push(AssetValidationError::MissingAsset {
                stable_id: declaration.id.stable_id().to_owned(),
            }),
        }
    }
    errors.sort_by_key(|error| format!("{error:?}"));
    errors.dedup();
    if errors.is_empty() {
        Ok(AssetValidationReport {
            license_registry_sha256: digest(&license_bytes),
            budget_registry_sha256: digest(&budget_bytes),
            development_fallbacks: fallbacks,
        })
    } else {
        Err(errors)
    }
}

fn read(root: &Path, path: &str) -> Result<Vec<u8>, Vec<AssetValidationError>> {
    fs::read(root.join(path)).map_err(|_| {
        vec![AssetValidationError::Read {
            path: path.to_owned(),
        }]
    })
}

fn parse_registry<T: for<'de> Deserialize<'de>>(
    bytes: &[u8],
    name: &'static str,
) -> Result<T, Vec<AssetValidationError>> {
    ron::de::from_bytes(bytes)
        .map_err(|_| vec![AssetValidationError::RegistrySchema { registry: name }])
}

fn validate_registries(
    root: &Path,
    licenses: &AssetLicenseRegistry,
    budgets: &AssetBudgetRegistry,
) -> Vec<AssetValidationError> {
    let mut errors = Vec::new();
    if licenses.schema_version != 1 {
        errors.push(AssetValidationError::RegistrySchema {
            registry: "licenses",
        });
    }
    if budgets.schema_version != 1 {
        errors.push(AssetValidationError::RegistrySchema {
            registry: "budgets",
        });
    }
    if licenses.entries.len() != CONTENT_ASSET_COUNT {
        errors.push(AssetValidationError::RegistryInventory {
            registry: "licenses",
        });
    }
    if budgets.entries.len() != CONTENT_ASSET_COUNT {
        errors.push(AssetValidationError::RegistryInventory {
            registry: "budgets",
        });
    }
    let declared: BTreeSet<_> = ASSET_DECLARATIONS
        .iter()
        .filter(|entry| !is_registry(entry.id))
        .map(|entry| entry.id.stable_id())
        .collect();
    let mut seen = BTreeSet::new();
    for pair in licenses.entries.windows(2) {
        if pair[0].stable_id >= pair[1].stable_id {
            errors.push(AssetValidationError::RegistryInventory {
                registry: "licenses",
            });
        }
    }
    for entry in &licenses.entries {
        let valid = declared.contains(entry.stable_id.as_str())
            && seen.insert(entry.stable_id.as_str())
            && declaration_matches(&entry.stable_id, &entry.path)
            && valid_digest(&entry.content_sha256)
            && valid_provenance(root, &entry.provenance);
        if !valid {
            errors.push(AssetValidationError::RegistryEntry {
                stable_id: entry.stable_id.clone(),
            });
        }
    }
    for pair in budgets.entries.windows(2) {
        if pair[0].stable_id >= pair[1].stable_id {
            errors.push(AssetValidationError::RegistryInventory {
                registry: "budgets",
            });
        }
    }
    for entry in &budgets.entries {
        let license = licenses
            .entries
            .iter()
            .find(|license| license.stable_id == entry.stable_id);
        let valid = declared.contains(entry.stable_id.as_str())
            && declaration_matches(&entry.stable_id, &entry.path)
            && valid_digest(&entry.content_sha256)
            && entry.max_file_bytes > 0
            && license.is_some_and(|license| {
                license.path == entry.path && license.content_sha256 == entry.content_sha256
            })
            && contract_matches_path(&entry.contract, &entry.path);
        if !valid {
            errors.push(AssetValidationError::RegistryEntry {
                stable_id: entry.stable_id.clone(),
            });
        }
    }
    errors
}

fn valid_content(id: AssetId, bytes: &[u8], contract: &AssetBudgetContract) -> bool {
    match contract {
        AssetBudgetContract::Ron { .. } => match id {
            AssetId::RuinStamp => {
                ron::de::from_bytes::<SparseVoxelStamp>(bytes).is_ok_and(|stamp| {
                    stamp.validate().is_ok()
                        && ["stair_bottom", "stair_top", "entrance"]
                            .iter()
                            .all(|tag| stamp.tags.contains_key(*tag))
                })
            }
            AssetId::Materials => ron::de::from_bytes::<MaterialRegistry>(bytes).is_ok(),
            _ => ron::de::from_bytes::<ron::value::Value>(bytes).is_ok(),
        },
        AssetBudgetContract::Ktx2 {
            width,
            height,
            layers,
            mip_count,
            color_space,
            basis_payload,
        } => valid_ktx2(
            bytes,
            *width,
            *height,
            *layers,
            *mip_count,
            *color_space,
            *basis_payload,
        ),
        AssetBudgetContract::Glb {
            max_triangles_per_primitive,
            required_named_primitives,
            required_animation_clips,
            bounds_min_q8,
            bounds_max_q8,
            support_origin_q8,
        } => valid_glb(
            bytes,
            *max_triangles_per_primitive,
            required_named_primitives,
            required_animation_clips,
            *bounds_min_q8,
            *bounds_max_q8,
            *support_origin_q8,
        ),
        AssetBudgetContract::Wgsl {
            entry_points,
            forbids_i64_atomics,
        } => valid_wgsl(bytes, entry_points, *forbids_i64_atomics),
    }
}

fn valid_ktx2(
    bytes: &[u8],
    width: u32,
    height: u32,
    layers: u16,
    mip_count: u8,
    color_space: TextureColorSpace,
    basis: bool,
) -> bool {
    let Ok(reader) = ktx2::Reader::new(bytes) else {
        return false;
    };
    let header = reader.header();
    let transfer_matches = match color_space {
        TextureColorSpace::Srgb => format!("{:?}", reader.transfer_function()).contains("Srgb"),
        TextureColorSpace::Linear => format!("{:?}", reader.transfer_function()).contains("Linear"),
    };
    header.pixel_width == width
        && header.pixel_height == height
        && header.layer_count == u32::from(layers)
        && header.level_count == u32::from(mip_count)
        && transfer_matches
        && (!basis || header.format.is_none())
}

fn valid_glb(
    bytes: &[u8],
    max_triangles: u32,
    primitive_names: &[String],
    clips: &[String],
    min: [i32; 3],
    max: [i32; 3],
    origin: [i32; 3],
) -> bool {
    let Ok(value) = glb_json(bytes) else {
        return false;
    };
    let Some(meshes) = value.get("meshes").and_then(serde_json::Value::as_array) else {
        return false;
    };
    let names: BTreeSet<_> = meshes
        .iter()
        .filter_map(|mesh| mesh.get("name").and_then(serde_json::Value::as_str))
        .collect();
    if !primitive_names
        .iter()
        .all(|name| names.contains(name.as_str()))
    {
        return false;
    }
    let animation_names: BTreeSet<_> = value
        .get("animations")
        .and_then(serde_json::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|animation| animation.get("name").and_then(serde_json::Value::as_str))
        .collect();
    if !clips
        .iter()
        .all(|clip| animation_names.contains(clip.as_str()))
    {
        return false;
    }
    min.iter()
        .zip(max)
        .all(|(minimum, maximum)| *minimum < maximum)
        && min
            .iter()
            .zip(max)
            .all(|(minimum, maximum)| *minimum <= 0 && 0 <= maximum)
        && origin
            .iter()
            .zip(min)
            .zip(max)
            .all(|((origin, minimum), maximum)| *origin >= minimum && *origin <= maximum)
        && meshes
            .iter()
            .flat_map(|mesh| {
                mesh.get("primitives")
                    .and_then(serde_json::Value::as_array)
                    .into_iter()
                    .flatten()
            })
            .all(|primitive| {
                primitive
                    .get("attributes")
                    .and_then(serde_json::Value::as_object)
                    .is_some_and(|attributes| {
                        attributes.contains_key("POSITION")
                            && attributes.contains_key("NORMAL")
                            && attributes.contains_key("TEXCOORD_0")
                    })
                    && primitive
                        .pointer("/extras/triangle_count")
                        .and_then(serde_json::Value::as_u64)
                        .is_some_and(|count| count <= u64::from(max_triangles))
            })
}

fn glb_json(bytes: &[u8]) -> Result<serde_json::Value, ()> {
    if bytes.starts_with(b"glTF") && bytes.len() >= 20 {
        let length = u32::from_le_bytes(bytes[12..16].try_into().map_err(|_| ())?) as usize;
        serde_json::from_slice(bytes.get(20..20 + length).ok_or(())?).map_err(|_| ())
    } else {
        serde_json::from_slice(bytes).map_err(|_| ())
    }
}

fn valid_wgsl(bytes: &[u8], entry_points: &[String], forbids_i64_atomics: bool) -> bool {
    let Ok(source) = std::str::from_utf8(bytes) else {
        return false;
    };
    naga::front::wgsl::parse_str(source).is_ok()
        && entry_points
            .iter()
            .all(|entry| source.contains(&format!("fn {entry}")))
        && (!forbids_i64_atomics
            || (!source.contains("atomic<i64>") && !source.contains("atomic<u64>")))
}

fn declaration_matches(id: &str, path: &str) -> bool {
    ASSET_DECLARATIONS
        .iter()
        .any(|entry| entry.id.stable_id() == id && entry.path == path)
}
fn declaration_for(id: AssetId) -> &'static AssetDeclaration {
    ASSET_DECLARATIONS
        .iter()
        .find(|declaration| declaration.id == id)
        .expect("the immutable declaration table includes every AssetId")
}
fn is_registry(id: AssetId) -> bool {
    matches!(id, AssetId::AssetLicenses | AssetId::AssetBudgets)
}
fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn valid_digest(value: &str) -> bool {
    value.len() == 64
        && value.bytes().all(|byte| {
            byte.is_ascii_digit() || (byte.is_ascii_lowercase() && byte.is_ascii_hexdigit())
        })
}
fn contract_matches_path(contract: &AssetBudgetContract, path: &str) -> bool {
    matches!(
        (
            contract,
            Path::new(path)
                .extension()
                .and_then(|extension| extension.to_str())
        ),
        (AssetBudgetContract::Ron { .. }, Some("ron"))
            | (AssetBudgetContract::Glb { .. }, Some("glb"))
            | (AssetBudgetContract::Ktx2 { .. }, Some("ktx2"))
            | (AssetBudgetContract::Wgsl { .. }, Some("wgsl"))
    )
}
fn valid_provenance(root: &Path, provenance: &AssetProvenance) -> bool {
    match provenance {
        AssetProvenance::InHouseGenerated {
            generator_or_tool,
            author,
            source_path,
            modifications,
        } => {
            nonblank(generator_or_tool)
                && nonblank(author)
                && source_path.as_ref().is_none_or(|path| relative(path))
                && modifications.iter().all(|value| nonblank(value))
        }
        AssetProvenance::External {
            source_url,
            author,
            license_spdx,
            license_text_path,
            modifications,
        } => {
            source_url.starts_with("https://")
                && nonblank(author)
                && nonblank(license_spdx)
                && relative(license_text_path)
                && root
                    .parent()
                    .is_some_and(|repository| repository.join(license_text_path).is_file())
                && !modifications.is_empty()
                && modifications.iter().all(|value| nonblank(value))
        }
    }
}
fn nonblank(value: &str) -> bool {
    let value = value.trim();
    !value.is_empty()
        && !matches!(
            value.to_ascii_lowercase().as_str(),
            "tbd" | "unknown" | "placeholder"
        )
}
fn relative(path: &str) -> bool {
    !Path::new(path).is_absolute()
        && !Path::new(path)
            .components()
            .any(|part| matches!(part, Component::ParentDir))
}
fn handles_for(declaration: &AssetDeclaration) -> ObjectRenderHandles {
    let hash = Sha256::digest(declaration.id.stable_id().as_bytes());
    let mesh = u64::from_le_bytes(hash[..8].try_into().expect("SHA-256 has 32 bytes"));
    let material = u64::from_le_bytes(hash[8..16].try_into().expect("SHA-256 has 32 bytes"));
    ObjectRenderHandles {
        mesh: RenderAssetHandleId(mesh),
        material: RenderAssetHandleId(material),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeated_instances_clone_identical_shared_handles() {
        let assets = WorldRenderAssets::from_declarations();
        assert_eq!(
            assets.object_handles(AssetId::BirchNear),
            assets.object_handles(AssetId::BirchNear)
        );
        assert_ne!(
            assets.object_handles(AssetId::BirchNear),
            assets.object_handles(AssetId::PineNear)
        );
    }

    #[test]
    fn invalid_shader_and_texture_contracts_are_rejected() {
        assert!(!valid_wgsl(
            b"fn main() {} atomic<i64>",
            &["main".into()],
            true
        ));
        assert!(!valid_ktx2(
            b"not a texture",
            1,
            1,
            1,
            1,
            TextureColorSpace::Linear,
            true
        ));
    }
}
