use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LastFMApiError {
    pub message: String,
    pub error: i64,
}
