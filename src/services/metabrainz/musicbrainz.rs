use crate::prelude::*;

use std::sync::LazyLock;

use chrono::{DateTime, Utc};
use dioxus::prelude::error;
use format_serde_error::SerdeError;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use uuid::Uuid;

const MB_URL: &str = "https://musicbrainz.org";

static SINGLETON_MUSIC: LazyLock<MusicBrainzApi> = LazyLock::new(MusicBrainzApi::new);

pub trait SearchType {
    fn type_str() -> &'static str;
    fn type_string() -> String {
        Self::type_str().to_string()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct SearchReleaseApi<T> {
    pub created: DateTime<Utc>,
    pub count: usize,
    pub offset: usize,
    pub releases: Vec<T>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct ReleaseApi {
    pub id: Uuid,
    pub score: u32,
    pub title: String,
    #[serde(rename = "artist-credit")]
    pub artist_credit: Vec<ArtistCreditApi>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct ArtistCreditApi {
    pub name: String
}

impl SearchType for ReleaseApi {
    fn type_str() -> &'static str {
        "release"
    }
}


#[derive(Clone, Debug)]
pub struct MusicBrainzApi {
    client: reqwest::Client,
}

impl MusicBrainzApi {
    fn new() -> Self {
        let client = reqwest::Client::builder()
            .user_agent(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:136.0) Gecko/20100101 Firefox/136.0",
            )
            .build()
            .unwrap();

        Self { client }
    }

    pub async fn search<T: SearchType + DeserializeOwned>(
        &self,
        query: String,
        limit: usize,
        offset: usize,
    ) -> Result<SearchReleaseApi<T>> {
        let resp = self
            .client
            .get(format!("{}{}{}", MB_URL, "/ws/2/", T::type_string()))
            .query(&[
                ("query", query),
                ("limit", limit.to_string()),
                ("offset", offset.to_string()),
                ("fmt", "json".to_string()),
            ])
            .send()
            .await
            .unwrap();

        let text = resp.text().await?;
        let json = serde_json::from_str::<SearchReleaseApi<T>>(&text).map_err(|err| SerdeError::new(text, err));

        match json {
            Err(err) => {
                error!("Error decoding json: {}", err);
                Err(anyhow!(err))
            }
            Ok(json) => Ok(json),
        }
    }
}

impl Default for MusicBrainzApi {
    fn default() -> Self {
        SINGLETON_MUSIC.clone()
    }
}
