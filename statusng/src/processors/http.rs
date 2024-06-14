use std::time::{Duration, Instant};
use log::{debug, info};
use ureq::{Error, Response};
use crate::config::{HttpServiceConfig, ServiceCode};
use crate::processors::{Processor, ProcessorResult};

pub struct Http;

fn do_request(service: &HttpServiceConfig, timeout: Duration) -> (Result<Response, Error>, u128) {
    let url = service.get_url();
    info!(target: "http", "Requesting {}", url);

    let start = Instant::now();
    let response = ureq::get(&url)
        .timeout(timeout)
        .set("User-Agent",
             &format!("Mozilla/5.0 (Equestriadev; statusng; +https://status.equestria.dev) statusng/{}", crate::VERSION))
        .call();
    let ping = start.elapsed().as_millis();

    (response, ping)
}

fn check_success(response: Result<Response, Error>, expected_code: u16) -> bool {
    match response {
        Ok(response) => {
            debug!(target: "http", "ureq reported success (status code {})", response.status());
            response.status() == expected_code
        }
        Err(Error::Status(code, _)) => {
            debug!(target: "http", "ureq reported status code {}", code);
            code == expected_code
        }
        Err(Error::Transport(error)) => {
            debug!(target: "http", "ureq reported transport error: {:?}", &error);
            false
        }
    }
}

impl Processor<HttpServiceConfig> for Http {
    fn process(service: &HttpServiceConfig, timeout: Duration, slow_threshold: u32) -> ProcessorResult {
        info!(target: "http", "Processing {}", service.host);
        let (response, ping) = do_request(service, timeout);

        let success = check_success(response, service.expected_code);

        let status = match (service.maintenance, success, ping) {
            (true, _, _) => ServiceCode::Maintenance,
            (_, true, ping) if ping > slow_threshold as u128 => ServiceCode::Unstable,
            (_, false, _) => ServiceCode::Offline,
            _ => ServiceCode::Online
        };

        ProcessorResult {
            status,
            ping: ping as u32,
        }
    }
}
