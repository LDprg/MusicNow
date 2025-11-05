use std::sync::LazyLock;

static SINGLETON_MUSIC: LazyLock<ListenBrainzApi> = LazyLock::new(ListenBrainzApi::new);

#[derive(Clone, Debug)]
pub struct ListenBrainzApi {}

impl ListenBrainzApi {
    fn new() -> Self {
        Self {}
    }
}

impl Default for ListenBrainzApi {
    fn default() -> Self {
        SINGLETON_MUSIC.clone()
    }
}
