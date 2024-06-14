use std::net::{AddrParseError, Shutdown, SocketAddr, TcpStream};
use std::time::{Duration, Instant};
use log::{debug, error, info, warn};
use ureq::{Error, Response};
use crate::config::{HttpServiceConfig, ServiceCode, TcpServiceConfig};
use crate::processors::{Processor, ProcessorResult};

pub struct Tcp;

impl Processor<TcpServiceConfig> for Tcp {
    fn process(service: &TcpServiceConfig, timeout: Duration, slow_threshold: u32) -> ProcessorResult {
        info!(target: "tcp", "Processing {}", service.host);

        let address = service.get_address();

        match address {
            Ok(mut addr) => {
                let addr = addr.next();
                let addr = match addr {
                    Some(a) => a,
                    None => {
                        error!(target: "tcp", "Found no valid addresses from {}:{}", &service.host, service.port);

                        return ProcessorResult {
                            status: ServiceCode::Offline,
                            ping: 0,
                        };
                    }
                };

                info!(target: "tcp", "Connecting to {}", &addr);

                let start = Instant::now();
                let stream = TcpStream::connect_timeout(&addr, timeout);
                let ping = start.elapsed().as_millis();

                match stream {
                    Ok(stream) => {
                        if let Err(e) = stream.shutdown(Shutdown::Both) {
                            warn!(target: "tcp", "Failed to close socket: {:?}", e);
                        }

                        ProcessorResult {
                            status: ServiceCode::Online,
                            ping: ping as u32,
                        }
                    },
                    Err(e) => {
                        error!(target: "tcp", "Failed to connect: {:?}", e);

                        ProcessorResult {
                            status: ServiceCode::Offline,
                            ping: ping as u32,
                        }
                    }
                }
            }
            Err(err) => {
                error!(target: "tcp", "Failed to parse address {}:{}: {:?}", &service.host, service.port, err);

                ProcessorResult {
                    status: ServiceCode::Offline,
                    ping: 0,
                }
            }
        }
    }
}
