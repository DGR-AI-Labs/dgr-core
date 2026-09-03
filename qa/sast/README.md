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

## CORE-004 T0 two-surface approval timeout

**Scanned commit:** `60febb08ac9c3e207d6f7a3563b6824374c5c93e`

**Reviewed patch SHA-256:**
`71f051e24055cb0febd620d84a2703ea43d7277f5af4266feef8d03d0fbb9f1f`

**Cargo.lock SHA-256:**
`59c1a398c6a3405a6a895da2e794b7ed7cb0d6970ac74840cbd698c40b0f39ad`

| Engine | Raw artifact | SHA-256 | Result |
|---|---|---|---|
| Semgrep | `core-004-t0-semgrep-2026-08-21.txt` | `1cff06e57cb8f7b256abfd91f0c58b5598edfadf21de81536cc3141664dd3463` | 20/20 targets, 0 errors, 1 INFO finding |
| Semgrep | `core-004-t0-semgrep-2026-08-21.json` | `1a18b314f853591469943751290779bf14bf7b59d5ac971ffbcc2fa4e07e5084` | machine-readable raw result |
| CodeQL | `core-004-t0-codeql-2026-08-21.txt` | `b2ec19be1f5b114787bf0096a6b9cda434965331f8d669c5b70e7ffef3648850` | 20/20 extracted, 0 errors, 9 findings |
| CodeQL | `core-004-t0-codeql-2026-08-21.sarif` | `85e7d20718c2fd8732699b61ba88a1d635fa927a35809e2e877efae3e1e801ba` | valid SARIF, one run |
| cargo-deny | `core-004-t0-cargo-deny-2026-08-21.txt` | `23e4ec1efb089c4c960f977999520e1bd1be13b36b3c8e49200258708b917cb6` | exit 0; no blocking diagnostics |

The Semgrep finding is the existing test-only temporary-path construction in
`tests/consumption_store.rs`. CodeQL reports the seven previously reviewed
VAL-002 deterministic fixture nonces plus two deterministic VAL-004 fixture
nonces at `src/val_004_fixtures.rs:136,143`. The CodeQL Rust diagnostics also
report seven path-resolution inconsistencies, but all 20 tracked Rust files
were extracted without error and the analysis produced no execution errors.
Every finding and diagnostic still requires explicit founder disposition for
this exact-commit gate.

## PROD-000 supervised-agent T0 partition

**Scanned commit:** `425d7718ecf83086776de8fc09caec26c728df92`

**Agent implementation commit:** `40b713039a5612831df415cdd785271a7342be74`

**Cargo.lock SHA-256:**
`59c1a398c6a3405a6a895da2e794b7ed7cb0d6970ac74840cbd698c40b0f39ad`

The scanned commit is a documentation-only descendant of the implementation commit. The evidence
commit is a non-Rust descendant of the scanned input. Founder approval must name the final PR head.

| Engine | Raw artifact | SHA-256 | Result |
|---|---|---|---|
| Semgrep | `prod-000-semgrep-2026-09-01.txt` | `8b7eb9e3b62066c8dfe6d82bde0fec634ca5577f3a26657fd4ba6ba4d318fcbe` | 21/21 targets, 0 scan errors, 1 finding, exit 1 under `--error` |
| Semgrep | `prod-000-semgrep-2026-09-01.json` | `cc84acaf9645f48605fd84ad48558930f9276082d72649c106260ecaf44950c7` | machine-readable raw result |
| CodeQL | `prod-000-codeql-2026-09-01.txt` | `3c6b73865eee28839dea7ac3e7681a767c2365665e36dc087b2e8d21ed2e9c20` | 21/21 tracked Rust files scanned, 9 findings, one generated dependency-output extraction warning |
| CodeQL | `prod-000-codeql-2026-09-01.sarif` | `4c22554bbda1637966303d722bfc450aa651f1ded79eaec7539f2161797c76b7` | valid SARIF, one run |
| cargo-deny | `prod-000-cargo-deny-2026-09-01.txt` | `13acf718133ce67aa84c0780632fe0f444eee6f20063256012485a272ad7ef8a` | exit 0; no blocking diagnostics |

The Semgrep and CodeQL findings are the same temporary-path and deterministic-fixture-nonce
surfaces present in the fresh CORE-004 scan. No finding touches the new
`founder_before_tool_call_floor.rs` module or the R5.1 timeout ownership change. That comparison is
context only: every result, plus CodeQL's generated `libsqlite3-sys` extraction warning, remains
undispositioned until independent-human and founder review of this exact PROD-000 PR.

### Cross-model remediation scan

**Scanned replacement source commit:** `b19f33ae16698a81b993e6cc5a751360b6109577`

**Scanned tree:** `12fadecee74b6387095977980fb215ddc7fe3c1c`

The fresh scan follows the first Claude review and the source/document corrections responding to
it. A later T3-only guard commit, `587585cf476431f078efe587c5dbcc052389cdad`, changes only the two
JavaScript ignored/active-test enumeration files. No Rust, Cargo, lockfile, deny policy, or workflow
change follows the scanned tree.

| Engine | Raw artifact | SHA-256 | Result |
|---|---|---|---|
| Semgrep | `prod-000-remediation-semgrep-2026-09-02.txt` | recorded by the resubmission manifest | wrapper and command binding |
| Semgrep | `prod-000-remediation-semgrep-2026-09-02.raw.txt` | `ee610b6a9fe897650b14845c3032100ab819abe0edf2d1877493c43c205e6d6c` | unaltered tool text |
| Semgrep | `prod-000-remediation-semgrep-2026-09-02.json` | `a3b5138d02b4801a332a3d2786b7567ceaa0494cd38c848495b96911604a315b` | 21/21 Rust targets, one INFO finding, zero scan errors |
| CodeQL | `prod-000-remediation-codeql-2026-09-02.txt` | recorded by the resubmission manifest | wrapper, command, coverage, and diagnostic binding |
| CodeQL | `prod-000-remediation-codeql-2026-09-02.sarif` | `a8c08a09592603f38b266959cef875a06240e25ecc952e606f3d0c367e88b82c` | 21/21 tracked Rust files, nine fixture findings, one generated-file warning, no error-level notification |
| cargo-deny | `prod-000-remediation-cargo-deny-2026-09-02.txt` | recorded by the resubmission manifest | exit 0; no blocking diagnostic |

The finding identities are unchanged from the first PROD-000 scan. The fresh records do not
self-disposition them. Rust-only scanner scope is explicit; the T3 JavaScript guard is covered by
syntax, unit, and live libtest-enumeration checks rather than a claimed JavaScript SAST leg.

### Canonical final-executable-input scan

**Exact scanned commit:** `587585cf476431f078efe587c5dbcc052389cdad`

**Exact scanned tree:** `89e5de51d23ead98d24bbbe1b4cd57db343b2dc4`

This is the canonical PROD-000 resubmission scan. It was run from an immutable `git archive` of the
commit that contains both the replacement Rust source and the T3 non-droppable-assertion guard.

| Engine | Raw artifact | SHA-256 | Result |
|---|---|---|---|
| Semgrep | `prod-000-final-input-semgrep-2026-09-02.raw.txt` | `ee610b6a9fe897650b14845c3032100ab819abe0edf2d1877493c43c205e6d6c` | unaltered text output |
| Semgrep | `prod-000-final-input-semgrep-2026-09-02.json` | `a6ba26f5d90f716c48b2199a802a534accdd6119cb72a0c270a3c86b2f5aebd3` | 21/21 Rust targets, one result, zero scan errors |
| CodeQL | `prod-000-final-input-codeql-2026-09-02.sarif` | `37947d2ec95a60b120664413ccc009b6c15ebf240ae84eb253cdac8556402c10` | 21/21 tracked Rust files extracted without error, nine results, zero extraction warnings |
| cargo-deny | `prod-000-final-input-cargo-deny-2026-09-02.txt` | recorded by the bundle manifest | exit 0; no error or warning |

The wrappers adjacent to these artifacts record exact commands and the archive materialization.
The earlier `prod-000-remediation-*` run is retained as intermediate audit evidence and is not the
canonical exact-input scan.
