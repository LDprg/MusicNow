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

#[get("/api/player/is_paused")]
pub async fn is_player() -> Result<bool> {
    let player = AudioPlayer::default();

    Ok(player.is_paused())
}

#[get("/api/player/progress")]
pub async fn progress() -> Result<f64> {
    let player = AudioPlayer::default();

    player.progress()
}
