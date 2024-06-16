use std::net::{Shutdown, TcpStream};
use std::time::{Duration, Instant};

use log::{error, info, warn};

use crate::models::service::{Service, ServiceStatus, kind::TcpService};
use crate::processors::{Processor, ProcessorResult};

pub struct Tcp;

impl Processor<TcpService> for Tcp {
    fn process(service: TcpService, timeout: Duration, slow_threshold: u32) -> ProcessorResult {
        info!("Processing {}", service.host);

        let address = match service.get_socket() {
            Ok(Some(address)) => address,
            Ok(None) => {
                error!("Found no valid addresses from {}", service.get_address());
                return ProcessorResult {
                    status: ServiceStatus::Offline,
                    ping: 0,
                    service: Service::Tcp(service),
                };
            }
            Err(e) => {
                error!("{}", e);
                return ProcessorResult {
                    status: ServiceStatus::Offline,
                    ping: 0,
                    service: Service::Tcp(service),
                };
            }
        };

        info!("Connecting to {}", &address);

        let start = Instant::now();
        let stream = match TcpStream::connect_timeout(&address, timeout) {
            Ok(stream) => stream,
            Err(e) => {
                error!("Failed to connect: {:?}", e);
                return ProcessorResult {
                    status: if service.maintenance {
                        ServiceStatus::Maintenance
                    } else {
                        ServiceStatus::Offline
                    },
                    ping: 0,
                    service: Service::Tcp(service)
                }
            }
        };

        let ping = start.elapsed().as_millis() as u32;

        if let Err(e) = stream.shutdown(Shutdown::Both) {
            warn!("Failed to close socket: {:?}", e);
        }

        ProcessorResult {
            status: match (&service.maintenance, ping) {
                (false, ping) if ping > slow_threshold => ServiceStatus::Unstable,
                (false, _) => ServiceStatus::Online,
                (true, _) => ServiceStatus::Maintenance,
            },
            ping: ping as u32,
            service: Service::Tcp(service)
        }
    }
}
