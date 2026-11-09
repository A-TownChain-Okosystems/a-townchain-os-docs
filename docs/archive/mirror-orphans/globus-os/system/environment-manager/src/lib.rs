//! Deterministic environment-variable policy for user-space applications.

use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Environment {
    pub key: String,
    pub value: String,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvironmentError {
    EmptyKey,
    InvalidKey,
}
#[derive(Debug, Default)]
pub struct EnvironmentManager {
    values: BTreeMap<String, String>,
}

impl EnvironmentManager {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set(&mut self, key: &str, value: &str) -> Result<(), EnvironmentError> {
        if key.trim().is_empty() {
            return Err(EnvironmentError::EmptyKey);
        }
        if key
            .bytes()
            .any(|b| !(b.is_ascii_alphanumeric() || b == b'_'))
        {
            return Err(EnvironmentError::InvalidKey);
        }
        self.values.insert(key.to_owned(), value.to_owned());
        Ok(())
    }
    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.values.remove(key)
    }
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.values.iter()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deterministic_environment() {
        let mut m = EnvironmentManager::new();
        m.set("PATH", "/bin").unwrap();
        m.set("LANG", "de_DE").unwrap();
        assert_eq!(m.get("PATH"), Some("/bin"));
        assert!(m.set("bad-key", "x").is_err());
    }
}
