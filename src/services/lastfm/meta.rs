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
    #[serde(rename = "opensearch:totalResults")]
    pub total_results: String,
    #[serde(rename = "opensearch:startIndex")]
    pub start_index: String,
    #[serde(rename = "opensearch:itemsPerPage")]
    pub items_per_page: String,

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
    pub listeners: String,
    pub mbid: Option<Uuid>,
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
