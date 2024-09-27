use std::fmt::{Display, Formatter};
use std::io;
use std::string::FromUtf8Error;

pub type StatusResult<T> = Result<T, StatusError>;

#[derive(Debug)]
pub enum StatusError {
    YamlError(serde_yml::Error),
    JsonError(serde_json::Error),
    TomlSerializeError(toml::ser::Error),
    TomlDeserializeError(toml::de::Error),
    UreqError(ureq::Error),
    IoError(io::Error),
    HistoryError(HistoryError),
    GenericError(String),
}

#[derive(Debug)]
pub enum HistoryError {
    EndOfFile,
    InvalidString(FromUtf8Error),
    IoError(io::Error),
    InvalidDate,
}

impl From<serde_yml::Error> for StatusError {
    fn from(value: serde_yml::Error) -> Self {
        Self::YamlError(value)
    }
}

impl From<serde_json::Error> for StatusError {
    fn from(value: serde_json::Error) -> Self {
        Self::JsonError(value)
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

impl From<toml::ser::Error> for StatusError {
    fn from(value: toml::ser::Error) -> Self {
        Self::TomlSerializeError(value)
    }
}

impl From<toml::de::Error> for StatusError {
    fn from(value: toml::de::Error) -> Self {
        Self::TomlDeserializeError(value)
    }
}

impl Display for StatusError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::YamlError(err) => write!(f, "{:?}", err),
            Self::JsonError(err) => write!(f, "{:?}", err),
            Self::TomlSerializeError(err) => write!(f, "{:?}", err),
            Self::TomlDeserializeError(err) => write!(f, "{:?}", err),
            Self::IoError(err) => write!(f, "{:?}", err),
            Self::UreqError(err) => write!(f, "{:?}", err),
            Self::HistoryError(err) => write!(f, "{:?}", err),
            Self::GenericError(message) => write!(f, "{}", message),
        }
    }
}

impl From<FromUtf8Error> for HistoryError {
    fn from(value: FromUtf8Error) -> Self {
        Self::InvalidString(value)
    }
}

impl From<io::Error> for HistoryError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}
