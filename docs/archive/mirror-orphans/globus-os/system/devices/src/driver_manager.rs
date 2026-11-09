//! Userspace driver lifecycle and explicit device binding.
use crate::{Device, DeviceClass, DeviceId};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverState {
    Registered,
    Probing,
    Bound,
    Failed,
    Stopped,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriverSpec {
    pub name: String,
    pub class: DeviceClass,
    pub version: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriverBinding {
    pub device: DeviceId,
    pub driver: String,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverError {
    Duplicate,
    UnknownDriver,
    UnknownDevice,
    AlreadyBound,
    InvalidTransition,
}
#[derive(Debug, Default)]
pub struct DriverManager {
    drivers: Vec<(DriverSpec, DriverState)>,
    devices: Vec<Device>,
    bindings: Vec<DriverBinding>,
}
impl DriverManager {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn register_driver(&mut self, s: DriverSpec) -> Result<(), DriverError> {
        if self.drivers.iter().any(|(d, _)| d.name == s.name) {
            return Err(DriverError::Duplicate);
        }
        self.drivers.push((s, DriverState::Registered));
        Ok(())
    }
    pub fn register_device(&mut self, d: Device) -> Result<(), DriverError> {
        if self.devices.iter().any(|x| x.id == d.id) {
            return Err(DriverError::Duplicate);
        }
        self.devices.push(d);
        Ok(())
    }
    pub fn bind(&mut self, id: DeviceId, name: &str) -> Result<(), DriverError> {
        let dev = self
            .devices
            .iter()
            .find(|d| d.id == id)
            .ok_or(DriverError::UnknownDevice)?;
        let Some((spec, state)) = self.drivers.iter_mut().find(|(d, _)| d.name == name) else {
            return Err(DriverError::UnknownDriver);
        };
        if self.bindings.iter().any(|b| b.device == id) {
            return Err(DriverError::AlreadyBound);
        }
        if spec.class != dev.class {
            return Err(DriverError::InvalidTransition);
        }
        *state = DriverState::Bound;
        self.bindings.push(DriverBinding {
            device: id,
            driver: name.into(),
        });
        Ok(())
    }
    pub fn unbind(&mut self, id: DeviceId) -> Result<(), DriverError> {
        let i = self
            .bindings
            .iter()
            .position(|b| b.device == id)
            .ok_or(DriverError::UnknownDevice)?;
        let name = self.bindings.remove(i).driver;
        if let Some((_, s)) = self.drivers.iter_mut().find(|(d, _)| d.name == name) {
            *s = DriverState::Stopped
        }
        Ok(())
    }
    pub fn bindings(&self) -> &[DriverBinding] {
        &self.bindings
    }
}
