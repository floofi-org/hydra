#[cfg(test)]
use std::time::Duration;
use crate::config::{HttpService, ServiceCategory, ServiceCode, ServiceHostingProvider};
use crate::processors::{Http, Processor};
use crate::processors::http::{check_success, do_request};

#[test]
fn http_request() {
    let service = HttpService {
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
    };

    let response = do_request(&service, Duration::from_millis(2_000));
    assert!(response.0.is_ok())
}

#[test]
fn http_success() {
    let service = HttpService {
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
    };

    let response = do_request(&service, Duration::from_millis(2_000));
    assert!(check_success(response.0, 200))
}

#[test]
fn http_process() {
    let service = HttpService {
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
    };

    let response = Http::process(service, Duration::from_millis(2_000), 1_000);
    assert_eq!(response.status, ServiceCode::Online)
}

#[test]
fn http_invalid() {
    let service = HttpService {
        maintenance: false,
        host: "google.com".to_string(),
        port: 443,
        name: None,
        category: ServiceCategory::Websites,
        hosting_provider: ServiceHostingProvider::EquestriaDev,
        _legacy_id: "google".to_string(),
        url: "teapot".to_string(),
        expected_code: 200,
        tls: true
    };

    let response = Http::process(service, Duration::from_millis(2_000), 1_000);
    assert_eq!(response.status, ServiceCode::Offline)
}

#[test]
fn http_threshold() {
    let service = HttpService {
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
    };

    let response = Http::process(service, Duration::from_millis(2_000), 0);
    assert_eq!(response.status, ServiceCode::Unstable)
}

#[test]
fn http_offline() {
    let service = HttpService {
        maintenance: false,
        host: "invalid.invalid".to_string(),
        port: 443,
        name: None,
        category: ServiceCategory::Websites,
        hosting_provider: ServiceHostingProvider::EquestriaDev,
        _legacy_id: "invalid".to_string(),
        url: "".to_string(),
        expected_code: 200,
        tls: true
    };

    let response = Http::process(service, Duration::from_millis(2_000), 1_000);
    assert_eq!(response.status, ServiceCode::Offline)
}

#[test]
fn http_maintenance() {
    let service = HttpService {
        maintenance: true,
        host: "invalid.invalid".to_string(),
        port: 443,
        name: None,
        category: ServiceCategory::Websites,
        hosting_provider: ServiceHostingProvider::EquestriaDev,
        _legacy_id: "invalid".to_string(),
        url: "".to_string(),
        expected_code: 200,
        tls: true
    };

    let response = Http::process(service, Duration::from_millis(2_000), 1_000);
    assert_eq!(response.status, ServiceCode::Maintenance)
}
