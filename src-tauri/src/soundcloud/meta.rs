use serde::Deserialize;
use tauri_plugin_http::reqwest::Url;

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct SoundlcoudApiSearch {
    pub collection: Vec<SoundcloudApiSearchElement>,
    pub total_results: u64,
    pub next_href: Option<Url>,
    pub query_urn: String,
}

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum SoundcloudApiSearchElement {
    Track(Box<SoundcloudApiSearchTrack>),
    User(Box<SoundcloudApiSearchUser>),
    Playlist(Box<SoundcloudApiSearchPlaylist>),
}

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct SoundcloudApiSearchTrack {
    pub artwork_url: Option<Url>,
    pub id: u64,
    pub title: String,
    pub urn: String,
    pub user_id: u64,
    pub publisher_metadata: Option<SoundcloudApiPublisherMetadata>,
    pub user: Option<SoundcloudApiSearchUser>,
}

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct SoundcloudApiSearchUser {
    pub avatar_url: Url,
    pub id: u64,
    pub full_name: String,
    pub urn: String,
    pub username: String,
}

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct SoundcloudApiSearchPlaylist {
    pub artwork_url: Option<Url>,
    pub id: u64,
    pub title: String,
    pub user_id: u64,
}

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct SoundcloudApiPublisherMetadata {
    pub id: u64,
    pub artist: Option<String>,
    pub album_title: Option<String>,
}
