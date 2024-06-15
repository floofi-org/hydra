use serde::Deserialize;
use crate::config::Service;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub interval: u32,
    #[serde(alias = "slow")]
    pub slow_threshold: u32,
    pub timeout: u32,
    pub outage: OutageConfig,
    pub services: Vec<Service>
}

#[derive(Deserialize, Debug)]
pub struct OutageConfig {
    enabled: bool,
    title: String,
    description: String,
    link: String
}
