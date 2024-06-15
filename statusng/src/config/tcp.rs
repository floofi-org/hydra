use serde::Deserialize;
use crate::config::{ServiceCategory, ServiceHostingProvider};

#[derive(Deserialize, Debug)]
pub struct TcpService {
    pub maintenance: bool,
    pub host: String,
    pub port: u16,
    pub name: Option<String>,  // By default, use 'host'
    pub category: ServiceCategory,
    #[serde(alias = "hosting")]
    pub hosting_provider: ServiceHostingProvider,
    #[serde(alias = "id")]
    pub _legacy_id: String
}
