use std::fmt::Display;

use strum;

#[derive(Clone, Copy, Debug)]
pub enum LastFMAuthLevel {
    None,
    Session,
}

pub trait LastFMMethod: Display {
    fn auth_level(&self) -> LastFMAuthLevel;
}

#[derive(Clone, strum::Display)]
#[strum(serialize_all = "lowercase", prefix = "track.")]
pub enum LastFMTrackMethod {
    Search,
}

impl LastFMMethod for LastFMTrackMethod {
    fn auth_level(&self) -> LastFMAuthLevel {
        match self {
            LastFMTrackMethod::Search => LastFMAuthLevel::None,
        }
    }
}
