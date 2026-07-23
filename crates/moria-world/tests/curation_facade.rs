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

    let first: Result<CurationReport, CurationError> = validate_manifest(&config, &manifest);
    let second = validate_manifest(&config, &manifest);

    assert_eq!(first.unwrap(), second.unwrap());
}

#[test]
fn facade_derivation_enforces_the_supplied_region_index_limits() {
    let (mut config, stamp) = inputs();
    config.objects.max_retained_index_bytes = 1;

    assert!(matches!(
        derive_manifest(config.seed, &config, &stamp),
        Err(CurationError::Manifest(message))
            if message.contains("ObjectIndexRetainedBytesExceeded")
                && message.contains("maximum: 1")
    ));
}

#[test]
fn facade_validation_enforces_the_supplied_region_index_limits() {
    let (mut config, stamp) = inputs();
    let manifest = derive_manifest(config.seed, &config, &stamp).unwrap();
    config.objects.max_retained_index_bytes = 1;

    let result = validate_manifest(&config, &manifest);
    assert!(
        matches!(
            &result,
            Err(CurationError::Manifest(message))
                if message.contains("ObjectIndexRetainedBytesExceeded")
                    && message.contains("maximum: 1")
        ),
        "{result:?}"
    );
}
