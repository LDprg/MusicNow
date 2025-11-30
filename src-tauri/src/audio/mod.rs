// TODO: Rewrite all of the audio backend
use std::{error::Error, sync::mpsc};

use bytes::Bytes;
use log::{error, info, warn};

mod error;
mod sink;
mod stream;

use sink::*;
use stream::*;
use tauri::State;
use tauri_plugin_http::reqwest;

use crate::{
    audio::error::AudioError,
    soundcloud::{Soundcloud, meta},
};

type SinkFunc = Box<dyn FnOnce(&AudioSink) -> Result<(), AudioError> + Send + 'static>;

#[derive(Debug)]
pub struct AudioPlayer {
    tx: mpsc::Sender<SinkFunc>,
}

impl Default for AudioPlayer {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();

        std::thread::spawn(move || audio_service(rx));

        Self { tx }
    }
}

impl AudioPlayer {
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
            .map_err(|e| AudioError::M3u8Error(e.to_string()))?;

        info!("Starting playback");

        let segments = playlist.segments.clone();
        let tx = self.tx.clone();
        tauri::async_runtime::spawn(async move {
            let err: Result<(), Box<dyn Error>> = async move {
                let (audio_tx, audio_rx) = audio_channel();

                info!("Starting audio");
                tx.send(Box::new(move |sink| {
                    sink.play(audio_rx)?;
                    Ok(())
                }))
                .unwrap();

                info!("Starting stream");

                for segment in segments {
                    if let Some(map) = &segment.map {
                        let resp = reqwest::get(&map.uri).await?;
                        let data: Bytes = resp.bytes().await?;

                        audio_tx.append(&data).await;
                    }

                    let resp = reqwest::get(&segment.uri).await?;
                    let data: Bytes = resp.bytes().await?;
                    audio_tx.append(&data).await;
                }

                audio_tx.finish().await;

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
        self.tx
            .send(Box::new(move |sink| {
                sink.pause();
                Ok(())
            }))
            .unwrap();
    }

    pub fn resume(&self) {
        self.tx
            .send(Box::new(move |sink| {
                sink.resume();
                Ok(())
            }))
            .unwrap();
    }

    pub async fn is_playing(&self) -> bool {
        let (tx, rx) = tokio::sync::oneshot::channel::<bool>();
        self.tx
            .send(Box::new(move |sink| {
                tx.send(!sink.is_paused()).unwrap();
                Ok(())
            }))
            .unwrap();

        rx.await.unwrap()
    }

    pub fn set_volume(&self, volume: f64) {
        self.tx
            .send(Box::new(move |sink| {
                sink.set_volume(volume / 100.0);
                Ok(())
            }))
            .unwrap();
    }

    pub async fn get_volume(&self) -> f64 {
        let (tx, rx) = tokio::sync::oneshot::channel::<f64>();
        self.tx
            .send(Box::new(move |sink| {
                tx.send(sink.get_volume() * 100.0).unwrap();
                Ok(())
            }))
            .unwrap();

        rx.await.unwrap()
    }
}

fn audio_service(rx: mpsc::Receiver<SinkFunc>) {
    let sink = AudioSink::default();

    loop {
        match rx.recv() {
            Ok(cmd) => {
                if let Err(err) = cmd(&sink) {
                    error!("Audio Thread Error: {:#?}", err);
                }
            }
            Err(err) => unreachable!("Audio Thread RX Error: {}", err),
        }
    }
}
