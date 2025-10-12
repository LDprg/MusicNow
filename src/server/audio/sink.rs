use std::sync::Arc;

use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};

use super::*;

pub struct AudioSink {
    #[allow(dead_code)]
    stream_handle: Arc<OutputStream>,
    sink: Arc<Sink>,
}

#[allow(dead_code)]
impl AudioSink {
    pub fn play(&self, stream: AudioStreamer) -> Result<()> {
        let source = Decoder::try_from(stream)?;
        self.sink.append(source);
        Ok(())
    }

    pub fn sleep(&self) {
        self.sink.sleep_until_end();
    }
}

impl Default for AudioSink {
    fn default() -> Self {
        let stream_handle = Arc::new(OutputStreamBuilder::open_default_stream().unwrap());
        let sink = Arc::new(Sink::connect_new(stream_handle.mixer()));

        sink.set_volume(0.1);

        Self {
            stream_handle,
            sink,
        }
    }
}
