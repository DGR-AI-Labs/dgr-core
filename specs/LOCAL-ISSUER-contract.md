# Offline simulation issuer contract

This contract describes only the separately manifested offline signer under
[ADR-14 Amendment B](ADR-14-AMENDMENT-B-reference-contract.md). All policy modules remain T0.
The existing core verifier, replay consumption and approval behavior are unchanged. Runtime
integration, installation, backup/key provisioning and operation require separate authority.

## Request and commitment

Accept only ASCII minimal JSON, the following seven **string** fields in this order, and one LF:

| Field | Value |
|---|---|
| action | `record_invoice` |
| amount | `0` or `[1-9][0-9]{0,19}`, retained as text even above u64 |
| currency | `USD` |
| destination | `simulation-vendor-1` |
| invoice_id | `SIM-` followed by 1–32 uppercase ASCII letters/digits |
| source_account | `simulation-account-1` |
| tool | `dgr.openclaw.invoice.record.v1` |

Maximum 245 bytes. Reject escapes, BOM, CR, extra whitespace/fields, duplicates, reordering,
nonstrings, trailing bytes and missing EOF. Stdin must be regular file or pipe; one absolute
10-second monotonic deadline covers bounded reads plus one lookahead. Restore altered flags.
The input descriptor must be exclusively used by this attempt.

Commitment is SHA-256 of `DGR-ACT2` plus NUL, followed by tags 1–7: one u8 tag, u32 big-endian
byte length, exact raw ASCII value. Maximum 186 preimage bytes. No profile hash is added.
No six-field fallback exists. The maximum amount/all-Z invoice vector hashes to
`89f5d9ac860ed8f405e47ebecb446a9cda24d5a381fee164cbb1e01de0045f7f`.

## Closed profile and registry

Profile: `DGRPROF1`, u16 BE version 1, u16 field count 18, u32 exact total length. Maximum
16384 bytes. Exactly ordered tags 1–18, each u16 tag/u32 length/payload; no unknown fields,
omissions, defaults or trailing data. Entire exact bytes are SHA-256 bound by the launch record.

| Tag | Required payload |
|---|---|
| 1 | `dgr-runtime002/1` |
| 2 | `linux-x86_64-gnu` |
| 3 | `24.21.0` |
| 4 | u32 8 |
| 5 | 20 raw bytes: OpenClaw `9f1c8a1c58bd1e889df4f7e78742ade56d7efefe` |
| 6 | 32-byte build-base digest `69cecf4bbf72d2d44a9eef1b71fb98c7fb973d78af11399deccef19beb008ad9`, then 32-byte runtime image digest |
| 7 | 32-byte nonzero state ID |
| 8 | u32 UID 65532, u32 GID 65532 |
| 9 | Exactly two directory roles, described below |
| 10 | Exactly eight distinct state-object identities, described below |
| 11 | Exactly eight artifact digest/path roles, described below |
| 12 | u8 count 1, 16-byte key ID, 32-byte Ed25519 public key |
| 13 | Five u32 values: 1000, 5000, 1, 0, 1 |
| 14 | `dgr.openclaw.invoice.record.v1` |
| 15 | `DGR-ACT2` plus NUL |
| 16 | u32 1 |
| 17 | `sync-port/1` |
| 18 | u32 31 |

A variable path is u16 length and absolute ASCII bytes, at most 512, with components at most
255 bytes containing only letters/digits/period/underscore/hyphen. No empty/dot/dot-dot segments,
repeated/trailing slash or root-only path. All integers here are unsigned big-endian.

Directory payload: u8 count 2; each role in order has u8 role, u64 device, nonzero u64 inode,
u32 mode 0700, variable path. Paths are `/var/lib/dgr/state` and `/var/lib/dgr/invoice-output`.
State objects: u8 count 8; u8 role/u64 device/nonzero u64 inode/u32 mode 0600. Roles are
consumption.sqlite3, approval.sqlite3, consumption.journal.data, consumption.journal.records,
approval.journal.data, approval.journal.records, clock-state.records and instance.lock. Reject
reused object device/inode pairs. Signer validates encoding; runtime validates actual Docker-local
artifacts and objects. Signer never treats these identities as paths to inspect on its own host.

Artifact payload: count 8 and ordered u8 role/32-byte digest/variable path. Exact paths:
`/opt/dgr/lib/dgr_core_node.node`, `/opt/dgr/lib/bootstrap.mjs`, `/opt/dgr/lib/invoice-port.mjs`,
`/etc/dgr/openclaw.json`, `/etc/dgr/protected-tools.json`, `/etc/dgr/dependency-inventory.json`,
`/usr/local/bin/node`, `/opt/dgr/openclaw/openclaw.mjs`. Real digests and allocated identities
require later reviewed build/enrollment; zeros/wildcards are not substitutes for operational
approval. Profile and registry remain outside the image they describe to avoid circular hashes.

Registry: `DGRREG1` plus NUL, u16 version 1, u16 count 1–256, u32 total 16+112*count (max28688).
Each row is profile hash32/key ID16/public key32/envelope hash32. Strict profile-digest order,
independently unique digests, IDs and public bytes. Every key must decode, have canonical y below
2^255−19 and be non-weak. The selected profile/key/envelope and derived unlocked public key must
all match. No discovery, rotation, reload or historical/global uniqueness claim.

Compiled inventory: `DGRPUB1` plus NUL, u32 count 3 and sorted raw public keys:

- a09aa5f47a6759802ff955f8dc2d2a14a5c99d23be97f864127ff9383455a4f0
- d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737
- d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a

Its hash is `9ad0e809fa2d8ab77fb24c1cbb11748cbfeba03d183ee96ad35e50cf6231afd7`.
These public fixture keys are rejected for operational acceptance. This finite list does not
prove that an unlisted key is private; actual custody/provenance and exclusive enrollment remain
operator responsibilities.

## Host trust, descriptors and process limits

Only Linux x86_64, a matching non-root real/effective/saved UID/GID and zero effective/permitted/
inheritable/ambient capabilities are supported. No setuid service, environment-selected trust
anchor or caller namespace establishes authority. Before secrets: lower RLIMIT_AS to at most
512 MiB and CPU soft/hard to at most 10 seconds, preserving tighter inherited limits; disable
core dumps/dumpability, set no-new-privileges, umask077 and checked close_range above fd2.
Unsupported or failed hardening aborts, never relaxes policy.

Fixed `/etc/dgr/local-issuer/launch.bin` is root-owned regular0444/nlink1, opened nofollow,
nonblocking and close-on-exec through a held root-owned directory chain without group/other
write or set-ID. Its maximum1236-byte record is immutable and exactly encoded:

| Offset | Bytes | Field |
|---|---:|---|
| 0 | 8 | `DGRLCH1` plus NUL |
| 8 | 2 | version1 |
| 10 | 2 | flags0 |
| 12 | 4 | exact total212+two path lengths |
| 16,20 | 4 each | operator UID/GID |
| 24,32,40,48 | 8 each | mount namespace device/inode; user namespace device/inode |
| 56 | 8 | root mount ID |
| 64,96 | 32 each | registry/profile SHA-256 |
| 128,136,144 | 8 each | custody device/inode/mount ID |
| 152,160,168 | 8 each | output device/inode/mount ID |
| 176 | 32 | compiled inventory hash |
| 208,210 | 2 each | custody/output path lengths1–512 |
| 212 | variable | custody path then output path |

The two roots are distinct, neither an ancestor of the other. Ancestors are root/operator-owned,
without group/other write or set-ID. Final roots are operator0700 on ext4 with exact recorded
identities. statx mount-ID support is required. Only the approved root-to-selected-mount
transition is allowed, at most once. Required APIs have no weaker fallback.

Inside roots, open fixed basenames with openat2 BENEATH/NO_SYMLINKS/NO_MAGICLINKS/NO_XDEV,
NONBLOCK/CLOEXEC; reject nonregular files, extra links, wrong owner/mode/bounds/hash and metadata
changes. Keep descriptors and immutable input bytes through confirmation and publication.
For filesystem custody/launch/output paths, access ACL and directory default ACL queries accept
only ENODATA. Any nonnegative size or other error, including EOPNOTSUPP, rejects.

Approved P1 exception: fixed kernel procfs descriptors form a separate class with no POSIX ACL
queries. The held nofollow `/proc` must be procfs, root-owned directory, without group/other write
or set-ID, with expected current PID/UID observations. Fixed namespace handles must match the
launch record; recheck proc identity/mode/owner/filesystem and namespaces before publication.
Only `self`, `self/ns/mnt`, `self/ns/user` and the live owned `self/fd/<fd>` publication source
are used. This is not a generic unsupported-ACL fallback or arbitrary proc traversal permission.
Trusted administrator/kernel/mount provenance remains a host assumption.

## Unlock and confirmation

age0.12.1, default-features=false. Input bounded to4096 then exact214 bytes: 150-byte header
and nonce16/ciphertext48. Exact LF lines: `age-encryption.org/v1`; `-> scrypt ` plus canonical
unpadded standard base64 salt16, ` 18`; canonical base64 body32; `--- ` plus canonical base64
MAC32. Decode/re-encode must be identical, including pad bits. Reject armor, extra stanzas,
other costs, concatenation and variations before identity/KDF. scryptN=2^18/r8/p1; set maximum18
in addition to the exact precheck. Library calibration itself occurs after process hardening.
Require authenticated32 bytes and a successful additional EOF read before deriving the key.

Controlling `/dev/tty` must be foreground. Save exact termios and signal mask. Bound passphrase
to1–1024 UTF-8 bytes, no trimming/normalization/NUL/control/escape; CR or LF terminates, BS/DEL
erases one full codepoint. Disable echo/canonical buffering/CR translation. Absolute60-second
monotonic deadline, no per-byte extension. Poll signalfd for blocked INT/TERM/HUP/QUIT/TSTP;
handled cancellation ends the attempt. Flush queued input, restore/verify termios and restore
mask after closing terminal/signal descriptors before KDF. Fatal termination has no cleanup
guarantee. Late handled signals still cancel; restoration failure takes precedence.

After key correspondence, preview the immutable seven fields, profile, key ID/public key,
commitment and issuance parameters. Flush input and require exact `ISSUE ` plus commitment
lowercase hex under a separate60-second deadline. No input reread, suffix, re-prompt or mutation
is authorized. Owned secret and capability buffers zeroize; no blanket erasure claim.

## Token and publication

Observe wall and monotonic clocks throughout the attempt; reject observed backward readings,
read/arithmetic errors and expiry overflow. No cross-launch rollback protection is claimed.
Choose Unix seconds after confirmation, expiry=issue+300 and fully filled16 OS-random bytes.
No supplied nonce, ledger, auto-retry or mathematical global uniqueness guarantee.

Wire145 bytes: version1 at0, key ID1..17, issued u64BE17..25, expiry25..33, nonce33..49,
commitment49..81, signature81..145. Plain Ed25519 signs `DGR-CAP1`+NUL and first81 bytes
(90 total); self-verify before publishing. Preserve runtime30-second skew, global nonce-only
consumption and existing approval threshold/window semantics.

Create an anonymous O_TMPFILE0600 in held output root, verify owner/device/type/nlink0, write
291 bytes (290 lowercase hex plusLF), fsync. Mark possible publication before one linkat via the
fixed owned proc FD to `cap-<noncehex>.txt`. EEXIST is definitely unpublished; never follow,
overwrite or remove a collision. Other link errors and every later failure are uncertain.
Verify linked inode/nlink1, fsync directory and verify approved path still reaches the held root.
No success until every step passes. Preserve possibly published artifacts and require operator
investigation after uncertainty/crash; no cleanup/reissue/reset command exists.

## Fixed outcomes

For issue, stdout is empty and ordinary reporting emits exactly one fixed ASCII stderr line.
No caller text, paths, errno/library details, passwords, requests or tokens are interpolated.

| Exit | Line |
|---:|---|
| 0 | DGR-I000 published |
| 2 | DGR-E002 invocation |
| 3 | DGR-E003 request |
| 4 | DGR-E004 launch |
| 5 | DGR-E005 custody |
| 6 | DGR-E006 terminal |
| 7 | DGR-E007 cancelled |
| 8 | DGR-E008 unlock |
| 9 | DGR-E009 clock |
| 10 | DGR-E010 entropy |
| 11 | DGR-E011 unpublished |
| 12 | DGR-E012 uncertain |
| 13 | DGR-E013 resource |
| 70 | DGR-E070 internal |

First failing phase applies, except terminal restoration failure overrides cancellation/unlock,
and possible publication overrides later errors/panics. Kill/stop/abort/power loss cannot
promise a status line. A missing message does not authorize retry.
