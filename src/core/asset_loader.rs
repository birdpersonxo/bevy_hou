use crate::component::file::HouData;
use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
};
use thiserror::Error;

/// Possible errors that can be produced by [`CustomAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum HouAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    // #[error("Could not parse RON: {0}")]
    // RonSpannedError(#[from] ron::error::SpannedError),
    #[error("Could not parse JSON: {0}")]
    JsonError(#[from] serde_json::Error), // Change from RonSpannedError to JsonError
}

#[derive(Default)]
pub struct HouAssetLoader;

impl AssetLoader for HouAssetLoader {
    type Asset = HouData;
    type Settings = ();
    type Error = HouAssetLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let custom_asset = serde_json::from_slice::<HouData>(&bytes)?;
        Ok(custom_asset)
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}
