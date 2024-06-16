use serde::Serialize;
use crate::models::service::{Service, ServiceCategory, ServiceHostingProvider, ServiceStatus};
use crate::processors::ProcessorResult;

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

impl From<&ProcessorResult> for ClientService {
    fn from(value: &ProcessorResult) -> Self {
        ClientService {
            id: format!("{:x}", md5::compute(value.service.get_unique_id().into_bytes())),
            label: value.service.get_label(),
            ping: value.ping,
            status: value.status,
            category: match &value.service {
                Service::Http(service) => service.category,
                Service::Tcp(service) => service.category
            },
            service_hosting_provider: match &value.service {
                Service::Http(service) => service.hosting_provider,
                Service::Tcp(service) => service.hosting_provider
            }
        }
    }
}
