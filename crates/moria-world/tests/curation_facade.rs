use moria_world::{
    CuratedManifest, CurationError, CurationReport, RegionConfig, SparseVoxelStamp,
    derive_manifest, validate_manifest,
};

fn inputs() -> (RegionConfig, SparseVoxelStamp) {
    (
        ron::de::from_bytes(include_bytes!(
            "../../../assets/config/product_one_region.ron"
        ))
        .unwrap(),
        ron::de::from_bytes(include_bytes!("../../../assets/stamps/ruin_p1.ron")).unwrap(),
    )
}

#[test]
fn feature_gated_facade_derives_and_validates_the_same_manifest() {
    let (config, stamp) = inputs();

    let first: Result<CuratedManifest, CurationError> =
        derive_manifest(config.seed, &config, &stamp);
    let second = derive_manifest(config.seed, &config, &stamp);

    assert_eq!(first.unwrap(), second.unwrap());
}

#[test]
fn facade_validation_returns_a_deterministic_typed_report() {
    let (config, stamp) = inputs();
    let manifest = derive_manifest(config.seed, &config, &stamp).unwrap();

    let mut first: CurationReport = validate_manifest(&config, &manifest).unwrap();
    let mut second = validate_manifest(&config, &manifest).unwrap();

    assert!(first.object_index_validation_us > 0);
    assert!(first.object_index_build_us > 0);
    assert!(second.object_index_validation_us > 0);
    assert!(second.object_index_build_us > 0);

    first.object_index_validation_us = 0;
    first.object_index_build_us = 0;
    second.object_index_validation_us = 0;
    second.object_index_build_us = 0;
    assert_eq!(first, second);
}
