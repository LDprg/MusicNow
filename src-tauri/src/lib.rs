mod audio;
mod lastfm;
mod soundcloud;
mod storage;

use log::{error, info};
use serde::Serialize;
use std::error::Error;

use audio::*;
// use lastfm::*;
use soundcloud::*;
use storage::*;

use tauri::{AppHandle, Manager, State};

#[tauri::command]
async fn play(
    soundcloud: State<'_, Soundcloud>,
    audio_player: State<'_, AudioPlayer>,
    track_id: u64,
) -> Result<(), ()> {
    info!("Playing rust");
    audio_player
        .play(soundcloud, track_id)
        .await
        .map_err(|err| error!("Error in AudioPlayer play: {:#?}", err))
}

#[tauri::command]
fn pause(audio_player: State<'_, AudioPlayer>) {
    audio_player.pause();
}

#[tauri::command]
fn resume(audio_player: State<'_, AudioPlayer>) {
    audio_player.resume();
}

#[tauri::command]
fn set_volume(audio_player: State<'_, AudioPlayer>, volume: f64) {
    audio_player.set_volume(volume);
}

#[derive(Serialize)]
struct SearchApi {
    title: String,
    artist: String,
    mbid: Option<String>,
}

#[tauri::command]
async fn search(
    // lastfm: State<'_, LastFM>,
    soundcloud: State<'_, Soundcloud>,
    query: String,
    limit: usize,
    offset: usize,
) -> Result<Vec<SearchApi>, ()> {
    // lastfm
    //     .search(query, limit, offset)
    //     .await
    //     .map(|item| {
    //         item.trackmatches
    //             .track
    //             .into_iter()
    //             .map(|track| SearchApi {
    //                 title: track.name,
    //                 artist: track.artist,
    //                 mbid: track.mbid.map(|mbid| mbid.to_string()),
    //             })
    //             .collect()
    //     })
    //     .map_err(|err| error!("Error in LastFM search: {:#?}", err))
    soundcloud
        .search(query, limit, offset)
        .await
        .map(|item| {
            item.collection
                .into_iter()
                .filter_map(|track| {
                    if let soundcloud::meta::SearchElement::Track(track) = track {
                        let artist = if let Some(publisher_metadata) = track.publisher_metadata
                            && let Some(artist) = publisher_metadata.artist
                        {
                            artist
                        } else if let Some(user) = track.user {
                            user.full_name
                        } else {
                            "None".to_string()
                        };

                        Some(SearchApi {
                            title: track.title,
                            artist,
                            mbid: Some(track.id.to_string()),
                        })
                    } else {
                        error!("Non Track in Soundcloud Track Search: {:#?}", track);
                        None
                    }
                })
                .collect()
        })
        .map_err(|err| error!("Error in Soundcloud seach: {:#?}", err))
}

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
            play, pause, resume, set_volume, search
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
