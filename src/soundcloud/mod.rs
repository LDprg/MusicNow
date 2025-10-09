use crate::prelude::*;

use regex::Regex;
use scraper::{Html, Selector};
use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

pub struct SoundCloudApi {
    client: reqwest::Client,
    inner: Arc<RwLock<SoundCloudApiInner>>,
}

struct SoundCloudApiInner {
    client_id: Option<String>,
}

impl SoundCloudApi {
    pub async fn login_anonymous(&self) -> Result<()> {
        let mut inner = self.inner.write().unwrap();
        inner.client_id = Some(self.fetch_client_id().await?);
        Ok(())
    }

    pub fn get_client_id(&self) ->Option<String> {
        let inner = self.inner.read().unwrap();

        inner.client_id.clone()
    } 

    async fn fetch_client_id(&self) -> Result<String> {
        let resp = self.client.get("https://soundcloud.com").send().await?;
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
