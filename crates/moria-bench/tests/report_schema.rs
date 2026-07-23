use std::collections::BTreeMap;

use moria_bench::capture::schema::{
    ActiveBand, ActiveCounts, AssetEvidence, BaselineStatus, BenchmarkReport, CoverageEvidence,
    FrameRateMetrics, GraphicsMemoryEstimate, GraphicsMemoryEvidence, MutationLatencyMetrics,
    QueueDepths, ResidentGraphicsMeasurement, RoundTripEvidence, SaveEvidence, ScenarioName,
    StreamingEvidence,
};
use moria_world::telemetry::{
    BuildProfile, Distribution, MachineProfile, ObjectIndexEvidence, ReportValidationError,
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

fn distribution() -> Distribution {
    Distribution {
        min: 10.0,
        p50: 12.0,
        p95: 15.0,
        p99: 16.0,
        max: 16.5,
    }
}

fn machine() -> MachineProfile {
    MachineProfile {
        profile_id_sha256: "c".repeat(64),
        os_name: "macOS".into(),
        os_version: "15".into(),
        architecture: "aarch64".into(),
        cpu_model: "Apple M4".into(),
        logical_cores: 10,
        total_physical_memory_bytes: 32 * 1024 * 1024 * 1024,
        gpu_adapter_name: "Apple M4".into(),
        gpu_vendor: 1,
        gpu_device: 1,
        gpu_device_class: "integrated".into(),
        wgpu_backend: "metal".into(),
        driver: None,
        driver_metadata_available: false,
        memory_architecture: "unified".into(),
        acceptance_label: "m4-mac-mini-32gb".into(),
    }
}

fn object_index() -> ObjectIndexEvidence {
    ObjectIndexEvidence {
        validation_ms: 10.0,
        build_ms: 5.0,
        retained_bytes: 1024,
        retained_byte_categories: BTreeMap::from([("records".into(), 1024)]),
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
    }
}

fn report() -> BenchmarkReport {
    BenchmarkReport {
        schema: "moria-product-one-benchmark".into(),
        timestamp_utc: "2026-07-17T00:00:00Z".into(),
        scenario: ScenarioName::Flythrough,
        passed: true,
        failure_reasons: Vec::new(),
        baseline_status: BaselineStatus::Provisional,
        build: Some(BuildProfile {
            cargo_profile: "release".into(),
            git_commit: "a".repeat(40),
            rustc_version: "rustc 1.89.0".into(),
        }),
        world: Some(identity()),
        assets: Some(AssetEvidence {
            manifest_sha256: "b".repeat(64),
            fallbacks: Vec::new(),
            warnings: Vec::new(),
        }),
        machine: Some(machine()),
        resolution: Some([2560, 1440]),
        cold_start_ms: Some(100.0),
        frame_rate: Some(FrameRateMetrics {
            sample_count: 60,
            measured_seconds: 1.0,
            arithmetic_fps: 60.0,
            one_percent_low_fps: 60.0,
        }),
        frame_time_ms: Some(distribution()),
        graphics_memory: Some(GraphicsMemoryEvidence {
            application_ledger: GraphicsMemoryEstimate {
                peak_bytes: 1024,
                end_bytes: 512,
                categories: BTreeMap::from([("buffers".into(), 1024)]),
                untracked_driver_overhead: true,
            },
            resident_measurement: None,
            product_target_proven: false,
            estimate_substitution_approval_id: Some("PRODUCT-42".into()),
        }),
        mutation_latency: None,
        save: SaveEvidence {
            attempted: false,
            completed: false,
            size_bytes: Some(0),
            changed_voxels: Some(0),
            changed_bricks: Some(0),
            round_trip: None,
        },
        coverage: Some(CoverageEvidence {
            route_tags_visited: complete_route_tags(),
            active_bands_entered: vec![
                ActiveBand::Far,
                ActiveBand::Horizon,
                ActiveBand::Middle,
                ActiveBand::Near,
            ],
            edited_material_counts: BTreeMap::new(),
            final_changed_spheres: 0,
            final_changed_region_cells: 0,
            workload_minimum_met: true,
        }),
        streaming: Some(StreamingEvidence {
            peak_active_counts: ActiveCounts {
                bricks: 1,
                meshes: 1,
                objects: 1,
            },
            peak_queue_depths: QueueDepths {
                extraction: 1,
                installation: 1,
                render: 1,
            },
            first_steady_derived_bytes: 100,
            return_steady_derived_bytes: 100,
            monotonic_growth_check_passed: true,
            object_index: object_index(),
        }),
    }
}

const ALL_BANDS: [ActiveBand; 4] = [
    ActiveBand::Far,
    ActiveBand::Horizon,
    ActiveBand::Middle,
    ActiveBand::Near,
];

fn complete_route_tags() -> Vec<String> {
    [
        "aquifer",
        "cave-floor",
        "cave-mouth",
        "cliff-top",
        "forest",
        "lake",
        "meadow",
        "ore-vein",
        "river",
        "rock-shelves",
        "ruin-stair-bottom",
        "ruin-stair-top",
        "signature-carve-hillside",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}

fn mutation_report() -> BenchmarkReport {
    let mut report = report();
    let latency = Distribution {
        min: 0.1,
        p50: 0.2,
        p95: 0.3,
        p99: 0.4,
        max: 0.5,
    };
    report.scenario = ScenarioName::CarveStorm;
    report.mutation_latency = Some(MutationLatencyMetrics {
        sample_count: 3,
        admission_ms: latency,
        accepted_to_first_commit_ms: latency,
        commit_to_primary_ready_ms: latency,
        accepted_to_reconciliation_ms: latency,
        changed_bricks_per_second: 64.0,
        maximum_runnable_wait_ms: 10.0,
        representative_max_frame_ms: 10.0,
    });
    report.save = SaveEvidence {
        attempted: true,
        completed: true,
        size_bytes: Some(1024),
        changed_voxels: Some(512),
        changed_bricks: Some(256),
        round_trip: Some(RoundTripEvidence {
            passed: true,
            delta_voxels_compared: 512,
            base_samples_compared: 128,
            identity_match: true,
            derived_bytes_found: false,
        }),
    };
    report.coverage = Some(CoverageEvidence {
        route_tags_visited: complete_route_tags(),
        active_bands_entered: ALL_BANDS.to_vec(),
        edited_material_counts: BTreeMap::from([("granite".into(), 512)]),
        final_changed_spheres: 3,
        final_changed_region_cells: 256,
        workload_minimum_met: true,
    });
    report
}

#[test]
fn completed_report_rejects_fabricated_zero_measurements() {
    assert!(report().validate().is_ok());

    let mut invalid = report();
    invalid.cold_start_ms = Some(0.0);
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Missing { .. })
    ));

    let mut invalid = report();
    invalid
        .graphics_memory
        .as_mut()
        .unwrap()
        .application_ledger
        .peak_bytes = 0;
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Missing { .. })
    ));

    let mut invalid = report();
    invalid
        .streaming
        .as_mut()
        .unwrap()
        .object_index
        .placement_records = 0;
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Missing { .. })
    ));
}

#[test]
fn application_ledger_cannot_stand_in_for_resident_evidence() {
    let mut invalid = report();
    let graphics = invalid.graphics_memory.as_mut().unwrap();
    graphics.estimate_substitution_approval_id = None;
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Inconsistent { .. })
    ));

    let mut invalid = report();
    let graphics = invalid.graphics_memory.as_mut().unwrap();
    graphics.product_target_proven = true;
    graphics.estimate_substitution_approval_id = None;
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Inconsistent { .. })
    ));

    let mut invalid = report();
    let graphics = invalid.graphics_memory.as_mut().unwrap();
    graphics.resident_measurement = Some(ResidentGraphicsMeasurement {
        provider: "application allocation ledger".into(),
        scope: "game process resident graphics allocations".into(),
        sampling_interval_ms: 1,
        peak_bytes: 1024,
        artifact_sha256: "d".repeat(64),
        artifact_path: "ledger.json".into(),
    });
    graphics.product_target_proven = true;
    graphics.estimate_substitution_approval_id = None;
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Identity {
            field: "resident_measurement"
        })
    ));

    let mut invalid = report();
    let graphics = invalid.graphics_memory.as_mut().unwrap();
    graphics.resident_measurement = Some(ResidentGraphicsMeasurement {
        provider: "reviewed harness".into(),
        scope: "application ledger for game process resident graphics allocations".into(),
        sampling_interval_ms: 1,
        peak_bytes: 1024,
        artifact_sha256: "d".repeat(64),
        artifact_path: "resident.json".into(),
    });
    graphics.product_target_proven = true;
    graphics.estimate_substitution_approval_id = None;
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Identity {
            field: "resident_measurement"
        })
    ));
}

#[test]
fn completed_report_rejects_wrong_product_and_machine_identities() {
    let mut invalid = report();
    invalid.build.as_mut().unwrap().cargo_profile = "dev".into();
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Identity { field: "build" })
    ));

    let mut invalid = report();
    invalid.world.as_mut().unwrap().seed = 0;
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Identity { field: "world" })
    ));

    let mut invalid = report();
    invalid.machine.as_mut().unwrap().driver = Some(String::new());
    invalid.machine.as_mut().unwrap().driver_metadata_available = true;
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Identity { field: "machine" })
    ));
}

#[test]
fn failed_report_validates_a_present_machine_without_a_resolution() {
    let mut invalid = report();
    invalid.passed = false;
    invalid.failure_reasons = vec!["resolution".into()];
    invalid.resolution = None;
    invalid.machine.as_mut().unwrap().driver = Some(String::new());
    invalid.machine.as_mut().unwrap().driver_metadata_available = true;
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Identity { field: "machine" })
    ));
}

#[test]
fn canonical_json_rejects_unknown_enum_literals_and_unsorted_vectors() {
    let json = report().to_canonical_json().unwrap();
    assert!(json.starts_with("{\"schema\":\"moria-product-one-benchmark\""));
    assert!(json.contains("\"categories\":{\"buffers\":1024}"));

    let invalid_band = json.replace("\"near\"", "\"nearest\"");
    assert!(matches!(
        BenchmarkReport::from_json(&invalid_band),
        Err(ReportValidationError::Serialization)
    ));

    let mut missing_key: serde_json::Value = serde_json::from_str(&json).unwrap();
    missing_key.as_object_mut().unwrap().remove("machine");
    assert!(matches!(
        BenchmarkReport::from_json(&serde_json::to_string(&missing_key).unwrap()),
        Err(ReportValidationError::Missing {
            field: "top-level key"
        })
    ));

    for path in [
        ["save", "round_trip"],
        ["graphics_memory", "resident_measurement"],
        ["graphics_memory", "estimate_substitution_approval_id"],
        ["machine", "driver"],
    ] {
        let mut missing_nested: serde_json::Value = serde_json::from_str(&json).unwrap();
        missing_nested[path[0]]
            .as_object_mut()
            .unwrap()
            .remove(path[1]);
        assert!(
            BenchmarkReport::from_json(&serde_json::to_string(&missing_nested).unwrap()).is_err()
        );
    }

    let mut invalid = report();
    invalid.assets.as_mut().unwrap().warnings = vec!["z".into(), "a".into()];
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Identity { field: "assets" })
    ));

    let mut invalid = report();
    invalid.coverage.as_mut().unwrap().route_tags_visited = vec![" ".into()];
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Inconsistent { field: "coverage" })
    ));
}

#[test]
fn passing_flythrough_requires_every_product_one_route_tag() {
    let mut invalid = report();
    invalid.coverage.as_mut().unwrap().route_tags_visited = vec!["cave".into(), "forest".into()];
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Inconsistent {
            field: "coverage pass"
        })
    ));
}

#[test]
fn scenario_complete_and_null_rules_reject_impossible_evidence_states() {
    assert!(mutation_report().validate().is_ok());

    let mut invalid = mutation_report();
    invalid.save.completed = false;
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Inconsistent {
            field: "mutation save state"
        })
    ));

    let mut invalid = report();
    invalid.passed = false;
    invalid.failure_reasons = vec!["coverage".into()];
    invalid.coverage = Some(CoverageEvidence {
        route_tags_visited: vec!["failure-recorded".into()],
        active_bands_entered: Vec::new(),
        edited_material_counts: BTreeMap::new(),
        final_changed_spheres: 0,
        final_changed_region_cells: 0,
        workload_minimum_met: false,
    });
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Missing {
            field: "coverage.active_bands_entered"
        })
    ));

    let mut early_failure = report();
    early_failure.passed = false;
    early_failure.failure_reasons = vec!["runtime-failure".into()];
    early_failure.save = SaveEvidence {
        attempted: false,
        completed: false,
        size_bytes: None,
        changed_voxels: None,
        changed_bricks: None,
        round_trip: None,
    };
    let save_count_reasons = [
        "runtime-failure".into(),
        "save.changed_bricks".into(),
        "save.changed_voxels".into(),
        "save.size_bytes".into(),
    ];
    for missing_reason in &save_count_reasons[1..] {
        early_failure.failure_reasons = save_count_reasons
            .iter()
            .filter(|reason| *reason != missing_reason)
            .cloned()
            .collect();
        assert!(matches!(
            early_failure.validate(),
            Err(ReportValidationError::Missing { field }) if field == missing_reason
        ));
    }

    early_failure.failure_reasons = save_count_reasons.to_vec();
    assert!(early_failure.validate().is_ok());

    let mut mutation_failure = mutation_report();
    mutation_failure.passed = false;
    mutation_failure.failure_reasons = vec!["runtime-failure".into()];
    mutation_failure.save = SaveEvidence {
        attempted: false,
        completed: false,
        size_bytes: None,
        changed_voxels: None,
        changed_bricks: None,
        round_trip: None,
    };
    assert!(matches!(
        mutation_failure.validate(),
        Err(ReportValidationError::Missing {
            field: "save.size_bytes"
        })
    ));

    mutation_failure.failure_reasons = save_count_reasons.to_vec();
    assert!(mutation_failure.validate().is_ok());
}

#[test]
fn frame_rate_rejects_one_percent_low_above_arithmetic_rate() {
    let mut invalid = report();
    invalid.frame_rate.as_mut().unwrap().one_percent_low_fps = 61.0;

    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Inconsistent {
            field: "one_percent_low_fps"
        })
    ));
}

#[test]
fn canonical_json_preserves_measured_benchmark_failures() {
    let mut failed = report();
    failed.passed = false;
    failed.failure_reasons = vec!["cold_start_ms".into()];
    failed.build.as_mut().unwrap().cargo_profile = "dev".into();
    failed.cold_start_ms = Some(5_000.0);

    let json = failed.to_canonical_json().unwrap();
    assert!(json.contains("\"passed\":false"));
    assert!(BenchmarkReport::from_json(&json).is_ok());
}

#[test]
fn failed_streaming_object_index_limit_is_serializable() {
    let mut failed = report();
    failed.passed = false;
    failed.failure_reasons = vec!["object_index".into()];
    failed
        .streaming
        .as_mut()
        .unwrap()
        .object_index
        .max_edit_candidates = 257;

    let json = failed.to_canonical_json().unwrap();
    assert!(BenchmarkReport::from_json(&json).is_ok());
}

#[test]
fn failed_round_trip_comparison_is_serializable() {
    let mut failed = mutation_report();
    failed.passed = false;
    failed.failure_reasons = vec!["round_trip".into()];
    failed.save.round_trip.as_mut().unwrap().passed = false;

    let json = failed.to_canonical_json().unwrap();
    assert!(json.contains("\"round_trip\":{\"passed\":false"));
    assert!(BenchmarkReport::from_json(&json).is_ok());
}

#[test]
fn passed_flag_cannot_hide_failed_acceptance_metrics() {
    let mut invalid = mutation_report();
    invalid
        .mutation_latency
        .as_mut()
        .unwrap()
        .accepted_to_reconciliation_ms
        .max = 30_001.0;
    assert!(matches!(
        invalid.validate(),
        Err(ReportValidationError::Limit {
            field: "mutation latency"
        })
    ));
}
