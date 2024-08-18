use std::fmt::{Display, Formatter};
use std::net::IpAddr;
use std::time::Duration;
use dns_lookup::lookup_addr;
use serde::{Deserialize, Serialize};

use crate::models::service::kind::{HttpService, TcpService, IcmpService};

pub mod kind;
mod processor;
mod status;

pub use processor::*;
pub use status::*;

#[derive(Deserialize, Debug)]
pub struct Service {
    pub maintenance: bool,
    pub host: String,
    pub category: ServiceCategory,
    #[serde(alias = "hosting")]
    pub hosting_provider: ServiceHostingProvider,
    #[serde(alias = "name")]
    pub network_name: Option<String>,

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
    #[serde(rename = "icmp")]
    Icmp(IcmpService),
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Copy, Clone)]
#[repr(u8)]  // This should be fairly safe as long as order never changes.
pub enum ServiceCategory {
    #[serde(rename = "websites")]
    Websites,
    #[serde(rename = "servers")]
    Servers,
    #[serde(rename = "network")]
    Network,
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
#[repr(u8)]  // This should be fairly safe as long as order never changes.
pub enum ServiceHostingProvider {
    #[serde(rename = "self")]
    #[serde(alias = "equestriadev")]
    Own,
    #[serde(rename = "scaleway")]
    Scaleway,
    #[serde(rename = "ovh")]
    Ovh,
    #[serde(rename = "vercel")]
    Vercel,
    #[serde(rename = "gitbook")]
    GitBook,
    #[serde(rename = "azure")]
    Azure,
}

impl Display for ServiceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceType::Http(_) => write!(f, "http"),
            ServiceType::Tcp(_) => write!(f, "tcp"),
            ServiceType::Icmp(_) => write!(f, "icmp"),
        }
    }
}

impl Service {
    pub fn process(&self, timeout: Duration, slow_threshold: u32) -> ProcessorResult {
        let result = self.process_service(timeout);
        let status = self.make_status(&result, slow_threshold);
        let ping = result.unwrap_or(0);

        ProcessorResult { ping, status }
    }

    fn process_service(&self, timeout: Duration) -> ServiceResult {
        let ping = match &self.service_type {
            ServiceType::Http(service) => service.process(self, timeout)?,
            ServiceType::Tcp(service) => service.process(self, timeout)?,
            ServiceType::Icmp(service) => service.process(self, timeout)?,
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

    pub fn get_unique_id(&self) -> String {
        format!("{}:{}", self.host, self.service_type)
    }

    pub fn get_label(&self) -> String {
        match &self.category {
            ServiceCategory::Websites => self.host.clone(),
            ServiceCategory::Servers => {
                if let Ok(addr) = self.host.parse::<IpAddr>() {
                    if let Ok(name) = lookup_addr(&addr) {
                        name
                    } else {
                        self.host.clone()
                    }
                } else {
                    self.host.clone()
                }
            },
            ServiceCategory::Network => self.network_name.clone().unwrap_or(self.host.clone())
        }
    }
}

impl Display for Service {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.service_type {
            ServiceType::Http(_) => write!(f, "{} is an http service", self.host),
            ServiceType::Tcp(_) => write!(f, "{} is a tcp service", self.host),
            ServiceType::Icmp(_) => write!(f, "{} is an icmp service", self.host),
        }
    }
}
