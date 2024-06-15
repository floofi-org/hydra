use std::time::Duration;
use simple_logger::SimpleLogger;
use log::{info, debug, LevelFilter, error};
use statusng::config::{load_data, BaseHistory, Config};
use statusng::export::PrivateAPI;

fn start_process(config: Config, mut history: BaseHistory) {
    info!("Processing config...");
    let timeout = Duration::from_millis(config.timeout as u64);
    let slow_threshold = config.slow_threshold;
    let mut api = PrivateAPI::new(if config.outage.enabled {
        Some(config.outage)
    } else {
        None
    });

    for base_service in config.services {
        info!("{}", &base_service);
        let result = base_service.process(timeout, slow_threshold);

        history.add_entry(&result.service, result.status);
        api.add(&result);

        info!("{}", result);
    }

    history.vacuum();
    if let Err(e) = history.sync() {
        error!("Failed to save history to disk: {}", e);
    }
    api.seal(history);
    if let Err(e) = api.sync() {
        error!("Failed to save private API data to disk: {}", e);
    }
}

fn main() {
    SimpleLogger::new()
        .with_module_level("rustls", LevelFilter::Info)
        .with_module_level("ureq", LevelFilter::Info)
        .init().unwrap();
    info!("statusng version {}.", statusng::VERSION);

    match load_data() {
        Ok((config, history)) => {
            info!("Loaded data.");

            let interval = config.interval;
            debug!("Refreshing every {} ms.", interval);
            start_process(config, history);
        }
        Err(err) => {
            error!("Failed to load data: {:?}", err);
        }
    }
}
