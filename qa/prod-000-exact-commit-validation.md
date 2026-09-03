# PROD-000 exact-input validation record

**Scanned implementation/provenance commit:**
`425d7718ecf83086776de8fc09caec26c728df92`

**Agent implementation commit:**
`40b713039a5612831df415cdd785271a7342be74`

The scanned commit is a documentation-only descendant of the implementation commit. The evidence
commit that adds this record and raw artifacts is a non-Rust descendant of the scanned commit. The
founder must review and approve the final PR head; this record does not substitute for that review.

## Functional and policy gates

| Gate | Result |
|---|---|
| `cargo fmt --manifest-path tests/bypass-rust/Cargo.toml --all -- --check` | PASS |
| `cargo build --manifest-path tests/bypass-rust/Cargo.toml --all-targets --locked` | PASS |
| `cargo clippy --manifest-path tests/bypass-rust/Cargo.toml --all-targets --locked -- -D warnings` | PASS |
| `cargo test --manifest-path tests/bypass-rust/Cargo.toml --all-targets --locked` | PASS: 52 passed, 5 ignored |
| `npm run check:structure` | PASS: 18 governance files present |
| `node scripts/check-ignored-attacks.test.mjs` | PASS: 4 of 4 |
| `node scripts/check-ignored-attacks.mjs` | PASS: exact ATK-04/05/12/14/15 ignored set |

The CI workflow, Cargo manifest, lockfile, `deny.toml`, and `attack_set.rs` remained byte-identical
to the recorded baseline hashes. No test expectation or ignored-test set was changed.

## Three-engine evidence

| Engine | Result | Raw evidence |
|---|---|---|
| Semgrep 1.173.0 / `p/rust` | COMPLETE: 21/21 tracked Rust files; 1 finding; exit 1 under `--error` | `qa/sast/prod-000-semgrep-2026-09-01.txt`, `.json` |
| CodeQL 2.25.5 / `codeql/rust-queries@0.1.35` | COMPLETE for 21/21 tracked Rust files; 9 findings; one generated dependency-output extraction warning | `qa/sast/prod-000-codeql-2026-09-01.txt`, `.sarif` |
| cargo-deny 0.20.2 | PASS: advisories, bans, licenses, sources; 0 blocking diagnostics | `qa/sast/prod-000-cargo-deny-2026-09-01.txt` |

## Findings awaiting disposition

No finding is suppressed or dispositioned by the authoring agent.

- Semgrep reports `rust.lang.security.temp-dir.temp-dir` at
  `tests/bypass-rust/tests/consumption_store.rs:19`.
- CodeQL reports seven hard-coded nonce findings in `val_002_fixtures.rs` and two in
  `val_004_fixtures.rs` at the exact lines listed in the raw text record.
- CodeQL scanned every tracked Rust file after matching `rust-src` was installed. It also reports
  one extraction warning for generated `libsqlite3-sys` `bindgen.rs`; this warning and its scope
  require reviewer acceptance or remediation.
- cargo-deny reports two bans notes and 54 license notes with no errors or warnings.

## Review gates still open

- non-author cross-model review;
- independent-human review;
- founder line-by-line review and disposition of every SAST/SCA finding and diagnostic;
- founder approval of the exact final PR head.

Until those records are complete, PROD-000 is not merge-ready and PROD-001 extraction is not
authorized.
