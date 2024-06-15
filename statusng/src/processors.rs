use std::fmt::{Display, Formatter};
use std::time::Duration;
use crate::config::ServiceCode;

pub mod http;
pub mod tcp;

pub use http::Http;
pub use tcp::Tcp;

pub struct ProcessorResult {
    pub status: ServiceCode,
    pub ping: u32,
    pub host: String
}

pub trait Processor<C> {
    fn process(service: &C, timeout: Duration, slow_threshold: u32) -> ProcessorResult;
}

impl Display for ProcessorResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is {} with a ping of {} ms", self.host, self.status, self.ping)
    }
}
