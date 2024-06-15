use std::fs;
use log::debug;
use crate::error::StatusError;

pub mod base;
pub mod status;
pub mod address;
pub mod services;
pub mod history;
pub mod http;
pub mod tcp;

pub use base::*;
pub use status::*;
pub use services::*;
pub use history::*;
pub use http::*;
pub use tcp::*;

pub fn load_data() -> Result<(Config, BaseHistory), StatusError> {
    let config_raw = fs::read_to_string("./config.yaml")?;
    let config: Config = serde_yml::from_str(&config_raw)?;
    debug!("Done loading config.yaml.");

    let history_raw = fs::read_to_string("./history.json")?;
    let history: BaseHistory = serde_json::from_str(&history_raw)?;
    debug!("Done loading history.json.");

    Ok((config, history))
}
