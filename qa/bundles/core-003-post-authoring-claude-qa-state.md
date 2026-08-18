# CORE-003 post-authoring Claude QA state

**Snapshot date:** 2026-08-18
**Review subject:** uncommitted founder-authored working tree
**Authoring branch:** `codex/core-003-t0-founder-authoring`
**Baseline commit:** `4c7f6a33a5f0c01c42eed81b936a77450c8edd40`
**Complete patch SHA-256:** `d689cfe6fc092b7cac1fcfea397e09288a169aa2be14c94583cff57eedc905d9`

## Authored change

The founder-authored `BeforeToolCallAdapter::before_tool_call` boundary now:

- contains Rust unwinding panics raised by `GuardDecisionPort::decide`;
- converts both a typed `Err(GuardFault)` and a caught unwind to
  `BeforeToolCallObservation::Blocked` with
  `RequiredOutcome::FailClosed`;
- issues no authorization and does not invoke the effectful probe on either
  failure path;
- preserves the established relay behavior for returned `Ok(Deny | Allow)`;
- discards the panic payload; and
- states the bounded `AssertUnwindSafe`, unwind-only, hook-reached, and
  post-panic store-reuse limits.

The ownership map now records this method as founder-only T0 behavior. The two
dedicated ATK-07 tests are active, and the obsolete generic ATK-07 macro test
has been removed rather than incorrectly activated.

## Changed files

- `AGENTS.md`
- `CLAUDE.md`
- `T0-AUTHORS.md`
- `tests/bypass-rust/T0-BOUNDARY.md`
- `tests/bypass-rust/src/before_tool_call.rs`
- `tests/bypass-rust/tests/adapter_harness.rs`
- `tests/bypass-rust/tests/attack_set.rs`

## Local verification

- `cargo fmt --check`: pass
- `cargo clippy --locked --all-targets -- -D warnings`: pass
- `cargo test --locked --all-targets`: pass — 39 passed, 6 intentionally
  ignored
- dedicated typed-fault ATK-07 case: pass and active
- dedicated caught-panic ATK-07 case: pass and active
- `node scripts/check-structure.mjs`: pass
- `git diff --check`: pass

The raw command transcript is included as `VALIDATION.txt`. Claude must verify
the implementation and tests independently rather than treating this summary
as proof.

## Scope and evidence limits

This isolation proof covers a reached boundary whose guard returns a typed
fault or raises a Rust unwinding panic. It does not prove behavior for
`panic=abort`, process termination, OOM abort, a hook that never fires, a route
around the hook, a missing plugin, or operator bypass. Those runtime-integration
threats remain with RUNTIME-003/004.

This snapshot is not yet a commit and is not final SAST or approval evidence.
After review corrections, the founder must commit the exact accepted tree and
run Semgrep, CodeQL, and cargo-deny against that full commit. Human review,
cross-model disposition, and founder approval must also identify the exact
commit before merge.
