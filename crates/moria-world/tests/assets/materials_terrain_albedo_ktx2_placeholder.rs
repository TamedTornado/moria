use std::{fs, path::PathBuf};

use moria_world::presentation::{AssetId, AssetLoader, AssetMissingAction, RuntimeAssetProfile};

const KTX2_IDENTIFIER: [u8; 12] = [
    0xAB, b'K', b'T', b'X', b' ', b'2', b'0', 0xBB, 0x0D, 0x0A, 0x1A, 0x0A,
];
const KTX_SS_BASIS_LZ: u32 = 1;
const KTX_DF_TRANSFER_SRGB: u8 = 2;
const TERRAIN_LAYER_COUNT: u32 = 14;
const TERRAIN_LAYER_EDGE_PIXELS: u32 = 1024;
const TERRAIN_MIP_COUNT: u32 = 11;

fn read_u32_le(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
}

fn read_u64_le(bytes: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(bytes[offset..offset + 8].try_into().unwrap())
}

fn asset_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("assets/materials/terrain_albedo.ktx2")
}

#[test]
fn terrain_albedo_placeholder_is_a_mipmapped_srgb_basis_ktx2_array() {
    let bytes = fs::read(asset_path()).expect("terrain albedo placeholder exists");

    assert!(bytes.starts_with(&KTX2_IDENTIFIER));
    assert_eq!(read_u32_le(&bytes, 12), 0, "Basis textures use vkFormat 0");
    assert_eq!(
        read_u32_le(&bytes, 16),
        1,
        "Basis texture type size is one byte"
    );
    assert_eq!(read_u32_le(&bytes, 20), TERRAIN_LAYER_EDGE_PIXELS);
    assert_eq!(read_u32_le(&bytes, 24), TERRAIN_LAYER_EDGE_PIXELS);
    assert_eq!(read_u32_le(&bytes, 28), 0, "array is two-dimensional");
    assert_eq!(read_u32_le(&bytes, 32), TERRAIN_LAYER_COUNT);
    assert_eq!(read_u32_le(&bytes, 36), 1, "array has one face");
    assert_eq!(read_u32_le(&bytes, 40), TERRAIN_MIP_COUNT);
    assert_eq!(read_u32_le(&bytes, 44), KTX_SS_BASIS_LZ);

    let dfd_offset = read_u32_le(&bytes, 48) as usize;
    assert_eq!(
        bytes[dfd_offset + 14],
        KTX_DF_TRANSFER_SRGB,
        "albedo data-format descriptor uses sRGB transfer"
    );

    for mip in 0..TERRAIN_MIP_COUNT as usize {
        let index_offset = 80 + mip * 24;
        let byte_offset = read_u64_le(&bytes, index_offset) as usize;
        let byte_length = read_u64_le(&bytes, index_offset + 8) as usize;
        assert!(byte_length > 0, "mip {mip} contains array payload data");
        assert!(
            byte_offset
                .checked_add(byte_length)
                .is_some_and(|end| end <= bytes.len()),
            "mip {mip} payload remains inside the KTX2 file"
        );
    }
}

#[test]
fn terrain_albedo_placeholder_uses_its_immutable_loader_declaration() {
    let loader = AssetLoader::new();
    let declaration = loader
        .resolve_runtime_path("materials/terrain_albedo.ktx2")
        .expect("the predeclared runtime path resolves");

    assert_eq!(declaration.id, AssetId::TerrainAlbedo);
    assert_eq!(declaration.id.stable_id(), "moria.materials.terrain_albedo");
    assert_eq!(
        loader.validation_fixture(AssetId::TerrainAlbedo).key,
        declaration.id.stable_id()
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::TerrainAlbedo, RuntimeAssetProfile::Development),
        AssetMissingAction::DevelopmentFallback {
            warning: "moria.materials.terrain_albedo",
        }
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::TerrainAlbedo, RuntimeAssetProfile::Release),
        AssetMissingAction::Fatal
    );
}
