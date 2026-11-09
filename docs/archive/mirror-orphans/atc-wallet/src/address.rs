// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
//! ATC address encoding.
//!
//! WAL-ADDR-001:
//! payload = RIPEMD160(SHA256(pubkey))
//! checksum = first 4 bytes of SHA256(SHA256(version || payload))
//! encoding = Base58(version || payload || checksum)
//!
//! The version byte is supplied by network configuration. Concrete values
//! are not hard-coded until ATC-NETWORK-ID-001 freezes them.

use ripemd::{Digest as RipemdDigest, Ripemd160};
use sha2::Sha256;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressError {
    InvalidLength,
    InvalidChecksum,
    InvalidVersion,
    InvalidEncoding,
}

pub fn payload_from_public_key(public_key: &[u8; 32]) -> [u8; 20] {
    let sha = Sha256::digest(public_key);
    Ripemd160::digest(sha).into()
}

pub fn encode(public_key: &[u8; 32], version: u8) -> String {
    let payload = payload_from_public_key(public_key);
    let mut body = [0u8; 21];
    body[0] = version;
    body[1..].copy_from_slice(&payload);

    let first = Sha256::digest(body);
    let second = Sha256::digest(first);

    let mut full = [0u8; 25];
    full[..21].copy_from_slice(&body);
    full[21..].copy_from_slice(&second[..4]);
    bs58::encode(full).into_string()
}

pub fn decode(address: &str, expected_version: u8) -> Result<[u8; 20], AddressError> {
    let raw = bs58::decode(address)
        .into_vec()
        .map_err(|_| AddressError::InvalidEncoding)?;
    if raw.len() != 25 {
        return Err(AddressError::InvalidLength);
    }
    if raw[0] != expected_version {
        return Err(AddressError::InvalidVersion);
    }

    let first = Sha256::digest(&raw[..21]);
    let second = Sha256::digest(first);
    if raw[21..25] != second[..4] {
        return Err(AddressError::InvalidChecksum);
    }

    raw[1..21]
        .try_into()
        .map_err(|_| AddressError::InvalidLength)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address_roundtrip() {
        let public_key = [7u8; 32];
        let address = encode(&public_key, 42);
        assert_eq!(
            decode(&address, 42).unwrap(),
            payload_from_public_key(&public_key)
        );
    }

    #[test]
    fn checksum_corruption_is_rejected() {
        let address = encode(&[7u8; 32], 42);
        let mut raw = bs58::decode(address).into_vec().unwrap();
        raw[24] ^= 1;
        let corrupted = bs58::encode(raw).into_string();
        assert_eq!(decode(&corrupted, 42), Err(AddressError::InvalidChecksum));
    }
}
