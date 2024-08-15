use std::fs;
use std::time::Duration;

use log::{debug, error, info, warn};

use statusng::error::StatusError;
use statusng::export::private::PrivateAPI;
use statusng::export::public::PublicAPI;
use statusng::models::service::ServiceStatus;
use statusng::models::{Config, History};

pub struct App {
    pub(crate) config: Config,
    api: PrivateAPI,
    history: History,
}

impl App {
    pub fn build() -> Result<Self, StatusError> {
        let config = fs::read_to_string("./config.toml")?;
        let config: Config = toml::from_str(&config)?;
        debug!("Done loading config.toml.");

        let history = fs::read("./history.dat")?;
        let history = History::from_bytes(&history)?;
        debug!("Done loading history.dat.");

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

        let timeout = Duration::from_millis(self.config.timeout as u64);
        let slow_threshold = self.config.slow_threshold;

        for service in self.config.services {
            info!("{}", service);
            let result = service.process(timeout, slow_threshold);

            self.history.add_entry(&service, result.status);
            self.api.add(&service, &result);

            match result.status {
                ServiceStatus::Online => {
                    info!("{}: Online (ping: {} ms)", service.get_label(), result.ping)
                }
                ServiceStatus::Unstable => warn!(
                    "{}: Unstable (ping: {} ms)",
                    service.get_label(),
                    result.ping
                ),
                ServiceStatus::Offline => error!("{}: Offline", service.get_label()),
                ServiceStatus::Maintenance => info!(
                    "{}: Maintenance (ping: {} ms)",
                    service.get_label(),
                    result.ping
                ),
            }
        }

        self.history.vacuum();
        if let Err(e) = self.history.sync() {
            error!("Failed to save history to disk: {}", e);
        }

        info!("Saving public API data to disk and sending to Vercel...");
        let public_api = PublicAPI::from_private_api(&self.api);

        if let Some(token) = &self.config.vercel_token {
            if let Err(e) = public_api.sync(token) {
                error!("Failed to save public API data to disk: {}", e);
            }
        }

        info!("Saving private API data to disk and sending to Vercel...");
        self.api.seal(self.history);

        if let Some(token) = &self.config.vercel_token {
            if let Err(e) = self.api.sync(token) {
                error!("Failed to save private API data to disk: {}", e);
            }
        }
    }
}
