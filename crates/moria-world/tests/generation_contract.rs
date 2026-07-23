use moria_world::{
    AIR, BrickCoord, ColumnCoord, GRANITE, ProceduralClass, SUBSOIL, TOPSOIL, Voxel, VoxelCoord,
    WorldBounds, WorldIdentity, WorldPointQ8, classify_brick, evaluate_base_voxel, evaluate_column,
};

fn identity() -> WorldIdentity {
    WorldIdentity::new(
        0x4D4F_5249_415F_5031,
        [0; 32],
        WorldBounds::new(
            WorldPointQ8::new(-128_000, -32_768, -128_000),
            WorldPointQ8::new(128_000, 32_768, 128_000),
        )
        .unwrap(),
    )
}

#[test]
fn columns_are_ordered_cover_the_full_vertical_bounds_and_have_bounded_runs() {
    let identity = identity();
    let column = evaluate_column(&identity, ColumnCoord { x: 17, z: -23 });

    assert_eq!(column.runs.first().unwrap().y_min_voxel, -512);
    assert_eq!(column.runs.last().unwrap().y_max_voxel_exclusive, 512);
    assert!(column.runs.len() <= 64);
    assert!(column.runs.windows(2).all(|runs| {
        runs[0].y_max_voxel_exclusive == runs[1].y_min_voxel
            && runs[0].y_min_voxel < runs[0].y_max_voxel_exclusive
    }));
    assert!(column.runs.iter().any(|run| run.material == TOPSOIL));
    assert!(column.runs.iter().any(|run| run.material == SUBSOIL));
    assert!(column.runs.iter().any(|run| run.material == GRANITE));
}

#[test]
fn soil_runs_use_one_meter_of_topsoil_and_three_meters_of_subsoil() {
    let column = evaluate_column(&identity(), ColumnCoord { x: 17, z: -23 });
    let run_depth_voxels = |material| {
        column
            .runs
            .iter()
            .filter(|run| run.material == material)
            .map(|run| i32::from(run.y_max_voxel_exclusive - run.y_min_voxel))
            .sum::<i32>()
    };

    assert_eq!(run_depth_voxels(TOPSOIL), 4);
    assert_eq!(run_depth_voxels(SUBSOIL), 12);
}

#[test]
fn evaluation_is_independent_of_call_order() {
    let identity = identity();
    let coordinates = [
        VoxelCoord::new(-2000, -512, -2000),
        VoxelCoord::new(0, 0, 0),
        VoxelCoord::new(1999, 511, 1999),
        VoxelCoord::new(123, 251, -456),
    ];
    let forward: Vec<Voxel> = coordinates
        .iter()
        .map(|&coord| evaluate_base_voxel(&identity, coord))
        .collect();
    let reverse: Vec<Voxel> = coordinates
        .iter()
        .rev()
        .map(|&coord| evaluate_base_voxel(&identity, coord))
        .collect();

    assert_eq!(forward, reverse.into_iter().rev().collect::<Vec<_>>());
}

#[test]
fn uniform_bricks_are_classified_without_expanding_voxels() {
    let identity = identity();
    let high_brick = BrickCoord::new(0, 63, 0).unwrap();

    assert_eq!(
        classify_brick(&identity, high_brick),
        ProceduralClass::Uniform(Voxel::new(AIR, 0, 0, 0))
    );

    let low_brick = BrickCoord::new(0, 0, 0).unwrap();
    assert_eq!(
        classify_brick(&identity, low_brick),
        ProceduralClass::Uniform(Voxel::new(GRANITE, u8::MAX, 0, 0))
    );
}
