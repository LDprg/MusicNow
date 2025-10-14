use crate::prelude::*;
use dioxus::prelude::*;

use super::*;

#[post("/api/search")]
pub async fn search(query: String, limit: usize, offset: usize) -> Result<SearchApi> {
    let api = SoundCloudApi::default();

    api.search(&query, limit, offset).await
}
