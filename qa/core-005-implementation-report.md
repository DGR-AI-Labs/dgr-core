# CORE-005 implementation report

- **Date:** 2026-08-22
- **Branch:** `codex/core-005-implementation`
- **Review:** [dgr-core PR #84](https://github.com/DGR-AI-Labs/dgr-core/pull/84) (draft; founder review and merge required)
- **Base:** `b75efef49f278ac2828a9a341c88d56c15430ccb`
- **Precondition reconciliation commit:** `7695e44d1f2514eb3e9c7b1ee26ed34259a803b9`
- **CI implementation commit:** `d7c78e410f84139109075f3f9f2baf308e3ce8d9`
- **Scope:** T3 CI/governance wiring only; no T0 enforcement, attack outcome, test expectation, or `#[ignore]` change
- **Parent status:** In Progress until the founder applies and verifies the branch-protection settings

## Bounded completion claim

> All currently active conformance tests must pass on every merge, and the exact deferred/external set must remain visible and unchanged without review.

CORE-005 Done alone does not satisfy the runtime trigger. That trigger remains CORE-005 Done **and** ATK-01..14 green. ATK-04/05/12/14 remain deferred, ATK-15 remains external, and RUNTIME-003/004/006 remain Deferred.

## A. Precondition reconciliation

Task A was completed and committed before the CI gate implementation:

- `tests/bypass-rust/T0-BOUNDARY.md:27-37` records the completed CORE-002 Step 5, CORE-003, and CORE-004 human gates and retains the bounded isolation claim.
- `tests/bypass-rust/T0-BOUNDARY.md:90-103` records CORE-004 acceptance and the exact active/deferred/external split.
- `T0-AUTHORS.md:51-68` records the completed gate state without changing ownership or authorship restrictions.
- `scripts/check-structure.mjs:1-7` describes Phase 1 and an active suite expected green; its checks were not changed.
- `README.md:5-23` describes the canonical active Rust harness without claiming deployed enforcement.
- `package.json:15` replaces the legacy Phase-0/red-scaffold comment.

`node scripts/check-structure.mjs` passes with all 18 governance files present. The required README disclaimers remain intact.

## B. Pinned required Rust job

- `rust-toolchain.toml:1-4` pins Rust `1.94.1` with the minimal profile plus `rustfmt` and `clippy`.
- Local resolution reports `rustc 1.94.1 (e408947bf 2026-03-25)` and `cargo 1.94.1 (29ea6fb6a 2026-03-24)` from that repository override.
- `.github/workflows/ci.yml:26-42` keeps one required Rust context named `Rust format / build / test`.
- `.github/workflows/ci.yml:31-38` retains formatting, explicit locked all-target build, and locked all-target tests, and adds locked all-target Clippy with `-D warnings`.
- The former standalone `rustup component add rustfmt` step was removed as redundant: rustup honors `rust-toolchain.toml`, whose component list installs both `rustfmt` and `clippy` for the exact pinned toolchain.

Every required Cargo operation that resolves/builds the crate carries `--locked` (build, Clippy, test). `cargo fmt` does not resolve dependencies and does not accept Cargo's `--locked` option.

### Local required-surface results

All commands ran from a clean branch worktree with `CARGO_TARGET_DIR=/tmp/dgr-core005-implementation-target`:

| Check | Result | Elapsed |
| --- | --- | ---: |
| `node scripts/check-structure.mjs` | PASS; 18 governance files | 0.06 s |
| `cargo fmt --manifest-path tests/bypass-rust/Cargo.toml --all -- --check` | PASS | 0.34 s |
| `cargo build --manifest-path tests/bypass-rust/Cargo.toml --all-targets --locked` | PASS | 1.05 s |
| `cargo clippy --manifest-path tests/bypass-rust/Cargo.toml --all-targets --locked -- -D warnings` | PASS | 2.62 s |
| `cargo test --manifest-path tests/bypass-rust/Cargo.toml --all-targets --locked` | PASS; 52 passed, 5 ignored, 0 failed | 0.31 s |

The first live ignored-set enumeration took 22.22 s, including pinned-toolchain synchronization and compilation; subsequent CI runs can reuse the Rust job's build cache within the job.

## C. Observe-only ignored-set guard

- `scripts/check-ignored-attacks.mjs:7-16` contains the one explicit founder-reviewed expected set: ATK-04, ATK-05, ATK-12, ATK-14, and ATK-15, expressed as their five libtest names.
- `scripts/check-ignored-attacks.mjs:18-25` compares set membership in both directions.
- `scripts/check-ignored-attacks.mjs:37-83` runs locked libtest enumeration and fails closed on enumeration failure, an added ignore, or a missing ignore.
- `scripts/check-ignored-attacks.test.mjs:10-31` proves the exact set passes and both mutation directions fail comparison.
- `.github/workflows/ci.yml:39-42` wires the comparison tests and live enumeration into the existing Rust context. This placement keeps the number of required contexts at two and runs the Cargo-based observer against the same pinned harness.

The live command passed with exactly:

- `atk_04_missing_justification`
- `atk_05_ambiguous_evidence`
- `atk_12_revoked_credential`
- `atk_14_cross_tenant_use`
- `atk_15_deploy_role_data_access`

No `#[ignore]`, Rust source, Rust test, outcome, or expectation changed between the base and the implementation commits.

## D. Informational SAST/SCA

`.github/workflows/ci.yml:44-103` replaces the stale placeholder with three deliberately non-blocking jobs:

- Semgrep `1.173.0`, scanning the Rust harness with the Rust ruleset;
- CodeQL Rust with `security-extended` queries and a locked all-target build;
- cargo-deny using `deny.toml`, `Cargo.lock`, and `--locked`.

The workflow comments state the controlling policy: informational only; the blocking T0 gate remains the exact-commit, founder-dispositioned three-engine evidence (FND-7). None of these job contexts is proposed as required, and no baseline or suppression file was introduced.

Known reviewed findings are recorded in the workflow so routine output is not mistaken for a new regression: Semgrep's test-only `temp-dir`; CodeQL's deterministic fixture nonces plus diagnostic-only path-resolution notes; and cargo-deny's two reviewed duplicate-version skips with an empty advisory-ignore list. The exact local cargo-deny policy command passed in 1.11 s with 0 errors and 0 warnings (2 bans notes, 54 license notes). The Semgrep registry ruleset fetch could not be repeated inside the agent sandbox because its injected proxy URI is invalid; the CI job remains the intended networked execution environment, and prior exact-commit raw evidence is unchanged.

## E. Founder-owned GitHub settings

`qa/core-005-founder-settings-checklist.md:8-56` records the exact current-to-target values, ordered UI steps, protection-endpoint verification, test-PR proof, and the residual separation-of-duties limitation.

The protection endpoint was read on 2026-08-22 and returned:

```json
{"admins":true,"approvals":0,"contexts":["Structural / governance check"],"deletions":false,"force_pushes":false,"strict":true}
```

The founder must change approvals from 0 to 1 and add `Rust format / build / test`, keeping all other recorded values. Gaziz Nugmanov is the designated eligible independent reviewer. The agent did not apply or mutate branch protection.

## Boundary verification

- `git diff b75efef -- tests/bypass-rust/src tests/bypass-rust/tests` is empty.
- The source still contains exactly five `#[ignore]` expansions/attributes: the reviewed four-case deferred macro and the external ATK-15 case.
- `git diff --check` passes.
- The workflow YAML parses with the five expected jobs.
- No SAST baseline or suppression was added; `deny.toml` retains `advisories.ignore = []`.

## Remaining human actions

1. Review and merge the `dgr-core` implementation pull request; the agent must not self-merge it.
2. Apply the founder settings checklist to `main`.
3. Verify the protection endpoint and a test pull request as specified in the checklist.
4. Retain the settings/test-PR evidence and only then mark CORE-005 Done with the bounded claim above.
