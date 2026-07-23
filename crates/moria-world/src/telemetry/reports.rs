//! Feasibility evidence is deliberately separate from runtime allocation telemetry.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{WorldIdentity, WorldPointQ8};

const FOREST_SCHEMA: &str = "moria-product-one-forest-feasibility";
const MUTATION_SCHEMA: &str = "moria-product-one-mutation-feasibility";
const M4_ACCEPTANCE_LABEL: &str = "m4-mac-mini-32gb";
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
    /// A zero-sized, contract-valid evidence value useful before a manifest has placements.
    #[must_use]
    pub fn passing_fixture() -> Self {
        Self {
            validation_ms: 0.0,
            build_ms: 0.0,
            retained_bytes: 0,
            retained_byte_categories: BTreeMap::new(),
            placement_records: 0,
            dependency_grid_entries: 0,
            sample_grid_entries: 0,
            max_dependency_cell_entries: 0,
            max_sample_cell_entries: 0,
            max_horizon_tree_members_per_cell: 0,
            max_edit_candidates: 0,
            max_edit_affected_objects: 0,
            max_dependency_bricks: 0,
            dependency_coordinate_allocation_bytes: 0,
        }
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
        validate_named_counts(&self.object_counts)?;
        validate_named_counts(&self.required_object_counts)?;
        validate_named_counts(&self.species_counts)?;
        validate_named_counts(&self.canopy_range_bins)?;
        if self.forest_area_m2 == 0
            || self.eligible_land_area_m2 == 0
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
        for (kind, required) in &self.required_object_counts {
            if self.object_counts.get(kind).unwrap_or(&0) < required {
                return Err(ReportValidationError::Inconsistent {
                    field: "object counts",
                });
            }
        }
        if self.canopy_min_q8 > self.canopy_max_q8 {
            return Err(ReportValidationError::Inconsistent {
                field: "canopy range",
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
        validate_worst_target(&self.worst_edit_target)
    }

    pub fn to_canonical_json(&self) -> Result<String, ReportValidationError> {
        self.validate()?;
        serde_json::to_string(self).map_err(|_| ReportValidationError::Serialization)
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
        finite(self.cold_start_ms, "cold_start_ms")?;
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
            validate_workload(workload)?;
        }
        validate_query_costs(&self.query_costs)
    }

    pub fn to_canonical_json(&self) -> Result<String, ReportValidationError> {
        self.validate()?;
        serde_json::to_string(self).map_err(|_| ReportValidationError::Serialization)
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
    if header.passed
        && (header.build.cargo_profile != "release"
            || !is_git_commit(&header.build.git_commit)
            || header.build.rustc_version.is_empty())
    {
        return Err(ReportValidationError::Identity { field: "build" });
    }
    if !is_sha256(header.manifest_sha256) {
        return Err(ReportValidationError::Identity {
            field: "manifest_sha256",
        });
    }
    if header.passed {
        validate_m4_machine(header.machine)
    } else {
        Ok(())
    }
}

fn validate_m4_machine(machine: &MachineProfile) -> Result<(), ReportValidationError> {
    let required = [
        &machine.os_name,
        &machine.os_version,
        &machine.architecture,
        &machine.cpu_model,
        &machine.gpu_adapter_name,
        &machine.gpu_device_class,
        &machine.wgpu_backend,
        &machine.memory_architecture,
        &machine.acceptance_label,
    ];
    if required.iter().any(|value| value.is_empty())
        || machine.logical_cores == 0
        || machine.total_physical_memory_bytes == 0
        || !is_sha256(&machine.profile_id_sha256)
        || machine.driver_metadata_available != machine.driver.is_some()
    {
        return Err(ReportValidationError::Identity { field: "machine" });
    }
    if machine.acceptance_label != M4_ACCEPTANCE_LABEL
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

fn validate_object_index(value: &ObjectIndexEvidence) -> Result<(), ReportValidationError> {
    finite(value.validation_ms, "object_index.validation_ms")?;
    finite(value.build_ms, "object_index.build_ms")?;
    validate_named_counts(&value.retained_byte_categories)?;
    if value.validation_ms > 1_000.0
        || value.build_ms > 250.0
        || value.retained_bytes > 16 * 1024 * 1024
        || value.max_dependency_cell_entries > 1_024
        || value.max_sample_cell_entries > 64
        || value.max_horizon_tree_members_per_cell > 1_024
        || value.max_edit_candidates > 256
        || value.max_edit_affected_objects > 64
        || value.max_dependency_bricks > 128
        || value.dependency_coordinate_allocation_bytes != 0
    {
        return Err(ReportValidationError::Limit {
            field: "object_index",
        });
    }
    Ok(())
}

fn validate_forest_object_index(
    value: &ObjectIndexEvidence,
    enforce_limits: bool,
) -> Result<(), ReportValidationError> {
    if enforce_limits {
        validate_object_index(value)?;
    } else {
        finite(value.validation_ms, "object_index.validation_ms")?;
        finite(value.build_ms, "object_index.build_ms")?;
        validate_named_counts(&value.retained_byte_categories)?;
    }
    if value.validation_ms <= 0.0
        || value.build_ms <= 0.0
        || value.placement_records == 0
        || value.dependency_grid_entries == 0
        || value.sample_grid_entries == 0
        || value.retained_byte_categories.is_empty()
    {
        return Err(ReportValidationError::Missing {
            field: "object index measurement",
        });
    }
    Ok(())
}

fn validate_worst_target(target: &WorstEditTargetEvidence) -> Result<(), ReportValidationError> {
    if target.broad_candidates > 256
        || target.exact_dependency_ids > 64
        || target.dependency_bricks > 128
    {
        return Err(ReportValidationError::Limit {
            field: "worst_edit_target",
        });
    }
    Ok(())
}

fn validate_workload(value: &MutationWorkloadEvidence) -> Result<(), ReportValidationError> {
    if value.request_count == 0
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
        finite(*elapsed, "stage_timings_ms")?;
        if stage.is_empty() {
            return Err(ReportValidationError::Missing {
                field: "stage timing key",
            });
        }
    }
    if REQUIRED_MUTATION_STAGES.iter().any(|stage| {
        !value.stage_timings_ms.contains_key(*stage) || !value.stage_counts.contains_key(*stage)
    }) {
        return Err(ReportValidationError::Missing {
            field: "mutation stage",
        });
    }
    for (field, metric) in [
        ("changed_bricks_per_second", value.changed_bricks_per_second),
        ("maximum_runnable_wait_ms", value.maximum_runnable_wait_ms),
        ("maximum_frame_ms", value.maximum_frame_ms),
    ] {
        finite(metric, field)?;
    }
    Ok(())
}

fn validate_query_costs(value: &QueryCostEvidence) -> Result<(), ReportValidationError> {
    for distribution in value
        .cold_inactive_calls
        .values()
        .chain(value.frame_critical_calls.values())
    {
        validate_distribution(*distribution, "query distribution")?;
    }
    for distribution in [
        value.normal_bundle_ms,
        value.column_ms,
        value.diagnostic_metadata_page_ms,
        value.diagnostic_cells_page_ms,
    ] {
        validate_distribution(distribution, "query distribution")?;
    }
    Ok(())
}

fn validate_distribution(
    value: Distribution,
    field: &'static str,
) -> Result<(), ReportValidationError> {
    for metric in [value.min, value.p50, value.p95, value.p99, value.max] {
        finite(metric, field)?;
    }
    if !(value.min <= value.p50
        && value.p50 <= value.p95
        && value.p95 <= value.p99
        && value.p99 <= value.max)
    {
        return Err(ReportValidationError::Inconsistent { field });
    }
    Ok(())
}

fn validate_named_counts<T>(values: &BTreeMap<String, T>) -> Result<(), ReportValidationError> {
    if values.keys().any(String::is_empty) {
        Err(ReportValidationError::Missing { field: "map key" })
    } else {
        Ok(())
    }
}

fn finite(value: f64, field: &'static str) -> Result<(), ReportValidationError> {
    value
        .is_finite()
        .then_some(())
        .ok_or(ReportValidationError::NonFinite { field })
}

fn sorted_unique_nonempty(values: &[String]) -> bool {
    values.iter().all(|value| !value.is_empty()) && values.windows(2).all(|pair| pair[0] < pair[1])
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}
fn is_git_commit(value: &str) -> bool {
    value.len() == 40 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn rfc3339_utc(value: &str) -> bool {
    // The runner emits whole RFC 3339 UTC timestamps. This strict structural check avoids
    // accepting local timestamps without introducing a wall-clock dependency into telemetry.
    value.ends_with('Z')
        && value.len() >= 20
        && value.as_bytes().get(4) == Some(&b'-')
        && value.as_bytes().get(7) == Some(&b'-')
        && value.as_bytes().get(10) == Some(&b'T')
        && value.as_bytes().get(13) == Some(&b':')
        && value.as_bytes().get(16) == Some(&b':')
}
