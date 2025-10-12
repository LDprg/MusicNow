#[cfg(feature = "server")]
mod audio;
#[cfg(feature = "server")]
mod soundcloud;

#[cfg(feature = "server")]
pub mod service {
    use tokio::join;

    use crate::prelude::*;
    use crate::server::audio::*;
    use crate::server::soundcloud::*;

    async fn audio() {
        let api = SoundCloudApi::default();
        api.login_anonymous().await.unwrap();

        info!("{:#?}", api.search("Believer", 10, 0).await.unwrap());

        let client_id = api.get_client_id().unwrap();
        run_audio(client_id).await.unwrap();
    }

    pub fn spawn() {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async { join!(audio()) });
    }
}
