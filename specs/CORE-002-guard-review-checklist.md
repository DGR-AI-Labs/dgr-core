# CORE-002 founder guard review checklist

**Purpose:** review criteria for the founder-authored T0 guard after it exists.
This document is not implementation guidance and does not authorize an agent
to modify the guard.

**Scope:** OpenClaw `before_tool_call`, direct effectful action without a valid
capability token (ATK-01), and the fail-closed properties on which that result
depends.

## Authorship and scope gate

- [ ] The founder authored every changed T0 body listed in `T0-AUTHORS.md`.
- [ ] An assisting agent reviewed only; it did not generate, complete,
      refactor, or patch T0 code.
- [ ] No decision, token-verification, fail-closed, or audit logic was moved
      into the adapter, fixtures, or tests.
- [ ] The implementation remains developer-grade and operator-bypassable; no
      stronger security or production claim is made.
- [ ] Enforcement logic exists only in the five founder-authored units:
      token verification, guard decision, fail-closed mapping, S2 consumption
      store, and the `ConsumptionStore` interface.

## ATK-01 denial behavior

- [ ] An effectful call with no capability token emits an explicit block/deny
      signal.
- [ ] The same request produces no authorization signal or capability.
- [ ] The tool probe records zero invocations before and after the denial.
- [ ] The active ATK-01 conformance test passes for the founder implementation,
      without weakening its expected denial signal or observation checks.

## Token rejection review

- [ ] Missing token denies.
- [ ] Invalid or unverifiable signature denies.
- [ ] Expired token denies.
- [ ] Forged or altered token denies.
- [ ] Replayed token denies.
- [ ] A valid token presented outside its action, parameter, principal,
      request, or tenant scope denies.
- [ ] Expiry uses the settled 300-second lifetime and 30-second skew tolerance.
- [ ] The guard recomputes a typed per-tool action binding; it does not accept
      a generic request record or a client-supplied hash.
- [ ] No fixture label, caller assertion, or unverified token field is treated
      as proof of validity.

## Consumption and single-use review

- [ ] The S2 store performs durable-local, atomic single-use consumption.
- [ ] Consumption is durable before allow can be returned.
- [ ] An already-consumed authorization denies.
- [ ] S3 remains deferred; only the `ConsumptionStore` interface boundary is
      retained for the hosted tier.

## Fail-closed and ordering review

- [ ] Guard absence or unavailability denies and emits a denial signal; it
      never produces allow.
- [ ] Token-verification error denies.
- [ ] Internal exception, timeout, malformed input, and unknown state deny.
- [ ] No catch/default/fallback path converts an error or unknown state to
      allow.
- [ ] No effectful tool action occurs before a final decision.
- [ ] A candidate allow is durably recorded before tool execution. If the
      required record cannot be written, the call denies (ATK-13).
- [ ] The audit/hash-chain or ledger-write path, if added, is founder-authored
      T0 and listed in `T0-AUTHORS.md`.

## Allow-path review

- [ ] Allow requires a verified capability bound to the exact action and
      context under review.
- [ ] The adapter proceeds only for an explicit allow returned by the
      founder-authored guard.
- [ ] The observed authorization reference corresponds to the decision record
      created before execution.
- [ ] A valid-token fixture is test data only; its label does not bypass real
      verification.

## Required evidence before merge

- [ ] `cargo build --manifest-path tests/bypass-rust/Cargo.toml` succeeds.
- [ ] The complete adversarial suite result is attached and reviewed.
- [ ] Cross-model review findings are attached and resolved or accepted by the
      founder.
- [ ] Results from at least three SAST tools are attached and reviewed.
- [ ] Human review confirms no test was weakened and no ignored case was
      silently represented as coverage.
- [ ] The founder records the review decision and signs off before merge.

## Review record

| Evidence | Reviewer / location | Result |
|---|---|---|
| Founder authorship confirmation | `{FOUNDER-SUPPLY}` | Pending |
| Human code review | `{FOUNDER-SUPPLY}` | Pending |
| Adversarial test | `{FOUNDER-SUPPLY}` | Pending |
| Cross-model review | `{FOUNDER-SUPPLY}` | Pending |
| SAST 1 | `{FOUNDER-SUPPLY}` | Pending |
| SAST 2 | `{FOUNDER-SUPPLY}` | Pending |
| SAST 3 | `{FOUNDER-SUPPLY}` | Pending |
