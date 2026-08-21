# CORE-004 T0 independent human review disposition

**Status:** Completed — PASS

This record satisfies a different gate from founder authorship and sign-off.
The reviewer must be a human who did not author the CORE-004 founder-owned T0
implementation. Codex and Claude reviews do not satisfy this gate.

Do not check a box merely because it is recommended. The independent reviewer
must inspect the exact patch and personally author the identity, findings,
disposition, and attestation fields.

## Review identity and artifact binding

- **Independent reviewer:** `Gaziz Nugmanov`
- **Reviewer relationship/role:** `Co-Founder; independent reviewer; did not author the CORE-004 T0 implementation.`
- **Review date/time (UTC):** `2026-08-21T17:05:00Z`
- **Reviewed code commit:**
  `60febb08ac9c3e207d6f7a3563b6824374c5c93e`
- **Baseline commit:** `7324cbb33be59595657a2df13c300aa388208d77`
- **Reviewed patch SHA-256:**
  `71f051e24055cb0febd620d84a2703ea43d7277f5af4266feef8d03d0fbb9f1f`
- **Stable approval/review reference:** This signed repository record and [dgr-core PR #81](https://github.com/DGR-AI-Labs/dgr-core/pull/81)

Suggested integrity command:

```bash
git diff --no-ext-diff --full-index --binary \
  7324cbb33be59595657a2df13c300aa388208d77 \
  60febb08ac9c3e207d6f7a3563b6824374c5c93e | sha256sum
```

Expected result:

```text
71f051e24055cb0febd620d84a2703ea43d7277f5af4266feef8d03d0fbb9f1f  -
```

## Required review scope

Review the complete baseline-to-commit patch, including these eleven files:

- `qa/core-004-t0-founder-review-input.md`
- `tests/bypass-rust/src/before_tool_call.rs`
- `tests/bypass-rust/src/fixtures.rs`
- `tests/bypass-rust/src/founder_approval_store.rs`
- `tests/bypass-rust/src/founder_approval_timeout.rs`
- `tests/bypass-rust/src/founder_authored_guard.rs`
- `tests/bypass-rust/src/founder_s2_approval_store.rs`
- `tests/bypass-rust/src/lib.rs`
- `tests/bypass-rust/tests/adapter_harness.rs`
- `tests/bypass-rust/tests/attack_set.rs`
- `tests/bypass-rust/tests/core_004_conformance.rs`

## Independent code-review checks

- [x] I did not write the founder-owned CORE-004 T0 implementation.
- [x] I verified the exact commit and patch digest above.
- [x] I reviewed all eleven changed files, not only the live escalation branch.
- [x] `GuardDecisionPort::decide` and the adapter receive consumption and
  approval stores explicitly; neither store is global or constructed inside a
  decision.
- [x] Signature, lifetime/expiry, and action binding precede the approval
  trigger in `founder_authored_guard.rs:63-130`.
- [x] The amount parser at `founder_authored_guard.rs:182-200` accepts only
  canonical unsigned decimal strings and compares by length then bytes, so an
  arbitrarily long value cannot overflow into Allow.
- [x] Only a bound amount strictly greater than `1_000_000` enters escalation.
- [x] The review-request ID is domain-separated and derived only from verified
  key ID, nonce, and action commitment.
- [x] Deadline calculation uses checked `requested_at + 86_400`.
- [x] Pending state is committed before `Escalate` is returned, and the
  escalation branch returns before nonce consumption.
- [x] `Recorded` must return the exact candidate; `AlreadyPending` must retain
  the same review ID, key ID, nonce, action commitment, and original deadline;
  contradictory store results fail closed.
- [x] The SQLite table is STRICT, validates fixed BLOB lengths and timestamp
  order, constrains status, and uniquely binds both review ID and `(key_id,
  nonce)`.
- [x] First record, deduplication, and timeout evaluation use immediate
  transactions and observe only committed results.
- [x] `now <= deadline` returns the unchanged pending record; only
  `now > deadline` atomically commits `requested -> denied_on_timeout` before
  returning the terminal result.
- [x] The R-3 timeout function accepts no token and has no tool-invocation or
  authorization path.
- [x] Missing records, store faults, arithmetic failures, malformed rows, and
  identity mismatches cannot become Allow or a fresh escalation.
- [x] The five dedicated CORE-004 tests are active and prove the two-surface
  sequence, strict timeout boundary, unchanged below-threshold path,
  re-presentation identity/deadline, zero effects, and unconsumed nonce.
- [x] The full Rust result is 52 passed, 0 failed, with only five unrelated or
  external attacks ignored; no ATK-06 test remains ignored.
- [x] The bounded claim is accurate: this is a single-guard/local-store/modelled-
  clock isolation proof, not real human delivery, cross-instance approval,
  approve-to-allow, or deployed-runtime non-bypassability.
- [x] I found no weakened registry outcome, bypassed test, or unrelated
  enforcement change.

## Review findings

The reviewer recorded `None` only after completing the entire review.

```text
None
```

## Independent disposition

Select exactly one after completing the review:

- [x] **PASS** — no blocking defect; the exact reviewed commit is acceptable.
- [ ] **PASS WITH NON-BLOCKING FINDINGS** — all findings are recorded above.
- [ ] **CHANGES REQUIRED** — do not merge; repeat affected gates on the
  replacement commit.
- [ ] **REJECT** — the enforcement boundary or evidence is unacceptable.

## Reviewer attestation

The reviewer's attestation follows:

```text
Gaziz Nugmanov independently reviewed the complete CORE-004 patch from
7324cbb33be59595657a2df13c300aa388208d77 through
60febb08ac9c3e207d6f7a3563b6824374c5c93e and verified patch SHA-256
71f051e24055cb0febd620d84a2703ea43d7277f5af4266feef8d03d0fbb9f1f.
Gaziz Nugmanov did not author the founder-owned T0 implementation.
Disposition: PASS.
Findings: None.
```

- **Independent reviewer name:** Gaziz Nugmanov
- **Signature/approval reference:** `Gaziz Nugmanov`; this signed repository record and [dgr-core PR #81](https://github.com/DGR-AI-Labs/dgr-core/pull/81)
- **Decision timestamp (UTC):** `2026-08-21T17:10:00Z`
