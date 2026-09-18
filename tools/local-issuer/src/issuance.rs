//! One-attempt clock/entropy/signature and publication state. Agent-authored T0.
use crate::{ConfirmedSnapshot, Failure, custody};
use ed25519_dalek::{Signer, SigningKey};
use std::sync::atomic::{AtomicBool, Ordering};
use zeroize::Zeroizing;

pub struct TrustedClock {
    wall: std::time::Duration,
    monotonic: std::time::Duration,
}
impl TrustedClock {
    pub fn new() -> Result<Self, Failure> {
        let (wall, monotonic) = custody::clock_pair()?;
        Ok(Self { wall, monotonic })
    }
    pub fn observe(&mut self) -> Result<(std::time::Duration, std::time::Duration), Failure> {
        let (wall, monotonic) = custody::clock_pair()?;
        self.accept(wall, monotonic)?;
        Ok((wall, monotonic))
    }
    fn accept(
        &mut self,
        wall: std::time::Duration,
        monotonic: std::time::Duration,
    ) -> Result<(), Failure> {
        if wall < self.wall || monotonic < self.monotonic {
            return Err(Failure::Clock);
        }
        self.wall = wall;
        self.monotonic = monotonic;
        Ok(())
    }
    pub fn deadline(&mut self, seconds: u64) -> Result<std::time::Duration, Failure> {
        self.observe()?
            .1
            .checked_add(std::time::Duration::from_secs(seconds))
            .ok_or(Failure::Clock)
    }
    pub fn remaining_ms(&mut self, deadline: std::time::Duration) -> Result<i32, Failure> {
        let now = self.observe()?.1;
        let Some(remaining) = deadline.checked_sub(now) else {
            return Ok(0);
        };
        if remaining.is_zero() {
            return Ok(0);
        }
        let ms = remaining.as_millis().checked_add(1).ok_or(Failure::Clock)?;
        i32::try_from(ms).map_err(|_| Failure::Clock)
    }
}
pub struct CapabilityBytes(Zeroizing<[u8; 145]>);
pub fn issue_once(
    snapshot: &ConfirmedSnapshot,
    key: &SigningKey,
    clock: &mut TrustedClock,
) -> Result<CapabilityBytes, Failure> {
    let issued = clock.observe()?.0.as_secs();
    let expiry = issued.checked_add(300).ok_or(Failure::Clock)?;
    let mut nonce = [0_u8; 16];
    getrandom::getrandom(&mut nonce).map_err(|_| Failure::Entropy)?;
    clock.observe()?;
    sign(snapshot, key, issued, expiry, nonce)
}
fn sign(
    snapshot: &ConfirmedSnapshot,
    key: &SigningKey,
    issued: u64,
    expiry: u64,
    nonce: [u8; 16],
) -> Result<CapabilityBytes, Failure> {
    crate::profile::verify_unlocked_public_key(&snapshot.0.binding, &key.verifying_key())
        .map_err(|_| Failure::Unlock)?;
    let mut wire = Zeroizing::new([0; 145]);
    wire[0] = 1;
    wire[1..17].copy_from_slice(&snapshot.0.binding.key_id);
    wire[17..25].copy_from_slice(&issued.to_be_bytes());
    wire[25..33].copy_from_slice(&expiry.to_be_bytes());
    wire[33..49].copy_from_slice(&nonce);
    wire[49..81].copy_from_slice(&snapshot.0.commitment);
    let mut preimage = Zeroizing::new([0; 90]);
    preimage[..9].copy_from_slice(b"DGR-CAP1\0");
    preimage[9..].copy_from_slice(&wire[..81]);
    let signature = key.sign(&preimage[..]);
    key.verifying_key()
        .verify_strict(&preimage[..], &signature)
        .map_err(|_| Failure::Internal)?;
    wire[81..].copy_from_slice(&signature.to_bytes());
    Ok(CapabilityBytes(wire))
}
pub fn publish_capability_file(
    inputs: &custody::CustodyInputs,
    capability: CapabilityBytes,
    possible: &AtomicBool,
) -> Result<(), Failure> {
    let name = format!("cap-{}.txt", crate::hex(&capability.0[33..49]));
    let mut carrier = Zeroizing::new([0_u8; 291]);
    const HEX: &[u8; 16] = b"0123456789abcdef";
    for (i, b) in capability.0.iter().enumerate() {
        carrier[2 * i] = HEX[usize::from(b >> 4)];
        carrier[2 * i + 1] = HEX[usize::from(b & 15)];
    }
    carrier[290] = b'\n';
    inputs.recheck()?;
    let temporary = custody::anonymous_output(inputs)?;
    custody::write_output(&temporary, &carrier[..])?;
    custody::sync_output(&temporary)?;
    inputs.recheck()?;
    // Mark before the only link syscall. Every unexpected/ambiguous result keeps this set.
    possible.store(true, Ordering::SeqCst);
    match custody::link_output(inputs, &temporary, &name) {
        Ok(()) => {}
        Err(custody::LinkFailure::Exists) => {
            possible.store(false, Ordering::SeqCst);
            return Err(Failure::Unpublished);
        }
        Err(custody::LinkFailure::Uncertain) => return Err(Failure::Uncertain),
    }
    custody::verify_published(inputs, &temporary, &name).map_err(|_| Failure::Uncertain)?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn clock_rejects_each_backward_observation() {
        let mut clock = TrustedClock {
            wall: std::time::Duration::from_secs(100),
            monotonic: std::time::Duration::from_secs(10),
        };
        assert_eq!(
            clock.accept(
                std::time::Duration::from_secs(99),
                std::time::Duration::from_secs(11)
            ),
            Err(Failure::Clock)
        );
        assert_eq!(
            clock.accept(
                std::time::Duration::from_secs(101),
                std::time::Duration::from_secs(9)
            ),
            Err(Failure::Clock)
        );
        assert!(
            clock
                .accept(
                    std::time::Duration::from_secs(100),
                    std::time::Duration::from_secs(10)
                )
                .is_ok()
        );
    }
}
