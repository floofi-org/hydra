use std::time::Duration;
use crate::config::ServiceCode;

pub mod http;
pub mod tcp;

pub use http::Http;
pub use tcp::Tcp;

pub struct ProcessorResult {
    pub status: ServiceCode,
    pub ping: u32
}

pub trait Processor<C> {
    fn process(service: &C, timeout: Duration, slow_threshold: u32) -> ProcessorResult;
}
