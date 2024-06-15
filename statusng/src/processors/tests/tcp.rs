use std::time::Duration;
use crate::config::{HttpService, ServiceCategory, ServiceCode, ServiceHostingProvider, TcpService};
use crate::processors::{Processor, Tcp};
use crate::processors::tcp::get_valid_address;

#[cfg(test)]

#[test]
fn tcp_address_dns() {
    let service = TcpService {
        maintenance: false,
        host: "google.com".to_string(),
        port: 80,
        name: None,
        category: ServiceCategory::Websites,
        hosting_provider: ServiceHostingProvider::EquestriaDev,
        _legacy_id: "google".to_string()
    };

    let result = get_valid_address(&service);
    assert!(result.is_ok())
}

#[test]
fn tcp_address_ipv4() {
    let service = TcpService {
        maintenance: false,
        host: "8.8.8.8".to_string(),
        port: 80,
        name: None,
        category: ServiceCategory::Websites,
        hosting_provider: ServiceHostingProvider::EquestriaDev,
        _legacy_id: "google".to_string()
    };

    let result = get_valid_address(&service);
    assert!(result.is_ok())
}

#[test]
fn tcp_address_ipv6() {
    let service = TcpService {
        maintenance: false,
        host: "2001:4860:4860::8888".to_string(),
        port: 80,
        name: None,
        category: ServiceCategory::Websites,
        hosting_provider: ServiceHostingProvider::EquestriaDev,
        _legacy_id: "google".to_string()
    };

    let result = get_valid_address(&service);
    assert!(result.is_ok())
}

#[test]
fn tcp_address_invalid_dns() {
    let service = TcpService {
        maintenance: false,
        host: "invalid.invalid".to_string(),
        port: 80,
        name: None,
        category: ServiceCategory::Websites,
        hosting_provider: ServiceHostingProvider::EquestriaDev,
        _legacy_id: "invalid".to_string()
    };

    let result = get_valid_address(&service);
    assert!(result.is_err())
}

#[test]
fn tcp_address_invalid_ipv4() {
    let service = TcpService {
        maintenance: false,
        host: "256.0.918.12".to_string(),
        port: 80,
        name: None,
        category: ServiceCategory::Websites,
        hosting_provider: ServiceHostingProvider::EquestriaDev,
        _legacy_id: "invalid".to_string()
    };

    let result = get_valid_address(&service);
    assert!(result.is_err())
}

#[test]
fn tcp_address_invalid_ipv6() {
    let service = TcpService {
        maintenance: false,
        host: "ffff:ff:fff:f:f:fff:ffff".to_string(),
        port: 80,
        name: None,
        category: ServiceCategory::Websites,
        hosting_provider: ServiceHostingProvider::EquestriaDev,
        _legacy_id: "invalid".to_string()
    };

    let result = get_valid_address(&service);
    assert!(result.is_err())
}

#[test]
fn tcp_process() {
    let service = TcpService {
        maintenance: false,
        host: "google.com".to_string(),
        port: 80,
        name: None,
        category: ServiceCategory::Websites,
        hosting_provider: ServiceHostingProvider::EquestriaDev,
        _legacy_id: "google".to_string()
    };

    let result = Tcp::process(service, Duration::from_millis(2_000), 1_000);
    assert_eq!(result.status, ServiceCode::Online)
}

#[test]
fn tcp_maintenance() {
    let service = TcpService {
        maintenance: true,
        host: "google.com".to_string(),
        port: 80,
        name: None,
        category: ServiceCategory::Websites,
        hosting_provider: ServiceHostingProvider::EquestriaDev,
        _legacy_id: "google".to_string()
    };

    let result = Tcp::process(service, Duration::from_millis(2_000), 1_000);
    assert_eq!(result.status, ServiceCode::Maintenance)
}

#[test]
fn tcp_offline() {
    let service = TcpService {
        maintenance: true,
        host: "invalid.invalid".to_string(),
        port: 80,
        name: None,
        category: ServiceCategory::Websites,
        hosting_provider: ServiceHostingProvider::EquestriaDev,
        _legacy_id: "invalid".to_string()
    };

    let result = Tcp::process(service, Duration::from_millis(2_000), 1_000);
    assert_eq!(result.status, ServiceCode::Offline)
}

#[test]
fn tcp_threshold() {
    let service = TcpService {
        maintenance: false,
        host: "google.com".to_string(),
        port: 80,
        name: None,
        category: ServiceCategory::Websites,
        hosting_provider: ServiceHostingProvider::EquestriaDev,
        _legacy_id: "google".to_string()
    };

    let result = Tcp::process(service, Duration::from_millis(2_000), 0);
    assert_eq!(result.status, ServiceCode::Unstable)
}
