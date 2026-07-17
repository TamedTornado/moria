use std::{fs, path::PathBuf};

use moria_world::presentation::{AssetId, AssetLoader, AssetMissingAction, RuntimeAssetProfile};
use serde::Deserialize;

const MATERIALS_PATH: &str = "materials/materials.ron";

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
struct MaterialRegistry {
    materials: Vec<MaterialDef>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
struct MaterialDef {
    id: u8,
    key: String,
    hardness: u8,
    granular: bool,
    collision_class: CollisionClass,
    surface_class: SurfaceClass,
    albedo_layer: u16,
    normal_layer: u16,
    roughness: u8,
    state_default: u8,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
enum CollisionClass {
    Empty,
    Fluid,
    Solid,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
enum SurfaceClass {
    Empty,
    Water,
    Organic,
    Granular,
    Rock,
    Ore,
    OrganicObject,
    Masonry,
}

#[test]
fn materials_placeholder_uses_the_predeclared_required_loader_route() {
    let loader = AssetLoader::new();
    let declaration = loader.declaration(AssetId::Materials);

    assert_eq!(declaration.id.stable_id(), "moria.materials.registry");
    assert_eq!(declaration.path, MATERIALS_PATH);
    assert_eq!(loader.resolve_runtime_path(MATERIALS_PATH), Ok(declaration));
    assert_eq!(
        loader.validation_fixture(AssetId::Materials).key,
        AssetId::Materials.stable_id()
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::Materials, RuntimeAssetProfile::Development),
        AssetMissingAction::Fatal
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::Materials, RuntimeAssetProfile::Release),
        AssetMissingAction::Fatal
    );
}

#[test]
fn materials_placeholder_is_the_canonical_fourteen_material_registry() {
    let registry: MaterialRegistry = ron::from_str(
        &fs::read_to_string(asset_path())
            .expect("materials placeholder exists at its declared runtime path"),
    )
    .expect("materials placeholder uses the canonical registry schema");

    let expected = [
        ("air", 0, false, CollisionClass::Empty, SurfaceClass::Empty),
        (
            "water",
            0,
            false,
            CollisionClass::Fluid,
            SurfaceClass::Water,
        ),
        (
            "topsoil",
            28,
            false,
            CollisionClass::Solid,
            SurfaceClass::Organic,
        ),
        (
            "subsoil",
            42,
            false,
            CollisionClass::Solid,
            SurfaceClass::Organic,
        ),
        (
            "sand",
            18,
            true,
            CollisionClass::Solid,
            SurfaceClass::Granular,
        ),
        (
            "gravel",
            35,
            true,
            CollisionClass::Solid,
            SurfaceClass::Granular,
        ),
        (
            "limestone",
            82,
            false,
            CollisionClass::Solid,
            SurfaceClass::Rock,
        ),
        (
            "sandstone",
            68,
            false,
            CollisionClass::Solid,
            SurfaceClass::Rock,
        ),
        (
            "shale",
            60,
            false,
            CollisionClass::Solid,
            SurfaceClass::Rock,
        ),
        (
            "granite",
            120,
            false,
            CollisionClass::Solid,
            SurfaceClass::Rock,
        ),
        (
            "iron_ore",
            135,
            false,
            CollisionClass::Solid,
            SurfaceClass::Ore,
        ),
        (
            "wood",
            55,
            false,
            CollisionClass::Solid,
            SurfaceClass::OrganicObject,
        ),
        (
            "leaf",
            8,
            false,
            CollisionClass::Solid,
            SurfaceClass::OrganicObject,
        ),
        (
            "cut_stone",
            100,
            false,
            CollisionClass::Solid,
            SurfaceClass::Masonry,
        ),
    ];

    assert_eq!(registry.materials.len(), expected.len());
    for (id, (key, hardness, granular, collision_class, surface_class)) in
        expected.into_iter().enumerate()
    {
        let material = &registry.materials[id];
        assert_eq!(material.id, id as u8);
        assert_eq!(material.key, key);
        assert_eq!(material.hardness, hardness);
        assert_eq!(material.granular, granular);
        assert_eq!(material.collision_class, collision_class);
        assert_eq!(material.surface_class, surface_class);
        assert_eq!(material.albedo_layer, id as u16);
        assert_eq!(material.normal_layer, id as u16);
        assert_eq!(material.roughness, 128);
        assert_eq!(material.state_default, 0);
    }
}

fn asset_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("assets")
        .join(MATERIALS_PATH)
}
