use std::sync::LazyLock;

static SINGLETON_LASTFM: LazyLock<LastFM> = LazyLock::new(LastFM::new);

#[derive(Clone, Debug)]
pub struct LastFM {}

impl LastFM {
    fn new() -> Self {
        Self {}
    }
}

impl Default for LastFM {
    fn default() -> Self {
        SINGLETON_LASTFM.clone()
    }
}
