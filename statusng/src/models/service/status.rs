use std::fmt::{Display, Formatter};

use serde_repr::{Deserialize_repr, Serialize_repr};

use super::ServiceResult;

#[derive(Deserialize_repr, Serialize_repr, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum ServiceStatus {
    Online = 0,
    Unstable = 1,
    Offline = 2,
    Maintenance = 3
}

impl Display for ServiceStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceStatus::Online => write!(f, "online"),
            ServiceStatus::Unstable => write!(f, "unstable/slow"),
            ServiceStatus::Offline => write!(f, "offline"),
            ServiceStatus::Maintenance => write!(f, "under maintenance")
        }
    }
}

impl ServiceStatus {
    fn as_image_string(&self) -> String {
        match self {
            Self::Online => String::from("status-ok"),
            Self::Unstable => String::from("status-warning"),
            Self::Offline => String::from("status-error"),
            Self::Maintenance => String::from("status-error")
        }
    }

    fn get_description(&self) -> &str {
        match self {
            Self::Online => "All systems nominal",
            Self::Unstable => "Degraded performance",
            Self::Offline => "Servers outage",
            Self::Maintenance => "Servers outage",
        }
    }
}
