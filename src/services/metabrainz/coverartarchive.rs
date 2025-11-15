use crate::prelude::*;

use std::sync::LazyLock;

use format_serde_error::SerdeError;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const CAA_URL: &str = "https://coverartarchive.org";

static SINGLETON_MUSIC: LazyLock<CoverArtArchiveApi> = LazyLock::new(CoverArtArchiveApi::new);

#[derive(Clone, Debug)]
pub struct CoverArtArchiveApi {
    client: reqwest::Client,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct ImagesApi {
    pub images: Vec<ImageApi>,
    pub release: Url,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct ImageApi {
    pub id: u64,
    pub back: bool,
    pub front: bool,
    pub image: Url,
    pub thumbnails: ThumbnailApi,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct ThumbnailApi {
    #[serde(rename = "250")]
    pub s250: Url,
    #[serde(rename = "500")]
    pub s500: Url,
    #[serde(rename = "1200")]
    pub s1200: Url,
}

impl CoverArtArchiveApi {
    fn new() -> Self {
        let client = reqwest::Client::builder()
            .user_agent(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:136.0) Gecko/20100101 Firefox/136.0",
            )
            .build()
            .unwrap();

        Self { client }
    }

    pub async fn fetch_image(&self, mbid: Uuid) -> Result<ImageApi> {
        let resp = self
            .client
            .get(format!("{}{}{}", CAA_URL, "/release/", mbid))
            .send()
            .await
            .unwrap();

        let text = resp.text().await?;
        let json =
            serde_json::from_str::<ImagesApi>(&text).map_err(|err| SerdeError::new(text, err));

        match json {
            Err(err) => {
                error!("Error decoding json: {}", err);
                Err(anyhow!(err))
            }
            Ok(json) => {
                if let Some(image) = json.images.first() {
                    Ok(image.clone())
                } else {
                    Err(anyhow!("No image found"))
                }
            }
        }
    }
}

impl Default for CoverArtArchiveApi {
    fn default() -> Self {
        SINGLETON_MUSIC.clone()
    }
}
