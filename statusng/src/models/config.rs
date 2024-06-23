use serde::{Deserialize, Serialize};

use super::Service;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub interval: u32,
    #[serde(alias = "slow")]
    pub slow_threshold: u32,
    pub timeout: u32,
    #[serde(alias = "vercel")]
    pub vercel_token: String,
    pub outage: OutageConfig,
    pub services: Vec<Service>
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct OutageConfig {
    pub enabled: bool,
    title: String,
    description: String,
    link: String
}
