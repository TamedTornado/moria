use std::{fs, path::PathBuf};

use basisu::{DecodeFlags, TargetFormat, Transcoder};
use moria_world::presentation::{AssetId, AssetLoader, AssetMissingAction, RuntimeAssetProfile};

const KTX2_IDENTIFIER: [u8; 12] = [
    0xAB, b'K', b'T', b'X', b' ', b'2', b'0', 0xBB, 0x0D, 0x0A, 0x1A, 0x0A,
];
const KTX_SS_BASIS_LZ: u32 = 1;
const TERRAIN_WIDTH: u32 = 1024;
const TERRAIN_LAYER_COUNT: u32 = 14;
const TERRAIN_MIP_COUNT: u32 = 11;
const KTX2_HEADER_BYTES: usize = 80;
const KTX2_LEVEL_INDEX_BYTES: usize = 24;
const ETC1S_GLOBAL_HEADER_BYTES: usize = 20;
const ETC1S_IMAGE_DESCRIPTOR_BYTES: usize = 20;
const KHR_DF_MODEL_ETC1S: u8 = 163;
const KHR_DF_TRANSFER_LINEAR: u8 = 1;

#[test]
fn terrain_normal_placeholder_is_a_linear_mipmapped_basis_ktx2_array() {
    let bytes =
        fs::read(asset_path()).expect("terrain normal placeholder exists at its final path");

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
    assert_eq!(read_u32_le(&bytes, 44), KTX_SS_BASIS_LZ);

    assert!(
        bytes.len() >= KTX2_HEADER_BYTES + KTX2_LEVEL_INDEX_BYTES * TERRAIN_MIP_COUNT as usize,
        "every mip must have a KTX2 level-index entry"
    );
    let mut level_lengths = Vec::with_capacity(TERRAIN_MIP_COUNT as usize);
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
        level_lengths.push(byte_length);
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
        KHR_DF_MODEL_ETC1S,
        "the descriptor identifies a Basis Universal ETC1S payload"
    );
    assert_eq!(
        bytes[dfd_offset + 14],
        KHR_DF_TRANSFER_LINEAR,
        "normal layers use the KTX2 linear transfer function"
    );

    let sgd_offset = read_u64_le(&bytes, 64) as usize;
    let sgd_length = read_u64_le(&bytes, 72) as usize;
    let descriptor_count = TERRAIN_LAYER_COUNT as usize * TERRAIN_MIP_COUNT as usize;
    let descriptors_bytes = descriptor_count * ETC1S_IMAGE_DESCRIPTOR_BYTES;
    assert!(
        sgd_offset
            .checked_add(sgd_length)
            .is_some_and(|end| end <= bytes.len()),
        "BasisLZ supercompression global data lies within the KTX2 file"
    );
    assert!(
        sgd_length > ETC1S_GLOBAL_HEADER_BYTES + descriptors_bytes,
        "BasisLZ global data contains every layer/mip descriptor and shared codebooks"
    );

    let endpoint_count = read_u16_le(&bytes, sgd_offset);
    let selector_count = read_u16_le(&bytes, sgd_offset + 2);
    let endpoints_length = read_u32_le(&bytes, sgd_offset + 4) as usize;
    let selectors_length = read_u32_le(&bytes, sgd_offset + 8) as usize;
    let tables_length = read_u32_le(&bytes, sgd_offset + 12) as usize;
    let extended_length = read_u32_le(&bytes, sgd_offset + 16) as usize;
    assert!(endpoint_count > 0 && selector_count > 0);
    assert!(endpoints_length > 0 && selectors_length > 0 && tables_length > 0);
    assert_eq!(
        sgd_length,
        ETC1S_GLOBAL_HEADER_BYTES
            + descriptors_bytes
            + endpoints_length
            + selectors_length
            + tables_length
            + extended_length,
        "BasisLZ global-data sections exactly fill the declared range"
    );

    for (mip, &level_length) in level_lengths.iter().enumerate() {
        for layer in 0..TERRAIN_LAYER_COUNT as usize {
            let descriptor_offset = sgd_offset
                + ETC1S_GLOBAL_HEADER_BYTES
                + (mip * TERRAIN_LAYER_COUNT as usize + layer) * ETC1S_IMAGE_DESCRIPTOR_BYTES;
            let rgb_offset = read_u32_le(&bytes, descriptor_offset + 4) as usize;
            let rgb_length = read_u32_le(&bytes, descriptor_offset + 8) as usize;
            let alpha_length = read_u32_le(&bytes, descriptor_offset + 16);
            assert!(rgb_length > 0, "mip {mip} layer {layer} has Basis data");
            assert!(
                rgb_offset
                    .checked_add(rgb_length)
                    .is_some_and(|end| end <= level_length),
                "mip {mip} layer {layer} Basis slice lies within its mip payload"
            );
            assert_eq!(alpha_length, 0, "normal layers do not carry alpha slices");
        }
    }
}

#[test]
fn terrain_normal_basis_payload_decodes_every_array_layer_in_the_portable_path() {
    let bytes =
        fs::read(asset_path()).expect("terrain normal placeholder exists at its final path");
    let texture = Transcoder::new(&bytes).expect("the KTX2 carries decodable Basis data");

    assert_eq!(texture.base_dimensions(), (TERRAIN_WIDTH, TERRAIN_WIDTH));
    assert_eq!(texture.layer_count(), TERRAIN_LAYER_COUNT);
    assert_eq!(texture.level_count(), TERRAIN_MIP_COUNT);

    for layer in 0..TERRAIN_LAYER_COUNT {
        for mip in [0, TERRAIN_MIP_COUNT - 1] {
            let pixels = texture
                .transcode_image(mip, layer, 0, TargetFormat::Rgba32, DecodeFlags::NONE)
                .unwrap_or_else(|error| panic!("layer {layer} mip {mip} transcodes: {error:?}"));
            assert_eq!(
                pixels.len(),
                texture
                    .output_size(mip, TargetFormat::Rgba32)
                    .expect("mip has a portable output size"),
                "layer {layer} mip {mip} retains the cross-array layout"
            );
            if layer < 2 {
                assert_neutral_normal(&pixels, layer, mip);
            }
        }
    }
}

#[test]
fn terrain_normal_placeholder_uses_the_immutable_shared_loader_path_and_fallback() {
    let loader = AssetLoader::new();
    let declaration = loader
        .resolve_runtime_path("materials/terrain_normal.ktx2")
        .expect("the predeclared terrain normal path resolves");

    assert_eq!(declaration.id, AssetId::TerrainNormal);
    assert_eq!(declaration.id.stable_id(), "moria.materials.terrain_normal");
    assert_eq!(
        loader.validation_fixture(AssetId::TerrainNormal).key,
        declaration.id.stable_id()
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::TerrainNormal, RuntimeAssetProfile::Development),
        AssetMissingAction::DevelopmentFallback {
            warning: "moria.materials.terrain_normal",
        }
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::TerrainNormal, RuntimeAssetProfile::Release),
        AssetMissingAction::Fatal
    );
}

fn asset_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("assets/materials/terrain_normal.ktx2")
}

fn read_u32_le(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(
        bytes[offset..offset + 4]
            .try_into()
            .expect("four-byte field"),
    )
}

fn read_u16_le(bytes: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes(
        bytes[offset..offset + 2]
            .try_into()
            .expect("two-byte field"),
    )
}

fn read_u64_le(bytes: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(
        bytes[offset..offset + 8]
            .try_into()
            .expect("eight-byte field"),
    )
}

fn assert_neutral_normal(pixels: &[u8], layer: u32, mip: u32) {
    let first = pixels
        .chunks_exact(4)
        .next()
        .expect("normal mip has at least one pixel");
    assert!(
        pixels.chunks_exact(4).all(|pixel| pixel == first),
        "reserved layer {layer} mip {mip} is spatially neutral"
    );
    assert!(
        (i16::from(first[0]) - 128).abs() <= 2
            && (i16::from(first[1]) - 128).abs() <= 2
            && first[2] >= 253
            && first[3] == 255,
        "reserved layer {layer} mip {mip} encodes a neutral tangent-space normal"
    );
}
