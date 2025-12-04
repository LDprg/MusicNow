use log::info;
use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, Runtime};

use crate::audio::AudioPlayer;

#[derive(Serialize, Clone, Debug)]
pub struct PlayStatus {
    pub is_playing: bool,
    pub progress: u64,
    pub duration: u64,
}

pub trait GlobalEvents {
    async fn event_volume(&self);
    async fn event_play_status(&self);
}

impl<R: Runtime> GlobalEvents for AppHandle<R> {
    async fn event_volume(&self) {
        let audio_player = self.state::<AudioPlayer>();
        let volume = audio_player.get_volume().await;
        info!("Event: volume: {}", volume);
        self.emit("volume", volume).unwrap();
    }

    async fn event_play_status(&self) {
        let audio_player = self.state::<AudioPlayer>();

        let duration = audio_player.get_duration().await.as_millis() as u64;
        let progress = audio_player.get_progress().await.as_millis() as u64;
        let is_playing = audio_player.is_playing().await;

        let state = PlayStatus {
            duration,
            progress,
            is_playing,
        };

        info!("Event: play_state: {:#?}", state);
        self.emit("play_state", state).unwrap();
    }
}
