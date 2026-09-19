//! One-attempt clock/entropy/signature and publication state. Agent-authored T0.
use crate::{ConfirmedSnapshot, Failure, custody};
use ed25519_dalek::{Signer, SigningKey};
use std::sync::atomic::{AtomicBool, Ordering};
use zeroize::Zeroizing;

fn sample_clock() -> Result<(std::time::Duration, std::time::Duration), Failure> {
    #[cfg(test)]
    if let Some(event) = tests::event() {
        return match event {
            tests::Event::Clock(value) => value,
            _ => panic!("clock event mismatch"),
        };
    }
    custody::clock_pair()
}
fn fresh_nonce() -> Result<[u8; 16], Failure> {
    #[cfg(test)]
    if let Some(event) = tests::event() {
        return match event {
            tests::Event::Entropy(value) => value,
            _ => panic!("entropy event mismatch"),
        };
    }
    let mut nonce = [0_u8; 16];
    getrandom::getrandom(&mut nonce).map_err(|_| Failure::Entropy)?;
    Ok(nonce)
}
pub struct TrustedClock {
    wall: std::time::Duration,
    monotonic: std::time::Duration,
}
impl TrustedClock {
    pub fn new() -> Result<Self, Failure> {
        let (wall, monotonic) = sample_clock()?;
        Ok(Self { wall, monotonic })
    }
    pub fn observe(&mut self) -> Result<(std::time::Duration, std::time::Duration), Failure> {
        let (wall, monotonic) = sample_clock()?;
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
    let nonce = fresh_nonce()?;
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
    clock: &mut TrustedClock,
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
    clock.observe()?;
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
    clock.observe().map_err(|_| Failure::Uncertain)?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::RefCell, collections::VecDeque, time::Duration};
    pub(super) enum Event {
        Clock(Result<(Duration, Duration), Failure>),
        Entropy(Result<[u8; 16], Failure>),
    }
    thread_local! { static EVENTS: RefCell<Option<VecDeque<Event>>> = const {RefCell::new(None)}; }
    pub(super) fn event() -> Option<Event> {
        EVENTS.with(|e| {
            e.borrow_mut()
                .as_mut()
                .map(|q| q.pop_front().expect("unexpected clock/entropy event"))
        })
    }
    struct Events;
    impl Events {
        fn install(events: Vec<Event>) -> Self {
            EVENTS.with(|e| {
                let mut slot = e.borrow_mut();
                assert!(slot.is_none());
                *slot = Some(events.into());
            });
            Self
        }
    }
    impl Drop for Events {
        fn drop(&mut self) {
            let q = EVENTS.with(|e| e.borrow_mut().take()).unwrap();
            if !std::thread::panicking() {
                assert!(q.is_empty(), "unconsumed clock/entropy event");
            }
        }
    }
    fn public_snapshot() -> (ConfirmedSnapshot, SigningKey) {
        let invoice=crate::request::parse_request(b"{\"action\":\"record_invoice\",\"amount\":\"1\",\"currency\":\"USD\",\"destination\":\"simulation-vendor-1\",\"invoice_id\":\"SIM-A\",\"source_account\":\"simulation-account-1\",\"tool\":\"dgr.openclaw.invoice.record.v1\"}\n").unwrap();
        // Already disclosed 0x11 seed, excluded by actual production registration acceptance.
        let key = SigningKey::from_bytes(&[0x11; 32]);
        let binding = crate::profile::Binding {
            profile_hash: [1; 32],
            key_id: *b"DGR-TEST-KEY-001",
            public_key: *key.verifying_key().as_bytes(),
            envelope_hash: [2; 32],
        };
        let preview = crate::freeze_issuance_preview(invoice, binding);
        (ConfirmedSnapshot(preview), key)
    }
    fn public_capability() -> CapabilityBytes {
        let (s, k) = public_snapshot();
        sign(&s, &k, 1000, 1300, [0x33; 16]).unwrap()
    }
    #[test]
    fn public_wire_signature_and_key_binding() {
        let (s, k) = public_snapshot();
        let token = sign(&s, &k, 1000, 1300, [0x33; 16]).unwrap();
        assert_eq!(token.0.len(), 145);
        assert_eq!(token.0[0], 1);
        assert_eq!(&token.0[1..17], &s.0.binding.key_id);
        assert_eq!(&token.0[17..25], &1000_u64.to_be_bytes());
        assert_eq!(&token.0[25..33], &1300_u64.to_be_bytes());
        assert_eq!(&token.0[49..81], &s.0.commitment);
        let mut msg = b"DGR-CAP1\0".to_vec();
        msg.extend_from_slice(&token.0[..81]);
        assert_eq!(msg.len(), 90);
        let signature = ed25519_dalek::Signature::from_slice(&token.0[81..]).unwrap();
        k.verifying_key().verify_strict(&msg, &signature).unwrap();
        msg[89] ^= 1;
        assert!(k.verifying_key().verify_strict(&msg, &signature).is_err());
        let other = SigningKey::from_bytes(&[0x22; 32]);
        assert!(sign(&s, &other, 1000, 1300, [0x33; 16]).is_err());
        assert!(crate::profile::accept_public_key(k.verifying_key().as_bytes()).is_err());
    }
    #[test]
    fn actual_issuance_rejects_clock_entropy_and_expiry_failures() {
        let (s, k) = public_snapshot();
        for events in [
            vec![Event::Clock(Err(Failure::Clock))],
            vec![Event::Clock(Ok((Duration::MAX, Duration::from_secs(1))))],
            vec![
                Event::Clock(Ok((Duration::from_secs(10), Duration::from_secs(1)))),
                Event::Entropy(Err(Failure::Entropy)),
            ],
        ] {
            let _guard = Events::install(events);
            let mut clock = TrustedClock {
                wall: Duration::ZERO,
                monotonic: Duration::ZERO,
            };
            assert!(issue_once(&s, &k, &mut clock).is_err());
        }
        let _guard = Events::install(vec![
            Event::Clock(Ok((Duration::from_secs(100), Duration::from_secs(1)))),
            Event::Entropy(Ok([0x33; 16])),
            Event::Clock(Ok((Duration::from_secs(100), Duration::from_secs(2)))),
        ]);
        let mut clock = TrustedClock {
            wall: Duration::ZERO,
            monotonic: Duration::ZERO,
        };
        assert!(issue_once(&s, &k, &mut clock).is_ok());
    }
    #[test]
    fn real_publication_collision_and_symlink_never_overwrite() {
        let fixture = custody::tests::Fixture::new();
        let mut clock = TrustedClock::new().unwrap();
        let possible = AtomicBool::new(false);
        publish_capability_file(&fixture.inputs, public_capability(), &possible, &mut clock)
            .unwrap();
        let files: Vec<_> = std::fs::read_dir(fixture.output()).unwrap().collect();
        assert_eq!(files.len(), 1);
        let file = files[0].as_ref().unwrap().path();
        let original = std::fs::read(&file).unwrap();
        assert_eq!(original.len(), 291);
        assert_eq!(original[290], b'\n');
        assert!(
            original[..290]
                .iter()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(b))
        );
        possible.store(false, Ordering::SeqCst);
        assert_eq!(
            publish_capability_file(&fixture.inputs, public_capability(), &possible, &mut clock),
            Err(Failure::Unpublished)
        );
        assert!(!possible.load(Ordering::SeqCst));
        assert_eq!(std::fs::read(&file).unwrap(), original);
        std::fs::remove_file(&file).unwrap();
        let target = fixture.path.join("unchanged-public-marker");
        std::fs::write(&target, b"PUBLIC").unwrap();
        std::os::unix::fs::symlink(&target, &file).unwrap();
        assert_eq!(
            publish_capability_file(&fixture.inputs, public_capability(), &possible, &mut clock),
            Err(Failure::Unpublished)
        );
        assert_eq!(std::fs::read(target).unwrap(), b"PUBLIC");
    }
    #[test]
    fn publication_faults_preserve_uncertainty_and_never_retry() {
        use custody::{Inject, Operation};
        for (op, action, uncertain, visible) in [
            (Operation::Write, Inject::Error(libc::ENOSPC), false, false),
            (Operation::Write, Inject::Zero, false, false),
            (Operation::SyncFile, Inject::Error(libc::EIO), false, false),
            (Operation::Link, Inject::Error(libc::EINTR), true, false),
            (
                Operation::Link,
                Inject::AfterLinkError(libc::EIO),
                true,
                true,
            ),
            (
                Operation::SyncDirectory,
                Inject::Error(libc::EIO),
                true,
                true,
            ),
            (Operation::OpenInput, Inject::Error(libc::EIO), true, true),
        ] {
            let fixture = custody::tests::Fixture::new();
            let mut clock = TrustedClock::new().unwrap();
            let possible = AtomicBool::new(false);
            let _script = custody::tests::script(&[(op, action)]);
            assert!(
                publish_capability_file(
                    &fixture.inputs,
                    public_capability(),
                    &possible,
                    &mut clock
                )
                .is_err()
            );
            assert_eq!(possible.load(Ordering::SeqCst), uncertain);
            assert_eq!(
                std::fs::read_dir(fixture.output()).unwrap().count(),
                usize::from(visible)
            );
        }
    }
    #[test]
    fn short_writes_and_postlink_panic_use_real_publication_state() {
        use custody::{Inject, Operation};
        let fixture = custody::tests::Fixture::new();
        let mut clock = TrustedClock::new().unwrap();
        let possible = AtomicBool::new(false);
        {
            let _guard = custody::tests::script(&[
                (Operation::Write, Inject::Limit(1)),
                (Operation::Write, Inject::Limit(10)),
                (Operation::Write, Inject::Limit(280)),
            ]);
            publish_capability_file(&fixture.inputs, public_capability(), &possible, &mut clock)
                .unwrap();
        }
        let fixture = custody::tests::Fixture::new();
        possible.store(false, Ordering::SeqCst);
        let _guard = custody::tests::script(&[(Operation::Link, Inject::AfterLinkPanic)]);
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| publish_capability_file(
                &fixture.inputs,
                public_capability(),
                &possible,
                &mut clock
            )))
            .is_err()
        );
        assert!(possible.load(Ordering::SeqCst));
        assert_eq!(std::fs::read_dir(fixture.output()).unwrap().count(), 1);
    }
    #[test]
    fn writable_output_ancestor_preserves_uncertain_publication() {
        use std::os::unix::fs::PermissionsExt;
        for mode in [0o720, 0o702] {
            let fixture = custody::tests::Fixture::new();
            // Only the disposable fixture ancestor is changed, never the user's home.
            std::fs::set_permissions(&fixture.path, std::fs::Permissions::from_mode(mode)).unwrap();
            let possible = AtomicBool::new(false);
            let mut clock = TrustedClock::new().unwrap();
            let outcome = publish_capability_file(
                &fixture.inputs,
                public_capability(),
                &possible,
                &mut clock,
            );
            std::fs::set_permissions(&fixture.path, std::fs::Permissions::from_mode(0o700))
                .unwrap();
            assert_eq!(outcome, Err(Failure::Uncertain));
            assert!(possible.load(Ordering::SeqCst));
            assert_eq!(std::fs::read_dir(fixture.output()).unwrap().count(), 1);
        }
    }
    #[test]
    fn default_acl_output_ancestor_preserves_uncertain_publication() {
        use std::os::fd::AsRawFd;
        use std::os::unix::fs::OpenOptionsExt;
        let fixture = custody::tests::Fixture::new();
        // Change only the disposable parent of output/, after its children exist.
        let ancestor = std::fs::OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC)
            .open(&fixture.path)
            .unwrap();
        let mut acl = 2_u32.to_le_bytes().to_vec();
        for (tag, permissions) in [(1_u16, 7_u16), (4, 0), (32, 0)] {
            acl.extend_from_slice(&tag.to_le_bytes());
            acl.extend_from_slice(&permissions.to_le_bytes());
            acl.extend_from_slice(&u32::MAX.to_le_bytes());
        }
        // SAFETY: live test-owned directory fd, fixed name and initialized public ACL bytes.
        let set = unsafe {
            libc::fsetxattr(
                ancestor.as_raw_fd(),
                c"system.posix_acl_default".as_ptr(),
                acl.as_ptr().cast(),
                acl.len(),
                0,
            )
        };
        assert_eq!(set, 0, "{}", std::io::Error::last_os_error());
        let possible = AtomicBool::new(false);
        let mut clock = TrustedClock::new().unwrap();
        let outcome =
            publish_capability_file(&fixture.inputs, public_capability(), &possible, &mut clock);
        // SAFETY: the same live fixture fd and fixed attribute set above. Fixture Drop
        // also removes this entire disposable tree if the test panics before cleanup.
        let removed = unsafe {
            libc::fremovexattr(ancestor.as_raw_fd(), c"system.posix_acl_default".as_ptr())
        };
        assert_eq!(removed, 0, "{}", std::io::Error::last_os_error());
        assert_eq!(outcome, Err(Failure::Uncertain));
        assert!(possible.load(Ordering::SeqCst));
        assert_eq!(std::fs::read_dir(fixture.output()).unwrap().count(), 1);
    }
    #[test]
    fn renamed_output_path_is_uncertain_without_second_publication() {
        use std::os::unix::fs::DirBuilderExt;
        let fixture = custody::tests::Fixture::new();
        let moved = fixture.path.join("moved-output");
        std::fs::rename(fixture.output(), &moved).unwrap();
        std::fs::DirBuilder::new()
            .mode(0o700)
            .create(fixture.output())
            .unwrap();
        let possible = AtomicBool::new(false);
        let mut clock = TrustedClock::new().unwrap();
        assert_eq!(
            publish_capability_file(&fixture.inputs, public_capability(), &possible, &mut clock),
            Err(Failure::Uncertain)
        );
        assert!(possible.load(Ordering::SeqCst));
        assert_eq!(std::fs::read_dir(moved).unwrap().count(), 1);
        assert_eq!(std::fs::read_dir(fixture.output()).unwrap().count(), 0);
    }
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
