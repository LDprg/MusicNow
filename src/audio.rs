use std::{io::{self, Read, Seek, SeekFrom}, sync::{Arc, Mutex, mpsc::{self, channel}}};
use dioxus::prelude::*;
use rodio::{Decoder, Sink, OutputStreamBuilder};
use rodio::decoder::DecoderError;
use serde_json::Value;
use tokio::task::spawn_blocking;

#[derive(Clone)]
struct AudioStreamer {
    inner: Arc<Mutex<AudioStreamerInner>>,
    tx: Arc<Mutex<Option<mpsc::Sender<Vec<u8>>>>>,
}

struct AudioStreamerInner {
    buffer: Vec<u8>,
    pos: usize,
    rx: mpsc::Receiver<Vec<u8>>,
}

impl AudioStreamer {
    pub fn default() -> Self {
        let (tx, rx) = channel();

        AudioStreamer {
            inner: Arc::new(Mutex::new(AudioStreamerInner {
                buffer: Vec::new(),
                pos: 0,
                rx,
            })),
            tx: Arc::new(Mutex::new(Some(tx))),
        }
    }

    pub fn append(&self, new_data: &[u8]) {
        let tx = self.tx.lock().unwrap();
        if let Some(tx) = tx.as_ref() {
            tx.send(new_data.to_vec()).unwrap();
        }
    }

    pub fn finish(&self) {
        self.tx.lock().unwrap().take();
    }
}

impl Read for AudioStreamer {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut inner = self.inner.lock().unwrap();

        while let Ok(data) = inner.rx.try_recv() {
            inner.buffer.extend(data);
        }

        while inner.pos >= inner.buffer.len() {
            match inner.rx.recv() {
                Ok(data) => inner.buffer.extend(data),
                Err(_) => return Ok(0),
            }
        }

        let end = buf.len().min(inner.buffer.len() - inner.pos);
        buf[..end].copy_from_slice(&inner.buffer[inner.pos..inner.pos + end]);
        inner.pos += end;
        Ok(end)
    }
}

// TODO: Seeking only works within the Buffer
impl Seek for AudioStreamer {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let mut inner = self.inner.lock().unwrap();
        let new_pos = match pos {
            SeekFrom::Start(off) => off as i64,
            SeekFrom::End(off) => inner.buffer.len() as i64 + off,
            SeekFrom::Current(off) => inner.pos as i64 + off,
        };

        println!("Seek: {:?}, {} to {}", pos, inner.pos, new_pos);

        if new_pos < 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid seek"));
        }

        inner.pos = new_pos as usize;
        Ok(inner.pos as u64)
    }
}

impl TryFrom<AudioStreamer> for Decoder<AudioStreamer> {
    type Error = DecoderError;

    fn try_from(data: AudioStreamer) -> Result<Self, Self::Error> {
        Self::new(data)
    }
}

#[allow(unused)]
pub async fn run_audio(client_id : String) -> Result<(), ServerFnError>{
    let resp = reqwest::get(format!("https://api-v2.soundcloud.com/media/soundcloud:tracks:1301000134/4d4ac9de-2dcd-440d-ab81-2e2a7d76282b/stream/hls?client_id={}", client_id)).await?;
    let url: Value = resp.json().await?;
    let url = url.get("url").unwrap().as_str().unwrap().to_string();

    println!("Url: {}", url);

    let resp = reqwest::get(url).await?;
    let stream = resp.bytes().await?;

    let playlist =
        m3u8_rs::parse_media_playlist_res(&stream).map_err(|e| ServerFnError::new(e.to_string()))?;

    let stream = AudioStreamer::default();
    let music_player = spawn_blocking({
        let stream = stream.clone();
        move || -> Result<(), ServerFnError> { play_music(stream) }
    });

    for segment in playlist.segments {
        if let Some(map) = &segment.map {
            println!("Download Segment Map!");
            let resp = reqwest::get(&map.uri).await?;
            let data = resp.bytes().await?;

            stream.append(&data);
        }

        println!("Download Segment!");
        let resp = reqwest::get(&segment.uri).await?;
        let data = resp.bytes().await?;
        stream.append(&data);
    }

    stream.finish();

    music_player.await??;

    Ok(())
}

fn play_music(cursor: AudioStreamer) -> Result<(), ServerFnError> {
    let stream_handle = OutputStreamBuilder::open_default_stream()?;
    let sink = Sink::connect_new(stream_handle.mixer());

    let source = Decoder::try_from(cursor)?;
    sink.append(source);

    println!("Wait");

    sink.sleep_until_end();

    println!("End");

    Ok(())
}
