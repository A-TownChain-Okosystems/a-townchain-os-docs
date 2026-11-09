//! Userspace network manager state machine.
//!
//! Hardware drivers and protocol engines remain outside this crate. This layer
//! owns deterministic interface state, DHCP/DNS readiness, and fail-closed
//! transitions.

use crate::{IpAddress, NetworkPolicy};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceState {
    Down,
    Configuring,
    Up,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigurationSource {
    Static,
    Dhcp,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceConfig {
    pub id: u64,
    pub name: String,
    pub source: ConfigurationSource,
    pub address: Option<IpAddress>,
    pub dns_ready: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkError {
    InvalidInterface,
    AlreadyExists,
    UnknownInterface,
    InvalidTransition,
    PolicyDisabled,
}

#[derive(Debug, Default)]
pub struct NetworkManager {
    interfaces: Vec<(InterfaceConfig, InterfaceState)>,
    next_id: u64,
    policy: NetworkPolicy,
}

impl NetworkManager {
    /// Creates a manager with networking disabled until explicitly enabled.
    pub fn new() -> Self {
        Self {
            interfaces: Vec::new(),
            next_id: 1,
            policy: NetworkPolicy::Disabled,
        }
    }

    /// Changes the global userspace network policy.
    pub fn set_policy(&mut self, policy: NetworkPolicy) {
        self.policy = policy;
        if policy == NetworkPolicy::Disabled {
            for (_, state) in &mut self.interfaces {
                *state = InterfaceState::Down;
            }
        }
    }

    /// Registers an interface without implicitly bringing it online.
    pub fn register_interface(
        &mut self,
        name: impl Into<String>,
        source: ConfigurationSource,
    ) -> Result<u64, NetworkError> {
        let name = name.into();
        if name.is_empty() {
            return Err(NetworkError::InvalidInterface);
        }
        if self.interfaces.iter().any(|(cfg, _)| cfg.name == name) {
            return Err(NetworkError::AlreadyExists);
        }
        let id = self.next_id;
        self.next_id = self
            .next_id
            .checked_add(1)
            .ok_or(NetworkError::InvalidInterface)?;
        self.interfaces.push((
            InterfaceConfig {
                id,
                name,
                source,
                address: None,
                dns_ready: false,
            },
            InterfaceState::Down,
        ));
        Ok(id)
    }

    /// Starts interface configuration. Disabled networking fails closed.
    pub fn configure(&mut self, id: u64) -> Result<(), NetworkError> {
        if self.policy == NetworkPolicy::Disabled {
            return Err(NetworkError::PolicyDisabled);
        }
        let (_, state) = self.find_mut(id)?;
        if *state != InterfaceState::Down {
            return Err(NetworkError::InvalidTransition);
        }
        *state = InterfaceState::Configuring;
        Ok(())
    }

    /// Records a successfully assigned address.
    pub fn set_address(&mut self, id: u64, address: IpAddress) -> Result<(), NetworkError> {
        let (config, state) = self.find_mut(id)?;
        if *state != InterfaceState::Configuring {
            return Err(NetworkError::InvalidTransition);
        }
        config.address = Some(address);
        *state = InterfaceState::Up;
        Ok(())
    }

    /// Records successful DNS configuration after an interface is online.
    pub fn set_dns_ready(&mut self, id: u64, ready: bool) -> Result<(), NetworkError> {
        let (config, state) = self.find_mut(id)?;
        if *state != InterfaceState::Up {
            return Err(NetworkError::InvalidTransition);
        }
        config.dns_ready = ready;
        Ok(())
    }

    /// Marks an interface failed and removes its usable address.
    pub fn fail(&mut self, id: u64) -> Result<(), NetworkError> {
        let (config, state) = self.find_mut(id)?;
        config.address = None;
        config.dns_ready = false;
        *state = InterfaceState::Failed;
        Ok(())
    }

    /// Returns an immutable interface snapshot.
    pub fn get(&self, id: u64) -> Option<(&InterfaceConfig, InterfaceState)> {
        self.interfaces
            .iter()
            .find(|(config, _)| config.id == id)
            .map(|(config, state)| (config, *state))
    }

    fn find_mut(
        &mut self,
        id: u64,
    ) -> Result<(&mut InterfaceConfig, &mut InterfaceState), NetworkError> {
        self.interfaces
            .iter_mut()
            .find(|(config, _)| config.id == id)
            .map(|(config, state)| (config, state))
            .ok_or(NetworkError::UnknownInterface)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Ipv4Address;

    #[test]
    fn networking_is_disabled_by_default() {
        let mut manager = NetworkManager::new();
        let id = manager
            .register_interface("eth0", ConfigurationSource::Dhcp)
            .unwrap();
        assert_eq!(manager.configure(id), Err(NetworkError::PolicyDisabled));
    }

    #[test]
    fn interface_requires_configuration_before_address() {
        let mut manager = NetworkManager::new();
        manager.set_policy(NetworkPolicy::Normal);
        let id = manager
            .register_interface("eth0", ConfigurationSource::Dhcp)
            .unwrap();
        assert_eq!(
            manager.set_address(id, IpAddress::V4(Ipv4Address::new([192, 0, 2, 10]))),
            Err(NetworkError::InvalidTransition)
        );
        manager.configure(id).unwrap();
        manager
            .set_address(id, IpAddress::V4(Ipv4Address::new([192, 0, 2, 10])))
            .unwrap();
        assert_eq!(manager.get(id).unwrap().1, InterfaceState::Up);
    }

    #[test]
    fn disabling_network_brings_interfaces_down() {
        let mut manager = NetworkManager::new();
        manager.set_policy(NetworkPolicy::Normal);
        let id = manager
            .register_interface("eth0", ConfigurationSource::Static)
            .unwrap();
        manager.configure(id).unwrap();
        manager
            .set_address(id, IpAddress::V4(Ipv4Address::new([192, 0, 2, 20])))
            .unwrap();
        manager.set_policy(NetworkPolicy::Disabled);
        assert_eq!(manager.get(id).unwrap().1, InterfaceState::Down);
    }
}
