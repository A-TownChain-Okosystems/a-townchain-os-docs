//! .gpkg package manifest and verification gate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GpkgManifest {
    pub name: String,
    pub version: String,
    pub payload_digest: String,
    pub permissions: Vec<String>,
    pub signature: String,
    pub provenance: String,
    pub sbom_digest: String,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpkgError {
    MissingField,
    InvalidDigest,
}
impl GpkgManifest {
    pub fn validate(&self) -> Result<(), GpkgError> {
        if self.name.is_empty()
            || self.version.is_empty()
            || self.payload_digest.is_empty()
            || self.signature.is_empty()
            || self.provenance.is_empty()
            || self.sbom_digest.is_empty()
        {
            return Err(GpkgError::MissingField);
        }
        if self.payload_digest.len() != 64
            || !self.payload_digest.bytes().all(|b| b.is_ascii_hexdigit())
        {
            return Err(GpkgError::InvalidDigest);
        }
        Ok(())
    }
}
