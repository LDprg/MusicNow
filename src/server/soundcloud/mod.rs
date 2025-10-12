use crate::prelude::*;

use regex::Regex;
use reqwest::Url;
use scraper::{Html, Selector};
use serde::Deserialize;
use std::sync::{Arc, RwLock};

pub struct SoundCloudApi {
    client: reqwest::Client,
    inner: Arc<RwLock<SoundCloudApiInner>>,
}

struct SoundCloudApiInner {
    client_id: Option<String>,
}

const SC_URL: &str = "https://soundcloud.com";
const SC_API_URL: &str = "https://api-v2.soundcloud.com";

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct SearchApi {
    pub collection: Vec<SearchElementApi>,
    pub total_results: u64,
    pub next_href: Url,
    pub query_urn: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum SearchElementApi {
    Track(SearchTrackApi),
    User(SearchUserApi),
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct SearchTrackApi {
    pub artwork_url: Url,
    pub id: u64,
    pub title: String,
    pub urn: String,
    pub user_id: u64,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct SearchUserApi {
    pub avatar_url: Url,
    pub id: u64,
    pub full_name: String,
    pub urn: String,
}

impl SoundCloudApi {
    pub async fn login_anonymous(&self) -> Result<()> {
        self.inner.write().unwrap().client_id = Some(self.fetch_client_id().await?);
        Ok(())
    }

    pub fn get_client_id(&self) -> Result<String> {
        self.inner.read().unwrap().client_id.clone().ok_or(anyhow!(
            "Client needs to login first before accessing client id!"
        ))
    }

    async fn fetch_client_id(&self) -> Result<String> {
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
        let client_id = client_id.get(..pos).unwrap();

        info!("{}", client_id);

        Ok(client_id.to_string())
    }

    pub async fn search(&self, query: &str, limit: usize, offset: usize) -> Result<SearchApi> {
        let resp = self
            .client
            .get(format!("{}{}", SC_API_URL, "/search"))
            .query(&[
                ("q", query),
                ("client_id", self.get_client_id()?.as_str()),
                ("limit", limit.to_string().as_str()),
                ("offset", offset.to_string().as_str()),
            ])
            .send()
            .await?;

        Ok(resp.json::<SearchApi>().await?)
    }
}

impl Default for SoundCloudApi {
    fn default() -> Self {
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
}
