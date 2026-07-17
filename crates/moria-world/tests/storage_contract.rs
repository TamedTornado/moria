use std::mem::size_of;

use moria_world::{
    AIR, BrickCoord, CollisionClass, MaterialId, MaterialRegistry, Voxel, VoxelCoord, WATER,
    WorldPointQ8, material_present, solid_collision, water_volume,
};

#[test]
fn q8_points_floor_to_voxels_and_respect_every_region_face() {
    let cases = [
        (WorldPointQ8::new(-1, -1, -1), VoxelCoord::new(-1, -1, -1)),
        (WorldPointQ8::new(0, 0, 0), VoxelCoord::new(0, 0, 0)),
        (WorldPointQ8::new(63, 63, 63), VoxelCoord::new(0, 0, 0)),
        (WorldPointQ8::new(64, 64, 64), VoxelCoord::new(1, 1, 1)),
        (
            WorldPointQ8::new(-128_000, -32_768, -128_000),
            VoxelCoord::new(-2_000, -512, -2_000),
        ),
        (
            WorldPointQ8::new(127_999, 32_767, 127_999),
            VoxelCoord::new(1_999, 511, 1_999),
        ),
    ];

    for (point, expected) in cases {
        assert_eq!(point.to_voxel_coord().unwrap(), expected);
    }

    for point in [
        WorldPointQ8::new(-128_001, 0, 0),
        WorldPointQ8::new(128_000, 0, 0),
        WorldPointQ8::new(0, -32_769, 0),
        WorldPointQ8::new(0, 32_768, 0),
        WorldPointQ8::new(0, 0, -128_001),
        WorldPointQ8::new(0, 0, 128_000),
    ] {
        assert!(point.to_voxel_coord().is_err());
    }
}

#[test]
fn bounded_voxels_convert_to_base_relative_bricks() {
    assert_eq!(
        VoxelCoord::new(-2_000, -512, -2_000)
            .to_brick_coord()
            .unwrap(),
        BrickCoord::new(0, 0, 0).unwrap()
    );
    assert_eq!(
        VoxelCoord::new(-1_985, -497, -1_985)
            .to_brick_coord()
            .unwrap(),
        BrickCoord::new(0, 0, 0).unwrap()
    );
    assert_eq!(
        VoxelCoord::new(1_999, 511, 1_999).to_brick_coord().unwrap(),
        BrickCoord::new(249, 63, 249).unwrap()
    );
    assert!(VoxelCoord::new(2_000, 0, 0).to_brick_coord().is_err());
    assert!(BrickCoord::new(250, 0, 0).is_err());
}

#[test]
fn brick_local_indices_use_x_z_y_linear_order() {
    let cases = [
        (VoxelCoord::new(-2_000, -512, -2_000), 0),
        (VoxelCoord::new(-1_985, -512, -2_000), 15),
        (VoxelCoord::new(-2_000, -512, -1_999), 16),
        (VoxelCoord::new(-1_985, -512, -1_985), 255),
        (VoxelCoord::new(-2_000, -511, -2_000), 256),
        (VoxelCoord::new(-1_985, -497, -1_985), 4_095),
    ];

    for (voxel, expected_index) in cases {
        let (_, local_index) = voxel.to_brick_and_local_index().unwrap();
        assert_eq!(local_index, expected_index);
    }
}

#[test]
fn voxel_layout_materials_and_predicates_follow_the_truth_contract() {
    assert_eq!(size_of::<Voxel>(), 4);

    let materials = MaterialRegistry::default();
    let expected = [
        (0, "air"),
        (1, "water"),
        (2, "topsoil"),
        (3, "subsoil"),
        (4, "sand"),
        (5, "gravel"),
        (6, "limestone"),
        (7, "sandstone"),
        (8, "shale"),
        (9, "granite"),
        (10, "iron_ore"),
        (11, "wood"),
        (12, "leaf"),
        (13, "cut_stone"),
    ];
    for (id, key) in expected {
        assert_eq!(materials.materials[id].id, MaterialId(id as u8));
        assert_eq!(materials.materials[id].key, key);
        assert_eq!(
            materials.materials[id].collision_class,
            match id {
                0 => CollisionClass::Empty,
                1 => CollisionClass::Fluid,
                _ => CollisionClass::Solid,
            }
        );
    }

    let empty_air = Voxel::new(AIR, 255, 0, 0);
    let partial_water = Voxel::new(WATER, 1, 0, 0);
    let partial_solid = Voxel::new(MaterialId(2), 127, 0, 0);
    let threshold_solid = Voxel::new(MaterialId(2), 128, 0, 0);

    assert!(!material_present(empty_air));
    assert!(!water_volume(empty_air));
    assert!(!solid_collision(empty_air, &materials));
    assert!(material_present(partial_water));
    assert!(water_volume(partial_water));
    assert!(!solid_collision(partial_water, &materials));
    assert!(material_present(partial_solid));
    assert!(!water_volume(partial_solid));
    assert!(!solid_collision(partial_solid, &materials));
    assert!(solid_collision(threshold_solid, &materials));

    for density in [0, 1, 127, 128, 255] {
        let water = Voxel::new(WATER, density, 0, 0);
        let solid = Voxel::new(MaterialId(2), density, 0, 0);
        assert_eq!(material_present(water), density > 0);
        assert_eq!(water_volume(water), density > 0);
        assert!(!solid_collision(water, &materials));
        assert_eq!(material_present(solid), density > 0);
        assert!(!water_volume(solid));
        assert_eq!(solid_collision(solid, &materials), density >= 128);
    }
}
