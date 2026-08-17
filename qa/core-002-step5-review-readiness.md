# CORE-002 Step 5 review readiness

**Prepared:** 2026-08-17

**Branch:** `codex/core-002-step5-single-use`

**Base:** `origin/main` at `d2d066720688583991f857010a4ab89ed29ad984`

## Review disposition

The implementation merged through PR #68. The independent-human, cross-model,
adversarial, and three-engine SAST/SCA reviews and founder finding dispositions
were recorded through PR #70. The protected checklist accurately records that
final checklist sign-off occurred after PR #68 merged, so the pre-merge timing
item remains unchecked.

On 2026-08-17, the founder confirmed that merging PR #70 was intended to
formally accept the post-merge remediation. The durable decision and its limits
are recorded in `qa/core-002-step5-governance-disposition.md`. This acceptance
closes the Step 5 process nonconformance for backlog-status purposes without
rewriting its history or weakening future T0 review gates.

The cross-model review covered T0 commit
`0727e327631b475990ef8d9b7ef3b2c3554050a8`. The final scans covered descendant
`0a54d4995d1b9d98ab8a3ec61861fe2fe7ae29c3`. The intervening commits add only
QA review records, the cargo-deny policy, and package-license metadata. No Rust
file changed between the reviewed T0 commit and the scanned commit.

The founder-owned implementation has one reachable allow path, after signature,
temporal, typed request-binding, and atomic single-use checks. An already-
consumed nonce denies. Store or internal faults become explicit fail-closed
denials before the adapter can invoke the effectful probe.

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

## Three-engine evidence

The founder-approved canonical label is **“three-engine SAST/SCA gate:
Semgrep, CodeQL, and cargo-deny.”** cargo-deny is the third
dependency/supply-chain analyzer; it is not represented as a third overlapping
first-party pattern scanner. Raw files, commands, versions, commit binding, and
digests are indexed in `qa/sast/README.md`.

| Engine | Execution and coverage | Findings | Current gate state |
|---|---|---|---|
| Semgrep 1.173.0, `p/rust` | completed; 14/14 Rust files; 0 scan errors | 1 INFO finding: test-only `std::env::temp_dir()` | accepted as test-only by founder |
| CodeQL 2.25.5, `codeql/rust-queries@0.1.35` | create/analyze exit 0; 14/14; 0 extraction/execution errors | 7 hard-coded cryptographic-value findings in deterministic fixtures | accepted as deterministic fixtures by founder |
| cargo-deny 0.20.2 | exit 0 using committed `deny.toml` | 0 blocking diagnostics; 2 ban notes; 54 accepted-license notes | pass; temporary duplicate-version exceptions accepted by founder |

The first unconfigured cargo-deny run against `0727e327...` is retained under
`qa/sast/pre-policy-0727e327/`. It failed closed on license policy and motivated
the committed, reviewable policy; it is not substituted for the final run.

## Local verification

From `tests/bypass-rust` on the policy descendant:

```text
cargo test --all-targets --all-features
  37 passed; 0 failed; 7 ignored

cargo clippy --all-targets --all-features -- -D warnings
  PASS

cargo fmt --all -- --check
  PASS

cargo deny --manifest-path tests/bypass-rust/Cargo.toml \
  --config deny.toml --locked --format human --color never check --show-stats
  PASS (0 errors; 0 warnings)
```

The seven ignored cases are explicit deferrals, not Step 5 coverage:
ATK-04/05/06/07/12/14 remain later-control cases and ATK-15 remains an external
hosted-IAM assertion.

## Closure actions

1. Review and merge the durable governance-disposition record without changing
   the protected checklist's historical process-nonconformance entry.
2. Reconcile `CORE-002-STEP5` to Done in the canonical backlog with links to
   PR #68, PR #70, and the merged governance disposition.
3. Keep parent `CORE-002` In Progress and retain all deferred attack and S3
   scope classifications.

## Known non-blocking scope limits

- This repository contains a developer-grade conformance harness, not the
  deployed DGR gate.
- Separate local database files can each consume the same nonce once; S3 must
  supply cross-instance coordination.
- Analyzer findings require human adjudication; a successful tool process or
  zero blocking diagnostics is not self-certifying.
