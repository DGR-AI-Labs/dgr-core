# CORE-002 Step 5 review readiness

**Prepared:** 2026-08-16  
**Branch:** `codex/core-002-step5-single-use`  
**Base:** `origin/main` at `d2d066720688583991f857010a4ab89ed29ad984`

## Review disposition

The Step 5 implementation has passed formatting and cross-model correctness
review. It remains in the T0 review gate and is not merge-ready until the
three-tool SAST, founder/human review, PR approval, and final sign-off evidence
is attached.

The cross-model disposition is recorded in
`qa/core-002-step5-cross-model-review.md` against the exact reviewed commit and
bundle digest.

The founder-owned implementation now has one reachable allow path, after
signature, temporal, typed request-binding, and atomic single-use checks. An
already-consumed nonce denies. Store or internal faults are converted to an
explicit fail-closed denial before the adapter can invoke the effectful probe.

## S2 scope

S2 is durable-local. `open_in_memory()` proves single-use behavior within one
live store instance. `open_at(path)` plus the restart regression proves that a
consumed nonce remains consumed when a new connection reopens the same SQLite
file. Separate database files are not coordinated; distributed replay
protection remains deferred S3 scope.

Any production construction that claims restart durability must use
`S2ConsumptionStore::open_at` with a stable local path. The conformance harness
does not claim that production wiring already exists.

## Evidence map

| Requirement | Review location | Executable evidence |
|---|---|---|
| Consume outcome is closed and three-way | `tests/bypass-rust/src/founder_consumption_store.rs` | default store faults closed |
| Same store retains live consumption | `tests/bypass-rust/src/founder_s2_consumption_store.rs` | `atk_03_replayed_token` |
| Restart durability uses the same file | `S2ConsumptionStore::open_at` | `file_backed_consumption_survives_connection_restart` |
| Concurrent presentation cannot permit twice | SQLite primary-key insert | `concurrent_presentations_cannot_both_consume` |
| Only PK/UNIQUE means replay | extended SQLite result-code match | store tests plus code review |
| Every other store error faults | final `Err(_)` match arm | `atk_13_audit_append_failure` |
| Persist precedes allow | consume match is the final guard check | ATK-03 first/second observations |
| All declared guard faults deny explicitly | `fail_closed_decision` | `every_guard_fault_maps_to_an_explicit_fail_closed_denial` |
| Deny/fault never invokes the tool | adapter relay | adapter harness and ATK-03/13 invocation counts |

## Local verification

From `tests/bypass-rust`:

```text
cargo test --all-targets --all-features
  37 passed; 0 failed; 7 ignored

cargo clippy --all-targets --all-features -- -D warnings
  PASS

cargo audit --no-fetch
  PASS (no advisories reported for the locked dependency graph)

cargo fmt --all -- --check
  PASS
```

The seven ignored cases are explicit deferrals, not Step 5 coverage:
ATK-04/05/06/07/12/14 remain later-control cases and ATK-15 remains an external
hosted-IAM assertion.

## Required reviewer actions

1. Review every founder-owned line against
   `specs/CORE-002-guard-review-checklist.md` and attach file-and-line findings.
2. Run and attach three independent SAST results. Clippy alone does not close
   this gate.
3. Confirm no test expectation was weakened and that every ignored case is
   still accurately classified.
4. Record founder/human approval in the protected review checklist, then merge
   through the normal PR path.

## Known non-blocking scope limits

- This repository contains a developer-grade conformance harness, not the
  deployed DGR gate.
- Separate local database files can each consume the same nonce once; S3 must
  supply cross-instance coordination.
- Dependency auditing is recorded separately from the constitution's required
  three independent SAST tools.
