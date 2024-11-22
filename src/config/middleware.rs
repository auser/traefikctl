use serde::{Deserialize, Serialize};

use crate::{
    core::{
        etcd_trait::{EtcdPair, ToEtcdPairs},
        util::validate_is_alphanumeric,
        Validate,
    },
    error::{TraefikError, TraefikResult},
};

use super::headers::HeadersConfig;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MiddlewareConfig {
    /// The name of the middleware
    #[serde(default)]
    pub name: String,
    /// The headers configuration for the middleware
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HeadersConfig>,
    /// The type of middleware
    #[serde(default = "default_protocol")]
    pub protocol: String,
}

impl Default for MiddlewareConfig {
    fn default() -> Self {
        MiddlewareConfig {
            name: "test-middleware".to_string(),
            headers: None,
            protocol: default_protocol(),
        }
    }
}

fn default_protocol() -> String {
    "http".to_string()
}

impl MiddlewareConfig {
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn set_protocol(&mut self, protocol: &str) {
        self.protocol = protocol.to_string();
    }
}

impl ToEtcdPairs for MiddlewareConfig {
    /// Convert the middleware configuration to etcd pairs
    ///
    /// The middleware configuration is stored in etcd under the following path:
    /// `{base_key}/{protocol}/middlewares/{name}`
    fn to_etcd_pairs(&self, base_key: &str) -> TraefikResult<Vec<EtcdPair>> {
        // First set the middleware name to true
        let mut pairs = vec![EtcdPair::new(
            format!("{}/{}", base_key, self.name),
            "true".to_string(),
        )];
        let headers_base_key = format!("{}/{}", base_key, self.name);
        // Next add the headers if they are present
        if let Some(headers) = &self.headers {
            let headers_pairs = headers.to_etcd_pairs(&headers_base_key)?;
            pairs.extend(headers_pairs);
        }
        Ok(pairs)
    }
}

impl Validate for MiddlewareConfig {
    /// Validate the middleware configuration
    fn validate(&self) -> TraefikResult<()> {
        if self.name.is_empty() {
            return Err(TraefikError::MiddlewareConfig(
                "middleware name is empty".into(),
            ));
        }

        validate_is_alphanumeric(&self.name)?;

        if let Some(headers) = &self.headers {
            headers.validate()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::create_test_middleware;

    #[test]
    fn test_headers_config_validate() {
        let middleware = create_test_middleware();
        middleware
            .get("enable-headers")
            .unwrap()
            .validate()
            .unwrap();
        assert!(!middleware.contains_key("invalid-middleware"));
    }

    #[test]
    fn test_middleware_is_invalid_if_name_is_empty() {
        let middleware = MiddlewareConfig {
            name: "".to_string(),
            ..Default::default()
        };
        assert!(middleware.validate().is_err());
    }

    #[test]
    fn test_middleware_is_invalid_if_name_is_not_alphanumeric_or_hyphens() {
        let middleware = MiddlewareConfig {
            name: "invalid-%middleware".to_string(),
            ..Default::default()
        };
        assert!(middleware.validate().is_err());
    }

    #[test]
    fn test_middleware_is_valid_if_name_is_alphanumeric_or_hyphens() {
        let middleware = MiddlewareConfig {
            name: "valid-middleware".to_string(),
            ..Default::default()
        };
        assert!(middleware.validate().is_ok());
    }

    #[test]
    fn test_to_etcd_pairs() {
        let middleware = create_test_middleware();
        let mut result_pairs = vec![];
        for (_name, middleware) in middleware {
            let pairs = middleware.to_etcd_pairs("test/middlewares").unwrap();
            result_pairs.extend(pairs);
        }
        let pair_strs: Vec<String> = result_pairs.iter().map(|p| p.to_string()).collect();
        assert!(pair_strs.contains(&"test/middlewares/enable-headers true".to_string()));
        assert!(pair_strs.contains(&"test/middlewares/handle-redirects true".to_string()));
        assert!(pair_strs.contains(
            &"test/middlewares/enable-headers/headers/customRequestHeaders/X-Forwarded-Proto https"
                .to_string()
        ));
        assert!(pair_strs.contains(
            &"test/middlewares/enable-headers/headers/customRequestHeaders/X-Forwarded-Port 443"
                .to_string()
        ));

        assert!(pair_strs.contains(
            &"test/middlewares/enable-headers/headers/customResponseHeaders/Location \"\""
                .to_string()
        ));
        assert!(pair_strs.contains(
            &"test/middlewares/enable-headers/headers/accessControlAllowMethods GET, POST, OPTIONS"
                .to_string()
        ));
        assert!(pair_strs.contains(
            &"test/middlewares/enable-headers/headers/accessControlAllowHeaders Content-Type, Authorization"
                .to_string()
        ));
    }
}