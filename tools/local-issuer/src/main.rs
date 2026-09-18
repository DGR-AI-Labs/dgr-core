//! Operator-only offline simulation issuer. Agent-authored T0 under Amendment B.
#![deny(unsafe_op_in_unsafe_fn)]
#[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
compile_error!("The approved local issuer requires Linux x86_64");
mod custody;
mod issuance;
mod profile;
mod request;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Failure {
    Invocation,
    Request,
    Launch,
    Custody,
    Terminal,
    Cancelled,
    Unlock,
    Clock,
    Entropy,
    Unpublished,
    Uncertain,
    Resource,
    Internal,
}
impl Failure {
    fn outcome(self) -> (u8, &'static str) {
        match self {
            Self::Invocation => (2, "DGR-E002 invocation"),
            Self::Request => (3, "DGR-E003 request"),
            Self::Launch => (4, "DGR-E004 launch"),
            Self::Custody => (5, "DGR-E005 custody"),
            Self::Terminal => (6, "DGR-E006 terminal"),
            Self::Cancelled => (7, "DGR-E007 cancelled"),
            Self::Unlock => (8, "DGR-E008 unlock"),
            Self::Clock => (9, "DGR-E009 clock"),
            Self::Entropy => (10, "DGR-E010 entropy"),
            Self::Unpublished => (11, "DGR-E011 unpublished"),
            Self::Uncertain => (12, "DGR-E012 uncertain"),
            Self::Resource => (13, "DGR-E013 resource"),
            Self::Internal => (70, "DGR-E070 internal"),
        }
    }
}
fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut result = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        result.push(char::from(DIGITS[usize::from(b >> 4)]));
        result.push(char::from(DIGITS[usize::from(b & 15)]));
    }
    result
}
struct ImmutablePreview {
    invoice: request::ValidatedInvoice,
    binding: profile::Binding,
    commitment: [u8; 32],
}
struct ConfirmedSnapshot(ImmutablePreview);
fn freeze_issuance_preview(
    invoice: request::ValidatedInvoice,
    binding: profile::Binding,
) -> ImmutablePreview {
    let commitment = invoice.action_commitment();
    ImmutablePreview {
        invoice,
        binding,
        commitment,
    }
}
fn confirm_snapshot(
    preview: ImmutablePreview,
    clock: &mut issuance::TrustedClock,
) -> Result<ConfirmedSnapshot, Failure> {
    let mut display = String::from("Simulation invoice capability\n");
    for (field, value) in preview.invoice.fields() {
        display.push_str(&format!("{field}: {value}\n"));
    }
    display.push_str(&format!("profile: {}\nkey_id: {}\npublic_key: {}\ncommitment: {}\nIssuance: current Unix seconds after confirmation; expiry +300 seconds; fresh OS-random nonce.\nType ISSUE {} and Enter: ",hex(&preview.binding.profile_hash),hex(&preview.binding.key_id),hex(&preview.binding.public_key),hex(&preview.commitment),hex(&preview.commitment)));
    let entered = custody::terminal_input(display.as_bytes(), 70, clock)?;
    let expected = format!("ISSUE {}", hex(&preview.commitment));
    if &entered[..] != expected.as_bytes() {
        return Err(Failure::Cancelled);
    }
    Ok(ConfirmedSnapshot(preview))
}
fn run_one_attempt(possible_publication: &AtomicBool) -> Result<(), Failure> {
    custody::harden_process()?;
    let mut clock = issuance::TrustedClock::new()?;
    let raw = custody::read_request(&mut clock)?;
    let invoice = request::parse_request(&raw).map_err(|_| Failure::Request)?;
    let inputs = custody::open_custody_inputs()?;
    clock.observe()?;
    let binding = profile::select_profile_registration(
        &inputs.profile.bytes,
        &inputs.registry.bytes,
        &inputs.config.profile_hash,
    )
    .map_err(|_| Failure::Custody)?;
    custody::verify_envelope_binding(&inputs.envelope.bytes, &binding.envelope_hash)?;
    let password = custody::terminal_input(
        b"Unlock existing simulation signing key: ",
        1024,
        &mut clock,
    )?;
    let key = custody::unlock_once(&inputs.envelope.bytes, password)?;
    profile::verify_unlocked_public_key(&binding, &key.verifying_key())
        .map_err(|_| Failure::Unlock)?;
    clock.observe()?;
    let confirmed = confirm_snapshot(freeze_issuance_preview(invoice, binding), &mut clock)?;
    inputs.recheck()?;
    clock.observe()?;
    let capability = issuance::issue_once(&confirmed, &key, &mut clock)?;
    drop(key);
    issuance::publish_capability_file(&inputs, capability, possible_publication)
}
fn main() -> std::process::ExitCode {
    // Suppress panic payload/backtrace before any input; boundary emits only a fixed code.
    std::panic::set_hook(Box::new(|_| {}));
    let possible_publication = AtomicBool::new(false);
    let result = std::panic::catch_unwind(|| {
        let mut args = std::env::args_os();
        args.next();
        let command = args.next();
        if args.next().is_some() {
            return Err(Failure::Invocation);
        }
        match command.as_deref().and_then(std::ffi::OsStr::to_str) {
            Some("--help") => {
                println!(
                    "Usage: dgr-local-issuer --help | --version | issue\nissue reads one canonical simulation invoice from stdin and requires a controlling terminal.\nOperational custody and backup approval are required before use."
                );
                Ok(false)
            }
            Some("--version") => {
                println!("dgr-local-issuer 0.1.0");
                Ok(false)
            }
            Some("issue") => {
                run_one_attempt(&possible_publication)?;
                Ok(true)
            }
            _ => Err(Failure::Invocation),
        }
    });
    let outcome = match result {
        Ok(Ok(false)) => return std::process::ExitCode::SUCCESS,
        Ok(Ok(true)) => (0, "DGR-I000 published"),
        Ok(Err(error)) if !possible_publication.load(Ordering::SeqCst) => error.outcome(),
        Err(_) if !possible_publication.load(Ordering::SeqCst) => Failure::Internal.outcome(),
        _ => Failure::Uncertain.outcome(),
    };
    eprintln!("{}", outcome.1);
    std::process::ExitCode::from(outcome.0)
}
