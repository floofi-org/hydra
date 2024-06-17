use serde::Serialize;

use crate::models::service::{ServiceCategory, ServiceHostingProvider, ServiceStatus};
use crate::models::Service;

#[derive(Serialize, Debug)]
pub struct ClientService {
    id: String,
    label: String,
    pub ping: u32,
    pub status: ServiceStatus,
    #[serde(alias = "type")]
    category: ServiceCategory,
    #[serde(alias = "hosting")]
    service_hosting_provider: ServiceHostingProvider
}

impl ClientService {
    pub fn new(service: &Service, status: ServiceStatus, ping: u32) -> Self {
        Self {
            id: format!("{:x}", md5::compute(service.get_unique_id().into_bytes())),
            label: service.get_label(),
            ping,
            status,
            category: service.category,
            service_hosting_provider: service.hosting_provider,
        }
    }
}
