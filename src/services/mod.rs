mod audio;
// mod metabrainz;
mod soundcloud;
// mod storage;

use tokio::join;

pub use self::soundcloud::*;

mod meta;

pub use self::meta::*;

use crate::prelude::*;

async fn audio() {
    // info!("Creating Path: {:#?}", get_cache_dir().to_str());
    // info!("Creating Path: {:#?}", get_data_dir().to_str());
    // let file = File::create(get_cache_dir().join("test.cache")).await.expect("File creation failed");

    info!("Logging in");
    let api = SoundCloudApi::default();
    api.login_anonymous().await.unwrap();
}

pub fn spawn() {
    #[cfg(not(feature = "web"))]
    let runtime = tokio::runtime::Runtime::new().unwrap();
    #[cfg(feature = "web")]
    let runtime = tokio::runtime::Builder::new_current_thread().build().unwrap();

    runtime.block_on(async { join!(audio()) });
}
