//! Closed profile/registry acceptance. Agent-authored T0.
use crate::request::TOOL;
use ed25519_dalek::VerifyingKey;
use sha2::{Digest, Sha256};

#[derive(Debug, PartialEq, Eq)]
pub struct ProfileError;
pub struct Binding {
    pub profile_hash: [u8; 32],
    pub key_id: [u8; 16],
    pub public_key: [u8; 32],
    pub envelope_hash: [u8; 32],
}
const FIXTURES: [[u8; 32]; 3] = [
    [
        0xa0, 0x9a, 0xa5, 0xf4, 0x7a, 0x67, 0x59, 0x80, 0x2f, 0xf9, 0x55, 0xf8, 0xdc, 0x2d, 0x2a,
        0x14, 0xa5, 0xc9, 0x9d, 0x23, 0xbe, 0x97, 0xf8, 0x64, 0x12, 0x7f, 0xf9, 0x38, 0x34, 0x55,
        0xa4, 0xf0,
    ],
    [
        0xd0, 0x4a, 0xb2, 0x32, 0x74, 0x2b, 0xb4, 0xab, 0x3a, 0x13, 0x68, 0xbd, 0x46, 0x15, 0xe4,
        0xe6, 0xd0, 0x22, 0x4a, 0xb7, 0x1a, 0x01, 0x6b, 0xaf, 0x85, 0x20, 0xa3, 0x32, 0xc9, 0x77,
        0x87, 0x37,
    ],
    [
        0xd7, 0x5a, 0x98, 0x01, 0x82, 0xb1, 0x0a, 0xb7, 0xd5, 0x4b, 0xfe, 0xd3, 0xc9, 0x64, 0x07,
        0x3a, 0x0e, 0xe1, 0x72, 0xf3, 0xda, 0xa6, 0x23, 0x25, 0xaf, 0x02, 0x1a, 0x68, 0xf7, 0x07,
        0x51, 0x1a,
    ],
];
pub fn fixture_inventory_hash() -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(b"DGRPUB1\0");
    h.update(3_u32.to_be_bytes());
    for key in FIXTURES {
        h.update(key);
    }
    h.finalize().into()
}
pub fn accept_public_key(bytes: &[u8; 32]) -> Result<VerifyingKey, ProfileError> {
    let mut y = *bytes;
    y[31] &= 0x7f;
    let mut modulus = [0xff; 32];
    modulus[0] = 0xed;
    modulus[31] = 0x7f;
    if y.iter().rev().cmp(modulus.iter().rev()) != std::cmp::Ordering::Less
        || FIXTURES.contains(bytes)
    {
        return Err(ProfileError);
    }
    let key = VerifyingKey::from_bytes(bytes).map_err(|_| ProfileError)?;
    if key.is_weak() {
        return Err(ProfileError);
    }
    Ok(key)
}
pub fn verify_unlocked_public_key(
    binding: &Binding,
    key: &VerifyingKey,
) -> Result<(), ProfileError> {
    if key.as_bytes() != &binding.public_key {
        return Err(ProfileError);
    }
    Ok(())
}
struct Cursor<'a>(&'a [u8]);
impl<'a> Cursor<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8], ProfileError> {
        let (v, rest) = self.0.split_at_checked(n).ok_or(ProfileError)?;
        self.0 = rest;
        Ok(v)
    }
    fn array<const N: usize>(&mut self) -> Result<[u8; N], ProfileError> {
        self.take(N)?.try_into().map_err(|_| ProfileError)
    }
    fn u8(&mut self) -> Result<u8, ProfileError> {
        Ok(self.array::<1>()?[0])
    }
    fn u16(&mut self) -> Result<u16, ProfileError> {
        Ok(u16::from_be_bytes(self.array()?))
    }
    fn u32(&mut self) -> Result<u32, ProfileError> {
        Ok(u32::from_be_bytes(self.array()?))
    }
    fn u64(&mut self) -> Result<u64, ProfileError> {
        Ok(u64::from_be_bytes(self.array()?))
    }
    fn end(self) -> Result<(), ProfileError> {
        if self.0.is_empty() {
            Ok(())
        } else {
            Err(ProfileError)
        }
    }
    fn path(&mut self, expected: &str) -> Result<(), ProfileError> {
        let n = usize::from(self.u16()?);
        let path = self.take(n)?;
        if path == expected.as_bytes() && valid_path(path) {
            Ok(())
        } else {
            Err(ProfileError)
        }
    }
}
pub fn valid_path(p: &[u8]) -> bool {
    (2..=512).contains(&p.len())
        && p[0] == b'/'
        && p[1..].split(|b| *b == b'/').all(|c| {
            !c.is_empty()
                && c.len() <= 255
                && c != b"."
                && c != b".."
                && c.iter()
                    .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(b))
        })
}
fn parse_profile(input: &[u8]) -> Result<([u8; 16], [u8; 32]), ProfileError> {
    if !(16..=16384).contains(&input.len()) {
        return Err(ProfileError);
    }
    let mut c = Cursor(input);
    if c.take(8)? != b"DGRPROF1"
        || c.u16()? != 1
        || c.u16()? != 18
        || c.u32()? as usize != input.len()
    {
        return Err(ProfileError);
    }
    let mut binding = None;
    for tag in 1..=18 {
        if c.u16()? != tag {
            return Err(ProfileError);
        }
        let n = usize::try_from(c.u32()?).map_err(|_| ProfileError)?;
        let payload = c.take(n)?;
        let mut p = Cursor(payload);
        let constant: Option<&[u8]> = match tag {
            1 => Some(b"dgr-runtime002/1"),
            2 => Some(b"linux-x86_64-gnu"),
            3 => Some(b"24.21.0"),
            4 => Some(&[0, 0, 0, 8]),
            5 => Some(&[
                0x9f, 0x1c, 0x8a, 0x1c, 0x58, 0xbd, 0x1e, 0x88, 0x9d, 0xf4, 0xf7, 0xe7, 0x87, 0x42,
                0xad, 0xe5, 0x6d, 0x7e, 0xfe, 0xfe,
            ]),
            8 => Some(&[0, 0, 0xff, 0xfc, 0, 0, 0xff, 0xfc]),
            13 => Some(&[
                0, 0, 3, 0xe8, 0, 0, 0x13, 0x88, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1,
            ]),
            14 => Some(TOOL.as_bytes()),
            15 => Some(b"DGR-ACT2\0"),
            16 => Some(&[0, 0, 0, 1]),
            17 => Some(b"sync-port/1"),
            18 => Some(&[0, 0, 0, 31]),
            _ => None,
        };
        if let Some(expected) = constant {
            if payload != expected {
                return Err(ProfileError);
            }
            continue;
        }
        match tag {
            6 => {
                if p.take(32)?
                    != [
                        0x69, 0xce, 0xcf, 0x4b, 0xbf, 0x72, 0xd2, 0xd4, 0x4a, 0x9e, 0xef, 0x1b,
                        0x71, 0xfb, 0x98, 0xc7, 0xfb, 0x97, 0x3d, 0x78, 0xaf, 0x11, 0x39, 0x9d,
                        0xec, 0xce, 0xf1, 0x9b, 0xeb, 0x00, 0x8a, 0xd9,
                    ]
                {
                    return Err(ProfileError);
                }
                p.take(32)?;
            }
            7 => {
                if p.take(32)? == [0; 32] {
                    return Err(ProfileError);
                }
            }
            9 => {
                if p.u8()? != 2 {
                    return Err(ProfileError);
                }
                let mut ids = Vec::new();
                for (role, path) in [
                    (1, "/var/lib/dgr/state"),
                    (2, "/var/lib/dgr/invoice-output"),
                ] {
                    if p.u8()? != role {
                        return Err(ProfileError);
                    }
                    let id = (p.u64()?, p.u64()?);
                    if id.1 == 0 || ids.contains(&id) || p.u32()? != 0o700 {
                        return Err(ProfileError);
                    }
                    ids.push(id);
                    p.path(path)?;
                }
            }
            10 => {
                if p.u8()? != 8 {
                    return Err(ProfileError);
                }
                let mut ids = Vec::new();
                for role in 1..=8 {
                    if p.u8()? != role {
                        return Err(ProfileError);
                    }
                    let id = (p.u64()?, p.u64()?);
                    if id.1 == 0 || ids.contains(&id) || p.u32()? != 0o600 {
                        return Err(ProfileError);
                    }
                    ids.push(id);
                }
            }
            11 => {
                if p.u8()? != 8 {
                    return Err(ProfileError);
                }
                let paths = [
                    "/opt/dgr/lib/dgr_core_node.node",
                    "/opt/dgr/lib/bootstrap.mjs",
                    "/opt/dgr/lib/invoice-port.mjs",
                    "/etc/dgr/openclaw.json",
                    "/etc/dgr/protected-tools.json",
                    "/etc/dgr/dependency-inventory.json",
                    "/usr/local/bin/node",
                    "/opt/dgr/openclaw/openclaw.mjs",
                ];
                for (role, path) in (1..=8).zip(paths) {
                    if p.u8()? != role {
                        return Err(ProfileError);
                    }
                    p.take(32)?;
                    p.path(path)?;
                }
            }
            12 => {
                if p.u8()? != 1 {
                    return Err(ProfileError);
                }
                let id = p.array()?;
                let public = p.array()?;
                accept_public_key(&public)?;
                binding = Some((id, public));
            }
            _ => return Err(ProfileError),
        }
        p.end()?;
    }
    c.end()?;
    binding.ok_or(ProfileError)
}
pub fn select_profile_registration(
    profile: &[u8],
    registry: &[u8],
    expected_profile: &[u8; 32],
) -> Result<Binding, ProfileError> {
    let profile_hash: [u8; 32] = Sha256::digest(profile).into();
    if &profile_hash != expected_profile {
        return Err(ProfileError);
    }
    let (key_id, public_key) = parse_profile(profile)?;
    if !(128..=28688).contains(&registry.len()) {
        return Err(ProfileError);
    }
    let mut c = Cursor(registry);
    if c.take(8)? != b"DGRREG1\0" || c.u16()? != 1 {
        return Err(ProfileError);
    }
    let count = usize::from(c.u16()?);
    if !(1..=256).contains(&count)
        || c.u32()? as usize != registry.len()
        || registry.len() != 16 + 112 * count
    {
        return Err(ProfileError);
    }
    let mut previous = None;
    let mut ids = Vec::new();
    let mut keys = Vec::new();
    let mut selected = None;
    for _ in 0..count {
        let digest: [u8; 32] = c.array()?;
        let id: [u8; 16] = c.array()?;
        let public: [u8; 32] = c.array()?;
        let envelope_hash = c.array()?;
        if previous.is_some_and(|last| last >= digest)
            || ids.contains(&id)
            || keys.contains(&public)
        {
            return Err(ProfileError);
        }
        accept_public_key(&public)?;
        previous = Some(digest);
        ids.push(id);
        keys.push(public);
        if digest == profile_hash {
            if id != key_id || public != public_key {
                return Err(ProfileError);
            }
            selected = Some(Binding {
                profile_hash,
                key_id,
                public_key,
                envelope_hash,
            });
        }
    }
    c.end()?;
    selected.ok_or(ProfileError)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fixture_inventory_and_key_rejection() {
        assert_eq!(
            crate::hex(&fixture_inventory_hash()),
            "9ad0e809fa2d8ab77fb24c1cbb11748cbfeba03d183ee96ad35e50cf6231afd7"
        );
        for fixture in FIXTURES {
            assert!(accept_public_key(&fixture).is_err());
        }
        for key in [[0; 32], [0xff; 32]] {
            assert!(accept_public_key(&key).is_err());
        }
        let mut identity = [0; 32];
        identity[0] = 1;
        assert!(accept_public_key(&identity).is_err());
        identity[31] = 0x80;
        assert!(accept_public_key(&identity).is_err());
    }
    #[test]
    fn closed_paths_and_profile_headers() {
        for path in [
            "/", "relative", "/a/", "/a//b", "/a/../b", "/a/./b", "/a b", "/a\0",
        ] {
            assert!(!valid_path(path.as_bytes()));
        }
        assert!(valid_path(b"/var/lib/dgr/state"));
        for length in 0..=16 {
            assert!(parse_profile(&vec![0; length]).is_err());
        }
        assert!(parse_profile(&vec![0; 16385]).is_err());
    }
}
