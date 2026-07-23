//! Bevy asset loading for BasisLZ KTX2 textures.
//!
//! Bevy 0.19's built-in KTX2 loader intentionally does not transcode ETC1S
//! BasisLZ payloads. Product One's terrain normal asset uses that portable
//! representation, so it is expanded to a linear RGBA image before Bevy
//! installs it as an asset.

use std::{error::Error, fmt};

use basisu::{DecodeFlags, TargetFormat, Transcoder};
use bevy::{
    asset::{AssetApp, AssetLoader, RenderAssetUsages, io::Reader},
    image::Image,
    prelude::{App, Plugin},
    reflect::TypePath,
    render::render_resource::{
        Extent3d, TextureDataOrder, TextureDimension, TextureFormat, TextureViewDescriptor,
        TextureViewDimension,
    },
};
use ktx2::{Reader as Ktx2Reader, TransferFunction};

/// Registers [`BasisKtx2Loader`] with a Bevy asset server.
pub struct BasisKtx2Plugin;

impl Plugin for BasisKtx2Plugin {
    fn build(&self, app: &mut App) {
        app.register_asset_loader(BasisKtx2Loader);
    }
}

/// Transcodes portable BasisLZ KTX2 textures into Bevy RGBA images.
#[derive(Default, TypePath)]
pub struct BasisKtx2Loader;

impl BasisKtx2Loader {
    /// Decodes every mip and array layer into the data layout Bevy uploads.
    pub fn decode(bytes: &[u8]) -> Result<Image, BasisKtx2Error> {
        let texture_format = match Ktx2Reader::new(bytes)
            .map_err(BasisKtx2Error::invalid_container)?
            .transfer_function()
        {
            Some(TransferFunction::SRGB) => TextureFormat::Rgba8UnormSrgb,
            _ => TextureFormat::Rgba8Unorm,
        };
        let texture = Transcoder::new(bytes).map_err(BasisKtx2Error::invalid_payload)?;
        let (width, height) = texture.base_dimensions();
        let layer_count = texture.layer_count().max(1);
        let level_count = texture.level_count().max(1);

        let mut data = Vec::new();
        for mip in 0..level_count {
            for layer in 0..layer_count {
                let pixels = texture
                    .transcode_image(mip, layer, 0, TargetFormat::Rgba32, DecodeFlags::NONE)
                    .map_err(BasisKtx2Error::invalid_payload)?;
                data.extend_from_slice(&pixels);
            }
        }

        let mut image = Image::new_uninit(
            Extent3d {
                width,
                height,
                depth_or_array_layers: layer_count,
            },
            TextureDimension::D2,
            texture_format,
            RenderAssetUsages::default(),
        );
        image.data = Some(data);
        image.data_order = TextureDataOrder::MipMajor;
        image.texture_descriptor.mip_level_count = level_count;
        if layer_count > 1 {
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::D2Array),
                ..Default::default()
            });
        }
        Ok(image)
    }
}

impl AssetLoader for BasisKtx2Loader {
    type Asset = Image;
    type Settings = ();
    type Error = BasisKtx2Error;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _: &Self::Settings,
        _: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Image, Self::Error> {
        let mut bytes = Vec::new();
        reader
            .read_to_end(&mut bytes)
            .await
            .map_err(BasisKtx2Error::read)?;
        Self::decode(&bytes)
    }

    fn extensions(&self) -> &[&str] {
        &["ktx2"]
    }
}

/// An error raised while preparing a BasisLZ texture for Bevy.
#[derive(Debug)]
pub struct BasisKtx2Error(String);

impl BasisKtx2Error {
    fn invalid_container(error: ktx2::ParseError) -> Self {
        Self(format!("invalid KTX2 container: {error:?}"))
    }

    fn invalid_payload(error: basisu::Error) -> Self {
        Self(format!("invalid Basis KTX2 payload: {error:?}"))
    }

    fn read(error: std::io::Error) -> Self {
        Self(format!("unable to read Basis KTX2 asset: {error}"))
    }
}

impl fmt::Display for BasisKtx2Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl Error for BasisKtx2Error {}
