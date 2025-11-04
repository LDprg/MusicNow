#[cfg(feature = "server")]
mod audio;
#[cfg(feature = "server")]
mod storage;
#[cfg(feature = "server")]
mod soundcloud;
#[cfg(feature = "server")]
mod metabrainz;

#[cfg(feature = "server")]
pub use self::audio::*;
#[cfg(feature = "server")]
pub use self::soundcloud::*;

mod api;
mod meta;

pub use self::api::*;
pub use self::meta::*;

#[cfg(feature = "server")]
pub mod service {
    use tokio::{fs::File, join};

    use crate::prelude::*;
    use super::storage::*;

    async fn audio() {
        info!("Creating Path: {:#?}", get_cache_dir().to_str());
        info!("Creating Path: {:#?}", get_data_dir().to_str());
        // let file = File::create(get_cache_dir().join("test.cache")).await.expect("File creation failed");

        info!("Logging in");
        let api = SoundCloudApi::default();
        api.login_anonymous().await.unwrap();
    }

    pub fn spawn() {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async { join!(audio()) });
    }
}
