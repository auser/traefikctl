#![allow(dead_code)]
use std::collections::HashMap;

use crate::{
    config::{
        deployment::{DeploymentConfig, DeploymentProtocol, DeploymentTarget},
        headers::HeadersConfig,
        host::{HostConfig, PathConfig},
        middleware::MiddlewareConfig,
        selections::{SelectionConfig, WithCookieConfig},
    },
    core::etcd_trait::EtcdPair,
    tracing::{init_tracing, LogConfig},
    TraefikConfig,
};

pub fn assert_contains_pair(pairs: &[EtcdPair], expected_value: &str) {
    let pair_strs: Vec<String> = pairs.iter().map(|p| p.to_string()).collect();
    assert!(pair_strs.contains(&expected_value.to_string()));
}

pub fn assert_does_not_contain_pair(pairs: &[EtcdPair], expected_value: &str) {
    let pair_strs: Vec<String> = pairs.iter().map(|p| p.to_string()).collect();
    assert!(!pair_strs.contains(&expected_value.to_string()));
}

pub fn read_test_config() -> TraefikConfig {
    let config_str = include_str!("../config/config.yml");
    serde_yaml::from_str(config_str).unwrap()
}

pub fn create_test_deployment() -> DeploymentConfig {
    DeploymentConfig {
        name: "blue".to_string(),
        target: DeploymentTarget::IpAndPort {
            ip: "10.0.0.1".to_string(),
            port: 8080,
        },
        weight: 100,
        selection: None,
        protocol: DeploymentProtocol::Http,
        middlewares: None,
    }
}

pub fn create_test_host() -> HostConfig {
    let mut host = HostConfig {
        domain: "test.example.com".to_string(),
        paths: Vec::new(),
        deployments: HashMap::new(),
        middlewares: vec![],
        selection: None,
        forward_host: false,
    };

    // Add a default blue deployment
    host.deployments.insert(
        "blue".to_string(),
        DeploymentConfig {
            name: "blue".to_string(),
            target: DeploymentTarget::IpAndPort {
                ip: "10.0.0.1".to_string(),
                port: 80,
            },
            weight: 100,
            selection: None,
            protocol: DeploymentProtocol::Http,
            middlewares: None,
        },
    );

    // Add a default path configuration with explicit middleware order
    host.paths.push(PathConfig {
        path: "/api".to_string(),
        deployments: {
            let mut map: HashMap<String, DeploymentConfig> = HashMap::new();
            map.insert(
                "blue".to_string(),
                DeploymentConfig {
                    name: "blue".to_string(),
                    target: DeploymentTarget::IpAndPort {
                        ip: "10.0.0.1".to_string(),
                        port: 80,
                    },
                    weight: 100,
                    selection: None,
                    protocol: DeploymentProtocol::Http,
                    middlewares: None,
                },
            );
            map
        },
        middlewares: vec!["enable-headers".to_string()], // Will be added after headers and strip
        strip_prefix: true,                              // Will be added second
        pass_through: true,
    });

    host
}

pub fn create_complex_test_config() -> TraefikConfig {
    let host_configs = vec![
        HostConfig::builder()
            .deployment(
                "green".to_string(),
                DeploymentConfig::builder()
                    .ip_and_port("10.0.0.2".to_string(), 8080)
                    .weight(50)
                    .selection(SelectionConfig {
                        with_cookie: Some(WithCookieConfig {
                            name: "test".to_string(),
                            value: Some("test".to_string()),
                        }),
                        ..Default::default()
                    })
                    .build(),
            )
            .deployment(
                "blue".to_string(),
                DeploymentConfig::builder()
                    .ip_and_port("10.0.0.3".to_string(), 8080)
                    .weight(50)
                    .build(),
            )
            .deployment(
                "catch-all".to_string(),
                DeploymentConfig::builder()
                    .ip_and_port("10.0.0.1".to_string(), 8080)
                    .weight(100)
                    .build(),
            )
            .domain("example.com".to_string())
            .path(
                "/test".to_string(),
                PathConfig::builder()
                    .deployment(
                        "test-1".to_string(),
                        DeploymentConfig::builder()
                            .ip_and_port("11.11.11.11".to_string(), 8080)
                            .weight(30)
                            .selection(SelectionConfig {
                                with_cookie: Some(WithCookieConfig {
                                    name: "test".to_string(),
                                    value: Some("test".to_string()),
                                }),
                                ..Default::default()
                            })
                            .build(),
                    )
                    .deployment(
                        "test-2".to_string(),
                        DeploymentConfig::builder()
                            .ip_and_port("22.22.22.22".to_string(), 8080)
                            .weight(40)
                            .build(),
                    )
                    .deployment(
                        "test-3".to_string(),
                        DeploymentConfig::builder()
                            .ip_and_port("33.33.33.33".to_string(), 8080)
                            .weight(30)
                            .build(),
                    )
                    .build(),
            )
            .build()
            .unwrap(),
        HostConfig::builder()
            .deployment(
                "bingo".to_string(),
                DeploymentConfig::builder()
                    .ip_and_port("1.1.1.1".to_string(), 8080)
                    .build(),
            )
            .build()
            .unwrap(),
    ];
    create_test_config(Some(host_configs))
}

pub fn create_test_config(host_configs: Option<Vec<HostConfig>>) -> TraefikConfig {
    let host_configs = host_configs.unwrap_or(vec![HostConfig {
        domain: "test.example.com".to_string(),
        selection: None,
        paths: vec![PathConfig {
            path: "/api".to_string(),
            deployments: HashMap::from([(
                "blue".to_string(),
                DeploymentConfig {
                    name: "blue".to_string(),
                    target: DeploymentTarget::IpAndPort {
                        ip: "10.0.0.1".to_string(),
                        port: 80,
                    },
                    weight: 100,
                    selection: None,
                    protocol: DeploymentProtocol::Http,
                    middlewares: None,
                },
            )]),
            middlewares: vec!["enable-headers".to_string()],
            strip_prefix: true,
            pass_through: false,
        }],
        forward_host: false,
        deployments: HashMap::from([("blue".to_string(), DeploymentConfig::default())]),
        middlewares: vec![],
    }]);

    TraefikConfig {
        etcd: Default::default(),
        middlewares: create_test_middleware(),
        hosts: host_configs,
        rule_prefix: "test".to_string(),
        name: Some("test".to_string()),
        description: Some("test".to_string()),
        services: None,
        entry_points: None,
    }
}

pub fn create_test_middleware() -> HashMap<String, MiddlewareConfig> {
    HashMap::from([
        (
            "enable-headers".to_string(),
            MiddlewareConfig {
                protocol: "http".to_string(),
                name: "enable-headers".to_string(),
                headers: Some(HeadersConfig {
                    headers: HashMap::from([
                        ("frameDeny".to_string(), "true".to_string()),
                        ("browserXssFilter".to_string(), "true".to_string()),
                        ("contentTypeNosniff".to_string(), "true".to_string()),
                        ("forceSTSHeader".to_string(), "true".to_string()),
                        ("stsIncludeSubdomains".to_string(), "true".to_string()),
                        ("stsPreload".to_string(), "true".to_string()),
                        ("stsSeconds".to_string(), "31536000".to_string()),
                        (
                            "customFrameOptionsValue".to_string(),
                            "SAMEORIGIN".to_string(),
                        ),
                    ]),
                    custom_request_headers: HashMap::from([
                        ("X-Forwarded-Proto".to_string(), "https".to_string()),
                        ("X-Forwarded-Port".to_string(), "443".to_string()),
                    ]),
                    custom_response_headers: HashMap::from([(
                        "Location".to_string(),
                        "".to_string(),
                    )]),
                    access_control_allow_methods: vec![
                        "GET".to_string(),
                        "POST".to_string(),
                        "OPTIONS".to_string(),
                    ],
                    access_control_allow_headers: vec![
                        "Content-Type".to_string(),
                        "Authorization".to_string(),
                    ],
                    access_control_expose_headers: vec!["Location".to_string()],
                    access_control_allow_origin_list: vec![],
                    add_vary_header: true,
                }),
                forward_auth: None,
            },
        ),
        (
            "handle-redirects".to_string(),
            MiddlewareConfig {
                protocol: "http".to_string(),
                name: "handle-redirects".to_string(),
                headers: Some(HeadersConfig {
                    custom_request_headers: HashMap::from([(
                        "Location".to_string(),
                        "".to_string(),
                    )]),
                    ..Default::default()
                }),
                forward_auth: None,
            },
        ),
    ])
}

pub fn init_test_tracing() {
    init_tracing(
        "traefik-ctl",
        &LogConfig {
            max_level: "debug".to_string(),
            filter: "debug".to_string(),
            rolling_file_path: None,
        },
    )
    .unwrap();
}
