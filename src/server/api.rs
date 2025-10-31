use crate::prelude::*;

#[post("/api/search")]
pub async fn search(query: String, limit: usize, offset: usize) -> Result<SearchApi> {
    let api = SoundCloudApi::default();

    api.search(&query, limit, offset).await
}

#[post("/api/player/play")]
pub async fn play(track_id : u64) -> Result<()> {
    info!("Fetch data for: {}", track_id);
    let api = SoundCloudApi::default();

    let tracks = api.tracks(track_id).await?;
    let track = tracks.first().ok_or(anyhow!("No track found!"))?;
    let transcode = track.media.transcodings.first().ok_or(anyhow!("No playback found!"))?;
    let data = api.stream(transcode.url.clone()).await?;

    info!("Start playing: {}", track_id);
    let player = AudioPlayer::default();

    player.play(data).await
}
