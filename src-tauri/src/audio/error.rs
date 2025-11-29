use thiserror::Error;

use crate::soundcloud::error::SoundcloudError;

#[derive(Error, Debug)]
pub enum AudioError {
    #[error("Rodio output stream error")]
    OutputStreamError(#[from] rodio::stream::StreamError),
    #[error("Track not found")]
    TrackNotFound,
    #[error("Media provider not found")]
    MediaNotFound,
    #[error("Format not compatible")]
    FormatIncompatible,
    #[error("Soundcloud error")]
    SoundcloudError(#[from] SoundcloudError),
    #[error("M3U8 error")]
    M3U8Error(String),
}
