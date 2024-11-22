use std::collections::{HashMap, HashSet};

use crate::{
    core::{util::validate_is_alphanumeric, Validate},
    error::{TraefikError, TraefikResult},
};

use super::{deployment::DeploymentConfig, selections::SelectionConfig};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct HostConfig {
    /// The domain of the host
    pub domain: String,
    /// The paths of the host
    #[serde(default)]
    pub paths: Vec<PathConfig>,
    /// The deployments of the host
    #[serde(default)]
    pub deployments: HashMap<String, DeploymentConfig>,
    /// The middlewares of the host
    #[serde(default)]
    pub middlewares: Vec<String>,
    /// The selection configuration of the host
    /// This is flattened to allow for more complex selection configurations
    /// such as weighted selections.
    #[serde(default, flatten)]
    pub selection: Option<SelectionConfig>,
}

impl Validate for HostConfig {
    fn validate(&self) -> TraefikResult<()> {
        // validate domain is not empty
        if self.domain.is_empty() {
            return Err(TraefikError::HostConfig("domain is empty".to_string()));
        }

        // validate paths if they exist
        for path in self.paths.iter() {
            path.validate()?;
        }

        // validate deployments if they exist
        for deployment in self.deployments.values() {
            deployment.validate()?;
        }

        if self.selection.is_some() {
            self.selection.as_ref().unwrap().validate()?;
        }

        self.validate_paths()?;

        Ok(())
    }
}

impl HostConfig {
    pub fn get_deployment(&self, name: &str) -> Option<&DeploymentConfig> {
        self.deployments.get(name)
    }

    fn validate_paths(&self) -> TraefikResult<()> {
        self.validate_has_deployments()?;

        let mut path_set = HashSet::new();
        for path in &self.paths {
            validate_is_alphanumeric(&path.path)?;
            self.validate_path(path)?;
            if !path_set.insert(&path.path) {
                return Err(TraefikError::HostConfig(format!(
                    "Duplicate path '{}'",
                    path.path
                )));
            }
        }

        self.validate_has_valid_middlewares()?;
        self.validate_deployment_ports()?;
        self.validate_deployment_weights()?;
        self.validate_middleware_references()?;

        Ok(())
    }

    fn validate_path(&self, path: &PathConfig) -> TraefikResult<()> {
        if path.path.contains("//") {
            return Err(TraefikError::ParseError(format!(
                "Path cannot contain //: {}",
                path.path
            )));
        }

        if !path.path.starts_with('/') {
            return Err(TraefikError::ParseError(format!(
                "Path must start with /: {}",
                path.path
            )));
        }

        Ok(())
    }

    fn validate_has_deployments(&self) -> TraefikResult<()> {
        if self.deployments.is_empty() {
            return Err(TraefikError::DeploymentError(format!(
                "No deployments defined for {}",
                self.domain
            )));
        }

        Ok(())
    }

    fn validate_has_valid_middlewares(&self) -> TraefikResult<()> {
        for middleware in &self.middlewares {
            if middleware.is_empty() {
                return Err(TraefikError::ParseError(format!(
                    "Middleware cannot be empty: {}",
                    middleware
                )));
            }
        }

        Ok(())
    }

    fn validate_deployment_ports(&self) -> TraefikResult<()> {
        for deployment in self.deployments.values() {
            if deployment.port == 0 {
                return Err(TraefikError::DeploymentError(
                    "Invalid port 0 for deployment".to_string(),
                ));
            }
        }

        Ok(())
    }

    fn validate_deployment_weights(&self) -> TraefikResult<()> {
        let total_weight: u8 = self.deployments.values().map(|d| d.weight).sum();
        if total_weight > 0 && total_weight != 100 {
            return Err(TraefikError::HostConfig(format!(
                "Deployment weights for {} must sum to 100, got {}",
                self.domain, total_weight
            )));
        }

        Ok(())
    }

    fn validate_middleware_references(&self) -> TraefikResult<()> {
        // Validate root middleware references
        for middleware in &self.middlewares {
            if middleware.is_empty() {
                return Err(TraefikError::MiddlewareConfig(
                    "Empty middleware reference in host configuration".to_string(),
                ));
            }
        }

        Ok(())
    }
}

impl HostConfig {
    pub fn builder() -> HostConfigBuilder {
        HostConfigBuilder::default()
    }
}

#[derive(Default)]
pub struct HostConfigBuilder {
    domain: String,
    deployments: HashMap<String, DeploymentConfig>,
    paths: HashMap<String, PathConfig>,
    middlewares: Vec<String>,
}

impl HostConfigBuilder {
    pub fn domain(mut self, domain: String) -> Self {
        self.domain = domain;
        self
    }

    pub fn deployment(mut self, name: String, deployment: DeploymentConfig) -> Self {
        self.deployments.insert(name, deployment);
        self
    }

    pub fn path(mut self, path: String, path_config: PathConfig) -> Self {
        self.paths.insert(path, path_config);
        self
    }

    pub fn middleware(mut self, middleware: String) -> Self {
        self.middlewares.push(middleware);
        self
    }

    pub fn build(self) -> TraefikResult<HostConfig> {
        let host_config = HostConfig {
            domain: self.domain,
            deployments: self.deployments,
            paths: self.paths.into_values().collect(),
            middlewares: self.middlewares,
            selection: None,
        };
        Ok(host_config)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PathConfig {
    /// The path of the host
    pub path: String,
    /// The deployments of the path
    pub deployments: HashMap<String, DeploymentConfig>,
    /// The middlewares of the path
    #[serde(default)]
    pub middlewares: Vec<String>,
    /// Whether to strip the prefix from the path
    #[serde(default)]
    pub strip_prefix: bool,
    /// Whether to pass through the path to the backend
    #[serde(default)]
    pub pass_through: bool,
}

impl PathConfig {
    pub fn builder() -> PathConfigBuilder {
        PathConfigBuilder::default()
    }
}

#[derive(Default)]
pub struct PathConfigBuilder {
    path: String,
    deployments: HashMap<String, DeploymentConfig>,
    middlewares: Vec<String>,
    strip_prefix: bool,
    pass_through: bool,
}

impl PathConfigBuilder {
    pub fn path(mut self, path: String) -> Self {
        self.path = path;
        self
    }

    pub fn deployment(mut self, name: String, deployment: DeploymentConfig) -> Self {
        self.deployments.insert(name, deployment);
        self
    }

    pub fn middleware(mut self, middleware: String) -> Self {
        self.middlewares.push(middleware);
        self
    }

    pub fn strip_prefix(mut self, strip_prefix: bool) -> Self {
        self.strip_prefix = strip_prefix;
        self
    }

    pub fn pass_through(mut self, pass_through: bool) -> Self {
        self.pass_through = pass_through;
        self
    }

    pub fn build(self) -> PathConfig {
        PathConfig {
            path: self.path,
            deployments: self.deployments,
            middlewares: self.middlewares,
            strip_prefix: self.strip_prefix,
            pass_through: self.pass_through,
        }
    }
}

impl Validate for PathConfig {
    fn validate(&self) -> TraefikResult<()> {
        // validate path is not empty
        if self.path.is_empty() {
            return Err(TraefikError::HostConfig("path is empty".to_string()));
        }

        // validate deployments if they exist
        for deployment in self.deployments.values() {
            deployment.validate()?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::config::deployment::DeploymentConfigBuilder;

    use super::*;

    #[test]
    fn test_validate_path() {
        let path = PathConfig {
            path: "/test".to_string(),
            deployments: HashMap::new(),
            middlewares: vec![],
            strip_prefix: false,
            pass_through: false,
        };
        assert!(path.validate().is_ok());
    }

    #[test]
    fn test_validate_host_with_path() {
        let host = HostConfigBuilder::default()
            .domain("test.com".to_string())
            .path(
                "/test".to_string(),
                PathConfigBuilder::default()
                    .path("/test".to_string())
                    .build(),
            )
            .deployment("test".to_string(), DeploymentConfig::default())
            .build()
            .unwrap();

        let validate_result = host.validate();
        assert!(validate_result.is_ok());
    }

    #[test]
    fn test_validate_host_with_invalid_path() {
        let host = HostConfig::default();
        assert!(host.validate().is_err());
    }

    #[test]
    fn test_validate_host_fails_with_empty_domain() {
        let host = HostConfigBuilder::default().domain("".to_string()).build();
        assert!(host.is_ok());
        let host = host.unwrap();
        let validate_result = host.validate();
        assert!(validate_result.is_err());
    }

    #[test]
    fn test_validate_host_fails_with_invalid_deployment_port() {
        let host = HostConfigBuilder::default()
            .domain("test.com".to_string())
            .deployment(
                "test".to_string(),
                DeploymentConfigBuilder::default().port(0).build(),
            )
            .build();
        assert!(host.is_ok());
        let host = host.unwrap();
        let validate_result = host.validate();
        assert!(validate_result.is_err());
    }

    #[test]
    fn test_validate_host_fails_with_invalid_deployment_weight() {
        let host = HostConfigBuilder::default()
            .domain("test.com".to_string())
            .deployment(
                "test".to_string(),
                DeploymentConfigBuilder::default().weight(101).build(),
            )
            .build();
        assert!(host.is_ok());
        let host = host.unwrap();
        let validate_result = host.validate();
        assert!(validate_result.is_err());
    }

    #[test]
    fn test_validate_host_fails_with_invalid_middleware_reference() {
        let host = HostConfigBuilder::default()
            .middleware("".to_string())
            .build();
        assert!(host.is_ok());
        let host = host.unwrap();
        let validate_result = host.validate();
        assert!(validate_result.is_err());
    }

    #[test]
    fn test_validate_host_fails_with_no_deployments() {
        let host = HostConfigBuilder::default()
            .domain("test.com".to_string())
            .build();
        assert!(host.is_ok());
        let host = host.unwrap();
        let validate_result = host.validate();
        assert!(validate_result.is_err());
    }

    #[test]
    fn test_validate_host_fails_with_invalid_path() {
        let host = HostConfigBuilder::default()
            .domain("test.com".to_string())
            .path("/test//test".to_string(), PathConfig::default())
            .build();
        assert!(host.is_ok());
        let host = host.unwrap();
        let validate_result = host.validate();
        assert!(validate_result.is_err());
    }

    #[test]
    fn test_validate_fails_with_path_that_does_not_start_with_slash() {
        let host = HostConfigBuilder::default()
            .domain("test.com".to_string())
            .path("test".to_string(), PathConfig::default())
            .build();
        assert!(host.is_ok());
        let host = host.unwrap();
        let validate_result = host.validate();
        assert!(validate_result.is_err());
    }
}