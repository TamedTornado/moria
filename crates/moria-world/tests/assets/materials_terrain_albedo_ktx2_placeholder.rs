use std::{collections::BTreeSet, fs, path::PathBuf, ptr};

use moria_world::presentation::{
    AssetId, AssetLoadPolicy, AssetLoader, AssetMissingAction, RuntimeAssetProfile,
};

const TERRAIN_ALBEDO_PATH: &str = "materials/terrain_albedo.ktx2";
const TERRAIN_ALBEDO_STABLE_ID: &str = "moria.materials.terrain_albedo";
const KTX2_IDENTIFIER: [u8; 12] = [
    0xAB, b'K', b'T', b'X', b' ', b'2', b'0', 0xBB, 0x0D, 0x0A, 0x1A, 0x0A,
];
const KTX_SS_BASIS_LZ: u32 = 1;
const KHR_DF_MODEL_ETC1S: u8 = 163;
const KHR_DF_PRIMARIES_BT709: u8 = 1;
const KHR_DF_TRANSFER_SRGB: u8 = 2;
const TERRAIN_WIDTH: u32 = 1024;
const TERRAIN_LAYER_COUNT: u32 = 14;
const TERRAIN_MIP_COUNT: u32 = TERRAIN_WIDTH.ilog2() + 1;
const AIR_LAYER: usize = 0;
const WATER_LAYER: usize = 1;
const KTX2_HEADER_BYTES: usize = 80;
const KTX2_LEVEL_INDEX_BYTES: usize = 24;
const ETC1S_GLOBAL_DATA_HEADER_BYTES: usize = 20;
const ETC1S_IMAGE_DESC_BYTES: usize = 20;

#[derive(Clone, Copy, Debug)]
struct Level {
    offset: usize,
    length: usize,
}

#[derive(Clone, Copy, Debug)]
struct ImageDesc {
    flags: u32,
    rgb_offset: usize,
    rgb_length: usize,
    alpha_offset: usize,
    alpha_length: usize,
}

#[derive(Debug)]
struct BasisKtx2<'a> {
    bytes: &'a [u8],
    levels: Vec<Level>,
    images: Vec<ImageDesc>,
    has_alpha: bool,
}

#[test]
fn terrain_albedo_placeholder_is_an_srgb_mipmapped_basis_ktx2_array() {
    let bytes =
        fs::read(asset_path()).expect("terrain albedo placeholder exists at its final path");
    let texture = BasisKtx2::parse(&bytes).unwrap_or_else(|error| {
        panic!("terrain albedo is not a valid Basis Universal KTX2: {error}")
    });

    assert_eq!(
        texture.levels.len(),
        TERRAIN_MIP_COUNT as usize,
        "a 1024-square texture has a complete 1024..1 mip chain"
    );
    assert_eq!(
        texture.images.len(),
        TERRAIN_MIP_COUNT as usize * TERRAIN_LAYER_COUNT as usize,
        "every one of the 14 authored layers has an ETC1S image at every mip"
    );

    for mip in 0..TERRAIN_MIP_COUNT as usize {
        let expected_edge = (TERRAIN_WIDTH >> mip).max(1);
        assert_eq!(
            expected_edge,
            1_u32 << (TERRAIN_MIP_COUNT as usize - 1 - mip),
            "mip {mip} has the expected square authored extent"
        );

        let air = texture.image(mip, AIR_LAYER);
        let water = texture.image(mip, WATER_LAYER);
        assert_eq!(
            texture.rgb_slice(mip, air),
            texture.rgb_slice(mip, water),
            "unused canonical air/water slots are the same neutral albedo at mip {mip}"
        );
        if texture.has_alpha {
            assert_eq!(
                texture.alpha_slice(mip, air),
                texture.alpha_slice(mip, water),
                "unused canonical air/water slots have the same neutral alpha at mip {mip}"
            );
        }
    }

    let authored_base_slices = (WATER_LAYER + 1..TERRAIN_LAYER_COUNT as usize)
        .map(|layer| texture.rgb_slice(0, texture.image(0, layer)))
        .collect::<BTreeSet<_>>();
    assert!(
        authored_base_slices.len() > 1,
        "the twelve solid-material slots must contain authored albedo variation"
    );
}

#[test]
fn terrain_albedo_placeholder_uses_the_immutable_shared_loader_path_and_fallback() {
    let loader = AssetLoader::new();
    let by_id = loader.declaration(AssetId::TerrainAlbedo);
    let by_path = loader
        .resolve_runtime_path(TERRAIN_ALBEDO_PATH)
        .expect("the exact predeclared terrain albedo path resolves");
    let repeated = loader
        .resolve_runtime_path(TERRAIN_ALBEDO_PATH)
        .expect("repeated loads resolve through the same declaration");

    assert!(
        ptr::eq(by_id, by_path) && ptr::eq(by_path, repeated),
        "ID and repeated path loads reuse the immutable shared declaration"
    );
    assert_eq!(by_path.id, AssetId::TerrainAlbedo);
    assert_eq!(by_path.path, TERRAIN_ALBEDO_PATH);
    assert_eq!(by_path.id.stable_id(), TERRAIN_ALBEDO_STABLE_ID);
    assert_eq!(by_path.load_policy, AssetLoadPolicy::DevelopmentFallback);
    assert_eq!(
        loader.validation_fixture(AssetId::TerrainAlbedo),
        by_path.validation_fixture
    );
    assert_eq!(by_path.validation_fixture.asset_id, AssetId::TerrainAlbedo);
    assert_eq!(by_path.validation_fixture.key, TERRAIN_ALBEDO_STABLE_ID);
    assert_eq!(
        loader.missing_asset_action(AssetId::TerrainAlbedo, RuntimeAssetProfile::Development),
        AssetMissingAction::DevelopmentFallback {
            warning: TERRAIN_ALBEDO_STABLE_ID,
        },
        "development fallback is observable and reports the exact stable ID"
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::TerrainAlbedo, RuntimeAssetProfile::Release),
        AssetMissingAction::Fatal,
        "release cannot silently substitute the development fallback"
    );
}

impl<'a> BasisKtx2<'a> {
    fn parse(bytes: &'a [u8]) -> Result<Self, String> {
        let header = checked_slice(bytes, 0, KTX2_HEADER_BYTES, "KTX2 header")?;
        require(
            header[..KTX2_IDENTIFIER.len()] == KTX2_IDENTIFIER,
            "missing KTX2 identifier",
        )?;
        require(read_u32(header, 12)? == 0, "Basis KTX2 vkFormat must be 0")?;
        require(read_u32(header, 16)? == 1, "Basis KTX2 typeSize must be 1")?;
        require(
            read_u32(header, 20)? == TERRAIN_WIDTH && read_u32(header, 24)? == TERRAIN_WIDTH,
            "terrain albedo must be authored at 1024x1024",
        )?;
        require(read_u32(header, 28)? == 0, "terrain albedo must be 2D")?;
        require(
            read_u32(header, 32)? == TERRAIN_LAYER_COUNT,
            "terrain albedo must have exactly 14 array layers",
        )?;
        require(
            read_u32(header, 36)? == 1,
            "terrain albedo must not be cubemapped",
        )?;
        require(
            read_u32(header, 40)? == TERRAIN_MIP_COUNT,
            "terrain albedo must have a complete 1024..1 mip chain",
        )?;
        require(
            read_u32(header, 44)? == KTX_SS_BASIS_LZ,
            "terrain albedo must use the cross-platform ETC1S BasisLZ payload",
        )?;

        let level_index_length = (TERRAIN_MIP_COUNT as usize)
            .checked_mul(KTX2_LEVEL_INDEX_BYTES)
            .ok_or_else(|| "level-index length overflow".to_owned())?;
        checked_slice(
            bytes,
            KTX2_HEADER_BYTES,
            level_index_length,
            "complete KTX2 level index",
        )?;

        let dfd_offset = usize_from_u32(read_u32(header, 48)?, "DFD offset")?;
        let dfd_length = usize_from_u32(read_u32(header, 52)?, "DFD length")?;
        let kvd_offset = usize_from_u32(read_u32(header, 56)?, "KVD offset")?;
        let kvd_length = usize_from_u32(read_u32(header, 60)?, "KVD length")?;
        let sgd_offset = usize_from_u64(read_u64(header, 64)?, "SGD offset")?;
        let sgd_length = usize_from_u64(read_u64(header, 72)?, "SGD length")?;
        require(
            sgd_offset % 8 == 0,
            "BasisLZ sgdByteOffset must be 8-byte aligned",
        )?;

        validate_dfd(bytes, dfd_offset, dfd_length)?;
        if kvd_length == 0 {
            require(kvd_offset == 0, "an empty KVD must have offset zero")?;
        } else {
            checked_slice(bytes, kvd_offset, kvd_length, "KTX2 key/value data")?;
        }

        let levels = parse_levels(bytes)?;
        validate_non_overlapping_levels(&levels)?;
        let has_alpha = dfd_length == 60;
        let images = parse_etc1s_global_data(bytes, sgd_offset, sgd_length, &levels, has_alpha)?;

        Ok(Self {
            bytes,
            levels,
            images,
            has_alpha,
        })
    }

    fn image(&self, mip: usize, layer: usize) -> ImageDesc {
        self.images[mip * TERRAIN_LAYER_COUNT as usize + layer]
    }

    fn rgb_slice(&self, mip: usize, image: ImageDesc) -> &'a [u8] {
        let level = self.levels[mip];
        &self.bytes
            [level.offset + image.rgb_offset..level.offset + image.rgb_offset + image.rgb_length]
    }

    fn alpha_slice(&self, mip: usize, image: ImageDesc) -> &'a [u8] {
        let level = self.levels[mip];
        &self.bytes[level.offset + image.alpha_offset
            ..level.offset + image.alpha_offset + image.alpha_length]
    }
}

fn validate_dfd(bytes: &[u8], offset: usize, length: usize) -> Result<(), String> {
    require(
        matches!(length, 44 | 60),
        "ETC1S DFD must contain exactly one RGB sample and optional alpha sample",
    )?;
    let dfd = checked_slice(bytes, offset, length, "data-format descriptor")?;
    require(
        usize_from_u32(read_u32(dfd, 0)?, "DFD total size")? == length,
        "DFD total size must match its header index",
    )?;
    require(
        read_u32(dfd, 4)? == 0,
        "DFD must use the Khronos basic descriptor",
    )?;
    require(
        read_u16(dfd, 8)? == 2 && usize::from(read_u16(dfd, 10)?) == length - 4,
        "DFD must be a complete version-2 basic descriptor block",
    )?;
    require(
        dfd[12] == KHR_DF_MODEL_ETC1S,
        "DFD color model must be ETC1S",
    )?;
    require(
        dfd[13] == KHR_DF_PRIMARIES_BT709,
        "albedo DFD must declare BT.709/sRGB primaries",
    )?;
    require(
        dfd[14] == KHR_DF_TRANSFER_SRGB,
        "albedo DFD must declare the sRGB transfer function",
    )?;
    require(
        dfd[15] == 0,
        "terrain albedo alpha must not be premultiplied",
    )?;
    require(
        dfd[16..20] == [3, 3, 0, 0],
        "ETC1S DFD must declare 4x4x1x1 texel blocks",
    )?;
    require(
        dfd[28 + 3] & 0x0f == 0,
        "the first ETC1S sample must contain RGB",
    )?;
    if length == 60 {
        require(
            dfd[44 + 3] & 0x0f == 15,
            "the second ETC1S sample must contain alpha",
        )?;
    }
    Ok(())
}

fn parse_levels(bytes: &[u8]) -> Result<Vec<Level>, String> {
    let mut levels = Vec::with_capacity(TERRAIN_MIP_COUNT as usize);
    for mip in 0..TERRAIN_MIP_COUNT as usize {
        let index = KTX2_HEADER_BYTES + mip * KTX2_LEVEL_INDEX_BYTES;
        let offset = usize_from_u64(read_u64(bytes, index)?, "level byte offset")?;
        let length = usize_from_u64(read_u64(bytes, index + 8)?, "level byte length")?;
        let uncompressed = read_u64(bytes, index + 16)?;
        require(length > 0, format!("mip {mip} must contain Basis data"))?;
        require(
            uncompressed == 0,
            format!("BasisLZ mip {mip} uncompressedByteLength must be zero"),
        )?;
        checked_slice(bytes, offset, length, &format!("mip {mip} payload"))?;
        levels.push(Level { offset, length });
    }
    Ok(levels)
}

fn validate_non_overlapping_levels(levels: &[Level]) -> Result<(), String> {
    for (mip, pair) in levels.windows(2).enumerate() {
        require(
            pair[0].offset > pair[1].offset,
            format!(
                "KTX2 mip payload ordering must be smallest-to-largest (mip {mip} must follow mip {})",
                mip + 1
            ),
        )?;
    }

    let mut ranges = levels
        .iter()
        .enumerate()
        .map(|(mip, level)| {
            let end = level
                .offset
                .checked_add(level.length)
                .ok_or_else(|| format!("mip {mip} range overflow"))?;
            Ok((level.offset, end, mip))
        })
        .collect::<Result<Vec<_>, String>>()?;
    ranges.sort_unstable();
    for pair in ranges.windows(2) {
        require(
            pair[0].1 <= pair[1].0,
            format!("mip {} overlaps mip {}", pair[0].2, pair[1].2),
        )?;
    }
    Ok(())
}

fn parse_etc1s_global_data(
    bytes: &[u8],
    offset: usize,
    length: usize,
    levels: &[Level],
    has_alpha: bool,
) -> Result<Vec<ImageDesc>, String> {
    let sgd = checked_slice(
        bytes,
        offset,
        length,
        "BasisLZ supercompression global data",
    )?;
    let image_count = (TERRAIN_MIP_COUNT as usize)
        .checked_mul(TERRAIN_LAYER_COUNT as usize)
        .ok_or_else(|| "ETC1S image count overflow".to_owned())?;
    let descriptor_bytes = image_count
        .checked_mul(ETC1S_IMAGE_DESC_BYTES)
        .ok_or_else(|| "ETC1S descriptor size overflow".to_owned())?;
    let fixed_bytes = ETC1S_GLOBAL_DATA_HEADER_BYTES
        .checked_add(descriptor_bytes)
        .ok_or_else(|| "ETC1S global-data size overflow".to_owned())?;
    require(
        sgd.len() >= fixed_bytes,
        format!(
            "BasisLZ SGD needs {image_count} image descriptors ({fixed_bytes} bytes before codebooks)"
        ),
    )?;

    require(
        read_u16(sgd, 0)? > 0 && read_u16(sgd, 2)? > 0,
        "BasisLZ endpoint and selector counts must be nonzero",
    )?;
    let endpoint_bytes = usize_from_u32(read_u32(sgd, 4)?, "endpoint byte length")?;
    let selector_bytes = usize_from_u32(read_u32(sgd, 8)?, "selector byte length")?;
    let table_bytes = usize_from_u32(read_u32(sgd, 12)?, "table byte length")?;
    let extended_bytes = usize_from_u32(read_u32(sgd, 16)?, "extended byte length")?;
    require(
        endpoint_bytes > 0 && selector_bytes > 0 && table_bytes > 0,
        "BasisLZ endpoint, selector, and Huffman-table payloads must be nonempty",
    )?;
    let expected_length = [
        fixed_bytes,
        endpoint_bytes,
        selector_bytes,
        table_bytes,
        extended_bytes,
    ]
    .into_iter()
    .try_fold(0_usize, |sum, value| sum.checked_add(value))
    .ok_or_else(|| "BasisLZ SGD length overflow".to_owned())?;
    require(
        expected_length == length,
        "BasisLZ SGD must contain exactly its descriptors and codebooks",
    )?;

    let mut images = Vec::with_capacity(image_count);
    for image_index in 0..image_count {
        let descriptor_offset =
            ETC1S_GLOBAL_DATA_HEADER_BYTES + image_index * ETC1S_IMAGE_DESC_BYTES;
        let descriptor = checked_slice(
            sgd,
            descriptor_offset,
            ETC1S_IMAGE_DESC_BYTES,
            "ETC1S image descriptor",
        )?;
        let mip = image_index / TERRAIN_LAYER_COUNT as usize;
        let layer = image_index % TERRAIN_LAYER_COUNT as usize;
        let image = ImageDesc {
            flags: read_u32(descriptor, 0)?,
            rgb_offset: usize_from_u32(read_u32(descriptor, 4)?, "RGB slice offset")?,
            rgb_length: usize_from_u32(read_u32(descriptor, 8)?, "RGB slice length")?,
            alpha_offset: usize_from_u32(read_u32(descriptor, 12)?, "alpha slice offset")?,
            alpha_length: usize_from_u32(read_u32(descriptor, 16)?, "alpha slice length")?,
        };
        require(
            image.flags == 0,
            format!("static terrain mip {mip} layer {layer} cannot be a video P-frame"),
        )?;
        validate_slice(
            levels[mip],
            image.rgb_offset,
            image.rgb_length,
            &format!("mip {mip} layer {layer} RGB"),
        )?;
        if has_alpha {
            validate_slice(
                levels[mip],
                image.alpha_offset,
                image.alpha_length,
                &format!("mip {mip} layer {layer} alpha"),
            )?;
        } else {
            require(
                image.alpha_offset == 0 && image.alpha_length == 0,
                format!("RGB-only mip {mip} layer {layer} cannot reference alpha data"),
            )?;
        }
        images.push(image);
    }
    Ok(images)
}

fn validate_slice(level: Level, offset: usize, length: usize, label: &str) -> Result<(), String> {
    require(length > 0, format!("{label} slice must be nonempty"))?;
    require(
        offset
            .checked_add(length)
            .is_some_and(|end| end <= level.length),
        format!("{label} slice must lie inside its mip payload"),
    )
}

fn checked_slice<'a>(
    bytes: &'a [u8],
    offset: usize,
    length: usize,
    label: &str,
) -> Result<&'a [u8], String> {
    let end = offset
        .checked_add(length)
        .ok_or_else(|| format!("{label} range overflow"))?;
    bytes
        .get(offset..end)
        .ok_or_else(|| format!("{label} lies outside the file"))
}

fn read_u16(bytes: &[u8], offset: usize) -> Result<u16, String> {
    let field = checked_slice(bytes, offset, 2, "two-byte field")?;
    Ok(u16::from_le_bytes([field[0], field[1]]))
}

fn read_u32(bytes: &[u8], offset: usize) -> Result<u32, String> {
    let field = checked_slice(bytes, offset, 4, "four-byte field")?;
    Ok(u32::from_le_bytes(
        field
            .try_into()
            .map_err(|_| "invalid four-byte field".to_owned())?,
    ))
}

fn read_u64(bytes: &[u8], offset: usize) -> Result<u64, String> {
    let field = checked_slice(bytes, offset, 8, "eight-byte field")?;
    Ok(u64::from_le_bytes(
        field
            .try_into()
            .map_err(|_| "invalid eight-byte field".to_owned())?,
    ))
}

fn usize_from_u32(value: u32, label: &str) -> Result<usize, String> {
    usize::try_from(value).map_err(|_| format!("{label} does not fit in memory address space"))
}

fn usize_from_u64(value: u64, label: &str) -> Result<usize, String> {
    usize::try_from(value).map_err(|_| format!("{label} does not fit in memory address space"))
}

fn require(condition: bool, message: impl Into<String>) -> Result<(), String> {
    condition.then_some(()).ok_or_else(|| message.into())
}

fn asset_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("assets")
        .join(TERRAIN_ALBEDO_PATH)
}
