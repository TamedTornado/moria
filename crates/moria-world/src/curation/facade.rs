//! Development-only pure manifest curation API.

use std::{error::Error, fmt};

use serde::Serialize;

use crate::{
    CuratedManifest, ObjectIndexConfig, RegionConfig, SparseVoxelStamp, build_object_index,
    validate_region_config,
};

use super::generate::{generate_manifest, validate_manifest_without_stamp};
use super::stress::{CurationStressTarget, select_radius_three_stress_target};

/// A typed curation failure that does not expose implementation state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CurationError {
    SeedMismatch { requested: u64, configured: u64 },
    InvalidRegionConfig(String),
    InvalidRuinStamp(String),
    Manifest(String),
}

impl fmt::Display for CurationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SeedMismatch {
                requested,
                configured,
            } => write!(
                formatter,
                "requested seed {requested} does not match configured seed {configured}"
            ),
            Self::InvalidRegionConfig(error) => {
                write!(formatter, "invalid region configuration: {error}")
            }
            Self::InvalidRuinStamp(error) => write!(formatter, "invalid ruin stamp: {error}"),
            Self::Manifest(error) => write!(formatter, "invalid curated manifest: {error}"),
        }
    }
}

impl Error for CurationError {}

/// Deterministic facts from validating a curated manifest.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CurationReport {
    pub placement_count: u32,
    pub retained_index_bytes: u64,
    pub radius_three_target: CurationStressTarget,
    pub dependency_coordinate_allocation_bytes: u64,
}

/// Derives a deterministic manifest from immutable input values.
pub fn derive_manifest(
    seed: u64,
    config: &RegionConfig,
    stamp: &SparseVoxelStamp,
) -> Result<CuratedManifest, CurationError> {
    if seed != config.seed {
        return Err(CurationError::SeedMismatch {
            requested: seed,
            configured: config.seed,
        });
    }
    validate_region_config(config)
        .map_err(|error| CurationError::InvalidRegionConfig(error.to_string()))?;
    stamp
        .validate()
        .map_err(|error| CurationError::InvalidRuinStamp(error.to_string()))?;
    let config_bytes = ron::ser::to_string(config)
        .map_err(|error| CurationError::InvalidRegionConfig(error.to_string()))?;
    let stamp_bytes = ron::ser::to_string(stamp)
        .map_err(|error| CurationError::InvalidRuinStamp(error.to_string()))?;
    derive_manifest_from_bytes(config_bytes.as_bytes(), stamp_bytes.as_bytes())
}

/// Derives a manifest from the authoritative configuration and stamp bytes.
pub fn derive_manifest_from_bytes(
    config_bytes: &[u8],
    stamp_bytes: &[u8],
) -> Result<CuratedManifest, CurationError> {
    let config: RegionConfig = ron::de::from_bytes(config_bytes)
        .map_err(|error| CurationError::InvalidRegionConfig(error.to_string()))?;
    validate_region_config(&config)
        .map_err(|error| CurationError::InvalidRegionConfig(error.to_string()))?;
    let stamp: SparseVoxelStamp = ron::de::from_bytes(stamp_bytes)
        .map_err(|error| CurationError::InvalidRuinStamp(error.to_string()))?;
    stamp
        .validate()
        .map_err(|error| CurationError::InvalidRuinStamp(error.to_string()))?;
    generate_manifest(config_bytes, stamp_bytes)
        .map_err(|error| CurationError::Manifest(error.to_string()))
}

/// Validates a manifest without accessing live world, delta, or render state.
pub fn validate_manifest(
    config: &RegionConfig,
    manifest: &CuratedManifest,
) -> Result<CurationReport, CurationError> {
    validate_region_config(config)
        .map_err(|error| CurationError::InvalidRegionConfig(error.to_string()))?;
    if manifest.seed != config.seed {
        return Err(CurationError::SeedMismatch {
            requested: manifest.seed,
            configured: config.seed,
        });
    }
    validate_manifest_without_stamp(manifest, config)
        .map_err(|error| CurationError::Manifest(error.to_string()))?;
    let index = build_object_index(
        &manifest.objects,
        &ObjectIndexConfig::from_configs(&config.objects, 1_024),
    )
    .map_err(|error| CurationError::Manifest(error.to_string()))?;
    let radius_three_target = select_radius_three_stress_target(&index).ok_or_else(|| {
        CurationError::Manifest("manifest has no legal radius-3 m forest stress target".to_owned())
    })?;
    if radius_three_target.broad_dependency_candidates
        > config.objects.max_edit_dependency_candidates
        || radius_three_target.exact_dependency_ids
            > u16::from(config.objects.max_affected_objects_per_edit)
        || index.dependency_coordinate_allocation_bytes() != 0
    {
        return Err(CurationError::Manifest(
            "radius-3 m forest stress target exceeds the index contract".to_owned(),
        ));
    }
    Ok(CurationReport {
        placement_count: u32::try_from(manifest.objects.len())
            .map_err(|_| CurationError::Manifest("placement count exceeds u32".to_owned()))?,
        retained_index_bytes: index.retained_bytes(),
        radius_three_target,
        dependency_coordinate_allocation_bytes: index.dependency_coordinate_allocation_bytes(),
    })
}
