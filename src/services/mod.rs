mod audio;
mod metabrainz;
mod soundcloud;
mod storage;

use tokio::join;

pub use self::audio::*;
pub use self::soundcloud::*;

// TODO: Remove meta crate and make it part of soundcloud
mod meta;

pub use self::meta::*;
pub use self::storage::*;

use crate::prelude::*;

async fn audio() {
    info!("Cache Path: {:?}", get_cache_dir());
    info!("Data Path: {:?}", get_data_dir());

    info!("Logging in");
    let api = SoundCloudApi::default();
    api.login_anonymous().await.unwrap();
}

pub fn spawn() {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    runtime.block_on(async move { join!(audio()) });
}
