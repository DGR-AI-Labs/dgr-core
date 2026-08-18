# T0 SAST/SCA evidence index

## CORE-002 Step 5

**Final scanned commit:** `0a54d4995d1b9d98ab8a3ec61861fe2fe7ae29c3`

**T0 implementation commit:** `0727e327631b475990ef8d9b7ef3b2c3554050a8`

**Cargo.lock SHA-256:** `59c1a398c6a3405a6a895da2e794b7ed7cb0d6970ac74840cbd698c40b0f39ad`

The final commit is a non-Rust descendant of the reviewed T0 commit. The
intervening changes are QA review records, `deny.toml`, and the package's
Apache-2.0 metadata; no Rust file changed. Each final artifact records its
command, version, exit code, and exact commit. A nonzero Semgrep exit indicates
a finding, not an execution failure.

## Final artifacts

| Engine | Raw artifact | SHA-256 | Result |
|---|---|---|---|
| Semgrep | `core-002-step5-semgrep-2026-08-17.txt` | `858d3da13051342bac9116dd0917668bc35bac941d28c133c28309e479bce625` | 14/14 targets, 0 errors, 1 INFO finding |
| Semgrep | `core-002-step5-semgrep-2026-08-17.json` | `bf67ccbf5be76df3d6b114bafdbadfa0f423204e26e7eb39a9c1af2e3002fa5e` | machine-readable raw result |
| CodeQL | `core-002-step5-codeql-2026-08-17.txt` | `374f0bfcc843d7e00fb109605510271feca2ce2e46b04e666ae0be3a64c19d40` | 14/14 extracted, 0 errors, 7 findings |
| CodeQL | `core-002-step5-codeql-2026-08-17.sarif` | `501992b1c163783259d04eebe81e2865d9d8986c0cecedd099da9c75dbca8eee` | valid SARIF 2.1.0, one run |
| cargo-deny | `core-002-step5-cargo-deny-2026-08-17.txt` | `4f96ce1b1931f776a18d0884e8863e794f1d778558a79cc790decb14b73cab7e` | exit 0; 0 blocking diagnostics |

## Findings requiring founder disposition

- Semgrep `rust.lang.security.temp-dir.temp-dir` at
  `tests/bypass-rust/tests/consumption_store.rs:19`. The use is confined to a
  local conformance test, but its predictable temp-path construction can race
  another local process. Accept explicitly as test-only risk or remediate and
  rerun all three scans.
- CodeQL `rust/hard-coded-cryptographic-value` at
  `tests/bypass-rust/src/val_002_fixtures.rs:126,152,169,186,204,222,339`.
  These are deliberate, distinct deterministic fixture nonces, not production
  keys, passwords, salts, or runtime-generated nonces. One grouped disposition
  may cover all seven if the founder confirms that boundary.
- cargo-deny passes, but founder review still covers the two version-pinned
  duplicate-dependency notes and all 54 accepted-license notes.

## Historical evidence

`pre-policy-0727e327/` preserves the first complete scans against the T0 commit.
The unconfigured cargo-deny run failed closed because default license policy did
not encode the project's accepted licenses. The directory is retained as an
audit trail; only the artifacts in this directory's root are the final gate
inputs.

## CORE-003 T0 reached-boundary floor

**Scanned commit:** `6cb6826fb29ee18bd2ce5f596c620f4170f37a47`

**Reviewed patch SHA-256:**
`d689cfe6fc092b7cac1fcfea397e09288a169aa2be14c94583cff57eedc905d9`

**Cargo.lock SHA-256:**
`59c1a398c6a3405a6a895da2e794b7ed7cb0d6970ac74840cbd698c40b0f39ad`

| Engine | Raw artifact | SHA-256 | Result |
|---|---|---|---|
| Semgrep | `core-003-t0-semgrep-2026-08-18.txt` | `d7ad41bcdf9f965a00cdbac474004c038e70d8f68cbfd8831560e823c5869fbe` | 14/14 targets, 0 errors, 1 INFO finding |
| Semgrep | `core-003-t0-semgrep-2026-08-18.json` | `3a48987313ca4762ac239fdd194c902d32db98780479aaa5548c12b62af2d4dc` | machine-readable raw result |
| CodeQL | `core-003-t0-codeql-2026-08-18.txt` | `5d87687f6343e31b91640caaab4fc9fa4ef6c423224274627135b77d23535ee3` | 14/14 extracted, 0 errors, 7 findings |
| CodeQL | `core-003-t0-codeql-2026-08-18.sarif` | `1cc6447023cf129fcf127b564b26ad6b7f9f60c08d9dc005b7258d39c13918ed` | valid SARIF, one run |
| cargo-deny | `core-003-t0-cargo-deny-2026-08-18.txt` | `eb78a5c15edf0ca315020d38a2136dcabedff79a427b4e3aa0149335779b220a` | exit 0; no blocking diagnostics |

The Semgrep and CodeQL findings are the same test-only surfaces reviewed for
CORE-002: one temporary-path helper in `tests/consumption_store.rs` and seven
deterministic fixture nonces in `src/val_002_fixtures.rs`. No result touches
`before_tool_call.rs` or the new CORE-003 boundary logic. These findings still
require explicit founder disposition for this exact-commit gate; prior
disposition is context, not a substitute for reviewing the fresh raw output.
