# CORE-003 final merged state for independent QA

## Claimed completed outcome

CORE-003's reached-hook ATK-07 boundary is implemented, tested, scanned,
reviewed, merged, and marked Done. The implementation maps typed guard faults
and Rust unwind-mode panics to an explicit fail-closed block before any
effectful invocation. It preserves returned Allow/Deny relay semantics.

## Commit and merge chain

- Baseline before founder authoring:
  `4c7f6a33a5f0c01c42eed81b936a77450c8edd40`
- Founder-authored exact implementation:
  `6cb6826fb29ee18bd2ce5f596c620f4170f37a47`
- QA/SAST evidence descendant before PR #73:
  `9fa1c2e3bbd9be4303ee49fb1b90a07c6b02fcd4`
- PR #73 merge:
  `50347fe169e1207146ffe7a111669cddcd22c664`
- Signed-review evidence commit:
  `9934c236728332d11b0d072ba1465951d0ffe7d5`
- PR #74 merge:
  `6db4761f42c79ccd757bcef9726466aef6610776`
- PR #75 final evidence-status merge / bundled dgr-core main:
  `4aeeceba24a353c399b09054bc84ea4ab84a55ba`
- Bundled dgr-backlog main:
  `7f5771108abe6540f208a6e89799d0258dfb1eb4`

The bundle's drift report should show no change after `6cb6826f...` to the
seven reviewed implementation/governance files, any Rust file, Cargo inputs,
or `deny.toml`.

## Fresh final-main validation

At dgr-core main `4aeeceba...`:

- `cargo fmt --check`: PASS.
- `cargo clippy --locked --all-targets -- -D warnings`: PASS.
- `cargo test --locked --all-targets`: PASS.
- Aggregate result: 39 passed; 6 unrelated/deferred cases ignored.
- Both dedicated ATK-07 cases: active and PASS.
- Structural/governance check: PASS.
- Baseline-to-final whitespace check: PASS.

## Review gates

- Claude source review of the founder-authored patch: PASS.
- Semgrep 1.173.0: 14/14 Rust targets, one adjudicated test-only finding.
- CodeQL 2.25.5 / Rust queries 0.1.35: 14/14 extracted, seven adjudicated
  deterministic fixture findings.
- cargo-deny 0.20.2: PASS with no warning/error.
- Founder review: APPROVE, including all scanner dispositions.
- Independent human review: Gaziz Nugmanov, PASS, no findings, exact patch.
- PR #73: merged; signed records persisted in PR #74; final readiness wording
  corrected in PR #75.

## Canonical backlog

At dgr-backlog main `7f577110...`:

- `CORE-003-T3-tests`: Done.
- `CORE-003-T0-boundary`: Done.
- `CORE-003`: Done.
- Offline backlog verification passed against the current rendered reference
  set before closeout merge.

## Claim boundary

The completed claim is limited to a reached Rust isolation boundary receiving
a typed guard fault or unwind-mode panic. RUNTIME-003/004 still own hook-never-
fired, route-around, missing-plugin, operator-bypass, and deployed-runtime
non-bypassability work. No bundle statement should be read as completing those
runtime guarantees.
