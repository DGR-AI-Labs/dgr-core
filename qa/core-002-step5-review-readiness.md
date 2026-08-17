# CORE-002 Step 5 review readiness

**Prepared:** 2026-08-17

**Branch:** `codex/core-002-step5-single-use`

**Base:** `origin/main` at `d2d066720688583991f857010a4ab89ed29ad984`

## Review disposition

The implementation, cross-model review, adversarial tests, and three-engine raw
SAST/SCA evidence are present. Step 5 remains **In Review**, not merge-ready,
until the founder records dispositions for the eight analyzer findings, an
independent human reviews the founder-authored T0 code, and the founder signs
the protected checklist before the normal PR/merge path.

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

FND-7 selects Semgrep, CodeQL, and cargo-deny. cargo-deny is the third
dependency/supply-chain analyzer; it is not represented as a third overlapping
first-party pattern scanner. Raw files, commands, versions, commit binding, and
digests are indexed in `qa/sast/README.md`.

| Engine | Execution and coverage | Findings | Current gate state |
|---|---|---|---|
| Semgrep 1.173.0, `p/rust` | completed; 14/14 Rust files; 0 scan errors | 1 INFO finding: test-only `std::env::temp_dir()` | founder disposition pending |
| CodeQL 2.25.5, `codeql/rust-queries@0.1.35` | create/analyze exit 0; 14/14; 0 extraction/execution errors | 7 hard-coded cryptographic-value findings in deterministic fixtures | founder disposition pending |
| cargo-deny 0.20.2 | exit 0 using committed `deny.toml` | 0 blocking diagnostics; 2 ban notes; 54 accepted-license notes | raw result passes; founder review pending |

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

## Remaining reviewer actions

1. Founder: record a disposition for the Semgrep finding and one grouped
   disposition for the seven CodeQL fixture findings. Suggested review text is
   in `qa/core-002-step5-founder-review-input.md`.
2. Independent human reviewer: review every founder-owned line against
   `specs/CORE-002-guard-review-checklist.md`, confirm no test was weakened, and
   record identity/date/file-and-line evidence.
3. Founder: review the cargo-deny policy, its two version-pinned duplicate notes,
   and accepted-license set; then complete and sign the protected checklist.
4. Open the PR, obtain approval, and merge through the normal human path. Only
   then may the backlog item move from In Review to Done.

## Known non-blocking scope limits

- This repository contains a developer-grade conformance harness, not the
  deployed DGR gate.
- Separate local database files can each consume the same nonce once; S3 must
  supply cross-instance coordination.
- Analyzer findings require human adjudication; a successful tool process or
  zero blocking diagnostics is not self-certifying.
