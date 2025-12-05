use thiserror::Error;

#[derive(Error, Debug)]
pub enum MusicBrainzError {
    #[error("Json parsing failed")]
    JsonParsingError(serde_json::Error, String),
    #[error("Reqwest middleware error")]
    ReqwestMiddleError(#[from] reqwest_middleware::Error),
    #[error("Reqwest error")]
    ReqwestError(#[from] tauri_plugin_http::reqwest::Error),
}
