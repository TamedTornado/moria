//! Integration coverage for the package's external-consumer boundary.

use moria::canonical::{PlacementFixedFormat, PlacementScalar};

#[test]
fn public_library_target_is_available_to_external_consumers() {
    let format = PlacementFixedFormat::try_new(1).unwrap();
    let product = PlacementScalar::from_raw(1)
        .try_mul(PlacementScalar::from_raw(3), format)
        .unwrap();
    assert_eq!(product.raw(), 2);
}
