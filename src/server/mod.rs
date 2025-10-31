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

    use crate::prelude::*;

    async fn audio() {
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
