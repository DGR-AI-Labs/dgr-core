# PROD-001 core-extraction review input

**Status:** implementation complete; independent and founder gates pending

**Base commit:** `8318f61eadf689f9b8a72f673cc68cd083dc7831`

**Base tree:** `227949bfdb0988fa465a1fc1d540c2a447daa0f7`

**Implementation commit:** `91589759964f2a409960c6a21a5d16795f1d95a1`

**Implementation tree:** `3c1295c4ba011eb660f4a92cddf1f8b06a0b7a2d`

This is evidence for review, not approval. It records an agent-assisted structural extraction under
ADR-13 and Amendments A and B. The founder must independently verify the byte-identity proof,
dispose of every analyzer result, approve the final reviewed head, and perform the merge.

## 1. Bounded result

- Root package `dgr-core` is version `0.1.0`, has `publish = false`, and is consumable as a Git
  dependency without publishing to crates.io.
- `tests/bypass-rust` remains the conformance package and consumes root `dgr-core` through its
  workspace path dependency.
- The repository has one Cargo workspace and one tracked `Cargo.lock` at the root.
- The nine reviewed enforcement files moved from `tests/bypass-rust/src/` to `src/` without byte
  changes.
- The mixed-crate definitions `RequiredOutcome`, `ProposedAction`, and `DecisionContext` moved
  byte-identically into root `src/lib.rs`; the harness re-exports them and the enforcement modules
  so existing conformance imports remain unchanged.
- The registry, fixtures, adapter, observations, probes, and all test sources remain under
  `tests/bypass-rust` as T3.
- No enforcement-body, conformance expectation, test source, attack classification, or ignored set
  changed.

## 2. Whole-file byte identity

Each row is the SHA-256 of both the base path at the base commit and the destination path at the
implementation commit. `git diff-tree -r -M100%` independently classifies every row as `R100`.

| Base path | Destination path | Before and after SHA-256 |
|---|---|---|
| `tests/bypass-rust/src/founder_approval_store.rs` | `src/founder_approval_store.rs` | `92b4bc4716725569e9dbf3834d9fd6d72128918f0937a31a9e7bfd5282dde7ec` |
| `tests/bypass-rust/src/founder_approval_timeout.rs` | `src/founder_approval_timeout.rs` | `14cb21fbb11ac0a0216ae31150f292e9a9e1995ad332acf75b661116c8b6d0c2` |
| `tests/bypass-rust/src/founder_authored_guard.rs` | `src/founder_authored_guard.rs` | `cf6f32d5d37ad990dfa04cf6ef18c86661c61e0d6236be48d287261e3ff808e1` |
| `tests/bypass-rust/src/founder_before_tool_call_floor.rs` | `src/founder_before_tool_call_floor.rs` | `d1c98dedbf544ab1e27d3d9e12055f96e8a5d5b76b2c63edb76e4df4ff0b542f` |
| `tests/bypass-rust/src/founder_consumption_store.rs` | `src/founder_consumption_store.rs` | `813ca93068f9d81ae84d79b7b52ecd4aa15fd7e894b7ca599b30472375ac157c` |
| `tests/bypass-rust/src/founder_fail_closed.rs` | `src/founder_fail_closed.rs` | `fba6071de88417e1c551fc1793f7d6a77a6547cce438c34648d88dfd9bc8c3fc` |
| `tests/bypass-rust/src/founder_s2_approval_store.rs` | `src/founder_s2_approval_store.rs` | `1e524445376f64158974ccfddfa9676199c1930659391353e0a51f0af800221a` |
| `tests/bypass-rust/src/founder_s2_consumption_store.rs` | `src/founder_s2_consumption_store.rs` | `3da870fc853939f8fcac4a657fdcaf41f85e12680e76d6e369d754895d5f923d` |
| `tests/bypass-rust/src/founder_token_verification.rs` | `src/founder_token_verification.rs` | `ba86403c02e6d3714a0cb9bc2abdbe04cfd6ad83c619a789a8251147a51e73e5` |

Reproduction pattern:

```bash
git show 8318f61eadf689f9b8a72f673cc68cd083dc7831:tests/bypass-rust/src/FILE.rs | sha256sum
git show 91589759964f2a409960c6a21a5d16795f1d95a1:src/FILE.rs | sha256sum
git diff-tree --no-commit-id --name-status -r -M100% 91589759964f2a409960c6a21a5d16795f1d95a1
```

No T0 `use`, module-path, or other source patch was required. None is proposed for later application.

## 3. Shared-definition byte identity

The three definitions below formerly lived inside the mixed harness crate root. Their complete
source regions, including documentation and derives, were copied without byte changes.

| Definition | Base region | Destination region | Before and after SHA-256 |
|---|---|---|---|
| `RequiredOutcome` | base `tests/bypass-rust/src/lib.rs:22-29` | implementation `src/lib.rs:17-24` | `96e1592caeb9dd191b9df7c0f74a116ff337a4dfed356319349e64f37cf94405` |
| `ProposedAction` | base `tests/bypass-rust/src/lib.rs:42-52` | implementation `src/lib.rs:26-36` | `39671820a2c1f3a23d46b0fed8ca136505dd35c747e96c555531f7b2b64896d0` |
| `DecisionContext` | base `tests/bypass-rust/src/lib.rs:54-59` | implementation `src/lib.rs:38-43` | `0e5ee3bca220bb3f0d76d375edb2a3a3061a2655ef0393fb49817b840b11b6f6` |

Reproduce a row with `git show COMMIT:PATH | sed -n 'START,ENDp' | sha256sum`.

## 4. Wiring-only changes

Review these separately from the byte-identical moves:

1. `Cargo.toml` creates the root 0.x unpublished library and workspace.
2. `tests/bypass-rust/Cargo.toml` adds only the root path dependency and removes the now-transitive
   direct `rusqlite` dependency.
3. `Cargo.lock` is relocated to the workspace root and regenerated offline. Its substantive package
   graph change is the split into `dgr-core` and `dgr-core-bypass-harness`; dependency versions stay
   pinned.
4. `src/lib.rs` declares the nine public enforcement modules and contains the three byte-identical
   shared definitions.
5. `tests/bypass-rust/src/lib.rs` removes the moved module declarations/definitions, re-exports the
   root package, and otherwise retains the attack registry.
6. `.github/workflows/ci.yml` runs format, build, Clippy, tests, CodeQL, Semgrep, and cargo-deny from
   the root workspace. Required job names remain byte-for-byte unchanged.
7. `.gitignore`, `README.md`, `T0-AUTHORS.md`, and `tests/bypass-rust/T0-BOUNDARY.md` reflect only the
   new location, package layout, provenance, and bounded non-claims.

The deliberately small root public surface is the nine enforcement modules plus the three data
types required by those modules and their conformance consumer. No registry, fixture, adapter,
observation, or probe is exported by `dgr-core`.

## 5. Conformance and guard evidence

The implementation commit produced these local results with Rust `1.94.1` from
`rust-toolchain.toml`:

| Check | Result |
|---|---|
| `node scripts/check-structure.mjs` | PASS; 18 governance files present |
| `node --check scripts/check-ignored-attacks.mjs` | PASS |
| `node --test scripts/check-ignored-attacks.test.mjs` | PASS; 1/1 |
| `node scripts/check-ignored-attacks.mjs` | PASS; exact five ignored and named ATK-06 active |
| `cargo fmt --manifest-path Cargo.toml --all -- --check` | PASS |
| `cargo build --workspace --all-targets --locked` | PASS |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | PASS |
| `cargo test --workspace --all-targets --locked` | PASS |
| `cargo test --manifest-path tests/bypass-rust/Cargo.toml --all-targets --locked` | PASS; 52 passed, 5 ignored |
| `git diff --exit-code BASE -- tests/bypass-rust/tests scripts` | PASS; test and guard sources unchanged |
| `find . -name Cargo.lock -not -path './target/*'` | PASS; only `./Cargo.lock` |

The exact ignored set remains ATK-04, ATK-05, ATK-12, ATK-14, and ATK-15. The named
`atk_06_sequence_is_escalated_then_registry_derived_timeout_block` test remains active. The guard's
existing limitation remains: it proves the test is listed and not ignored, not assertion-body
integrity. Assertion-body integrity is additionally supported here by the unchanged test-source
diff.

The required GitHub job names remain exactly:

- `Structural / governance check`
- `Rust format / build / test`

Fresh GitHub checks and analyzer artifacts must still be evaluated on the actual final PR head.

## 6. Provenance

- Founder-authored source moved byte-identically remains founder-authored source.
- The PROD-000 agent-authored T0 floor remains agent-authored T0 with its recorded founder-source
  provenance for transformed regions.
- PROD-000 agent-transformed T0 regions retain their existing classification.
- PROD-001 is agent-assisted relocation and wiring. It does not become authorship of any moved
  enforcement body.
- Founder supervision, review, approval, and merge do not convert agent-authored or
  agent-transformed bytes into founder-authored bytes.

## 7. Non-claims

This extraction proves package separation and preserved isolation-harness behavior only. It does
not prove runtime hook installation, complete interception, agent non-bypassability, same-process
store or key protection, deployed fail-closed behavior, or any other ADR-14 runtime-integration
claim. RUNTIME-003 and RUNTIME-004 remain inactive until PROD-001 is founder-reviewed and merged.

## 8. Required review and merge gate

- [ ] non-author cross-model review recorded and satisfied
- [ ] independent-human review recorded
- [ ] founder byte-level relocation, provenance, and semantic-identity review
- [ ] founder disposition of every analyzer finding and diagnostic
- [ ] at least three required analyzer engines run on the actual final head
- [ ] required checks pass on the actual final head
- [ ] independent GitHub approval of the actual final head
- [ ] founder GitHub approval of the actual final head
- [ ] founder-only merge
