mod lastfm;
mod soundcloud;
mod storage;

use log::error;
use serde::Serialize;
use std::error::Error;

use lastfm::*;
use soundcloud::*;
use storage::*;

use tauri::{AppHandle, Manager, State};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
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
                    if let SoundcloudApiSearchElement::Track(track) = track {
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
                            artist: artist,
                            mbid: Some(track.urn),
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

    let mut lastfm = LastFM::default();
    if let Err(err) = lastfm.login(app).await {
        error!("Error: {:#?}", err);
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
        .invoke_handler(tauri::generate_handler![greet, search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
