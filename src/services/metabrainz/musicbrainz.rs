use crate::prelude::*;

use std::sync::LazyLock;

use chrono::{DateTime, Utc};
use dioxus::prelude::error;
use serde::{Deserialize, Serialize};
use strum::Display;
use uuid::Uuid;

const MB_URL: &str = "https://musicbrainz.org";

static SINGLETON_MUSIC: LazyLock<MusicBrainzApi> = LazyLock::new(MusicBrainzApi::new);

#[derive(Display)]
#[strum(serialize_all = "lowercase")]
pub enum SearchType {
    Release,
    Artist,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct SearchReleaseApi {
    created : DateTime<Utc>,
    count: usize,
    offset: usize,
    releases: Vec<ReleaseApi>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct ReleaseApi {
    id: Uuid,
    score: u32,
    title: String,
}

#[derive(Clone, Debug)]
pub struct MusicBrainzApi {
    client: reqwest::Client,
}

impl MusicBrainzApi {
    fn new() -> Self {
        let client = reqwest::Client::builder().build().unwrap();

        Self { client }
    }

    pub async fn search(&self, typ: SearchType, query: String, limit: u64, offset: u64) {
        let resp = self
            .client
            .get(format!("{}{}{}", MB_URL, "/ws/2/", typ.to_string()))
            .query(&[
                ("query", query),
                ("limit", limit.to_string()),
                ("offset", offset.to_string()),
                ("fmt", "json".to_string()),
            ])
            .send()
            .await
            .unwrap();

        error!("{:#?}", resp.text().await);
    }
}

impl Default for MusicBrainzApi {
    fn default() -> Self {
        SINGLETON_MUSIC.clone()
    }
}
