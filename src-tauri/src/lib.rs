mod lastfm;

use std::error::Error;

use lastfm::*;
use tauri::{AppHandle, Manager};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

async fn setup(app: &AppHandle) -> Result<(), Box<dyn Error>> {
    let mut lastfm = LastFM::default();
    lastfm.login().await;
    app.manage(lastfm);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Trace)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .setup(|app| tauri::async_runtime::block_on(setup(app.handle())))
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
