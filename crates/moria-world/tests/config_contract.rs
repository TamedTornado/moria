use moria_world::{
    config::{
        parameters_digest_from_bytes, CollisionClass, InputConfig, MaterialRegistry,
        PresentationConfig, RangeQ8, RegionConfig,
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
    assert!(validate_input_config(&InputConfig::default()).is_ok());
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

    let mut materials = MaterialRegistry::default();
    materials.materials[13].normal_layer = 14;
    assert!(validate_material_registry(&materials).is_err());

    let mut presentation = PresentationConfig::default();
    presentation.streaming.bands[1].start_m = 65;
    assert!(validate_presentation_config(&presentation).is_err());

    let mut input = InputConfig::default();
    input.bindings[1].action = input.bindings[0].action;
    assert!(validate_input_config(&input).is_err());
}

#[test]
fn validation_rejects_values_outside_documented_product_one_envelopes() {
    let mut region = RegionConfig::default();
    region.bounds.x_min_m = i16::MIN;
    region.bounds.x_max_m = i16::MAX;
    assert!(validate_region_config(&region).is_err());

    let mut region = RegionConfig::default();
    region.objects.birch_trunk_radius_q8 = RangeQ8 {
        min_q8: 1,
        max_q8: 2,
    };
    assert!(validate_region_config(&region).is_err());

    let mut region = RegionConfig::default();
    region.biome.tree_species_mix_percent = [0, 100];
    assert!(validate_region_config(&region).is_err());

    let mut presentation = PresentationConfig::default();
    presentation.streaming.bands[0].end_m = 63;
    presentation.streaming.bands[1].start_m = 63;
    assert!(validate_presentation_config(&presentation).is_err());

    let mut presentation = PresentationConfig::default();
    presentation.mutation.min_radius_q8 = 1;
    assert!(validate_presentation_config(&presentation).is_err());

    let mut presentation = PresentationConfig::default();
    presentation.rendering.object_visibility_m = 319;
    assert!(validate_presentation_config(&presentation).is_err());

    let mut presentation = PresentationConfig::default();
    presentation.mutation.dig_strength = 0;
    assert!(validate_presentation_config(&presentation).is_err());

    let mut presentation = PresentationConfig::default();
    presentation.benchmark.watchdog_s = 0;
    assert!(validate_presentation_config(&presentation).is_err());

    let mut presentation = PresentationConfig::default();
    presentation.benchmark.forest_object_index_build_max_ms = 1_001;
    assert!(validate_presentation_config(&presentation).is_err());
}

#[test]
fn input_validation_rejects_unknown_empty_and_duplicate_physical_bindings() {
    let mut input = InputConfig::default();
    input.bindings[0].keyboard_mouse[0] = "NotARealKey".into();
    assert!(validate_input_config(&input).is_err());

    let mut input = InputConfig::default();
    input.bindings[0].keyboard_mouse[0].clear();
    assert!(validate_input_config(&input).is_err());

    let mut input = InputConfig::default();
    input.bindings[0].keyboard_mouse.push("W".into());
    assert!(validate_input_config(&input).is_err());
}

#[test]
fn input_validation_requires_each_action_to_keep_its_complete_product_one_bindings() {
    let mut input = InputConfig::default();
    let (move_binding, brick_bounds_binding) = input.bindings.split_at_mut(9);
    std::mem::swap(
        &mut move_binding[0].keyboard_mouse,
        &mut brick_bounds_binding[0].keyboard_mouse,
    );
    assert!(validate_input_config(&input).is_err());

    let mut input = InputConfig::default();
    input.bindings[0].keyboard_mouse.pop();
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
