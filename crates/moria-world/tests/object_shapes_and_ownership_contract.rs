use moria_world::{
    AIR, BaseVoxel, CUT_STONE, CollisionClass, GRANITE, LEAF, MaterialDef, MaterialRegistry,
    ObjectId, ObjectKind, ObjectPlacement, QuantizedTransform, SolidPresentationOwner,
    SparseVoxelStamp, StampRun, VOXEL_EDGE_Q8, Voxel, VoxelCoord, VoxelObjectShape, VoxelOffset,
    VoxelSource, WOOD, WorldPointQ8, dependency_contains, evaluate_base_voxel_with_objects,
    raw_shape_bounds, raw_shape_contains, sample_object_shape, sample_sparse_stamp,
    solid_presentation_owner,
};

fn placement(shape: VoxelObjectShape) -> ObjectPlacement {
    ObjectPlacement {
        id: ObjectId(9),
        kind: ObjectKind::TreeA,
        transform_q: QuantizedTransform {
            translation: WorldPointQ8::new(0, 0, 0),
            yaw_quarter_turns: 0,
            uniform_scale_q8: 256,
        },
        species: None,
        shape,
        anchor: VoxelCoord::new(0, 0, 0),
    }
}

fn materials() -> MaterialRegistry {
    MaterialRegistry {
        materials: vec![
            MaterialDef {
                id: AIR,
                key: "air".into(),
                hardness: 0,
                granular: false,
                collision_class: CollisionClass::Empty,
                surface_class: moria_world::SurfaceClass::Empty,
                albedo_layer: 0,
                normal_layer: 0,
                roughness: 0,
            },
            MaterialDef {
                id: GRANITE,
                key: "granite".into(),
                hardness: 64,
                granular: false,
                collision_class: CollisionClass::Solid,
                surface_class: moria_world::SurfaceClass::Rock,
                albedo_layer: 0,
                normal_layer: 0,
                roughness: 0,
            },
        ],
    }
}

#[test]
fn analytic_tree_samples_trunk_and_canopy_with_one_object_source() {
    let placement = placement(VoxelObjectShape::Tree {
        trunk_radius_q8: 64,
        trunk_height_q8: 256,
        canopy_radii_q8: [128, 128, 128],
    });

    assert_eq!(
        sample_object_shape(&placement, VoxelCoord::new(0, 0, 0)),
        Some(Voxel::new(WOOD, 255, 0, 0))
    );
    assert_eq!(
        sample_object_shape(&placement, VoxelCoord::new(1, 2, 0)),
        Some(Voxel::new(LEAF, 255, 0, 0))
    );
    assert!(raw_shape_contains(&placement, VoxelCoord::new(1, 2, 0)));
    assert!(!raw_shape_contains(&placement, VoxelCoord::new(5, 5, 5)));
}

#[test]
fn sparse_stamp_sampling_rotates_about_its_pivot_and_preserves_air_carves() {
    let stamp = SparseVoxelStamp {
        key: "test.stamp".into(),
        size_voxels: [2, 1, 2],
        pivot_voxel: [0, 0, 0],
        palette: vec![AIR, CUT_STONE],
        runs: vec![
            StampRun {
                start_linear: 0,
                len: 1,
                palette_index: 1,
                density: 255,
            },
            StampRun {
                start_linear: 1,
                len: 1,
                palette_index: 0,
                density: 0,
            },
        ],
        tags: Default::default(),
    };
    let mut placement = placement(VoxelObjectShape::SparseStamp {
        asset_key: stamp.key.clone(),
    });
    placement.kind = ObjectKind::Ruin;
    placement.transform_q.yaw_quarter_turns = 1;

    assert_eq!(
        sample_sparse_stamp(&placement, &stamp, VoxelCoord::new(0, 0, 0)),
        Some(Voxel::new(CUT_STONE, 255, 0, 0))
    );
    assert_eq!(
        sample_sparse_stamp(&placement, &stamp, VoxelCoord::new(0, 0, 1)),
        Some(Voxel::new(AIR, 0, 0, 0))
    );
    assert_eq!(
        sample_sparse_stamp(&placement, &stamp, VoxelCoord::new(1, 0, 0)),
        None
    );
    placement.shape = VoxelObjectShape::SparseStamp {
        asset_key: "different.stamp".into(),
    };
    assert_eq!(
        sample_sparse_stamp(&placement, &stamp, VoxelCoord::new(0, 0, 0)),
        None
    );
}

#[test]
fn owner_routing_uses_regenerated_base_provenance_not_current_material() {
    let materials = materials();
    let current = Voxel::new(GRANITE, 255, 0, 0);

    assert_eq!(
        solid_presentation_owner(current, VoxelSource::Object(ObjectId(9)), &materials),
        Some(SolidPresentationOwner::NonRuinObject(ObjectId(9)))
    );
    assert_eq!(
        solid_presentation_owner(current, VoxelSource::Ruin(ObjectId(0)), &materials),
        Some(SolidPresentationOwner::TerrainChunk)
    );
    assert_eq!(
        solid_presentation_owner(Voxel::new(AIR, 0, 0, 0), VoxelSource::Terrain, &materials),
        None
    );
}

#[test]
fn lazy_dependency_contains_raw_cells_and_excludes_distant_cells() {
    let placement = placement(VoxelObjectShape::Boulder {
        radii_q8: [96; 3],
        perturbation_key: 3,
    });

    assert!(moria_world::OBJECT_EXTRACTION_STENCIL.len() <= 512);
    assert!(dependency_contains(&placement, VoxelCoord::new(0, 0, 0)));
    assert!(!dependency_contains(
        &placement,
        VoxelCoord::new(64, 64, 64)
    ));
}

#[test]
fn extraction_stencil_contains_every_declared_object_extractor_read() {
    let stencil = moria_world::OBJECT_EXTRACTION_STENCIL;
    let unique = stencil
        .iter()
        .map(|offset| (offset.x, offset.y, offset.z))
        .collect::<std::collections::BTreeSet<_>>();

    assert!(stencil.len() <= 512);
    assert_eq!(
        unique.len(),
        stencil.len(),
        "stencil offsets must be unique"
    );
    for y in -2..=2 {
        for z in -2..=2 {
            for x in -2..=2 {
                assert!(
                    stencil.contains(&VoxelOffset { x, y, z }),
                    "missing extractor read ({x}, {y}, {z})"
                );
            }
        }
    }
}

#[test]
fn base_object_evaluation_applies_ruin_then_lowest_object_id_then_terrain_precedence() {
    let mut higher = placement(VoxelObjectShape::Bush { radii_q8: [256; 3] });
    higher.id = ObjectId(12);
    let mut lower = higher.clone();
    lower.id = ObjectId(3);
    let stamp = SparseVoxelStamp {
        key: "test.stamp".into(),
        size_voxels: [1, 1, 1],
        pivot_voxel: [0, 0, 0],
        palette: vec![AIR],
        runs: vec![StampRun {
            start_linear: 0,
            len: 1,
            palette_index: 0,
            density: 0,
        }],
        tags: Default::default(),
    };
    let mut ruin = placement(VoxelObjectShape::SparseStamp {
        asset_key: stamp.key.clone(),
    });
    ruin.id = ObjectId(0);
    ruin.kind = ObjectKind::Ruin;
    let terrain = Voxel::new(GRANITE, 255, 0, 0);

    assert_eq!(
        evaluate_base_voxel_with_objects(
            terrain,
            VoxelCoord::new(0, 0, 0),
            [&higher, &lower],
            &ruin,
            &stamp,
        ),
        BaseVoxel {
            voxel: Voxel::new(AIR, 0, 0, 0),
            source: VoxelSource::Ruin(ObjectId(0)),
        }
    );
    assert_eq!(
        evaluate_base_voxel_with_objects(
            terrain,
            VoxelCoord::new(1, 0, 0),
            [&higher, &lower],
            &ruin,
            &stamp,
        ),
        BaseVoxel {
            voxel: Voxel::new(LEAF, 255, 0, 0),
            source: VoxelSource::Object(ObjectId(3)),
        }
    );
    assert_eq!(
        evaluate_base_voxel_with_objects(
            terrain,
            VoxelCoord::new(20, 20, 20),
            [&higher, &lower],
            &ruin,
            &stamp,
        ),
        BaseVoxel {
            voxel: terrain,
            source: VoxelSource::Terrain,
        }
    );
}

#[test]
fn analytic_bounds_cover_tree_cells_when_canopy_extends_below_anchor() {
    let placement = placement(VoxelObjectShape::Tree {
        trunk_radius_q8: 32,
        trunk_height_q8: 64,
        canopy_radii_q8: [128; 3],
    });
    let bounds = raw_shape_bounds(&placement).expect("analytic tree has finite bounds");

    for y in -8..=8 {
        for z in -8..=8 {
            for x in -8..=8 {
                let coord = VoxelCoord::new(x, y, z);
                if raw_shape_contains(&placement, coord) {
                    assert!(
                        bounds.contains(WorldPointQ8::new(
                            x * VOXEL_EDGE_Q8 + VOXEL_EDGE_Q8 / 2,
                            y * VOXEL_EDGE_Q8 + VOXEL_EDGE_Q8 / 2,
                            z * VOXEL_EDGE_Q8 + VOXEL_EDGE_Q8 / 2,
                        )),
                        "bounds omitted sampled cell {coord:?}"
                    );
                }
            }
        }
    }
}
