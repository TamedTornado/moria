//! Feasibility evidence is deliberately separate from runtime allocation telemetry.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::config::PRODUCT_ONE_SEED;
use crate::{WorldIdentity, WorldPointQ8};

const FOREST_SCHEMA: &str = "moria-product-one-forest-feasibility";
const MUTATION_SCHEMA: &str = "moria-product-one-mutation-feasibility";
const M4_ACCEPTANCE_LABEL: &str = "m4-mac-mini-32gb";
const FOREST_MINIMUM_AREA_M2: u32 = 120_000;
const MINIMUM_TREE_COUNT: u32 = FOREST_MINIMUM_AREA_M2 / 25;
const MINIMUM_BUSH_COUNT: u32 = FOREST_MINIMUM_AREA_M2 * 450 / 10_000;
const MINIMUM_BOULDER_COUNT: u32 = FOREST_MINIMUM_AREA_M2 * 24 / 10_000;
const MINIMUM_STUMP_COUNT: u32 = FOREST_MINIMUM_AREA_M2 * 14 / 10_000;
const MINIMUM_ROCK_COUNT: u32 = FOREST_MINIMUM_AREA_M2 * 90 / 10_000;
const MINIMUM_BIRCH_COUNT: u32 = MINIMUM_TREE_COUNT * 55 / 100;
const MINIMUM_PINE_COUNT: u32 = MINIMUM_TREE_COUNT * 45 / 100;
const REQUIRED_FOREST_OBJECT_COUNTS: [(&str, u32); 7] = [
    ("boulder", MINIMUM_BOULDER_COUNT),
    ("bush", MINIMUM_BUSH_COUNT),
    ("rock", MINIMUM_ROCK_COUNT),
    ("ruin", 1),
    ("stump", MINIMUM_STUMP_COUNT),
    ("tree-a", MINIMUM_BIRCH_COUNT),
    ("tree-b", MINIMUM_PINE_COUNT),
];
const REQUIRED_FOREST_SPECIES_COUNTS: [(&str, u32); 2] =
    [("birch", MINIMUM_BIRCH_COUNT), ("pine", MINIMUM_PINE_COUNT)];
const REQUIRED_CANOPY_RANGE_BINS: [&str; 4] =
    ["birch-lower", "birch-upper", "pine-lower", "pine-upper"];
const MINIMUM_CANOPY_RANGE_BIN_COUNT: u32 = 16;
const REQUIRED_MUTATION_STAGES: [&str; 18] = [
    "admission",
    "schedule",
    "edit-stage",
    "commit",
    "dirty-discovery",
    "dependency-eligibility",
    "snapshot",
    "terrain-mesh",
    "object-mesh",
    "seams",
    "dressing-remove",
    "dressing-install",
    "bevy-install",
    "primary-ready",
    "render-extract",
    "gpu-upload",
    "render-queue",
    "reconciliation",
];
const NONEMPTY_MUTATION_STAGES: [&str; 11] = [
    "admission",
    "schedule",
    "edit-stage",
    "commit",
    "snapshot",
    "terrain-mesh",
    "primary-ready",
    "render-extract",
    "gpu-upload",
    "render-queue",
    "reconciliation",
];

/// Why an evidence object cannot be accepted as a complete gate artifact.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReportValidationError {
    Schema,
    Timestamp,
    PassedFlagMismatch,
    FailureReasons,
    Identity { field: &'static str },
    Missing { field: &'static str },
    NonFinite { field: &'static str },
    Inconsistent { field: &'static str },
    Limit { field: &'static str },
    Serialization,
}

impl core::fmt::Display for ReportValidationError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for ReportValidationError {}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuildProfile {
    pub cargo_profile: String,
    pub git_commit: String,
    pub rustc_version: String,
}

impl BuildProfile {
    pub fn validate_complete(&self) -> Result<(), ReportValidationError> {
        if blank(&self.cargo_profile)
            || !is_git_commit(&self.git_commit)
            || blank(&self.rustc_version)
        {
            return Err(ReportValidationError::Identity { field: "build" });
        }
        Ok(())
    }

    pub fn validate_release(&self) -> Result<(), ReportValidationError> {
        self.validate_complete()?;
        if self.cargo_profile != "release" {
            return Err(ReportValidationError::Identity { field: "build" });
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MachineProfile {
    pub profile_id_sha256: String,
    pub os_name: String,
    pub os_version: String,
    pub architecture: String,
    pub cpu_model: String,
    pub logical_cores: u32,
    pub total_physical_memory_bytes: u64,
    pub gpu_adapter_name: String,
    pub gpu_vendor: u32,
    pub gpu_device: u32,
    pub gpu_device_class: String,
    pub wgpu_backend: String,
    pub driver: Option<String>,
    pub driver_metadata_available: bool,
    pub memory_architecture: String,
    pub acceptance_label: String,
}

impl MachineProfile {
    pub fn validate_complete(&self) -> Result<(), ReportValidationError> {
        let required = [
            &self.os_name,
            &self.os_version,
            &self.architecture,
            &self.cpu_model,
            &self.gpu_adapter_name,
            &self.gpu_device_class,
            &self.wgpu_backend,
            &self.memory_architecture,
            &self.acceptance_label,
        ];
        if required.iter().any(|value| blank(value))
            || self.logical_cores == 0
            || self.total_physical_memory_bytes == 0
            || !is_sha256(&self.profile_id_sha256)
            || self.driver_metadata_available != self.driver.is_some()
            || self.driver.as_ref().is_some_and(|driver| blank(driver))
            || !matches!(self.memory_architecture.as_str(), "unified" | "discrete")
            || !matches!(self.gpu_device_class.as_str(), "integrated" | "discrete")
            || !matches!(self.wgpu_backend.as_str(), "metal" | "vulkan")
        {
            return Err(ReportValidationError::Identity { field: "machine" });
        }
        Ok(())
    }

    pub fn validate_m4_acceptance(&self) -> Result<(), ReportValidationError> {
        self.validate_complete()?;
        validate_m4_machine(self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Distribution {
    pub min: f64,
    pub p50: f64,
    pub p95: f64,
    pub p99: f64,
    pub max: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ObjectIndexEvidence {
    pub validation_ms: f64,
    pub build_ms: f64,
    pub retained_bytes: u64,
    pub retained_byte_categories: BTreeMap<String, u64>,
    pub placement_records: u32,
    pub dependency_grid_entries: u32,
    pub sample_grid_entries: u32,
    pub max_dependency_cell_entries: u16,
    pub max_sample_cell_entries: u8,
    pub max_horizon_tree_members_per_cell: u16,
    pub max_edit_candidates: u16,
    pub max_edit_affected_objects: u8,
    pub max_dependency_bricks: u16,
    pub dependency_coordinate_allocation_bytes: u64,
}

impl ObjectIndexEvidence {
    pub fn validate_measured(&self) -> Result<(), ReportValidationError> {
        validate_forest_object_index(self, false)
    }

    pub fn validate_complete(&self) -> Result<(), ReportValidationError> {
        validate_forest_object_index(self, true)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorstEditTargetEvidence {
    pub center: WorldPointQ8,
    pub broad_candidates: u16,
    pub exact_dependency_ids: u16,
    pub dependency_bricks: u16,
    pub tie_break_rank: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ForestFeasibilityReport {
    pub schema: String,
    pub timestamp_utc: String,
    pub passed: bool,
    pub failure_reasons: Vec<String>,
    pub build: BuildProfile,
    pub world: WorldIdentity,
    pub manifest_sha256: String,
    pub machine: MachineProfile,
    pub forest_area_m2: u32,
    pub eligible_land_area_m2: u32,
    pub object_counts: BTreeMap<String, u32>,
    pub required_object_counts: BTreeMap<String, u32>,
    pub species_counts: BTreeMap<String, u32>,
    pub minimum_tree_spacing_q8: u32,
    pub canopy_min_q8: u16,
    pub canopy_max_q8: u16,
    pub canopy_range_bins: BTreeMap<String, u32>,
    pub minimum_route_clearance_q8: u32,
    pub overlap_conflicts: u32,
    pub first_conflict: Option<String>,
    pub object_index: ObjectIndexEvidence,
    pub worst_edit_target: WorstEditTargetEvidence,
}

impl ForestFeasibilityReport {
    pub fn validate(&self) -> Result<(), ReportValidationError> {
        validate_header(FOREST_SCHEMA, GateHeader::from_forest(self))?;
        validate_forest_object_index(&self.object_index, self.passed)?;
        validate_positive_named_counts(&self.object_counts)?;
        validate_positive_named_counts(&self.required_object_counts)?;
        validate_positive_named_counts(&self.species_counts)?;
        validate_positive_named_counts(&self.canopy_range_bins)?;
        validate_world_identity(&self.world)?;
        if self.forest_area_m2 == 0
            || self.eligible_land_area_m2 == 0
            || self.forest_area_m2 > self.eligible_land_area_m2
            || self.object_counts.is_empty()
            || self.required_object_counts.is_empty()
            || self.species_counts.is_empty()
            || self.canopy_range_bins.is_empty()
            || self.minimum_tree_spacing_q8 == 0
            || self.minimum_route_clearance_q8 == 0
        {
            return Err(ReportValidationError::Missing {
                field: "forest measurement",
            });
        }
        if self.passed && self.forest_area_m2 < FOREST_MINIMUM_AREA_M2 {
            return Err(ReportValidationError::Missing {
                field: "forest measurement",
            });
        }
        if self.passed {
            if !matches_required_counts(
                &self.required_object_counts,
                &REQUIRED_FOREST_OBJECT_COUNTS,
            ) || REQUIRED_FOREST_OBJECT_COUNTS
                .iter()
                .any(|(kind, required)| self.object_counts.get(*kind).unwrap_or(&0) < required)
            {
                return Err(ReportValidationError::Inconsistent {
                    field: "forest object counts",
                });
            }
            if !matches_required_counts(&self.species_counts, &REQUIRED_FOREST_SPECIES_COUNTS)
                || self.species_counts.values().copied().sum::<u32>()
                    != self.object_counts.get("tree-a").unwrap_or(&0)
                        + self.object_counts.get("tree-b").unwrap_or(&0)
            {
                return Err(ReportValidationError::Inconsistent {
                    field: "forest species counts",
                });
            }
            if self.canopy_range_bins.len() != REQUIRED_CANOPY_RANGE_BINS.len()
                || REQUIRED_CANOPY_RANGE_BINS.iter().any(|bin| {
                    self.canopy_range_bins.get(*bin).unwrap_or(&0) < &MINIMUM_CANOPY_RANGE_BIN_COUNT
                })
            {
                return Err(ReportValidationError::Inconsistent {
                    field: "canopy range bins",
                });
            }
            if self
                .object_counts
                .values()
                .copied()
                .map(u64::from)
                .sum::<u64>()
                != u64::from(self.object_index.placement_records)
            {
                return Err(ReportValidationError::Inconsistent {
                    field: "placement records",
                });
            }
        }
        if self.canopy_min_q8 > self.canopy_max_q8 {
            return Err(ReportValidationError::Inconsistent {
                field: "canopy range",
            });
        }
        if self.passed && (self.canopy_min_q8 < 2 * 256 || self.canopy_max_q8 > 4 * 256) {
            return Err(ReportValidationError::Limit {
                field: "canopy range",
            });
        }
        if self.passed
            && (self.minimum_tree_spacing_q8 < 5 * 256 || self.minimum_route_clearance_q8 < 3 * 256)
        {
            return Err(ReportValidationError::Limit {
                field: "forest clearance",
            });
        }
        if self.passed && self.overlap_conflicts != 0 {
            return Err(ReportValidationError::Inconsistent {
                field: "overlap conflicts",
            });
        }
        if self.overlap_conflicts == 0 && self.first_conflict.is_some()
            || self.overlap_conflicts > 0
                && self
                    .first_conflict
                    .as_deref()
                    .unwrap_or_default()
                    .is_empty()
        {
            return Err(ReportValidationError::Inconsistent {
                field: "first_conflict",
            });
        }
        validate_worst_target(&self.worst_edit_target, &self.world, self.passed)
    }

    pub fn to_canonical_json(&self) -> Result<String, ReportValidationError> {
        self.validate()?;
        serde_json::to_string(self).map_err(|_| ReportValidationError::Serialization)
    }

    pub fn from_json(json: &str) -> Result<Self, ReportValidationError> {
        let report: Self =
            serde_json::from_str(json).map_err(|_| ReportValidationError::Serialization)?;
        let value: serde_json::Value =
            serde_json::from_str(json).map_err(|_| ReportValidationError::Serialization)?;
        if !json_object_has_keys(&value, &["first_conflict"])
            || !json_nested_object_has_keys(&value, "machine", &["driver"])
        {
            return Err(ReportValidationError::Missing { field: "JSON key" });
        }
        report.validate()?;
        Ok(report)
    }

    /// SHA-256 of the validated canonical JSON bytes consumed by Gate F2.
    pub fn canonical_sha256(&self) -> Result<String, ReportValidationError> {
        let json = self.to_canonical_json()?;
        Ok(format!("{:x}", Sha256::digest(json.as_bytes())))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MutationWorkloadRole {
    InteractiveCarve,
    ColonyVolume,
    CatastrophicCarve,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MutationWorkloadEvidence {
    pub role: MutationWorkloadRole,
    pub request_count: u32,
    pub submitted_frame: u64,
    pub first_committed_frame: u64,
    pub final_reconciled_frame: u64,
    pub admission_ms: Distribution,
    pub first_commit_ms: Distribution,
    pub primary_ready_ms: Distribution,
    pub reconciliation_ms: Distribution,
    pub changed_bricks_per_second: f64,
    pub maximum_runnable_wait_ms: f64,
    pub maximum_frame_ms: f64,
    pub traversable: bool,
    pub changed_voxels: u64,
    pub changed_bricks: u32,
    pub committed_batches: u32,
    pub stage_timings_ms: BTreeMap<String, f64>,
    pub stage_counts: BTreeMap<String, u64>,
    pub barrier_expected_items: u32,
    pub barrier_renderer_ready_items: u32,
    pub horizon_partition_checked: bool,
    pub horizon_excluded_base_cards: u16,
    pub horizon_derived_records: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QueryCostEvidence {
    pub sample_counts: BTreeMap<String, u32>,
    pub cold_inactive_calls: BTreeMap<String, Distribution>,
    pub frame_critical_calls: BTreeMap<String, Distribution>,
    pub normal_bundle_ms: Distribution,
    pub column_ms: Distribution,
    pub diagnostic_metadata_page_ms: Distribution,
    pub diagnostic_cells_page_ms: Distribution,
    pub observed_work_maxima: BTreeMap<String, u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MutationFeasibilityReport {
    pub schema: String,
    pub timestamp_utc: String,
    pub passed: bool,
    pub failure_reasons: Vec<String>,
    pub build: BuildProfile,
    pub world: WorldIdentity,
    pub manifest_sha256: String,
    pub forest_report_sha256: String,
    pub machine: MachineProfile,
    pub resolution: [u32; 2],
    pub backend: String,
    pub cold_start_ms: f64,
    pub workloads: Vec<MutationWorkloadEvidence>,
    pub query_costs: QueryCostEvidence,
}

impl MutationFeasibilityReport {
    pub fn validate(&self) -> Result<(), ReportValidationError> {
        validate_header(MUTATION_SCHEMA, GateHeader::from_mutation(self))?;
        if !is_sha256(&self.forest_report_sha256) {
            return Err(ReportValidationError::Identity {
                field: "forest_report_sha256",
            });
        }
        if self.resolution != [2560, 1440] || self.backend != "metal" {
            return Err(ReportValidationError::Identity {
                field: "mutation display",
            });
        }
        validate_world_identity(&self.world)?;
        finite_positive(self.cold_start_ms, "cold_start_ms")?;
        if self.passed && self.cold_start_ms >= 5_000.0 {
            return Err(ReportValidationError::Limit {
                field: "cold_start_ms",
            });
        }
        if self.workloads.len() != 3 {
            return Err(ReportValidationError::Missing { field: "workloads" });
        }
        let expected = [
            MutationWorkloadRole::InteractiveCarve,
            MutationWorkloadRole::ColonyVolume,
            MutationWorkloadRole::CatastrophicCarve,
        ];
        for (workload, role) in self.workloads.iter().zip(expected) {
            if workload.role != role {
                return Err(ReportValidationError::Inconsistent {
                    field: "workload roles",
                });
            }
            validate_workload(workload, self.passed)?;
        }
        if self.passed
            && ["seams", "bevy-install"].iter().any(|stage| {
                self.workloads
                    .iter()
                    .all(|workload| workload.stage_counts[*stage] == 0)
            })
        {
            return Err(ReportValidationError::Missing {
                field: "aggregate mutation stage count",
            });
        }
        if self.passed
            && ["object-mesh", "dressing-remove", "dressing-install"]
                .iter()
                .any(|stage| self.workloads[2].stage_counts[*stage] == 0)
        {
            return Err(ReportValidationError::Missing {
                field: "catastrophic mutation stage count",
            });
        }
        validate_query_costs(&self.query_costs, self.passed)
    }

    pub fn to_canonical_json(&self) -> Result<String, ReportValidationError> {
        self.validate()?;
        serde_json::to_string(self).map_err(|_| ReportValidationError::Serialization)
    }

    pub fn from_json(json: &str) -> Result<Self, ReportValidationError> {
        let report: Self =
            serde_json::from_str(json).map_err(|_| ReportValidationError::Serialization)?;
        let value: serde_json::Value =
            serde_json::from_str(json).map_err(|_| ReportValidationError::Serialization)?;
        if !json_nested_object_has_keys(&value, "machine", &["driver"]) {
            return Err(ReportValidationError::Missing { field: "JSON key" });
        }
        report.validate()?;
        Ok(report)
    }

    /// Verifies that F2 consumes the exact passing F1 artifact and gate identity.
    pub fn validate_against_forest(
        &self,
        forest: &ForestFeasibilityReport,
    ) -> Result<(), ReportValidationError> {
        self.validate()?;
        forest.validate()?;
        if !forest.passed
            || self.build != forest.build
            || self.world != forest.world
            || self.manifest_sha256 != forest.manifest_sha256
            || self.machine != forest.machine
            || self.forest_report_sha256 != forest.canonical_sha256()?
        {
            return Err(ReportValidationError::Identity {
                field: "forest feasibility gate",
            });
        }
        Ok(())
    }
}

struct GateHeader<'a> {
    schema: &'a str,
    timestamp_utc: &'a str,
    passed: bool,
    failure_reasons: &'a [String],
    build: &'a BuildProfile,
    manifest_sha256: &'a str,
    machine: &'a MachineProfile,
}

impl<'a> GateHeader<'a> {
    fn from_forest(report: &'a ForestFeasibilityReport) -> Self {
        Self {
            schema: &report.schema,
            timestamp_utc: &report.timestamp_utc,
            passed: report.passed,
            failure_reasons: &report.failure_reasons,
            build: &report.build,
            manifest_sha256: &report.manifest_sha256,
            machine: &report.machine,
        }
    }

    fn from_mutation(report: &'a MutationFeasibilityReport) -> Self {
        Self {
            schema: &report.schema,
            timestamp_utc: &report.timestamp_utc,
            passed: report.passed,
            failure_reasons: &report.failure_reasons,
            build: &report.build,
            manifest_sha256: &report.manifest_sha256,
            machine: &report.machine,
        }
    }
}

fn validate_header(
    expected_schema: &str,
    header: GateHeader<'_>,
) -> Result<(), ReportValidationError> {
    if header.schema != expected_schema {
        return Err(ReportValidationError::Schema);
    }
    if !rfc3339_utc(header.timestamp_utc) {
        return Err(ReportValidationError::Timestamp);
    }
    if header.passed != header.failure_reasons.is_empty() {
        return Err(ReportValidationError::PassedFlagMismatch);
    }
    if !sorted_unique_nonempty(header.failure_reasons) {
        return Err(ReportValidationError::FailureReasons);
    }
    header.build.validate_release()?;
    if !is_sha256(header.manifest_sha256) {
        return Err(ReportValidationError::Identity {
            field: "manifest_sha256",
        });
    }
    header.machine.validate_m4_acceptance()
}

fn validate_m4_machine(machine: &MachineProfile) -> Result<(), ReportValidationError> {
    if machine.acceptance_label != M4_ACCEPTANCE_LABEL
        || machine.os_name != "macOS"
        || machine.architecture != "aarch64"
        || !machine.cpu_model.contains("M4")
        || !machine.gpu_adapter_name.contains("M4")
        || machine.gpu_device_class != "integrated"
        || machine.wgpu_backend != "metal"
        || machine.memory_architecture != "unified"
        || machine.total_physical_memory_bytes < 32 * 1024 * 1024 * 1024
    {
        return Err(ReportValidationError::Identity {
            field: "M4 machine",
        });
    }
    Ok(())
}

fn matches_required_counts(observed: &BTreeMap<String, u32>, required: &[(&str, u32)]) -> bool {
    observed.len() == required.len()
        && required
            .iter()
            .all(|(name, count)| observed.get(*name) == Some(count))
}

fn validate_object_index(
    value: &ObjectIndexEvidence,
    passed: bool,
) -> Result<(), ReportValidationError> {
    finite_positive(value.validation_ms, "object_index.validation_ms")?;
    finite_positive(value.build_ms, "object_index.build_ms")?;
    validate_named_counts(&value.retained_byte_categories)?;
    let retained_category_bytes = value
        .retained_byte_categories
        .values()
        .try_fold(0_u64, |total, bytes| total.checked_add(*bytes))
        .ok_or(ReportValidationError::Inconsistent {
            field: "object_index retained bytes",
        })?;
    if retained_category_bytes != value.retained_bytes {
        return Err(ReportValidationError::Inconsistent {
            field: "object_index retained bytes",
        });
    }
    if passed
        && (value.validation_ms > 1_000.0
            || value.build_ms > 250.0
            || value.retained_bytes > 16 * 1024 * 1024
            || value.max_dependency_cell_entries > 1_024
            || value.max_sample_cell_entries > 64
            || value.max_horizon_tree_members_per_cell > 1_024
            || value.max_edit_candidates > 256
            || value.max_edit_affected_objects > 64
            || value.max_dependency_bricks > 128
            || value.dependency_coordinate_allocation_bytes != 0)
    {
        return Err(ReportValidationError::Limit {
            field: "object_index",
        });
    }
    Ok(())
}

fn validate_forest_object_index(
    value: &ObjectIndexEvidence,
    passed: bool,
) -> Result<(), ReportValidationError> {
    validate_object_index(value, passed)?;
    if value.validation_ms <= 0.0
        || value.build_ms <= 0.0
        || value.placement_records == 0
        || value.dependency_grid_entries == 0
        || value.sample_grid_entries == 0
        || value.retained_byte_categories.is_empty()
        || value
            .retained_byte_categories
            .values()
            .any(|bytes| *bytes == 0)
        || value.max_dependency_cell_entries == 0
        || value.max_sample_cell_entries == 0
        || value.max_horizon_tree_members_per_cell == 0
        || value.max_edit_candidates == 0
        || value.max_edit_affected_objects == 0
        || value.max_dependency_bricks == 0
    {
        return Err(ReportValidationError::Missing {
            field: "object index measurement",
        });
    }
    Ok(())
}

fn validate_worst_target(
    target: &WorstEditTargetEvidence,
    world: &WorldIdentity,
    passed: bool,
) -> Result<(), ReportValidationError> {
    if passed
        && (target.broad_candidates > 256
            || target.exact_dependency_ids > 64
            || target.dependency_bricks > 128)
    {
        return Err(ReportValidationError::Limit {
            field: "worst_edit_target",
        });
    }
    if !world.bounds.contains(target.center)
        || target.broad_candidates == 0
        || target.exact_dependency_ids == 0
        || target.dependency_bricks == 0
    {
        return Err(ReportValidationError::Missing {
            field: "worst_edit_target",
        });
    }
    Ok(())
}

fn validate_workload(
    value: &MutationWorkloadEvidence,
    passed: bool,
) -> Result<(), ReportValidationError> {
    let expected_request_count = match value.role {
        MutationWorkloadRole::InteractiveCarve | MutationWorkloadRole::CatastrophicCarve => 1,
        MutationWorkloadRole::ColonyVolume => 8,
    };
    if value.request_count != expected_request_count
        || passed && value.committed_batches < value.request_count
        || value.first_committed_frame < value.submitted_frame
        || value.final_reconciled_frame < value.first_committed_frame
        || value.barrier_expected_items != value.barrier_renderer_ready_items
    {
        return Err(ReportValidationError::Inconsistent {
            field: "workload reconciliation",
        });
    }
    for (name, distribution) in [
        ("admission_ms", value.admission_ms),
        ("first_commit_ms", value.first_commit_ms),
        ("primary_ready_ms", value.primary_ready_ms),
        ("reconciliation_ms", value.reconciliation_ms),
    ] {
        validate_distribution(distribution, name)?;
    }
    for (stage, elapsed) in &value.stage_timings_ms {
        finite_nonnegative(*elapsed, "stage_timings_ms")?;
        if stage.is_empty() {
            return Err(ReportValidationError::Missing {
                field: "stage timing key",
            });
        }
    }
    if value.stage_timings_ms.len() != REQUIRED_MUTATION_STAGES.len()
        || value.stage_counts.len() != REQUIRED_MUTATION_STAGES.len()
        || REQUIRED_MUTATION_STAGES.iter().any(|stage| {
            !value.stage_timings_ms.contains_key(*stage) || !value.stage_counts.contains_key(*stage)
        })
    {
        return Err(ReportValidationError::Missing {
            field: "mutation stage",
        });
    }
    if NONEMPTY_MUTATION_STAGES
        .iter()
        .any(|stage| value.stage_counts[*stage] == 0)
    {
        return Err(ReportValidationError::Missing {
            field: "mutation stage count",
        });
    }
    for (field, metric) in [
        ("changed_bricks_per_second", value.changed_bricks_per_second),
        ("maximum_runnable_wait_ms", value.maximum_runnable_wait_ms),
        ("maximum_frame_ms", value.maximum_frame_ms),
    ] {
        finite_positive(metric, field)?;
    }
    if value.changed_voxels == 0
        || value.changed_bricks == 0
        || value.committed_batches == 0
        || value.barrier_expected_items == 0
    {
        return Err(ReportValidationError::Missing {
            field: "workload measurement",
        });
    }
    if passed
        && (value.admission_ms.max > 2.0
            || value.first_commit_ms.max
                > match value.role {
                    MutationWorkloadRole::ColonyVolume => 250.0,
                    MutationWorkloadRole::InteractiveCarve
                    | MutationWorkloadRole::CatastrophicCarve => 100.0,
                }
            || value.primary_ready_ms.p95 > 250.0
            || value.primary_ready_ms.max > 500.0
            || value.reconciliation_ms.max
                > match value.role {
                    MutationWorkloadRole::InteractiveCarve => 1_000.0,
                    MutationWorkloadRole::ColonyVolume
                    | MutationWorkloadRole::CatastrophicCarve => 30_000.0,
                }
            || value.changed_bricks_per_second < 32.0
            || value.maximum_runnable_wait_ms > 500.0
            || value.maximum_frame_ms > 33.3)
    {
        return Err(ReportValidationError::Limit {
            field: "workload acceptance",
        });
    }
    if passed && value.role == MutationWorkloadRole::InteractiveCarve && !value.traversable {
        return Err(ReportValidationError::Inconsistent {
            field: "interactive traversal",
        });
    }
    if value.role == MutationWorkloadRole::CatastrophicCarve {
        if passed
            && (!value.horizon_partition_checked
                || value.horizon_excluded_base_cards == 0
                || value.horizon_derived_records == 0)
        {
            return Err(ReportValidationError::Missing {
                field: "catastrophic Horizon evidence",
            });
        }
        let dependency_ms = value.stage_timings_ms["dirty-discovery"]
            + value.stage_timings_ms["dependency-eligibility"];
        if passed && dependency_ms > 1.0 {
            return Err(ReportValidationError::Limit {
                field: "object dependency",
            });
        }
    }
    Ok(())
}

fn validate_query_costs(
    value: &QueryCostEvidence,
    passed: bool,
) -> Result<(), ReportValidationError> {
    if value.sample_counts.is_empty()
        || value.cold_inactive_calls.is_empty()
        || value.frame_critical_calls.is_empty()
        || value.observed_work_maxima.is_empty()
        || value.sample_counts.keys().any(|key| blank(key))
        || value.sample_counts.values().any(|count| *count == 0)
        || value.cold_inactive_calls.keys().any(|key| blank(key))
        || value.frame_critical_calls.keys().any(|key| blank(key))
        || value.observed_work_maxima.keys().any(|key| blank(key))
        || value.observed_work_maxima.values().any(|count| *count == 0)
    {
        return Err(ReportValidationError::Missing {
            field: "query evidence",
        });
    }
    for distribution in value.cold_inactive_calls.values() {
        validate_distribution(*distribution, "query distribution")?;
        if passed && distribution.max > 4.0 {
            return Err(ReportValidationError::Limit {
                field: "cold query cost",
            });
        }
    }
    for (name, count) in [
        ("cold_inactive_sample_voxel", 256),
        ("normal_query_bundles", 1_000),
        ("sample_column", 128),
        ("diagnostic_metadata_page", 128),
        ("diagnostic_cells_page", 128),
        ("player_sweep", 4_000),
        ("camera_sweep", 1_000),
        ("debug_ray", 1_000),
        ("water_surface", 1_000),
        ("water_contact", 1_000),
        ("active_band", 1_000),
    ] {
        if value.sample_counts.get(name) != Some(&count) {
            return Err(ReportValidationError::Missing {
                field: "query evidence",
            });
        }
    }
    if !value.cold_inactive_calls.contains_key("sample_voxel") {
        return Err(ReportValidationError::Missing {
            field: "query evidence",
        });
    }
    for name in [
        "player_sweep",
        "camera_sweep",
        "debug_ray",
        "water_surface",
        "water_contact",
        "active_band",
    ] {
        if !value.frame_critical_calls.contains_key(name) {
            return Err(ReportValidationError::Missing {
                field: "query evidence",
            });
        }
    }
    for (name, maximum) in [
        ("cold_coordinates_distinct", 256),
        ("ray_voxel_visits", 448),
        ("sweep_candidate_tests", 8_192),
        ("overlap_candidate_tests", 512),
        ("column_runs", 64),
        ("diagnostic_metadata_bricks", 256),
        ("diagnostic_cells_bricks", 2),
        ("diagnostic_cells", 8_192),
    ] {
        if value.observed_work_maxima.get(name) != Some(&maximum) {
            return Err(ReportValidationError::Missing {
                field: "query evidence",
            });
        }
    }
    for distribution in value.frame_critical_calls.values() {
        validate_distribution(*distribution, "query distribution")?;
        if passed && (distribution.p99 > 1.0 || distribution.max > 4.0) {
            return Err(ReportValidationError::Limit {
                field: "frame-critical query cost",
            });
        }
    }
    for distribution in [
        value.normal_bundle_ms,
        value.column_ms,
        value.diagnostic_metadata_page_ms,
        value.diagnostic_cells_page_ms,
    ] {
        validate_distribution(distribution, "query distribution")?;
    }
    if passed
        && (value.normal_bundle_ms.p99 > 2.0
            || value.normal_bundle_ms.max > 4.0
            || value.column_ms.p99 > 1.0
            || value.column_ms.max > 4.0
            || value.diagnostic_metadata_page_ms.p99 > 1.0
            || value.diagnostic_metadata_page_ms.max > 4.0
            || value.diagnostic_cells_page_ms.p99 > 4.0
            || value.diagnostic_cells_page_ms.max > 8.0)
    {
        return Err(ReportValidationError::Limit {
            field: "query cost",
        });
    }
    Ok(())
}

fn validate_distribution(
    value: Distribution,
    field: &'static str,
) -> Result<(), ReportValidationError> {
    for metric in [value.min, value.p50, value.p95, value.p99, value.max] {
        finite_nonnegative(metric, field)?;
    }
    if !(value.min <= value.p50
        && value.p50 <= value.p95
        && value.p95 <= value.p99
        && value.p99 <= value.max)
    {
        return Err(ReportValidationError::Inconsistent { field });
    }
    if value.max == 0.0 {
        return Err(ReportValidationError::Missing { field });
    }
    Ok(())
}

fn validate_named_counts<T>(values: &BTreeMap<String, T>) -> Result<(), ReportValidationError> {
    if values.keys().any(|key| blank(key)) {
        Err(ReportValidationError::Missing { field: "map key" })
    } else {
        Ok(())
    }
}

fn validate_positive_named_counts(
    values: &BTreeMap<String, u32>,
) -> Result<(), ReportValidationError> {
    validate_named_counts(values)?;
    if values.values().any(|count| *count == 0) {
        return Err(ReportValidationError::Missing {
            field: "map measurement",
        });
    }
    Ok(())
}

fn finite(value: f64, field: &'static str) -> Result<(), ReportValidationError> {
    value
        .is_finite()
        .then_some(())
        .ok_or(ReportValidationError::NonFinite { field })
}

fn finite_nonnegative(value: f64, field: &'static str) -> Result<(), ReportValidationError> {
    finite(value, field)?;
    if value < 0.0 {
        return Err(ReportValidationError::Inconsistent { field });
    }
    Ok(())
}

fn finite_positive(value: f64, field: &'static str) -> Result<(), ReportValidationError> {
    finite_nonnegative(value, field)?;
    if value == 0.0 {
        return Err(ReportValidationError::Missing { field });
    }
    Ok(())
}

fn sorted_unique_nonempty(values: &[String]) -> bool {
    values.iter().all(|value| !blank(value)) && values.windows(2).all(|pair| pair[0] < pair[1])
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}
fn is_git_commit(value: &str) -> bool {
    value.len() == 40
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn rfc3339_utc(value: &str) -> bool {
    let Some(date_time) = value.strip_suffix('Z') else {
        return false;
    };
    let (whole, fraction) = date_time
        .split_once('.')
        .map_or((date_time, None), |(whole, fraction)| {
            (whole, Some(fraction))
        });
    if fraction
        .is_some_and(|digits| digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()))
        || whole.len() != 19
        || whole.as_bytes()[4] != b'-'
        || whole.as_bytes()[7] != b'-'
        || whole.as_bytes()[10] != b'T'
        || whole.as_bytes()[13] != b':'
        || whole.as_bytes()[16] != b':'
    {
        return false;
    }
    let number = |range: core::ops::Range<usize>| whole[range].parse::<u32>().ok();
    let (Some(year), Some(month), Some(day), Some(hour), Some(minute), Some(second)) = (
        number(0..4),
        number(5..7),
        number(8..10),
        number(11..13),
        number(14..16),
        number(17..19),
    ) else {
        return false;
    };
    let leap = year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400));
    let days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => return false,
    };
    day != 0 && day <= days && hour < 24 && minute < 60 && second < 60
}

fn validate_world_identity(world: &WorldIdentity) -> Result<(), ReportValidationError> {
    let min = world.bounds.min();
    let max = world.bounds.max_exclusive();
    if world.seed != PRODUCT_ONE_SEED
        || world.parameters_digest.iter().all(|byte| *byte == 0)
        || min != WorldPointQ8::new(-128_000, -32_768, -128_000)
        || max != WorldPointQ8::new(128_000, 32_768, 128_000)
    {
        return Err(ReportValidationError::Identity { field: "world" });
    }
    Ok(())
}

fn blank(value: &str) -> bool {
    value.trim().is_empty()
}

fn json_object_has_keys(value: &serde_json::Value, keys: &[&str]) -> bool {
    value
        .as_object()
        .is_some_and(|object| keys.iter().all(|key| object.contains_key(*key)))
}

fn json_nested_object_has_keys(value: &serde_json::Value, field: &str, keys: &[&str]) -> bool {
    value
        .as_object()
        .and_then(|object| object.get(field))
        .is_some_and(|nested| json_object_has_keys(nested, keys))
}
