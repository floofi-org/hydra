use serde::Serialize;

use crate::models::service::{ServiceCategory, ServiceHostingProvider, ServiceStatus};
use crate::models::Service;

#[derive(Serialize, Debug)]
pub struct ClientService {
    id: String,
    pub label: String,
    pub ping: u32,
    pub status: ServiceStatus,
    #[serde(rename = "type")]
    category: ServiceCategory,
    #[serde(rename = "hosting")]
    service_hosting_provider: ServiceHostingProvider,
}

impl ClientService {
    pub fn new(service: &Service, status: ServiceStatus, ping: u32) -> Self {
        let hash = md5::compute(service.get_unique_id().into_bytes());

        Self {
            id: format!("{:x}", hash),
            label: service.get_label(),
            ping,
            status,
            category: service.category,
            service_hosting_provider: service.hosting_provider,
        }
    }
}
