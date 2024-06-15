use std::io;
use std::net::{SocketAddr, ToSocketAddrs};
use std::vec::IntoIter;
use crate::config::{HttpService, TcpService};

impl HttpService {
    //noinspection HttpUrlsUsage - Stupid RustRover
    pub fn get_url(&self) -> String {
        let mut url = String::from("");

        url.push_str(if self.tls {
            "https://"
        } else {
            "http://"
        });

        url.push_str(&self.host);

        if (self.tls && self.port != 443) ||
            (!self.tls && self.port != 80) {
            url.push_str(&format!(":{}", self.port));
        }

        if self.url.starts_with('/') {
            url.push_str(&self.url)
        } else {
            url.push('/');
            url.push_str(&self.url);
        };

        url
    }
}

impl TcpService {
    pub fn get_address(&self) -> io::Result<IntoIter<SocketAddr>> {
        format!("{}:{}", self.host, self.port)
            .to_socket_addrs()
    }
}