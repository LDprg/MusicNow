use log::{error, info};
use tauri::{AppHandle, State};
use tokio::sync::Mutex;

use crate::lastfm::LastFM;

#[tauri::command]
pub async fn login_listenbrainz() -> Result<(), ()> {
    info!("Listenbrainz login");
    Ok(())
}

#[tauri::command]
pub async fn is_listenbrainz() -> Result<bool, ()> {
    Ok(false)
}

#[tauri::command]
pub async fn login_lastfm(app: AppHandle, lastfm: State<'_, Mutex<LastFM>>) -> Result<(), ()> {
    info!("LastFM login");
    let mut lastfm = lastfm.lock().await;
    lastfm
        .login(&app)
        .await
        .map_err(|err| error!("Lastfm login error: {:#?}", err))
}

#[tauri::command]
pub async fn is_lastfm(lastfm: State<'_, Mutex<LastFM>>) -> Result<bool, ()> {
    Ok(lastfm.lock().await.is_login())
}

#[tauri::command]
pub async fn login_soundcloud() -> Result<(), ()> {
    info!("Soundcloud login");
    Ok(())
}

#[tauri::command]
pub async fn is_soundcloud() -> Result<bool, ()> {
    Ok(true)
}
