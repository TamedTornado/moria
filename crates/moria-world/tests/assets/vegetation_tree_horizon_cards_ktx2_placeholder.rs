use std::{fs, path::PathBuf};

use moria_world::presentation::{AssetId, AssetLoader, AssetMissingAction, RuntimeAssetProfile};

const KTX2_IDENTIFIER: [u8; 12] = [
    0xAB, b'K', b'T', b'X', b' ', b'2', b'0', 0xBB, 0x0D, 0x0A, 0x1A, 0x0A,
];
const KTX_SS_BASIS_LZ: u32 = 1;
const HORIZON_CARD_LAYERS: [&str; 6] = [
    "birch_color",
    "birch_normal",
    "birch_opacity",
    "pine_color",
    "pine_normal",
    "pine_opacity",
];

fn read_u32_le(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
}

fn asset_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("assets/vegetation/tree_horizon_cards.ktx2")
}

#[test]
fn tree_horizon_cards_placeholder_is_a_mipmapped_basis_ktx2_atlas() {
    let bytes = fs::read(asset_path()).expect("tree horizon card placeholder exists");

    assert!(bytes.starts_with(&KTX2_IDENTIFIER));
    assert_eq!(read_u32_le(&bytes, 12), 0, "Basis textures use vkFormat 0");
    assert_eq!(
        read_u32_le(&bytes, 16),
        1,
        "Basis texture type size is one byte"
    );
    assert!(read_u32_le(&bytes, 20) >= 2, "atlas has a nonzero width");
    assert!(read_u32_le(&bytes, 24) >= 2, "atlas has a nonzero height");
    assert_eq!(read_u32_le(&bytes, 28), 0, "atlas is two-dimensional");
    assert_eq!(
        read_u32_le(&bytes, 32),
        HORIZON_CARD_LAYERS.len() as u32,
        "atlas stores ordered birch/pine color, normal, and opacity card layers"
    );
    assert_eq!(read_u32_le(&bytes, 36), 1, "atlas has one face");
    assert!(read_u32_le(&bytes, 40) > 1, "atlas includes a mip chain");
    assert_eq!(
        read_u32_le(&bytes, 44),
        KTX_SS_BASIS_LZ,
        "atlas carries a cross-platform Basis Universal payload"
    );
}

#[test]
fn tree_horizon_cards_placeholder_uses_its_immutable_loader_declaration() {
    let loader = AssetLoader::new();
    let declaration = loader
        .resolve_runtime_path("vegetation/tree_horizon_cards.ktx2")
        .expect("the predeclared runtime path resolves");

    assert_eq!(declaration.id, AssetId::TreeHorizonCards);
    assert_eq!(
        declaration.id.stable_id(),
        "moria.vegetation.tree_horizon_cards"
    );
    assert_eq!(
        loader.validation_fixture(AssetId::TreeHorizonCards).key,
        declaration.id.stable_id()
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::TreeHorizonCards, RuntimeAssetProfile::Development),
        AssetMissingAction::DevelopmentFallback {
            warning: "moria.vegetation.tree_horizon_cards",
        }
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::TreeHorizonCards, RuntimeAssetProfile::Release),
        AssetMissingAction::Fatal
    );
}
