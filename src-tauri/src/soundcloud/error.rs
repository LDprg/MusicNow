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
    ScraperError(#[from] scraper::error::SelectorErrorKind<'static>),
    #[error("Regex Error")]
    RegexError(#[from] regex::Error),
    #[error("Reqwest failed")]
    ReqwestError(#[from] tauri_plugin_http::reqwest::Error),
}
