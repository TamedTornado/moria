//! Immutable runtime asset declarations and their loading contract.

mod asset_ids;
mod assets;
mod validation;

pub use asset_ids::{
    AssetDeclaration, AssetId, AssetLoadPolicy, AssetValidationFixture, ASSET_COUNT,
    ASSET_DECLARATIONS,
};
pub use assets::{
    validate_asset_declarations, AssetDeclarationError, AssetLoader, AssetMissingAction,
    RuntimeAssetProfile,
};
pub use validation::{
    validate_asset_directory, AssetBudgetContract, AssetBudgetEntry, AssetBudgetRegistry,
    AssetLicenseEntry, AssetLicenseRegistry, AssetProvenance, AssetValidationError,
    AssetValidationPlugin, AssetValidationReport, AssetValidationStatus, ObjectRenderHandles,
    RenderAssetHandleId, TextureColorSpace, WorldRenderAssets,
};
