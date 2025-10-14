#[cfg(feature = "server")]
mod audio;
#[cfg(feature = "server")]
mod soundcloud;

mod api;
mod meta;

#[cfg(feature = "server")]
pub use self::audio::*;
#[cfg(feature = "server")]
pub use self::soundcloud::*;

pub use self::api::*;
pub use self::meta::*;

#[cfg(feature = "server")]
pub mod service {
    use tokio::join;

    use super::*;
    use crate::prelude::*;

    async fn audio() {
        let api = SoundCloudApi::default();
        api.login_anonymous().await.unwrap();

        // info!("{:#?}", api.search("Believer", 10, 0).await.unwrap());

        let client_id = api.get_client_id().unwrap();
        // run_audio(client_id).await.unwrap();
    }

    pub fn spawn() {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async { join!(audio()) });
    }
}
