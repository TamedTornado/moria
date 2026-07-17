use moria_world::{
    AabbQ8, CuratedManifest, FeatureInstance, FeatureKind, ManifestError, ObjectId, ObjectKind,
    ObjectPlacement, QuantizedTransform, RouteTag, RouteWaypoint, RuinPoi, SparseVoxelStamp,
    StampRun, VoxelObjectShape, WaterBodyDef, WaterKind, WorldBounds, WorldIdentity, WorldPointQ8,
};

fn bounds() -> WorldBounds {
    WorldBounds::new(
        WorldPointQ8::new(-128_000, -32_768, -128_000),
        WorldPointQ8::new(128_000, 32_768, 128_000),
    )
    .unwrap()
}

fn placement(id: u64) -> ObjectPlacement {
    ObjectPlacement {
        id: ObjectId(id),
        kind: ObjectKind::Boulder,
        transform_q: QuantizedTransform {
            translation: WorldPointQ8::new(0, 0, 0),
            yaw_quarter_turns: 0,
            uniform_scale_q8: 256,
        },
        species: None,
        shape: VoxelObjectShape::Boulder {
            radii_q8: [256; 3],
            perturbation_key: 7,
        },
        anchor: moria_world::VoxelCoord::new(0, 0, 0),
    }
}

fn manifest() -> CuratedManifest {
    CuratedManifest {
        seed: 7,
        parameters_digest: [3; 32],
        generated_by: "test curator".into(),
        features: vec![FeatureInstance {
            id: 1,
            kind: FeatureKind::Stratum,
            bounds: AabbQ8::new(
                WorldPointQ8::new(-256, -256, -256),
                WorldPointQ8::new(256, 256, 256),
            )
            .unwrap(),
            host_material: moria_world::GRANITE,
            depth_q8: 256,
            orientation_q16: [0, 0, 0, 65_536],
            generator_key: 1,
        }],
        water_bodies: vec![WaterBodyDef {
            id: 1,
            kind: WaterKind::Lake,
            surface_y_q8: 0,
            footprint: vec![
                WorldPointQ8::new(-256, 0, -256),
                WorldPointQ8::new(256, 0, -256),
                WorldPointQ8::new(0, 0, 256),
            ],
            bed_profile_key: 5,
        }],
        objects: vec![placement(1), placement(2)],
        ruin: RuinPoi {
            placement: ObjectPlacement {
                id: ObjectId(0),
                kind: ObjectKind::Ruin,
                transform_q: QuantizedTransform {
                    translation: WorldPointQ8::new(0, 0, 0),
                    yaw_quarter_turns: 0,
                    uniform_scale_q8: 256,
                },
                species: None,
                shape: VoxelObjectShape::SparseStamp {
                    asset_key: "moria.stamps.ruin_p1".into(),
                },
                anchor: moria_world::VoxelCoord::new(0, 0, 0),
            },
            stair_bottom: WorldPointQ8::new(0, 0, 0),
            stair_top: WorldPointQ8::new(0, 256, 0),
        },
        route: vec![RouteWaypoint {
            order: 0,
            point: WorldPointQ8::new(0, 0, 0),
            tags: vec![RouteTag::Meadow, RouteTag::SignatureCarveHillside],
        }],
    }
}

#[test]
fn world_identity_is_an_immutable_value_with_valid_bounds() {
    let identity = WorldIdentity::new(7, [3; 32], bounds());

    assert_eq!(identity.seed, 7);
    assert!(bounds().contains(WorldPointQ8::new(0, 0, 0)));
    assert!(WorldBounds::new(WorldPointQ8::new(1, 0, 0), WorldPointQ8::new(1, 1, 1)).is_err());
}

#[test]
fn manifest_validation_requires_strict_canonical_order_and_feature_cap() {
    let mut unordered = manifest();
    unordered.objects.swap(0, 1);
    assert_eq!(
        unordered.validate(),
        Err(ManifestError::ObjectIdsNotStrictlyAscending)
    );

    let mut too_many_features = manifest();
    too_many_features.features = (0..17)
        .map(|id| FeatureInstance {
            id,
            ..too_many_features.features[0].clone()
        })
        .collect();
    assert_eq!(
        too_many_features.validate(),
        Err(ManifestError::FeatureCountExceedsMaximum {
            actual: 17,
            maximum: 16
        })
    );
}

#[test]
fn manifest_validation_rejects_duplicate_route_tags_and_invalid_ruin_identity() {
    let mut duplicate_tag = manifest();
    duplicate_tag.route[0].tags.push(RouteTag::Meadow);
    assert_eq!(
        duplicate_tag.validate(),
        Err(ManifestError::RouteTagsNotStrictlyAscending { order: 0 })
    );

    let mut invalid_ruin = manifest();
    invalid_ruin.ruin.placement.id = ObjectId(1);
    assert_eq!(
        invalid_ruin.validate(),
        Err(ManifestError::RuinObjectIdMustBeZero)
    );
}

#[test]
fn sparse_stamp_validation_enforces_canonical_non_overlapping_runs() {
    let stamp = SparseVoxelStamp {
        key: "moria.stamps.ruin_p1".into(),
        size_voxels: [2, 1, 1],
        pivot_voxel: [0, 0, 0],
        palette: vec![moria_world::AIR, moria_world::CUT_STONE],
        runs: vec![
            StampRun {
                start_linear: 0,
                len: 1,
                palette_index: 1,
                density: 255,
            },
            StampRun {
                start_linear: 0,
                len: 1,
                palette_index: 0,
                density: 0,
            },
        ],
        tags: Default::default(),
    };

    assert_eq!(
        stamp.validate(),
        Err(ManifestError::StampRunsNotStrictlyAscending)
    );
}
