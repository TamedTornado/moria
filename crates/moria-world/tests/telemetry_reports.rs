use std::collections::BTreeMap;

use moria_world::telemetry::{
    BuildProfile, Distribution, ForestFeasibilityReport, MachineProfile, MutationFeasibilityReport,
    MutationWorkloadEvidence, MutationWorkloadRole, ObjectIndexEvidence, QueryCostEvidence,
    ReportValidationError, WorstEditTargetEvidence,
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
    ForestFeasibilityReport {
        schema: "moria-product-one-forest-feasibility".into(),
        timestamp_utc: "2026-07-17T00:00:00Z".into(),
        passed: true,
        failure_reasons: Vec::new(),
        build: BuildProfile {
            cargo_profile: "release".into(),
            git_commit: "a".repeat(40),
            rustc_version: "rustc 1.89.0".into(),
        },
        world: identity(),
        manifest_sha256: "b".repeat(64),
        machine: MachineProfile {
            profile_id_sha256: "c".repeat(64),
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
        object_counts: BTreeMap::from([("tree-a".into(), 1)]),
        required_object_counts: BTreeMap::from([("tree-a".into(), 1)]),
        species_counts: BTreeMap::from([("birch".into(), 1)]),
        minimum_tree_spacing_q8: 1_280,
        canopy_min_q8: 512,
        canopy_max_q8: 1_024,
        canopy_range_bins: BTreeMap::from([("2m".into(), 1)]),
        minimum_route_clearance_q8: 768,
        overlap_conflicts: 0,
        first_conflict: None,
        object_index: ObjectIndexEvidence {
            validation_ms: 1.0,
            build_ms: 1.0,
            retained_bytes: 1,
            retained_byte_categories: BTreeMap::from([("records".into(), 1)]),
            placement_records: 1,
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
    }
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
        committed_batches: 1,
        stage_timings_ms: STAGES
            .into_iter()
            .map(|stage| (stage.into(), 0.1))
            .collect(),
        stage_counts: STAGES.into_iter().map(|stage| (stage.into(), 1)).collect(),
        barrier_expected_items: 1,
        barrier_renderer_ready_items: 1,
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
        build: forest.build.clone(),
        world: forest.world,
        manifest_sha256: forest.manifest_sha256.clone(),
        forest_report_sha256,
        machine: forest.machine.clone(),
        resolution: [2560, 1440],
        backend: "metal".into(),
        cold_start_ms: 100.0,
        workloads: [
            MutationWorkloadRole::InteractiveCarve,
            MutationWorkloadRole::ColonyVolume,
            MutationWorkloadRole::CatastrophicCarve,
        ]
        .into_iter()
        .map(workload)
        .collect(),
        query_costs: QueryCostEvidence {
            sample_counts: BTreeMap::from([("cold-inactive".into(), 256)]),
            cold_inactive_calls: BTreeMap::from([("voxel".into(), distribution())]),
            frame_critical_calls: BTreeMap::from([("capsule-sweep".into(), distribution())]),
            normal_bundle_ms: distribution(),
            column_ms: distribution(),
            diagnostic_metadata_page_ms: distribution(),
            diagnostic_cells_page_ms: distribution(),
            observed_work_maxima: BTreeMap::from([("voxel-candidates".into(), 1)]),
        },
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
    invalid.cold_start_ms = 0.0;
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Missing { .. })
    ));

    let mut invalid = mutation_report();
    invalid.query_costs.sample_counts.clear();
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Missing { .. })
    ));
}

#[test]
fn mutation_report_requires_the_exact_forest_gate_identity() {
    let forest = report();
    let mutation = mutation_report();
    assert!(mutation.validate_against_forest(&forest).is_ok());

    let mut wrong_world = forest.clone();
    wrong_world.world.seed += 1;
    assert!(matches!(
        mutation.validate_against_forest(&wrong_world),
        Err(ReportValidationError::Identity { .. })
    ));

    let mut wrong_manifest = forest;
    wrong_manifest.manifest_sha256 = "e".repeat(64);
    assert!(matches!(
        mutation.validate_against_forest(&wrong_manifest),
        Err(ReportValidationError::Identity { .. })
    ));
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
    forest.failure_reasons = vec!["forest clearance".into()];
    forest.minimum_tree_spacing_q8 = 1;
    let forest_json = forest.to_canonical_json().unwrap();
    assert!(forest_json.contains("\"passed\":false"));
    assert!(ForestFeasibilityReport::from_json(&forest_json).is_ok());

    let mut mutation = mutation_report();
    mutation.passed = false;
    mutation.failure_reasons = vec!["cold_start_ms".into()];
    mutation.cold_start_ms = 5_000.0;
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
