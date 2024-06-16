use std::fs;
use std::time::Duration;

use log::{debug, error, info};

use statusng::config::{BaseHistory, Config};
use statusng::error::StatusError;
use statusng::export::PrivateAPI;

pub struct App {
    config: Config,
    api: PrivateAPI,
    history: BaseHistory,
}

impl App {
    pub fn build() -> Result<Self, StatusError> {
        let config = fs::read_to_string("./self.config.yaml")?;
        let config: Config = serde_yml::from_str(&config)?;
        debug!("Done loading self.config.yaml.");

        let history = fs::read_to_string("./history.json")?;
        let history = serde_json::from_str(&history)?;
        debug!("Done loading history.json.");

        let outage = config.outage.enabled.then_some(config.outage.clone());
        let api = PrivateAPI::new(outage);

        Ok(Self {
            config,
            api,
            history,
        })
    }

    pub fn run(mut self) {
        info!("Processing config...");
        debug!("Refreshing every {} ms.", self.config.interval);

        let timeout = Duration::from_millis(self.config.timeout as u64);
        let slow_threshold = self.config.slow_threshold;

        for base_service in self.config.services {
            info!("{}", base_service);
            let result = base_service.process(timeout, slow_threshold);

            self.history.add_entry(&result.service, result.status);
            self.api.add(&result);

            info!("{}", result);
        }

        self.history.vacuum();
        if let Err(e) = self.history.sync() {
            error!("Failed to save history to disk: {}", e);
        }

        self.api.seal(self.history);
        if let Err(e) = self.api.sync() {
            error!("Failed to save private API data to disk: {}", e);
        }
    }
}
