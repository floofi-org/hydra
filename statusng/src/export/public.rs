use serde::Serialize;

use crate::models::service::ServiceStatus;

#[derive(Serialize, Debug)]
struct PublicAPIv1 {
    _notice: String,
    ping: f32,
    code: ServiceStatus,
    image: String,
    text: String,
    outages: Vec<(Option<()>, String)>
}

#[derive(Serialize, Debug)]
struct PublicAPIv2 {
    ping: f32,
    code: ServiceStatus,
    outages: Vec<String>
}
