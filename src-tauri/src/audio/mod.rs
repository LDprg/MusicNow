// TODO: Rewrite all of the audio backend
use std::{
    error::Error,
    sync::{Arc, mpsc},
    thread::JoinHandle,
};

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

type RawFunc = Box<dyn Fn(Arc<AudioSink>) + Send + 'static>;

#[derive(Debug)]
pub struct AudioPlayer {
    audio_thread: JoinHandle<()>,
    tx: mpsc::Sender<RawFunc>,
}

impl Default for AudioPlayer {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        let audio_thread = std::thread::spawn(move || audio_service(rx));

        Self { audio_thread, tx }
    }
}

impl AudioPlayer {
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
            .map_err(|e| AudioError::M3u8Error(e.to_string()))?;

        info!("Starting playback");

        let segments = playlist.segments.clone();
        let tx = self.tx.clone();
        tauri::async_runtime::spawn(async move {
            let err: Result<(), Box<dyn Error>> = async move {
                let stream = AudioStreamer::default();

                info!("Starting audio");
                let stream_task = stream.clone();
                tx.send(Box::new(move |sink: Arc<AudioSink>| {
                    let stream = stream_task.clone();
                    sink.play(stream);
                }))
                .unwrap();

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
}

fn audio_service(rx: mpsc::Receiver<RawFunc>) {
    let sink = Arc::new(AudioSink::default());

    while let Ok(cmd) = rx.recv() {
        cmd(sink.clone());
    }
}
