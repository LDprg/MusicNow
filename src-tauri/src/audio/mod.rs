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

type SinkFunc = Box<dyn FnOnce(&AudioSink) + Send + 'static>;

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
                tx.send(Box::new(move |sink| {
                    sink.play(stream_task);
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

fn audio_service(rx: mpsc::Receiver<SinkFunc>) {
    let sink = AudioSink::default();

    loop {
        match rx.recv() {
            Ok(cmd) => cmd(&sink),
            Err(err) => unreachable!("Audio RX Error: {}", err),
        }
    }
}
