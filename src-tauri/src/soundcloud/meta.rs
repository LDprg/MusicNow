use serde::Deserialize;
use tauri_plugin_http::reqwest::Url;

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct Search {
    pub collection: Vec<SearchElement>,
    pub total_results: u64,
    pub next_href: Option<Url>,
    pub query_urn: String,
}

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum SearchElement {
    Track(Box<SearchTrack>),
    User(Box<SearchUser>),
    Playlist(Box<SearchPlaylist>),
}

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct SearchTrack {
    pub artwork_url: Option<Url>,
    pub id: u64,
    pub title: String,
    pub urn: String,
    pub user_id: u64,
    pub publisher_metadata: Option<PublisherMetadata>,
    pub user: Option<SearchUser>,
}

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct SearchUser {
    pub avatar_url: Url,
    pub id: u64,
    pub full_name: String,
    pub urn: String,
    pub username: String,
}

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct SearchPlaylist {
    pub artwork_url: Option<Url>,
    pub id: u64,
    pub title: String,
    pub user_id: u64,
}

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct PublisherMetadata {
    pub id: u64,
    pub artist: Option<String>,
    pub album_title: Option<String>,
}

#[allow(dead_code)]
pub type Tracks = Vec<TrackElement>;

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct TrackElement {
    pub artwork_url: Url,
    pub id: u64,
    pub title: String,
    pub user_id: u64,
    pub media: TrackMedia,
    pub publisher_metadata: Option<PublisherMetadata>,
    pub user: Option<SearchUser>,
}

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct TrackMedia {
    pub transcodings: Vec<TrackTranscode>,
}

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct TrackTranscode {
    pub url: Url,
    pub preset: String,
    pub duration: u64,
    pub format: TrackTranscodeFormat,
    pub is_legacy_transcoding: bool,
}

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct TrackTranscodeFormat {
    pub protocol: String,
    pub mime_type: String,
}
