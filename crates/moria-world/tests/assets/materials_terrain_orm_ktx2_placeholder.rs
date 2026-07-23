use std::{fs, path::PathBuf};

use bevy::{
    asset::RenderAssetUsages,
    image::{CompressedImageFormats, Image, ImageSampler, ImageType},
};
use moria_world::presentation::{AssetId, AssetLoader, AssetMissingAction, RuntimeAssetProfile};

const KTX2_IDENTIFIER: [u8; 12] = [
    0xAB, b'K', b'T', b'X', b' ', b'2', b'0', 0xBB, 0x0D, 0x0A, 0x1A, 0x0A,
];
const KTX_SS_ZSTANDARD: u32 = 2;
const TERRAIN_WIDTH: u32 = 1024;
const TERRAIN_LAYER_COUNT: u32 = 14;
const TERRAIN_MIP_COUNT: u32 = 11;
const KTX2_HEADER_BYTES: usize = 80;
const KTX2_LEVEL_INDEX_BYTES: usize = 24;
const KHR_DF_MODEL_UASTC: u8 = 166;
const KHR_DF_TRANSFER_LINEAR: u8 = 1;

#[test]
fn terrain_orm_placeholder_is_a_linear_mipmapped_basis_ktx2_array() {
    let bytes = fs::read(asset_path()).expect("terrain ORM placeholder exists at its final path");

    for supported_formats in [
        CompressedImageFormats::ASTC_LDR,
        CompressedImageFormats::BC,
        CompressedImageFormats::ETC2,
    ] {
        let image = Image::from_buffer(
            &bytes,
            ImageType::Extension("ktx2"),
            supported_formats,
            false,
            ImageSampler::Default,
            RenderAssetUsages::default(),
        )
        .unwrap_or_else(|error| {
            panic!(
                "Bevy can parse and transcode the Basis Universal KTX2 payload for {supported_formats:?}: {error}"
            )
        });
        assert_eq!(image.width(), TERRAIN_WIDTH);
        assert_eq!(image.height(), TERRAIN_WIDTH);
        assert_eq!(
            image.texture_descriptor.size.depth_or_array_layers,
            TERRAIN_LAYER_COUNT
        );
        assert_eq!(image.texture_descriptor.mip_level_count, TERRAIN_MIP_COUNT);
        assert!(
            !image.texture_descriptor.format.is_srgb(),
            "ORM data must remain linear after transcoding"
        );
    }

    assert!(bytes.starts_with(&KTX2_IDENTIFIER));
    assert_eq!(read_u32_le(&bytes, 12), 0, "Basis textures use vkFormat 0");
    assert_eq!(
        read_u32_le(&bytes, 16),
        1,
        "Basis texture type size is one byte"
    );
    assert_eq!(read_u32_le(&bytes, 20), TERRAIN_WIDTH);
    assert_eq!(read_u32_le(&bytes, 24), TERRAIN_WIDTH);
    assert_eq!(read_u32_le(&bytes, 28), 0, "the texture is two-dimensional");
    assert_eq!(read_u32_le(&bytes, 32), TERRAIN_LAYER_COUNT);
    assert_eq!(
        read_u32_le(&bytes, 36),
        1,
        "the array has one face per layer"
    );
    assert_eq!(read_u32_le(&bytes, 40), TERRAIN_MIP_COUNT);
    assert_eq!(
        read_u32_le(&bytes, 44),
        KTX_SS_ZSTANDARD,
        "the UASTC payload uses KTX2 Zstandard supercompression"
    );

    assert!(
        bytes.len() >= KTX2_HEADER_BYTES + KTX2_LEVEL_INDEX_BYTES * TERRAIN_MIP_COUNT as usize,
        "every mip must have a KTX2 level-index entry"
    );
    for mip in 0..TERRAIN_MIP_COUNT as usize {
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
        bytes[dfd_offset + 12],
        KHR_DF_MODEL_UASTC,
        "the KTX2 data-format descriptor declares a Basis Universal UASTC payload"
    );
    assert_eq!(
        bytes[dfd_offset + 14],
        KHR_DF_TRANSFER_LINEAR,
        "ORM layers use the KTX2 linear transfer function"
    );
}

#[test]
fn terrain_orm_placeholder_uses_the_immutable_shared_loader_path_and_fallback() {
    let loader = AssetLoader::new();
    let declaration = loader
        .resolve_runtime_path("materials/terrain_orm.ktx2")
        .expect("the predeclared terrain ORM path resolves");

    assert_eq!(declaration.id, AssetId::TerrainOrm);
    assert_eq!(declaration.id.stable_id(), "moria.materials.terrain_orm");
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
}

fn asset_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("assets/materials/terrain_orm.ktx2")
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
