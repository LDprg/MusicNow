use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct LastFMApiError {
    message: String,
    error: i64,
}
