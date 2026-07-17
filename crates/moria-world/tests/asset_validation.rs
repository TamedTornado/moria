use bevy::prelude::*;
use moria_world::presentation::{
    AssetId, AssetValidationError, AssetValidationPlugin, AssetValidationStatus, WorldRenderAssets,
};

#[test]
fn validation_plugin_installs_one_shared_render_asset_resource() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(AssetValidationPlugin::for_development());

    assert!(app.world().contains_resource::<WorldRenderAssets>());
    assert!(matches!(
        app.world().resource::<AssetValidationStatus>(),
        AssetValidationStatus::Failed { errors }
            if errors.contains(&AssetValidationError::RegistryInventory { registry: "licenses" })
    ));
}

#[test]
fn repeated_placements_clone_the_same_shared_handles() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(AssetValidationPlugin::for_development());

    let assets = app.world().resource::<WorldRenderAssets>();
    let expected = assets.object_handles(AssetId::BirchNear).unwrap();
    for _ in 0..1_000 {
        assert_eq!(assets.object_handles(AssetId::BirchNear), Some(expected));
    }
}
