use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStorageError {
    #[error("File io error")]
    FileError(#[from] std::io::Error),
    #[error("Bincode encode error")]
    BincodeEncodeError(#[from] bincode::error::EncodeError),
    #[error("Bincode decode error")]
    BincodeDecodeError(#[from] bincode::error::DecodeError),
    #[error("Tauri error")]
    TauriError(#[from] tauri::Error),
}
