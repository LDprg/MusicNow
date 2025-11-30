use log::info;
use tauri::{AppHandle, Emitter, Manager};

use crate::audio::AudioPlayer;

pub trait GlobalEvents {
    async fn event_volume(&self);
    async fn event_play_status(&self);
}

impl GlobalEvents for AppHandle {
    async fn event_volume(&self) {
        let audio_player = self.state::<AudioPlayer>();
        let volume = audio_player.get_volume().await;
        info!("Event: volume: {}", volume);
        self.emit("volume", volume).unwrap();
    }

    async fn event_play_status(&self) {
        let audio_player = self.state::<AudioPlayer>();

        let duration = audio_player.get_duration().await.as_millis() as u64;
        info!("Event: duration: {}", duration);
        self.emit("duration", duration).unwrap();

        let is_playing = audio_player.is_playing().await;
        info!("Event: play_state: {}", is_playing);
        self.emit("play_state", is_playing).unwrap();

        let progress = audio_player.get_progress().await.as_millis() as u64;
        info!("Event: progress: {}", progress);
        self.emit("progress", progress).unwrap();
    }
}
