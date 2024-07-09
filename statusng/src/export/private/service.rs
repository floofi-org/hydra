use serde::Serialize;

use crate::models::service::{ServiceCategory, ServiceHostingProvider, ServiceStatus};
use crate::models::Service;

#[derive(Serialize, Debug)]
pub struct ClientService {
    pub id: String,
    pub label: String,
    pub ping: u32,
    pub status: ServiceStatus,
    #[serde(rename = "type")]
    pub category: ServiceCategory,
    #[serde(rename = "hosting")]
    pub service_hosting_provider: ServiceHostingProvider,
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

    pub fn into_bytes(self) -> Vec<u8> {
        let mut bytes = vec![];

        let id = &mut self.id.into_bytes();
        let label = &mut self.label.into_bytes();
        let ping = &mut self.ping.to_le_bytes().to_vec();
        let status = self.status as u8;
        let category = self.category as u8;
        let hosting_provider = self.service_hosting_provider as u8;

        bytes.push(id.len() as u8);
        bytes.append(id);

        bytes.push(label.len() as u8);
        bytes.append(label);

        bytes.append(ping);
        bytes.push(status);
        bytes.push(category);
        bytes.push(hosting_provider);

        bytes
    }
}
