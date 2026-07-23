use moria_world::{
    CUT_STONE, ManifestError, ObjectId, ObjectIndexConfig, ObjectKind, ObjectPlacement,
    QuantizedTransform, RuinPoi, SparseVoxelStamp, StampRun, VoxelCoord, VoxelObjectShape,
    WorldPointQ8, build_object_index, horizon_tree_ids, placement_ids_in,
    validate_object_shape_disjointness,
};

fn boulder(id: u64, x_voxels: i32, z_voxels: i32) -> ObjectPlacement {
    ObjectPlacement {
        id: ObjectId(id),
        kind: ObjectKind::Boulder,
        transform_q: QuantizedTransform {
            translation: WorldPointQ8::new(x_voxels * 64, 0, z_voxels * 64),
            yaw_quarter_turns: 0,
            uniform_scale_q8: 256,
        },
        species: None,
        shape: VoxelObjectShape::Boulder {
            radii_q8: [128, 128, 128],
            perturbation_key: id,
        },
        anchor: VoxelCoord::new(x_voxels, 0, z_voxels),
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

fn bush(id: u64, x_voxels: i32, z_voxels: i32) -> ObjectPlacement {
    let mut placement = boulder(id, x_voxels, z_voxels);
    placement.kind = ObjectKind::Bush;
    placement.shape = VoxelObjectShape::Bush {
        radii_q8: [128, 128, 128],
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
fn horizon_members_and_overlap_witnesses_are_stable() {
    let placements = vec![tree(9, 1, 1), tree(3, 2, 2), boulder(15, 40, 40)];
    let index = build_object_index(&placements, &ObjectIndexConfig::default()).unwrap();
    assert_eq!(
        horizon_tree_ids(&index, moria_world::HorizonCellKey::new(0, 0)),
        vec![ObjectId(3), ObjectId(9)]
    );

    let overlapping = vec![boulder(9, 0, 0), boulder(3, 0, 0)];
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
            lower_id: ObjectId(3),
            higher_id: ObjectId(9),
            first_voxel: VoxelCoord::new(-2, -1, -1),
        })
    );
}

#[test]
fn broad_edit_candidates_are_not_mistaken_for_exact_affected_objects() {
    let placements = (0..70)
        .map(|index| {
            boulder(
                u64::try_from(index + 1).unwrap(),
                (index % 10) * 4,
                (index / 10) * 4,
            )
        })
        .collect::<Vec<_>>();

    let index = build_object_index(&placements, &ObjectIndexConfig::default()).unwrap();

    assert_eq!(index.placements().len(), 70);
}

#[test]
fn registered_bushes_participate_in_dependency_queries() {
    let placements = vec![bush(5, 1, 1)];
    let index = build_object_index(&placements, &ObjectIndexConfig::default()).unwrap();
    let bounds = moria_world::AabbQ8::new(
        WorldPointQ8::new(-256, -256, -256),
        WorldPointQ8::new(64 * 8, 64 * 8, 64 * 8),
    )
    .unwrap();

    assert_eq!(placement_ids_in(&index, bounds), vec![ObjectId(5)]);
}

#[test]
fn exact_edit_hits_cannot_exceed_the_configured_cap() {
    let placements = (0..65)
        .map(|index| boulder(u64::try_from(index + 1).unwrap(), 0, 0))
        .collect::<Vec<_>>();
    let config = ObjectIndexConfig {
        max_edit_dependency_candidates: 256,
        max_affected_objects_per_edit: 64,
        max_sample_members_per_cell: 65,
        ..ObjectIndexConfig::default()
    };

    assert!(matches!(
        build_object_index(&placements, &config),
        Err(ManifestError::ObjectEditAffectedExceeded {
            actual: 65,
            maximum: 64,
        })
    ));
}
