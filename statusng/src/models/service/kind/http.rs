use serde::Deserialize;
use crate::models::service::{ServiceCategory, ServiceHostingProvider};

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
    //noinspection HttpUrlsUsage - Stupid RustRover
    pub fn get_url(&self) -> String {
        let mut url = String::from("");

        url.push_str(if self.tls {
            "https://"
        } else {
            "http://"
        });

        url.push_str(&self.host);

        if (self.tls && self.port != 443) ||
            (!self.tls && self.port != 80) {
            url.push_str(&format!(":{}", self.port));
        }

        if self.url.starts_with('/') {
            url.push_str(&self.url)
        } else {
            url.push('/');
            url.push_str(&self.url);
        };

        url
    }

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
