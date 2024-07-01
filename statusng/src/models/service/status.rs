use std::fmt::{Display, Formatter};

use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Deserialize_repr, Serialize_repr)]
#[derive(Default, Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
#[repr(u8)]
pub enum ServiceStatus {
    #[default]
    Online = 0,
    Unstable = 1,
    Offline = 2,
    Maintenance = 3,
}

impl Display for ServiceStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceStatus::Online => write!(f, "online"),
            ServiceStatus::Unstable => write!(f, "unstable/slow"),
            ServiceStatus::Offline => write!(f, "offline"),
            ServiceStatus::Maintenance => write!(f, "under maintenance"),
        }
    }
}

impl ServiceStatus {
    pub fn as_image_string(&self) -> &str {
        match self {
            Self::Online => "status-ok",
            Self::Unstable => "status-warning",
            Self::Offline => "status-error",
            Self::Maintenance => "status-error",
        }
    }

    pub fn get_description(&self) -> &str {
        match self {
            Self::Online => "All systems nominal",
            Self::Unstable => "Degraded performance",
            Self::Offline => "Servers outage",
            Self::Maintenance => "Servers outage",
        }
    }
}

impl From<u8> for ServiceStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => ServiceStatus::Online,
            1 => ServiceStatus::Unstable,
            2 => ServiceStatus::Offline,
            3 => ServiceStatus::Maintenance,
            _ => ServiceStatus::Offline,
        }
    }
}
