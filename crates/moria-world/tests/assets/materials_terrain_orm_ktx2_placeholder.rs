use std::{fs, path::PathBuf};

use moria_world::presentation::{AssetId, AssetLoader, AssetMissingAction, RuntimeAssetProfile};

const TERRAIN_ORM_PATH: &str = "materials/terrain_orm.ktx2";
const KTX2_IDENTIFIER: [u8; 12] = [
    0xAB, b'K', b'T', b'X', b' ', b'2', b'0', 0xBB, 0x0D, 0x0A, 0x1A, 0x0A,
];
const KTX2_HEADER_BYTES: usize = 80;
const KTX2_LEVEL_INDEX_BYTES: usize = 24;
const KHR_DF_MODEL_UASTC: u8 = 166;
const KHR_DF_TRANSFER_LINEAR: u8 = 1;
const TERRAIN_LAYER_COUNT: u32 = 14;
const TERRAIN_DIMENSION_PIXELS: u32 = 1024;
const TERRAIN_MIP_COUNT: u32 = 11;

#[test]
fn terrain_orm_placeholder_uses_the_declared_loader_route_and_linear_basis_ktx2_layout() {
    let loader = AssetLoader::new();
    let declaration = loader
        .resolve_runtime_path(TERRAIN_ORM_PATH)
        .expect("the predeclared terrain-ORM runtime path resolves");

    assert_eq!(declaration.id, AssetId::TerrainOrm);
    assert_eq!(declaration.id.stable_id(), "moria.materials.terrain_orm");
    assert_eq!(declaration.path, TERRAIN_ORM_PATH);
    assert_eq!(
        loader.validation_fixture(AssetId::TerrainOrm).key,
        declaration.id.stable_id()
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::TerrainOrm, RuntimeAssetProfile::Development),
        AssetMissingAction::DevelopmentFallback {
            warning: "moria.materials.terrain_orm",
        }
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::TerrainOrm, RuntimeAssetProfile::Release),
        AssetMissingAction::Fatal
    );

    let bytes = fs::read(asset_path()).expect("terrain ORM placeholder exists at its final path");
    assert!(bytes.starts_with(&KTX2_IDENTIFIER));
    assert_eq!(u32_at(&bytes, 12), 0, "Basis textures use vkFormat 0");
    assert_eq!(u32_at(&bytes, 16), 1, "Basis texture type size is one byte");
    assert_eq!(u32_at(&bytes, 20), TERRAIN_DIMENSION_PIXELS);
    assert_eq!(u32_at(&bytes, 24), TERRAIN_DIMENSION_PIXELS);
    assert_eq!(u32_at(&bytes, 28), 0, "array is two-dimensional");
    assert_eq!(u32_at(&bytes, 32), TERRAIN_LAYER_COUNT);
    assert_eq!(u32_at(&bytes, 36), 1, "array has one face");
    assert_eq!(
        u32_at(&bytes, 40),
        TERRAIN_MIP_COUNT,
        "complete 1024-to-1 mip chain"
    );
    assert_eq!(u32_at(&bytes, 44), 0, "UASTC Basis Universal payload");

    let level_count = u32_at(&bytes, 40) as usize;
    let level_index_end = KTX2_HEADER_BYTES + level_count * KTX2_LEVEL_INDEX_BYTES;
    assert!(bytes.len() >= level_index_end);
    for level in 0..level_count {
        let entry = KTX2_HEADER_BYTES + level * KTX2_LEVEL_INDEX_BYTES;
        let offset = u64_at(&bytes, entry) as usize;
        let byte_length = u64_at(&bytes, entry + 8) as usize;
        assert!(offset >= level_index_end);
        assert!(byte_length > 0);
        assert!(
            offset
                .checked_add(byte_length)
                .is_some_and(|end| end <= bytes.len())
        );
    }

    let dfd_offset = u32_at(&bytes, 48) as usize;
    assert_eq!(bytes[dfd_offset + 12], KHR_DF_MODEL_UASTC);
    assert_eq!(bytes[dfd_offset + 14], KHR_DF_TRANSFER_LINEAR);
}

fn asset_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("assets")
        .join(TERRAIN_ORM_PATH)
}

fn u32_at(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().expect("u32 field"))
}

fn u64_at(bytes: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(bytes[offset..offset + 8].try_into().expect("u64 field"))
}
