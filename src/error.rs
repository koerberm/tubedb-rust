use crate::error::Error::{Diqwest, Reqwest};

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidUrl,
    Reqwest { source: reqwest::Error },
    Diqwest { source: diqwest::error::Error },
    Parse(String),
    InvalidRegion(String),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest { source: e }
    }
}

impl From<diqwest::error::Error> for Error {
    fn from(source: diqwest::error::Error) -> Self {
        match source {
            diqwest::error::Error::ReqwestError(src) => Reqwest { source: src },
            _ => Diqwest { source }
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::Parse(e.to_string())
    }
}


