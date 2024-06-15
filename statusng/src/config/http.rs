use serde::Deserialize;
use crate::config::{ServiceCategory, ServiceHostingProvider};

#[derive(Deserialize, Debug)]
pub struct HttpService {
    pub maintenance: bool,
    pub host: String,
    pub port: u16,
    pub name: Option<String>,  // By default, use 'host'
    pub category: ServiceCategory,
    #[serde(alias = "hosting")]
    pub hosting_provider: ServiceHostingProvider,
    #[serde(alias = "id")]
    pub _legacy_id: String,

    pub url: String,
    #[serde(alias = "expect")]
    pub expected_code: u16,
    #[serde(default = "default_tls")]
    pub tls: bool
}

fn default_tls() -> bool {
    true
}

impl HttpService {
    pub fn get_unique_id(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn get_label(&self) -> String {
        match self.category {
            ServiceCategory::Network | ServiceCategory::Servers => self.name.clone().unwrap_or(self.host.clone()),
            _ => self.host.clone()
        }
    }
}
