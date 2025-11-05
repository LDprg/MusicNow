use std::sync::LazyLock;

static SINGLETON_MUSIC: LazyLock<CoverArtArchiveApi> = LazyLock::new(CoverArtArchiveApi::new);

#[derive(Clone, Debug)]
pub struct CoverArtArchiveApi {}

impl CoverArtArchiveApi {
    fn new() -> Self {
        Self {}
    }
}

impl Default for CoverArtArchiveApi {
    fn default() -> Self {
        SINGLETON_MUSIC.clone()
    }
}

