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

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct LastFMApiTrackSearch {
    #[serde(rename = "opensearch:totalResults")]
    pub total_results: u64,
    #[serde(rename = "opensearch:startIndex")]
    pub start_index: u64,
    #[serde(rename = "opensearch:itemsPerPage")]
    pub items_per_page: u64,

    pub trackmatches: LastFMApiTrackWrapper,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct LastFMApiTrackWrapper {
    pub track: Vec<LastFMApiTrack>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct LastFMApiTrack {
    pub name: String,
    pub artist: String,
    pub url: Url,
    pub listeners: u64,
    pub image: LastFMApiTrackImage,
    pub mbid: Uuid,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct LastFMApiTrackImage {
    small: Url,
    medium: Url,
    large: Url,
    extralarge: Url,
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

        let out = Self {
            small: search(&items, "small")?,
            medium: search(&items, "medium")?,
            large: search(&items, "large")?,
            extralarge: search(&items, "extralarge")?,
        };

        Ok(out)
    }
}
