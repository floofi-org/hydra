use std::time::Duration;

use crate::models::service::kind::{http::*, tcp::*};
use crate::models::service::{Service, ServiceStatus};
use crate::models::service::kind::icmp::IcmpError;

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
    Tcp(TcpError),
    Icmp(IcmpError)
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

impl From<IcmpError> for ServiceError {
    fn from(value: IcmpError) -> Self {
        Self::Icmp(value)
    }
}
