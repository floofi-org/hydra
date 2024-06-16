use std::io;
use std::net::{SocketAddr, ToSocketAddrs};
use std::vec::IntoIter;

use serde::Deserialize;

use crate::models::service::{ServiceCategory, ServiceHostingProvider};

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

impl TcpService {
    pub fn get_unique_id(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn get_label(&self) -> String {
        match self.category {
            ServiceCategory::Network | ServiceCategory::Servers => self.name.clone().unwrap_or(self.host.clone()),
            _ => self.host.clone()
        }
    }

    pub fn get_address(&self) -> io::Result<IntoIter<SocketAddr>> {
        format!("{}:{}", self.host, self.port)
            .to_socket_addrs()
    }
}
