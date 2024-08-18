use std::io;
use std::net::{AddrParseError, IpAddr, Ipv4Addr, Shutdown, SocketAddr, TcpStream, ToSocketAddrs};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use log::{info, warn};
use serde::{Deserialize, Serialize};

use crate::models::service::{Service, ServiceProcessor};

#[derive(Deserialize, Serialize, Debug)]
pub struct IcmpService {}

#[derive(Debug)]
pub enum IcmpError {
    PingError(io::Error),
    Failed(i32)
}

impl ServiceProcessor<IcmpError> for IcmpService {
    fn process(&self, service: &Service, timeout: Duration) -> Result<u32, IcmpError> {
        info!("Connecting to {}", &service.host);

        let start = Instant::now();
        let mut ping_cmd = Command::new("ping");

        #[cfg(target_os = "windows")]
        ping_cmd.args([
            "/w", &timeout.as_secs().to_string(),
            "/n", "1",
            &service.host
        ]);

        #[cfg(target_os = "linux")]
        ping_cmd.args([
            "-W", &timeout.as_secs().to_string(),
            "-c", "1",
            &service.host
        ]);

        #[cfg(target_os = "macos")]
        ping_cmd.args([
            "-t", &timeout.as_secs().to_string(),
            "-c", "1",
            &service.host
        ]);

        ping_cmd.stdout(Stdio::null());
        ping_cmd.stderr(Stdio::null());

        let status = ping_cmd.status()?;
        let success = status.success();
        if success {
            let ping = start.elapsed().as_millis() as u32;
            Ok(ping)
        } else {
            Err(IcmpError::Failed(status.code().unwrap_or(-1)))
        }
    }
}

impl From<io::Error> for IcmpError {
    fn from(value: io::Error) -> Self {
        IcmpError::PingError(value)
    }
}
