//! The sole final benchmark JSON representation.

use std::collections::BTreeMap;

use moria_world::WorldIdentity;
use moria_world::telemetry::{
    BuildProfile, Distribution, MachineProfile, ObjectIndexEvidence, ReportValidationError,
};
use serde::{Deserialize, Serialize};

const SCHEMA: &str = "moria-product-one-benchmark";
const GRAPHICS_MEMORY_TARGET_BYTES: u64 = 2_097_152_000;
const TOP_LEVEL_KEYS: [&str; 19] = [
    "schema",
    "timestamp_utc",
    "scenario",
    "passed",
    "failure_reasons",
    "baseline_status",
    "build",
    "world",
    "assets",
    "machine",
    "resolution",
    "cold_start_ms",
    "frame_rate",
    "frame_time_ms",
    "graphics_memory",
    "mutation_latency",
    "save",
    "coverage",
    "streaming",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ScenarioName {
    Flythrough,
    CarveStorm,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BaselineStatus {
    Provisional,
    Verified,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AssetEvidence {
    pub manifest_sha256: String,
    pub fallbacks: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FrameRateMetrics {
    pub sample_count: u64,
    pub measured_seconds: f64,
    pub arithmetic_fps: f64,
    pub one_percent_low_fps: f64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GraphicsMemoryEstimate {
    pub peak_bytes: u64,
    pub end_bytes: u64,
    pub categories: BTreeMap<String, u64>,
    pub untracked_driver_overhead: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResidentGraphicsMeasurement {
    pub provider: String,
    pub scope: String,
    pub sampling_interval_ms: u32,
    pub peak_bytes: u64,
    pub artifact_sha256: String,
    pub artifact_path: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GraphicsMemoryEvidence {
    pub application_ledger: GraphicsMemoryEstimate,
    pub resident_measurement: Option<ResidentGraphicsMeasurement>,
    pub product_target_proven: bool,
    pub estimate_substitution_approval_id: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RoundTripEvidence {
    pub passed: bool,
    pub delta_voxels_compared: u32,
    pub base_samples_compared: u32,
    pub identity_match: bool,
    pub derived_bytes_found: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SaveEvidence {
    pub attempted: bool,
    pub completed: bool,
    pub size_bytes: Option<u64>,
    pub changed_voxels: Option<u32>,
    pub changed_bricks: Option<u32>,
    pub round_trip: Option<RoundTripEvidence>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MutationLatencyMetrics {
    pub sample_count: u32,
    pub admission_ms: Distribution,
    pub accepted_to_first_commit_ms: Distribution,
    pub commit_to_primary_ready_ms: Distribution,
    pub accepted_to_reconciliation_ms: Distribution,
    pub changed_bricks_per_second: f64,
    pub maximum_runnable_wait_ms: f64,
    pub representative_max_frame_ms: f64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoverageEvidence {
    pub route_tags_visited: Vec<String>,
    pub active_bands_entered: Vec<String>,
    pub edited_material_counts: BTreeMap<String, u32>,
    pub final_changed_spheres: u32,
    pub final_changed_region_cells: u32,
    pub workload_minimum_met: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActiveCounts {
    pub bricks: u32,
    pub meshes: u32,
    pub objects: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QueueDepths {
    pub extraction: u32,
    pub installation: u32,
    pub render: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StreamingEvidence {
    pub peak_active_counts: ActiveCounts,
    pub peak_queue_depths: QueueDepths,
    pub first_steady_derived_bytes: u64,
    pub return_steady_derived_bytes: u64,
    pub monotonic_growth_check_passed: bool,
    pub object_index: ObjectIndexEvidence,
}

/// `Option` fields are always serialized: `null` records unavailable runtime evidence.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BenchmarkReport {
    pub schema: String,
    pub timestamp_utc: String,
    pub scenario: ScenarioName,
    pub passed: bool,
    pub failure_reasons: Vec<String>,
    pub baseline_status: BaselineStatus,
    pub build: Option<BuildProfile>,
    pub world: Option<WorldIdentity>,
    pub assets: Option<AssetEvidence>,
    pub machine: Option<MachineProfile>,
    pub resolution: Option<[u32; 2]>,
    pub cold_start_ms: Option<f64>,
    pub frame_rate: Option<FrameRateMetrics>,
    pub frame_time_ms: Option<Distribution>,
    pub graphics_memory: Option<GraphicsMemoryEvidence>,
    pub mutation_latency: Option<MutationLatencyMetrics>,
    pub save: SaveEvidence,
    pub coverage: Option<CoverageEvidence>,
    pub streaming: Option<StreamingEvidence>,
}

impl BenchmarkReport {
    #[must_use]
    pub fn failed_before_start(timestamp_utc: &str, failure_reason: &str) -> Self {
        Self {
            schema: SCHEMA.into(),
            timestamp_utc: timestamp_utc.into(),
            scenario: ScenarioName::Flythrough,
            passed: false,
            failure_reasons: missing_evidence_reasons(failure_reason),
            baseline_status: BaselineStatus::Provisional,
            build: None,
            world: None,
            assets: None,
            machine: None,
            resolution: None,
            cold_start_ms: None,
            frame_rate: None,
            frame_time_ms: None,
            graphics_memory: None,
            mutation_latency: None,
            save: SaveEvidence {
                attempted: false,
                completed: false,
                size_bytes: None,
                changed_voxels: None,
                changed_bricks: None,
                round_trip: None,
            },
            coverage: None,
            streaming: None,
        }
    }

    pub fn validate(&self) -> Result<(), ReportValidationError> {
        if self.schema != SCHEMA {
            return Err(ReportValidationError::Schema);
        }
        if !rfc3339_utc(&self.timestamp_utc) {
            return Err(ReportValidationError::Timestamp);
        }
        if self.passed != self.failure_reasons.is_empty() {
            return Err(ReportValidationError::PassedFlagMismatch);
        }
        if !sorted_unique_nonempty(&self.failure_reasons) {
            return Err(ReportValidationError::FailureReasons);
        }
        let required = [
            ("build", self.build.is_some()),
            ("world", self.world.is_some()),
            ("assets", self.assets.is_some()),
            ("machine", self.machine.is_some()),
            ("resolution", self.resolution.is_some()),
            ("cold_start_ms", self.cold_start_ms.is_some()),
            ("frame_rate", self.frame_rate.is_some()),
            ("frame_time_ms", self.frame_time_ms.is_some()),
            ("graphics_memory", self.graphics_memory.is_some()),
            ("coverage", self.coverage.is_some()),
            ("streaming", self.streaming.is_some()),
        ];
        for (name, present) in required {
            if !present && !self.failure_reasons.iter().any(|reason| reason == name) {
                return Err(ReportValidationError::Missing { field: name });
            }
        }
        if self.passed && required.iter().any(|(_, present)| !present) {
            return Err(ReportValidationError::Missing {
                field: "completed common evidence",
            });
        }

        if let Some(value) = &self.build {
            validate_build(value)?;
        }

        if let Some(resolution) = self.resolution
            && (resolution[0] == 0 || resolution[1] == 0)
        {
            return Err(ReportValidationError::Identity {
                field: "resolution",
            });
        }
        if let Some(value) = self.cold_start_ms {
            finite(value, "cold_start_ms")?;
        }
        if let Some(value) = &self.assets {
            validate_asset_evidence(value)?;
        }
        if let Some(value) = self.frame_rate {
            validate_frame_rate(value)?;
        }
        if let Some(value) = self.frame_time_ms {
            validate_distribution(value, "frame_time_ms")?;
        }
        if let Some(value) = &self.graphics_memory {
            validate_graphics_memory(value, self.passed, &self.failure_reasons)?;
        }
        if let Some(value) = &self.coverage {
            validate_coverage(value)?;
        }
        if let Some(value) = &self.streaming {
            validate_streaming(value)?;
        }

        match self.scenario {
            ScenarioName::Flythrough => validate_flythrough(self),
            ScenarioName::CarveStorm => validate_carve_storm(self),
        }
    }

    pub fn to_canonical_json(&self) -> Result<String, ReportValidationError> {
        self.validate()?;
        serde_json::to_string(self).map_err(|_| ReportValidationError::Serialization)
    }

    pub fn from_json(json: &str) -> Result<Self, ReportValidationError> {
        let value: serde_json::Value =
            serde_json::from_str(json).map_err(|_| ReportValidationError::Serialization)?;
        let object = value
            .as_object()
            .ok_or(ReportValidationError::Serialization)?;
        let keys = TOP_LEVEL_KEYS
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>();
        if object.len() != keys.len() || object.keys().any(|key| !keys.contains(key.as_str())) {
            return Err(ReportValidationError::Missing {
                field: "top-level key",
            });
        }
        let report: Self =
            serde_json::from_value(value).map_err(|_| ReportValidationError::Serialization)?;
        report.validate()?;
        Ok(report)
    }
}

fn validate_flythrough(report: &BenchmarkReport) -> Result<(), ReportValidationError> {
    if report.mutation_latency.is_some() {
        return Err(ReportValidationError::Inconsistent {
            field: "mutation_latency",
        });
    }
    let save = &report.save;
    if save.attempted || save.completed || save.round_trip.is_some() {
        return Err(ReportValidationError::Inconsistent {
            field: "flythrough save",
        });
    }
    for (field, value) in [
        ("save.size_bytes", save.size_bytes.is_some()),
        ("save.changed_voxels", save.changed_voxels.is_some()),
        ("save.changed_bricks", save.changed_bricks.is_some()),
    ] {
        if !value && !has_failure_reason(report, field) {
            return Err(ReportValidationError::Missing { field });
        }
    }
    if report.passed
        && (save.size_bytes != Some(0)
            || save.changed_voxels != Some(0)
            || save.changed_bricks != Some(0))
    {
        return Err(ReportValidationError::Inconsistent {
            field: "flythrough save",
        });
    }
    Ok(())
}

fn validate_carve_storm(report: &BenchmarkReport) -> Result<(), ReportValidationError> {
    if report.mutation_latency.is_none() && !has_failure_reason(report, "mutation_latency") {
        return Err(ReportValidationError::Missing {
            field: "mutation_latency",
        });
    }
    if let Some(value) = &report.mutation_latency {
        validate_mutation_latency(value)?;
    }
    let save = &report.save;
    if report.passed
        && (!save.attempted
            || !save.completed
            || save.size_bytes.is_none()
            || save.changed_voxels.is_none()
            || save.changed_bricks.is_none()
            || save.round_trip.is_none())
    {
        return Err(ReportValidationError::Missing {
            field: "mutation save",
        });
    }
    if let Some(round_trip) = save.round_trip
        && report.passed
        && (!round_trip.passed || !round_trip.identity_match || round_trip.derived_bytes_found)
    {
        return Err(ReportValidationError::Inconsistent {
            field: "round_trip",
        });
    }
    Ok(())
}

fn missing_evidence_reasons(failure_reason: &str) -> Vec<String> {
    let mut reasons = [
        "assets",
        "build",
        "cold_start_ms",
        "coverage",
        "frame_rate",
        "frame_time_ms",
        "graphics_memory",
        "machine",
        "resolution",
        "save.changed_bricks",
        "save.changed_voxels",
        "save.size_bytes",
        "streaming",
        "world",
        failure_reason,
    ]
    .into_iter()
    .map(str::to_owned)
    .collect::<Vec<_>>();
    reasons.sort();
    reasons.dedup();
    reasons
}

fn has_failure_reason(report: &BenchmarkReport, field: &str) -> bool {
    report.failure_reasons.iter().any(|reason| reason == field)
}

fn validate_build(build: &BuildProfile) -> Result<(), ReportValidationError> {
    if build.cargo_profile.is_empty()
        || build.git_commit.len() != 40
        || !build
            .git_commit
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
        || build.rustc_version.is_empty()
    {
        return Err(ReportValidationError::Identity { field: "build" });
    }
    Ok(())
}

fn validate_asset_evidence(value: &AssetEvidence) -> Result<(), ReportValidationError> {
    if !sha256(&value.manifest_sha256)
        || !sorted_unique_nonempty(&value.fallbacks)
        || !sorted_unique_nonempty(&value.warnings)
    {
        return Err(ReportValidationError::Identity { field: "assets" });
    }
    Ok(())
}

fn validate_frame_rate(value: FrameRateMetrics) -> Result<(), ReportValidationError> {
    if value.sample_count == 0 {
        return Err(ReportValidationError::Missing {
            field: "frame samples",
        });
    }
    for (name, metric) in [
        ("measured_seconds", value.measured_seconds),
        ("arithmetic_fps", value.arithmetic_fps),
        ("one_percent_low_fps", value.one_percent_low_fps),
    ] {
        finite(metric, name)?;
    }
    Ok(())
}

fn validate_graphics_memory(
    value: &GraphicsMemoryEvidence,
    passed: bool,
    reasons: &[String],
) -> Result<(), ReportValidationError> {
    if !value.application_ledger.untracked_driver_overhead
        || value
            .application_ledger
            .categories
            .keys()
            .any(String::is_empty)
    {
        return Err(ReportValidationError::Inconsistent {
            field: "application_ledger",
        });
    }
    if let Some(measurement) = &value.resident_measurement {
        if measurement.provider.is_empty()
            || measurement.scope.is_empty()
            || measurement.sampling_interval_ms == 0
            || !sha256(&measurement.artifact_sha256)
            || measurement.artifact_path.is_empty()
        {
            return Err(ReportValidationError::Missing {
                field: "resident_measurement",
            });
        }
        let covers_target = measurement.scope.contains("game process")
            && measurement.scope.contains("resident graphics")
            && measurement.peak_bytes < GRAPHICS_MEMORY_TARGET_BYTES;
        if value.product_target_proven != covers_target {
            return Err(ReportValidationError::Inconsistent {
                field: "product_target_proven",
            });
        }
    } else if value.product_target_proven {
        return Err(ReportValidationError::Inconsistent {
            field: "product_target_proven",
        });
    }
    if value
        .estimate_substitution_approval_id
        .as_deref()
        .is_some_and(str::is_empty)
    {
        return Err(ReportValidationError::Identity {
            field: "estimate_substitution_approval_id",
        });
    }
    if passed && !value.product_target_proven && value.estimate_substitution_approval_id.is_none() {
        return Err(ReportValidationError::Inconsistent {
            field: "resident graphics memory",
        });
    }
    if !passed
        && value.resident_measurement.is_none()
        && value.estimate_substitution_approval_id.is_none()
        && !reasons
            .iter()
            .any(|reason| reason == "resident_graphics_memory_unproven")
    {
        return Err(ReportValidationError::Missing {
            field: "resident_graphics_memory_unproven",
        });
    }
    Ok(())
}

fn validate_coverage(value: &CoverageEvidence) -> Result<(), ReportValidationError> {
    if !sorted_unique_nonempty(&value.route_tags_visited)
        || !sorted_unique_nonempty(&value.active_bands_entered)
        || value.edited_material_counts.keys().any(String::is_empty)
    {
        return Err(ReportValidationError::Inconsistent { field: "coverage" });
    }
    Ok(())
}

fn validate_streaming(value: &StreamingEvidence) -> Result<(), ReportValidationError> {
    if !value.monotonic_growth_check_passed
        || value.return_steady_derived_bytes
            > value.first_steady_derived_bytes.saturating_mul(105) / 100
    {
        return Err(ReportValidationError::Inconsistent {
            field: "monotonic growth",
        });
    }
    finite(
        value.object_index.validation_ms,
        "object_index.validation_ms",
    )?;
    finite(value.object_index.build_ms, "object_index.build_ms")?;
    Ok(())
}

fn validate_mutation_latency(value: &MutationLatencyMetrics) -> Result<(), ReportValidationError> {
    if value.sample_count == 0 {
        return Err(ReportValidationError::Missing {
            field: "mutation samples",
        });
    }
    for distribution in [
        value.admission_ms,
        value.accepted_to_first_commit_ms,
        value.commit_to_primary_ready_ms,
        value.accepted_to_reconciliation_ms,
    ] {
        validate_distribution(distribution, "mutation distribution")?;
    }
    for (name, metric) in [
        ("changed_bricks_per_second", value.changed_bricks_per_second),
        ("maximum_runnable_wait_ms", value.maximum_runnable_wait_ms),
        (
            "representative_max_frame_ms",
            value.representative_max_frame_ms,
        ),
    ] {
        finite(metric, name)?;
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

fn finite(value: f64, field: &'static str) -> Result<(), ReportValidationError> {
    value
        .is_finite()
        .then_some(())
        .ok_or(ReportValidationError::NonFinite { field })
}
fn sorted_unique_nonempty(values: &[String]) -> bool {
    values.iter().all(|value| !value.is_empty()) && values.windows(2).all(|pair| pair[0] < pair[1])
}
fn sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}
fn rfc3339_utc(value: &str) -> bool {
    value.ends_with('Z')
        && value.len() >= 20
        && value.as_bytes().get(4) == Some(&b'-')
        && value.as_bytes().get(7) == Some(&b'-')
        && value.as_bytes().get(10) == Some(&b'T')
        && value.as_bytes().get(13) == Some(&b':')
        && value.as_bytes().get(16) == Some(&b':')
}

#[cfg(test)]
mod tests {
    use super::BenchmarkReport;

    #[test]
    fn report_before_start_is_complete_null_json_with_missing_evidence_reasons() {
        let report = BenchmarkReport::failed_before_start("2026-07-17T00:00:00Z", "runtime");

        let json = report.to_canonical_json().unwrap();
        let restored = BenchmarkReport::from_json(&json).unwrap();

        assert!(!restored.passed);
        assert!(
            restored
                .failure_reasons
                .binary_search(&"build".into())
                .is_ok()
        );
        assert!(
            restored
                .failure_reasons
                .binary_search(&"world".into())
                .is_ok()
        );
        assert!(
            restored
                .failure_reasons
                .binary_search(&"runtime".into())
                .is_ok()
        );
        assert!(json.contains("\"build\":null"));
        assert!(
            json.contains("\"save\":{\"attempted\":false,\"completed\":false,\"size_bytes\":null")
        );
    }

    #[test]
    fn partial_runtime_failure_requires_reasons_only_for_unavailable_evidence() {
        let mut report = BenchmarkReport::failed_before_start("2026-07-17T00:00:00Z", "runtime");
        report.resolution = Some([2560, 1440]);
        report
            .failure_reasons
            .retain(|reason| reason != "resolution");

        report.validate().unwrap();
        assert!(
            report
                .to_canonical_json()
                .unwrap()
                .contains("\"resolution\":[2560,1440]")
        );
    }
}
