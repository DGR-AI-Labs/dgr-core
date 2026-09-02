# PROD-000 remediation exact-input validation

## Commit binding

- Replacement/remediation Rust source commit: `b19f33ae16698a81b993e6cc5a751360b6109577`.
- Replacement source tree: `12fadecee74b6387095977980fb215ddc7fe3c1c`.
- T3 active-assertion guard commit: `587585cf476431f078efe587c5dbcc052389cdad`.
- T3 guard tree: `89e5de51d23ead98d24bbbe1b4cd57db343b2dc4`.
- Baseline: `e9c8f585809c15d2464b3d45bc2ce26d716c8673`.

The three-engine scan ran against the replacement Rust source commit. The subsequent guard commit
changes only `scripts/check-ignored-attacks.mjs` and `scripts/check-ignored-attacks.test.mjs`; no
Rust source, Cargo input, dependency policy, or workflow changed. Those two JavaScript files were
then validated at the guard commit together with the unchanged Rust tree.

## Functional and policy gate

| Command | Result |
|---|---|
| `node --check scripts/check-ignored-attacks.mjs` | PASS |
| `node --check scripts/check-ignored-attacks.test.mjs` | PASS |
| `node scripts/check-ignored-attacks.test.mjs` | PASS: 7/7 |
| `node scripts/check-ignored-attacks.mjs` | PASS: exact five-member ignored set; required ATK-06 equality test present and active |
| `npm run check:structure` | PASS: 18 governance files |
| `cargo fmt --manifest-path tests/bypass-rust/Cargo.toml --all -- --check` | PASS |
| `cargo clippy --manifest-path tests/bypass-rust/Cargo.toml --all-targets --locked -- -D warnings` | PASS |
| `cargo test --manifest-path tests/bypass-rust/Cargo.toml --all-targets --locked` | PASS: 52 passed, 5 ignored, 0 failed |

## Fresh three-engine evidence

| Engine | Exact scanned input | Result |
|---|---|---|
| Semgrep 1.173.0 / `p/rust` | `b19f33a...`, all 21 tracked Rust files | COMPLETE: one existing test-only temp-dir finding, zero scan errors; exit 1 under `--error` |
| CodeQL 2.25.5 / `codeql/rust-queries@0.1.35` | `b19f33a...`, all 21 tracked Rust files | COMPLETE: nine deterministic fixture-nonce findings; no error-level notification; one retained generated `bindgen.rs` warning |
| cargo-deny 0.20.2 | `b19f33a...`, unchanged lock/policy | PASS: zero errors and warnings; two bans notes and 54 license notes |

No scanner finding touches the replacement floor, adapter, R5.1 timeout change, or the T3 guard.
This observation is not a disposition. Every finding and the complete CodeQL diagnostic array remain
for independent-human and founder review.

## Baseline identity evidence

The following hashes are identical at the baseline and replacement commits:

| File | SHA-256 |
|---|---|
| `.github/workflows/ci.yml` | `8e1619e0f2120cf1086d8095e65b8bb4d2c08bdc2125fbd7760917f542d25378` |
| `tests/bypass-rust/Cargo.toml` | `e161cbd225c19adb79ae5be7aa9c3169eb98807280b69462b0a74a874b82939e` |
| `tests/bypass-rust/Cargo.lock` | `59c1a398c6a3405a6a895da2e794b7ed7cb0d6970ac74840cbd698c40b0f39ad` |
| `deny.toml` | `8b1fbe5dece4e19de0ed231a4dde2a41c7d258a2bd24ebe6930929fa6b751ad8` |
| `tests/bypass-rust/tests/attack_set.rs` | `4ec860d8651402e4a1324ed78ea0f41a3071e3564a720253fddf7a0d51725a0d` |
| `rust-toolchain.toml` | `d52c5633ea77aefd345519d0a6c87e19c2636a1e90178585c30db481b3de9de0` |

`tests/bypass-rust/tests/core_004_conformance.rs` changed only by adding the T3 equality assertion
inside the pre-existing ATK-06 sequence test. Its current SHA-256 is
`29b3b06830d8d6db6cd07008575d9dfeca4dfb6767e7a476c3ed89df9e1cea09`.

## Gate state

The remediation is ready for a fresh non-author cross-model review. It is not merge-ready until:

1. Claude returns a satisfactory addendum against the supplied bundle;
2. a non-author human completes the independent-human review;
3. the founder personally reviews the final diff, every SAST/SCA result and diagnostic, both review
   records, and dispositions every open item; and
4. the founder approves the exact final PR head.

PROD-001 extraction remains unauthorized until those gates complete and PR #90 merges.
