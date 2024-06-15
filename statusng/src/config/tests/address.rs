use crate::config::{HttpService, ServiceCategory, ServiceHostingProvider, TcpService};

#[cfg(test)]

#[test]
fn address_http_path_root() {
    let service = HttpService {
        maintenance: false,
        host: "sample.invalid".to_string(),
        port: 80,
        name: None,
        category: ServiceCategory::Websites,
        hosting_provider: ServiceHostingProvider::EquestriaDev,
        _legacy_id: "invalid".to_string(),
        url: "".to_string(),
        expected_code: 200,
        tls: false
    };

    assert_eq!(service.get_url(), "http://sample.invalid/")
}

#[test]
fn address_http_path_sub() {
    let service = HttpService {
        maintenance: false,
        host: "sample.invalid".to_string(),
        port: 80,
        name: None,
        category: ServiceCategory::Websites,
        hosting_provider: ServiceHostingProvider::EquestriaDev,
        _legacy_id: "invalid".to_string(),
        url: "some/path".to_string(),
        expected_code: 200,
        tls: false
    };

    assert_eq!(service.get_url(), "http://sample.invalid/some/path")
}

#[test]
fn address_http_path_leading() {
    let service = HttpService {
        maintenance: false,
        host: "sample.invalid".to_string(),
        port: 80,
        name: None,
        category: ServiceCategory::Websites,
        hosting_provider: ServiceHostingProvider::EquestriaDev,
        _legacy_id: "invalid".to_string(),
        url: "/some/path".to_string(),
        expected_code: 200,
        tls: false
    };

    assert_eq!(service.get_url(), "http://sample.invalid/some/path")
}

#[test]
fn address_http_path_leading_root() {
    let service = HttpService {
        maintenance: false,
        host: "sample.invalid".to_string(),
        port: 80,
        name: None,
        category: ServiceCategory::Websites,
        hosting_provider: ServiceHostingProvider::EquestriaDev,
        _legacy_id: "invalid".to_string(),
        url: "/".to_string(),
        expected_code: 200,
        tls: false
    };

    assert_eq!(service.get_url(), "http://sample.invalid/")
}

#[test]
fn address_http_port_custom() {
    let service = HttpService {
        maintenance: false,
        host: "sample.invalid".to_string(),
        port: 8080,
        name: None,
        category: ServiceCategory::Websites,
        hosting_provider: ServiceHostingProvider::EquestriaDev,
        _legacy_id: "invalid".to_string(),
        url: "".to_string(),
        expected_code: 200,
        tls: false
    };

    assert_eq!(service.get_url(), "http://sample.invalid:8080/")
}

#[test]
fn address_http_ssl() {
    let service = HttpService {
        maintenance: false,
        host: "sample.invalid".to_string(),
        port: 443,
        name: None,
        category: ServiceCategory::Websites,
        hosting_provider: ServiceHostingProvider::EquestriaDev,
        _legacy_id: "invalid".to_string(),
        url: "".to_string(),
        expected_code: 200,
        tls: true
    };

    assert_eq!(service.get_url(), "https://sample.invalid/")
}

#[test]
fn address_http_ssl_custom() {
    let service = HttpService {
        maintenance: false,
        host: "sample.invalid".to_string(),
        port: 80,
        name: None,
        category: ServiceCategory::Websites,
        hosting_provider: ServiceHostingProvider::EquestriaDev,
        _legacy_id: "invalid".to_string(),
        url: "".to_string(),
        expected_code: 200,
        tls: true
    };

    assert_eq!(service.get_url(), "https://sample.invalid:80/")
}

#[test]
fn address_tcp_invalid() {
    let service = TcpService {
        maintenance: false,
        host: "sample.invalid".to_string(),
        port: 80,
        name: None,
        category: ServiceCategory::Websites,
        hosting_provider: ServiceHostingProvider::EquestriaDev,
        _legacy_id: "invalid".to_string()
    };

    assert!(service.get_address().is_err())
}

#[test]
fn address_tcp_multiple() {
    let service = TcpService {
        maintenance: false,
        host: "raw.githubusercontent.com".to_string(),
        port: 80,
        name: None,
        category: ServiceCategory::Websites,
        hosting_provider: ServiceHostingProvider::EquestriaDev,
        _legacy_id: "invalid".to_string()
    };

    assert!(service.get_address().unwrap().count() > 1)
}

#[test]
fn address_tcp_ipv4() {
    let service = TcpService {
        maintenance: false,
        host: "8.8.8.8".to_string(),
        port: 80,
        name: None,
        category: ServiceCategory::Websites,
        hosting_provider: ServiceHostingProvider::EquestriaDev,
        _legacy_id: "invalid".to_string()
    };

    assert_eq!(format!("{}", service.get_address().unwrap().next().unwrap()), "8.8.8.8:80")
}

#[test]
fn address_tcp_ipv6() {
    let service = TcpService {
        maintenance: false,
        host: "2001:4860:4860::8888".to_string(),
        port: 80,
        name: None,
        category: ServiceCategory::Websites,
        hosting_provider: ServiceHostingProvider::EquestriaDev,
        _legacy_id: "invalid".to_string()
    };

    assert_eq!(format!("{}", service.get_address().unwrap().next().unwrap()), "[2001:4860:4860::8888]:80")
}

#[test]
fn address_tcp_dns() {
    let service = TcpService {
        maintenance: false,
        host: "dns.google".to_string(),
        port: 80,
        name: None,
        category: ServiceCategory::Websites,
        hosting_provider: ServiceHostingProvider::EquestriaDev,
        _legacy_id: "invalid".to_string()
    };
    
    let address = format!("{}", service.get_address().unwrap().next().unwrap());

    assert!(address == "8.8.8.8:80" || address == "8.8.4.4:80" ||
        address == "[2001:4860:4860::8888]:80" || address == "[2001:4860:4860::8844]:80")
}
