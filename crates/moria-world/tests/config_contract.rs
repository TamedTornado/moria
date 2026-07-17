use moria_world::{
    config::{
        CollisionClass, InputConfig, MaterialRegistry, PresentationConfig, RegionConfig,
        parameters_digest_from_bytes,
    },
    config_validation::{
        validate_input_config, validate_material_registry, validate_presentation_config,
        validate_region_config,
    },
};

#[test]
fn product_one_defaults_are_valid() {
    assert!(validate_region_config(&RegionConfig::default()).is_ok());
    assert!(validate_material_registry(&MaterialRegistry::default()).is_ok());
    assert!(validate_presentation_config(&PresentationConfig::default()).is_ok());
}

#[test]
fn region_ron_rejects_unknown_fields() {
    let mut source = ron::ser::to_string(&RegionConfig::default()).unwrap();
    source.pop();
    source.push_str(", unexpected: true)");
    assert!(ron::from_str::<RegionConfig>(&source).is_err());
}

#[test]
fn validation_rejects_documented_cross_field_violations() {
    let mut region = RegionConfig::default();
    region.biome.tree_species_mix_percent = [60, 30];
    assert!(validate_region_config(&region).is_err());

    let mut materials = MaterialRegistry::default();
    materials.materials[2].collision_class = CollisionClass::Fluid;
    assert!(validate_material_registry(&materials).is_err());

    let mut presentation = PresentationConfig::default();
    presentation.streaming.bands[1].start_m = 65;
    assert!(validate_presentation_config(&presentation).is_err());

    let mut input = InputConfig::default();
    input.bindings[1].action = input.bindings[0].action;
    assert!(validate_input_config(&input).is_err());
}

#[test]
fn only_authoritative_bytes_contribute_to_parameters_digest() {
    let digest = parameters_digest_from_bytes(b"canonical region", b"ruin stamp");
    assert_eq!(
        digest,
        parameters_digest_from_bytes(b"canonical region", b"ruin stamp")
    );
    assert_ne!(
        digest,
        parameters_digest_from_bytes(b"canonical region changed", b"ruin stamp")
    );
    assert_ne!(
        digest,
        parameters_digest_from_bytes(b"canonical region", b"ruin stamp changed")
    );
}
