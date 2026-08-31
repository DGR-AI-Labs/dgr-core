# PROD-000 T0 decoupling preparation

**Status:** READY FOR FOUNDER T0 AUTHORING; agent stopped before implementation

**Baseline commit:** `ca6493408c5bf0cdd58e2f234d12feed22b161c8`

**Prepared:** 2026-08-28

**Authority:** The founder approved ADR-13 Amendment A, R5.2a's library-outcome/semantic-identity
model, and authorship option (c). The canonical active records are pinned at dgr-internal source
commit `104dbe651a869f198f2c76a58d7b2682bb82fbd6`. This report does not authorize PROD-001 extraction.

## Scope and boundary

This report prepares the two founder-authored R5 changes without applying either one. No
`{FOUNDER-AUTHORS}` unit, enforcement expression, test, Cargo manifest, lockfile, `deny.toml`, or
CI workflow was changed. The proposed patches below are review material, not patches authored into
the working tree.

The baseline files are:

| File | SHA-256 |
|---|---|
| `tests/bypass-rust/src/founder_approval_timeout.rs` | `989ae1df102c1a78638b28354e1cbcbd37e7cc3565292703840b2e7222f2c9b7` |
| `tests/bypass-rust/src/before_tool_call.rs` | `5e44f9d6c4451bbe80c7821a6587663110b1144a6895a4dd2f8548d0e0de049d` |
| `tests/bypass-rust/tests/core_004_conformance.rs` | `7ff1b9a0bed1b16dc6745bad5643aaa209936a97d31ecba97445dda846a74221` |
| `.github/workflows/ci.yml` | `8e1619e0f2120cf1086d8095e65b8bb4d2c08bdc2125fbd7760917f542d25378` |
| `tests/bypass-rust/Cargo.toml` | `e161cbd225c19adb79ae5be7aa9c3169eb98807280b69462b0a74a874b82939e` |
| `tests/bypass-rust/Cargo.lock` | `59c1a398c6a3405a6a895da2e794b7ed7cb0d6970ac74840cbd698c40b0f39ad` |
| `deny.toml` | `8b1fbe5dece4e19de0ed231a4dde2a41c7d258a2bd24ebe6930929fa6b751ad8` |

## R5.1 — registry decoupling proposal

### Source-verified identity argument

The current registry row at `tests/bypass-rust/src/lib.rs:161-170` defines ATK-06 with
`RequiredOutcome::EscalateThenDenyOnTimeout`; the exact assignment is at line 166. The T0 timeout
path at `tests/bypass-rust/src/founder_approval_timeout.rs:47-50` currently resolves that same value
through `attack_by_id("ATK-06")`.

Therefore, at baseline `ca64934...`, the proposed founder-owned literal and the value returned by
the registry are equal. The behavior change is limited to ownership and dependency direction:

- before: T0 enforcement trusts a T3 registry lookup and fails closed if the row is absent;
- after: T0 enforcement owns `RequiredOutcome::EscalateThenDenyOnTimeout`, while a T3 test checks
  that the registry still matches it.

This removes the runtime missing-row branch. That is an enforcement-expression change and must be
authored and reviewed by the founder even though the current registered behavior is identical.

### Existing mirror precedent

The proposed private-constant/public-mirror shape follows:

- `MAXIMUM_LIFETIME_SECONDS` and `CONFORMANCE_MAXIMUM_LIFETIME_SECONDS` at
  `tests/bypass-rust/src/founder_authored_guard.rs:31-35`;
- `EXPIRY_SKEW_SECONDS` and `CONFORMANCE_EXPIRY_SKEW_SECONDS` at lines 32 and 37-38;
- `APPROVAL_WINDOW_SECONDS` and `CONFORMANCE_APPROVAL_WINDOW_SECONDS` at lines 41 and 47-48; and
- `CONFORMANCE_K2_KEY_ID` / `CONFORMANCE_K2_PUBLIC_KEY` at
  `tests/bypass-rust/src/founder_token_verification.rs:5-9`, mirroring the private registered key
  constants at lines 25-29.

### Exact founder-authored proposal — do not apply as agent

```diff
diff --git a/tests/bypass-rust/src/founder_approval_timeout.rs b/tests/bypass-rust/src/founder_approval_timeout.rs
--- a/tests/bypass-rust/src/founder_approval_timeout.rs
+++ b/tests/bypass-rust/src/founder_approval_timeout.rs
@@
 use crate::RequiredOutcome;
-use crate::attack_by_id;
 use crate::before_tool_call::{GuardDecision, GuardFault};
 use crate::founder_approval_store::{
     ApprovalStore, EvaluatePendingOutcome, PendingApproval, ReviewRequestId,
 };
 use crate::founder_fail_closed::fail_closed_decision;
 
+const ATK_06_TIMEOUT_OUTCOME: RequiredOutcome =
+    RequiredOutcome::EscalateThenDenyOnTimeout;
+
+#[doc(hidden)]
+pub const CONFORMANCE_ATK_06_TIMEOUT_OUTCOME: RequiredOutcome = ATK_06_TIMEOUT_OUTCOME;
+
 fn matching_pending(
@@
-            let outcome = match attack_by_id("ATK-06") {
-                Some(case) => case.expected,
-                None => return fail_closed_decision(GuardFault::InternalError),
-            };
-
             Ok(GuardDecision::Deny {
-                outcome,
+                outcome: ATK_06_TIMEOUT_OUTCOME,
                 denial_signal: "ATK-06 approval timed out",
             })
```

The enforcement expression that changes is exactly the timed-out arm's source of `outcome`:
`match attack_by_id("ATK-06")` becomes the founder-owned `ATK_06_TIMEOUT_OUTCOME`. No other match
arm, deadline comparison, store call, signal, or returned decision is proposed to change.

### Exact T3 test-side proposal — do not apply before the founder constant exists

The equality assertion is folded into the existing
`atk_06_sequence_is_escalated_then_registry_derived_timeout_block` test so the total remains 52
passed / 5 ignored. It adds no test and changes no expectation.

```diff
diff --git a/tests/bypass-rust/tests/core_004_conformance.rs b/tests/bypass-rust/tests/core_004_conformance.rs
--- a/tests/bypass-rust/tests/core_004_conformance.rs
+++ b/tests/bypass-rust/tests/core_004_conformance.rs
@@
-use dgr_core_bypass_harness::founder_approval_timeout::evaluate_approval_timeout;
+use dgr_core_bypass_harness::founder_approval_timeout::{
+    CONFORMANCE_ATK_06_TIMEOUT_OUTCOME, evaluate_approval_timeout,
+};
@@
     let case = attack_by_id(ATTACK_ID).expect("ATK-06 is registered");
     let expected_terminal = case.expected;
 
+    assert_eq!(CONFORMANCE_ATK_06_TIMEOUT_OUTCOME, expected_terminal);
     assert_eq!(
         expected_terminal,
         RequiredOutcome::EscalateThenDenyOnTimeout
```

The dependency is test → T0 mirror. A later registry change still fails this test. Applying this T3
diff before the founder creates the mirror would intentionally break compilation, so it remains
review material in this preparation branch.

## R5.2 — fail-closed floor relocation

### Current ownership and byte evidence

The mixed file currently contains:

- library-bound candidates at `tests/bypass-rust/src/before_tool_call.rs:13-60`:
  `OpaqueCapabilityToken`, `BeforeToolCallRequest`, `GuardDecision`, `GuardFault`, and
  `GuardDecisionPort`;
- harness-only `EffectfulToolProbe` at lines 62-66;
- harness-only `BeforeToolCallObservation` at lines 68-93;
- harness `BeforeToolCallAdapter` at lines 95-172; and
- the founder-owned floor inside `BeforeToolCallAdapter::before_tool_call` at lines 115-171.

Hashes of the exact baseline regions are:

| Region | SHA-256 |
|---|---|
| Complete `before_tool_call` method, lines 115-171 | `35639394ea9d089f86b31d103027353e99e3e55522db8bf57112526d3ccfacc7` |
| Floor comment, `catch_unwind`, and match, lines 126-170 | `d9f2b23fb9a44422d38d8231b530ea0d739244897a9f147234e65608a1793c3f` |

The following baseline block carries the required unwind-only bound, bounded
`AssertUnwindSafe` reasoning, dropped error/panic payload, and fail-closed construction:

```rust
/// This containment covers Rust unwinding panics after the boundary is reached.
/// It does not cover `panic=abort`, process termination, OOM abort, or a hook
/// that is never invoked.
// `AssertUnwindSafe` is deliberately bounded to this invocation. If `decide`
// unwinds, this method does not inspect or reuse either store and returns
// fail-closed immediately. This does not certify either store's invariants
// for reuse by a later invocation.
let decision = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
    self.guard
        .decide(request, now_unix_seconds, consumption_store, approval_store)
}));

// ... unchanged Ok(Deny), Ok(Escalate), and Ok(Allow) relay arms ...

Ok(Err(_)) | Err(_) => BeforeToolCallObservation::Blocked {
    outcome: RequiredOutcome::FailClosed,
    denial_signal: "CORE-003 boundary fail-closed",
    authorization_issued: false,
    effectful_invocations: tool.invocation_count(),
},
```

### Proposed physical split common to either ownership decision

The intended new T0 home before extraction is
`tests/bypass-rust/src/founder_before_tool_call_floor.rs`. It would later move as one founder-owned
file into the root `dgr-core` library. The existing `before_tool_call.rs` would remain T3 and retain
the probe, recording observation, and adapter.

The new founder file would own:

- `OpaqueCapabilityToken`;
- `BeforeToolCallRequest`;
- `GuardDecision`;
- `GuardFault`;
- `GuardDecisionPort`; and
- a public floor function containing the unwind boundary.

The harness file would import those five types and the floor function, then map its result to
`BeforeToolCallObservation` and invoke `EffectfulToolProbe` only for the returned allow case. The
two existing ATK-07 tests at `tests/bypass-rust/tests/attack_set.rs:251-287` must remain textually
unchanged and pass through that delegation.

### Resolved observation-type finding

The preparation identified that no exact R5.2 patch could satisfy all original requirements without
a founder ownership decision:

1. A Rust library cannot construct a type owned by a downstream harness. If
   `BeforeToolCallObservation` remains harness-bound, the library floor cannot return
   `BeforeToolCallObservation::Blocked` without creating the forbidden library → harness
   dependency.
2. A library-side equivalent solves the dependency direction, but necessarily changes the
   constructor from `BeforeToolCallObservation::Blocked` to a new library-owned variant. The
   harness must then convert it, so the existing `Blocked { ... }` construction is not
   byte-identical in its new home.
3. Moving `BeforeToolCallObservation` into the library preserves the constructor name more closely
   but contradicts Amendment A R5.2's explicit decision that observation is a harness/test concept.
   It still leaves `EffectfulToolProbe` and `tool.invocation_count()` coupled to the current method.
4. A generic callback or conversion trait would let the library ask the harness to construct the
   observation, but then the fail-closed construction remains harness-authored rather than inside
   the library floor.
5. Returning a fail-closed `GuardDecision::Deny` is the cleanest library boundary, but it changes
   the enforcement expression and makes identity semantic/test-proven rather than byte-identical.

The founder resolved the question through Amendment A R5.2a:

- `BeforeToolCallOutcome::{Blocked, Escalated, Authorized}` is library-owned;
- `BeforeToolCallObservation` and invocation/authorization counters remain T3;
- the harness invokes its probe only after `Authorized`;
- identity is semantic because the constructor necessarily changes, with the `catch_unwind`
  boundary, bounded `AssertUnwindSafe` reasoning, dropped payloads, unwind-only limitation,
  `FailClosed` value, and denial signal retained as explicit proof obligations; and
- the unchanged ATK-07 tests and complete active/ignored sets provide behavioral evidence.

### Authorship option (c) handoff

The binding boundary is now:

1. **Founder first:** author the complete new T0 boundary module, including the product outcome,
   relocated floor, R5.1 constant/control-flow change, and import-only edits inside founder-owned
   files. Commit that work without an agent-authored T0 stub.
2. **Agent second:** after the founder commit exists, author only the T3 facade/re-exports, adapter
   conversion and probe invocation, folded registry-mirror assertion, and ownership documentation.
3. **Exact-commit gate:** run the unchanged conformance sets, both required contexts, fresh
   Semgrep/CodeQL/cargo-deny, cross-model review, independent-human review, and founder disposition
   against the final combined commit before human merge.

This report intentionally does not add a ready-to-paste R5.2 T0 implementation. The founder should
relocate the existing floor from the founder's own source and author the new consequential outcome
surface directly. The agent remains stopped until that founder-authored module is present.

Founder authoring and evidence should be recorded in these five documentation-only templates:

1. `qa/prod-000-authoring/01-founder-boundary-module.md`
2. `qa/prod-000-authoring/02-floor-relocation-semantic-identity.md`
3. `qa/prod-000-authoring/03-old-floor-removal-and-handoff.md`
4. `qa/prod-000-authoring/04-module-registration-and-dependency-boundary.md`
5. `qa/prod-000-authoring/05-founder-import-rewrite-ledger.md`

The templates contain no implementation body or temporary T0 stub. They become exact-commit
evidence only after the founder fills and signs them against the authored commit.

## Untouched baseline verification

Executed at `ca6493408c5bf0cdd58e2f234d12feed22b161c8` before preparing this report:

| Check | Result |
|---|---|
| `npm run check:structure` | PASS — 18 governance files |
| `cargo fmt --manifest-path tests/bypass-rust/Cargo.toml --all -- --check` | PASS |
| `cargo build --manifest-path tests/bypass-rust/Cargo.toml --all-targets --locked` | PASS |
| `cargo clippy --manifest-path tests/bypass-rust/Cargo.toml --all-targets --locked -- -D warnings` | PASS |
| `cargo test --manifest-path tests/bypass-rust/Cargo.toml --all-targets --locked` | PASS — 52 passed / 5 ignored |
| `node scripts/check-ignored-attacks.test.mjs` | PASS — 4/4 |
| `node scripts/check-ignored-attacks.mjs` | PASS — exactly ATK-04/05/12/14/15 |

The required CI context names configured at `.github/workflows/ci.yml:16` and line 27 are exactly:

- `Structural / governance check`
- `Rust format / build / test`

PROD-000 must not rename either context. Informational Semgrep, CodeQL, and cargo-deny jobs remain
non-blocking and are not substitutes for the fresh exact-commit T0 gate required after founder
authoring.

Neither proposed decoupling requires a `Cargo.toml`, `Cargo.lock`, `deny.toml`, or CI-workflow
change. This preparation report changes none of them.

## Founder gate

STOP. The founder must:

1. merge the active ADR-13 and Amendment A documentation branches through human review;
2. author the R5.1 T0 patch and complete R5.2 T0 module under option (c);
3. hand the exact founder commit back for the T3-only follow-up;
4. bind fresh three-engine SAST, cross-model review, independent-human review, byte-level founder
   review, and the unchanged conformance suite to the exact resulting commit; and
5. merge PROD-000 before PROD-001 extraction begins.

No extraction, runtime work, enforcement claim expansion, or T0 code change is authorized by this
report.
