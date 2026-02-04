mod proxy;
pub use proxy::*;

use std::marker::PhantomData;

use bevy_app::{App, Plugin};
use bevy_asset::{Asset, AssetApp, AssetLoader, LoadContext, io::Reader};
use bevy_reflect::TypePath;
use eure::{FromEure, document::parse::ParseError};
use thiserror::Error;

pub struct EureAssetPlugin<T> {
    extensions: Vec<&'static str>,
    _phantom: PhantomData<T>,
}

impl<T> Plugin for EureAssetPlugin<T>
where
    T: Asset + for<'doc> FromEure<'doc, Error = ParseError>,
{
    fn build(&self, app: &mut App) {
        app.init_asset::<T>()
            .register_asset_loader(EureAssetLoader::<T> {
                extensions: self.extensions.clone(),
                _marker: PhantomData,
            });
    }
}

// Caller can get the insufficient trait bounds error instead of Plugin is not implemented error
impl<T> EureAssetPlugin<T>
where
    T: Asset + for<'doc> FromEure<'doc, Error = ParseError>,
{
    pub fn new(extensions: &'static [&str]) -> Self {
        Self {
            extensions: extensions.to_vec(),
            _phantom: PhantomData,
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum EureLoaderError {
    /// An [IO Error](std::io::Error)
    #[error("Could not read the file: {0}")]
    Io(#[from] std::io::Error),
    /// A [FromUtf8Error](std::string::FromUtf8Error)
    #[error("Could not interpret as UTF-8: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    /// A [EureError](eure::query::EureQueryError)
    #[error("Could not parse Eure: {0}")]
    EureError(String),
}

/// An asset loader for Eure assets.
#[derive(TypePath)]
pub struct EureAssetLoader<T> {
    extensions: Vec<&'static str>,
    _marker: PhantomData<T>,
}

#[derive(FromEure)]
pub struct DummyPartialEq<T>(T);

impl<T> PartialEq for DummyPartialEq<T> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T> AssetLoader for EureAssetLoader<T>
where
    T: Asset + for<'doc> FromEure<'doc, Error = ParseError>,
{
    type Asset = T;

    type Settings = ();

    type Error = EureLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        eure::parse_content::<T>(
            &String::from_utf8(bytes)?,
            load_context.path().path().to_path_buf(),
        )
        .map_err(EureLoaderError::EureError)
    }

    fn extensions(&self) -> &[&str] {
        &self.extensions
    }
}
