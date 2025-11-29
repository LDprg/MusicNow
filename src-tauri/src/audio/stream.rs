// TODO: Rewrite this (seeking doesn't work at all)
use log::info;
use rodio::Decoder;
use rodio::decoder::DecoderError;
use std::{
    io::{self, Read, Seek, SeekFrom},
    sync::{
        Arc, Mutex,
        mpsc::{self, channel},
    },
};

#[derive(Clone)]
pub struct AudioStreamer {
    inner: Arc<Mutex<AudioStreamerInner>>,
    tx: Arc<Mutex<Option<mpsc::Sender<Vec<u8>>>>>,
}

struct AudioStreamerInner {
    buffer: Vec<u8>,
    pos: usize,
    rx: mpsc::Receiver<Vec<u8>>,
}

impl Default for AudioStreamer {
    fn default() -> Self {
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
}

impl AudioStreamer {
    pub async fn append(&self, new_data: &[u8]) {
        let tx = self.tx.lock().unwrap();
        if let Some(tx) = tx.as_ref() {
            tx.send(new_data.to_vec()).unwrap();
        }
    }

    pub async fn finish(&self) {
        self.tx.lock().unwrap().take();
    }
}

impl Read for AudioStreamer {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut inner = self
            .inner
            .lock()
            .map_err(|_| std::io::ErrorKind::Deadlock)?;

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
        let mut inner = self
            .inner
            .lock()
            .map_err(|_| std::io::ErrorKind::Deadlock)?;
        let new_pos = match pos {
            SeekFrom::Start(off) => off as i64,
            SeekFrom::End(off) => inner.buffer.len() as i64 + off,
            SeekFrom::Current(off) => inner.pos as i64 + off,
        };

        info!("Seek: {:?}, {} to {}", pos, inner.pos, new_pos);

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
