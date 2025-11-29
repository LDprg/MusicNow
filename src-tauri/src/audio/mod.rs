// TODO: Rewrite all of the audio backend
use std::{error::Error, sync::LazyLock, thread, time::Duration};

use bytes::Bytes;
use log::{error, info, warn};

mod error;
mod sink;
mod stream;

use sink::*;
use stream::*;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_http::reqwest;

use crate::{
    audio::error::AudioError,
    soundcloud::{Soundcloud, meta},
};

static SINGLETON_PLAYER: LazyLock<AudioPlayer> = LazyLock::new(AudioPlayer::new);

#[derive(Clone, Debug)]
pub struct AudioPlayer {
    tx: std::sync::mpsc::Sender<AudioPlayerCommands>,
    pub is_playing: tokio::sync::watch::Receiver<bool>,
    pub position: tokio::sync::watch::Receiver<Duration>,
    pub duration: tokio::sync::watch::Receiver<Duration>,
    duration_tx: tokio::sync::watch::Sender<Duration>,
    pub volume: tokio::sync::watch::Receiver<f64>,
}

#[derive(Debug)]
#[allow(dead_code)]
enum AudioPlayerCommands {
    Play(AudioStreamer),
    Pause,
    Resume,
    IsPlaying,
    Position,
    SetVolume(f64),
    Volume,
}

impl AudioPlayer {
    fn new() -> Self {
        let (sync_tx, sync_rx) = std::sync::mpsc::channel::<AudioPlayerCommands>();
        let is_playing = tokio::sync::watch::channel::<bool>(false);
        let position = tokio::sync::watch::channel::<Duration>(Duration::ZERO);
        let duration = tokio::sync::watch::channel::<Duration>(Duration::ZERO);
        let volume = tokio::sync::watch::channel::<f64>(50.0);

        thread::spawn(move || audio_task(sync_rx, is_playing.0, position.0, volume.0));

        Self {
            tx: sync_tx,
            is_playing: is_playing.1,
            position: position.1,
            duration: duration.1,
            duration_tx: duration.0,
            volume: volume.1,
        }
    }

    #[allow(dead_code)]
    pub async fn play(
        &self,
        soundcloud: State<'_, Soundcloud>,
        track_id: u64,
    ) -> Result<(), AudioError> {
        info!("Fetch data for: {}", track_id);
        let api = soundcloud;

        let tracks = api.tracks(track_id).await?;
        let track = tracks.first().ok_or(AudioError::TrackNotFound)?;

        // TODO: Implement more formats
        let transcodes: Vec<meta::TrackTranscode> = track
            .media
            .transcodings
            .clone()
            .into_iter()
            .filter(|x| x.format.protocol == "hls")
            .collect();
        let transcode = transcodes.first().ok_or({
            if track.media.transcodings.is_empty() {
                AudioError::MediaNotFound
            } else {
                AudioError::FormatIncompatible
            }
        })?;

        if transcode.is_legacy_transcoding {
            warn!("Legacy format detected!");
        }

        info!("Format {:?} with {}", transcode.format, transcode.preset);

        let stream = api.stream(transcode.url.clone()).await?;

        info!("Parsing m3u8");

        let playlist = m3u8_rs::parse_media_playlist_res(&stream)
            .map_err(|e| AudioError::M3U8Error(e.to_string()))?;

        info!("Starting playback");

        let duration: f32 = playlist.segments.iter().map(|i| i.duration).sum();
        self.duration_tx
            .send(Duration::from_secs_f32(duration))
            .unwrap();

        let segments = playlist.segments.clone();
        let tx = self.tx.clone();
        tauri::async_runtime::spawn(async move {
            let err: Result<(), Box<dyn Error>> = async move {
                let stream = AudioStreamer::default();

                info!("Starting audio");
                tx.send(AudioPlayerCommands::Play(stream.clone())).unwrap();
                tx.send(AudioPlayerCommands::Resume).unwrap();
                tx.send(AudioPlayerCommands::Position).unwrap();
                tx.send(AudioPlayerCommands::IsPlaying).unwrap();

                info!("Starting stream");

                for segment in segments {
                    if let Some(map) = &segment.map {
                        let resp = reqwest::get(&map.uri).await?;
                        let data: Bytes = resp.bytes().await?;

                        stream.append(&data).await;
                    }

                    let resp = reqwest::get(&segment.uri).await?;
                    let data: Bytes = resp.bytes().await?;
                    stream.append(&data).await;
                }

                stream.finish().await;

                info!("Stream finished");

                Ok(())
            }
            .await;

            if let Err(e) = err {
                error!("{:#?}", e);
            }
        });

        Ok(())
    }

    pub fn pause(&self) {
        self.tx.send(AudioPlayerCommands::Pause).unwrap();
        self.tx.send(AudioPlayerCommands::Position).unwrap();
        self.tx.send(AudioPlayerCommands::IsPlaying).unwrap();
    }

    pub fn resume(&self) {
        self.tx.send(AudioPlayerCommands::Resume).unwrap();
        self.tx.send(AudioPlayerCommands::Position).unwrap();
        self.tx.send(AudioPlayerCommands::IsPlaying).unwrap();
    }

    pub fn update_postion(&self) {
        self.tx.send(AudioPlayerCommands::Position).unwrap();
    }

    pub fn set_volume(&self, value: f64) {
        self.tx.send(AudioPlayerCommands::SetVolume(value)).unwrap();
        self.tx.send(AudioPlayerCommands::Volume).unwrap();
    }
}

impl Default for AudioPlayer {
    fn default() -> Self {
        SINGLETON_PLAYER.clone()
    }
}

// Audio on Web and Android is single threaded
fn audio_task(
    rx: std::sync::mpsc::Receiver<AudioPlayerCommands>,
    is_playing: tokio::sync::watch::Sender<bool>,
    position: tokio::sync::watch::Sender<Duration>,
    volume: tokio::sync::watch::Sender<f64>,
) {
    let sink = AudioSink::default();

    while let Ok(cmd) = rx.recv() {
        match cmd {
            AudioPlayerCommands::Play(stream) => sink.play(stream),
            AudioPlayerCommands::Pause => sink.pause(),
            AudioPlayerCommands::Resume => sink.resume(),
            AudioPlayerCommands::IsPlaying => is_playing.send(!sink.is_paused()).unwrap(),
            AudioPlayerCommands::Position => position.send(sink.position()).unwrap(),
            AudioPlayerCommands::SetVolume(value) => sink.set_volume(value / 500.0),
            AudioPlayerCommands::Volume => volume.send(sink.get_volume() * 500.0).unwrap(),
        };
    }
}
