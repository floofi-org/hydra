use std::fmt::{Display, Formatter};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Deserialize_repr, Serialize_repr, Debug, Copy, Clone)]
#[repr(u8)]
pub enum ServiceCode {
    Online = 0,
    Unstable = 1,
    Offline = 2,
    Maintenance = 3
}

impl Display for ServiceCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceCode::Online => write!(f, "online"),
            ServiceCode::Unstable => write!(f, "unstable/slow"),
            ServiceCode::Offline => write!(f, "offline"),
            ServiceCode::Maintenance => write!(f, "under maintenance")
        }
    }
}
