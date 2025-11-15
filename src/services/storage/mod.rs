mod paths;

use crate::prelude::*;

use paths::*;
use serde::{Deserialize, Serialize};

use std::{fs::{self, File}, io::Write, path::PathBuf, sync::{Arc, LazyLock}};

static SINGLETON_STORAGE: LazyLock<Storage> = LazyLock::new(Storage::new);

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct StaticConfig {
    lastfm_api_key: String,
}

#[derive(Clone, Debug)]
pub struct Storage {
    config: Arc<StaticConfig>,
}

impl Storage {
    fn new() -> Self {
        info!("Data dir: {:?}", get_data_dir());
        info!("Cache dir: {:?}", get_cache_dir());

        let config_path = Storage::get_config_path();

        let config_data = match fs::read_to_string(config_path) {
            Ok(str) => str,
            Err(_) => Storage::create_config(),
        };

        let config : StaticConfig = toml::from_str(&config_data).unwrap();
        let config = Arc::new(config);

        Self{config}
    }

    fn create_config() -> String {
        let data = toml::to_string(&StaticConfig::default()).unwrap();

        let mut file = File::create(Storage::get_config_path()).unwrap();

        file.write_all(data.as_bytes()).unwrap();

        data
    }

    fn get_config_path()-> PathBuf{
        get_data_dir().join("Config.toml")
    }
}

impl Default for Storage {
    fn default() -> Self {
        SINGLETON_STORAGE.clone()
    }
}


