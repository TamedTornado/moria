use std::collections::BTreeMap;

use moria_world::telemetry::{
    BuildProfile, ForestFeasibilityReport, MachineProfile, ObjectIndexEvidence,
    ReportValidationError, WorstEditTargetEvidence,
};
use moria_world::{WorldBounds, WorldIdentity, WorldPointQ8};

fn identity() -> WorldIdentity {
    WorldIdentity::new(
        7,
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
        forest_area_m2: 1,
        eligible_land_area_m2: 1,
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
            broad_candidates: 0,
            exact_dependency_ids: 0,
            dependency_bricks: 0,
            tie_break_rank: 0,
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
