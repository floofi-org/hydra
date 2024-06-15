use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::net::{AddrParseError, SocketAddr, ToSocketAddrs};
use serde::Deserialize;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::{fs, io};
use std::vec::IntoIter;
use log::debug;
use serde::ser::Error;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub interval: u32,
    #[serde(alias = "slow")]
    pub slow_threshold: u32,
    pub timeout: u32,
    pub outage: OutageConfig,
    pub services: Vec<ServiceConfig>
}

#[derive(Deserialize, Debug)]
pub struct OutageConfig {
    enabled: bool,
    title: String,
    description: String,
    link: String
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ServiceConfig {
    #[serde(rename = "http", alias = "https")]
    HttpServiceConfig(HttpServiceConfig),
    #[serde(rename = "tcp")]
    TcpServiceConfig(TcpServiceConfig),
}

#[derive(Deserialize, Debug)]
pub struct HttpServiceConfig {
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

#[derive(Deserialize, Debug)]
pub struct TcpServiceConfig {
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

#[derive(Deserialize, Debug)]
enum ServiceType {
    #[serde(rename = "https")]
    Https,
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "tcp")]
    Tcp
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

type ServiceHistory = HashMap<String, Vec<ServiceCode>>;
pub type BaseHistory = HashMap<String, ServiceHistory>;

#[derive(Deserialize_repr, Serialize_repr, Debug)]
#[repr(u8)]
pub enum ServiceCode {
    Online = 0,
    Unstable = 1,
    Offline = 2,
    Maintenance = 3
}

impl Display for ServiceCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceCode::Online => write!(f, "online"),
            ServiceCode::Unstable => write!(f, "unstable/slow"),
            ServiceCode::Offline => write!(f, "offline"),
            ServiceCode::Maintenance => write!(f, "under maintenance")
        }
    }
}

impl HttpServiceConfig {
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

        let path = if self.url.starts_with("/") {
            &self.url
        } else {
            &format!("/{}", self.url)
        };
        url.push_str(path);

        url
    }
}

impl TcpServiceConfig {
    pub fn get_address(&self) -> io::Result<IntoIter<SocketAddr>> {
        format!("{}:{}", self.host, self.port)
            .to_socket_addrs()
    }
}

pub fn load_data() -> Result<(Config, BaseHistory), Box<dyn Error>> {
    let config_raw = fs::read_to_string("./config.yaml")?;
    let config: Config = serde_yaml::from_str(&config_raw)?;
    debug!("Done loading config.yaml.");

    let history_raw = fs::read_to_string("./history.json")?;
    let history: BaseHistory = serde_json::from_str(&history_raw)?;
    debug!("Done loading history.json.");

    Ok((config, history))
}
