use serde::Deserialize;
use std::str::FromStr;
use tauri_plugin_http::reqwest::Url;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct LastFMApiError {
    pub message: String,
    pub error: i64,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct LastFMApiTrackSearchWrapper {
    pub results: LastFMApiTrackSearch,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct LastFMApiTrackSearch {
    #[serde(
        rename = "opensearch:totalResults",
        deserialize_with = "string_unwrap_deserialize"
    )]
    pub total_results: u64,
    #[serde(
        rename = "opensearch:startIndex",
        deserialize_with = "string_unwrap_deserialize"
    )]
    pub start_index: u64,
    #[serde(
        rename = "opensearch:itemsPerPage",
        deserialize_with = "string_unwrap_deserialize"
    )]
    pub items_per_page: u64,

    pub trackmatches: LastFMApiTrackWrapper,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct LastFMApiTrackWrapper {
    pub track: Vec<LastFMApiTrack>,
}

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct LastFMApiTrack {
    pub name: String,
    pub artist: String,
    pub url: Url,
    #[serde(deserialize_with = "string_unwrap_deserialize")]
    pub listeners: u64,
    #[serde(deserialize_with = "option_deserialize")]
    pub mbid: Option<Uuid>,
}

fn option_deserialize<'d, D, T: Deserialize<'d>>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'d>,
{
    let data = T::deserialize(deserializer);
    Ok(data.ok())
}

fn string_unwrap_deserialize<'d, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'d>,
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let string = String::deserialize(deserializer)?;
    string
        .parse::<T>()
        .map_err(|e| serde::de::Error::custom(format!("{:?}", e)))
}
