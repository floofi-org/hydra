use std::net::{Shutdown, SocketAddr, TcpStream};
use std::time::{Duration, Instant};
use log::{error, info, warn};
use crate::config::{Service, ServiceStatus, TcpService};
use crate::error::StatusError;
use crate::processors::{Processor, ProcessorResult};

pub struct Tcp;

fn get_valid_address(service: &TcpService) -> Result<SocketAddr, StatusError> {
    let mut address = service.get_address()?;

    let addr = address.next();

    match addr {
        Some(a) => Ok(a),
        None => {
            Err(StatusError::from(format!("Found no valid addresses from {}:{}", &service.host, service.port)))
        }
    }
}

impl Processor<TcpService> for Tcp {
    fn process(service: TcpService, timeout: Duration, slow_threshold: u32) -> ProcessorResult {
        info!("Processing {}", service.host);

        match get_valid_address(&service) {
            Ok(address) => {
                info!("Connecting to {}", &address);

                let start = Instant::now();
                let stream = TcpStream::connect_timeout(&address, timeout);
                let ping = start.elapsed().as_millis();

                match stream {
                    Ok(stream) => {
                        if let Err(e) = stream.shutdown(Shutdown::Both) {
                            warn!("Failed to close socket: {:?}", e);
                        }

                        ProcessorResult {
                            status: match (&service.maintenance, ping) {
                                (false, ping) if ping > slow_threshold as u128 => ServiceStatus::Unstable,
                                (false, _) => ServiceStatus::Online,
                                (true, _) => ServiceStatus::Maintenance,
                            },
                            ping: ping as u32,
                            service: Service::Tcp(service)
                        }
                    },
                    Err(e) => {
                        error!("Failed to connect: {:?}", e);

                        ProcessorResult {
                            status: if service.maintenance {
                                ServiceStatus::Maintenance
                            } else {
                                ServiceStatus::Offline
                            },
                            ping: ping as u32,
                            service: Service::Tcp(service)
                        }
                    }
                }
            },
            Err(err) => {
                error!("{}", err);
                ProcessorResult {
                    status: ServiceStatus::Offline,
                    ping: 0,
                    service: Service::Tcp(service)
                }
            }
        }
    }
}
