use std::fmt::{Display, Formatter};
use std::io;

#[derive(Debug)]
pub enum StatusError {
    YamlParseError(serde_yml::Error),
    JsonParseError(serde_json::Error),
    IoError(io::Error),
    GenericError(String)
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

impl From<String> for StatusError {
    fn from(value: String) -> Self {
        Self::GenericError(value)
    }
}

impl Display for StatusError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::GenericError(message) => write!(f, "{}", message),
            Self::IoError(err) => write!(f, "{:?}", err),
            Self::JsonParseError(err) => write!(f, "{:?}", err),
            Self::YamlParseError(err) => write!(f, "{:?}", err)
        }
    }
}
