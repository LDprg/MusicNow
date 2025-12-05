use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct SearchRelease {
    pub created: DateTime<Utc>,
    pub count: usize,
    pub offset: usize,
    pub releases: Vec<Release>,
}

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct Release {
    pub id: Uuid,
    pub score: u32,
    pub title: String,
    #[serde(rename = "artist-credit")]
    pub artist_credit: Vec<ArtistCredit>,
}

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct ArtistCredit {
    pub name: String,
}
