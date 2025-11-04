use std::{
    sync::{Arc, LazyLock, Mutex},
    thread,
    time::Duration,
};

use crate::prelude::*;

use bytes::Bytes;
use dioxus::logger::tracing::info;
use m3u8_rs::MediaPlaylist;
use serde::de;
use tokio::task::{JoinHandle, spawn_blocking};

mod sink;
mod stream;

use sink::*;
use stream::*;

static SINGLETON_PLAYER: LazyLock<AudioPlayer> = LazyLock::new(AudioPlayer::new);

#[derive(Debug)]
pub struct AudioPlayer {
    tx: std::sync::mpsc::Sender<AudioPlayerCommands>,
    rx: tokio::sync::broadcast::Receiver<AudioPlayerStatus>,
}

#[derive(Debug)]
enum AudioPlayerCommands {
    Play(AudioStreamer),
    Pause,
    Resume,
    IsPlaying,
}

#[derive(Clone, Debug)]
enum AudioPlayerStatus {
    IsPlaying(bool),
    Volume(f64),
}

impl AudioPlayer {
    fn new() -> Self {
        let (sync_tx, sync_rx) = std::sync::mpsc::channel::<AudioPlayerCommands>();
        let (async_tx, async_rx) = tokio::sync::broadcast::channel(100);

        thread::spawn(|| audio_task(async_tx, sync_rx));

        Self {
            tx: sync_tx,
            rx: async_rx,
        }
    }

    pub async fn play(&self, stream: Bytes) -> Result<()> {
        info!("Parsing m3u8");

        let playlist =
            m3u8_rs::parse_media_playlist_res(&stream).map_err(|e| anyhow!(e.to_string()))?;

        info!("Starting playback");

        let segments = playlist.segments.clone();
        let tx = self.tx.clone();
        let handle: JoinHandle<Result<()>> = tokio::task::spawn_local(async move {
            let stream = AudioStreamer::default();

            info!("Starting audio");
            tx.send(AudioPlayerCommands::Play(stream.clone())).unwrap();

            info!("Starting stream");

            for segment in segments {
                if let Some(map) = &segment.map {
                    let resp = reqwest::get(&map.uri).await?;
                    let data = resp.bytes().await?;

                    stream.append(&data).await;
                }

                let resp = reqwest::get(&segment.uri).await?;
                let data = resp.bytes().await?;
                stream.append(&data).await;
            }

            stream.finish().await;

            Ok(())
        });

        Ok(())
    }

    pub async fn pause(&self) {
        self.tx.send(AudioPlayerCommands::Pause).unwrap();
    }

    pub async fn resume(&self) {
        self.tx.send(AudioPlayerCommands::Resume).unwrap();
    }

    pub async fn is_playing(&self) -> bool {
        self.tx.send(AudioPlayerCommands::IsPlaying).unwrap();

        let mut rx = self.rx.resubscribe();
        loop {
            if let AudioPlayerStatus::IsPlaying(status) = rx.recv().await.unwrap() {
                return status;
            }
        }
    }

    // pub fn position(&self) -> Duration {
    //     let inner = self.inner.lock().unwrap();
    //     if inner.playlist.is_some() {
    //         self.sink.postion()
    //     } else {
    //         Duration::ZERO
    //     }
    // }
    //
    // pub fn duration(&self) -> Duration {
    //     let inner = self.inner.lock().unwrap();
    //     if let Some(playlist) = &inner.playlist {
    //         let duration: f32 = playlist.segments.iter().map(|i| i.duration).sum();
    //         Duration::from_secs_f32(duration)
    //     } else {
    //         Duration::ZERO
    //     }
    // }

    // pub fn set_volume(&self, value: f64) {
    //     self.sink.set_volume(value / 500.0);
    // }
    //
    // pub fn get_volume(&self) -> f64 {
    //     self.sink.get_volume() * 500.0
    // }
}

impl Clone for AudioPlayer {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
            rx: self.rx.resubscribe(),
        }
    }
}

impl Default for AudioPlayer {
    fn default() -> Self {
        SINGLETON_PLAYER.clone()
    }
}

// Audio on Web and Android is single threaded
fn audio_task(
    tx: tokio::sync::broadcast::Sender<AudioPlayerStatus>,
    rx: std::sync::mpsc::Receiver<AudioPlayerCommands>,
) {
    let sink = AudioSink::default();

    while let Ok(cmd) = rx.recv() {
        match cmd {
            AudioPlayerCommands::Play(stream) => sink.play(stream),
            AudioPlayerCommands::Pause => sink.pause(),
            AudioPlayerCommands::Resume => sink.resume(),
            AudioPlayerCommands::IsPlaying => {
                let is_playing = !sink.is_paused();
                tx.send(AudioPlayerStatus::IsPlaying(is_playing)).unwrap();
            }
        };
    }
}
