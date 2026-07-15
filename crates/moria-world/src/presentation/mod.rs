//! Immutable runtime asset declarations and their loading contract.

mod asset_ids;
mod assets;

pub use asset_ids::{
    ASSET_COUNT, ASSET_DECLARATIONS, AssetDeclaration, AssetId, AssetLoadPolicy,
    AssetValidationFixture,
};
pub use assets::{
    AssetDeclarationError, AssetLoader, AssetMissingAction, RuntimeAssetProfile,
    validate_asset_declarations,
};
