use std::thread;

use dioxus::logger::tracing::Level;
use tokio::runtime::*;

mod app;
mod audio;
mod components;
mod prelude;
mod soundcloud;

use crate::app::*;
use crate::audio::*;
use crate::soundcloud::*;

async fn audio_service() {
    let api = SoundCloudApi::default();
    api.login_anonymous().await.unwrap();

    let client_id = api.get_client_id().unwrap();
    run_audio(client_id).await.unwrap();
}

fn spawn_services() {
    let rt = Builder::new_current_thread().enable_all().build().unwrap();

    thread::spawn(move || {
        rt.block_on(audio_service());
    });
}

fn main() {
    // Init logger
    dioxus::logger::init(Level::INFO).expect("failed to init logger");

    thread::spawn(spawn_services);

    dioxus::launch(App);
}
