use std::time::Duration;

use crate::models::service::{Service, ServiceStatus};
use crate::models::service::kind::{
    http::*,
    tcp::*,
};

pub type ServiceResult = Result<u32, ServiceError>;

pub trait ServiceProcessor<E> {
    fn process(&self, service: &Service, timeout: Duration) -> Result<u32, E>;
}

pub struct ProcessorResult {
    pub status: ServiceStatus,
    pub ping: u32,
}

#[derive(Debug)]
pub enum ServiceError {
    Http(HttpError),
    Tcp(TcpError)
}

impl From<HttpError> for ServiceError {
    fn from(value: HttpError) -> Self {
        Self::Http(value)
    }
}

impl From<TcpError> for ServiceError {
    fn from(value: TcpError) -> Self {
        Self::Tcp(value)
    }
}
