use std::time::Duration;
use crate::config::{HttpService, Service, ServiceCategory, ServiceCode, ServiceHostingProvider, TcpService};

#[cfg(test)]

#[test]
fn services_process_http() {
    let service = Service::HttpServiceConfig(HttpService {
        maintenance: false,
        host: "google.com".to_string(),
        port: 443,
        name: None,
        category: ServiceCategory::Websites,
        hosting_provider: ServiceHostingProvider::EquestriaDev,
        _legacy_id: "google".to_string(),
        url: "".to_string(),
        expected_code: 200,
        tls: true
    });

    assert_eq!(service.process(Duration::from_millis(2_000), 1_000).status, ServiceCode::Online)
}

#[test]
fn services_process_tcp() {
    let service = Service::TcpServiceConfig(TcpService {
        maintenance: false,
        host: "google.com".to_string(),
        port: 443,
        name: None,
        category: ServiceCategory::Websites,
        hosting_provider: ServiceHostingProvider::EquestriaDev,
        _legacy_id: "google".to_string()
    });

    assert_eq!(service.process(Duration::from_millis(2_000), 1_000).status, ServiceCode::Online)
}

#[test]
fn services_display_http() {
    let service = Service::HttpServiceConfig(HttpService {
        maintenance: false,
        host: "google.com".to_string(),
        port: 443,
        name: None,
        category: ServiceCategory::Websites,
        hosting_provider: ServiceHostingProvider::EquestriaDev,
        _legacy_id: "google".to_string(),
        url: "".to_string(),
        expected_code: 200,
        tls: true
    });

    assert_eq!(format!("{}", service), "google.com is an http service")
}

#[test]
fn services_display_tcp() {
    let service = Service::TcpServiceConfig(TcpService {
        maintenance: false,
        host: "myfunnyservice.invalid".to_string(),
        port: 443,
        name: None,
        category: ServiceCategory::Websites,
        hosting_provider: ServiceHostingProvider::EquestriaDev,
        _legacy_id: "google".to_string()
    });

    assert_eq!(format!("{}", service), "myfunnyservice.invalid is a tcp service")
}
