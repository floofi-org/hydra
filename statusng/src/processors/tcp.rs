use std::net::{Shutdown, SocketAddr, TcpStream};
use std::time::{Duration, Instant};
use log::{error, info, warn};
use crate::config::{ServiceCode, TcpService};
use crate::error::StatusError;
use crate::processors::{Processor, ProcessorResult};

#[cfg(test)]
#[path="tests/tcp.rs"]
mod tests;

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
        info!(target: "tcp", "Processing {}", service.host);

        match get_valid_address(&service) {
            Ok(address) => {
                info!(target: "tcp", "Connecting to {}", &address);

                let start = Instant::now();
                let stream = TcpStream::connect_timeout(&address, timeout);
                let ping = start.elapsed().as_millis();

                match stream {
                    Ok(stream) => {
                        if let Err(e) = stream.shutdown(Shutdown::Both) {
                            warn!(target: "tcp", "Failed to close socket: {:?}", e);
                        }

                        ProcessorResult {
                            status: match (&service.maintenance, ping) {
                                (false, ping) if ping > slow_threshold as u128 => ServiceCode::Unstable,
                                (false, _) => ServiceCode::Online,
                                (true, _) => ServiceCode::Maintenance,
                            },
                            ping: ping as u32,
                            host: service.host
                        }
                    },
                    Err(e) => {
                        error!(target: "tcp", "Failed to connect: {:?}", e);

                        ProcessorResult {
                            status: if service.maintenance {
                                ServiceCode::Maintenance
                            } else {
                                ServiceCode::Offline
                            },
                            ping: ping as u32,
                            host: service.host
                        }
                    }
                }
            },
            Err(err) => {
                error!(target: "tcp", "{}", err);
                ProcessorResult {
                    status: ServiceCode::Offline,
                    ping: 0,
                    host: service.host
                }
            }
        }
    }
}
