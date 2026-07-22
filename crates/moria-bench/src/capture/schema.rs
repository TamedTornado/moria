//! The sole final benchmark JSON representation.

use std::collections::BTreeMap;

use moria_world::config::PRODUCT_ONE_SEED;
use moria_world::telemetry::{
    BuildProfile, Distribution, MachineProfile, ObjectIndexEvidence, ReportValidationError,
};
use moria_world::{WorldIdentity, WorldPointQ8};
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

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ActiveBand {
    Far,
    Horizon,
    Middle,
    Near,
}

const ALL_ACTIVE_BANDS: [ActiveBand; 4] = [
    ActiveBand::Far,
    ActiveBand::Horizon,
    ActiveBand::Middle,
    ActiveBand::Near,
];

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
    pub active_bands_entered: Vec<ActiveBand>,
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
        if !sorted_unique(&self.failure_reasons) {
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
            validate_build(value, self.passed)?;
        }
        if let Some(resolution) = self.resolution
            && !matches!(resolution, [1920, 1080] | [2560, 1440])
        {
            return Err(ReportValidationError::Identity {
                field: "resolution",
            });
        }
        if let Some(value) = self.cold_start_ms {
            finite_positive(value, "cold_start_ms")?;
            if self.passed && value >= 5_000.0 {
                return Err(ReportValidationError::Limit {
                    field: "cold_start_ms",
                });
            }
        }
        if let Some(value) = &self.world {
            validate_world(value)?;
        }
        if let Some(value) = &self.assets {
            validate_asset_evidence(value, self.passed)?;
        }
        if let (Some(machine), Some(resolution)) = (&self.machine, self.resolution) {
            validate_machine(machine, resolution)?;
        }
        if let Some(value) = self.frame_rate {
            validate_frame_rate(value, self.passed)?;
        }
        if let Some(value) = self.frame_time_ms {
            validate_distribution(value, "frame_time_ms")?;
            if self.passed && value.p95 > 16.67 {
                return Err(ReportValidationError::Limit {
                    field: "frame_time_ms.p95",
                });
            }
        }
        if let Some(value) = &self.graphics_memory {
            validate_graphics_memory(value, self.passed, &self.failure_reasons)?;
        }
        if let Some(value) = &self.coverage {
            validate_coverage(value, self.scenario, self.passed)?;
        }
        if let Some(value) = &self.streaming {
            validate_streaming(value, self.passed)?;
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
        let report: Self =
            serde_json::from_str(json).map_err(|_| ReportValidationError::Serialization)?;
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
        if !json_object_has_keys(
            object.get("save").unwrap_or(&serde_json::Value::Null),
            &[
                "attempted",
                "completed",
                "size_bytes",
                "changed_voxels",
                "changed_bricks",
                "round_trip",
            ],
        ) || !json_nullable_object_has_keys(object.get("machine"), &["driver"])
            || !json_nullable_object_has_keys(
                object.get("graphics_memory"),
                &["resident_measurement", "estimate_substitution_approval_id"],
            )
        {
            return Err(ReportValidationError::Missing { field: "JSON key" });
        }
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
    if save.attempted
        || save.completed
        || save.round_trip.is_some()
        || save.size_bytes.is_some_and(|value| value != 0)
        || save.changed_voxels.is_some_and(|value| value != 0)
        || save.changed_bricks.is_some_and(|value| value != 0)
    {
        return Err(ReportValidationError::Inconsistent {
            field: "flythrough save",
        });
    }
    if report.passed
        && (save.size_bytes != Some(0)
            || save.changed_voxels != Some(0)
            || save.changed_bricks != Some(0))
    {
        return Err(ReportValidationError::Missing {
            field: "flythrough save",
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

fn validate_carve_storm(report: &BenchmarkReport) -> Result<(), ReportValidationError> {
    if report.mutation_latency.is_none()
        && (report.passed
            || !report
                .failure_reasons
                .iter()
                .any(|reason| reason == "mutation_latency"))
    {
        return Err(ReportValidationError::Missing {
            field: "mutation_latency",
        });
    }
    if let Some(value) = &report.mutation_latency {
        validate_mutation_latency(value, report.passed)?;
    }
    let save = &report.save;
    if save.completed && !save.attempted
        || !save.completed && save.round_trip.is_some()
        || !save.attempted
            && (save.completed
                || save.size_bytes.is_some()
                || save.changed_voxels.is_some()
                || save.changed_bricks.is_some()
                || save.round_trip.is_some())
    {
        return Err(ReportValidationError::Inconsistent {
            field: "mutation save state",
        });
    }
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
        && round_trip.passed
        && !(round_trip.delta_voxels_compared > 0
            && round_trip.base_samples_compared > 0
            && round_trip.identity_match
            && !round_trip.derived_bytes_found)
    {
        return Err(ReportValidationError::Inconsistent {
            field: "round_trip",
        });
    }
    for (field, value) in [
        ("save.size_bytes", save.size_bytes),
        ("save.changed_voxels", save.changed_voxels.map(u64::from)),
        ("save.changed_bricks", save.changed_bricks.map(u64::from)),
    ] {
        if value == Some(0) {
            return Err(ReportValidationError::Missing { field });
        }
    }
    if report.passed
        && (save.size_bytes.is_some_and(|bytes| bytes >= 50_000_000)
            || save.changed_bricks.is_some_and(|bricks| bricks < 256)
            || save.round_trip.is_some_and(|round_trip| !round_trip.passed))
    {
        return Err(ReportValidationError::Limit {
            field: "mutation save",
        });
    }
    Ok(())
}

fn validate_build(build: &BuildProfile, passed: bool) -> Result<(), ReportValidationError> {
    if passed {
        build.validate_release()
    } else {
        build.validate_complete()
    }
}

fn validate_asset_evidence(
    value: &AssetEvidence,
    passed: bool,
) -> Result<(), ReportValidationError> {
    if !sha256(&value.manifest_sha256)
        || !sorted_unique(&value.fallbacks)
        || !sorted_unique(&value.warnings)
    {
        return Err(ReportValidationError::Identity { field: "assets" });
    }
    if passed && !value.fallbacks.is_empty() {
        return Err(ReportValidationError::Inconsistent {
            field: "asset fallbacks",
        });
    }
    Ok(())
}

fn validate_frame_rate(value: FrameRateMetrics, passed: bool) -> Result<(), ReportValidationError> {
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
        finite_positive(metric, name)?;
    }
    let calculated_fps = value.sample_count as f64 / value.measured_seconds;
    if (calculated_fps - value.arithmetic_fps).abs() > calculated_fps.abs() * 0.000_001 {
        return Err(ReportValidationError::Inconsistent {
            field: "arithmetic_fps",
        });
    }
    if passed && value.arithmetic_fps < 60.0 {
        return Err(ReportValidationError::Limit {
            field: "arithmetic_fps",
        });
    }
    Ok(())
}

fn validate_graphics_memory(
    value: &GraphicsMemoryEvidence,
    passed: bool,
    reasons: &[String],
) -> Result<(), ReportValidationError> {
    if value.application_ledger.peak_bytes == 0
        || value.application_ledger.end_bytes == 0
        || value.application_ledger.categories.is_empty()
        || value
            .application_ledger
            .categories
            .values()
            .any(|bytes| *bytes == 0)
    {
        return Err(ReportValidationError::Missing {
            field: "application_ledger",
        });
    }
    if !value.application_ledger.untracked_driver_overhead
        || value.application_ledger.end_bytes > value.application_ledger.peak_bytes
        || value
            .application_ledger
            .categories
            .iter()
            .any(|(name, _)| blank(name))
    {
        return Err(ReportValidationError::Inconsistent {
            field: "application_ledger",
        });
    }
    let category_total = value
        .application_ledger
        .categories
        .values()
        .try_fold(0_u64, |total, bytes| total.checked_add(*bytes))
        .ok_or(ReportValidationError::Inconsistent {
            field: "application_ledger",
        })?;
    if category_total != value.application_ledger.peak_bytes {
        return Err(ReportValidationError::Inconsistent {
            field: "application_ledger",
        });
    }
    if let Some(measurement) = &value.resident_measurement {
        let provider = measurement.provider.to_ascii_lowercase();
        let scope = measurement.scope.to_ascii_lowercase();
        if blank(&measurement.provider)
            || blank(&measurement.scope)
            || measurement.sampling_interval_ms == 0
            || measurement.peak_bytes == 0
            || !sha256(&measurement.artifact_sha256)
            || blank(&measurement.artifact_path)
        {
            return Err(ReportValidationError::Missing {
                field: "resident_measurement",
            });
        }
        if provider.contains("application allocation ledger")
            || provider == "application ledger"
            || scope.contains("application allocation ledger")
            || scope.contains("application ledger")
        {
            return Err(ReportValidationError::Identity {
                field: "resident_measurement",
            });
        }
        let covers_target = scope.contains("game process")
            && scope.contains("resident graphics")
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
        .is_some_and(blank)
    {
        return Err(ReportValidationError::Identity {
            field: "estimate_substitution_approval_id",
        });
    }
    let approved_estimate = value.estimate_substitution_approval_id.is_some()
        && value.application_ledger.peak_bytes < GRAPHICS_MEMORY_TARGET_BYTES;
    if passed && !value.product_target_proven && !approved_estimate {
        return Err(ReportValidationError::Inconsistent {
            field: "resident graphics memory",
        });
    }
    if !passed
        && !value.product_target_proven
        && !approved_estimate
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

fn validate_coverage(
    value: &CoverageEvidence,
    scenario: ScenarioName,
    passed: bool,
) -> Result<(), ReportValidationError> {
    if value.route_tags_visited.is_empty()
        || !sorted_unique(&value.route_tags_visited)
        || value.active_bands_entered.is_empty()
        || !value
            .active_bands_entered
            .windows(2)
            .all(|pair| pair[0] < pair[1])
        || value.edited_material_counts.keys().any(|key| blank(key))
    {
        if value.active_bands_entered.is_empty() {
            return Err(ReportValidationError::Missing {
                field: "coverage.active_bands_entered",
            });
        }
        return Err(ReportValidationError::Inconsistent { field: "coverage" });
    }
    if passed
        && (value.active_bands_entered != ALL_ACTIVE_BANDS
            || !value.workload_minimum_met
            || match scenario {
                ScenarioName::Flythrough => {
                    !value.edited_material_counts.is_empty()
                        || value.final_changed_spheres != 0
                        || value.final_changed_region_cells != 0
                }
                ScenarioName::CarveStorm => {
                    value.edited_material_counts.is_empty()
                        || value
                            .edited_material_counts
                            .values()
                            .any(|count| *count == 0)
                        || value.final_changed_spheres == 0
                        || value.final_changed_region_cells == 0
                }
            })
    {
        return Err(ReportValidationError::Inconsistent {
            field: "coverage pass",
        });
    }
    Ok(())
}

fn validate_streaming(
    value: &StreamingEvidence,
    passed: bool,
) -> Result<(), ReportValidationError> {
    if value.peak_active_counts.bricks == 0
        || value.peak_active_counts.meshes == 0
        || value.peak_active_counts.objects == 0
        || value.peak_queue_depths.extraction == 0
        || value.peak_queue_depths.installation == 0
        || value.peak_queue_depths.render == 0
        || value.first_steady_derived_bytes == 0
        || value.return_steady_derived_bytes == 0
    {
        return Err(ReportValidationError::Missing {
            field: "streaming measurement",
        });
    }
    if passed {
        value.object_index.validate_complete()?;
    } else {
        value.object_index.validate_measured()?;
    }
    if passed
        && (!value.monotonic_growth_check_passed
            || value.return_steady_derived_bytes
                > value.first_steady_derived_bytes.saturating_mul(105) / 100)
    {
        return Err(ReportValidationError::Inconsistent {
            field: "monotonic growth",
        });
    }
    Ok(())
}

fn validate_mutation_latency(
    value: &MutationLatencyMetrics,
    passed: bool,
) -> Result<(), ReportValidationError> {
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
        finite_positive(metric, name)?;
    }
    if passed
        && (value.admission_ms.max > 2.0
            || value.accepted_to_first_commit_ms.max > 250.0
            || value.commit_to_primary_ready_ms.p95 > 250.0
            || value.commit_to_primary_ready_ms.max > 500.0
            || value.accepted_to_reconciliation_ms.max > 30_000.0
            || value.changed_bricks_per_second < 32.0
            || value.maximum_runnable_wait_ms > 500.0
            || value.representative_max_frame_ms > 33.3)
    {
        return Err(ReportValidationError::Limit {
            field: "mutation latency",
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
fn sorted_unique(values: &[String]) -> bool {
    values.iter().all(|value| !blank(value)) && values.windows(2).all(|pair| pair[0] < pair[1])
}
fn sha256(value: &str) -> bool {
    value.len() == 64
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

fn validate_world(world: &WorldIdentity) -> Result<(), ReportValidationError> {
    if world.seed != PRODUCT_ONE_SEED
        || world.parameters_digest.iter().all(|byte| *byte == 0)
        || world.bounds.min() != WorldPointQ8::new(-128_000, -32_768, -128_000)
        || world.bounds.max_exclusive() != WorldPointQ8::new(128_000, 32_768, 128_000)
    {
        return Err(ReportValidationError::Identity { field: "world" });
    }
    Ok(())
}

fn validate_machine(
    machine: &MachineProfile,
    resolution: [u32; 2],
) -> Result<(), ReportValidationError> {
    machine.validate_complete()?;
    match machine.wgpu_backend.as_str() {
        "metal" => {
            machine.validate_m4_acceptance()?;
            if !matches!(resolution, [1920, 1080] | [2560, 1440]) {
                return Err(ReportValidationError::Identity {
                    field: "M4 resolution",
                });
            }
        }
        "vulkan" => {
            if resolution != [2560, 1440]
                || machine.os_name != "Linux"
                || machine.memory_architecture != "discrete"
                || machine.gpu_device_class != "discrete"
                || !machine.gpu_adapter_name.contains("3060")
            {
                return Err(ReportValidationError::Identity {
                    field: "Linux 3060 machine",
                });
            }
        }
        _ => return Err(ReportValidationError::Identity { field: "machine" }),
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

fn json_nullable_object_has_keys(value: Option<&serde_json::Value>, keys: &[&str]) -> bool {
    value.is_some_and(|value| value.is_null() || json_object_has_keys(value, keys))
}
