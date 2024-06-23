use std::fmt::{Display, Formatter};
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::models::service::kind::{HttpService, TcpService};

mod status;
mod processor;
pub mod kind;

pub use status::*;
pub use processor::*;

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub struct Service {
    pub maintenance: bool,
    pub host: String,
    pub port: u16,
    pub name: Option<String>,  // By default, use 'host'
    pub category: ServiceCategory,
    #[serde(alias = "hosting")]
    pub hosting_provider: ServiceHostingProvider,
    #[serde(alias = "id")]
    pub _legacy_id: String,

    #[serde(flatten)]
    pub service_type: ServiceType,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ServiceType {
    #[serde(rename = "http", alias = "https")]
    Http(HttpService),
    #[serde(rename = "tcp")]
    Tcp(TcpService),
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Copy, Clone)]
pub enum ServiceCategory {
    #[serde(rename = "websites")]
    Websites,
    #[serde(rename = "servers")]
    Servers,
    #[serde(rename = "network")]
    Network
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub enum ServiceHostingProvider {
    #[serde(rename = "equestriadev")]
    EquestriaDev,
    #[serde(rename = "scaleway")]
    Scaleway,
    #[serde(rename = "ovh")]
    Ovh,
    #[serde(rename = "vercel")]
    Vercel,
    #[serde(rename = "gitbook")]
    GitBook
}

impl Service {
    pub fn process(&self, timeout: Duration, slow_threshold: u32) -> ProcessorResult {
        let result = self.process_service(timeout);

        match result {
            Err(_) => {
                ProcessorResult {
                    ping: 0,
                    status: ServiceStatus::Offline,
                }
            },
            Ok(ping) => {
                let status = self.make_status(&result, slow_threshold);

                ProcessorResult {
                    ping,
                    status,
                }
            }
        }
    }

    fn process_service(&self, timeout: Duration) -> ServiceResult {
        let ping = match &self.service_type {
            ServiceType::Http(service) => service.process(self, timeout)?,
            ServiceType::Tcp(service) => service.process(self, timeout)?,
        };

        Ok(ping)
    }

    fn make_status(&self, result: &ServiceResult, slow_threshold: u32) -> ServiceStatus {
        match result {
            Ok(ping) if *ping < slow_threshold => ServiceStatus::Online,
            Ok(_) => ServiceStatus::Unstable,
            Err(_) if self.maintenance => ServiceStatus::Maintenance,
            _ => ServiceStatus::Offline,
        }
    }

    pub fn get_legacy_id(&self) -> &str {
        &self._legacy_id
    }

    pub fn get_unique_id(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn get_label(&self) -> String {
        match self.category {
            ServiceCategory::Network | ServiceCategory::Servers => self.name.clone().unwrap_or_else(|| self.host.clone()),
            _ => self.host.clone()
        }
    }
}

impl Display for Service {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.service_type {
            ServiceType::Http(_) => write!(f, "{} is an http service", self.host),
            ServiceType::Tcp(_) => write!(f, "{} is a tcp service", self.host),
        }
    }
}
