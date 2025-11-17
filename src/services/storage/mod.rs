mod paths;

use crate::prelude::*;

use paths::*;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    sync::{Arc, LazyLock},
};

const BINCODE_CONFIG: bincode::config::Configuration = bincode::config::standard();

static SINGLETON_STORAGE: LazyLock<DataStorage> = LazyLock::new(DataStorage::new);

// Only make android systemcalls once
static DATA_DIR: LazyLock<PathBuf> = LazyLock::new(get_data_dir);
static CACHE_DIR: LazyLock<PathBuf> = LazyLock::new(get_cache_dir);

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct StaticConfig {}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SecretStorage {
    pub last_fm: LastFMStorage,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct LastFMStorage {
    pub session_key: String,
    pub username: String,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct DataStorage {
    pub static_config: Arc<StaticConfig>,
}

impl DataStorage {
    fn new() -> Self {
        info!("Data dir: {:?}", &*DATA_DIR);
        info!("Cache dir: {:?}", &*CACHE_DIR);

        if !DATA_DIR.exists() {
            fs::create_dir(&*DATA_DIR).unwrap();
        }

        if !CACHE_DIR.exists() {
            fs::create_dir(&*CACHE_DIR).unwrap();
        }

        let static_config = Self::get_storage_toml::<StaticConfig>(Self::get_config_path());
        let static_config = Arc::new(static_config);

        Self { static_config }
    }

    pub fn load_lastfm(&self) -> LastFMStorage {
        let secret_storage = Self::get_storage_bincode::<SecretStorage>(Self::get_secret_path());

        secret_storage.last_fm
    }

    pub fn store_lastfm(&self, data: LastFMStorage) {
        let mut secret_storage =
            Self::get_storage_bincode::<SecretStorage>(Self::get_secret_path());

        secret_storage.last_fm = data;

        let mut file = File::create(Self::get_secret_path()).unwrap();
        _ = bincode::serde::encode_into_std_write(secret_storage, &mut file, BINCODE_CONFIG)
            .unwrap();
    }

    fn get_storage_toml<T: Serialize + DeserializeOwned + Default>(path: PathBuf) -> T {
        if !path.exists() {
            let data = toml::to_string(&T::default()).unwrap();

            let mut file = File::create(&path).unwrap();
            file.write_all(data.as_bytes()).unwrap();
        }

        let data = fs::read_to_string(path).unwrap();

        toml::from_str(&data).unwrap()
    }

    fn get_storage_bincode<T: Serialize + DeserializeOwned + Default>(path: PathBuf) -> T {
        if !path.exists() {
            let mut file = File::create(&path).unwrap();
            _ = bincode::serde::encode_into_std_write(&T::default(), &mut file, BINCODE_CONFIG)
                .unwrap();
        }

        let mut file = File::open(path).unwrap();
        bincode::serde::decode_from_std_read(&mut file, BINCODE_CONFIG).unwrap()
    }
    fn get_config_path() -> PathBuf {
        DATA_DIR.join("Config.toml")
    }

    fn get_secret_path() -> PathBuf {
        DATA_DIR.join("Secret.bin")
    }
}

impl Default for DataStorage {
    fn default() -> Self {
        SINGLETON_STORAGE.clone()
    }
}
