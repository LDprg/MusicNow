use thiserror::Error;

use crate::{lastfm::LastFMApiError, storage::error::DataStorageError};

#[derive(Error, Debug)]
pub enum LastFMError {
    #[error("Login data missing")]
    LoginDataMissing,
    #[error("The api returned an error")]
    ApiError(LastFMApiError),
    #[error("Json parsing failed")]
    JsonParsingError(serde_json::Error, String),
    #[error("Open Url/Path failed")]
    OpenError(#[from] tauri_plugin_opener::Error),
    #[error("Data storage error")]
    DataStorageError(#[from] DataStorageError),
}
