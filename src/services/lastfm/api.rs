use std::fmt;


pub trait AuthLevel {
    fn auth_level(&self) -> LastFMAuthLevel;
}

pub enum LastFMMethod {
    Auth(LastFMAuthMethod),
}

impl fmt::Display for LastFMMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LastFMMethod::Auth(auth) => write!(f, "{}", auth),
        }
    }
}

impl AuthLevel for LastFMMethod {
    fn auth_level(&self) -> LastFMAuthLevel {
        match self {
            LastFMMethod::Auth(auth) => auth.auth_level(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum LastFMAuthLevel {
    None,
    Token,
    Session,
}

#[derive(Clone, strum::Display)]
#[strum(serialize_all = "lowercase", prefix = "auth.")]
pub enum LastFMAuthMethod {
    GetSession,
    GetToken,
}

impl AuthLevel for LastFMAuthMethod {
    fn auth_level(&self) -> LastFMAuthLevel {
        match self {
            LastFMAuthMethod::GetSession => LastFMAuthLevel::Token,
            LastFMAuthMethod::GetToken => LastFMAuthLevel::None,
        }
    }
}

impl From<LastFMAuthMethod> for LastFMMethod {
    fn from(value: LastFMAuthMethod) -> Self {
        LastFMMethod::Auth(value)
    }
}
