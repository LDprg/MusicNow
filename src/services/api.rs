use std::time::Duration;

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

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientEvent {
    GetPostion,
    Play(u64),
    Pause,
    Resume,
    GetPaused,
    SetVolume(f64),
    GetVolume,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerEvent {
    Position(Duration),
    Duration(Duration),
    Volume(f64),
    IsPaused(bool),
}

#[get("/api/status")]
pub async fn status_ws(options: WebSocketOptions) -> Result<Websocket<ClientEvent, ServerEvent>> {
    Ok(options.on_upgrade(move |mut socket| async move {
        let player = AudioPlayer::default();

        // Loop and echo back uppercase messages
        while let Ok(msg) = socket.recv().await {
            let err : Result<()>  = async {
                match msg {
                    ClientEvent::GetPostion => {
                        socket
                            .send(ServerEvent::Position(player.position()))
                            .await?;
                        Ok(())
                    }
                    ClientEvent::Play(track_id) => {
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

                        player.play(data).await?;
                        socket.send(ServerEvent::IsPaused(false)).await?;
                        socket
                            .send(ServerEvent::Duration(player.duration()))
                            .await?;
                        socket
                            .send(ServerEvent::Position(player.position()))
                            .await?;
                        Ok(())
                    }
                    ClientEvent::Pause => {
                        player.pause();
                        socket.send(ServerEvent::IsPaused(true)).await?;
                        socket
                            .send(ServerEvent::Position(player.position()))
                            .await?;
                        Ok(())
                    }
                    ClientEvent::Resume => {
                        player.resume();
                        socket.send(ServerEvent::IsPaused(false)).await?;
                        socket
                            .send(ServerEvent::Position(player.position()))
                            .await?;
                        Ok(())
                    }
                    ClientEvent::SetVolume(value) => {
                        player.set_volume(value);
                        socket.send(ServerEvent::Volume(value)).await?;
                        Ok(())
                    }
                    ClientEvent::GetVolume => {
                        let value = player.get_volume();
                        socket.send(ServerEvent::Volume(value)).await?;
                        Ok(())
                    }
                    ClientEvent::GetPaused => {
                        let value = player.is_paused();
                        socket.send(ServerEvent::IsPaused(value)).await?;
                        Ok(())
                    }
                }
            }.await;

            if let Err(err) = err {
                error!("{}", err);
            }
        }
    }))
}
