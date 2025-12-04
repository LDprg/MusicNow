mod api;
mod audio;
mod event;
mod lastfm;
mod soundcloud;
mod storage;

use log::error;
use std::error::Error;

use audio::*;
// use lastfm::*;
use api::*;
use soundcloud::*;
use storage::*;

use tauri::{AppHandle, Manager};

async fn setup(app: &AppHandle) -> Result<(), Box<dyn Error>> {
    let data_storage = DataStorage::new(app)?;
    app.manage(data_storage);

    let audio_player = AudioPlayer::default();
    app.manage(audio_player);

    // TODO: LastFM Login seems broken on android
    // let mut lastfm = LastFM::default();
    // if let Err(err) = lastfm.login(app).await {
    //     error!("Error: {:#?}", err);
    // }
    // app.manage(lastfm);

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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
