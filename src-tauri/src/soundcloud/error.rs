use thiserror::Error;

#[derive(Error, Debug)]
pub enum SoundcloudError {
    #[error("Login data missing")]
    LoginDataMissing,
    #[error("Anynymous login failed")]
    AnonymousLoginFailed,
    #[error("Json parsing failed")]
    JsonParsingError(serde_json::Error, String),
    #[error("Scraper Error")]
    ScraperError(String),
    #[error("Regex Error")]
    RegexError(#[from] regex::Error),
    #[error("Reqwest failed")]
    ReqwestError(#[from] tauri_plugin_http::reqwest::Error),
}

// Stupid non owned errors
impl<'a> From<scraper::error::SelectorErrorKind<'a>> for SoundcloudError {
    fn from(value: scraper::error::SelectorErrorKind<'a>) -> Self {
        SoundcloudError::ScraperError(value.to_string())
    }
}
