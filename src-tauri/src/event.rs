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
        self.emit("volume", volume).unwrap();
    }

    async fn event_play_status(&self) {
        let audio_player = self.state::<AudioPlayer>();
        let is_playing = audio_player.is_playing().await;
        self.emit("play_state", is_playing).unwrap();
    }
}
