use std::{fmt, time::Duration};

use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};

use crate::audio::stream::AudioStreamer;

pub struct AudioSink {
    #[allow(dead_code)]
    stream_handle: OutputStream,
    sink: Sink,
}

#[allow(dead_code)]
impl AudioSink {
    pub fn play(&self, stream: AudioStreamer) {
        let source = Decoder::try_from(stream).expect("Decoding audio failed!");
        self.sink.clear();
        self.sink.play();

        self.sink.append(source);
    }

    pub fn pause(&self) {
        self.sink.pause();
    }

    pub fn resume(&self) {
        self.sink.play();
    }

    pub fn is_paused(&self) -> bool {
        self.sink.empty() || self.sink.is_paused()
    }

    pub fn position(&self) -> Duration {
        if !self.sink.empty() {
            self.sink.get_pos()
        } else {
            Duration::ZERO
        }
    }

    pub fn set_volume(&self, value: f64) {
        self.sink.set_volume(value as f32);
    }

    pub fn get_volume(&self) -> f64 {
        self.sink.volume() as f64
    }

    pub fn sleep(&self) {
        self.sink.sleep_until_end();
    }
}

impl Default for AudioSink {
    fn default() -> Self {
        let stream_handle = OutputStreamBuilder::open_default_stream().unwrap();
        let sink = Sink::connect_new(stream_handle.mixer());

        sink.set_volume(0.1);

        Self {
            stream_handle,
            sink,
        }
    }
}

impl fmt::Debug for AudioSink {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        Ok(())
    }
}
