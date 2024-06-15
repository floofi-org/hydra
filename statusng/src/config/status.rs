use std::fmt::{Display, Formatter};
use serde_repr::{Deserialize_repr, Serialize_repr};

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
    pub fn as_usize(&self) -> usize {
        self.clone() as usize
    }

    fn as_image_string(&self) -> String {
        match self {
            ServiceStatus::Online => String::from("status-ok"),
            ServiceStatus::Unstable => String::from("status-warning"),
            ServiceStatus::Offline => String::from("status-error"),
            ServiceStatus::Maintenance => String::from("status-error")
        }
    }
}

impl From<ServiceStatus> for String {
    fn from(value: ServiceStatus) -> Self {
        match value {
            ServiceStatus::Online => String::from("All systems nominal"),
            ServiceStatus::Unstable => String::from("Degraded performance"),
            ServiceStatus::Offline => String::from("Servers outage"),
            ServiceStatus::Maintenance => String::from("Servers outage")
        }
    }
}
