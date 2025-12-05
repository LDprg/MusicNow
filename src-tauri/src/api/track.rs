use log::{error, info};
use serde::Serialize;
use tauri::{AppHandle, Emitter, State};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{
    LastFM,
    audio::AudioPlayer,
    event::{GlobalEvents, PlayStatus},
    musicbrainz::MusicBrainz,
    soundcloud::{self, Soundcloud},
};

#[tauri::command]
pub async fn play(
    app: AppHandle,
    soundcloud: State<'_, Soundcloud>,
    audio_player: State<'_, AudioPlayer>,
    track_id: &str,
) -> Result<(), ()> {
    info!("Playing rust");
    let state = PlayStatus {
        duration: u64::MAX,
        progress: 0,
        is_playing: false,
    };

    info!("Event: play_state: {:#?}", state);
    app.emit("play_state", state).unwrap();

    let res = audio_player
        .play(soundcloud, track_id.parse().unwrap())
        .await
        .map_err(|err| error!("Error in AudioPlayer play: {:#?}", err));

    if res.is_ok() {
        let duration = audio_player.get_duration().await.as_millis() as u64;

        let state = PlayStatus {
            duration,
            progress: 0,
            is_playing: true,
        };

        info!("Event: play_state: {:#?}", state);
        app.emit("play_state", state).unwrap();
    } else {
        app.event_play_status().await;
    }

    res
}

#[tauri::command]
pub async fn pause(app: AppHandle, audio_player: State<'_, AudioPlayer>) -> Result<(), ()> {
    audio_player.pause();
    app.event_play_status().await;
    Ok(())
}

#[tauri::command]
pub async fn resume(app: AppHandle, audio_player: State<'_, AudioPlayer>) -> Result<(), ()> {
    audio_player.resume();
    app.event_play_status().await;
    Ok(())
}

#[tauri::command]
pub async fn toggle_play(app: AppHandle, audio_player: State<'_, AudioPlayer>) -> Result<(), ()> {
    if audio_player.is_playing().await {
        audio_player.pause();
    } else {
        audio_player.resume();
    }
    app.event_play_status().await;
    Ok(())
}

#[tauri::command]
pub async fn is_playing(audio_player: State<'_, AudioPlayer>) -> Result<bool, ()> {
    let val = audio_player.is_playing().await;
    info!("Playing: {}", val);
    Ok(val)
}

#[tauri::command]
pub async fn set_volume(
    app: AppHandle,
    audio_player: State<'_, AudioPlayer>,
    volume: f64,
) -> Result<(), ()> {
    audio_player.set_volume(volume);
    app.event_volume().await;
    Ok(())
}

#[tauri::command]
pub async fn get_volume(audio_player: State<'_, AudioPlayer>) -> Result<f64, ()> {
    Ok(audio_player.get_volume().await)
}

#[tauri::command]
pub async fn get_duration(audio_player: State<'_, AudioPlayer>) -> Result<u64, ()> {
    Ok(audio_player.get_duration().await.as_millis() as u64)
}

#[tauri::command]
pub async fn get_progress(audio_player: State<'_, AudioPlayer>) -> Result<u64, ()> {
    Ok(audio_player.get_progress().await.as_millis() as u64)
}

#[derive(Serialize)]
pub struct SearchApi {
    title: String,
    artist: String,
    image_url: Option<String>,
    mbid: Option<String>,
    available: bool,
}

#[tauri::command]
pub async fn search(
    // lastfm: State<'_, Mutex<LastFM>>,
    soundcloud: State<'_, Soundcloud>,
    // musicbrainz: State<'_, MusicBrainz>,
    query: String,
    limit: usize,
    offset: usize,
) -> Result<Vec<SearchApi>, ()> {
    // let lastfm = lastfm.lock().await;
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
    //                 image_url: None,
    //                 mbid: track.mbid.map(|mbid| mbid.to_string()),
    //                 available: true,
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
                            image_url: track.artwork_url.map(|i| i.to_string()),
                            mbid: Some(track.id.to_string()),
                            available: track.monetization_model != "SUB_HIGH_TIER",
                        })
                    } else {
                        error!("Non Track in Soundcloud Track Search: {:#?}", track);
                        None
                    }
                })
                .collect()
        })
        .map_err(|err| error!("Error in Soundcloud seach: {:#?}", err))

    // musicbrainz
    //     .search(query, limit, offset)
    //     .await
    //     .map(|item| {
    //         item.releases
    //             .into_iter()
    //             .map(|track| SearchApi {
    //                 title: track.title,
    //                 artist: track
    //                     .artist_credit
    //                     .into_iter()
    //                     .map(|item| item.name)
    //                     .collect::<Vec<_>>()
    //                     .join(" & "),
    //                 image_url: None,
    //                 mbid: Some(track.id.to_string()),
    //                 available: true,
    //             })
    //             .collect()
    //     })
    //     .map_err(|err| error!("Error in MusicBrainz search: {:#?}", err))
}
