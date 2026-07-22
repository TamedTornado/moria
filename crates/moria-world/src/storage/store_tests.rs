use super::store::WorldStore;
use crate::{
    AIR, GRANITE, Voxel, VoxelCoord, WorldBounds, WorldIdentity, WorldPointQ8, evaluate_base_voxel,
};

fn identity() -> WorldIdentity {
    let bounds = WorldBounds::new(
        WorldPointQ8::new(-128_000, -32_768, -128_000),
        WorldPointQ8::new(128_000, 32_768, 128_000),
    )
    .unwrap();
    WorldIdentity::new(0xD3E1_A5E5, [0; 32], bounds)
}

#[test]
fn inactive_samples_regenerate_base_without_materializing_detail() {
    let identity = identity();
    let mut store = WorldStore::new(identity);
    let coordinate = VoxelCoord::new(0, 0, 0);

    assert_eq!(
        store.current_voxel(coordinate),
        evaluate_base_voxel(&identity, coordinate)
    );
    assert_eq!(store.active_brick_count(), 0);

    store.materialize_detail(coordinate.to_brick_coord().unwrap());

    assert_eq!(
        store.current_voxel(coordinate),
        evaluate_base_voxel(&identity, coordinate)
    );
    assert_eq!(store.active_brick_count(), 1);
}

#[test]
fn sorted_deltas_override_base_and_exact_reversions_disappear() {
    let identity = identity();
    let mut store = WorldStore::new(identity);
    let first = VoxelCoord::new(-2_000, -512, -2_000);
    let second = VoxelCoord::new(-1_999, -512, -2_000);
    let changed = Voxel::new(AIR, 0, 0, 0);

    let revision = store.commit_current([(second, changed), (first, changed)]);

    assert_eq!(revision, 1);
    assert_eq!(store.revision(), revision);
    assert_eq!(store.current_voxel(first), changed);
    assert_eq!(store.current_voxel(second), changed);
    assert_eq!(store.delta_count(), 2);
    assert!(store.delta_entries_are_sorted());

    let reverted = store.commit_current([(first, evaluate_base_voxel(&identity, first))]);

    assert_eq!(reverted, 2);
    assert_eq!(
        store.current_voxel(first),
        evaluate_base_voxel(&identity, first)
    );
    assert_eq!(store.current_voxel(second), changed);
    assert_eq!(store.delta_count(), 1);
    assert!(store.delta_entries_are_sorted());
}

#[test]
fn empty_or_base_equal_commits_do_not_advance_the_revision() {
    let identity = identity();
    let mut store = WorldStore::new(identity);
    let coordinate = VoxelCoord::new(-2_000, -512, -2_000);

    assert_eq!(store.commit_current([]), 0);
    assert_eq!(
        store.commit_current([(coordinate, Voxel::new(GRANITE, u8::MAX, 0, 0))]),
        0
    );
    assert_eq!(store.revision(), 0);
}

#[test]
fn materializing_after_an_edit_uses_the_current_world_revision() {
    let identity = identity();
    let mut store = WorldStore::new(identity);
    let coordinate = VoxelCoord::new(-2_000, -512, -2_000);
    let brick = coordinate.to_brick_coord().unwrap();

    assert_eq!(
        store.commit_current([(coordinate, Voxel::new(AIR, 0, 0, 0))]),
        1
    );

    store.materialize_detail(brick);

    assert_eq!(store.active_revision(brick), Some(store.revision()));
}

#[test]
fn unchanged_bricks_do_not_receive_another_bricks_batch_revision() {
    let identity = identity();
    let mut store = WorldStore::new(identity);
    let unchanged = VoxelCoord::new(-2_000, -512, -2_000);
    let changed = VoxelCoord::new(-1_984, -512, -2_000);
    let unchanged_brick = unchanged.to_brick_coord().unwrap();
    store.materialize_detail(unchanged_brick);

    assert_eq!(store.active_revision(unchanged_brick), Some(0));
    assert_eq!(
        store.commit_current([
            (unchanged, evaluate_base_voxel(&identity, unchanged)),
            (changed, Voxel::new(AIR, 0, 0, 0)),
        ]),
        1
    );

    assert_eq!(store.active_revision(unchanged_brick), Some(0));
}
