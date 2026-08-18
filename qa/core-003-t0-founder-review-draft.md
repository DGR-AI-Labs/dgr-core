# CORE-003 T0 founder review disposition

**Status:** Complete — APPROVE

This record captures the founder's completed review, scanner adjudication, and
final gate disposition.

## Review identity

- **Reviewer:** `Khazretgali Sapen`
- **Review date/time (UTC):** `2026-08-18T19:12:00Z`
- **Reviewed code commit:**
  `6cb6826fb29ee18bd2ce5f596c620f4170f37a47`
- **Baseline commit:** `4c7f6a33a5f0c01c42eed81b936a77450c8edd40`
- **Reviewed patch SHA-256:**
  `d689cfe6fc092b7cac1fcfea397e09288a169aa2be14c94583cff57eedc905d9`
- **Cross-model bundle SHA-256:**
  `b457be430192665cb9a180423c9ab0eafba6e428f7b1529a9a1ff41b816df8c8`

## A. Authorship and integrity

- [x] I confirm that I authored the T0 enforcement behavior in
  `tests/bypass-rust/src/before_tool_call.rs` at the reviewed commit.
- [x] I confirmed the reviewed commit is exactly
  `6cb6826fb29ee18bd2ce5f596c620f4170f37a47`.
- [x] I confirmed the baseline-to-commit patch SHA-256 is
  `d689cfe6fc092b7cac1fcfea397e09288a169aa2be14c94583cff57eedc905d9`.
- [x] I reviewed all seven changed files rather than only the new match arm.
- [x] I found no unrelated T0 implementation change hidden in the patch.

Suggested verification command:

```bash
git diff --no-ext-diff --full-index --binary \
  4c7f6a33a5f0c01c42eed81b936a77450c8edd40 \
  6cb6826fb29ee18bd2ce5f596c620f4170f37a47 | sha256sum
```

## B. Reached-boundary enforcement review

Review `tests/bypass-rust/src/before_tool_call.rs:82-148`.

- [x] Lines 116-118 place the complete `guard.decide` call inside
  `catch_unwind(AssertUnwindSafe(...))`.
- [x] The unwind closure contains no call to `tool.invoke`.
- [x] Lines 121-129 preserve the returned-Deny relay and do not issue
  authorization or invoke the tool.
- [x] Lines 130-139 preserve the returned-Allow relay and contain the only
  `tool.invoke` call.
- [x] Lines 140-145 map both a typed guard fault and a caught unwind to
  `BeforeToolCallObservation::Blocked`.
- [x] The failure floor uses `RequiredOutcome::FailClosed` and
  `authorization_issued: false`.
- [x] The fixed denial signal does not expose the typed fault or panic payload.
- [x] The panic payload is discarded and is never inspected or resumed.
- [x] Retaining `BeforeToolCallObservation::GuardFault` as a negative-test
  sentinel does not create a reachable raw-fault path in this adapter method.

The reached boundary contains typed guard faults and Rust unwinding panics
from `decide`, returns an explicit fail-closed block, issues no authorization,
and performs no effectful invocation on either failure path. Returned Allow
and Deny relay semantics remain unchanged.

## C. Unwind-safety and scope review

Review `tests/bypass-rust/src/before_tool_call.rs:87-89,112-118` and
`tests/bypass-rust/T0-BOUNDARY.md:3-43`.

- [x] I accept the bounded use of `AssertUnwindSafe` for this invocation.
- [x] I confirm the method does not inspect or reuse `store` after a caught
  unwind before returning fail closed.
- [x] I understand this does not certify the store for a later invocation.
- [x] I confirm the claim covers Rust unwinding panics only, not
  `panic=abort`, process termination, or OOM abort.
- [x] I confirm CORE-003 covers a reached boundary only.
- [x] I confirm hook-never-fired, route-around, missing-plugin, operator-bypass,
  and deployed-runtime non-bypassability remain RUNTIME-003/004 scope.
- [x] I confirm the test adapter invokes only a probe and is not represented as
  the deployed DGR gate.

## D. Adversarial and regression test review

Review `tests/bypass-rust/tests/attack_set.rs:180-270` and
`tests/bypass-rust/tests/adapter_harness.rs:30-109`.

- [x] `Atk07FaultingGuard` injects `Err(GuardFault::InternalError)`.
- [x] `Atk07PanickingGuard` injects an unwind from `decide`.
- [x] The shared ATK-07 assertion derives `FailClosed` from the attack registry.
- [x] The assertion rejects `Proceeded` and a raw `GuardFault` observation.
- [x] Both dedicated ATK-07 tests are active, not ignored.
- [x] Both tests assert the effectful probe remains at zero invocations.
- [x] The obsolete generic `atk_07_hook_error` macro invocation is absent and
  must not be restored.
- [x] Returned-Deny and returned-Allow adapter regression tests remain green.
- [x] The typed-fault adapter regression expects the fixed CORE-003 fail-closed
  observation.
- [x] I reviewed the full all-target result: 39 passed, 6 unrelated cases
  intentionally ignored.

## E. Governance and ownership review

- [x] `T0-AUTHORS.md` records only the
  `BeforeToolCallAdapter::before_tool_call` body as the new founder-owned T0
  surface; surrounding test plumbing remains outside it.
- [x] `AGENTS.md` and `CLAUDE.md` continue to prohibit agent authorship or
  modification of founder-owned enforcement.
- [x] `T0-BOUNDARY.md` accurately records the implemented floor, test-probe
  limitation, and RUNTIME-003/004 scope fence.
- [x] ATK-07 is active without reclassifying ATK-04/05/06/12/14 or hosted
  ATK-15.

## F. Three-engine SAST/SCA review

All raw evidence below is bound to
`6cb6826fb29ee18bd2ce5f596c620f4170f37a47`.

### Semgrep 1.173.0

- Evidence:
  `qa/sast/core-003-t0-semgrep-2026-08-18.txt` and `.json`

- Coverage: 14 of 14 Rust files; 11 Rust rules; 0 scan errors.

- Finding: one `rust.lang.security.temp-dir.temp-dir` information-level result
  at `tests/bypass-rust/tests/consumption_store.rs:19`.

- [x] I reviewed the raw Semgrep output.

- [x] I confirm the finding is confined to the restart-durability test helper,
  not production or new CORE-003 boundary code.

- [x] I accept the existing test-only disposition or record a required fix
  below.

Founder Semgrep disposition:

Semgrep: ACCEPT TEST-ONLY — The single information-level temp-dir finding is
confined to the existing restart-durability test helper and does not touch the
CORE-003 boundary or production enforcement code.

### CodeQL 2.25.5 — `codeql/rust-queries@0.1.35`

- Evidence:
  `qa/sast/core-003-t0-codeql-2026-08-18.txt` and `.sarif`

- Coverage: 14 of 14 Rust files; 0 extraction errors; 0 execution errors.

- Findings: seven `rust/hard-coded-cryptographic-value` results at fixture
  nonce lines 126, 152, 169, 186, 204, 222, and 339 of
  `tests/bypass-rust/src/val_002_fixtures.rs`.

- [x] I reviewed the raw CodeQL output and SARIF.

- [x] I confirm all seven values are deterministic, non-secret test fixture
  nonces and not embedded production credentials or keys.

- [x] I confirm no CodeQL result touches `before_tool_call.rs` or the new
  CORE-003 boundary logic.

- [x] I accept the test-fixture disposition or record a required fix below.

Founder CodeQL disposition:

CodeQL: ACCEPT TEST FIXTURES — All seven findings are deterministic, non-secret
fixture nonces in val_002_fixtures.rs. None touches before_tool_call.rs or the
new CORE-003 enforcement behavior.

### cargo-deny 0.20.2

- Evidence: `qa/sast/core-003-t0-cargo-deny-2026-08-18.txt`

- Result: exit 0; advisories, bans, licenses, and sources all pass with no
  warnings or errors.

- Cargo.lock and `deny.toml` hashes match the reviewed policy baseline.

- [x] I reviewed the complete cargo-deny output.

- [x] I confirm there is no ignored advisory and no blocking diagnostic.

- [x] I confirm this T0 patch changed no dependency or supply-chain policy.

Founder cargo-deny disposition:

cargo-deny: ACCEPT — The scan completed successfully with no advisory, license,
ban, or source warning/error. This change modifies neither dependencies nor
supply-chain policy.

## G. Cross-model review

- [x] I reviewed `qa/core-003-t0-cross-model-review.md`.
- [x] I accept Claude's source-verified PASS as bound to this exact commit.
- [x] I accept the recorded precision corrections: containment is for
  `guard.decide`; the probe-count zero claim is test-bounded; and no generic
  ATK-07 stub should be restored.

## H. Founder decision

Select exactly one:

- [x] **APPROVE** — the CORE-003 T0 boundary is accepted for PR review and
  human merge at the reviewed commit.
- [ ] **APPROVE WITH RECORDED NON-BLOCKING FOLLOW-UP** — describe below.
- [ ] **CHANGES REQUIRED** — do not merge; describe below and repeat affected
  review/SAST gates after changes.
- [ ] **REJECT** — enforcement or evidence is unacceptable.

Findings, exceptions, or follow-up:

No blocking findings. I accept the bounded AssertUnwindSafe use for the reached
guard.decide invocation, the identical fail-closed treatment of typed faults and
unwinding panics, the unchanged Allow/Deny relay behavior, and the explicit
RUNTIME-003/004 scope fence.

Final attestation:

I personally reviewed the founder-authored T0 boundary, adversarial tests,
governance changes, cross-model disposition, and all three raw scanner
outputs at commit `6cb6826fb29ee18bd2ce5f596c620f4170f37a47`. My selected
decision above is the authoritative human gate disposition.

- **Founder name:** `Khazretgali Sapen`

- **Founder signature/approval reference:**
  `https://github.com/DGR-AI-Labs/dgr-core/pull/73`

- **Decision timestamp (UTC):** `2026-08-18T19:59:00Z`
