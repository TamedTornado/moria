use moria_world::{CapsuleQ8, QueryMask, Vec3Q8, WorldPointQ8};

#[test]
fn capsule_query_values_are_constructible_by_external_consumers() {
    let capsule = CapsuleQ8::new(WorldPointQ8::new(0, 0, 0), 32, 0);

    assert_eq!(capsule.radius_q8, 32);
    assert_eq!(Vec3Q8::new(1, -2, 3).y, -2);
    assert!(QueryMask::SOLID.is_valid());
    assert!(QueryMask::WATER.is_valid());
    assert!(!QueryMask::empty().is_valid());
}
