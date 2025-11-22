use std::str::FromStr;

use reqwest::Url;
use serde::Deserialize;
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

// TODO: fix u64/UUUID curently as String
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
    let uuid = T::deserialize(deserializer);
    Ok(uuid.ok())
}

fn string_unwrap_deserialize<'d, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'d>,
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let string = String::deserialize(deserializer)?;
    Ok(string
        .parse::<T>()
        .map_err(|e| serde::de::Error::custom(format!("{:?}", e)))?)
}

// BROKEN LASFTFM
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct LastFMApiTrackImage {
    pub small: Option<Url>,
    pub medium: Option<Url>,
    pub large: Option<Url>,
    pub extralarge: Option<Url>,
}

#[derive(Deserialize)]
struct RawItemLastFMApiTrackImage {
    pub size: String,
    #[serde(rename = "#text")]
    pub url: String,
}

impl<'d> Deserialize<'d> for LastFMApiTrackImage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'d>,
    {
        let items: Vec<RawItemLastFMApiTrackImage> = Vec::deserialize(deserializer)?;

        let search = |items: &Vec<RawItemLastFMApiTrackImage>,
                      size: &'static str|
         -> Result<Url, D::Error> {
            items
                .iter()
                .find_map(|x| {
                    if x.size == size {
                        Url::parse(&x.url).ok()
                    } else {
                        None
                    }
                })
                .ok_or(serde::de::Error::missing_field(size))
        };

        let small = search(&items, "small").ok();
        let medium = search(&items, "medium").ok();
        let large = search(&items, "large").ok();
        let extralarge = search(&items, "extralarge").ok();

        let out = Self {
            small,
            medium,
            large,
            extralarge,
        };

        Ok(out)
    }
}
