//! Signed/provenance gate for A/B updates. Cryptographic verification is injected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateArtifact {
    pub version: String,
    pub payload_digest: String,
    pub signature: String,
    pub provenance_digest: String,
    pub sbom_digest: String,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationError {
    MissingMetadata,
    DigestMismatch,
    SignatureRejected,
    ProvenanceRejected,
}
pub trait UpdateVerifier {
    fn verify_signature(&self, a: &UpdateArtifact) -> bool;
    fn verify_provenance(&self, a: &UpdateArtifact) -> bool;
}
pub fn verify_artifact<V: UpdateVerifier>(
    a: &UpdateArtifact,
    expected: &str,
    v: &V,
) -> Result<(), VerificationError> {
    if a.version.is_empty()
        || a.signature.is_empty()
        || a.provenance_digest.is_empty()
        || a.sbom_digest.is_empty()
    {
        return Err(VerificationError::MissingMetadata);
    }
    if a.payload_digest != expected {
        return Err(VerificationError::DigestMismatch);
    }
    if !v.verify_signature(a) {
        return Err(VerificationError::SignatureRejected);
    }
    if !v.verify_provenance(a) {
        return Err(VerificationError::ProvenanceRejected);
    }
    Ok(())
}
