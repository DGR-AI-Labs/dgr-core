# VAL-002 deterministic signed-token fixtures report

Date: 2026-08-11

Branch: `feat/val-002-fixtures`

Base: `dgr-core` `origin/main` at `dc594c434eff0b3f0df967dac33ec323a78da924`

## Result

VAL-002 fixture data is ready for founder review. The catalog deterministically
authors Ed25519-signed token artifacts, ARCH-005 action commitments, fixed-clock
boundary cases, request variants, attack tags, and expected outcome sequences.
It does not verify tokens, classify requests, consume nonces, or return an
authorization decision.

## Normative inputs

The refreshed `specs/CORE-002-reference-contracts.md` remains a pointer rather
than a spec copy. Both referenced documents resolved at immutable
`dgr-internal` commit `9dd14d757ed108037bdeda8e5bb14da22d90bbba`, were
`active`, and had `last-reviewed: 2026-08-10`. That commit is an ancestor of the
merged placement tip `a563f388bca240ab94ddbf491582e58ac96f988f`.

| Spec | SHA-256 of pinned file |
|---|---|
| `ARCH-005-typed-authorization-schema` | `178edbfc1ff0b597c412c782000c74c2db46d2f9bbee6d8ebb38a86d7afa3f98` |
| `ARCH-006-token-wire-format` | `28fbc85064765f4374f0fbd1167d282112baac768fcef1b1d0f519415614ff6a` |

Resolved frozen inputs: format version 1; `DGR-CAP1\x00` domain tag; 16-byte
key ID and nonce; unsigned big-endian Unix seconds; 300-second lifetime;
30-second skew; SHA-256 action commitment; Ed25519 signature; 145-byte token
wire format; unpadded base64url transport; K2 pinned public-key model.

## Deterministic fixture material

The fixed clock reports `1800000000`. The baseline `pay_invoice` action binds,
in tag order, `action=pay_invoice`, `amount=100000` integer minor units,
`currency=USD`, `destination=acct_payee_31`, `invoice_id=INV-8842`, and
`source_account=acct_ops_07`. All values are ASCII and therefore already NFC.
Each field is encoded as `tag || u32_be(length) || UTF-8 bytes`; the resulting
bytes hash to:

`eae6c0a0fea8c2a0902855a367108b90d359c491aab607cc33bd32ec18c76ef3`

The signature preimage stored with every signed artifact is exactly:

`DGR-CAP1\x00 || format_version || key_id || issued_at || expires_at || nonce || action_commitment`

The registered fixture key uses the deliberately public seed `0x11` repeated
32 times, key ID `DGR-TEST-KEY-001`, and public key
`d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737`.
The second seed (`0x22` repeated 32 times) and key ID `DGR-TEST-KEY-999` author
only the unknown-key negative fixture and are absent from the fixture K2 set.
Both are labelled test-only in source and `tests/fixtures/keys/README.md`.

Golden valid-token transport:

`AURHUi1URVNULUtFWS0wMDEAAAAAa0nRxAAAAABrSdLwAQEBAQEBAQEBAQEBAQEBAermwKD-qMKgkChVo2cQi5DTWcSRqrYHzDO9MuwYx27zJ1jcKrddW9PZg_AH0nK7pxAAf7li9ZIzyLJ4BAEWSyhV6jv5JS3WlXkZPWRKwuV7Sq7ro_GjpEmLMtPbkarpDw`

## Fixture matrix

Expected outcomes below are labels for future founder-authored conformance
behavior; fixture code does not compute them.

| Fixture | Attack/control tag | Variation | Expected sequence |
|---|---|---|---|
| `valid` | ATK-11 control | Matching signed token and request; 300s lifetime | allow |
| `expired-beyond-skew` | ATK-02 | Expired by 120s | deny |
| `expired-within-skew` | ATK-02 | Expired by exactly 30s | allow |
| `expired-just-outside-skew` | ATK-02 | Expired by 31s | deny |
| `swap-amount` | ATK-08 | Bound amount changed | deny |
| `swap-destination` | ATK-08 | Bound destination changed | deny |
| `swap-invoice-id` | ATK-09 | Bound invoice ID changed | deny |
| `wrong-action` | ATK-11 | Bound action literal changed | deny |
| `swap-source-account` | ATK-08 | Bound source account changed | deny |
| `change-idempotency-key` | ATK-08/09/11 control | Only non-binding idempotency key changed | allow |
| `change-memo` | ATK-08/09/11 control | Only non-binding memo changed | allow |
| `replay` | ATK-03 | Exact valid token presented twice | allow, then deny |
| `unknown-key-id` | ATK-10 | Signed by the unregistered fixture key | deny |
| `absent-token` | ATK-01 | Token omitted | deny |
| `tampered-expires-at` | ATK-10 | Wire expiry changed after signing | deny |
| `malformed-amount-decimal` | ATK-08 | `1000.00` is not canonical minor units | deny |
| `malformed-amount-leading-zero` | ATK-08 | `0100000` has a leading zero | deny |

The conformance adapter accepts each fixture's emitted wire bytes without
interpreting them. The complete presented action, fixed clock, canonical-byte
artifacts, malformed marker, attack tags, and expected sequence remain attached
to the fixture for founder-authored guard steps to consume later.

## Checks

- `cargo fmt --check`: PASS.
- `cargo build`: PASS.
- `cargo test --test val_002_fixtures`: PASS (9 passed).
- `cargo test -- --skip atk_01_no_authorization_is_blocked_before_tool_execution`:
  PASS (fixture and non-founder active tests pass; 14 future conformance tests
  remain ignored).
- `cargo test`: expected RED at active ATK-01 because
  `FounderAuthoredGuard::decide` still raises
  `FounderImplementationRequired`; result was 3 passed, 1 failed, 14 ignored in
  `attack_set` before Cargo stopped. This confirms the task did not make the
  founder-owned gate green.

## T0 boundary attestation

No signature-verification code, token parser, authorization decision,
fail-closed behavior, SQLite/nonce consumption, or production trust store was
authored. The only Ed25519 operation is fixture signing. The only SHA-256
operation is fixture commitment construction. The fixture K2 set is inert test
data and exposes no verification or lookup behavior.

All five founder units remain byte-identical to `origin/main` and retain their
single `unimplemented!("FounderImplementationRequired")` call:

| Founder unit | SHA-256 |
|---|---|
| `founder_authored_guard.rs` | `27132ba04cc955041926ef52582166b4eb200ebca2f9405143c4b8d5d2cd3b77` |
| `founder_consumption_store.rs` | `d161246cb7850375d3903a82899a48203d41872c52bbc5a135093f0a0475d2d9` |
| `founder_fail_closed.rs` | `10f8063a59848d623215276ed0ff30715c3bc337516cc5adc9c9dfa7c1edcb66` |
| `founder_s2_consumption_store.rs` | `d765d43063cb34d9bcd1d314e33307d199e39fe57da4aff24881543b5bbcfa0c` |
| `founder_token_verification.rs` | `f5b31fb01a314c9082b63e25cb253040be3c88d12d54205b54a934336d330628` |

Nothing in this work is merged to `main`. Founder review must confirm the six
field binding boundary, the non-binding controls, ARCH-006 bytes, 300s/30s
values, and the deliberately public test key material before merge.
