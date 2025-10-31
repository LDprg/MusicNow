use std::sync::{Arc, LazyLock, Mutex};

use crate::prelude::*;

use bytes::Bytes;
use dioxus::logger::tracing::info;
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
}

impl AudioPlayer {
    fn new() -> Self {
        Self {
            sink: AudioSink::default(),
            inner: Arc::new(Mutex::new(AudioPlayerInner {
                download_task: None,
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

            for segment in playlist.segments {
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
}

impl Default for AudioPlayer {
    fn default() -> Self {
        SINGLETON_PLAYER.clone()
    }
}
