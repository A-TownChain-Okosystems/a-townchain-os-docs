//! DNS, DHCP and TLS protocol-layer contracts above NetworkManager.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsQuestion {
    pub name: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsAnswer {
    pub name: String,
    pub address: String,
    pub ttl_seconds: u32,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DhcpLease {
    pub address: String,
    pub router: Option<String>,
    pub dns_servers: Vec<String>,
    pub expires_at: u64,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TlsPeer {
    pub hostname: String,
    pub certificate_fingerprint: String,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolError {
    InvalidName,
    InvalidAddress,
    EmptyDns,
    ExpiredLease,
    HostnameMismatch,
}
pub fn validate_dns_question(q: &DnsQuestion) -> Result<(), ProtocolError> {
    let n = q.name.trim();
    if n.is_empty() || n.len() > 253 || n.contains(' ') {
        Err(ProtocolError::InvalidName)
    } else {
        Ok(())
    }
}
pub fn validate_lease(l: &DhcpLease, now: u64) -> Result<(), ProtocolError> {
    if l.address.trim().is_empty() {
        return Err(ProtocolError::InvalidAddress);
    }
    if l.dns_servers.is_empty() {
        return Err(ProtocolError::EmptyDns);
    }
    if l.expires_at <= now {
        return Err(ProtocolError::ExpiredLease);
    }
    Ok(())
}
pub fn validate_tls_peer(p: &TlsPeer, expected: &str) -> Result<(), ProtocolError> {
    if p.hostname.eq_ignore_ascii_case(expected) && !p.certificate_fingerprint.trim().is_empty() {
        Ok(())
    } else {
        Err(ProtocolError::HostnameMismatch)
    }
}
