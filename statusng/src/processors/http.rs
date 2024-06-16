use std::time::{Duration, Instant};

use log::{debug, info};
use ureq::{Error, Response};

use crate::models::service::{Service, ServiceStatus, kind::HttpService};
use crate::processors::{Processor, ProcessorResult};

pub struct Http;

fn do_request(service: &HttpService, timeout: Duration) -> (Result<Response, Error>, u128) {
    let url = service.get_url();
    info!("Requesting {}", url);

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
            debug!("ureq reported success (status code {})", response.status());
            response.status() == expected_code
        }
        Err(Error::Status(code, _)) => {
            debug!("ureq reported status code {}", code);
            code == expected_code
        }
        Err(Error::Transport(error)) => {
            debug!("ureq reported transport error: {:?}", &error);
            false
        }
    }
}

impl Processor<HttpService> for Http {
    fn process(service: HttpService, timeout: Duration, slow_threshold: u32) -> ProcessorResult {
        info!("Processing {}", service.host);
        let (response, ping) = do_request(&service, timeout);

        let success = check_success(response, service.expected_code);

        let status = match (&service.maintenance, success, ping) {
            (true, _, _) => ServiceStatus::Maintenance,
            (_, true, ping) if ping > slow_threshold as u128 => ServiceStatus::Unstable,
            (_, false, _) => ServiceStatus::Offline,
            _ => ServiceStatus::Online
        };

        ProcessorResult {
            status,
            ping: ping as u32,
            service: Service::Http(service)
        }
    }
}
