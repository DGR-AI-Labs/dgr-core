# CORE-002 founder guard review checklist

**Purpose:** review criteria for the founder-authored T0 guard after it exists.
This document is not implementation guidance and does not authorize an agent
to modify the guard.

**Scope:** OpenClaw `before_tool_call`, direct effectful action without a valid
capability token (ATK-01), and the fail-closed properties on which that result
depends.

## Authorship and scope gate

- [x] The founder authored every changed T0 body listed in `T0-AUTHORS.md`.

- [x] An assisting agent reviewed only; it did not generate, complete,
      refactor, or patch T0 code.

- [x] No decision, token-verification, fail-closed, or audit logic was moved
      into the adapter, fixtures, or tests.

- [x] The implementation remains developer-grade and operator-bypassable; no
      stronger security or production claim is made.

- [x] Enforcement logic exists only in the five founder-authored units:
      token verification, guard decision, fail-closed mapping, S2 consumption
      store, and the `ConsumptionStore` interface.

## ATK-01 denial behavior

- [x] An effectful call with no capability token emits an explicit block/deny
      signal.

- [x] The same request produces no authorization signal or capability.

- [x] The tool probe records zero invocations before and after the denial.

- [x] The active ATK-01 conformance test passes for the founder implementation,
      without weakening its expected denial signal or observation checks.

## Token rejection review

- [x] Missing token denies.

- [x] Invalid or unverifiable signature denies.

- [x] Expired token denies.

- [x] Forged or altered token denies.

- [x] Replayed token denies.

- [x] A valid token presented outside its action, parameter, principal,
      request, or tenant scope denies.

- [x] Expiry uses the settled 300-second lifetime and 30-second skew tolerance.

- [x] The guard recomputes a typed per-tool action binding; it does not accept
      a generic request record or a client-supplied hash.

- [x] No fixture label, caller assertion, or unverified token field is treated
      as proof of validity.

## Consumption and single-use review

- [x] The S2 store performs durable-local, atomic single-use consumption.

- [x] Consumption is durable before allow can be returned.

- [x] An already-consumed authorization denies.

- [x] S3 remains deferred; only the `ConsumptionStore` interface boundary is
      retained for the hosted tier.

## Fail-closed and ordering review

- [x] Guard absence or unavailability denies and emits a denial signal; it
      never produces allow.

- [x] Token-verification error denies.

- [x] Internal exception, timeout, malformed input, and unknown state deny.

- [x] No catch/default/fallback path converts an error or unknown state to
      allow.

- [x] No effectful tool action occurs before a final decision.

- [x] A candidate allow is durably recorded before tool execution. If the
      required record cannot be written, the call denies (ATK-13).

- [x] The audit/hash-chain or ledger-write path, if added, is founder-authored
      T0 and listed in `T0-AUTHORS.md`.

## Allow-path review

- [x] Allow requires a verified capability bound to the exact action and
      context under review.

- [x] The adapter proceeds only for an explicit allow returned by the
      founder-authored guard.

- [x] The observed authorization reference corresponds to the decision record
      created before execution.

- [x] A valid-token fixture is test data only; its label does not bypass real
      verification.

## Required evidence before merge

- [x] `cargo build --manifest-path tests/bypass-rust/Cargo.toml` succeeds.

- [x] The complete adversarial suite result is attached and reviewed.

- [x] Cross-model review findings are attached and resolved or accepted by the
      founder.

- [x] Results from at least three SAST tools are attached and reviewed.

- [x] Human review confirms no test was weakened and no ignored case was
      silently represented as coverage.

- [ ] The founder records the review decision and signs off before merge.
  **Process nonconformance:** PR #68 merged at 2026-08-17T03:14:25Z before
  this checklist record was completed. Post-merge independent human review and
  founder disposition were completed on 2026-08-17; the required pre-merge timing
  was not satisfied.

## Review record

| Evidence                        | Reviewer / location                                                                                                            | Result                                                                        |
| ------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------- |
| Founder authorship confirmation | Founder — `T0-AUTHORS.md`; reviewed 2026-08-17                                                                                 | PASS                                                                          |
| Human code review               | Gaziz Nugmanov — reviewed PR #68 head `2712f0f815c5f132af5b2237c066b5e1b6b174b7`; five T0 units and affected tests; 2026-08-17 | PASS — no code findings; no test weakened; 7 ignored cases confirmed deferred |
| Adversarial test                | `qa/core-002-step5-review-readiness.md`                                                                                        | PASS — 37 passed, 7 explicitly deferred, 0 failed                             |
| Cross-model review              | `qa/core-002-step5-cross-model-review.md`; `qa/core-002-step5-claude-independent-qa-addendum.md`                               | PASS WITH ERRATA — findings resolved or accepted                              |
| SAST 1                          | `qa/sast/core-002-step5-semgrep-2026-08-17.txt`; founder disposition 2026-08-17                                                | PASS WITH ACCEPTED TEST-ONLY FINDING                                          |
| SAST 2                          | `qa/sast/core-002-step5-codeql-2026-08-17.sarif`; founder disposition 2026-08-17                                               | PASS WITH 7 ACCEPTED DETERMINISTIC FIXTURE FINDINGS                           |
| SAST 3                          | `qa/sast/core-002-step5-cargo-deny-2026-08-17.txt`; founder review 2026-08-17                                                  | PASS — 0 blocking diagnostics; notes reviewed                                 |

## Founder analyzer dispositions

### Semgrep — predictable temporary test directory

**Disposition (Founder, 2026-08-17): ACCEPTED — TEST ONLY.**

Accepted for this developer-only conformance test. The path contains the process
ID plus nanosecond time, stores no secrets, and is not used by production
construction. A malicious same-host process could race or pre-create the
predictable path, so this exception does not support a production-security
claim. Replace it with securely created temporary storage before reusing the
pattern outside isolated tests. No T0 path is affected.

### CodeQL — seven hard-coded cryptographic-value findings

**Disposition (Founder, 2026-08-17): ACCEPTED — DETERMINISTIC FIXTURES.**

The seven values are intentionally distinct, fixed 16-byte nonce inputs used by
the VAL-002 fixture suite. They are not production keys, passwords, salts,
initialization vectors, or runtime nonce-generation logic. This disposition is
limited to the seven findings recorded in
`qa/sast/core-002-step5-codeql-2026-08-17.sarif`.

### cargo-deny — duplicate-dependency exceptions

**Disposition (Founder, 2026-08-17): ACCEPTED — TEMPORARY POLICY EXCEPTIONS.**

The reviewed run against commit
`0a54d4995d1b9d98ab8a3ec61861fe2fe7ae29c3` reported zero advisory, license,
ban, and source errors or warnings. The `hashbrown@0.16.1` and `syn@2.0.119`
entries are version-pinned transitive duplicate-dependency exceptions, not
advisory waivers; `advisories.ignore = []`. Revisit them whenever `Cargo.lock`
or the advisory database changes, or when dependency convergence removes either
version split.

**Founder final decision (2026-08-17):** POST-MERGE REVIEW COMPLETE FOR CORE-002 STEP 5 IMPLEMENTATION PR #68.

I reviewed the independent human findings, adversarial evidence, cross-model
errata, all three raw analyzer outputs, and the recorded dispositions. The
reviewed PR head is `2712f0f815c5f132af5b2237c066b5e1b6b174b7`; the
SAST-scanned non-Rust ancestor is
`0a54d4995d1b9d98ab8a3ec61861fe2fe7ae29c3`; and the merge commit is
`ffe3a0fcb23f45d2d3b82e76df7e9bff44ff72e4`. No stronger claim than S2
durable-local developer-grade enforcement is made.
