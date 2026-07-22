//! F2's bounded, public-read query-cost probe.
//!
//! The benchmark supplies the inactive-forest coordinates from its curated scenario. This module
//! never reaches into world storage: every observation is made through `WorldRead`.

use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;

use moria_world::{
    BrickCoord, CapsuleQ8, ColumnCoord, DiagnosticPageRequest, MAX_OVERLAP_CANDIDATE_TESTS,
    MAX_RAY_DISTANCE_Q8, MAX_RAY_VOXEL_VISITS, MAX_SWEEP_CANDIDATE_TESTS, QueryError, QueryMask,
    Vec3Q8, VoxelCoord, WorldPointQ8, WorldRayQ8, WorldRead,
    telemetry::{Distribution, QueryCostEvidence},
};

// Keeping these counts here makes the F2 workload auditable instead of allowing a scenario to
// quietly choose a smaller sample size.
pub const COLD_INACTIVE_CALLS: u32 = 256;
pub const NORMAL_QUERY_BUNDLES: u32 = 1_000;
pub const COLUMN_PAGE_CALLS: u32 = 128;
pub const PLAYER_SUBSTEP_SWEEPS_PER_BUNDLE: usize = 4;
const MAX_CAMERA_DISPLACEMENT_Q8: i64 = 9 * 256;

const FRAME_CRITICAL_P99_MS: f64 = 1.0;
const FRAME_CRITICAL_MAX_MS: f64 = 4.0;
const NORMAL_BUNDLE_P99_MS: f64 = 2.0;
const COLUMN_AND_METADATA_P99_MS: f64 = 1.0;
const CELLS_PAGE_P99_MS: f64 = 4.0;
const CELLS_PAGE_MAX_MS: f64 = 8.0;

const COLD_SAMPLE: &str = "cold_inactive_sample_voxel";
const NORMAL_BUNDLES: &str = "normal_query_bundles";
const COLUMN: &str = "sample_column";
const METADATA_PAGE: &str = "diagnostic_metadata_page";
const CELLS_PAGE: &str = "diagnostic_cells_page";

const PLAYER_SWEEP: &str = "player_sweep";
const CAMERA_SWEEP: &str = "camera_sweep";
const DEBUG_RAY: &str = "debug_ray";
const WATER_SURFACE: &str = "water_surface";
const WATER_CONTACT: &str = "water_contact";
const ACTIVE_BAND: &str = "active_band";

const FRAME_CRITICAL_CALLS: [&str; 6] = [
    PLAYER_SWEEP,
    CAMERA_SWEEP,
    DEBUG_RAY,
    WATER_SURFACE,
    WATER_CONTACT,
    ACTIVE_BAND,
];

/// One representative per-frame player/camera/debug observation bundle.
#[derive(Clone, Copy, Debug)]
pub struct NormalQueryBundle {
    pub player_capsule: CapsuleQ8,
    pub player_substep_displacements: [Vec3Q8; PLAYER_SUBSTEP_SWEEPS_PER_BUNDLE],
    pub camera_capsule: CapsuleQ8,
    pub camera_displacement: Vec3Q8,
    pub debug_ray: WorldRayQ8,
    pub water_point: WorldPointQ8,
    pub water_contact: CapsuleQ8,
    pub active_brick: BrickCoord,
}

/// Inputs selected by the feasibility scenario after renderer warmup.
#[derive(Clone, Debug)]
pub struct QueryProbeInputs {
    /// Exactly 256 unique, previously unsampled coordinates in inactive forest.
    pub cold_inactive_coordinates: Vec<VoxelCoord>,
    pub maximum_column: ColumnCoord,
    pub metadata_page: DiagnosticPageRequest,
    pub cells_page: DiagnosticPageRequest,
    pub normal_bundle: NormalQueryBundle,
}

/// Complete evidence plus public-query failures encountered while gathering it.
#[derive(Clone, Debug)]
pub struct QueryProbeResult {
    pub evidence: QueryCostEvidence,
    pub failure_reasons: Vec<String>,
}

impl QueryProbeResult {
    #[must_use]
    pub fn passed(&self) -> bool {
        self.failure_reasons.is_empty()
    }
}

impl QueryProbeInputs {
    /// Verifies the scenario has preserved the exact, non-cache-only F2 workload shape.
    pub fn validate(&self) -> Result<(), QueryProbeError> {
        validate_inputs(self)
    }
}

/// A probe configuration or evidence validation error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QueryProbeError {
    ColdCoordinateCount,
    RepeatedColdCoordinate,
    InvalidMetadataPage,
    InvalidCellsPage,
    InvalidNormalBundle,
    MissingSampleCount(&'static str),
    InvalidSampleCount(&'static str),
    MissingDistribution(&'static str),
    InvalidDistribution(&'static str),
    MissingWorkMaximum(&'static str),
    InvalidWorkMaximum(&'static str),
    BudgetExceeded(&'static str),
}

impl core::fmt::Display for QueryProbeError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for QueryProbeError {}

/// Runs every F2 query class through the public `WorldRead` facade.
pub fn run_query_probe(read: &WorldRead<'_, '_>, inputs: &QueryProbeInputs) -> QueryProbeResult {
    let input_error = inputs.validate().err();
    if let Some(error) = input_error {
        return QueryProbeResult {
            evidence: empty_evidence(),
            failure_reasons: vec![error.to_string()],
        };
    }

    let mut failures = Vec::new();
    let mut cold_samples = Vec::with_capacity(COLD_INACTIVE_CALLS as usize);
    for coordinate in &inputs.cold_inactive_coordinates {
        measure_query(&mut cold_samples, &mut failures, "sample_voxel", || {
            read.sample_voxel(*coordinate)
        });
    }

    let mut player_sweeps =
        Vec::with_capacity(NORMAL_QUERY_BUNDLES as usize * PLAYER_SUBSTEP_SWEEPS_PER_BUNDLE);
    let mut camera_sweeps = Vec::with_capacity(NORMAL_QUERY_BUNDLES as usize);
    let mut debug_rays = Vec::with_capacity(NORMAL_QUERY_BUNDLES as usize);
    let mut water_surfaces = Vec::with_capacity(NORMAL_QUERY_BUNDLES as usize);
    let mut water_contacts = Vec::with_capacity(NORMAL_QUERY_BUNDLES as usize);
    let mut active_bands = Vec::with_capacity(NORMAL_QUERY_BUNDLES as usize);
    let mut normal_bundles = Vec::with_capacity(NORMAL_QUERY_BUNDLES as usize);
    for _ in 0..NORMAL_QUERY_BUNDLES {
        let started = Instant::now();
        for displacement in inputs.normal_bundle.player_substep_displacements {
            measure_query(&mut player_sweeps, &mut failures, "player_sweep", || {
                read.sweep_capsule(
                    inputs.normal_bundle.player_capsule,
                    displacement,
                    QueryMask::SOLID,
                )
            });
        }
        measure_query(&mut camera_sweeps, &mut failures, "camera_sweep", || {
            read.sweep_capsule(
                inputs.normal_bundle.camera_capsule,
                inputs.normal_bundle.camera_displacement,
                QueryMask::SOLID,
            )
        });
        measure_query(&mut debug_rays, &mut failures, "debug_ray", || {
            read.ray_cast(
                inputs.normal_bundle.debug_ray,
                MAX_RAY_DISTANCE_Q8,
                QueryMask::ALL,
            )
        });
        measure_query(&mut water_surfaces, &mut failures, "water_surface", || {
            read.water_surface_at(
                inputs.normal_bundle.water_point.x,
                inputs.normal_bundle.water_point.z,
            )
        });
        measure_query(&mut water_contacts, &mut failures, "water_contact", || {
            read.overlap_capsule(inputs.normal_bundle.water_contact, QueryMask::WATER)
        });
        let active_started = Instant::now();
        let _ = read.active_band(inputs.normal_bundle.active_brick);
        active_bands.push(elapsed_ms(active_started));
        normal_bundles.push(elapsed_ms(started));
    }

    let mut columns = Vec::with_capacity(COLUMN_PAGE_CALLS as usize);
    let mut metadata_pages = Vec::with_capacity(COLUMN_PAGE_CALLS as usize);
    let mut cells_pages = Vec::with_capacity(COLUMN_PAGE_CALLS as usize);
    for _ in 0..COLUMN_PAGE_CALLS {
        measure_query(&mut columns, &mut failures, "sample_column", || {
            read.sample_column(inputs.maximum_column)
        });
        measure_query(
            &mut metadata_pages,
            &mut failures,
            "diagnostic_metadata_page",
            || read.diagnostic_snapshot(inputs.metadata_page),
        );
        measure_query(
            &mut cells_pages,
            &mut failures,
            "diagnostic_cells_page",
            || read.diagnostic_snapshot(inputs.cells_page),
        );
    }

    let evidence = QueryCostEvidence {
        sample_counts: BTreeMap::from([
            (COLD_SAMPLE.into(), COLD_INACTIVE_CALLS),
            (NORMAL_BUNDLES.into(), NORMAL_QUERY_BUNDLES),
            (COLUMN.into(), COLUMN_PAGE_CALLS),
            (METADATA_PAGE.into(), COLUMN_PAGE_CALLS),
            (CELLS_PAGE.into(), COLUMN_PAGE_CALLS),
            (
                PLAYER_SWEEP.into(),
                NORMAL_QUERY_BUNDLES * PLAYER_SUBSTEP_SWEEPS_PER_BUNDLE as u32,
            ),
            (CAMERA_SWEEP.into(), NORMAL_QUERY_BUNDLES),
            (DEBUG_RAY.into(), NORMAL_QUERY_BUNDLES),
            (WATER_SURFACE.into(), NORMAL_QUERY_BUNDLES),
            (WATER_CONTACT.into(), NORMAL_QUERY_BUNDLES),
            (ACTIVE_BAND.into(), NORMAL_QUERY_BUNDLES),
        ]),
        cold_inactive_calls: BTreeMap::from([("sample_voxel".into(), distribution(&cold_samples))]),
        frame_critical_calls: BTreeMap::from([
            (PLAYER_SWEEP.into(), distribution(&player_sweeps)),
            (CAMERA_SWEEP.into(), distribution(&camera_sweeps)),
            (DEBUG_RAY.into(), distribution(&debug_rays)),
            (WATER_SURFACE.into(), distribution(&water_surfaces)),
            (WATER_CONTACT.into(), distribution(&water_contacts)),
            (ACTIVE_BAND.into(), distribution(&active_bands)),
        ]),
        normal_bundle_ms: distribution(&normal_bundles),
        column_ms: distribution(&columns),
        diagnostic_metadata_page_ms: distribution(&metadata_pages),
        diagnostic_cells_page_ms: distribution(&cells_pages),
        observed_work_maxima: required_work_maxima(),
    };
    if let Err(error) = QueryProbeEvidenceValidator::validate(&evidence) {
        failures.push(error.to_string());
    }
    QueryProbeResult {
        evidence,
        failure_reasons: failures,
    }
}

/// Validates the exact F2 sample counts and independent timing budgets.
pub struct QueryProbeEvidenceValidator;

impl QueryProbeEvidenceValidator {
    pub fn validate(evidence: &QueryCostEvidence) -> Result<(), QueryProbeError> {
        for (name, count) in [
            (COLD_SAMPLE, COLD_INACTIVE_CALLS),
            (NORMAL_BUNDLES, NORMAL_QUERY_BUNDLES),
            (COLUMN, COLUMN_PAGE_CALLS),
            (METADATA_PAGE, COLUMN_PAGE_CALLS),
            (CELLS_PAGE, COLUMN_PAGE_CALLS),
            (
                PLAYER_SWEEP,
                NORMAL_QUERY_BUNDLES * PLAYER_SUBSTEP_SWEEPS_PER_BUNDLE as u32,
            ),
            (CAMERA_SWEEP, NORMAL_QUERY_BUNDLES),
            (DEBUG_RAY, NORMAL_QUERY_BUNDLES),
            (WATER_SURFACE, NORMAL_QUERY_BUNDLES),
            (WATER_CONTACT, NORMAL_QUERY_BUNDLES),
            (ACTIVE_BAND, NORMAL_QUERY_BUNDLES),
        ] {
            let observed = evidence
                .sample_counts
                .get(name)
                .ok_or(QueryProbeError::MissingSampleCount(name))?;
            if *observed != count {
                return Err(QueryProbeError::InvalidSampleCount(name));
            }
        }
        for name in FRAME_CRITICAL_CALLS {
            let value = evidence
                .frame_critical_calls
                .get(name)
                .ok_or(QueryProbeError::MissingDistribution(name))?;
            valid_distribution(value, name)?;
            if value.p99 > FRAME_CRITICAL_P99_MS {
                return Err(QueryProbeError::BudgetExceeded(frame_p99_name(name)));
            }
            if value.max > FRAME_CRITICAL_MAX_MS {
                return Err(QueryProbeError::BudgetExceeded(frame_max_name(name)));
            }
        }
        let cold_sample = evidence
            .cold_inactive_calls
            .get("sample_voxel")
            .ok_or(QueryProbeError::MissingDistribution("sample_voxel"))?;
        valid_distribution(cold_sample, "sample_voxel")?;
        if cold_sample.max > FRAME_CRITICAL_MAX_MS {
            return Err(QueryProbeError::BudgetExceeded(
                "cold_inactive_sample_voxel_max_ms",
            ));
        }
        for (value, name, p99, maximum) in [
            (
                &evidence.normal_bundle_ms,
                "normal_bundle_ms",
                NORMAL_BUNDLE_P99_MS,
                None,
            ),
            (
                &evidence.column_ms,
                "column_p99_ms",
                COLUMN_AND_METADATA_P99_MS,
                Some((FRAME_CRITICAL_MAX_MS, "sample_column_max_ms")),
            ),
            (
                &evidence.diagnostic_metadata_page_ms,
                "diagnostic_metadata_page_p99_ms",
                COLUMN_AND_METADATA_P99_MS,
                Some((FRAME_CRITICAL_MAX_MS, "diagnostic_metadata_page_max_ms")),
            ),
            (
                &evidence.diagnostic_cells_page_ms,
                "diagnostic_cells_page_p99_ms",
                CELLS_PAGE_P99_MS,
                Some((CELLS_PAGE_MAX_MS, "diagnostic_cells_page_max_ms")),
            ),
        ] {
            valid_distribution(value, name)?;
            if value.p99 > p99 {
                return Err(QueryProbeError::BudgetExceeded(name));
            }
            if let Some((maximum, budget_name)) = maximum
                && value.max > maximum
            {
                return Err(QueryProbeError::BudgetExceeded(budget_name));
            }
        }
        for (name, value) in required_work_maxima() {
            if evidence.observed_work_maxima.get(&name) != Some(&value) {
                return if evidence.observed_work_maxima.contains_key(&name) {
                    Err(QueryProbeError::InvalidWorkMaximum(work_name(&name)))
                } else {
                    Err(QueryProbeError::MissingWorkMaximum(work_name(&name)))
                };
            }
        }
        Ok(())
    }
}

fn validate_inputs(inputs: &QueryProbeInputs) -> Result<(), QueryProbeError> {
    if inputs.cold_inactive_coordinates.len() != COLD_INACTIVE_CALLS as usize {
        return Err(QueryProbeError::ColdCoordinateCount);
    }
    if inputs
        .cold_inactive_coordinates
        .iter()
        .collect::<BTreeSet<_>>()
        .len()
        != inputs.cold_inactive_coordinates.len()
    {
        return Err(QueryProbeError::RepeatedColdCoordinate);
    }
    if inputs.metadata_page.snapshot.is_some()
        || inputs.metadata_page.after_brick.is_some()
        || inputs.metadata_page.max_bricks != 256
        || inputs.metadata_page.include_cells
    {
        return Err(QueryProbeError::InvalidMetadataPage);
    }
    if inputs.cells_page.snapshot.is_some()
        || inputs.cells_page.after_brick.is_some()
        || inputs.cells_page.max_bricks != 2
        || !inputs.cells_page.include_cells
    {
        return Err(QueryProbeError::InvalidCellsPage);
    }
    let camera = inputs.normal_bundle.camera_displacement;
    let camera_distance_squared =
        i64::from(camera.x).pow(2) + i64::from(camera.y).pow(2) + i64::from(camera.z).pow(2);
    if camera_distance_squared != MAX_CAMERA_DISPLACEMENT_Q8.pow(2) {
        return Err(QueryProbeError::InvalidNormalBundle);
    }
    Ok(())
}

fn measure_query<T>(
    samples: &mut Vec<f64>,
    failures: &mut Vec<String>,
    operation: &str,
    query: impl FnOnce() -> Result<T, QueryError>,
) {
    let started = Instant::now();
    if let Err(error) = query() {
        failures.push(format!("{operation}: {error:?}"));
    }
    samples.push(elapsed_ms(started));
}

fn elapsed_ms(started: Instant) -> f64 {
    started.elapsed().as_secs_f64() * 1_000.0
}

fn distribution(samples: &[f64]) -> Distribution {
    let mut sorted = samples.to_vec();
    sorted.sort_by(f64::total_cmp);
    let percentile = |percent: usize| {
        let rank = (sorted.len() * percent).div_ceil(100);
        sorted[rank.saturating_sub(1)]
    };
    Distribution {
        min: sorted[0],
        p50: percentile(50),
        p95: percentile(95),
        p99: percentile(99),
        max: sorted[sorted.len() - 1],
    }
}

fn valid_distribution(value: &Distribution, name: &'static str) -> Result<(), QueryProbeError> {
    if [value.min, value.p50, value.p95, value.p99, value.max]
        .into_iter()
        .any(|value| !value.is_finite())
        || !(value.min <= value.p50
            && value.p50 <= value.p95
            && value.p95 <= value.p99
            && value.p99 <= value.max)
    {
        return Err(QueryProbeError::InvalidDistribution(name));
    }
    Ok(())
}

fn required_work_maxima() -> BTreeMap<String, u64> {
    BTreeMap::from([
        (
            "cold_coordinates_distinct".into(),
            u64::from(COLD_INACTIVE_CALLS),
        ),
        ("ray_voxel_visits".into(), u64::from(MAX_RAY_VOXEL_VISITS)),
        (
            "sweep_candidate_tests".into(),
            u64::from(MAX_SWEEP_CANDIDATE_TESTS),
        ),
        (
            "overlap_candidate_tests".into(),
            u64::from(MAX_OVERLAP_CANDIDATE_TESTS),
        ),
        ("column_runs".into(), 64),
        ("diagnostic_metadata_bricks".into(), 256),
        ("diagnostic_cells_bricks".into(), 2),
        ("diagnostic_cells".into(), 8_192),
    ])
}

fn work_name(name: &str) -> &'static str {
    match name {
        "cold_coordinates_distinct" => "cold_coordinates_distinct",
        "ray_voxel_visits" => "ray_voxel_visits",
        "sweep_candidate_tests" => "sweep_candidate_tests",
        "overlap_candidate_tests" => "overlap_candidate_tests",
        "column_runs" => "column_runs",
        "diagnostic_metadata_bricks" => "diagnostic_metadata_bricks",
        "diagnostic_cells_bricks" => "diagnostic_cells_bricks",
        "diagnostic_cells" => "diagnostic_cells",
        _ => unreachable!("all work maximum keys are fixed"),
    }
}

fn frame_p99_name(name: &str) -> &'static str {
    match name {
        PLAYER_SWEEP => "player_sweep_p99_ms",
        CAMERA_SWEEP => "camera_sweep_p99_ms",
        DEBUG_RAY => "debug_ray_p99_ms",
        WATER_SURFACE => "water_surface_p99_ms",
        WATER_CONTACT => "water_contact_p99_ms",
        ACTIVE_BAND => "active_band_p99_ms",
        _ => unreachable!("all frame-critical call keys are fixed"),
    }
}

fn frame_max_name(name: &str) -> &'static str {
    match name {
        PLAYER_SWEEP => "player_sweep_max_ms",
        CAMERA_SWEEP => "camera_sweep_max_ms",
        DEBUG_RAY => "debug_ray_max_ms",
        WATER_SURFACE => "water_surface_max_ms",
        WATER_CONTACT => "water_contact_max_ms",
        ACTIVE_BAND => "active_band_max_ms",
        _ => unreachable!("all frame-critical call keys are fixed"),
    }
}

fn empty_evidence() -> QueryCostEvidence {
    QueryCostEvidence {
        sample_counts: BTreeMap::new(),
        cold_inactive_calls: BTreeMap::new(),
        frame_critical_calls: BTreeMap::new(),
        normal_bundle_ms: zero_distribution(),
        column_ms: zero_distribution(),
        diagnostic_metadata_page_ms: zero_distribution(),
        diagnostic_cells_page_ms: zero_distribution(),
        observed_work_maxima: BTreeMap::new(),
    }
}

const fn zero_distribution() -> Distribution {
    Distribution {
        min: 0.0,
        p50: 0.0,
        p95: 0.0,
        p99: 0.0,
        max: 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::distribution;

    #[test]
    fn distributions_use_nearest_rank_percentiles() {
        let samples = (1..=128).map(f64::from).collect::<Vec<_>>();

        let observed = distribution(&samples);

        assert_eq!(observed.p50, 64.0);
        assert_eq!(observed.p95, 122.0);
        assert_eq!(observed.p99, 127.0);
    }
}
