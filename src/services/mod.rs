mod audio;
mod metabrainz;
mod lastfm;
mod soundcloud;
mod storage;

use tokio::join;

pub use self::audio::*;
pub use self::soundcloud::*;
pub use self::metabrainz::*;

pub use self::storage::*;
use self::storage::Storage;

use crate::prelude::*;

async fn audio() {
    info!("Logging in");
    let api = SoundCloudApi::default();
    api.login_anonymous().await.unwrap();
}

pub fn spawn() {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    let _ = Storage::default();
    info!("Storage initizialised!");

    runtime.block_on(async move { join!(audio()) });
}
