mod lastfm;
mod storage;

use log::error;
use serde::Serialize;
use std::error::Error;

use lastfm::*;
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
    lastfm: State<'_, LastFM>,
    query: String,
    limit: usize,
    page: usize,
) -> Result<Vec<SearchApi>, ()> {
    lastfm
        .search(query, limit, page)
        .await
        .map(|item| {
            item.trackmatches
                .track
                .into_iter()
                .map(|track| SearchApi {
                    title: track.name,
                    artist: track.artist,
                    mbid: track.mbid.map(|mbid| mbid.to_string()),
                })
                .collect()
        })
        .map_err(|err| error!("Error in LastFM search: {}", err))
}

async fn setup(app: &AppHandle) -> Result<(), Box<dyn Error>> {
    let data_storage = DataStorage::new(app)?;
    app.manage(data_storage);

    let mut lastfm = LastFM::default();
    if let Err(err) = lastfm.login(app).await {
        error!("Error: {:#?}", err);
    }
    app.manage(lastfm);
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
