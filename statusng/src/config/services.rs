use std::fmt::{Display, Formatter};
use std::time::Duration;
use serde::Deserialize;
use crate::config::{HttpService, TcpService};
use crate::processors::{Http, Processor, ProcessorResult, Tcp};

#[cfg(test)]
#[path="tests/services.rs"]
mod tests;

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Service {
    #[serde(rename = "http", alias = "https")]
    HttpServiceConfig(HttpService),
    #[serde(rename = "tcp")]
    TcpServiceConfig(TcpService),
}

#[derive(Deserialize, Debug)]
pub enum ServiceCategory {
    #[serde(rename = "websites")]
    Websites,
    #[serde(rename = "servers")]
    Servers,
    #[serde(rename = "network")]
    Network
}

#[derive(Deserialize, Debug)]
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
            Service::HttpServiceConfig(service) => Http::process(service, timeout, slow_threshold),
            Service::TcpServiceConfig(service) => Tcp::process(service, timeout, slow_threshold),
        }
    }
}

impl Display for Service {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Service::HttpServiceConfig(service) => write!(f, "{} is an http service", service.host),
            Service::TcpServiceConfig(service) => write!(f, "{} is a tcp service", service.host),
        }
    }
}
