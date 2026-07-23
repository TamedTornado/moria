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

use super::{ASSET_DECLARATIONS, AssetDeclaration, AssetId, AssetLoadPolicy, RuntimeAssetProfile};

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
    let Ok((value, binary)) = glb_document(bytes) else {
        return false;
    };
    let Some(layout) = GlbLayout::new(&value, binary) else {
        return false;
    };
    if !valid_gltf_references(&value, &layout) {
        return false;
    }
    let Some(meshes) = value.get("meshes").and_then(serde_json::Value::as_array) else {
        return false;
    };
    if meshes.is_empty() {
        return false;
    }
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
    if !clips.is_empty() {
        let Some(animations) = value
            .get("animations")
            .and_then(serde_json::Value::as_array)
        else {
            return false;
        };
        if !clips.iter().all(|clip| {
            animations.iter().any(|animation| {
                animation.get("name").and_then(serde_json::Value::as_str) == Some(clip)
                    && valid_animation(animation, &layout, value.get("nodes"))
            })
        }) {
            return false;
        }
    }
    if !min
        .iter()
        .zip(max)
        .all(|(minimum, maximum)| *minimum < maximum)
        || !min
            .iter()
            .zip(max)
            .all(|(minimum, maximum)| *minimum <= 0 && 0 <= maximum)
        || !origin
            .iter()
            .zip(min)
            .zip(max)
            .all(|((origin, minimum), maximum)| *origin >= minimum && *origin <= maximum)
    {
        return false;
    }
    if !valid_support_centered_scenes(&value, meshes.len()) {
        return false;
    }

    let mut asset_bounds = None;
    for mesh in meshes {
        let Some(primitives) = mesh.get("primitives").and_then(serde_json::Value::as_array) else {
            return false;
        };
        if primitives.is_empty() {
            return false;
        }
        for primitive in primitives {
            let Some(bounds) = valid_primitive(primitive, &value, &layout, max_triangles) else {
                return false;
            };
            asset_bounds = Some(match asset_bounds {
                Some((asset_minimum, asset_maximum)) => {
                    combine_bounds(asset_minimum, asset_maximum, bounds.0, bounds.1)
                }
                None => bounds,
            });
        }
    }
    asset_bounds.is_some_and(|(actual_minimum, actual_maximum)| {
        q8_bounds_match(actual_minimum, actual_maximum, min, max)
    })
}

fn valid_gltf_references(document: &serde_json::Value, layout: &GlbLayout<'_>) -> bool {
    let Some(meshes) = document.get("meshes").and_then(serde_json::Value::as_array) else {
        return false;
    };
    let Some(materials) = document
        .get("materials")
        .and_then(serde_json::Value::as_array)
    else {
        return false;
    };
    let textures = document
        .get("textures")
        .and_then(serde_json::Value::as_array)
        .map_or(&[] as &[serde_json::Value], Vec::as_slice);
    let images = document
        .get("images")
        .and_then(serde_json::Value::as_array)
        .map_or(&[] as &[serde_json::Value], Vec::as_slice);
    let samplers = document
        .get("samplers")
        .and_then(serde_json::Value::as_array)
        .map_or(&[] as &[serde_json::Value], Vec::as_slice);
    let nodes = document
        .get("nodes")
        .and_then(serde_json::Value::as_array)
        .map_or(&[] as &[serde_json::Value], Vec::as_slice);
    let skins = document
        .get("skins")
        .and_then(serde_json::Value::as_array)
        .map_or(&[] as &[serde_json::Value], Vec::as_slice);
    let cameras = document
        .get("cameras")
        .and_then(serde_json::Value::as_array)
        .map_or(&[] as &[serde_json::Value], Vec::as_slice);

    meshes.iter().all(|mesh| {
        mesh.get("primitives")
            .and_then(serde_json::Value::as_array)
            .is_some_and(|primitives| {
                primitives
                    .iter()
                    .all(|primitive| valid_primitive_references(primitive, layout, materials))
            })
    }) && materials
        .iter()
        .all(|material| valid_material_references(material, textures))
        && textures.iter().all(|texture| {
            texture.is_object()
                && texture.get("source").is_none_or(|source| {
                    value_index(source).is_some_and(|index| index < images.len())
                })
                && texture.get("sampler").is_none_or(|sampler| {
                    value_index(sampler).is_some_and(|index| index < samplers.len())
                })
        })
        && images.iter().all(|image| {
            image.is_object()
                && image.get("bufferView").is_none_or(|view| {
                    value_index(view).is_some_and(|index| index < layout.view_count())
                })
        })
        && nodes.iter().all(|node| {
            node.is_object()
                && node
                    .get("mesh")
                    .is_none_or(|mesh| value_index(mesh).is_some_and(|index| index < meshes.len()))
                && node
                    .get("skin")
                    .is_none_or(|skin| value_index(skin).is_some_and(|index| index < skins.len()))
                && node.get("camera").is_none_or(|camera| {
                    value_index(camera).is_some_and(|index| index < cameras.len())
                })
        })
        && skins
            .iter()
            .all(|skin| valid_skin_references(skin, layout, nodes))
        && nodes
            .iter()
            .all(|node| valid_skinned_node(node, meshes, skins, layout))
        && document
            .get("animations")
            .and_then(serde_json::Value::as_array)
            .is_none_or(|animations| {
                animations
                    .iter()
                    .all(|animation| valid_animation(animation, layout, document.get("nodes")))
            })
}

fn valid_skin_references(
    skin: &serde_json::Value,
    layout: &GlbLayout<'_>,
    nodes: &[serde_json::Value],
) -> bool {
    let Some(joints) = skin.get("joints").and_then(serde_json::Value::as_array) else {
        return false;
    };
    let Some(joints) = joints.iter().map(value_index).collect::<Option<Vec<_>>>() else {
        return false;
    };
    skin.is_object()
        && !joints.is_empty()
        && joints.iter().all(|&joint| joint < nodes.len())
        && joints.iter().collect::<BTreeSet<_>>().len() == joints.len()
        && skin.get("inverseBindMatrices").is_none_or(|accessor| {
            value_index(accessor)
                .and_then(|index| layout.accessor(index))
                .is_some_and(|accessor| {
                    accessor.is_f32_type("MAT4")
                        && accessor.count >= joints.len() as u32
                        && finite_f32_accessor(accessor, layout.binary)
                })
        })
        && skin
            .get("skeleton")
            .is_none_or(|node| valid_skeleton_root(node, &joints, nodes))
}

fn valid_skeleton_root(
    skeleton: &serde_json::Value,
    joints: &[usize],
    nodes: &[serde_json::Value],
) -> bool {
    let Some(skeleton) = value_index(skeleton).filter(|&index| index < nodes.len()) else {
        return false;
    };
    let mut parents = vec![None; nodes.len()];
    for (parent, node) in nodes.iter().enumerate() {
        let Some(children) = node.get("children") else {
            continue;
        };
        let Some(children) = children.as_array() else {
            return false;
        };
        for child in children {
            let Some(child) = value_index(child).filter(|&index| index < nodes.len()) else {
                return false;
            };
            if parents[child].replace(parent).is_some() {
                return false;
            }
        }
    }
    joints.iter().copied().all(|joint| {
        std::iter::successors(Some(joint), |&node| parents[node]).any(|node| node == skeleton)
    })
}

fn valid_skinned_node(
    node: &serde_json::Value,
    meshes: &[serde_json::Value],
    skins: &[serde_json::Value],
    layout: &GlbLayout<'_>,
) -> bool {
    let Some(skin) = node
        .get("skin")
        .and_then(value_index)
        .and_then(|index| skins.get(index))
    else {
        return node.get("skin").is_none();
    };
    let Some(mesh) = node
        .get("mesh")
        .and_then(value_index)
        .and_then(|index| meshes.get(index))
    else {
        return false;
    };
    let Some(joints) = skin.get("joints").and_then(serde_json::Value::as_array) else {
        return false;
    };
    mesh.get("primitives")
        .and_then(serde_json::Value::as_array)
        .is_some_and(|primitives| {
            primitives
                .iter()
                .all(|primitive| valid_skinned_primitive(primitive, layout, joints.len() as u32))
        })
}

fn valid_skinned_primitive(
    primitive: &serde_json::Value,
    layout: &GlbLayout<'_>,
    joint_count: u32,
) -> bool {
    let Some(attributes) = primitive
        .get("attributes")
        .and_then(serde_json::Value::as_object)
    else {
        return false;
    };
    let Some(position) = attributes
        .get("POSITION")
        .and_then(value_index)
        .and_then(|index| layout.accessor(index))
    else {
        return false;
    };
    let Some(Some((joints_0, weights_0))) =
        skin_attribute_pair(attributes, "JOINTS_0", "WEIGHTS_0", layout)
    else {
        return false;
    };
    let joints_1 = skin_attribute_pair(attributes, "JOINTS_1", "WEIGHTS_1", layout);
    let Some(joints_1) = joints_1 else {
        return false;
    };
    let weight_sets =
        std::iter::once(weights_0).chain(joints_1.iter().map(|(_, weights)| *weights));
    valid_joint_weight_set(joints_0, weights_0, position, layout, joint_count)
        && joints_1.is_none_or(|(joints, weights)| {
            valid_joint_weight_set(joints, weights, position, layout, joint_count)
        })
        && valid_weight_sum(weight_sets, layout.binary, position.count)
}

fn skin_attribute_pair<'a>(
    attributes: &serde_json::Map<String, serde_json::Value>,
    joints_name: &str,
    weights_name: &str,
    layout: &'a GlbLayout<'_>,
) -> Option<Option<(&'a GlbAccessor, &'a GlbAccessor)>> {
    match (attributes.get(joints_name), attributes.get(weights_name)) {
        (None, None) => Some(None),
        (Some(joints), Some(weights)) => Some(Some((
            layout.accessor(value_index(joints)?)?,
            layout.accessor(value_index(weights)?)?,
        ))),
        _ => None,
    }
}

fn valid_joint_weight_set(
    joints: &GlbAccessor,
    weights: &GlbAccessor,
    position: &GlbAccessor,
    layout: &GlbLayout<'_>,
    joint_count: u32,
) -> bool {
    joints.is_joint_type()
        && weights.is_weight_type()
        && joints.count == position.count
        && weights.count == position.count
        && joint_indices_in_range(joints, layout.binary, joint_count)
}

fn valid_weight_sum<'a>(
    weight_sets: impl Iterator<Item = &'a GlbAccessor>,
    binary: &[u8],
    vertex_count: u32,
) -> bool {
    let weight_sets = weight_sets.collect::<Vec<_>>();
    (0..usize::try_from(vertex_count).unwrap_or(usize::MAX)).all(|element| {
        let Some(sum) = weight_sets.iter().try_fold(0.0_f32, |sum, accessor| {
            (0..4).try_fold(sum, |sum, component| {
                let value = weight_component(accessor, binary, element, component)?;
                (value.is_finite() && value >= 0.0).then_some(sum + value)
            })
        }) else {
            return false;
        };
        (sum - 1.0).abs() <= 0.0001
    })
}

fn weight_component(
    accessor: &GlbAccessor,
    binary: &[u8],
    element: usize,
    component: usize,
) -> Option<f32> {
    let offset = element
        .checked_mul(accessor.stride)?
        .checked_add(accessor.offset)?
        .checked_add(component * component_size(accessor.component_type)?)?;
    match accessor.component_type {
        5121 => Some(f32::from(*binary.get(offset)?) / f32::from(u8::MAX)),
        5123 => Some(
            f32::from(u16::from_le_bytes(
                binary.get(offset..offset + 2)?.try_into().ok()?,
            )) / f32::from(u16::MAX),
        ),
        5126 => f32_at(binary, offset),
        _ => None,
    }
}

fn joint_indices_in_range(accessor: &GlbAccessor, binary: &[u8], joint_count: u32) -> bool {
    (0..usize::try_from(accessor.count).unwrap_or(usize::MAX)).all(|element| {
        (0..4).all(|component| {
            let offset = element
                .checked_mul(accessor.stride)
                .and_then(|offset| offset.checked_add(accessor.offset))
                .and_then(|offset| {
                    offset.checked_add(component * component_size(accessor.component_type)?)
                });
            match accessor.component_type {
                5121 => offset
                    .and_then(|offset| binary.get(offset).copied())
                    .is_some_and(|index| u32::from(index) < joint_count),
                5123 => offset
                    .and_then(|offset| binary.get(offset..offset + 2))
                    .and_then(|bytes| bytes.try_into().ok())
                    .map(u16::from_le_bytes)
                    .is_some_and(|index| u32::from(index) < joint_count),
                _ => false,
            }
        })
    })
}

fn valid_primitive_references(
    primitive: &serde_json::Value,
    layout: &GlbLayout<'_>,
    materials: &[serde_json::Value],
) -> bool {
    primitive.is_object()
        && primitive
            .get("attributes")
            .and_then(serde_json::Value::as_object)
            .is_some_and(|attributes| {
                attributes.values().all(|accessor| {
                    value_index(accessor).is_some_and(|index| layout.accessor(index).is_some())
                })
            })
        && primitive.get("indices").is_none_or(|accessor| {
            value_index(accessor).is_some_and(|index| layout.accessor(index).is_some())
        })
        && primitive.get("material").is_some_and(|material| {
            value_index(material).is_some_and(|index| index < materials.len())
        })
        && primitive.get("targets").is_none_or(|targets| {
            targets.as_array().is_some_and(|targets| {
                targets.iter().all(|target| {
                    target.as_object().is_some_and(|attributes| {
                        attributes.values().all(|accessor| {
                            value_index(accessor)
                                .is_some_and(|index| layout.accessor(index).is_some())
                        })
                    })
                })
            })
        })
}

fn valid_material_references(material: &serde_json::Value, textures: &[serde_json::Value]) -> bool {
    material.is_object()
        && ["normalTexture", "occlusionTexture", "emissiveTexture"]
            .iter()
            .all(|field| {
                material
                    .get(*field)
                    .is_none_or(|texture| valid_texture_reference(texture, textures))
            })
        && material.get("pbrMetallicRoughness").is_none_or(|pbr| {
            pbr.as_object().is_some_and(|pbr| {
                ["baseColorTexture", "metallicRoughnessTexture"]
                    .iter()
                    .all(|field| {
                        pbr.get(*field)
                            .is_none_or(|texture| valid_texture_reference(texture, textures))
                    })
            })
        })
}

fn valid_texture_reference(texture: &serde_json::Value, textures: &[serde_json::Value]) -> bool {
    texture
        .get("index")
        .and_then(value_index)
        .is_some_and(|index| index < textures.len())
}

fn valid_animation(
    animation: &serde_json::Value,
    layout: &GlbLayout<'_>,
    nodes: Option<&serde_json::Value>,
) -> bool {
    let Some(nodes) = nodes.and_then(serde_json::Value::as_array) else {
        return false;
    };
    let Some(samplers) = animation
        .get("samplers")
        .and_then(serde_json::Value::as_array)
    else {
        return false;
    };
    let Some(channels) = animation
        .get("channels")
        .and_then(serde_json::Value::as_array)
    else {
        return false;
    };
    if samplers.is_empty() || channels.is_empty() {
        return false;
    }
    if !samplers
        .iter()
        .all(|sampler| valid_animation_sampler(sampler, layout))
    {
        return false;
    }

    channels.iter().all(|channel| {
        let Some(sampler) = channel
            .get("sampler")
            .and_then(value_index)
            .and_then(|index| samplers.get(index))
        else {
            return false;
        };
        let Some(input) = sampler
            .get("input")
            .and_then(value_index)
            .and_then(|index| layout.accessor(index))
        else {
            return false;
        };
        let Some(output) = sampler
            .get("output")
            .and_then(value_index)
            .and_then(|index| layout.accessor(index))
        else {
            return false;
        };
        let interpolation =
            animation_interpolation(sampler).expect("validated sampler interpolation");
        let Some(expected_output_count) = (match interpolation {
            "LINEAR" | "STEP" => Some(input.count),
            "CUBICSPLINE" => input.count.checked_mul(3),
            _ => None,
        }) else {
            return false;
        };
        let Some(target) = channel.get("target").and_then(serde_json::Value::as_object) else {
            return false;
        };
        let Some(node) = target.get("node").and_then(value_index) else {
            return false;
        };
        let Some(path) = target.get("path").and_then(serde_json::Value::as_str) else {
            return false;
        };
        nodes.get(node).is_some()
            && match path {
                "translation" | "scale" => {
                    output.is_f32_type("VEC3") && output.count == expected_output_count
                }
                "rotation" => output.is_f32_type("VEC4") && output.count == expected_output_count,
                "weights" => {
                    output.is_f32_type("SCALAR")
                        && output.count >= expected_output_count
                        && output.count % expected_output_count == 0
                }
                _ => false,
            }
    })
}

fn valid_animation_sampler(sampler: &serde_json::Value, layout: &GlbLayout<'_>) -> bool {
    let Some(input) = sampler
        .get("input")
        .and_then(value_index)
        .and_then(|index| layout.accessor(index))
    else {
        return false;
    };
    let Some(output) = sampler
        .get("output")
        .and_then(value_index)
        .and_then(|index| layout.accessor(index))
    else {
        return false;
    };

    sampler.is_object()
        && animation_interpolation(sampler).is_some()
        && input.is_f32_type("SCALAR")
        && finite_f32_accessor(input, layout.binary)
        && strictly_increasing_f32_accessor(input, layout.binary)
        && output.component_type == 5126
        && finite_f32_accessor(output, layout.binary)
}

fn animation_interpolation(sampler: &serde_json::Value) -> Option<&str> {
    match sampler.get("interpolation") {
        None => Some("LINEAR"),
        Some(interpolation) => match interpolation.as_str()? {
            "LINEAR" | "STEP" | "CUBICSPLINE" => interpolation.as_str(),
            _ => None,
        },
    }
}

fn strictly_increasing_f32_accessor(accessor: &GlbAccessor, binary: &[u8]) -> bool {
    let mut previous = None;
    for element in 0..usize::try_from(accessor.count).unwrap_or(usize::MAX) {
        let Some(value) = element
            .checked_mul(accessor.stride)
            .and_then(|offset| offset.checked_add(accessor.offset))
            .and_then(|offset| f32_at(binary, offset))
        else {
            return false;
        };
        if previous.is_some_and(|previous| previous >= value) {
            return false;
        }
        previous = Some(value);
    }
    true
}

fn valid_primitive(
    primitive: &serde_json::Value,
    document: &serde_json::Value,
    layout: &GlbLayout<'_>,
    max_triangles: u32,
) -> Option<([f32; 3], [f32; 3])> {
    if primitive
        .get("mode")
        .is_some_and(|mode| mode.as_u64() != Some(4))
    {
        return None;
    }
    let attributes = primitive
        .get("attributes")
        .and_then(serde_json::Value::as_object)?;
    let position = attributes
        .get("POSITION")
        .and_then(value_index)
        .and_then(|index| layout.accessor(index))?;
    let normal = attributes
        .get("NORMAL")
        .and_then(value_index)
        .and_then(|index| layout.accessor(index))?;
    let uv = attributes
        .get("TEXCOORD_0")
        .and_then(value_index)
        .and_then(|index| layout.accessor(index))?;
    if !position.is_f32_type("VEC3")
        || !normal.is_f32_type("VEC3")
        || !uv.is_f32_type("VEC2")
        || position.count == 0
        || normal.count != position.count
        || uv.count != position.count
        || !finite_f32_accessor(position, layout.binary)
        || !finite_f32_accessor(normal, layout.binary)
        || !finite_f32_accessor(uv, layout.binary)
    {
        return None;
    }
    let position_bounds = position_bounds(position, layout.binary)?;

    let material = primitive
        .get("material")
        .and_then(value_index)
        .and_then(|index| document.get("materials")?.as_array()?.get(index))?;
    let normal_mapped = material.get("normalTexture").is_some();
    if normal_mapped {
        let tangent = attributes
            .get("TANGENT")
            .and_then(value_index)
            .and_then(|index| layout.accessor(index))?;
        if !tangent.is_f32_type("VEC4")
            || tangent.count != position.count
            || !finite_f32_accessor(tangent, layout.binary)
        {
            return None;
        }
    }

    let indices = primitive
        .get("indices")
        .and_then(value_index)
        .and_then(|index| layout.accessor(index))?;
    (indices.is_index_type()
        && indices.count != 0
        && indices.count % 3 == 0
        && indices.count / 3 <= max_triangles
        && indices_are_in_range(indices, layout.binary, position.count))
    .then_some(position_bounds)
}

fn glb_document(bytes: &[u8]) -> Result<(serde_json::Value, &[u8]), ()> {
    if bytes.len() < 20 || &bytes[..4] != b"glTF" || le_u32(bytes, 4)? != 2 {
        return Err(());
    }
    if usize::try_from(le_u32(bytes, 8)?).map_err(|_| ())? != bytes.len() {
        return Err(());
    }
    let json_length = usize::try_from(le_u32(bytes, 12)?).map_err(|_| ())?;
    if !json_length.is_multiple_of(4) || le_u32(bytes, 16)? != 0x4E4F_534A {
        return Err(());
    }
    let json_end = 20usize.checked_add(json_length).ok_or(())?;
    let binary_header = json_end.checked_add(8).ok_or(())?;
    if binary_header > bytes.len() || le_u32(bytes, json_end + 4)? != 0x004E_4942 {
        return Err(());
    }
    let binary_length = usize::try_from(le_u32(bytes, json_end)?).map_err(|_| ())?;
    let binary_end = binary_header.checked_add(binary_length).ok_or(())?;
    if !binary_length.is_multiple_of(4) || binary_end > bytes.len() {
        return Err(());
    }
    let mut chunk_offset = binary_end;
    while chunk_offset < bytes.len() {
        let chunk_header = chunk_offset.checked_add(8).ok_or(())?;
        if chunk_header > bytes.len() {
            return Err(());
        }
        let chunk_length = usize::try_from(le_u32(bytes, chunk_offset)?).map_err(|_| ())?;
        if !chunk_length.is_multiple_of(4) {
            return Err(());
        }
        chunk_offset = chunk_header.checked_add(chunk_length).ok_or(())?;
        if chunk_offset > bytes.len() {
            return Err(());
        }
    }
    let document = serde_json::from_slice(bytes.get(20..json_end).ok_or(())?).map_err(|_| ())?;
    Ok((document, &bytes[binary_header..binary_end]))
}

struct GlbLayout<'a> {
    binary: &'a [u8],
    accessors: Vec<GlbAccessor>,
    view_count: usize,
}

impl<'a> GlbLayout<'a> {
    fn new(document: &serde_json::Value, binary: &'a [u8]) -> Option<Self> {
        let buffers = document.get("buffers")?.as_array()?;
        if buffers.len() != 1 || buffers[0].get("uri").is_some() {
            return None;
        }
        let buffer_length = value_usize(buffers[0].get("byteLength")?)?;
        let padded_buffer_length = buffer_length.checked_add(3)? & !3;
        if padded_buffer_length != binary.len() {
            return None;
        }
        let views = document.get("bufferViews")?.as_array()?;
        let mut view_ranges = Vec::with_capacity(views.len());
        for view in views {
            if value_usize(view.get("buffer")?)? != 0 {
                return None;
            }
            let offset = match view.get("byteOffset") {
                Some(offset) => value_usize(offset)?,
                None => 0,
            };
            let length = value_usize(view.get("byteLength")?)?;
            if offset.checked_add(length)? > buffer_length {
                return None;
            }
            let stride = match view.get("byteStride") {
                Some(stride) => Some(value_usize(stride)?),
                None => None,
            };
            if stride.is_some_and(|stride| !(4..=252).contains(&stride) || stride % 4 != 0) {
                return None;
            }
            view_ranges.push((offset, length, stride));
        }
        let accessors = document.get("accessors")?.as_array()?;
        let mut parsed = Vec::with_capacity(accessors.len());
        for accessor in accessors {
            if accessor.get("sparse").is_some() {
                return None;
            }
            let view_index = value_usize(accessor.get("bufferView")?)?;
            let &(view_offset, view_length, stride) = view_ranges.get(view_index)?;
            let component_type =
                u32::try_from(value_usize(accessor.get("componentType")?)?).ok()?;
            let component_size = component_size(component_type)?;
            let normalized = accessor
                .get("normalized")
                .map_or(Some(false), serde_json::Value::as_bool)?;
            let accessor_type = match accessor.get("type")?.as_str()? {
                "SCALAR" => "SCALAR",
                "VEC2" => "VEC2",
                "VEC3" => "VEC3",
                "VEC4" => "VEC4",
                "MAT2" => "MAT2",
                "MAT3" => "MAT3",
                "MAT4" => "MAT4",
                _ => return None,
            };
            let components = component_count(accessor_type)?;
            let count = u32::try_from(value_usize(accessor.get("count")?)?).ok()?;
            if count == 0 {
                return None;
            }
            let element_size = component_size.checked_mul(components)?;
            let accessor_offset = match accessor.get("byteOffset") {
                Some(offset) => value_usize(offset)?,
                None => 0,
            };
            let element_stride = stride.unwrap_or(element_size);
            let used = usize::try_from(count)
                .ok()?
                .checked_sub(1)?
                .checked_mul(element_stride)?
                .checked_add(element_size)?;
            if view_offset.checked_add(accessor_offset)? % component_size != 0
                || element_stride < element_size
                || accessor_offset.checked_add(used)? > view_length
            {
                return None;
            }
            parsed.push(GlbAccessor {
                component_type,
                normalized,
                count,
                accessor_type,
                offset: view_offset.checked_add(accessor_offset)?,
                stride: element_stride,
                bounds: accessor_bounds(accessor, accessor_type),
            });
        }
        Some(Self {
            binary,
            accessors: parsed,
            view_count: view_ranges.len(),
        })
    }

    fn accessor(&self, index: usize) -> Option<&GlbAccessor> {
        self.accessors.get(index)
    }

    fn view_count(&self) -> usize {
        self.view_count
    }
}

struct GlbAccessor {
    component_type: u32,
    normalized: bool,
    count: u32,
    accessor_type: &'static str,
    offset: usize,
    stride: usize,
    bounds: Option<([f32; 3], [f32; 3])>,
}

impl GlbAccessor {
    fn is_f32_type(&self, expected_type: &str) -> bool {
        self.component_type == 5126 && self.accessor_type == expected_type
    }

    fn is_index_type(&self) -> bool {
        self.accessor_type == "SCALAR" && matches!(self.component_type, 5121 | 5123 | 5125)
    }

    fn is_joint_type(&self) -> bool {
        self.accessor_type == "VEC4" && matches!(self.component_type, 5121 | 5123)
    }

    fn is_weight_type(&self) -> bool {
        self.accessor_type == "VEC4"
            && (self.component_type == 5126
                || (self.normalized && matches!(self.component_type, 5121 | 5123)))
    }
}

fn finite_f32_accessor(accessor: &GlbAccessor, binary: &[u8]) -> bool {
    let Some(components) = component_count(accessor.accessor_type) else {
        return false;
    };
    (0..usize::try_from(accessor.count).unwrap_or(usize::MAX)).all(|element| {
        (0..components).all(|component| {
            let offset = element
                .checked_mul(accessor.stride)
                .and_then(|offset| offset.checked_add(accessor.offset))
                .and_then(|offset| offset.checked_add(component * 4));
            offset
                .and_then(|offset| binary.get(offset..offset + 4))
                .and_then(|bytes| bytes.try_into().ok())
                .map(f32::from_le_bytes)
                .is_some_and(f32::is_finite)
        })
    })
}

fn valid_support_centered_scenes(document: &serde_json::Value, mesh_count: usize) -> bool {
    if document
        .pointer("/asset/extras/origin")
        .and_then(serde_json::Value::as_str)
        != Some("support_center")
    {
        return false;
    }
    let Some(nodes) = document.get("nodes").and_then(serde_json::Value::as_array) else {
        return false;
    };
    let joint_nodes = document
        .get("skins")
        .and_then(serde_json::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|skin| skin.get("joints").and_then(serde_json::Value::as_array))
        .flatten()
        .filter_map(value_index)
        .collect::<BTreeSet<_>>();
    if nodes.is_empty()
        || !nodes.iter().enumerate().all(|(index, node)| {
            joint_nodes.contains(&index) && node.get("mesh").is_none()
                || identity_node_transform(node)
        })
    {
        return false;
    }
    let Some(scenes) = document.get("scenes").and_then(serde_json::Value::as_array) else {
        return false;
    };
    if !valid_node_hierarchy(nodes, scenes) {
        return false;
    }
    let Some(scene) = document
        .get("scene")
        .and_then(value_index)
        .and_then(|index| scenes.get(index))
    else {
        return false;
    };
    let Some(roots) = scene.get("nodes").and_then(serde_json::Value::as_array) else {
        return false;
    };
    if roots.is_empty() {
        return false;
    }

    let mut visited_nodes = BTreeSet::new();
    let mut reachable_meshes = BTreeSet::new();
    for root in roots {
        let Some(root_index) = value_index(root) else {
            return false;
        };
        if !visit_active_scene_node(
            nodes,
            root_index,
            mesh_count,
            &mut visited_nodes,
            &mut reachable_meshes,
        ) {
            return false;
        }
    }
    reachable_meshes.len() == mesh_count
}

fn visit_active_scene_node(
    nodes: &[serde_json::Value],
    node_index: usize,
    mesh_count: usize,
    visited_nodes: &mut BTreeSet<usize>,
    reachable_meshes: &mut BTreeSet<usize>,
) -> bool {
    let Some(node) = nodes.get(node_index) else {
        return false;
    };
    if !visited_nodes.insert(node_index) {
        return true;
    }
    if let Some(mesh) = node.get("mesh") {
        let Some(mesh_index) = value_index(mesh) else {
            return false;
        };
        if mesh_index >= mesh_count {
            return false;
        }
        reachable_meshes.insert(mesh_index);
    }
    if let Some(children) = node.get("children") {
        let Some(children) = children.as_array() else {
            return false;
        };
        for child in children {
            let Some(child_index) = value_index(child) else {
                return false;
            };
            if !visit_active_scene_node(
                nodes,
                child_index,
                mesh_count,
                visited_nodes,
                reachable_meshes,
            ) {
                return false;
            }
        }
    }
    true
}

fn valid_node_hierarchy(nodes: &[serde_json::Value], scenes: &[serde_json::Value]) -> bool {
    let mut parent_counts = vec![0_u32; nodes.len()];
    for node in nodes {
        if let Some(children) = node.get("children") {
            let Some(children) = children.as_array() else {
                return false;
            };
            for child in children {
                let Some(child_index) = value_index(child) else {
                    return false;
                };
                let Some(parent_count) = parent_counts.get_mut(child_index) else {
                    return false;
                };
                *parent_count = match parent_count.checked_add(1) {
                    Some(parent_count) if parent_count <= 1 => parent_count,
                    _ => return false,
                };
            }
        }
    }

    for scene in scenes {
        let mut scene_roots = BTreeSet::new();
        if let Some(roots) = scene.get("nodes") {
            let Some(roots) = roots.as_array() else {
                return false;
            };
            for root in roots {
                let Some(root_index) = value_index(root) else {
                    return false;
                };
                if parent_counts.get(root_index) != Some(&0) || !scene_roots.insert(root_index) {
                    return false;
                }
            }
        }
    }

    let mut complete_nodes = BTreeSet::new();
    for node_index in 0..nodes.len() {
        if !visit_node_hierarchy(node_index, nodes, &mut complete_nodes, &mut BTreeSet::new()) {
            return false;
        }
    }
    true
}

fn visit_node_hierarchy(
    node_index: usize,
    nodes: &[serde_json::Value],
    complete_nodes: &mut BTreeSet<usize>,
    ancestors: &mut BTreeSet<usize>,
) -> bool {
    if complete_nodes.contains(&node_index) {
        return true;
    }
    if !ancestors.insert(node_index) {
        return false;
    }
    let Some(node) = nodes.get(node_index) else {
        return false;
    };
    let Some(children) = node.get("children").and_then(serde_json::Value::as_array) else {
        ancestors.remove(&node_index);
        complete_nodes.insert(node_index);
        return true;
    };
    for child in children {
        let Some(child_index) = value_index(child) else {
            return false;
        };
        if !visit_node_hierarchy(child_index, nodes, complete_nodes, ancestors) {
            return false;
        }
    }
    ancestors.remove(&node_index);
    complete_nodes.insert(node_index);
    true
}

fn identity_node_transform(node: &serde_json::Value) -> bool {
    if !node.is_object() {
        return false;
    }
    let translation = node
        .get("translation")
        .is_none_or(|value| identity_vector(value, &[0.0, 0.0, 0.0]));
    let rotation = node
        .get("rotation")
        .is_none_or(|value| identity_vector(value, &[0.0, 0.0, 0.0, 1.0]));
    let scale = node
        .get("scale")
        .is_none_or(|value| identity_vector(value, &[1.0, 1.0, 1.0]));
    let matrix = node.get("matrix").is_none_or(|value| {
        identity_vector(
            value,
            &[
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        )
    });
    translation && rotation && scale && matrix
}

fn identity_vector(value: &serde_json::Value, expected: &[f64]) -> bool {
    value.as_array().is_some_and(|values| {
        values.len() == expected.len()
            && values
                .iter()
                .zip(expected)
                .all(|(value, expected)| value.as_f64() == Some(*expected))
    })
}

fn position_bounds(position: &GlbAccessor, binary: &[u8]) -> Option<([f32; 3], [f32; 3])> {
    let mut minimum = [f32::INFINITY; 3];
    let mut maximum = [f32::NEG_INFINITY; 3];
    for element in 0..usize::try_from(position.count).unwrap_or(usize::MAX) {
        for component in 0..3 {
            let value = f32_at(
                binary,
                position.offset + element * position.stride + component * 4,
            )?;
            minimum[component] = minimum[component].min(value);
            maximum[component] = maximum[component].max(value);
        }
    }
    position
        .bounds
        .filter(|(declared_minimum, declared_maximum)| {
            *declared_minimum == minimum && *declared_maximum == maximum
        })
        .map(|_| (minimum, maximum))
}

fn combine_bounds(
    first_minimum: [f32; 3],
    first_maximum: [f32; 3],
    second_minimum: [f32; 3],
    second_maximum: [f32; 3],
) -> ([f32; 3], [f32; 3]) {
    (
        std::array::from_fn(|index| first_minimum[index].min(second_minimum[index])),
        std::array::from_fn(|index| first_maximum[index].max(second_maximum[index])),
    )
}

fn q8_bounds_match(
    minimum: [f32; 3],
    maximum: [f32; 3],
    expected_minimum: [i32; 3],
    expected_maximum: [i32; 3],
) -> bool {
    minimum
        .iter()
        .zip(maximum)
        .zip(expected_minimum)
        .zip(expected_maximum)
        .all(
            |(((minimum, maximum), expected_minimum), expected_maximum)| {
                q8(*minimum) == Some(expected_minimum) && q8(maximum) == Some(expected_maximum)
            },
        )
}

fn accessor_bounds(value: &serde_json::Value, accessor_type: &str) -> Option<([f32; 3], [f32; 3])> {
    if accessor_type != "VEC3" {
        return None;
    }
    Some((
        f32_values(value.get("min")?)?,
        f32_values(value.get("max")?)?,
    ))
}

fn f32_values(value: &serde_json::Value) -> Option<[f32; 3]> {
    let values = value.as_array()?;
    if values.len() != 3 {
        return None;
    }
    let values = [
        values[0].as_f64()? as f32,
        values[1].as_f64()? as f32,
        values[2].as_f64()? as f32,
    ];
    values
        .iter()
        .all(|value| value.is_finite())
        .then_some(values)
}

fn indices_are_in_range(accessor: &GlbAccessor, binary: &[u8], vertex_count: u32) -> bool {
    (0..usize::try_from(accessor.count).unwrap_or(usize::MAX)).all(|element| {
        let offset = accessor.offset + element * accessor.stride;
        let index = match accessor.component_type {
            5121 => binary.get(offset).copied().map(u32::from),
            5123 => binary
                .get(offset..offset + 2)
                .and_then(|bytes| bytes.try_into().ok())
                .map(u16::from_le_bytes)
                .map(u32::from),
            5125 => binary
                .get(offset..offset + 4)
                .and_then(|bytes| bytes.try_into().ok())
                .map(u32::from_le_bytes),
            _ => None,
        };
        index.is_some_and(|index| index < vertex_count)
    })
}

fn component_size(component_type: u32) -> Option<usize> {
    match component_type {
        5120 | 5121 => Some(1),
        5122 | 5123 => Some(2),
        5125 | 5126 => Some(4),
        _ => None,
    }
}

fn component_count(accessor_type: &str) -> Option<usize> {
    match accessor_type {
        "SCALAR" => Some(1),
        "VEC2" => Some(2),
        "VEC3" => Some(3),
        "VEC4" => Some(4),
        "MAT2" => Some(4),
        "MAT3" => Some(9),
        "MAT4" => Some(16),
        _ => None,
    }
}

fn f32_at(bytes: &[u8], offset: usize) -> Option<f32> {
    bytes
        .get(offset..offset + 4)?
        .try_into()
        .ok()
        .map(f32::from_le_bytes)
}

fn q8(value: f32) -> Option<i32> {
    let scaled = value * 256.0;
    if !scaled.is_finite() || scaled < i32::MIN as f32 || scaled > i32::MAX as f32 {
        return None;
    }
    Some(scaled.round() as i32)
}

fn le_u32(bytes: &[u8], offset: usize) -> Result<u32, ()> {
    bytes
        .get(offset..offset + 4)
        .and_then(|value| value.try_into().ok())
        .map(u32::from_le_bytes)
        .ok_or(())
}

fn value_index(value: &serde_json::Value) -> Option<usize> {
    value_usize(value)
}

fn value_usize(value: &serde_json::Value) -> Option<usize> {
    usize::try_from(value.as_u64()?).ok()
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

#[cfg(test)]
mod glb_tests {
    use std::fs;

    use serde_json::Value;

    use super::{
        GlbLayout, glb_document, valid_animation, valid_glb, valid_gltf_references,
        valid_support_centered_scenes,
    };

    #[test]
    fn glb_validation_rejects_a_truncated_binary_chunk() {
        let mut bytes = fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../assets/vegetation/pine_near.glb"
        ))
        .expect("pine-near fixture exists");
        bytes.pop();

        assert!(!valid_pine(&bytes));
    }

    #[test]
    fn glb_validation_rejects_invalid_pine_layouts_and_geometry() {
        let fixture = pine_fixture();
        assert!(valid_pine(&fixture));

        let shared_scene_root = mutate_pine(&fixture, |document, _| {
            document["scenes"]
                .as_array_mut()
                .unwrap()
                .push(serde_json::json!({ "nodes": [0] }));
        });
        assert!(valid_pine(&shared_scene_root));

        let wrong_support_origin = mutate_pine(&fixture, |document, _| {
            document["asset"]["extras"]["origin"] = Value::from("base_center");
        });
        assert!(!valid_pine(&wrong_support_origin));

        let translated_node = mutate_pine(&fixture, |document, _| {
            document["nodes"][0]["translation"] = serde_json::json!([1.0, 0.0, 0.0]);
        });
        assert!(!valid_pine(&translated_node));

        let integer_identity_transform = mutate_pine(&fixture, |document, _| {
            document["nodes"][0]["translation"] = serde_json::json!([0, 0, 0]);
            document["nodes"][0]["rotation"] = serde_json::json!([0, 0, 0, 1]);
            document["nodes"][0]["scale"] = serde_json::json!([1, 1, 1]);
        });
        assert!(valid_pine(&integer_identity_transform));

        let self_referential_node = mutate_pine(&fixture, |document, _| {
            document["nodes"][0]["children"] = serde_json::json!([0]);
        });
        assert!(!valid_pine(&self_referential_node));

        let shared_child = mutate_pine(&fixture, |document, _| {
            let nodes = document["nodes"].as_array_mut().unwrap();
            nodes.push(serde_json::json!({}));
            nodes.push(serde_json::json!({}));
            nodes[0]["children"] = serde_json::json!([2]);
            nodes[1]["children"] = serde_json::json!([2]);
            document["scenes"][0]["nodes"] = serde_json::json!([0, 1]);
        });
        assert!(!valid_pine(&shared_child));

        let cycle_in_nondefault_scene = mutate_pine(&fixture, |document, _| {
            let nodes = document["nodes"].as_array_mut().unwrap();
            nodes.push(serde_json::json!({ "children": [2] }));
            nodes.push(serde_json::json!({ "children": [1] }));
            document["scenes"]
                .as_array_mut()
                .unwrap()
                .push(serde_json::json!({ "nodes": [1] }));
        });
        assert!(!valid_pine(&cycle_in_nondefault_scene));

        let orphaned_mesh = mutate_pine(&fixture, |document, _| {
            document["nodes"][0].as_object_mut().unwrap().remove("mesh");
        });
        assert!(!valid_pine(&orphaned_mesh));

        let omitted_triangle_mode = mutate_pine(&fixture, |document, _| {
            document["meshes"][0]["primitives"][0]
                .as_object_mut()
                .unwrap()
                .remove("mode");
        });
        assert!(valid_pine(&omitted_triangle_mode));

        let out_of_range_material = mutate_pine(&fixture, |document, _| {
            document["meshes"][0]["primitives"][0]["material"] = Value::from(1);
        });
        assert!(!valid_pine(&out_of_range_material));

        let split_bounds = split_pine_bounds(&fixture);
        assert!(valid_pine(&split_bounds));

        let misaligned_view = mutate_pine(&fixture, |document, _| {
            document["bufferViews"][0]["byteOffset"] = Value::from(1);
        });
        assert!(!valid_pine(&misaligned_view));

        let malformed_view_offset = mutate_pine(&fixture, |document, _| {
            document["bufferViews"][0]["byteOffset"] = Value::from("invalid");
        });
        assert!(!valid_pine(&malformed_view_offset));

        let malformed_accessor_offset = mutate_pine(&fixture, |document, _| {
            document["accessors"][0]["byteOffset"] = Value::from("invalid");
        });
        assert!(!valid_pine(&malformed_accessor_offset));

        let two_byte_aligned_index_view = mutate_pine(&fixture, |document, binary| {
            document["buffers"][0]["byteLength"] = Value::from(722);
            document["bufferViews"][4]["byteOffset"] = Value::from(626);
            binary.resize(722, 0);
            binary.copy_within(624..720, 626);
        });
        assert!(valid_pine(&two_byte_aligned_index_view));

        let accessor_overrun = mutate_pine(&fixture, |document, _| {
            document["accessors"][0]["count"] = Value::from(14);
        });
        assert!(!valid_pine(&accessor_overrun));

        let zero_count_index_accessor = mutate_pine(&fixture, |document, _| {
            document["accessors"][4]["count"] = Value::from(0);
        });
        assert!(!valid_pine(&zero_count_index_accessor));

        let zero_count_accessor = mutate_pine(&fixture, |document, _| {
            let mut accessor = document["accessors"][0].clone();
            accessor["count"] = Value::from(0);
            document["accessors"].as_array_mut().unwrap().push(accessor);
        });
        assert!(!valid_pine(&zero_count_accessor));

        let excess_binary_padding = mutate_pine(&fixture, |_, binary| {
            binary.resize(binary.len() + 4, 0);
        });
        assert!(!valid_pine(&excess_binary_padding));

        let index_overrun = mutate_pine(&fixture, |_, binary| {
            binary[624..626].copy_from_slice(&13_u16.to_le_bytes());
        });
        assert!(!valid_pine(&index_overrun));

        let missing_uv = mutate_pine(&fixture, |document, _| {
            document["meshes"][0]["primitives"][0]["attributes"]
                .as_object_mut()
                .unwrap()
                .remove("TEXCOORD_0");
        });
        assert!(!valid_pine(&missing_uv));

        let missing_normal = mutate_pine(&fixture, |document, _| {
            document["meshes"][0]["primitives"][0]["attributes"]
                .as_object_mut()
                .unwrap()
                .remove("NORMAL");
        });
        assert!(!valid_pine(&missing_normal));

        let missing_normal_map_tangent = mutate_pine(&fixture, |document, _| {
            document["materials"][0]["normalTexture"] = serde_json::json!({ "index": 0 });
            document["meshes"][0]["primitives"][0]["attributes"]
                .as_object_mut()
                .unwrap()
                .remove("TANGENT");
        });
        assert!(!valid_pine(&missing_normal_map_tangent));

        let texture_without_a_declared_texture = mutate_pine(&fixture, |document, _| {
            document["materials"][0]["normalTexture"] = serde_json::json!({ "index": 0 });
        });
        assert!(!valid_pine(&texture_without_a_declared_texture));

        let invalid_morph_target_accessor = mutate_pine(&fixture, |document, _| {
            document["meshes"][0]["primitives"][0]["targets"] =
                serde_json::json!([{ "POSITION": 999 }]);
        });
        assert!(!valid_pine(&invalid_morph_target_accessor));

        let non_finite_position = mutate_pine(&fixture, |_, binary| {
            binary[..4].copy_from_slice(&f32::NAN.to_le_bytes());
        });
        assert!(!valid_pine(&non_finite_position));

        let wrong_declared_bounds = mutate_pine(&fixture, |document, _| {
            document["accessors"][0]["max"][1] = Value::from(19);
        });
        assert!(!valid_pine(&wrong_declared_bounds));

        let over_triangle_limit = mutate_pine(&fixture, |document, binary| {
            document["buffers"][0]["byteLength"] = Value::from(72_630);
            document["bufferViews"][4]["byteLength"] = Value::from(72_006);
            document["accessors"][4]["count"] = Value::from(36_003);
            document["meshes"][0]["primitives"][0]["extras"]["triangle_count"] = Value::from(0);
            binary.resize(72_630, 0);
        });
        assert!(!valid_pine(&over_triangle_limit));
    }

    #[test]
    fn glb_validation_rejects_named_clips_without_animation_data() {
        let malformed_clip = mutate_pine(&pine_fixture(), |document, _| {
            document["animations"] = serde_json::json!([{ "name": "Idle" }]);
        });

        assert!(!valid_glb(
            &malformed_clip,
            12_000,
            &["PineNear".to_owned()],
            &["Idle".to_owned()],
            [-1_024, 0, -1_024],
            [1_024, 4_608, 1_024],
            [0, 0, 0],
        ));
    }

    #[test]
    fn explorer_animation_clips_have_valid_graphs_and_accessors() {
        let explorer = fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../assets/player/explorer.glb"
        ))
        .expect("explorer fixture exists");
        let (document, binary) = glb_document(&explorer).expect("explorer GLB document");
        let layout = GlbLayout::new(&document, binary).expect("explorer GLB layout");
        let nodes = document.get("nodes");

        assert!(
            document["animations"]
                .as_array()
                .expect("explorer animations")
                .iter()
                .all(|animation| valid_animation(animation, &layout, nodes))
        );
    }

    #[test]
    fn glb_validation_enforces_skin_accessor_semantics_and_joint_rest_poses() {
        let explorer = explorer_fixture();
        let (document, binary) = glb_document(&explorer).expect("explorer GLB document");
        let layout = GlbLayout::new(&document, binary).expect("explorer GLB layout");
        assert!(valid_gltf_references(&document, &layout));

        for (field, accessor) in [
            ("inverseBindMatrices", 3),
            ("JOINTS_0", 5),
            ("WEIGHTS_0", 4),
        ] {
            let mut malformed = document.clone();
            if field == "inverseBindMatrices" {
                malformed["skins"][0][field] = Value::from(accessor);
            } else {
                malformed["meshes"][0]["primitives"][0]["attributes"][field] =
                    Value::from(accessor);
            }
            assert!(
                !valid_gltf_references(&malformed, &layout),
                "{field} must use its required accessor type"
            );
        }

        let wrong_joint_count = mutate_explorer(&explorer, |document, _| {
            document["accessors"][4]["count"] = Value::from(7);
        });
        let (document, binary) = glb_document(&wrong_joint_count).expect("mutated explorer");
        let layout = GlbLayout::new(&document, binary).expect("valid mutated layout");
        assert!(
            !valid_gltf_references(&document, &layout),
            "joint attributes must have one entry per vertex"
        );

        let out_of_range_joint = mutate_explorer(&explorer, |_, binary| {
            binary[384] = 1;
        });
        let (document, binary) = glb_document(&out_of_range_joint).expect("mutated explorer");
        let layout = GlbLayout::new(&document, binary).expect("valid mutated layout");
        assert!(
            !valid_gltf_references(&document, &layout),
            "joint attribute values must index the declared skin joints"
        );

        let non_finite_bind_matrix = mutate_explorer(&explorer, |_, binary| {
            binary[616..620].copy_from_slice(&f32::NAN.to_le_bytes());
        });
        let (document, binary) = glb_document(&non_finite_bind_matrix).expect("mutated explorer");
        let layout = GlbLayout::new(&document, binary).expect("valid mutated layout");
        assert!(
            !valid_gltf_references(&document, &layout),
            "inverse bind matrices must be finite"
        );

        let multi_joint_rest_pose = mutate_explorer(&explorer, |document, binary| {
            document["nodes"]
                .as_array_mut()
                .expect("nodes")
                .push(serde_json::json!({
                    "translation": [0.0, 0.5, 0.0],
                }));
            document["nodes"][0]["children"] = serde_json::json!([2]);
            document["skins"][0]["joints"] = serde_json::json!([0, 2]);
            document["skins"][0]["inverseBindMatrices"] =
                Value::from(append_identity_bind_matrices(document, binary, 2));
        });
        let (document, binary) = glb_document(&multi_joint_rest_pose).expect("mutated explorer");
        let layout = GlbLayout::new(&document, binary).expect("valid multi-joint layout");
        assert!(valid_gltf_references(&document, &layout));
        assert!(
            valid_support_centered_scenes(
                &document,
                document["meshes"].as_array().expect("meshes").len()
            ),
            "joint-local rest poses do not move the support-centered mesh placement"
        );
    }

    #[test]
    fn glb_validation_enforces_skin_node_hierarchy_and_weight_set_semantics() {
        let explorer = explorer_fixture();
        let (document, binary) = glb_document(&explorer).expect("explorer GLB document");
        let layout = GlbLayout::new(&document, binary).expect("explorer GLB layout");
        assert!(valid_gltf_references(&document, &layout));

        let no_mesh = mutate_explorer(&explorer, |document, _| {
            document["nodes"][1]
                .as_object_mut()
                .expect("node")
                .remove("mesh");
        });
        assert_invalid_gltf_references(&no_mesh, "skinned nodes require meshes");

        let extra_bind_matrix = mutate_explorer(&explorer, |document, binary| {
            document["skins"][0]["inverseBindMatrices"] =
                Value::from(append_identity_bind_matrices(document, binary, 2));
        });
        assert_valid_gltf_references(&extra_bind_matrix, "skins permit extra bind matrices");

        let duplicate_joint = mutate_explorer(&explorer, |document, binary| {
            document["skins"][0]["joints"] = serde_json::json!([0, 0]);
            document["skins"][0]["inverseBindMatrices"] =
                Value::from(append_identity_bind_matrices(document, binary, 2));
        });
        assert_invalid_gltf_references(&duplicate_joint, "skin joints must be unique");

        let unrelated_skeleton = mutate_explorer(&explorer, |document, _| {
            document["nodes"]
                .as_array_mut()
                .expect("nodes")
                .push(serde_json::json!({}));
            document["skins"][0]["skeleton"] = Value::from(2);
        });
        assert_invalid_gltf_references(
            &unrelated_skeleton,
            "the skeleton node must be a common joint ancestor",
        );

        for field in ["JOINTS_1", "WEIGHTS_1"] {
            let unmatched_set = mutate_explorer(&explorer, |document, _| {
                document["meshes"][0]["primitives"][0]["attributes"][field] =
                    Value::from(if field == "JOINTS_1" { 4 } else { 5 });
            });
            assert_invalid_gltf_references(
                &unmatched_set,
                "{field} requires its matching skin attribute",
            );
        }

        let negative_weights = mutate_explorer(&explorer, |_, binary| {
            binary[layout.accessor(5).expect("weights accessor").offset..][..4]
                .copy_from_slice(&(-0.25_f32).to_le_bytes());
        });
        assert_invalid_gltf_references(&negative_weights, "weights must not be negative");

        let invalid_quantized_weight_sum = mutate_explorer(&explorer, |document, binary| {
            let count = layout.accessor(5).expect("weights accessor").count;
            let accessor = append_normalized_u8_weights(document, binary, count, [128, 128, 0, 0]);
            document["meshes"][0]["primitives"][0]["attributes"]["WEIGHTS_0"] =
                Value::from(accessor);
        });
        assert_invalid_gltf_references(
            &invalid_quantized_weight_sum,
            "normalized integer weights must sum to one",
        );
    }

    #[test]
    fn glb_validation_rejects_unreferenced_animation_samplers_without_accessors() {
        let explorer = fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../assets/player/explorer.glb"
        ))
        .expect("explorer fixture exists");
        let (document, binary) = glb_document(&explorer).expect("explorer GLB document");
        let layout = GlbLayout::new(&document, binary).expect("explorer GLB layout");
        let animation = &document["animations"]
            .as_array()
            .expect("explorer animations")[0];

        assert!(valid_animation(animation, &layout, document.get("nodes")));
        assert!(valid_gltf_references(&document, &layout));

        for sampler in [
            serde_json::json!({ "input": 999, "output": 1 }),
            serde_json::json!({ "input": 0, "output": 999 }),
            serde_json::json!({ "input": 0 }),
        ] {
            let mut malformed = document.clone();
            malformed["animations"][0]["samplers"]
                .as_array_mut()
                .expect("animation samplers")
                .push(sampler);

            assert!(
                !valid_animation(&malformed["animations"][0], &layout, malformed.get("nodes")),
                "all declared samplers require in-range input and output accessors"
            );
            assert!(
                !valid_gltf_references(&malformed, &layout),
                "unreferenced animation samplers must still be validated"
            );
        }
    }

    #[test]
    fn glb_validation_rejects_unreferenced_animation_sampler_semantic_errors() {
        let explorer = fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../assets/player/explorer.glb"
        ))
        .expect("explorer fixture exists");
        let (document, binary) = glb_document(&explorer).expect("explorer GLB document");
        let layout = GlbLayout::new(&document, binary).expect("explorer GLB layout");
        let sampler = document["animations"][0]["samplers"][0].clone();

        for (field, value) in [
            ("interpolation", Value::from("BEZIER")),
            ("interpolation", Value::from(7)),
            (
                "input",
                Value::from(
                    layout
                        .accessors
                        .iter()
                        .position(|accessor| !accessor.is_f32_type("SCALAR"))
                        .expect("explorer has a non-time accessor"),
                ),
            ),
            (
                "output",
                Value::from(
                    layout
                        .accessors
                        .iter()
                        .position(|accessor| accessor.component_type != 5126)
                        .expect("explorer has a non-float accessor"),
                ),
            ),
        ] {
            let mut malformed = document.clone();
            let mut malformed_sampler = sampler.clone();
            malformed_sampler[field] = value;
            malformed["animations"][0]["samplers"]
                .as_array_mut()
                .expect("animation samplers")
                .push(malformed_sampler);

            assert!(
                !valid_animation(&malformed["animations"][0], &layout, malformed.get("nodes")),
                "unreferenced samplers must validate their {field}"
            );
        }

        for field in ["input", "output"] {
            let mut malformed = document.clone();
            let mut malformed_binary = binary.to_vec();
            let accessor_index =
                append_non_finite_scalar_accessor(&mut malformed, &mut malformed_binary);
            let mut malformed_sampler = sampler.clone();
            malformed_sampler[field] = Value::from(accessor_index);
            malformed["animations"][0]["samplers"]
                .as_array_mut()
                .expect("animation samplers")
                .push(malformed_sampler);
            let malformed_layout =
                GlbLayout::new(&malformed, &malformed_binary).expect("valid malformed layout");

            assert!(
                !valid_animation(
                    &malformed["animations"][0],
                    &malformed_layout,
                    malformed.get("nodes")
                ),
                "unreferenced sampler {field} must reject non-finite data"
            );
        }
    }

    #[test]
    fn glb_validation_ignores_aligned_unknown_chunks_after_binary() {
        let mut extensible_glb = pine_fixture();
        extensible_glb.extend_from_slice(&4_u32.to_le_bytes());
        extensible_glb.extend_from_slice(&0x1234_5678_u32.to_le_bytes());
        extensible_glb.extend_from_slice(b"test");
        let total_length = u32::try_from(extensible_glb.len()).unwrap();
        extensible_glb[8..12].copy_from_slice(&total_length.to_le_bytes());

        assert!(valid_pine(&extensible_glb));
    }

    fn valid_pine(bytes: &[u8]) -> bool {
        valid_glb(
            bytes,
            12_000,
            &["PineNear".to_owned()],
            &[],
            [-1_024, 0, -1_024],
            [1_024, 4_608, 1_024],
            [0, 0, 0],
        )
    }

    fn pine_fixture() -> Vec<u8> {
        fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../assets/vegetation/pine_near.glb"
        ))
        .expect("pine-near fixture exists")
    }

    fn explorer_fixture() -> Vec<u8> {
        fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../assets/player/explorer.glb"
        ))
        .expect("explorer fixture exists")
    }

    fn mutate_explorer(
        mutate_from: &[u8],
        mutate: impl FnOnce(&mut Value, &mut Vec<u8>),
    ) -> Vec<u8> {
        mutate_pine(mutate_from, mutate)
    }

    fn append_identity_bind_matrices(
        document: &mut Value,
        binary: &mut Vec<u8>,
        count: usize,
    ) -> usize {
        let offset = binary.len();
        for _ in 0..count {
            for value in [
                1.0_f32, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ] {
                binary.extend_from_slice(&value.to_le_bytes());
            }
        }
        document["buffers"][0]["byteLength"] = Value::from(binary.len());
        let views = document["bufferViews"]
            .as_array_mut()
            .expect("buffer views");
        views.push(serde_json::json!({
            "buffer": 0,
            "byteOffset": offset,
            "byteLength": count * 64,
        }));
        let view_index = views.len() - 1;
        let accessors = document["accessors"].as_array_mut().expect("accessors");
        accessors.push(serde_json::json!({
            "bufferView": view_index,
            "componentType": 5126,
            "count": count,
            "type": "MAT4",
        }));
        accessors.len() - 1
    }

    fn append_normalized_u8_weights(
        document: &mut Value,
        binary: &mut Vec<u8>,
        count: u32,
        weights: [u8; 4],
    ) -> usize {
        let offset = binary.len();
        for _ in 0..count {
            binary.extend_from_slice(&weights);
        }
        document["buffers"][0]["byteLength"] = Value::from(binary.len());
        let views = document["bufferViews"]
            .as_array_mut()
            .expect("buffer views");
        views.push(serde_json::json!({
            "buffer": 0,
            "byteOffset": offset,
            "byteLength": usize::try_from(count).expect("weight count") * 4,
        }));
        let view_index = views.len() - 1;
        let accessors = document["accessors"].as_array_mut().expect("accessors");
        accessors.push(serde_json::json!({
            "bufferView": view_index,
            "componentType": 5121,
            "normalized": true,
            "count": count,
            "type": "VEC4",
        }));
        accessors.len() - 1
    }

    fn assert_valid_gltf_references(bytes: &[u8], message: &str) {
        let (document, binary) = glb_document(bytes).expect("mutated explorer");
        let layout = GlbLayout::new(&document, binary).expect("valid mutated layout");
        assert!(valid_gltf_references(&document, &layout), "{message}");
    }

    fn assert_invalid_gltf_references(bytes: &[u8], message: &str) {
        let (document, binary) = glb_document(bytes).expect("mutated explorer");
        let layout = GlbLayout::new(&document, binary).expect("valid mutated layout");
        assert!(!valid_gltf_references(&document, &layout), "{message}");
    }

    fn mutate_pine(mutate_from: &[u8], mutate: impl FnOnce(&mut Value, &mut Vec<u8>)) -> Vec<u8> {
        let json_length = u32::from_le_bytes(mutate_from[12..16].try_into().unwrap()) as usize;
        let binary_offset = 20 + json_length + 8;
        let mut document = serde_json::from_slice(&mutate_from[20..20 + json_length]).unwrap();
        let mut binary = mutate_from[binary_offset..].to_vec();
        mutate(&mut document, &mut binary);
        glb_bytes(document, binary)
    }

    fn split_pine_bounds(mutate_from: &[u8]) -> Vec<u8> {
        mutate_pine(mutate_from, |document, binary| {
            document["accessors"][0]["count"] = Value::from(8);
            document["accessors"][0]["min"] = serde_json::json!([-0.45, 0.0, -0.45]);
            document["accessors"][0]["max"] = serde_json::json!([0.45, 10.0, 0.45]);
            document["accessors"][1]["count"] = Value::from(8);
            document["accessors"][2]["count"] = Value::from(8);
            document["accessors"][3]["count"] = Value::from(8);
            document["accessors"][4]["count"] = Value::from(3);
            binary[624..630].copy_from_slice(&[0, 0, 1, 0, 2, 0]);

            let accessors = document["accessors"].as_array_mut().unwrap();
            for (source, offset) in [(0, 96), (1, 96), (2, 64), (3, 128)] {
                let mut accessor = accessors[source].clone();
                accessor["count"] = Value::from(5);
                accessor["byteOffset"] = Value::from(offset);
                accessors.push(accessor);
            }
            accessors[5]["min"] = serde_json::json!([-4.0, 10.0, -4.0]);
            accessors[5]["max"] = serde_json::json!([4.0, 18.0, 4.0]);

            let second_primitive = serde_json::json!({
                "attributes": { "POSITION": 5, "NORMAL": 6, "TEXCOORD_0": 7, "TANGENT": 8 },
                "indices": 4,
                "material": 0,
                "mode": 4,
            });
            document["meshes"][0]["primitives"]
                .as_array_mut()
                .unwrap()
                .push(second_primitive);
        })
    }

    fn glb_bytes(document: Value, mut binary: Vec<u8>) -> Vec<u8> {
        let mut json = serde_json::to_vec(&document).unwrap();
        while !json.len().is_multiple_of(4) {
            json.push(b' ');
        }
        while !binary.len().is_multiple_of(4) {
            binary.push(0);
        }
        let mut bytes = Vec::with_capacity(28 + json.len() + binary.len());
        bytes.extend_from_slice(b"glTF");
        bytes.extend_from_slice(&2_u32.to_le_bytes());
        let total_length = u32::try_from(28 + json.len() + binary.len()).unwrap();
        bytes.extend_from_slice(&total_length.to_le_bytes());
        bytes.extend_from_slice(&(json.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&0x4E4F_534Au32.to_le_bytes());
        bytes.extend_from_slice(&json);
        bytes.extend_from_slice(&(binary.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&0x004E_4942u32.to_le_bytes());
        bytes.extend_from_slice(&binary);
        bytes
    }

    fn append_non_finite_scalar_accessor(document: &mut Value, binary: &mut Vec<u8>) -> usize {
        let offset = binary.len();
        binary.extend_from_slice(&f32::NAN.to_le_bytes());
        document["buffers"][0]["byteLength"] = Value::from(binary.len());

        let views = document["bufferViews"]
            .as_array_mut()
            .expect("buffer views");
        views.push(serde_json::json!({
            "buffer": 0,
            "byteOffset": offset,
            "byteLength": 4,
        }));
        let view_index = views.len() - 1;
        let accessors = document["accessors"].as_array_mut().expect("accessors");
        accessors.push(serde_json::json!({
            "bufferView": view_index,
            "componentType": 5126,
            "count": 1,
            "type": "SCALAR",
        }));
        accessors.len() - 1
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
