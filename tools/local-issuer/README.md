# Offline simulation invoice issuer

This Linux x86_64 operator CLI creates one simulation capability after a controlling-terminal
unlock and confirmation. It is separate from OpenClaw/Docker and has no network service, batch
mode, key-generation command or runtime callback. It does not decide whether the runtime should
execute an invoice. All five policy modules are agent-authored T0 under the bounded Amendment B
exception; implementation and operation have separate review/authorization gates.

The current public core uses an earlier commitment and test trust key. This issuer's seven-field
DGR-ACT2/profile contract requires the separately authorized runtime work before interoperability
can be claimed. A successful fixture test is not operational issuance or integration evidence.

## Build and check

Use the repository's pinned Rust 1.94.1 toolchain and Cargo.lock:

```sh
cargo build --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --all-targets --locked
cargo build -p dgr-local-issuer --release --locked
target/release/dgr-local-issuer --help
```

The Linux syscall tests require a non-root user with an owner-controlled ext4 home directory
whose ancestors meet the custody rules. They create random mode-0700 `.dgr-issuer-test-*`
directories there, using public fixtures only, and remove them on normal completion. Run tests
on an isolated host/account with no hostile same-UID co-tenant and a private mode-0700 TMPDIR.
The cryptographic fixture tests are intentionally expensive in debug builds. An interrupted
run can leave disposable fixture directories; they are not custody roots or operational tokens.

The pinned instrumented unit-test lane uses rebuilt std and AddressSanitizer separately from
release hardening. It cannot run under the production 512 MiB virtual-memory limit. Production
has no sanitizer detection, test feature or environment override that relaxes that limit.

## Before any operational use

An administrator must separately approve/install the root-owned launch record at
`/etc/dgr/local-issuer/launch.bin`. It binds the non-root operator, trusted mount/user namespaces,
root mount, custody/output directories, full registry/profile hashes and public-fixture inventory.
Installation, enrollment, backup, restore, rotation and key import are not implemented here.
An independent encrypted backup, actual existing-key provenance, the complete operational
profile and explicit operation approval are prerequisites; a code merge supplies none of them.

Both custody and output roots must be distinct owner-only ext4 directories on the trusted
Linux/WSL host. Windows `/mnt/c`, overlay, FUSE and network storage are outside this profile.
Both roots and every directory in their full ancestor chains must have no POSIX access or
default ACL: both ACL queries must report ENODATA. Root ownership and mode 0755 alone do not
establish this; an inherited default ACL on a parent such as `/home` is rejected. Check the
full chain before use; changing shared-host ACLs requires separate administrator approval.
Keep the signer and its secrets outside runtime containers and agent-accessible tool catalogs.
Trusted root/same-UID/kernel compromise is outside the protection claimed by these checks.

Custody contains only previously approved `registrations.bin` and `profile.bin` (0400), and
`signing-key.age` (0600). The single accepted encrypted format is binary age, scrypt logN=18,
r=8, p=1, exactly one 32-byte seed and exactly 214 encrypted bytes. Other valid age forms are
rejected, not converted. The complete registry reserves a different key ID and public key for
each profile; the compiled public fixtures are always rejected by production acceptance.

## One authorized attempt

The only commands are `--help`, `--version`, and `issue`, with no additional arguments.
`issue` reads one minimal ASCII JSON object followed by exactly one LF from regular-file or
pipe stdin. For example, this is the exact request carrier, not a shell command:

```json
{"action":"record_invoice","amount":"1","currency":"USD","destination":"simulation-vendor-1","invoice_id":"SIM-A","source_account":"simulation-account-1","tool":"dgr.openclaw.invoice.record.v1"}
```

The approved request file is supplied on stdin. Unlock and confirmation come only from the
foreground controlling terminal, with separate 60-second deadlines. No password, key, profile,
nonce or time may be supplied through flags or environment. Read the entire preview, including
all seven fields, profile hash, public key, key ID, commitment and issuance parameters. Confirm
by typing `ISSUE ` and the exact 64 lowercase hexadecimal commitment shown. Cancellation ends
the attempt and drops the unlocked key; there is no re-prompt or persistent unlocked session.

Success writes a mode-0600 file named `cap-<32 lowercase nonce hex>.txt` in the approved output
root. It contains 290 lowercase token hex characters and one LF. It is a bearer capability:
keep it private, out of logs/evidence and out of terminal previews. stdout is empty. A single
fixed stderr status reports the result; see the [contract](../../specs/LOCAL-ISSUER-contract.md).

Never retry automatically. `DGR-E012 uncertain`, unexplained termination or a missing success
message can leave a published file. Preserve it and investigate the private output directory,
possible consumption/effect and expiry under the separately approved recovery procedure. Never
overwrite/delete a collision, reset runtime state or infer non-consumption from an error.
Fatal signals/power loss cannot guarantee terminal restoration or erasure; terminal recovery
may require `stty sane`, but recovery is not authorization to issue again.

Owned passphrase/seed/token buffers use zeroizing storage. This is not a guarantee about library
intermediates, allocator copies, swap/hibernation, administrator access or fatal termination.
