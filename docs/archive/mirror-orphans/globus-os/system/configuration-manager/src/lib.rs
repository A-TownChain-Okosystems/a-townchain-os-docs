//! Deterministic, capability-gated system configuration registry.

use globus_settings::{Capability, Domain, Mutation, Policy, Value, authorize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Configuration {
    pub domain: Domain,
    pub key: String,
    pub value: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigurationError {
    Unauthorized,
    EmptyKey,
}

#[derive(Debug, Default)]
pub struct ConfigurationManager {
    values: BTreeMap<(String, String), Configuration>,
}

impl ConfigurationManager {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set(
        &mut self,
        capability: Capability,
        configuration: Configuration,
    ) -> Result<(), ConfigurationError> {
        let mutation = Mutation {
            domain: configuration.domain,
            key: configuration.key.clone(),
            value: configuration.value.clone(),
        };
        authorize(Policy, capability, &mutation).map_err(|e| match e {
            globus_settings::Error::EmptyKey => ConfigurationError::EmptyKey,
            _ => ConfigurationError::Unauthorized,
        })?;
        self.values.insert(
            (
                format!("{:?}", configuration.domain),
                configuration.key.clone(),
            ),
            configuration,
        );
        Ok(())
    }
    pub fn get(&self, domain: Domain, key: &str) -> Option<&Configuration> {
        self.values.get(&(format!("{domain:?}"), key.to_owned()))
    }
    pub fn values(&self) -> impl Iterator<Item = &Configuration> {
        self.values.values()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scoped_write_and_read() {
        let mut m = ConfigurationManager::new();
        let c = Configuration {
            domain: Domain::Privacy,
            key: "telemetry.enabled".into(),
            value: Value::Bool(false),
        };
        m.set(Capability::WritePrivacy, c).unwrap();
        assert!(m.get(Domain::Privacy, "telemetry.enabled").is_some());
    }
    #[test]
    fn cross_domain_denied() {
        let mut m = ConfigurationManager::new();
        let c = Configuration {
            domain: Domain::Security,
            key: "secure_boot.enabled".into(),
            value: Value::Bool(true),
        };
        assert_eq!(
            m.set(Capability::WriteSystem, c),
            Err(ConfigurationError::Unauthorized)
        );
    }
}
