use std::fmt::{Display, Formatter};
use std::time::Duration;
use crate::models::service::{Service, ServiceStatus};

pub mod http;
pub mod tcp;

pub use http::Http;
pub use tcp::Tcp;

pub struct ProcessorResult {
    pub status: ServiceStatus,
    pub ping: u32,
    pub service: Service
}

pub trait Processor<C> {
    fn process(service: C, timeout: Duration, slow_threshold: u32) -> ProcessorResult;
}

impl Display for ProcessorResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is {} with a ping of {} ms", self.service.get_label(), self.status, self.ping)
    }
}
