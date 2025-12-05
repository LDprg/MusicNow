mod error;
mod meta;
mod rate_limiter;

use error::*;

use serde::de::DeserializeOwned;
use tauri_plugin_http::reqwest;

const MB_API_URL: &str = "http://musicbrainz.org/ws/2/";

pub struct MusicBrainz {
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for MusicBrainz {
    fn default() -> Self {
        let client = reqwest::Client::builder()
            .user_agent(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:136.0) Gecko/20100101 Firefox/136.0",
            )
            .build()
            .unwrap();

        let client = reqwest_middleware::ClientBuilder::new(client)
            .with(rate_limiter::RateLimiterMiddleware::default())
            .build();

        Self { client }
    }
}

impl MusicBrainz {
    fn create_req(
        &self,
        path: &str,
    ) -> Result<reqwest_middleware::RequestBuilder, MusicBrainzError> {
        let req = self
            .client
            .get(format!("{}{}", MB_API_URL, path))
            .query(&[("fmt", "json")]);

        Ok(req)
    }

    fn unwrap_api_error<T: DeserializeOwned>(text: String) -> Result<T, MusicBrainzError> {
        match serde_json::from_str::<T>(&text) {
            Ok(value) => Ok(value),
            Err(err) => Err(MusicBrainzError::JsonParsingError(err, text)),
        }
    }

    pub async fn search(
        &self,
        query: String,
        limit: usize,
        offset: usize,
    ) -> Result<meta::SearchRelease, MusicBrainzError> {
        let resp = self
            .create_req("release/")?
            .query(&[
                ("query", query),
                ("limit", limit.to_string()),
                ("offset", offset.to_string()),
            ])
            .send()
            .await?;

        let text = resp.text().await?;
        let search_api: meta::SearchRelease = Self::unwrap_api_error(text)?;
        Ok(search_api)
    }
}
