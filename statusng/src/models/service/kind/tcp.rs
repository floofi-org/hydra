use std::io;
use std::net::{Shutdown, SocketAddr, TcpStream, ToSocketAddrs};
use std::time::{Duration, Instant};

use log::{info, warn};
use serde::Deserialize;

use crate::models::service::{Service, ServiceProcessor};

#[derive(Deserialize, Debug)]
pub struct TcpService {}

pub enum TcpError {
    ConnectionError(io::Error),
    AddressNotFound
}

impl TcpService {
    pub fn get_socket(&self, host: &str, port: u16) -> Result<SocketAddr, TcpError> {
        let mut addrs = self.get_address(host, port).to_socket_addrs()?;

        addrs.next().ok_or(TcpError::AddressNotFound)
    }

    pub fn get_address(&self, host: &str, port: u16) -> String {
        format!("{}:{}", host, port)
    }
}

impl ServiceProcessor<TcpError> for TcpService {
    fn process(&self, service: &Service, timeout: Duration) -> Result<u32, TcpError> {
        let socket = self.get_socket(&service.host, service.port)?;

        info!("Connecting to {}", &socket);

        let start = Instant::now();
        let stream = TcpStream::connect_timeout(&socket, timeout)?;
        let ping = start.elapsed().as_millis() as u32;

        if let Err(e) = stream.shutdown(Shutdown::Both) {
            warn!("Failed to close socket: {:?}", e);
        }

        Ok(ping)
    }
}


impl From<io::Error> for TcpError {
    fn from(value: io::Error) -> Self {
        TcpError::ConnectionError(value)
    }
}
