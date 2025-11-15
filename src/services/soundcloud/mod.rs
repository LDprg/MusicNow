mod meta;

use crate::prelude::*;
pub use self::meta::*;

use bytes::Bytes;
use format_serde_error::SerdeError;
use regex::Regex;
use reqwest::Url;
use scraper::{Html, Selector};
use serde_json::Value;
use std::sync::{Arc, LazyLock, RwLock};

#[derive(Clone, Debug)]
pub struct SoundCloudApi {
    client: reqwest::Client,
    inner: Arc<RwLock<SoundCloudApiInner>>,
}

#[derive(Debug)]
struct SoundCloudApiInner {
    client_id: Option<String>,
}

const SC_URL: &str = "https://soundcloud.com";
const SC_API_URL: &str = "https://api-v2.soundcloud.com";

static SINGLETON_API: LazyLock<SoundCloudApi> = LazyLock::new(SoundCloudApi::new);

impl SoundCloudApi {
    fn new() -> Self {
        let client = reqwest::Client::builder()
            .user_agent(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:136.0) Gecko/20100101 Firefox/136.0",
            )
            .build()
            .unwrap();

        Self {
            client,
            inner: Arc::new(RwLock::new(SoundCloudApiInner { client_id: None })),
        }
    }

    pub async fn login_anonymous(&self) -> Result<()> {
        let resp = self.client.get(SC_URL).send().await?;
        let body = resp.text().await?;

        let site = Html::parse_document(&body);
        let script = Selector::parse("script[src]").map_err(|e| anyhow!(e.to_string()))?;
        let script_regex = Regex::new(r"https://.*/assets/0.*.js")?;

        let mut script_src = "";

        for element in site.select(&script) {
            if let Some(src) = element.value().attr("src")
                && script_regex.is_match(src)
            {
                script_src = src;
            }
        }

        info!("{}", script_src);

        let resp = self.client.get(script_src).send().await?;
        let body = resp.text().await?;

        let start = "\"client_id=";
        let end = "\"";

        let pos = body.find(start).unwrap();
        let client_id = body.get(pos + start.len()..).unwrap();

        let pos = client_id.find(end).unwrap();
        let client_id = client_id.get(..pos).unwrap().to_string();

        info!("{}", client_id);

        self.inner.write().unwrap().client_id = Some(client_id);
        Ok(())
    }

    pub fn get_client_id(&self) -> Result<String> {
        self.inner.read().unwrap().client_id.clone().ok_or(anyhow!(
            "Client needs to login first before accessing client id!"
        ))
    }

    #[allow(dead_code)]
    pub async fn search(&self, query: String, limit: usize, offset: usize) -> Result<SearchApi> {
        let resp = self
            .client
            .get(format!("{}{}", SC_API_URL, "/search"))
            .query(&[
                ("q", query),
                ("client_id", self.get_client_id()?),
                ("limit", limit.to_string()),
                ("offset", offset.to_string()),
            ])
            .send()
            .await?;

        let text = resp.text().await?;
        let json =
            serde_json::from_str::<SearchApi>(&text).map_err(|err| SerdeError::new(text, err));

        match json {
            Err(err) => {
                error!("Error decoding json: {}", err);
                Err(anyhow!(err))
            }
            Ok(json) => Ok(json),
        }
    }

    #[allow(dead_code)]
    pub async fn tracks(&self, track_id: u64) -> Result<TracksApi> {
        let resp = self
            .client
            .get(format!("{}{}", SC_API_URL, "/tracks"))
            .query(&[
                ("ids", track_id.to_string()),
                ("client_id", self.get_client_id()?),
            ])
            .send()
            .await?;

        let text = resp.text().await?;
        let json =
            serde_json::from_str::<TracksApi>(&text).map_err(|err| SerdeError::new(text, err));

        match json {
            Err(err) => {
                error!("Error decoding json: {}", err);
                Err(anyhow!(err))
            }
            Ok(json) => Ok(json),
        }
    }

    #[allow(dead_code)]
    pub async fn stream(&self, url: Url) -> Result<Bytes> {
        info!("Url from: {}", url);
        let resp = self
            .client
            .get(url)
            .query(&[("client_id", self.get_client_id()?)])
            .send()
            .await?;
        let url: Value = resp.json().await?;
        let url = url
            .get("url")
            .ok_or(anyhow!("Url missing"))?
            .as_str()
            .ok_or(anyhow!("Fatal failure"))?
            .to_string();

        info!("Data from: {}", url);
        let resp = self.client.get(url).send().await?;

        Ok(resp.bytes().await?)
    }
}

impl Default for SoundCloudApi {
    fn default() -> Self {
        SINGLETON_API.clone()
    }
}
