use moria_world::presentation::{AssetId, AssetLoader, AssetMissingAction, RuntimeAssetProfile};

const TERRAIN_NORMAL_PATH: &str = "materials/terrain_normal.ktx2";
const KTX2_IDENTIFIER: [u8; 12] = [
    0xAB, b'K', b'T', b'X', b' ', b'2', b'0', 0xBB, b'\r', b'\n', 0x1A, b'\n',
];
const KTX2_HEADER_BYTES: usize = 80;
const KTX2_LEVEL_INDEX_BYTES: usize = 24;
const VK_FORMAT_UNDEFINED: u32 = 0;
const KTX2_NO_SUPERCOMPRESSION: u32 = 0;
const KHR_DF_MODEL_UASTC: u8 = 166;
const KHR_DF_TRANSFER_LINEAR: u8 = 1;

#[test]
fn terrain_normal_placeholder_uses_the_declared_loader_route_and_basis_ktx2_layout() {
    let loader = AssetLoader::new();
    let declaration = loader.declaration(AssetId::TerrainNormal);
    assert_eq!(declaration.id.stable_id(), "moria.materials.terrain_normal");
    assert_eq!(declaration.path, TERRAIN_NORMAL_PATH);
    assert_eq!(
        loader.resolve_runtime_path(TERRAIN_NORMAL_PATH),
        Ok(declaration)
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::TerrainNormal, RuntimeAssetProfile::Development),
        AssetMissingAction::DevelopmentFallback {
            warning: "moria.materials.terrain_normal",
        },
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::TerrainNormal, RuntimeAssetProfile::Release),
        AssetMissingAction::Fatal,
    );

    let bytes = include_bytes!("../../../../assets/materials/terrain_normal.ktx2");
    assert_eq!(bytes[..12], KTX2_IDENTIFIER);
    assert_eq!(
        u32_at(bytes, 12),
        VK_FORMAT_UNDEFINED,
        "Basis uses vkFormat 0"
    );
    assert_eq!(u32_at(bytes, 20), 1024);
    assert_eq!(u32_at(bytes, 24), 1024);
    assert_eq!(u32_at(bytes, 32), 14, "terrain material-array layers");
    assert_eq!(u32_at(bytes, 36), 1, "not cubemap faces");
    assert_eq!(u32_at(bytes, 40), 11, "complete 1024-to-1 mip chain");
    assert_eq!(
        u32_at(bytes, 44),
        KTX2_NO_SUPERCOMPRESSION,
        "UASTC Basis Universal payload"
    );

    let level_count = u32_at(bytes, 40) as usize;
    let level_index_end = KTX2_HEADER_BYTES + level_count * KTX2_LEVEL_INDEX_BYTES;
    assert!(bytes.len() >= level_index_end);
    for level in 0..level_count {
        let entry = KTX2_HEADER_BYTES + level * KTX2_LEVEL_INDEX_BYTES;
        let offset = u64_at(bytes, entry) as usize;
        let byte_length = u64_at(bytes, entry + 8) as usize;
        let uncompressed_length = u64_at(bytes, entry + 16);
        assert!(offset >= level_index_end);
        assert!(byte_length > 0);
        assert!(
            offset
                .checked_add(byte_length)
                .is_some_and(|end| end <= bytes.len())
        );
        assert!(uncompressed_length > 0);
    }

    let dfd_offset = u32_at(bytes, 48) as usize;
    let dfd_length = u32_at(bytes, 52) as usize;
    assert!(dfd_length > 0);
    assert!(
        dfd_offset
            .checked_add(dfd_length)
            .is_some_and(|end| end <= bytes.len())
    );
    assert_eq!(bytes[dfd_offset + 12], KHR_DF_MODEL_UASTC);
    assert_eq!(
        bytes[dfd_offset + 14],
        KHR_DF_TRANSFER_LINEAR,
        "normal maps use linear sampling"
    );
}

fn u32_at(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().expect("u32 field"))
}

fn u64_at(bytes: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(bytes[offset..offset + 8].try_into().expect("u64 field"))
}
