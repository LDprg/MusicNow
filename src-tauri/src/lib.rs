mod lastfm;
mod storage;

use log::error;
use std::error::Error;

use lastfm::*;
use storage::*;

use tauri::{AppHandle, Manager};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
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
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
