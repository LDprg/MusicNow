use reqwest::Url;
use serde::{Deserialize, Serialize};

// Search Api

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct SearchApi {
    pub collection: Vec<SearchElementApi>,
    pub total_results: u64,
    pub next_href: Option<Url>,
    pub query_urn: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum SearchElementApi {
    Track(SearchTrackApi),
    User(SearchUserApi),
    Playlist(SearchPlaylistApi),
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct SearchTrackApi {
    pub artwork_url: Option<Url>,
    pub id: u64,
    pub title: String,
    pub urn: String,
    pub user_id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct SearchUserApi {
    pub avatar_url: Url,
    pub id: u64,
    pub full_name: String,
    pub urn: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct SearchPlaylistApi {
    pub artwork_url: Option<Url>,
    pub id: u64,
    pub title: String,
    pub user_id: u64,
}

// Track Api

#[allow(dead_code)]
pub type TracksApi = Vec<TrackElementApi>;

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct TrackElementApi {
    pub artwork_url: Url,
    pub id: u64,
    pub title: String,
    pub user_id: u64,
    pub media: TrackMediaApi,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct TrackMediaApi {
    pub transcodings: Vec<TrackTranscodeApi>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct TrackTranscodeApi {
    pub url: Url,
    pub preset: String,
    pub duration: u64,
    pub is_legacy_transcoding: bool,
}
