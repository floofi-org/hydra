use std::io;
use std::fmt::{Display, Formatter};
use std::string::FromUtf8Error;

pub type StatusResult<T> = Result<T, StatusError>;

#[derive(Debug)]
pub enum StatusError {
    YamlParseError(serde_yml::Error),
    JsonParseError(serde_json::Error),
    UreqError(ureq::Error),
    IoError(io::Error),
    HistoryError(HistoryError),
    GenericError(String)
}
#[derive(Debug)]
pub enum HistoryError {
    EndOfFile,
    InvalidString(FromUtf8Error),
    InvalidDate
}

impl From<serde_yml::Error> for StatusError {
    fn from(value: serde_yml::Error) -> Self {
        Self::YamlParseError(value)
    }
}

impl From<serde_json::Error> for StatusError {
    fn from(value: serde_json::Error) -> Self {
        Self::JsonParseError(value)
    }
}

impl From<io::Error> for StatusError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<ureq::Error> for StatusError {
    fn from(value: ureq::Error) -> Self {
        Self::UreqError(value)
    }
}

impl From<HistoryError> for StatusError {
    fn from(value: HistoryError) -> Self {
        Self::HistoryError(value)
    }
}

impl From<String> for StatusError {
    fn from(value: String) -> Self {
        Self::GenericError(value)
    }
}

impl Display for StatusError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::YamlParseError(err) => write!(f, "{:?}", err),
            Self::JsonParseError(err) => write!(f, "{:?}", err),
            Self::IoError(err) => write!(f, "{:?}", err),
            Self::UreqError(err) => write!(f, "{:?}", err),
            Self::HistoryError(err) => write!(f, "{:?}", err),
            Self::GenericError(message) => write!(f, "{}", message)
        }
    }
}

impl From<FromUtf8Error> for HistoryError {
    fn from(value: FromUtf8Error) -> Self {
        Self::InvalidString(value)
    }
}
