use std::collections::BTreeMap;

use moria_world::telemetry::{
    BuildProfile, Distribution, ForestFeasibilityReport, MachineProfile, MutationFeasibilityReport,
    MutationWorkloadEvidence, MutationWorkloadRole, ObjectIndexEvidence, QueryCostEvidence,
    ReportValidationError, TrustedGateIdentity, WorstEditTargetEvidence,
};
use moria_world::{WorldBounds, WorldIdentity, WorldPointQ8};

fn identity() -> WorldIdentity {
    WorldIdentity::new(
        moria_world::config::PRODUCT_ONE_SEED,
        [3; 32],
        WorldBounds::new(
            WorldPointQ8::new(-128_000, -32_768, -128_000),
            WorldPointQ8::new(128_000, 32_768, 128_000),
        )
        .unwrap(),
    )
}

fn report() -> ForestFeasibilityReport {
    let mut report = ForestFeasibilityReport {
        schema: "moria-product-one-forest-feasibility".into(),
        timestamp_utc: "2026-07-17T00:00:00Z".into(),
        passed: true,
        failure_reasons: Vec::new(),
        build: Some(BuildProfile {
            cargo_profile: "release".into(),
            git_commit: "a".repeat(40),
            rustc_version: "rustc 1.89.0".into(),
        }),
        world: identity(),
        manifest_sha256: "b".repeat(64),
        machine: MachineProfile {
            profile_id_sha256: String::new(),
            os_name: "macOS".into(),
            os_version: "15".into(),
            architecture: "aarch64".into(),
            cpu_model: "Apple M4".into(),
            logical_cores: 10,
            total_physical_memory_bytes: 32 * 1024 * 1024 * 1024,
            gpu_adapter_name: "Apple M4".into(),
            gpu_vendor: 0,
            gpu_device: 0,
            gpu_device_class: "integrated".into(),
            wgpu_backend: "metal".into(),
            driver: None,
            driver_metadata_available: false,
            memory_architecture: "unified".into(),
            acceptance_label: "m4-mac-mini-32gb".into(),
        },
        forest_area_m2: 120_000,
        eligible_land_area_m2: 120_000,
        object_counts: BTreeMap::from([
            ("boulder".into(), 288),
            ("bush".into(), 5_400),
            ("rock".into(), 1_080),
            ("ruin".into(), 1),
            ("stump".into(), 168),
            ("tree-a".into(), 2_640),
            ("tree-b".into(), 2_160),
        ]),
        required_object_counts: BTreeMap::from([
            ("boulder".into(), 288),
            ("bush".into(), 5_400),
            ("rock".into(), 1_080),
            ("ruin".into(), 1),
            ("stump".into(), 168),
            ("tree-a".into(), 2_640),
            ("tree-b".into(), 2_160),
        ]),
        species_counts: BTreeMap::from([("birch".into(), 2_640), ("pine".into(), 2_160)]),
        minimum_tree_spacing_q8: 1_280,
        canopy_min_q8: 512,
        canopy_max_q8: 1_024,
        canopy_range_bins: BTreeMap::from([
            ("birch-lower".into(), 16),
            ("birch-upper".into(), 16),
            ("pine-lower".into(), 16),
            ("pine-upper".into(), 16),
        ]),
        minimum_route_clearance_q8: 768,
        overlap_conflicts: 0,
        first_conflict: None,
        object_index: ObjectIndexEvidence {
            validation_ms: 1.0,
            build_ms: 1.0,
            retained_bytes: 1,
            retained_byte_categories: BTreeMap::from([("records".into(), 1)]),
            placement_records: 11_737,
            dependency_grid_entries: 1,
            sample_grid_entries: 1,
            max_dependency_cell_entries: 1,
            max_sample_cell_entries: 1,
            max_horizon_tree_members_per_cell: 1,
            max_edit_candidates: 1,
            max_edit_affected_objects: 1,
            max_dependency_bricks: 1,
            dependency_coordinate_allocation_bytes: 0,
        },
        worst_edit_target: WorstEditTargetEvidence {
            center: WorldPointQ8::new(0, 0, 0),
            broad_candidates: 1,
            exact_dependency_ids: 1,
            dependency_bricks: 1,
            tie_break_rank: 0,
        },
    };
    report.machine.profile_id_sha256 = report.machine.stable_profile_id_sha256();
    report
}

fn distribution() -> Distribution {
    Distribution {
        min: 0.1,
        p50: 0.2,
        p95: 0.3,
        p99: 0.4,
        max: 0.5,
    }
}

fn query_costs() -> QueryCostEvidence {
    let distribution = distribution();
    QueryCostEvidence {
        sample_counts: BTreeMap::from([
            ("cold_inactive_sample_voxel".into(), 256),
            ("normal_query_bundles".into(), 1_000),
            ("sample_column".into(), 128),
            ("diagnostic_metadata_page".into(), 128),
            ("diagnostic_cells_page".into(), 128),
            ("player_sweep".into(), 4_000),
            ("camera_sweep".into(), 1_000),
            ("debug_ray".into(), 1_000),
            ("water_surface".into(), 1_000),
            ("water_contact".into(), 1_000),
            ("active_band".into(), 1_000),
        ]),
        cold_inactive_calls: BTreeMap::from([("sample_voxel".into(), distribution)]),
        frame_critical_calls: BTreeMap::from([
            ("player_sweep".into(), distribution),
            ("camera_sweep".into(), distribution),
            ("debug_ray".into(), distribution),
            ("water_surface".into(), distribution),
            ("water_contact".into(), distribution),
            ("active_band".into(), distribution),
        ]),
        normal_bundle_ms: distribution,
        column_ms: distribution,
        diagnostic_metadata_page_ms: distribution,
        diagnostic_cells_page_ms: distribution,
        observed_work_maxima: BTreeMap::from([
            ("cold_coordinates_distinct".into(), 256),
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

fn workload(role: MutationWorkloadRole) -> MutationWorkloadEvidence {
    const STAGES: [&str; 18] = [
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
    let request_count = match role {
        MutationWorkloadRole::ColonyVolume => 8,
        MutationWorkloadRole::InteractiveCarve | MutationWorkloadRole::CatastrophicCarve => 1,
    };
    MutationWorkloadEvidence {
        role,
        request_count,
        submitted_frame: 1,
        first_committed_frame: 2,
        final_reconciled_frame: 3,
        admission_ms: distribution(),
        first_commit_ms: distribution(),
        primary_ready_ms: distribution(),
        reconciliation_ms: distribution(),
        changed_bricks_per_second: 64.0,
        maximum_runnable_wait_ms: 10.0,
        maximum_frame_ms: 10.0,
        traversable: role == MutationWorkloadRole::InteractiveCarve,
        changed_voxels: 1,
        changed_bricks: 1,
        committed_batches: request_count,
        stage_timings_ms: STAGES
            .into_iter()
            .map(|stage| (stage.into(), 0.1))
            .collect(),
        stage_counts: STAGES
            .into_iter()
            .map(|stage| {
                let count = match stage {
                    "admission" | "schedule" | "commit" | "reconciliation" => request_count,
                    _ => 1,
                };
                (stage.into(), u64::from(count))
            })
            .collect(),
        barrier_expected_items: request_count,
        barrier_renderer_ready_items: request_count,
        horizon_partition_checked: role == MutationWorkloadRole::CatastrophicCarve,
        horizon_excluded_base_cards: u16::from(role == MutationWorkloadRole::CatastrophicCarve),
        horizon_derived_records: u16::from(role == MutationWorkloadRole::CatastrophicCarve),
    }
}

fn mutation_report() -> MutationFeasibilityReport {
    let forest = report();
    let forest_report_sha256 = forest.canonical_sha256().unwrap();
    MutationFeasibilityReport {
        schema: "moria-product-one-mutation-feasibility".into(),
        timestamp_utc: forest.timestamp_utc.clone(),
        passed: true,
        failure_reasons: Vec::new(),
        build: forest
            .build
            .clone()
            .expect("passing forest report has a build"),
        world: forest.world,
        manifest_sha256: forest.manifest_sha256.clone(),
        forest_report_sha256,
        machine: forest.machine.clone(),
        resolution: [2560, 1440],
        backend: "metal".into(),
        cold_start_ms: Some(100.0),
        workloads: [
            MutationWorkloadRole::InteractiveCarve,
            MutationWorkloadRole::ColonyVolume,
            MutationWorkloadRole::CatastrophicCarve,
        ]
        .into_iter()
        .map(workload)
        .collect(),
        query_costs: Some(query_costs()),
    }
}

fn trusted_identity(report: &ForestFeasibilityReport) -> TrustedGateIdentity {
    TrustedGateIdentity {
        git_commit: report
            .build
            .as_ref()
            .expect("passing forest report has a build")
            .git_commit
            .clone(),
        parameters_digest: report.world.parameters_digest,
        manifest_sha256: report.manifest_sha256.clone(),
    }
}

#[test]
fn forest_report_rejects_inconsistent_pass_flags_and_non_finite_metrics() {
    let mut invalid = report();
    invalid.failure_reasons.push("missing-stage".into());
    assert_eq!(
        invalid.validate(),
        Err(ReportValidationError::PassedFlagMismatch)
    );

    let mut invalid = report();
    invalid.object_index.validation_ms = f64::NAN;
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::NonFinite { .. })
    ));

    let mut invalid = report();
    invalid.object_index.build_ms = 250.0;
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Inconsistent {
            field: "object_index timing"
        })
    ));
}

#[test]
fn forest_report_json_is_canonical_and_rejects_wrong_gate_identity() {
    let valid = report();
    let json = valid.to_canonical_json().unwrap();
    assert!(json.starts_with("{\"schema\":\"moria-product-one-forest-feasibility\""));
    assert!(json.contains("\"retained_byte_categories\":{\"records\":1}"));

    let mut invalid = valid;
    invalid.machine.wgpu_backend = "vulkan".into();
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Identity { .. })
    ));
}

#[test]
fn feasibility_reports_reject_wrong_world_identity_and_fabricated_zeroes() {
    let mut invalid = report();
    invalid.world.seed = 0;
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Identity { field: "world" })
    ));

    let mut invalid = report();
    invalid.worst_edit_target.exact_dependency_ids = 0;
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Missing { .. })
    ));

    let mut invalid = mutation_report();
    invalid.cold_start_ms = Some(0.0);
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Missing { .. })
    ));

    let mut invalid = mutation_report();
    invalid.query_costs.as_mut().unwrap().sample_counts.clear();
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Missing { .. })
    ));
}

#[test]
fn passing_feasibility_reports_enforce_forest_and_query_probe_contracts() {
    let mut invalid = report();
    invalid.forest_area_m2 = 1;
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Missing {
            field: "forest measurement"
        })
    ));

    let mut invalid = mutation_report();
    invalid
        .query_costs
        .as_mut()
        .unwrap()
        .sample_counts
        .remove("normal_query_bundles");
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Missing {
            field: "query evidence"
        })
    ));

    let mut invalid = mutation_report();
    invalid
        .query_costs
        .as_mut()
        .unwrap()
        .sample_counts
        .insert("normal_query_bundles".into(), 999);
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Missing {
            field: "query evidence"
        })
    ));
}

#[test]
fn passing_forest_report_requires_complete_density_species_canopy_and_placement_evidence() {
    let mut invalid = report();
    invalid.object_counts.insert("tree-a".into(), 1);
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Inconsistent {
            field: "forest object counts"
        })
    ));

    let mut invalid = report();
    invalid.species_counts.insert("birch".into(), 1);
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Inconsistent {
            field: "forest species counts"
        })
    ));

    let mut invalid = report();
    invalid.canopy_range_bins.insert("pine-upper".into(), 15);
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Inconsistent {
            field: "canopy range bins"
        })
    ));

    let mut invalid = report();
    invalid.object_index.placement_records -= 1;
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Inconsistent {
            field: "placement records"
        })
    ));
}

#[test]
fn passing_forest_report_derives_density_minima_from_reported_areas() {
    let mut invalid = report();
    invalid.forest_area_m2 = 240_000;
    invalid.eligible_land_area_m2 = 240_000;
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Inconsistent {
            field: "forest object counts"
        })
    ));

    let mut valid = invalid;
    valid.object_counts = BTreeMap::from([
        ("boulder".into(), 576),
        ("bush".into(), 10_800),
        ("rock".into(), 2_160),
        ("ruin".into(), 1),
        ("stump".into(), 336),
        ("tree-a".into(), 5_280),
        ("tree-b".into(), 4_320),
    ]);
    valid.required_object_counts = valid.object_counts.clone();
    valid.species_counts = BTreeMap::from([("birch".into(), 5_280), ("pine".into(), 4_320)]);
    valid.object_index.placement_records = valid.object_counts.values().sum();
    assert!(valid.validate().is_ok());
}

#[test]
fn passing_forest_report_requires_the_rounded_total_tree_minimum() {
    let mut valid = report();
    valid.forest_area_m2 = 120_001;
    valid.eligible_land_area_m2 = 120_001;
    valid.object_counts = BTreeMap::from([
        ("boulder".into(), 289),
        ("bush".into(), 5_401),
        ("rock".into(), 1_081),
        ("ruin".into(), 1),
        ("stump".into(), 169),
        ("tree-a".into(), 2_641),
        ("tree-b".into(), 2_160),
    ]);
    valid.required_object_counts = BTreeMap::from([
        ("boulder".into(), 289),
        ("bush".into(), 5_401),
        ("rock".into(), 1_081),
        ("ruin".into(), 1),
        ("stump".into(), 169),
        ("tree-a".into(), 2_640),
        ("tree-b".into(), 2_160),
    ]);
    valid.species_counts = BTreeMap::from([("birch".into(), 2_641), ("pine".into(), 2_160)]);
    valid.object_index.placement_records = valid.object_counts.values().sum();
    assert!(valid.validate().is_ok());

    valid.object_counts.insert("tree-a".into(), 2_640);
    valid.species_counts.insert("birch".into(), 2_640);
    valid.object_index.placement_records -= 1;
    assert!(matches!(
        valid.validate(),
        Err(ReportValidationError::Inconsistent {
            field: "forest tree count"
        })
    ));
}

#[test]
fn passing_forest_report_requires_worst_target_to_match_index_candidate_maximum() {
    let mut invalid = report();
    invalid.object_index.max_edit_candidates = 256;
    invalid.worst_edit_target.broad_candidates = 1;
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Inconsistent {
            field: "worst edit target"
        })
    ));
}

#[test]
fn passing_forest_report_requires_a_consistent_maximal_stress_target() {
    let mut invalid = report();
    invalid.object_index.max_edit_candidates = 2;
    invalid.worst_edit_target.broad_candidates = 2;
    invalid.worst_edit_target.exact_dependency_ids = 2;
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Inconsistent {
            field: "worst edit target"
        })
    ));

    let mut valid = report();
    valid.object_index.max_edit_candidates = 2;
    valid.object_index.max_edit_affected_objects = 2;
    valid.worst_edit_target.broad_candidates = 2;
    assert!(valid.validate().is_ok());

    let mut valid = report();
    valid.object_index.max_dependency_bricks = 2;
    assert!(valid.validate().is_ok());

    let mut invalid = report();
    invalid.worst_edit_target.tie_break_rank = 1;
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Inconsistent {
            field: "worst edit target"
        })
    ));
}

#[test]
fn passing_forest_report_requires_affected_object_maximum_to_fit_candidates() {
    let mut invalid = report();
    invalid.object_index.max_edit_affected_objects = 2;
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Inconsistent {
            field: "object index edit maxima"
        })
    ));
}

#[test]
fn passing_colony_workload_requires_a_committed_batch_for_each_request() {
    let mut invalid = mutation_report();
    invalid.workloads[1].committed_batches = 1;
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Inconsistent {
            field: "workload trace"
        })
    ));
}

#[test]
fn passing_colony_workload_requires_a_trace_for_every_request() {
    let mut invalid = mutation_report();
    for stage in ["admission", "schedule", "commit", "reconciliation"] {
        invalid.workloads[1].stage_counts.insert(stage.into(), 1);
    }
    invalid.workloads[1].barrier_expected_items = 1;
    invalid.workloads[1].barrier_renderer_ready_items = 1;
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Inconsistent {
            field: "workload trace"
        })
    ));
}

#[test]
fn passing_workload_requires_exactly_one_request_stage_record_per_request() {
    let mut invalid = mutation_report();
    invalid.workloads[1]
        .stage_counts
        .insert("admission".into(), 9);
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Inconsistent {
            field: "workload trace"
        })
    ));
}

#[test]
fn passing_workload_requires_commit_records_to_match_committed_batches() {
    let mut invalid = mutation_report();
    invalid.workloads[1].stage_counts.insert("commit".into(), 9);
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Inconsistent {
            field: "workload trace"
        })
    ));
}

#[test]
fn failed_workload_allows_a_renderer_barrier_shortfall_only_when_reported() {
    let mut failed = mutation_report();
    failed.passed = false;
    failed.failure_reasons = vec!["workload reconciliation".into()];
    failed.workloads[1].barrier_renderer_ready_items -= 1;
    assert!(failed.to_canonical_json().is_ok());

    failed.failure_reasons.clear();
    assert_eq!(
        failed.validate(),
        Err(ReportValidationError::PassedFlagMismatch)
    );

    failed.failure_reasons = vec!["workload measurement".into()];
    assert!(matches!(
        failed.validate(),
        Err(ReportValidationError::Inconsistent {
            field: "workload reconciliation"
        })
    ));
}

#[test]
fn machine_profile_id_binds_every_identity_field() {
    let baseline = report().machine;
    assert!(baseline.validate_complete().is_ok());

    let mutations: &[fn(&mut MachineProfile)] = &[
        |value| value.os_name.push('x'),
        |value| value.os_version.push('x'),
        |value| value.architecture.push('x'),
        |value| value.cpu_model.push('x'),
        |value| value.logical_cores += 1,
        |value| value.total_physical_memory_bytes += 1,
        |value| value.gpu_adapter_name.push('x'),
        |value| value.gpu_vendor += 1,
        |value| value.gpu_device += 1,
        |value| value.gpu_device_class = "discrete".into(),
        |value| value.wgpu_backend = "vulkan".into(),
        |value| value.driver = Some("driver".into()),
        |value| value.driver_metadata_available = true,
        |value| value.memory_architecture = "discrete".into(),
        |value| value.acceptance_label.push('x'),
    ];
    for mutate in mutations {
        let mut invalid = baseline.clone();
        mutate(&mut invalid);
        let updated_profile_id = invalid.stable_profile_id_sha256();
        assert_ne!(updated_profile_id, baseline.profile_id_sha256);
        assert_eq!(
            invalid.validate_complete(),
            Err(ReportValidationError::Identity { field: "machine" })
        );
    }
}

#[test]
fn cold_query_samples_allow_any_p99_within_the_four_millisecond_maximum() {
    let mut valid = mutation_report();
    let cold = valid
        .query_costs
        .as_mut()
        .unwrap()
        .cold_inactive_calls
        .get_mut("sample_voxel")
        .unwrap();
    cold.p99 = 3.0;
    cold.max = 4.0;
    assert!(valid.validate().is_ok());
}

#[test]
fn mutation_report_requires_the_exact_forest_gate_identity() {
    let forest = report();
    let mutation = mutation_report();
    let trusted = trusted_identity(&forest);
    assert!(mutation.validate_against_forest(&forest, &trusted).is_ok());

    let mut wrong_world = forest.clone();
    wrong_world.world.seed += 1;
    assert!(matches!(
        mutation.validate_against_forest(&wrong_world, &trusted),
        Err(ReportValidationError::Identity { .. })
    ));

    let mut wrong_manifest = forest;
    wrong_manifest.manifest_sha256 = "e".repeat(64);
    assert!(matches!(
        mutation.validate_against_forest(&wrong_manifest, &trusted),
        Err(ReportValidationError::Identity { .. })
    ));
}

#[test]
fn mutation_report_rejects_a_mutually_consistent_stale_gate_identity() {
    let trusted = trusted_identity(&report());
    let mut forest = report();
    forest.build.as_mut().unwrap().git_commit = "d".repeat(40);
    forest.world.parameters_digest = [4; 32];
    forest.manifest_sha256 = "e".repeat(64);

    let mut mutation = mutation_report();
    mutation.build = forest.build.clone().expect("forest build was updated");
    mutation.world = forest.world;
    mutation.manifest_sha256 = forest.manifest_sha256.clone();
    mutation.forest_report_sha256 = forest.canonical_sha256().unwrap();

    assert!(matches!(
        mutation.validate_against_forest(&forest, &trusted),
        Err(ReportValidationError::Identity {
            field: "trusted gate identity"
        })
    ));
}

#[test]
fn failed_environment_reports_serialize_only_with_matching_failure_reasons() {
    let mut forest = report();
    forest.passed = false;
    forest.failure_reasons = vec!["M4 machine".into(), "build".into()];
    forest.build.as_mut().unwrap().cargo_profile = "dev".into();
    forest.machine.os_name = "Linux".into();
    forest.machine.wgpu_backend = "vulkan".into();
    forest.machine.profile_id_sha256 = forest.machine.stable_profile_id_sha256();
    assert!(forest.to_canonical_json().is_ok());

    forest.failure_reasons = vec!["build".into()];
    assert_eq!(
        forest.validate(),
        Err(ReportValidationError::FailureReasons)
    );

    let mut mutation = mutation_report();
    mutation.passed = false;
    mutation.failure_reasons = vec!["mutation display".into()];
    mutation.resolution = [1920, 1080];
    mutation.backend = "vulkan".into();
    assert!(mutation.to_canonical_json().is_ok());

    mutation.failure_reasons = vec!["runtime".into()];
    assert_eq!(
        mutation.validate(),
        Err(ReportValidationError::FailureReasons)
    );
}

#[test]
fn feasibility_json_parsers_reject_missing_fields_and_noncanonical_vectors() {
    let forest_json = report().to_canonical_json().unwrap();
    let missing_machine = forest_json.replace(",\"machine\":", ",\"removed_machine\":");
    assert!(ForestFeasibilityReport::from_json(&missing_machine).is_err());

    let mut missing_nullable: serde_json::Value = serde_json::from_str(&forest_json).unwrap();
    missing_nullable
        .as_object_mut()
        .unwrap()
        .remove("first_conflict");
    assert!(
        ForestFeasibilityReport::from_json(&serde_json::to_string(&missing_nullable).unwrap())
            .is_err()
    );

    let mut missing_driver: serde_json::Value = serde_json::from_str(&forest_json).unwrap();
    missing_driver["machine"]
        .as_object_mut()
        .unwrap()
        .remove("driver");
    assert!(
        ForestFeasibilityReport::from_json(&serde_json::to_string(&missing_driver).unwrap())
            .is_err()
    );

    let mut mutation = mutation_report();
    mutation.failure_reasons = vec!["z-last".into(), "a-first".into()];
    mutation.passed = false;
    assert_eq!(
        mutation.validate(),
        Err(ReportValidationError::FailureReasons)
    );

    let mut invalid = report();
    invalid.passed = false;
    invalid.failure_reasons = vec![" ".into()];
    assert_eq!(
        invalid.validate(),
        Err(ReportValidationError::FailureReasons)
    );
}

#[test]
fn feasibility_reports_reject_zero_filled_present_collections() {
    let mut invalid = report();
    invalid.species_counts.insert("birch".into(), 0);
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Missing { .. })
    ));

    let mut invalid = mutation_report();
    invalid.workloads[0].stage_counts.insert("commit".into(), 0);
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Missing { .. })
    ));

    let mut invalid = mutation_report();
    invalid
        .query_costs
        .as_mut()
        .unwrap()
        .observed_work_maxima
        .insert("voxel-candidates".into(), 0);
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Missing { .. })
    ));
}

#[test]
fn feasibility_failure_reports_serialize_measured_gate_violations() {
    let mut forest = report();
    forest.passed = false;
    forest.failure_reasons = vec!["forest area".into()];
    forest.forest_area_m2 = 1;
    let forest_json = forest.to_canonical_json().unwrap();
    assert!(forest_json.contains("\"passed\":false"));
    assert!(ForestFeasibilityReport::from_json(&forest_json).is_ok());

    let mut mutation = mutation_report();
    mutation.passed = false;
    mutation.failure_reasons = vec!["cold_start_ms".into()];
    mutation.cold_start_ms = Some(5_000.0);
    let mutation_json = mutation.to_canonical_json().unwrap();
    assert!(mutation_json.contains("\"passed\":false"));
    assert!(MutationFeasibilityReport::from_json(&mutation_json).is_ok());

    let mut workload_failure = mutation_report();
    workload_failure.passed = false;
    workload_failure.failure_reasons = vec!["workload acceptance".into()];
    workload_failure.workloads[0].maximum_frame_ms = 33.4;
    let workload_json = workload_failure.to_canonical_json().unwrap();
    assert!(workload_json.contains("\"maximum_frame_ms\":33.4"));
    assert!(MutationFeasibilityReport::from_json(&workload_json).is_ok());

    let mut traversal_failure = mutation_report();
    traversal_failure.passed = false;
    traversal_failure.failure_reasons = vec!["interactive traversal".into()];
    traversal_failure.workloads[0].traversable = false;
    let traversal_json = traversal_failure.to_canonical_json().unwrap();
    assert!(MutationFeasibilityReport::from_json(&traversal_json).is_ok());

    let mut horizon_failure = mutation_report();
    horizon_failure.passed = false;
    horizon_failure.failure_reasons = vec!["catastrophic Horizon evidence".into()];
    let catastrophic = &mut horizon_failure.workloads[2];
    catastrophic.horizon_partition_checked = false;
    catastrophic.horizon_excluded_base_cards = 0;
    catastrophic.horizon_derived_records = 0;
    let horizon_json = horizon_failure.to_canonical_json().unwrap();
    assert!(MutationFeasibilityReport::from_json(&horizon_json).is_ok());
}

#[test]
fn forest_failure_before_build_capture_serializes_null_build() {
    let mut failed = report();
    failed.passed = false;
    failed.failure_reasons = vec!["build".into()];
    failed.build = None;

    let json = failed.to_canonical_json().unwrap();
    assert!(json.contains("\"build\":null"));
    assert!(ForestFeasibilityReport::from_json(&json).is_ok());
}

#[test]
fn mutation_input_failure_preserves_unavailable_runtime_evidence() {
    let mut failed = mutation_report();
    failed.passed = false;
    failed.failure_reasons = vec![
        "cold_start_ms".into(),
        "forest feasibility gate".into(),
        "query costs".into(),
        "workloads".into(),
    ];
    failed.cold_start_ms = None;
    failed.workloads.clear();
    failed.query_costs = None;

    let json = failed.to_canonical_json().unwrap();
    assert!(json.contains("\"cold_start_ms\":null"));
    assert!(json.contains("\"workloads\":[]"));
    assert!(json.contains("\"query_costs\":null"));
    assert!(MutationFeasibilityReport::from_json(&json).is_ok());
}

#[test]
fn failed_mutation_report_preserves_partial_missing_stage_trace() {
    let mut failed = mutation_report();
    failed.passed = false;
    failed.failure_reasons = vec!["mutation stage".into()];
    failed.workloads[0].stage_timings_ms.remove("gpu-upload");
    failed.workloads[0].stage_counts.remove("gpu-upload");

    let json = failed.to_canonical_json().unwrap();
    let parsed = MutationFeasibilityReport::from_json(&json).unwrap();
    assert!(
        !parsed.workloads[0]
            .stage_timings_ms
            .contains_key("gpu-upload")
    );
    assert!(!parsed.workloads[0].stage_counts.contains_key("gpu-upload"));
}

#[test]
fn failed_mutation_reports_require_reasons_for_unavailable_workload_evidence() {
    let mut failed = mutation_report();
    failed.passed = false;
    failed.failure_reasons = vec!["mutation stage".into(), "workload measurement".into()];
    for workload in &mut failed.workloads {
        let unavailable = Distribution {
            min: 0.0,
            p50: 0.0,
            p95: 0.0,
            p99: 0.0,
            max: 0.0,
        };
        workload.admission_ms = unavailable;
        workload.first_commit_ms = unavailable;
        workload.primary_ready_ms = unavailable;
        workload.reconciliation_ms = unavailable;
        workload.stage_timings_ms.clear();
        workload.stage_counts.clear();
        workload.changed_bricks_per_second = 0.0;
        workload.maximum_runnable_wait_ms = 0.0;
        workload.maximum_frame_ms = 0.0;
        workload.changed_voxels = 0;
        workload.changed_bricks = 0;
        workload.committed_batches = 0;
        workload.barrier_expected_items = 0;
        workload.barrier_renderer_ready_items = 0;
    }
    assert!(matches!(
        failed.validate(),
        Err(ReportValidationError::Missing {
            field: "admission_ms"
        })
    ));

    failed.failure_reasons = vec![
        "mutation stage".into(),
        "workload distributions".into(),
        "workload measurement".into(),
    ];
    let json = failed.to_canonical_json().unwrap();
    assert!(MutationFeasibilityReport::from_json(&json).is_ok());

    let mut zero_stage_counts = mutation_report();
    zero_stage_counts.passed = false;
    zero_stage_counts.failure_reasons = vec!["mutation stage count".into()];
    zero_stage_counts.workloads[0]
        .stage_counts
        .insert("commit".into(), 0);
    assert!(zero_stage_counts.to_canonical_json().is_ok());
}

#[test]
fn mutation_workload_rejects_more_changed_bricks_than_voxels() {
    let mut invalid = mutation_report();
    invalid.workloads[0].changed_voxels = 1;
    invalid.workloads[0].changed_bricks = 2;

    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Inconsistent {
            field: "workload measurement"
        })
    ));
}

#[test]
fn passing_mutation_reports_require_stage_evidence() {
    for stage in ["seams", "bevy-install"] {
        let mut invalid = mutation_report();
        for workload in &mut invalid.workloads {
            workload.stage_counts.insert(stage.into(), 0);
        }
        assert!(matches!(
            invalid.validate(),
            Err(ReportValidationError::Missing {
                field: "aggregate mutation stage count"
            })
        ));
    }

    for stage in ["object-mesh", "dressing-remove", "dressing-install"] {
        let mut invalid = mutation_report();
        invalid.workloads[2].stage_counts.insert(stage.into(), 0);
        assert!(matches!(
            invalid.validate(),
            Err(ReportValidationError::Missing {
                field: "catastrophic mutation stage count"
            })
        ));
    }
}
