use moria_world::{
    MAX_RAY_DISTANCE_Q8, MAX_RAY_VOXEL_VISITS, QueryError, QueryLimitKind, QueryMask, WorldPointQ8,
    WorldRayQ8,
};

#[test]
fn ray_contract_exports_the_exact_public_limits_and_validates_direction() {
    assert_eq!(MAX_RAY_DISTANCE_Q8, 16_384);
    assert_eq!(MAX_RAY_VOXEL_VISITS, 448);

    assert!(WorldRayQ8::new(WorldPointQ8::new(0, 0, 0), [65_536, 0, 0]).is_ok());
    assert!(matches!(
        WorldRayQ8::new(WorldPointQ8::new(0, 0, 0), [0, 0, 0]),
        Err(QueryError::InvalidInput)
    ));
    assert!(QueryMask::SOLID.matches(QueryMask::SOLID));
    assert!(!QueryMask::SOLID.matches(QueryMask::WATER));
    assert!((QueryMask::SOLID | QueryMask::WATER).matches(QueryMask::WATER));
    assert_eq!(QueryLimitKind::RayDistance, QueryLimitKind::RayDistance);
}
