pub mod error;
pub mod meta;

use error::*;

use log::info;
use regex::Regex;
use scraper::{Html, Selector};
use serde::Deserialize;
use serde::de::DeserializeOwned;
use tauri_plugin_http::reqwest;
use tauri_plugin_http::reqwest::Url;

const SC_URL: &str = "https://soundcloud.com";
const SC_API_URL: &str = "https://api-v2.soundcloud.com";

pub struct Soundcloud {
    client: reqwest::Client,
    client_id: Option<String>,
}

impl Default for Soundcloud {
    fn default() -> Self {
        let client = reqwest::Client::builder()
            .user_agent(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:136.0) Gecko/20100101 Firefox/136.0",
            )
            .build()
            .unwrap();

        Self {
            client,
            client_id: None,
        }
    }
}

impl Soundcloud {
    fn create_req(&self, path: &str) -> Result<reqwest::RequestBuilder, SoundcloudError> {
        let client_id = self
            .client_id
            .as_ref()
            .ok_or(SoundcloudError::LoginDataMissing)?;

        let req = self
            .client
            .get(format!("{}{}", SC_API_URL, path))
            .query(&[("client_id", &client_id)]);

        Ok(req)
    }

    fn unwrap_api_error<T: DeserializeOwned>(text: String) -> Result<T, SoundcloudError> {
        match serde_json::from_str::<T>(&text) {
            Ok(value) => Ok(value),
            Err(err) => Err(SoundcloudError::JsonParsingError(err, text)),
        }
    }

    pub async fn login_anonymous(&mut self) -> Result<(), SoundcloudError> {
        let resp = self.client.get(SC_URL).send().await?;
        let body = resp.text().await?;

        let site = Html::parse_document(&body);
        let script = Selector::parse("script[src]")?;
        let script_regex = Regex::new(r"https://.*/assets/0.*.js")?;

        let mut script_src = "";

        for element in site.select(&script) {
            if let Some(src) = element.value().attr("src")
                && script_regex.is_match(src)
            {
                script_src = src;
            }
        }

        let resp = self.client.get(script_src).send().await?;
        let body = resp.text().await?;

        let start = "\"client_id=";
        let end = "\"";

        let pos = body
            .find(start)
            .ok_or(SoundcloudError::AnonymousLoginFailed)?;
        let client_id = body
            .get(pos + start.len()..)
            .ok_or(SoundcloudError::AnonymousLoginFailed)?;

        let pos = client_id
            .find(end)
            .ok_or(SoundcloudError::AnonymousLoginFailed)?;
        let client_id = client_id
            .get(..pos)
            .ok_or(SoundcloudError::AnonymousLoginFailed)?
            .to_string();

        self.client_id = Some(client_id.clone());

        info!("Client ID: {}", client_id);

        Ok(())
    }

    pub async fn search(
        &self,
        query: String,
        limit: usize,
        offset: usize,
    ) -> Result<meta::Search, SoundcloudError> {
        let offset = limit * offset;

        info!("Soundcloud Search: {}, {}", query, offset);
        let resp = self
            .create_req("/search/tracks")?
            .query(&[
                ("q", query),
                ("limit", limit.to_string()),
                ("offset", offset.to_string()),
            ])
            .send()
            .await?;

        let text = resp.text().await?;
        let search_api: meta::Search = Self::unwrap_api_error(text)?;

        Ok(search_api)
    }

    #[allow(dead_code)]
    pub async fn tracks(&self, track_id: u64) -> Result<meta::Tracks, SoundcloudError> {
        info!("Soundcloud Tracks: {}", track_id);

        let resp = self
            .create_req("/tracks")?
            .query(&[("ids", track_id.to_string())])
            .send()
            .await?;

        let text = resp.text().await?;
        let tracks: meta::Tracks = Self::unwrap_api_error(text)?;

        Ok(tracks)
    }

    #[allow(dead_code)]
    pub async fn stream(&self, url: Url) -> Result<bytes::Bytes, SoundcloudError> {
        info!("Soundcloud Stream Url: {}", url);

        let client_id = self
            .client_id
            .as_ref()
            .ok_or(SoundcloudError::LoginDataMissing)?;

        let resp = self
            .client
            .get(url)
            .query(&[("client_id", client_id)])
            .send()
            .await?;

        #[derive(Deserialize)]
        struct UrlStruct {
            url: String,
        }

        let text = resp.text().await?;
        let url: UrlStruct = Self::unwrap_api_error(text)?;
        let url = url.url;

        info!("Audio Stream Url: {}", url);
        let resp = self.client.get(url).send().await?;

        Ok(resp.bytes().await?)
    }
}
