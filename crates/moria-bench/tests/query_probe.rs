use std::collections::BTreeMap;

use moria_bench::scenarios::query_probe::{
    COLD_INACTIVE_CALLS, COLUMN_PAGE_CALLS, NORMAL_QUERY_BUNDLES, NormalQueryBundle,
    QueryProbeError, QueryProbeEvidenceValidator, QueryProbeInputs,
};
use moria_world::{
    BrickCoord, CapsuleQ8, ColumnCoord, DiagnosticPageRequest, Vec3Q8, VoxelCoord, WorldPointQ8,
    WorldRayQ8,
    telemetry::{Distribution, QueryCostEvidence},
};

fn distribution(max: f64) -> Distribution {
    Distribution {
        min: 0.1,
        p50: 0.2,
        p95: 0.3,
        p99: max,
        max,
    }
}

fn complete_evidence() -> QueryCostEvidence {
    QueryCostEvidence {
        sample_counts: BTreeMap::from([
            ("cold_inactive_sample_voxel".into(), COLD_INACTIVE_CALLS),
            ("normal_query_bundles".into(), NORMAL_QUERY_BUNDLES),
            ("sample_column".into(), COLUMN_PAGE_CALLS),
            ("diagnostic_metadata_page".into(), COLUMN_PAGE_CALLS),
            ("diagnostic_cells_page".into(), COLUMN_PAGE_CALLS),
            ("player_sweep".into(), NORMAL_QUERY_BUNDLES * 4),
            ("camera_sweep".into(), NORMAL_QUERY_BUNDLES),
            ("debug_ray".into(), NORMAL_QUERY_BUNDLES),
            ("water_surface".into(), NORMAL_QUERY_BUNDLES),
            ("water_contact".into(), NORMAL_QUERY_BUNDLES),
            ("active_band".into(), NORMAL_QUERY_BUNDLES),
        ]),
        cold_inactive_calls: BTreeMap::from([("sample_voxel".into(), distribution(1.0))]),
        frame_critical_calls: BTreeMap::from([
            ("player_sweep".into(), distribution(1.0)),
            ("camera_sweep".into(), distribution(1.0)),
            ("debug_ray".into(), distribution(1.0)),
            ("water_surface".into(), distribution(1.0)),
            ("water_contact".into(), distribution(1.0)),
            ("active_band".into(), distribution(1.0)),
        ]),
        normal_bundle_ms: distribution(2.0),
        column_ms: distribution(1.0),
        diagnostic_metadata_page_ms: distribution(1.0),
        diagnostic_cells_page_ms: distribution(4.0),
        observed_work_maxima: BTreeMap::from([
            (
                "cold_coordinates_distinct".into(),
                u64::from(COLD_INACTIVE_CALLS),
            ),
            ("ray_voxel_visits".into(), 448),
            ("sweep_candidate_tests".into(), 8_192),
            ("overlap_candidate_tests".into(), 512),
            ("column_runs".into(), 64),
            ("diagnostic_metadata_bricks".into(), 256),
            ("diagnostic_cells_bricks".into(), 2),
            ("diagnostic_cells".into(), 8_192),
        ]),
    }
}

fn query_probe_inputs() -> QueryProbeInputs {
    let cold_inactive_coordinates = (0..COLD_INACTIVE_CALLS)
        .map(|index| VoxelCoord::new(100 + index as i32, 100, 100))
        .collect();
    QueryProbeInputs {
        cold_inactive_coordinates,
        maximum_column: ColumnCoord { x: 0, z: 0 },
        metadata_page: DiagnosticPageRequest {
            snapshot: None,
            after_brick: None,
            max_bricks: 256,
            include_cells: false,
        },
        cells_page: DiagnosticPageRequest {
            snapshot: None,
            after_brick: None,
            max_bricks: 2,
            include_cells: true,
        },
        normal_bundle: NormalQueryBundle {
            player_capsule: CapsuleQ8::new(WorldPointQ8::new(0, 0, 0), 32, 0),
            player_substep_displacements: [Vec3Q8::new(0, 0, 0); 4],
            camera_capsule: CapsuleQ8::new(WorldPointQ8::new(0, 0, 0), 32, 0),
            camera_displacement: Vec3Q8::new(9 * 256, 0, 0),
            debug_ray: WorldRayQ8::new(WorldPointQ8::new(0, 0, 0), [65_536, 0, 0]).unwrap(),
            water_point: WorldPointQ8::new(0, 0, 0),
            water_contact: CapsuleQ8::new(WorldPointQ8::new(0, 0, 0), 32, 0),
            active_brick: BrickCoord::new(0, 0, 0).unwrap(),
        },
    }
}

#[test]
fn complete_query_probe_evidence_meets_each_independent_budget() {
    QueryProbeEvidenceValidator::validate(&complete_evidence()).unwrap();
}

#[test]
fn query_probe_rejects_missing_counts_and_an_independent_page_overrun() {
    let mut evidence = complete_evidence();
    evidence.sample_counts.remove("normal_query_bundles");
    assert_eq!(
        QueryProbeEvidenceValidator::validate(&evidence),
        Err(QueryProbeError::MissingSampleCount("normal_query_bundles"))
    );

    let mut evidence = complete_evidence();
    evidence.diagnostic_cells_page_ms.p99 = 4.1;
    evidence.diagnostic_cells_page_ms.max = 4.1;
    assert_eq!(
        QueryProbeEvidenceValidator::validate(&evidence),
        Err(QueryProbeError::BudgetExceeded(
            "diagnostic_cells_page_p99_ms"
        ))
    );
}

#[test]
fn query_probe_rejects_every_non_cell_query_over_four_milliseconds() {
    let mut evidence = complete_evidence();
    evidence.cold_inactive_calls.clear();
    assert_eq!(
        QueryProbeEvidenceValidator::validate(&evidence),
        Err(QueryProbeError::MissingDistribution("sample_voxel"))
    );

    let mut evidence = complete_evidence();
    evidence
        .cold_inactive_calls
        .get_mut("sample_voxel")
        .unwrap()
        .max = 4.1;
    assert_eq!(
        QueryProbeEvidenceValidator::validate(&evidence),
        Err(QueryProbeError::BudgetExceeded(
            "cold_inactive_sample_voxel_max_ms"
        ))
    );

    let mut evidence = complete_evidence();
    evidence.column_ms.max = 4.1;
    assert_eq!(
        QueryProbeEvidenceValidator::validate(&evidence),
        Err(QueryProbeError::BudgetExceeded("sample_column_max_ms"))
    );

    let mut evidence = complete_evidence();
    evidence.diagnostic_metadata_page_ms.max = 4.1;
    assert_eq!(
        QueryProbeEvidenceValidator::validate(&evidence),
        Err(QueryProbeError::BudgetExceeded(
            "diagnostic_metadata_page_max_ms"
        ))
    );
}

#[test]
fn query_probe_rejects_repeated_cold_coordinates_before_any_cacheable_read() {
    let mut inputs = query_probe_inputs();
    inputs.cold_inactive_coordinates[1] = inputs.cold_inactive_coordinates[0];

    assert_eq!(
        inputs.validate(),
        Err(QueryProbeError::RepeatedColdCoordinate)
    );
}

#[test]
fn maximum_diagnostic_pages_must_start_at_the_first_brick() {
    let mut inputs = query_probe_inputs();
    inputs.metadata_page.after_brick = Some(BrickCoord::new(1, 1, 1).unwrap());

    assert_eq!(inputs.validate(), Err(QueryProbeError::InvalidMetadataPage));

    inputs.metadata_page.after_brick = None;
    assert_eq!(inputs.validate(), Ok(()));
}

#[test]
fn normal_bundle_requires_the_contracted_nine_meter_camera_probe() {
    let mut inputs = query_probe_inputs();
    inputs.normal_bundle.camera_displacement = Vec3Q8::new(0, 0, 0);

    assert_eq!(inputs.validate(), Err(QueryProbeError::InvalidNormalBundle));
}
