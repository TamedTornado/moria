use moria_world::{
    CUT_STONE, ManifestError, ObjectId, ObjectIndexConfig, ObjectKind, ObjectPlacement,
    QuantizedTransform, RuinPoi, SparseVoxelStamp, StampRun, VoxelCoord, VoxelObjectShape,
    WorldPointQ8, build_object_index, dependency_ids_at, horizon_tree_ids, placement_ids_in,
    raw_shape_contains, sample_object_ids_at, validate_object_shape_disjointness,
};

fn boulder(id: u64, x_voxels: i32, z_voxels: i32) -> ObjectPlacement {
    boulder_at(id, x_voxels, 0, z_voxels)
}

fn boulder_at(id: u64, x_voxels: i32, y_voxels: i32, z_voxels: i32) -> ObjectPlacement {
    ObjectPlacement {
        id: ObjectId(id),
        kind: ObjectKind::Boulder,
        transform_q: QuantizedTransform {
            translation: WorldPointQ8::new(x_voxels * 64, y_voxels * 64, z_voxels * 64),
            yaw_quarter_turns: 0,
            uniform_scale_q8: 256,
        },
        species: None,
        shape: VoxelObjectShape::Boulder {
            radii_q8: [128, 128, 128],
            perturbation_key: id,
        },
        anchor: VoxelCoord::new(x_voxels, y_voxels, z_voxels),
    }
}

fn tree(id: u64, x_voxels: i32, z_voxels: i32) -> ObjectPlacement {
    let mut placement = boulder(id, x_voxels, z_voxels);
    placement.kind = ObjectKind::TreeA;
    placement.shape = VoxelObjectShape::Tree {
        trunk_radius_q8: 64,
        trunk_height_q8: 256,
        canopy_radii_q8: [128, 128, 128],
    };
    placement
}

#[test]
fn object_index_queries_are_sorted_deduplicated_and_horizon_filtered() {
    let placements = vec![boulder(2, 1, 1), boulder(7, 130, 1), boulder(11, 3, 1)];
    let index = build_object_index(&placements, &ObjectIndexConfig::default()).unwrap();

    let bounds = moria_world::AabbQ8::new(
        WorldPointQ8::new(-256, -256, -256),
        WorldPointQ8::new(64 * 8, 64 * 8, 64 * 8),
    )
    .unwrap();
    assert_eq!(
        placement_ids_in(&index, bounds),
        vec![ObjectId(2), ObjectId(11)]
    );
    assert_eq!(
        horizon_tree_ids(&index, moria_world::HorizonCellKey::new(0, 0)),
        Vec::<ObjectId>::new()
    );
    assert_eq!(index.dependency_coordinate_allocation_bytes(), 0);
    assert!(index.retained_bytes() <= 16 * 1024 * 1024);
}

#[test]
fn sample_and_dependency_queries_match_brute_force_oracles() {
    let placements = vec![boulder(9, 0, 0), boulder(3, 20, 0), boulder(7, 130, 0)];
    let index = build_object_index(&placements, &ObjectIndexConfig::default()).unwrap();

    for coordinate in [
        VoxelCoord::new(0, 0, 0),
        VoxelCoord::new(2, 0, 0),
        VoxelCoord::new(20, 0, 0),
        VoxelCoord::new(127, 0, 0),
    ] {
        let mut sampled = placements
            .iter()
            .filter(|placement| raw_shape_contains(placement, coordinate))
            .map(|placement| placement.id)
            .collect::<Vec<_>>();
        sampled.sort_unstable();
        let mut dependencies = placements
            .iter()
            .filter(|placement| moria_world::dependency_contains(placement, coordinate))
            .map(|placement| placement.id)
            .collect::<Vec<_>>();
        dependencies.sort_unstable();

        assert_eq!(sample_object_ids_at(&index, coordinate), sampled);
        assert_eq!(dependency_ids_at(&index, coordinate), dependencies);
    }

    let out_of_region = VoxelCoord::new(-2_001, 0, 0);
    assert!(sample_object_ids_at(&index, out_of_region).is_empty());
    assert!(dependency_ids_at(&index, out_of_region).is_empty());
}

#[test]
fn dependency_bounds_clip_and_horizon_cells_align_to_region_minimum() {
    let placements = vec![tree(4, -1_999, -1_999)];
    let index = build_object_index(&placements, &ObjectIndexConfig::default()).unwrap();

    assert_eq!(index.records()[0].dependency_bounds.min.x, -500 * 256);
    assert_eq!(index.records()[0].dependency_bounds.min.z, -500 * 256);
    assert_eq!(
        horizon_tree_ids(&index, moria_world::HorizonCellKey::new(0, 0)),
        vec![ObjectId(4)]
    );
}

#[test]
fn horizon_cells_overlap_at_most_four_aligned_dependency_cells() {
    let placements = [tree(1, -2_000, -2_000), tree(2, -1_873, -1_873)];
    let index = build_object_index(&placements, &ObjectIndexConfig::default()).unwrap();

    let x_keys = index
        .dependency_cells()
        .iter()
        .map(|cell| cell.key.x)
        .collect::<std::collections::BTreeSet<_>>();
    let z_keys = index
        .dependency_cells()
        .iter()
        .map(|cell| cell.key.z)
        .collect::<std::collections::BTreeSet<_>>();

    assert_eq!(x_keys.len(), 2);
    assert_eq!(z_keys.len(), 2);
    assert_eq!(index.dependency_cells().len(), 4);
    assert_eq!(
        horizon_tree_ids(&index, moria_world::HorizonCellKey::new(0, 0)),
        vec![ObjectId(1), ObjectId(2)]
    );
}

#[test]
fn every_index_capacity_is_rejected_instead_of_truncated() {
    let placement = boulder(1, 0, 0);

    let config = ObjectIndexConfig {
        max_dependency_bricks_per_object: 0,
        ..Default::default()
    };
    assert!(matches!(
        build_object_index(std::slice::from_ref(&placement), &config),
        Err(ManifestError::ObjectDependencyBricksExceeded {
            object_id: ObjectId(1),
            maximum: 0,
            ..
        })
    ));

    for (placement, config) in [
        (
            boulder(1, -1_872, -1_872),
            ObjectIndexConfig {
                max_dependency_cells_per_object: 1,
                ..Default::default()
            },
        ),
        (
            placement.clone(),
            ObjectIndexConfig {
                max_sample_cells_per_object: 1,
                ..Default::default()
            },
        ),
    ] {
        assert!(matches!(
            build_object_index(std::slice::from_ref(&placement), &config),
            Err(ManifestError::ObjectIndexCellsExceeded {
                object_id: ObjectId(1),
                maximum: 1,
                ..
            })
        ));
    }

    let collocated = [boulder(1, 0, 0), boulder(2, 0, 0)];
    let config = ObjectIndexConfig {
        max_dependency_members_per_cell: 1,
        ..Default::default()
    };
    assert!(matches!(
        build_object_index(&collocated, &config),
        Err(ManifestError::ObjectIndexCellCapacityExceeded { maximum: 1, .. })
    ));

    let config = ObjectIndexConfig {
        max_sample_members_per_cell: 1,
        ..Default::default()
    };
    assert!(matches!(
        build_object_index(&collocated, &config),
        Err(ManifestError::ObjectSampleCellCapacityExceeded { maximum: 1, .. })
    ));

    let config = ObjectIndexConfig {
        max_edit_dependency_candidates: 1,
        ..Default::default()
    };
    assert!(matches!(
        build_object_index(&collocated, &config),
        Err(ManifestError::ObjectEditCandidatesExceeded { maximum: 1, .. })
    ));

    let config = ObjectIndexConfig {
        max_affected_objects_per_edit: 1,
        ..Default::default()
    };
    assert!(matches!(
        build_object_index(&collocated, &config),
        Err(ManifestError::ObjectEditAffectedExceeded { maximum: 1, .. })
    ));

    let trees = [tree(1, 0, 0), tree(2, 20, 0)];
    let config = ObjectIndexConfig {
        max_horizon_tree_members_per_cell: 1,
        ..Default::default()
    };
    assert!(matches!(
        build_object_index(&trees, &config),
        Err(ManifestError::HorizonTreeCellCapacityExceeded { maximum: 1, .. })
    ));

    let empty = build_object_index(&[], &ObjectIndexConfig::default()).unwrap();
    assert_eq!(empty.retained_bytes(), 32);
    let config = ObjectIndexConfig {
        max_retained_bytes: 31,
        ..Default::default()
    };
    assert_eq!(
        build_object_index(&[], &config).unwrap_err(),
        ManifestError::ObjectIndexRetainedBytesExceeded {
            actual: 32,
            maximum: 31,
        }
    );
}

#[test]
fn edit_affected_cap_filters_separated_broad_candidates_exactly() {
    let separated = [boulder(1, 0, 0), boulder(2, 100, 0)];
    let config = ObjectIndexConfig {
        max_affected_objects_per_edit: 1,
        ..Default::default()
    };

    assert!(build_object_index(&separated, &config).is_ok());

    let config = ObjectIndexConfig {
        max_edit_dependency_candidates: 1,
        ..Default::default()
    };
    assert!(matches!(
        build_object_index(&separated, &config),
        Err(ManifestError::ObjectEditCandidatesExceeded { maximum: 1, .. })
    ));

    let collocated = [boulder(1, 0, 0), boulder(2, 0, 0)];
    let config = ObjectIndexConfig {
        max_affected_objects_per_edit: 1,
        ..Default::default()
    };
    assert!(matches!(
        build_object_index(&collocated, &config),
        Err(ManifestError::ObjectEditAffectedExceeded { maximum: 1, .. })
    ));
}

#[test]
fn edit_affected_cap_rejects_dependencies_reached_from_an_intervening_center() {
    // Neither object's dependency contains the legal center at x = 4, but a
    // three-meter edit centered there reaches both dependency sets.
    let placements = [boulder(1, 0, 0), boulder(2, 20, 0)];
    let config = ObjectIndexConfig {
        max_affected_objects_per_edit: 1,
        ..Default::default()
    };

    assert!(matches!(
        build_object_index(&placements, &config),
        Err(ManifestError::ObjectEditAffectedExceeded { maximum: 1, .. })
    ));
}

#[test]
fn edit_affected_cap_excludes_diagonally_separated_dependencies() {
    // Their dependency boxes overlap after a 3 m cube expansion, but the
    // nearest dependency voxels are more than a 3 m sphere radius apart.
    let diagonal = [boulder(1, 0, 0), boulder(2, 30, 30)];
    let config = ObjectIndexConfig {
        max_affected_objects_per_edit: 1,
        ..Default::default()
    };

    assert!(build_object_index(&diagonal, &config).is_ok());
}

#[test]
fn edit_affected_cap_excludes_vertically_separated_dependencies() {
    let vertical = [boulder_at(1, 0, -100, 0), boulder_at(2, 0, 100, 0)];
    let config = ObjectIndexConfig {
        max_affected_objects_per_edit: 1,
        ..Default::default()
    };

    assert!(build_object_index(&vertical, &config).is_ok());
}

#[test]
fn edit_affected_cap_handles_maximum_candidate_count() {
    let placements = (0..16)
        .flat_map(|x| {
            (0..16).map(move |z| boulder(u64::from((x * 16 + z + 1) as u32), x * 4, z * 4))
        })
        .collect::<Vec<_>>();
    let config = ObjectIndexConfig {
        max_edit_dependency_candidates: 256,
        max_affected_objects_per_edit: 64,
        ..Default::default()
    };

    assert!(build_object_index(&placements, &config).is_ok());
}

#[test]
fn edit_affected_cap_skips_region_volume_for_spread_candidates() {
    // All placements are members of the same horizontal edit broad phase, but
    // their centers span the entire legal vertical range.  No radius-3 m edit
    // can reach more than the exact cap, so index construction must establish
    // that from the bounded candidate geometry rather than scan the union AABB.
    let placements = (0..256)
        .map(|index| {
            boulder_at(
                u64::try_from(index + 1).expect("nonnegative test index"),
                (index % 16) * 4,
                -512 + index * 4,
                (index / 16) * 4,
            )
        })
        .collect::<Vec<_>>();
    let config = ObjectIndexConfig {
        max_edit_dependency_candidates: 256,
        max_affected_objects_per_edit: 64,
        ..Default::default()
    };

    assert!(build_object_index(&placements, &config).is_ok());
}

#[test]
fn edit_affected_cap_uses_overlap_events_for_distant_candidates() {
    // These 65 boxes meet at one conservative edit-center event, but the
    // boulders occupy eight distant corners so no radius-3 m edit reaches
    // more than nine exact dependency sets. The remaining candidates stretch
    // the region height. Validation must therefore inspect the local overlap
    // geometry, not every voxel in the union of candidate center boxes.
    let mut placements = (1..=65)
        .map(|id| {
            let corner = id % 8;
            boulder_at(
                id,
                if corner & 1 == 0 { -16 } else { 16 },
                if corner & 2 == 0 { -16 } else { 16 },
                if corner & 4 == 0 { -16 } else { 16 },
            )
        })
        .collect::<Vec<_>>();
    placements.extend(
        (-512..=-32)
            .step_by(8)
            .chain((32..512).step_by(8))
            .enumerate()
            .map(|(index, y)| {
                boulder_at(
                    u64::try_from(index + 66).expect("nonnegative test index"),
                    0,
                    y,
                    0,
                )
            }),
    );
    let config = ObjectIndexConfig {
        max_edit_dependency_candidates: 256,
        max_affected_objects_per_edit: 64,
        max_sample_members_per_cell: 255,
        ..Default::default()
    };

    let result = build_object_index(&placements, &config);
    assert!(result.is_ok(), "{result:?}");
}

#[test]
fn placement_query_outside_region_does_not_overflow_grid_math() {
    let placements = [boulder(1, 0, 0)];
    let index = build_object_index(&placements, &ObjectIndexConfig::default()).unwrap();
    let bounds = moria_world::AabbQ8::new(
        WorldPointQ8::new(i32::MAX - 8, 0, i32::MAX - 8),
        WorldPointQ8::new(i32::MAX, 1, i32::MAX),
    )
    .unwrap();

    assert!(placement_ids_in(&index, bounds).is_empty());
}

#[test]
fn horizon_members_and_overlap_witnesses_are_stable() {
    let placements = vec![tree(9, 1, 1), tree(3, 2, 2), boulder(15, 40, 40)];
    let index = build_object_index(&placements, &ObjectIndexConfig::default()).unwrap();
    assert_eq!(
        horizon_tree_ids(&index, moria_world::HorizonCellKey::new(7, 7)),
        vec![ObjectId(3), ObjectId(9)]
    );

    let overlapping = vec![
        boulder(9, 0, 0),
        boulder(2, 50, 0),
        boulder(3, 50, 0),
        boulder(1, 0, 0),
    ];
    let expected_voxel = (-8..=8)
        .flat_map(|x| (-8..=8).map(move |y| (x, y)))
        .flat_map(|(x, y)| (-8..=8).map(move |z| VoxelCoord::new(x, y, z)))
        .find(|coordinate| {
            raw_shape_contains(&overlapping[0], *coordinate)
                && raw_shape_contains(&overlapping[3], *coordinate)
        })
        .unwrap();
    let index = build_object_index(&overlapping, &ObjectIndexConfig::default()).unwrap();
    let ruin = RuinPoi {
        placement: ObjectPlacement {
            id: ObjectId(0),
            kind: ObjectKind::Ruin,
            transform_q: QuantizedTransform {
                translation: WorldPointQ8::new(10_000, 0, 10_000),
                yaw_quarter_turns: 0,
                uniform_scale_q8: 256,
            },
            species: None,
            shape: VoxelObjectShape::SparseStamp {
                asset_key: "test.ruin".into(),
            },
            anchor: VoxelCoord::new(0, 0, 0),
        },
        stair_bottom: WorldPointQ8::new(0, 0, 0),
        stair_top: WorldPointQ8::new(0, 0, 0),
    };
    let stamp = SparseVoxelStamp {
        key: "test.ruin".into(),
        size_voxels: [1, 1, 1],
        pivot_voxel: [0, 0, 0],
        palette: vec![CUT_STONE],
        runs: vec![StampRun {
            start_linear: 0,
            len: 1,
            palette_index: 0,
            density: 255,
        }],
        tags: Default::default(),
    };
    assert_eq!(
        validate_object_shape_disjointness(&index, &ruin, &stamp),
        Err(ManifestError::ObjectShapeOverlap {
            lower_id: ObjectId(1),
            higher_id: ObjectId(9),
            first_voxel: expected_voxel,
        })
    );
}
