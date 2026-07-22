use bevy::prelude::*;
use moria_world::presentation::{
    AssetId, AssetValidationError, AssetValidationPlugin, AssetValidationStatus, WorldRenderAssets,
};

#[test]
fn failed_validation_does_not_publish_render_assets() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(AssetValidationPlugin::for_development());

    assert!(!app.world().contains_resource::<WorldRenderAssets>());
    assert!(matches!(
        app.world().resource::<AssetValidationStatus>(),
        AssetValidationStatus::Failed { errors }
            if errors.contains(&AssetValidationError::RegistryInventory { registry: "licenses" })
    ));
}

#[test]
fn repeated_placements_clone_the_same_shared_handles() {
    let assets = WorldRenderAssets::default();
    let expected = assets.object_handles(AssetId::BirchNear).unwrap();
    for _ in 0..1_000 {
        assert_eq!(assets.object_handles(AssetId::BirchNear), Some(expected));
    }
}
