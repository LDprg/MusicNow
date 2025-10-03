use crate::prelude::*;

use dioxus::logger::tracing::{info, warn};
use regex::Regex;
use scraper::{Html, Selector};

// use crate::audio::*;

pub async fn run() -> Result<()> {
    warn!("Run Audio Service");
    // let client_id = get_client_id().await?;
    //
    // run_audio(client_id).await?;

    Ok(())
}

#[allow(unused)]
async fn get_client_id() -> Result<String> {
    let resp = reqwest::get("https://soundcloud.com").await.unwrap();
    let body = resp.text().await.unwrap();

    let site = Html::parse_document(&body);
    let script = Selector::parse("script[src]").unwrap();
    let script_regex = Regex::new(r"https://.*/assets/0.*.js").unwrap();

    let mut script_src = "";

    for element in site.select(&script) {
        if let Some(src) = element.value().attr("src")
            && script_regex.is_match(src)
        {
            script_src = src;
        }
    }

    info!("{}", script_src);

    let resp = reqwest::get(script_src).await.unwrap();
    let body = resp.text().await.unwrap();

    let start = "\"client_id=";
    let end = "\"";

    let pos = body.find(start).unwrap();
    let client_id = body.get(pos + start.len()..).unwrap();

    let pos = client_id.find(end).unwrap();
    let client_id = client_id.get(..pos).unwrap();

    info!("{}", client_id);

    Ok(client_id.to_string())
}
