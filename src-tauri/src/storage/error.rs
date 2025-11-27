use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStorageError {
    #[error("File io error")]
    File(#[from] std::io::Error),
    #[error("Bincode encode error")]
    BincodeEncode(#[from] bincode::error::EncodeError),
    #[error("Bincode decode error")]
    BincodeDecode(#[from] bincode::error::DecodeError),
    #[error("Tauri error")]
    Tauri(#[from] tauri::Error),
}
