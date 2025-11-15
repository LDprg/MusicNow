mod audio;
mod lastfm;
mod metabrainz;
mod soundcloud;
mod storage;

use tokio::join;

pub use self::audio::*;
pub use self::lastfm::*;
pub use self::metabrainz::*;
pub use self::soundcloud::*;

use self::storage::Storage;
pub use self::storage::*;

use crate::prelude::*;

async fn soundcloud() {
    info!("Soundcloud: Logging in");
    let api = SoundCloudApi::default();
    api.login_anonymous().await.unwrap();
}

pub fn spawn() {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    let _ = Storage::default();
    info!("Storage initizialised!");

    runtime.block_on(async move { join!(soundcloud()) });
}
