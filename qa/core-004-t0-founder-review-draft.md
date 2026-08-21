# CORE-004 T0 founder review disposition

**Status:** Draft — founder completion required

This agent-prepared form records factual evidence and suggested wording. It is
not founder approval. The founder must personally inspect the exact patch,
cross-model disposition, raw scanner artifacts, and independent-human review;
then author every marked field, check each accepted statement, and select the
final decision.

## Review identity

- **Founder/reviewer:** `Khazretgali Sapen`
- **Review date/time (UTC):** `[FOUNDER TO AUTHOR]`
- **Reviewed code commit:**
  `60febb08ac9c3e207d6f7a3563b6824374c5c93e`
- **Baseline commit:** `7324cbb33be59595657a2df13c300aa388208d77`
- **Reviewed patch SHA-256:**
  `71f051e24055cb0febd620d84a2703ea43d7277f5af4266feef8d03d0fbb9f1f`
- **Cross-model bundle SHA-256:** `[ADD AFTER BUNDLE VERIFICATION]`
- **Independent-human review reference:** `[ADD AFTER INDEPENDENT REVIEW]`

## A. Authorship and integrity

- [ ] I confirm I authored the founder-owned CORE-004 T0 behavior in the five
  implementation files listed by `T0-AUTHORS.md`.
- [ ] I verified the reviewed commit is exactly
  `60febb08ac9c3e207d6f7a3563b6824374c5c93e`.
- [ ] I verified the baseline-to-commit patch SHA-256 is
  `71f051e24055cb0febd620d84a2703ea43d7277f5af4266feef8d03d0fbb9f1f`.
- [ ] I reviewed the complete eleven-file patch.
- [ ] I found no unrelated or agent-authored T0 enforcement logic in the patch.

## B. Guard and escalation review

Review `tests/bypass-rust/src/founder_authored_guard.rs:55-235`.

- [ ] Verification, lifetime/expiry, and binding run before the escalation
  trigger.
- [ ] Canonical amount validation rejects empty, non-digit, and leading-zero
  multi-digit strings.
- [ ] The overflow-safe length/byte comparison escalates only above
  `1_000_000` minor units.
- [ ] Review IDs use the domain tag `DGR-CORE004-REVIEW-V1\0` and verified
  key ID, nonce, and action commitment only.
- [ ] Deadline creation uses checked `requested_at + 86_400`.
- [ ] Pending state is durably recorded before `Escalate` is observed.
- [ ] Escalation returns before consumption; below-threshold requests retain
  the existing consume-then-Allow path.
- [ ] First-write contradictions, re-presentation identity mismatch, invalid
  stored timestamps, and store faults fail closed.

## C. Approval port, SQLite store, and timeout review

Review `founder_approval_store.rs:8-66`,
`founder_s2_approval_store.rs:21-364`, and
`founder_approval_timeout.rs:14-63`.

- [ ] The port distinguishes Recorded, AlreadyPending, Pending, TimedOut,
  NotFound, and Faulted without a fail-open default.
- [ ] The SQLite schema is STRICT and enforces all selected length, timestamp,
  status, primary-key, and unique-identity constraints.
- [ ] `synchronous=FULL` is configured and insertion/deduplication use an
  immediate transaction.
- [ ] Re-presentation returns the original committed pending record and cannot
  extend its deadline.
- [ ] Timeout evaluation performs lookup, trusted-clock comparison, status
  transition, and commit as one store operation.
- [ ] `now <= deadline` remains pending; only `deadline < now` commits and
  returns TimedOut.
- [ ] An already timed-out record remains terminal and does not reopen.
- [ ] The token-free R-3 surface cannot authorize or invoke a tool.
- [ ] Missing and operationally uncertain state maps to explicit fail-closed
  denial; only a committed timeout receives the registry-derived ATK-06
  terminal outcome.

## D. Adapter, tests, and bounded claim

- [ ] The adapter relays `Escalate` without authorization or tool invocation.
- [ ] Both stores are injected explicitly and are not reused after a caught
  unwind within the current invocation.
- [ ] All five CORE-004 conformance tests are active and pass.
- [ ] Re-presentation preserves the review ID and deadline, and the test proves
  the capability nonce was not consumed by escalation.
- [ ] `deadline - 1`, `deadline`, and `deadline + 1` prove the strict boundary.
- [ ] The full suite passes 52 tests; the five ignored attacks are unrelated
  deferred or external scope, and ATK-06 is not ignored.
- [ ] I accept the explicit scope fence: no real human delivery/wait,
  approve-to-allow, cross-instance state, or deployed-runtime
  non-bypassability is claimed.

## E. Three-engine SAST/SCA review

All evidence below is bound to
`60febb08ac9c3e207d6f7a3563b6824374c5c93e`.

### Semgrep 1.173.0 — `p/rust`

- Evidence: `qa/sast/core-004-t0-semgrep-2026-08-21.txt` and `.json`.
- Coverage: 20 of 20 tracked Rust files; 11 Rust rules; 0 scan errors.
- Finding: one information-level `rust.lang.security.temp-dir.temp-dir` at
  `tests/bypass-rust/tests/consumption_store.rs:19`.
- [ ] I reviewed the complete text and JSON evidence.
- [ ] I confirmed the finding is confined to the existing test-only temporary
  SQLite path helper and does not touch CORE-004 enforcement.
- [ ] I either accept the risk explicitly below or require remediation and a
  complete rerun.

**Suggested founder wording — replace or personally accept:**

```text
Semgrep: ACCEPT TEST-ONLY. The single INFO finding is confined to the existing
restart-durability test helper. Its PID-plus-high-resolution-time filename can
be raced by another local process, so I do not treat it as safe production
temporary-file construction. The helper is test-only, invokes no production
tool or authorization path, and does not touch CORE-004 enforcement. No
suppression was added. I accept this bounded test-only risk for this gate.
```

**Founder Semgrep disposition:** `[FOUNDER TO AUTHOR]`

### CodeQL 2.25.5 — `codeql/rust-queries@0.1.35`

- Evidence: `qa/sast/core-004-t0-codeql-2026-08-21.txt` and `.sarif`.
- Coverage: 20 of 20 tracked Rust files; 0 extraction errors; 0 execution
  errors.
- Findings: nine `rust/hard-coded-cryptographic-value` results: seven VAL-002
  fixture nonces and two VAL-004 fixture nonces at lines 136 and 143.
- Diagnostic metric: seven path-resolution inconsistencies, with complete
  file extraction and no extraction or execution error.
- [ ] I reviewed the complete text evidence and SARIF.
- [ ] I confirmed all nine values are deterministic, non-secret fixture nonces
  and not production credentials, keys, salts, or runtime nonces.
- [ ] I confirmed no CodeQL finding touches a founder-owned CORE-004 file.
- [ ] I reviewed and explicitly accept or reject the path-resolution diagnostic
  limitation below.

**Suggested founder wording — replace or personally accept:**

```text
CodeQL: ACCEPT TEST FIXTURES WITH DIAGNOSTIC LIMITATION. All nine findings are
deliberate deterministic nonces used to make VAL-002 and VAL-004 conformance
artifacts reproducible; none is a secret or production runtime nonce, and no
finding touches founder-owned CORE-004 enforcement. CodeQL extracted all 20
tracked Rust files with zero extraction and execution errors. I acknowledge
the Rust extractor's seven path-resolution inconsistency diagnostics; because
they produced no lost file coverage or reported security result in the
reviewed founder-owned files, I accept that analyzer limitation for this gate.
```

**Founder CodeQL disposition:** `[FOUNDER TO AUTHOR]`

### cargo-deny 0.20.2

- Evidence: `qa/sast/core-004-t0-cargo-deny-2026-08-21.txt`.
- Result: exit 0; advisories, bans, licenses, and sources contain no error or
  warning.
- Policy notes: two explicitly justified duplicate-version skips and 54
  accepted-license notes.
- `advisories.ignore` remains empty.
- [ ] I reviewed the complete cargo-deny evidence and `deny.toml`.
- [ ] I confirmed no advisory is ignored and no blocking diagnostic exists.
- [ ] I confirmed the existing `hashbrown@0.16.1` and `syn@2.0.119` skips remain
  temporary transitive-version exceptions, not advisory suppressions.

**Suggested founder wording — replace or personally accept:**

```text
cargo-deny: ACCEPT. The configured gate exits 0 with no advisory, license,
ban, or source warning/error, and advisories.ignore is empty. I reviewed the
two duplicate-version skips for hashbrown@0.16.1 and syn@2.0.119; they remain
documented temporary transitive-version exceptions and suppress no advisory.
The reviewed CORE-004 patch changes neither Cargo.lock nor deny.toml.
```

**Founder cargo-deny disposition:** `[FOUNDER TO AUTHOR]`

## F. Cross-model and independent-human review

- [ ] I verified the Claude bundle SHA-256 recorded above.
- [ ] I read the complete Claude disposition and resolved every requested
  confirmation or finding: `[FOUNDER TO CITE DISPOSITION FILE]`.
- [ ] I read the independent-human review completed by a reviewer who did not
  author the T0 implementation: `[FOUNDER TO CITE REVIEW FILE/URL]`.
- [ ] Any blocking finding from either reviewer has been corrected on a new
  exact commit and all invalidated gates were repeated.

## G. Founder decision

Select exactly one after every preceding gate is complete:

- [ ] **APPROVE** — accept CORE-004 for human PR review and merge.
- [ ] **APPROVE WITH RECORDED NON-BLOCKING FOLLOW-UP** — describe below.
- [ ] **CHANGES REQUIRED** — do not merge; repeat affected gates on the
  replacement exact commit.
- [ ] **REJECT** — the enforcement or evidence is unacceptable.

**Findings, exceptions, or follow-up:** `[FOUNDER TO AUTHOR]`

## Final attestation

Replace this block with the founder's own words after all gates complete.
Suggested structure:

```text
I personally reviewed the founder-authored CORE-004 T0 implementation, the
complete baseline-to-commit patch, active adversarial and regression tests,
the independent-human review, the cross-model disposition, and all three raw
scanner outputs at commit 60febb08ac9c3e207d6f7a3563b6824374c5c93e.
I verified patch SHA-256
71f051e24055cb0febd620d84a2703ea43d7277f5af4266feef8d03d0fbb9f1f.
My selected decision above is [APPROVE / APPROVE WITH FOLLOW-UP / CHANGES
REQUIRED / REJECT]. The bounded claim remains the single-guard, local-store,
modeled-clock timeout-only 6-A isolation contract.
```

- **Founder name:** `Khazretgali Sapen`
- **Founder signature/approval reference:** `[FOUNDER TO AUTHOR — PR URL or signed repository record]`
- **Decision timestamp (UTC):** `[FOUNDER TO AUTHOR]`
