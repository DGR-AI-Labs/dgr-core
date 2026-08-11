# ARCH-004 clean Rust scaffold report

**Date:** 2026-08-10

**Branch:** `feat/core-002-scaffold-clean`

**Base:** GitHub `main` at `da6e39e291dcf689ec294463d7e3be8116eeb756`

## Result

The branch reconstructs the reviewed CORE-001 attack registry and Rust
CORE-002 harness as fresh content on Phase-1 `main`. No commit was cherry-picked.
The branch contains no TypeScript file or dependency on PR #2's TypeScript T0
modules. The five enforcement units remain founder-only, nonfunctional
`unimplemented!("FounderImplementationRequired")` stubs.

## Branch ancestry

The branch point and current GitHub `main` were identical when reconstruction
started:

```text
$ git merge-base HEAD origin/main
da6e39e291dcf689ec294463d7e3be8116eeb756
$ git rev-parse origin/main
da6e39e291dcf689ec294463d7e3be8116eeb756
```

PR #2's TypeScript head is not an ancestor:

```text
$ git merge-base --is-ancestor aec599fe7b0efe6663129f00a6f33061610d0e73 HEAD
$ echo $?
1
```

Exit 1 is the expected proof that the old head is **not** an ancestor. At the
scaffold commit, fresh branch history was:

```text
$ git log --oneline origin/main..HEAD
9f5f7ea test(core): reconstruct clean Rust bypass scaffold
```

The only subsequent commit is this evidence report. The PR body records the
post-push log, including both fresh reconstruction commits.

## File inventory

Branch additions before this report:

```text
T0-AUTHORS.md
specs/CORE-001-bypass-attack-set.md
specs/CORE-002-guard-review-checklist.md
tests/bypass-rust/.gitignore
tests/bypass-rust/Cargo.lock
tests/bypass-rust/Cargo.toml
tests/bypass-rust/T0-BOUNDARY.md
tests/bypass-rust/src/before_tool_call.rs
tests/bypass-rust/src/fixtures.rs
tests/bypass-rust/src/founder_authored_guard.rs
tests/bypass-rust/src/founder_consumption_store.rs
tests/bypass-rust/src/founder_fail_closed.rs
tests/bypass-rust/src/founder_s2_consumption_store.rs
tests/bypass-rust/src/founder_token_verification.rs
tests/bypass-rust/src/lib.rs
tests/bypass-rust/tests/adapter_harness.rs
tests/bypass-rust/tests/attack_set.rs
```

The branch delta contains no `.ts` or `.tsx` file. Build output under
`tests/bypass-rust/target/` is ignored and untracked.

## No TypeScript T0 path

The following scan over the added source, specifications, and ownership map
returned no matches:

```text
$ rg -n 'Record<string[ ,]*unknown>|30s/5s|30s|5s|packages/core|capability-token\.ts|@dgr/core|@dgr/openclaw' T0-AUTHORS.md specs tests/bypass-rust --glob '!target/**'
<no output>
```

The only referenced timing values are the settled 300-second lifetime and
30-second skew tolerance in the review checklist. Binding is described as a
typed per-tool action binding; no generic TypeScript record binding is present.

## Five founder-only units

Exactly five `founder_*.rs` files exist and exactly five `unimplemented!()`
invocations occur across them:

```text
founder_authored_guard.rs
founder_consumption_store.rs
founder_fail_closed.rs
founder_s2_consumption_store.rs
founder_token_verification.rs
```

They contain signatures, types needed to compile those signatures, ownership
headers, and nonfunctional defaults only. No token verification, guard
decision, fail-closed mapping, persistence, replay, hashing, or authorization
logic is implemented.

## Build and deliberately red test evidence

```text
$ cargo fmt --manifest-path tests/bypass-rust/Cargo.toml -- --check
PASS
$ cargo build --manifest-path tests/bypass-rust/Cargo.toml
PASS
$ cargo test --manifest-path tests/bypass-rust/Cargo.toml --no-run
PASS
$ cargo test --manifest-path tests/bypass-rust/Cargo.toml
EXPECTED RED (exit 101)
```

Test results:

- Adapter harness: 3 passed.
- Attack registry/harness: 3 passed, 1 failed, 14 ignored.
- ATK-01 is the sole active red conformance test. It stops at
  `FounderImplementationRequired` in the founder guard stub.
- `unimplemented_guard_cannot_reach_the_effectful_probe` passes and confirms
  zero effectful invocations while the founder stub remains unimplemented.
- ATK-02 through ATK-15 remain ignored.

The red result is intentional and must remain red until the founder authors the
T0 guard through the required process. No test was weakened and no required
status check was enabled.
