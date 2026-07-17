use std::{fs, path::PathBuf};

use moria_world::presentation::{AssetId, AssetLoader, AssetMissingAction, RuntimeAssetProfile};

const KTX2_IDENTIFIER: [u8; 12] = [
    0xAB, b'K', b'T', b'X', b' ', b'2', b'0', 0xBB, 0x0D, 0x0A, 0x1A, 0x0A,
];
const KTX_SS_BASIS_LZ: u32 = 1;
const WATER_NORMAL_WIDTH: u32 = 64;
const WATER_NORMAL_MIP_COUNT: u32 = 7;
const KTX2_HEADER_BYTES: usize = 80;
const KTX2_LEVEL_INDEX_BYTES: usize = 24;
const KHR_DF_TRANSFER_LINEAR: u8 = 1;

#[test]
fn water_normal_placeholder_is_a_linear_mipmapped_basis_ktx2_texture() {
    let bytes = fs::read(asset_path()).expect("water normal placeholder exists at its final path");

    assert!(bytes.starts_with(&KTX2_IDENTIFIER));
    assert_eq!(read_u32_le(&bytes, 12), 0, "Basis textures use vkFormat 0");
    assert_eq!(
        read_u32_le(&bytes, 16),
        1,
        "Basis texture type size is one byte"
    );
    assert_eq!(read_u32_le(&bytes, 20), WATER_NORMAL_WIDTH);
    assert_eq!(read_u32_le(&bytes, 24), WATER_NORMAL_WIDTH);
    assert_eq!(read_u32_le(&bytes, 28), 0, "the texture is two-dimensional");
    assert_eq!(read_u32_le(&bytes, 32), 0, "the texture is not an array");
    assert_eq!(read_u32_le(&bytes, 36), 1, "the texture has one face");
    assert_eq!(read_u32_le(&bytes, 40), WATER_NORMAL_MIP_COUNT);
    assert_eq!(read_u32_le(&bytes, 44), KTX_SS_BASIS_LZ);

    assert!(
        bytes.len() >= KTX2_HEADER_BYTES + KTX2_LEVEL_INDEX_BYTES * WATER_NORMAL_MIP_COUNT as usize,
        "every mip has a KTX2 level-index entry"
    );
    for mip in 0..WATER_NORMAL_MIP_COUNT as usize {
        let index_offset = KTX2_HEADER_BYTES + mip * KTX2_LEVEL_INDEX_BYTES;
        let byte_offset = read_u64_le(&bytes, index_offset) as usize;
        let byte_length = read_u64_le(&bytes, index_offset + 8) as usize;
        assert!(byte_length > 0, "mip {mip} has encoded Basis data");
        assert!(
            byte_offset
                .checked_add(byte_length)
                .is_some_and(|end| end <= bytes.len()),
            "mip {mip} payload lies within the KTX2 file"
        );
    }

    let dfd_offset = read_u32_le(&bytes, 48) as usize;
    let dfd_length = read_u32_le(&bytes, 52) as usize;
    assert!(
        dfd_length >= 20,
        "the KTX2 data-format descriptor is present"
    );
    assert!(
        dfd_offset
            .checked_add(dfd_length)
            .is_some_and(|end| end <= bytes.len()),
        "the data-format descriptor lies within the KTX2 file"
    );
    assert_eq!(
        bytes[dfd_offset + 14],
        KHR_DF_TRANSFER_LINEAR,
        "water normals use linear sampling"
    );
}

#[test]
fn water_normal_placeholder_uses_the_immutable_shared_loader_path_and_fallback() {
    let loader = AssetLoader::new();
    let declaration = loader
        .resolve_runtime_path("materials/water_normal.ktx2")
        .expect("the predeclared water normal path resolves");

    assert_eq!(declaration.id, AssetId::WaterNormal);
    assert_eq!(declaration.id.stable_id(), "moria.materials.water_normal");
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
        .join("assets/materials/water_normal.ktx2")
}

fn read_u32_le(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(
        bytes[offset..offset + 4]
            .try_into()
            .expect("four-byte field"),
    )
}

fn read_u64_le(bytes: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(
        bytes[offset..offset + 8]
            .try_into()
            .expect("eight-byte field"),
    )
}
