mod config;
mod history;
mod processors;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

use std::fs;
use std::time::Duration;
use simple_logger::SimpleLogger;
use log::{info, debug, LevelFilter, error, warn};
use crate::config::{BaseHistory, Config, load_data, ServiceCode, ServiceConfig};
use crate::processors::{http, tcp, Processor};

fn display_summary(name: &String, status: &ServiceCode, ping: u32) {
    match status {
        ServiceCode::Online => info!("{name} is {status} with a ping of {ping} ms"),
        ServiceCode::Unstable => warn!("{name} is {status} with a ping of {ping} ms"),
        ServiceCode::Offline => error!("{name} is {status} with a ping of {ping} ms"),
        ServiceCode::Maintenance => info!("{name} is {status} with a ping of {ping} ms")
    }
}

fn start_process(config: Config, history: BaseHistory) {
    info!("Processing config...");
    let timeout = Duration::from_millis(config.timeout as u64);
    let slow_threshold = config.slow_threshold;

    for base_service in config.services {
        match base_service {
            ServiceConfig::HttpServiceConfig(service) => {
                info!("{} is an HTTP service, using the http processor", &service.host);
                let result = http::Http::process(&service, timeout, slow_threshold);
                display_summary(&service.host, &result.status, result.ping);
            }
            ServiceConfig::TcpServiceConfig(service) => {
                info!("{} is a TCP service, using the tcp processor", &service.host);
                let result = tcp::Tcp::process(&service, timeout, slow_threshold);
                display_summary(&service.host, &result.status, result.ping);
            }
        }
    }
}

fn main() {
    SimpleLogger::new()
        .with_module_level("rustls", LevelFilter::Info)
        .with_module_level("ureq", LevelFilter::Info)
        .init().unwrap();
    info!("Started statusng.");

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
