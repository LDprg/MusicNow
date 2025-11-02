use dioxus::fullstack::{WebSocketOptions, Websocket};
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[post("/api/search")]
pub async fn search(query: String, limit: usize, offset: usize) -> Result<SearchApi> {
    {
        let api = SoundCloudApi::default();

        api.search(&query, limit, offset).await
    }
    .inspect_err(|e| error!("{}", e))
}

#[post("/api/player/play")]
pub async fn play(track_id: u64) -> Result<()> {
    {
        info!("Fetch data for: {}", track_id);
        let api = SoundCloudApi::default();

        let tracks = api.tracks(track_id).await?;
        let track = tracks.first().ok_or(anyhow!("No track found!"))?;

        // TODO: Implement more formats
        let transcodes: Vec<TrackTranscodeApi> = track
            .media
            .transcodings
            .clone()
            .into_iter()
            .filter(|x| x.format.protocol == "hls")
            .collect();
        let transcode = transcodes.first().ok_or({
            if track.media.transcodings.is_empty() {
                anyhow!("No playback found!")
            } else {
                anyhow!("No compatible format found!")
            }
        })?;

        if transcode.is_legacy_transcoding {
            warn!("Legacy format detected!");
        }

        info!("Format {:?} with {}", transcode.format, transcode.preset);

        let data = api.stream(transcode.url.clone()).await?;

        info!("Start playing: {}", track_id);
        let player = AudioPlayer::default();

        player.play(data).await
    }
    .inspect_err(|e| error!("{}", e))
}

#[post("/api/player/pause")]
pub async fn pause() -> Result<()> {
    let player = AudioPlayer::default();
    player.pause();

    Ok(())
}

#[post("/api/player/resume")]
pub async fn resume() -> Result<()> {
    let player = AudioPlayer::default();
    player.pause();

    Ok(())
}

// Events flowing *from* the client to the server
#[derive(Serialize, Deserialize, Debug)]
pub enum ClientEvent {
    UpdateStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    pub is_paused: bool,
    pub progress: Option<f64>,
}

// Events flowing *to* the client from the server
#[derive(Serialize, Deserialize, Debug)]
pub enum ServerEvent {
    Status(Status),
}

#[get("/api/status")]
pub async fn status_ws(options: WebSocketOptions) -> Result<Websocket<ClientEvent, ServerEvent>> {
    Ok(options.on_upgrade(move |mut socket| async move {
        let player = AudioPlayer::default();

        // Loop and echo back uppercase messages
        while let Ok(msg) = socket.recv().await {
            match msg {
                ClientEvent::UpdateStatus => {
                    _ = socket.send(ServerEvent::Status(Status{
                        is_paused: player.is_paused(),
                        progress: player.progress().inspect_err(|e| error!("{}", e)).ok(),
                    })).await;
                }
            };
        }
    }))
}
