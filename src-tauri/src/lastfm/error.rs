use thiserror::Error;

use crate::lastfm::LastFMApiError;

#[derive(Error, Debug)]
pub enum LastFMError {
    #[error("Login data missing")]
    LoginDataMissing,
    #[error("The api returned an error")]
    ApiError(LastFMApiError),
    #[error("Json parsering failed")]
    JsonParsingError(#[from] serde_json::Error),
}
