use std::fmt::{Display, Formatter};
use std::time::Duration;

use serde::{Deserialize, Serialize};

use kind::{HttpService, TcpService};
use crate::processors::{Processor, ProcessorResult, Http, Tcp};

mod status;
pub mod kind;

pub use status::ServiceStatus;

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Service {
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
    pub fn process(self, timeout: Duration, slow_threshold: u32) -> ProcessorResult {
        match self {
            Service::Http(service) => Http::process(service, timeout, slow_threshold),
            Service::Tcp(service) => Tcp::process(service, timeout, slow_threshold),
        }
    }

    pub fn get_legacy_id(&self) -> &str {
        match self {
            Service::Http(service) => &service._legacy_id,
            Service::Tcp(service) => &service._legacy_id,
        }
    }

    pub fn get_unique_id(&self) -> String {
        match self {
            Service::Http(service) => service.get_unique_id(),
            Service::Tcp(service) => service.get_unique_id(),
        }
    }

    pub fn get_label(&self) -> String {
        match self {
            Service::Http(service) => service.get_label(),
            Service::Tcp(service) => service.get_label(),
        }
    }
}

impl Display for Service {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Service::Http(service) => write!(f, "{} is an http service", service.host),
            Service::Tcp(service) => write!(f, "{} is a tcp service", service.host),
        }
    }
}
