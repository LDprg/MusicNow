use std::{
    sync::{Arc, LazyLock, Mutex},
    time::Duration,
};

use crate::prelude::*;

use bytes::Bytes;
use dioxus::logger::tracing::info;
use m3u8_rs::MediaPlaylist;
use reqwest::Url;
use tokio::task::{JoinHandle, spawn_blocking};

mod sink;
mod stream;

use sink::*;
use stream::*;

static SINGLETON_PLAYER: LazyLock<AudioPlayer> = LazyLock::new(|| AudioPlayer::new());

#[derive(Clone, Debug)]
pub struct AudioPlayer {
    sink: AudioSink,
    inner: Arc<Mutex<AudioPlayerInner>>,
}

#[derive(Debug)]
struct AudioPlayerInner {
    download_task: Option<JoinHandle<Result<()>>>,
    playlist: Option<MediaPlaylist>,
}

impl AudioPlayer {
    fn new() -> Self {
        Self {
            sink: AudioSink::default(),
            inner: Arc::new(Mutex::new(AudioPlayerInner {
                download_task: None,
                playlist: None,
            })),
        }
    }

    pub async fn play(&self, stream: Bytes) -> Result<()> {
        let mut inner = self.inner.lock().unwrap();

        if let Some(handle) = inner.download_task.take() {
            handle.abort();
        }

        info!("Parsing m3u8");

        let playlist =
            m3u8_rs::parse_media_playlist_res(&stream).map_err(|e| anyhow!(e.to_string()))?;

        info!("Starting playback");
        let sink = self.sink.clone();

        let segments = playlist.segments.clone();
        let handle: JoinHandle<Result<()>> = tokio::spawn(async move {
            let stream = AudioStreamer::default();

            info!("Starting audio");
            let music_player = spawn_blocking({
                let stream = stream.clone();
                move || -> Result<()> {
                    info!("Playing audio");
                    sink.play(stream)
                }
            });

            info!("Starting stream");

            for segment in segments {
                if let Some(map) = &segment.map {
                    let resp = reqwest::get(&map.uri).await?;
                    let data = resp.bytes().await?;

                    stream.append(&data);
                }

                let resp = reqwest::get(&segment.uri).await?;
                let data = resp.bytes().await?;
                stream.append(&data);
            }

            stream.finish();

            music_player.await??;

            Ok(())
        });

        info!("Playback started");

        inner.download_task = Some(handle);
        inner.playlist = Some(playlist);
        Ok(())
    }

    pub fn pause(&self) {
        self.sink.pause();
    }

    pub fn resume(&self) {
        self.sink.resume();
    }

    pub fn is_paused(&self) -> bool {
        self.sink.is_paused()
    }

    pub fn position(&self) -> Duration {
        let inner = self.inner.lock().unwrap();
        if inner.playlist.is_some() {
            self.sink.postion()
        } else {
            Duration::ZERO
        }
    }

    pub fn duration(&self) -> Duration {
        let inner = self.inner.lock().unwrap();
        if let Some(playlist) = &inner.playlist {
            let duration: f32 = playlist.segments.iter().map(|i| i.duration).sum();
            Duration::from_secs_f32(duration)
        } else {
            Duration::ZERO
        }
    }

    pub fn set_volume(&self, value: f64) {
        self.sink.set_volume(value / 500.0);
    }

    pub fn get_volume(&self) -> f64 {
        self.sink.get_volume() * 500.0
    }
}

impl Default for AudioPlayer {
    fn default() -> Self {
        SINGLETON_PLAYER.clone()
    }
}
