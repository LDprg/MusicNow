use crate::{
    prelude::*,
    services::storage::{DataStorage, LastFMStorage},
};

use std::{
    sync::{Arc, LazyLock, Mutex},
    time::Duration,
};

use md5::{Digest, Md5};
use reqwest::RequestBuilder;
use serde_json::Value;
use tokio::time::sleep;

mod api;
mod meta;

pub use self::api::*;
pub use self::meta::*;

const LASTFM_URL: &str = "http://www.last.fm";
const LASTFM_API_URL: &str = "http://ws.audioscrobbler.com/2.0/";

const LASTFM_API_KEY: &str = "581cd09d1d47ce7e760ce5ff9a8513e2";
const LASTFM_SECRET: &str = "9ee7f889d438878fc0560f3ef38b2016";

const LASTFM_SESSION_INTERVAL: Duration = Duration::from_secs(2);

static SINGLETON_LASTFM: LazyLock<LastFM> = LazyLock::new(LastFM::new);

#[derive(Clone, Debug)]
pub struct LastFM {
    client: reqwest::Client,
    inner: Arc<Mutex<LastFMInner>>,
}

#[derive(Debug)]
struct LastFMInner {
    token: String,
    username: String,
    session_key: String,
}

impl LastFM {
    fn new() -> Self {
        let client = reqwest::Client::builder()
            .user_agent(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:136.0) Gecko/20100101 Firefox/136.0",
            )
            .build()
            .unwrap();

        Self {
            client,
            inner: Arc::new(Mutex::new(LastFMInner {
                token: "".to_string(),
                username: "".to_string(),
                session_key: "".to_string(),
            })),
        }
    }

    pub async fn login(&self) {
        let req = self
            .create_req(LastFMAuthMethod::GetToken.into())
            .send()
            .await
            .unwrap();
        let json = req.json::<Value>().await.unwrap();
        let token = json.get("token").unwrap().as_str().unwrap();

        {
            let mut inner = self.inner.lock().unwrap();
            inner.token = token.to_string();
        }

        info!("Token: {}", token);

        let storage = DataStorage::default();
        let data = storage.load_lastfm();

        if !data.session_key.is_empty() && !data.username.is_empty() {
            info!("Load Session from file");
            {
                let mut inner = self.inner.lock().unwrap();
                inner.username = data.username;
                inner.session_key = data.session_key;
            }

            return;
        }

        info!("Opening Browser");

        let url = format!(
            "{}/api/auth?api_key={}&token={}",
            LASTFM_URL, LASTFM_API_KEY, token
        );

        // TODO: Maybe a wrapper?
        desktop!(
            dioxus::desktop::use_window().webview.load_url(url.as_str()).unwrap();
        );
        mobile!(
            dioxus::mobile::use_window().webview.load_url(url.as_str()).unwrap();
        );

        info!("Get Session!");
        loop {
            let req = self
                .create_req(LastFMAuthMethod::GetSession.into())
                .send()
                .await
                .unwrap();

            let text = req.text().await.unwrap();

            if let Ok(err) = serde_json::from_str::<LastFMApiError>(&text) {
                warn!("Res: {:#?}", err);
            } else {
                let json = serde_json::from_str::<Value>(&text).unwrap();
                let session = json.get("session").unwrap();

                let username = session.get("name").unwrap().as_str().unwrap().to_string();
                let session_key = session.get("key").unwrap().as_str().unwrap().to_string();

                let storage = DataStorage::default();
                storage.store_lastfm(LastFMStorage {
                    username: username.clone(),
                    session_key: session_key.clone(),
                });

                {
                    let mut inner = self.inner.lock().unwrap();
                    inner.username = username;
                    inner.session_key = session_key;
                }

                info!("Success: {:#?}", self);
                break;
            }

            sleep(LASTFM_SESSION_INTERVAL).await;
        }
    }

    fn create_sig(&self, method: LastFMMethod, token: String) -> String {
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

    fn create_req(&self, method: LastFMMethod) -> RequestBuilder {
        let method_str: String = method.to_string();

        let req = self.client.get(LASTFM_API_URL).query(&[
            ("method", method_str.as_str()),
            ("api_key", LASTFM_API_KEY),
            ("format", "json"),
        ]);

        let inner = self.inner.lock().unwrap();
        let token = inner.token.clone();
        let session_key = inner.session_key.clone();
        drop(inner);

        match method.auth_level() {
            LastFMAuthLevel::None => req,
            LastFMAuthLevel::Token => req.query(&[
                ("api_sig", self.create_sig(method, token.clone())),
                ("token", token),
            ]),
            LastFMAuthLevel::Session => req.query(&[
                ("api_sig", self.create_sig(method, session_key.clone())),
                ("sk", session_key),
            ]),
        }
    }
}

impl Default for LastFM {
    fn default() -> Self {
        SINGLETON_LASTFM.clone()
    }
}
