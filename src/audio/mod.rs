use crate::prelude::*;

use dioxus::logger::tracing::info;
use serde_json::Value;
use tokio::task::spawn_blocking;

mod sink;
mod stream;

use sink::*;
use stream::*;

#[allow(unused)]
pub async fn run_audio(client_id: String) -> Result<()> {
    let resp = reqwest::get(format!("https://api-v2.soundcloud.com/media/soundcloud:tracks:1301000134/4d4ac9de-2dcd-440d-ab81-2e2a7d76282b/stream/hls?client_id={}", client_id)).await?;
    let url: Value = resp.json().await?;
    let url = url.get("url").unwrap().as_str().unwrap().to_string();

    info!("Url: {}", url);

    let resp = reqwest::get(url).await?;
    let stream = resp.bytes().await?;

    let playlist =
        m3u8_rs::parse_media_playlist_res(&stream).map_err(|e| anyhow!(e.to_string()))?;

    let stream = AudioStreamer::default();
    let music_player = spawn_blocking({
        let stream = stream.clone();
        move || -> Result<()> { play_music(stream) }
    });

    for segment in playlist.segments {
        if let Some(map) = &segment.map {
            info!("Download Segment Map!");
            let resp = reqwest::get(&map.uri).await?;
            let data = resp.bytes().await?;

            stream.append(&data);
        }

        info!("Download Segment!");
        let resp = reqwest::get(&segment.uri).await?;
        let data = resp.bytes().await?;
        stream.append(&data);
    }

    stream.finish();

    music_player.await??;

    Ok(())
}

fn play_music(stream: AudioStreamer) -> Result<()> {
    let sink = AudioSink::default();

    sink.play(stream)?;

    info!("Wait");

    sink.sleep();

    info!("End");

    Ok(())
}
