use moria_world::{
    AIR, GRANITE, MaterialId, Voxel, VoxelCoord, WATER,
    testing::conformance::{DenseFeature, DenseFeaturePrecedence, DenseRegion, DenseWorld},
};
use proptest::prelude::*;

fn region() -> DenseRegion {
    DenseRegion::new(VoxelCoord::new(-1, -1, -1), VoxelCoord::new(2, 2, 2)).unwrap()
}

#[test]
fn dense_oracle_applies_feature_precedence_and_keeps_edits_base_relative() {
    let mut world = DenseWorld::from_base(region(), |_| Voxel::new(AIR, 0, 0, 0));
    let target = VoxelCoord::new(0, 0, 0);

    world
        .register_feature(DenseFeature::new(
            9,
            DenseFeaturePrecedence::Water,
            [(target, Voxel::new(WATER, 255, 7, 0))],
        ))
        .unwrap();
    world
        .register_feature(DenseFeature::new(
            3,
            DenseFeaturePrecedence::Object,
            [(target, Voxel::new(GRANITE, 200, 4, 0))],
        ))
        .unwrap();

    assert_eq!(
        world.sample(target).unwrap(),
        Voxel::new(GRANITE, 200, 4, 0)
    );
    assert_eq!(world.overlay_len(), 0);

    let edited = Voxel::new(MaterialId(12), 99, 2, 1);
    assert!(world.set_overlay(target, edited).unwrap());
    assert_eq!(world.sample(target).unwrap(), edited);
    assert_eq!(world.overlay_len(), 1);

    assert!(world.revert_overlay(target).unwrap());
    assert_eq!(
        world.sample(target).unwrap(),
        Voxel::new(GRANITE, 200, 4, 0)
    );
    assert_eq!(world.overlay_len(), 0);
}

#[test]
fn dense_oracle_enforces_small_region_edges_and_exposes_every_voxel() {
    let world = DenseWorld::from_base(region(), |coordinate| {
        Voxel::new(
            MaterialId((coordinate.x + 1) as u8),
            coordinate.y as u8,
            coordinate.z as u8,
            9,
        )
    });

    assert_eq!(world.voxel_count(), 27);
    assert!(world.sample(VoxelCoord::new(-1, -1, -1)).is_ok());
    assert!(world.sample(VoxelCoord::new(1, 1, 1)).is_ok());
    assert!(world.sample(VoxelCoord::new(2, 0, 0)).is_err());
    assert!(world.sample(VoxelCoord::new(0, -2, 0)).is_err());
    assert_eq!(world.all_voxels().count(), 27);
}

#[test]
fn conformance_comparison_reports_the_first_authoritative_byte_mismatch() {
    let world = DenseWorld::from_base(region(), |_| Voxel::new(AIR, 0, 0, 0));
    let mismatch = world
        .compare_voxels(|coordinate| {
            Ok::<_, ()>(if coordinate == VoxelCoord::new(0, 0, 0) {
                Voxel::new(GRANITE, 1, 2, 3)
            } else {
                Voxel::new(AIR, 0, 0, 0)
            })
        })
        .unwrap_err();

    let mismatch = mismatch.mismatch().unwrap();
    assert_eq!(mismatch.coordinate(), VoxelCoord::new(0, 0, 0));
    assert_eq!(mismatch.expected(), Voxel::new(AIR, 0, 0, 0));
    assert_eq!(mismatch.actual(), Voxel::new(GRANITE, 1, 2, 3));
}

proptest! {
    #[test]
    fn generated_edit_histories_are_invariant_to_feature_insertion_order(
        operations in prop::collection::vec((0usize..27, any::<u8>(), any::<u8>(), any::<u8>(), any::<u8>(), any::<bool>()), 1..96)
    ) {
        let target = VoxelCoord::new(0, 0, 0);
        let water = DenseFeature::new(9, DenseFeaturePrecedence::Water, [(target, Voxel::new(WATER, 255, 0, 0))]);
        let object = DenseFeature::new(3, DenseFeaturePrecedence::Object, [(target, Voxel::new(GRANITE, 255, 0, 0))]);
        let mut first = DenseWorld::from_base(region(), |_| Voxel::new(AIR, 0, 0, 0));
        let mut second = DenseWorld::from_base(region(), |_| Voxel::new(AIR, 0, 0, 0));
        first.register_feature(water.clone()).unwrap();
        first.register_feature(object.clone()).unwrap();
        second.register_feature(object).unwrap();
        second.register_feature(water).unwrap();

        for (index, material, density, state, flags, revert) in operations {
            let coordinate = first.region().coordinate_at(index).unwrap();
            if revert {
                first.revert_overlay(coordinate).unwrap();
                second.revert_overlay(coordinate).unwrap();
            } else {
                let voxel = Voxel::new(MaterialId(material % 14), density, state, flags);
                first.set_overlay(coordinate, voxel).unwrap();
                second.set_overlay(coordinate, voxel).unwrap();
            }
            first.compare_voxels(|coordinate| second.sample(coordinate)).unwrap();
        }
    }
}
