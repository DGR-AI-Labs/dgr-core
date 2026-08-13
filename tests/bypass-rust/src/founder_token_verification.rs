//! {FOUNDER-AUTHORS}: T0 capability-token verification unit.
//!
//! Founder-authored capability-token signature verification and pinned K2 trust set.

#[doc(hidden)]
pub const CONFORMANCE_K2_KEY_ID: [u8; 16] = REGISTERED_K2_KEY_ID;

#[doc(hidden)]
pub const CONFORMANCE_K2_PUBLIC_KEY: [u8; 32] = REGISTERED_K2_PUBLIC_KEY;

use ed25519_dalek::{Signature, VerifyingKey};

use crate::before_tool_call::GuardFault;

const FORMAT_VERSION: u8 = 1;
const TOKEN_WIRE_LENGTH: usize = 145;
const SIGNATURE_PREIMAGE_LENGTH: usize = 90;
const DOMAIN_TAG: &[u8; 9] = b"DGR-CAP1\x00";

struct PinnedK2Key {
    key_id: [u8; 16],
    public_key: [u8; 32],
}

const REGISTERED_K2_KEY_ID: [u8; 16] = *b"DGR-TEST-KEY-001";

const REGISTERED_K2_PUBLIC_KEY: [u8; 32] = [
    0xd0, 0x4a, 0xb2, 0x32, 0x74, 0x2b, 0xb4, 0xab, 0x3a, 0x13, 0x68, 0xbd, 0x46, 0x15, 0xe4, 0xe6,
    0xd0, 0x22, 0x4a, 0xb7, 0x1a, 0x01, 0x6b, 0xaf, 0x85, 0x20, 0xa3, 0x32, 0xc9, 0x77, 0x87, 0x37,
];

const PINNED_K2_KEYS: &[PinnedK2Key] = &[PinnedK2Key {
    key_id: REGISTERED_K2_KEY_ID,
    public_key: REGISTERED_K2_PUBLIC_KEY,
}];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VerifyOutcome {
    Verified(VerifiedToken),
    Rejected(TokenRejection),
    Faulted(GuardFault),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenRejection {
    MalformedLength,
    UnsupportedVersion,
    UnknownKeyId,
    InvalidSignature,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VerifiedToken {
    key_id: [u8; 16],
    issued_at: u64,
    expires_at: u64,
    nonce: [u8; 16],
    action_commitment: [u8; 32],
}

struct ParsedToken {
    format_version: u8,
    key_id: [u8; 16],
    issued_at: u64,
    expires_at: u64,
    nonce: [u8; 16],
    action_commitment: [u8; 32],
    signature: [u8; 64],
}

impl ParsedToken {
    fn parse(bytes: &[u8]) -> Result<Self, TokenRejection> {
        if bytes.len() != TOKEN_WIRE_LENGTH {
            return Err(TokenRejection::MalformedLength);
        }

        let format_version = bytes[0];
        if format_version != FORMAT_VERSION {
            return Err(TokenRejection::UnsupportedVersion);
        }

        Ok(Self {
            format_version,
            key_id: fixed_array(&bytes[1..17])?,
            issued_at: u64::from_be_bytes(fixed_array(&bytes[17..25])?),
            expires_at: u64::from_be_bytes(fixed_array(&bytes[25..33])?),
            nonce: fixed_array(&bytes[33..49])?,
            action_commitment: fixed_array(&bytes[49..81])?,
            signature: fixed_array(&bytes[81..145])?,
        })
    }

    fn signature_preimage(&self) -> [u8; SIGNATURE_PREIMAGE_LENGTH] {
        let mut preimage = [0_u8; SIGNATURE_PREIMAGE_LENGTH];

        preimage[0..9].copy_from_slice(DOMAIN_TAG);
        preimage[9] = self.format_version;
        preimage[10..26].copy_from_slice(&self.key_id);
        preimage[26..34].copy_from_slice(&self.issued_at.to_be_bytes());
        preimage[34..42].copy_from_slice(&self.expires_at.to_be_bytes());
        preimage[42..58].copy_from_slice(&self.nonce);
        preimage[58..90].copy_from_slice(&self.action_commitment);

        preimage
    }
}

fn fixed_array<const N: usize>(bytes: &[u8]) -> Result<[u8; N], TokenRejection> {
    bytes
        .try_into()
        .map_err(|_| TokenRejection::MalformedLength)
}

fn pinned_key(key_id: [u8; 16]) -> Option<&'static PinnedK2Key> {
    PINNED_K2_KEYS.iter().find(|entry| entry.key_id == key_id)
}

pub fn verify_capability_token(bytes: &[u8]) -> VerifyOutcome {
    let parsed = match ParsedToken::parse(bytes) {
        Ok(parsed) => parsed,
        Err(rejection) => return VerifyOutcome::Rejected(rejection),
    };

    let pinned = match pinned_key(parsed.key_id) {
        Some(pinned) => pinned,
        None => return VerifyOutcome::Rejected(TokenRejection::UnknownKeyId),
    };

    let verifying_key = match VerifyingKey::from_bytes(&pinned.public_key) {
        Ok(key) => key,
        Err(_) => return VerifyOutcome::Faulted(GuardFault::InternalError),
    };

    let signature = Signature::from_bytes(&parsed.signature);
    let preimage = parsed.signature_preimage();

    if verifying_key.verify_strict(&preimage, &signature).is_err() {
        return VerifyOutcome::Rejected(TokenRejection::InvalidSignature);
    }

    VerifyOutcome::Verified(VerifiedToken {
        key_id: parsed.key_id,
        issued_at: parsed.issued_at,
        expires_at: parsed.expires_at,
        nonce: parsed.nonce,
        action_commitment: parsed.action_commitment,
    })
}
