//! Hardware abstraction contracts. Concrete MMIO/port access remains behind trusted drivers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuArch {
    X86_64,
    Aarch64,
    Riscv64,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CpuInfo {
    pub arch: CpuArch,
    pub logical_cpus: u16,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InterruptLine {
    pub vector: u16,
    pub level_triggered: bool,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimerSpec {
    pub frequency_hz: u64,
    pub deadline_ticks: u64,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PciAddress {
    pub segment: u16,
    pub bus: u8,
    pub device: u8,
    pub function: u8,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PciDeviceInfo {
    pub address: PciAddress,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class_code: u32,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DmaRegion {
    pub address: u64,
    pub length: u64,
    pub writable: bool,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerAction {
    Suspend,
    Hibernate,
    Reboot,
    Shutdown,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HalError {
    InvalidCpuCount,
    InvalidTimer,
    InvalidDma,
    InvalidPciAddress,
}
pub fn validate_cpu(i: CpuInfo) -> Result<(), HalError> {
    if i.logical_cpus == 0 {
        Err(HalError::InvalidCpuCount)
    } else {
        Ok(())
    }
}
pub fn validate_timer(t: TimerSpec) -> Result<(), HalError> {
    if t.frequency_hz == 0 {
        Err(HalError::InvalidTimer)
    } else {
        Ok(())
    }
}
pub fn validate_dma(r: DmaRegion) -> Result<(), HalError> {
    if r.length == 0 || r.address.checked_add(r.length).is_none() {
        Err(HalError::InvalidDma)
    } else {
        Ok(())
    }
}
pub fn validate_pci(a: PciAddress) -> Result<(), HalError> {
    if a.device >= 32 || a.function >= 8 {
        Err(HalError::InvalidPciAddress)
    } else {
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_invalid() {
        assert_eq!(
            validate_cpu(CpuInfo {
                arch: CpuArch::X86_64,
                logical_cpus: 0
            }),
            Err(HalError::InvalidCpuCount)
        );
        assert_eq!(
            validate_timer(TimerSpec {
                frequency_hz: 0,
                deadline_ticks: 1
            }),
            Err(HalError::InvalidTimer)
        );
        assert_eq!(
            validate_dma(DmaRegion {
                address: u64::MAX,
                length: 2,
                writable: false
            }),
            Err(HalError::InvalidDma)
        );
        assert_eq!(
            validate_pci(PciAddress {
                segment: 0,
                bus: 0,
                device: 32,
                function: 0
            }),
            Err(HalError::InvalidPciAddress)
        );
    }
}
