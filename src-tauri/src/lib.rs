mod api;
mod audio;
mod event;
mod lastfm;
mod soundcloud;
mod storage;

use log::{error, info};
use std::error::Error;
use tokio::sync::Mutex;

use api::*;
use audio::*;
use lastfm::*;
use soundcloud::*;
use storage::*;

use tauri::{AppHandle, Manager};

async fn setup(app: &AppHandle) -> Result<(), Box<dyn Error>> {
    let data_storage = DataStorage::new(app)?;
    app.manage(data_storage);

    let audio_player = AudioPlayer::default();
    app.manage(audio_player);

    let lastfm = Mutex::new(LastFM::default());
    {
        let mut lastfm = lastfm.lock().await;
        if !lastfm.load_login(app).await? {
            info!("No LastFM login data!");
        }
    }
    app.manage(lastfm);

    let mut soundcloud = Soundcloud::default();
    if let Err(err) = soundcloud.login_anonymous().await {
        error!("Error: {:#?}", err);
    }
    app.manage(soundcloud);

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .setup(|app| tauri::async_runtime::block_on(setup(app.handle())))
        .invoke_handler(tauri::generate_handler![
            // track
            play,
            pause,
            resume,
            toggle_play,
            is_playing,
            set_volume,
            get_volume,
            get_duration,
            get_progress,
            search,
            // provider
            login_listenbrainz,
            is_listenbrainz,
            login_lastfm,
            is_lastfm,
            login_soundcloud,
            is_soundcloud,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
