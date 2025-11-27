use std::{fs::File, path::PathBuf};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod error;
use error::*;
use tauri::{AppHandle, Manager};

const BINCODE_CONFIG: bincode::config::Configuration = bincode::config::standard();

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct LastFMStorage {
    pub session_key: String,
    pub username: String,
}

pub struct DataStorage {
    lastfm_path: PathBuf,
}

impl DataStorage {
    pub fn new(app: &AppHandle) -> Result<Self, DataStorageError> {
        let lastfm_path = app
            .path()
            .resolve("lastfm.bin", tauri::path::BaseDirectory::Data)?;
        let _: LastFMStorage = Self::get_storage_bincode(lastfm_path.clone())?;

        Ok(Self { lastfm_path })
    }

    fn get_storage_bincode<T: DeserializeOwned + Serialize + Default>(
        path: PathBuf,
    ) -> Result<T, DataStorageError> {
        if !path.exists() {
            let mut file = File::create(&path)?;
            _ = bincode::serde::encode_into_std_write(T::default(), &mut file, BINCODE_CONFIG)?;
        }

        let mut file = File::open(path)?;
        let data = bincode::serde::decode_from_std_read(&mut file, BINCODE_CONFIG)?;

        Ok(data)
    }

    pub fn read_lastfm(&self) -> Result<LastFMStorage, DataStorageError> {
        let mut file = File::open(&self.lastfm_path)?;
        let data = bincode::serde::decode_from_std_read(&mut file, BINCODE_CONFIG)?;
        Ok(data)
    }

    pub fn write_lastfm(&self, lastfm: &LastFMStorage) -> Result<(), DataStorageError> {
        let mut file = File::create(&self.lastfm_path)?;
        _ = bincode::serde::encode_into_std_write(lastfm, &mut file, BINCODE_CONFIG)?;
        Ok(())
    }
}
