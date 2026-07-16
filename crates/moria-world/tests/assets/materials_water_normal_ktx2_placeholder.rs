use std::{fs, path::PathBuf};

use moria_world::presentation::{AssetId, AssetLoader, AssetMissingAction, RuntimeAssetProfile};

const WATER_NORMAL_PATH: &str = "materials/water_normal.ktx2";
const KTX2_IDENTIFIER: [u8; 12] = [
    0xAB, b'K', b'T', b'X', b' ', b'2', b'0', 0xBB, 0x0D, 0x0A, 0x1A, 0x0A,
];
const KTX2_HEADER_BYTES: usize = 80;
const KTX2_LEVEL_INDEX_BYTES: usize = 24;
const KHR_DF_MODEL_UASTC: u8 = 166;
const KHR_DF_TRANSFER_LINEAR: u8 = 1;

#[test]
fn water_normal_placeholder_is_a_linear_mipmapped_basis_ktx2_texture() {
    let bytes = fs::read(asset_path()).expect("water normal placeholder exists at its final path");

    assert!(bytes.starts_with(&KTX2_IDENTIFIER));
    assert_eq!(u32_at(&bytes, 12), 0, "Basis textures use vkFormat 0");
    assert_eq!(u32_at(&bytes, 16), 1, "Basis texture type size is one byte");
    assert!(u32_at(&bytes, 20) > 1, "texture has a nonzero width");
    assert!(u32_at(&bytes, 24) > 1, "texture has a nonzero height");
    assert_eq!(u32_at(&bytes, 28), 0, "texture is two-dimensional");
    assert_eq!(
        u32_at(&bytes, 32),
        0,
        "water normal is not an array texture"
    );
    assert_eq!(u32_at(&bytes, 36), 1, "texture has one face");
    assert!(u32_at(&bytes, 40) > 1, "texture includes a mip chain");
    assert_eq!(
        u32_at(&bytes, 44),
        0,
        "texture carries a UASTC Basis payload"
    );

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
    assert_eq!(
        bytes[dfd_offset + 14],
        KHR_DF_TRANSFER_LINEAR,
        "normal maps use linear sampling"
    );
}

#[test]
fn water_normal_placeholder_uses_its_immutable_loader_declaration() {
    let loader = AssetLoader::new();
    let declaration = loader
        .resolve_runtime_path(WATER_NORMAL_PATH)
        .expect("the predeclared water-normal runtime path resolves");

    assert_eq!(declaration.id, AssetId::WaterNormal);
    assert_eq!(declaration.id.stable_id(), "moria.materials.water_normal");
    assert_eq!(declaration.path, WATER_NORMAL_PATH);
    assert_eq!(
        loader.validation_fixture(AssetId::WaterNormal).key,
        declaration.id.stable_id()
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::WaterNormal, RuntimeAssetProfile::Development),
        AssetMissingAction::DevelopmentFallback {
            warning: "moria.materials.water_normal",
        }
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::WaterNormal, RuntimeAssetProfile::Release),
        AssetMissingAction::Fatal
    );
}

fn asset_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("assets")
        .join(WATER_NORMAL_PATH)
}

fn u32_at(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().expect("u32 field"))
}

fn u64_at(bytes: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(bytes[offset..offset + 8].try_into().expect("u64 field"))
}
