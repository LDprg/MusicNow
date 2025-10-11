use std::thread;

use dioxus::logger::tracing::Level;

mod app;
mod components;
mod prelude;

#[cfg(feature = "server")]
mod audio;
#[cfg(feature = "server")]
mod soundcloud;

use crate::app::*;

#[cfg(feature = "server")]
mod service {
    use tokio::join;

    use crate::audio::*;
    use crate::soundcloud::*;

    async fn audio() {
        let api = SoundCloudApi::default();
        api.login_anonymous().await.unwrap();

        let client_id = api.get_client_id().unwrap();
        run_audio(client_id).await.unwrap();
    }

    pub fn spawn() {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async { join!(audio()) });
    }
}

fn main() {
    // Init logger
    dioxus::logger::init(Level::INFO).expect("failed to init logger");

    #[cfg(feature = "server")]
    thread::spawn(service::spawn);

    #[cfg(not(feature = "server"))]
    dioxus::fullstack::set_server_url("http://127.0.0.1:8080");

    dioxus::launch(App);
}
