use log::info;
use md5::{Digest, Md5};
use serde::{Deserialize, de::DeserializeOwned};
use tauri_plugin_http::reqwest;

mod api;
mod error;
mod meta;

pub use api::*;
use error::*;
pub use meta::*;

const LASTFM_URL: &str = "http://www.last.fm";
const LASTFM_API_URL: &str = "http://ws.audioscrobbler.com/2.0/";

const LASTFM_API_KEY: &str = "581cd09d1d47ce7e760ce5ff9a8513e2";
const LASTFM_SECRET: &str = "9ee7f889d438878fc0560f3ef38b2016";

pub struct LastFM {
    client: reqwest::Client,
    login_data: Option<LoginData>,
}

pub struct LoginData {
    username: String,
    session_key: String,
}

impl Default for LastFM {
    fn default() -> Self {
        let client = reqwest::Client::builder()
            .user_agent(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:136.0) Gecko/20100101 Firefox/136.0",
            )
            .build()
            .unwrap();

        Self {
            client,
            login_data: None,
        }
    }
}

impl LastFM {
    fn create_sig<T: LastFMMethod>(method: T, token: String) -> String {
        let mut md5 = Md5::new();
        md5.update("api_key");
        md5.update(LASTFM_API_KEY);
        md5.update("method");
        md5.update(method.to_string());
        md5.update("token");
        md5.update(token);
        md5.update(LASTFM_SECRET);
        let hash = md5.finalize();

        base16ct::lower::encode_string(&hash)
    }

    fn create_req<T: LastFMMethod>(
        &self,
        method: T,
    ) -> Result<reqwest::RequestBuilder, LastFMError> {
        let method_str: String = method.to_string();

        let req = self.client.get(LASTFM_API_URL).query(&[
            ("method", method_str.as_str()),
            ("api_key", LASTFM_API_KEY),
            ("format", "json"),
        ]);

        match method.auth_level() {
            LastFMAuthLevel::None => Ok(req),
            LastFMAuthLevel::Session => {
                if let Some(login_data) = &self.login_data {
                    Ok(req.query(&[
                        (
                            "api_sig",
                            LastFM::create_sig(method, login_data.session_key.clone()),
                        ),
                        ("sk", login_data.session_key.clone()),
                    ]))
                } else {
                    Err(LastFMError::LoginDataMissing)
                }
            }
        }
    }

    fn unwrap_api_error<T: DeserializeOwned>(text: String) -> Result<T, LastFMError> {
        if let Ok(err) = serde_json::from_str::<LastFMApiError>(&text) {
            Err(LastFMError::ApiError(err))
        } else {
            match serde_json::from_str::<T>(&text) {
                Ok(value) => Ok(value),
                Err(err) => Err(LastFMError::JsonParsingError(err)),
            }
        }
    }

    async fn req_token(&self) -> Result<String, LastFMError> {
        let req = self
            .client
            .get(LASTFM_API_URL)
            .query(&[
                ("method", "auth.gettoken"),
                ("api_key", LASTFM_API_KEY),
                ("format", "json"),
            ])
            .send()
            .await
            .unwrap();

        #[derive(Deserialize)]
        struct TokenStruct {
            token : String
        }

        let text = req.text().await.unwrap();
        let token = Self::unwrap_api_error::<TokenStruct>(text)?;

        Ok(token.token)
    }

    pub async fn login(&mut self) -> Result<(), LastFMError> {
        let token = self.req_token().await?;

        info!("Token: {}", token);
        Ok(())
    }
}
