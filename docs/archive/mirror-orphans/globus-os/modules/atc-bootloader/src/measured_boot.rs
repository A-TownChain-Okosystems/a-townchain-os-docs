//! Measured-boot evidence contracts. Actual TPM PCR extension is performed by trusted boot code.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum MeasurementKind{Firmware,Bootloader,Kernel,Policy}
#[derive(Debug,Clone,PartialEq,Eq)]pub struct Measurement{pub kind:MeasurementKind,pub digest:[u8;32],pub pcr:u8}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum RecoveryReason{SignatureFailure,MeasurementMismatch,BootLoop,CorruptState}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum BootDisposition{Continue,Recovery}
pub fn evaluate_measurements(measurements:&[Measurement],expected:&[[u8;32]])->BootDisposition{if measurements.len()!=expected.len(){return BootDisposition::Recovery}if measurements.iter().zip(expected).all(|(m,e)|&m.digest==e){BootDisposition::Continue}else{BootDisposition::Recovery}}
