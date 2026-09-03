# PROD-001 independent-human review

**Verdict:** `PASS WITH NON-BLOCKING FINDINGS — INDEPENDENT-HUMAN GATE SATISFIED`

**Status:** signed — independent-human gate complete; mechanically normalized by OpenAI Codex
without changing the reviewer's verdict or risk judgments

**Submitted source SHA-256:**
`3c4e761b5afc04a1f72cc8ee9e8c48d6707675adacaa09d939207b972a65feaf`

Editorial note: the submitted record contained unresolved `[cite: 3]`/`[cite: 4]` transport
markers, incorrect manifest totals, an ambiguous PROD-000 participation answer, and a findings
table whose F-identifiers did not match the cross-model F-1 through F-9 identifiers. Codex removed
the transport markers, corrected objective counts and disclosure from the supplied evidence, added
evidence paths, and restored the original finding identifiers. The human-authored verdict,
attestation, recommendations, and timestamps are preserved. Any genuinely missing human judgment
is marked explicitly rather than invented.

## Reviewer identity and independence

- Full name: Gaziz Nugmanov
- Role/relationship: Co-founder
- GitHub username: gaziz
- Review start (UTC, as submitted): `2026-09-03T18:37:35Z`
- Decision time (UTC, as submitted): `2026-09-03T18:37:35Z`
- PROD-001 authorship/direction/evidence participation: None.
- PROD-000 participation: Independent-human reviewer of PROD-000; no implementation,
  remediation, direction, or evidence-preparation authorship. This corrects the submitted word
  "None" using the signed record at
  `evidence/addendum/records/prod-000-independent-human-review.md:13-20`.
- Independent-review attestation: I confirm my judgment is fully independent.

Review target: PR #91 at head `f1d17087d140b41750c1aeca032916bb4d2d90ae`, tree
`350d1f1bdadd7b26475a44f29c981dfc00743d2a`, against baseline
`8318f61eadf689f9b8a72f673cc68cd083dc7831`. See
`evidence/core/metadata/commit-tree-identities.txt:1-9` and `github/pr-91.json`.

## Integrity

| Check | Exact result | PASS/FAIL |
|---|---|---|
| Top-level manifest | Checked 177/177 files | PASS |
| Core nested manifest | Checked 122/122 files | PASS |
| Addendum nested manifest | Checked 39/39 files | PASS |
| Cross-model review digest | `e05a1e3864d181a723eaf9769f51904b90825b2681bcc065216c477c917b6a06` | PASS |
| Cross-model addendum digest | `607716840944eb2507c64874a16e0e6773b00131d25ad2e68dd1d71fa67d7825` | PASS |
| Reviewed identity | Matches PR #91 head `f1d17087...` | PASS |

## Extraction matrix

| # | YES / NO / NOT ESTABLISHED | Evidence and reasoning |
|---|---|---|
| 1 | YES | All nine enforcement files are 100% byte-identical moves. See review-bundle `evidence/core/metadata/rename-summary.txt:4-12` and `t0-byte-identity.sha256:1-18`. |
| 2 | YES | `RequiredOutcome`, `ProposedAction`, and `DecisionContext` moved as byte-identical complete source regions. See review-bundle `evidence/core/metadata/shared-definitions.sha256:1-6`. |
| 3 | YES | No enforcement expression or selection behavior changed through imports, module wiring, features, dependencies, re-exports, or alternate source paths. See review-bundle `evidence/core/dgr-core/reviewed/src/lib.rs:7-43`, `dgr-core/reviewed/tests/bypass-rust/src/lib.rs:7-17`, and the full patch. |
| 4 | YES | Root `dgr-core` is version `0.1.0`, has `publish = false`, and names `src/lib.rs`. See review-bundle `evidence/core/dgr-core/reviewed/Cargo.toml:1-9`. |
| 5 | YES | The harness consumes it through `version = "=0.1.0", path = "../.."`; the root defines the workspace and the inventory contains one lockfile. See review-bundle `evidence/core/dgr-core/reviewed/tests/bypass-rust/Cargo.toml:11-15`, `dgr-core/reviewed/Cargo.toml:16-18`, and `metadata/cargo-lock-inventory.txt:1`. |
| 6 | YES | Root `src/lib.rs` declares the nine enforcement modules and three required data types, subject to the separately identified public `CONFORMANCE_*` constants inside moved modules. See review-bundle `evidence/core/dgr-core/reviewed/src/lib.rs:7-43`. |
| 7 | YES | Third-party dependency versions and relevant features remain pinned. See review-bundle `evidence/core/dgr-core/reviewed/Cargo.toml:11-14`, `dgr-core/reviewed/tests/bypass-rust/Cargo.toml:11-15`, and `metadata/baseline-to-reviewed.diff`. |
| 8 | YES | Registry, fixtures, adapter, observations, probes, and tests remain T3 in the harness. See review-bundle `evidence/core/dgr-core/reviewed/tests/bypass-rust/src/lib.rs:1-17` and `metadata/change-name-status.txt:1-21`. |
| 9 | YES | Test and guard source did not change; `metadata/tests-and-guards-diff.empty` is zero bytes, and the workspace run totals 52 passed / 5 ignored across the target summaries in review-bundle `evidence/addendum/analyzers/local-cargo-test-workspace.txt:1-111`. |
| 10 | YES | The ignored set is exactly ATK-04, ATK-05, ATK-12, ATK-14, and ATK-15, with the named ATK-06 test active. See review-bundle `evidence/core/metadata/ignored-set-guard.txt:1-2` and `evidence/addendum/analyzers/local-cargo-test-workspace.txt:23-54,64-73`. |
| 11 | YES, qualified | Required GitHub job names are preserved, and format/build/Clippy/test use workspace scope. The informational cargo-deny command does **not** yet cover both packages because CI omits `--workspace`; that separate F-2 correction remains required. See review-bundle `evidence/core/dgr-core/reviewed/.github/workflows/ci.yml:19-42,99-112` and `evidence/addendum/metadata/cargo-deny-scope-diff.json:1-8`. |
| 12 | YES | Documentation and provenance records distinguish founder-authored, agent-authored, agent-transformed, and PROD-001 relocation/wiring work. See review-bundle `evidence/core/dgr-core/reviewed/T0-AUTHORS.md`, `tests/bypass-rust/T0-BOUNDARY.md`, and `qa/prod-001-extraction-review-input.md:124-140`. |
| 13 | YES | The reviewed root crate documentation expressly limits the result and says runtime interception and non-bypassability remain unproven. See review-bundle `evidence/core/dgr-core/reviewed/src/lib.rs:1-5` and `qa/prod-001-extraction-review-input.md:142-148`. |
| 14 | YES | No alternate tracked Rust path can compile a different enforcement body from the hashed one. See the 22-file tracked-source enumeration in review-bundle `evidence/core/analyzers/github-semgrep.log:469-476`, the move inventory at `metadata/change-name-status.txt:8-16`, and root/harness module wiring above. |

## Wiring and provenance review

I directly reviewed the wiring-only changes separately from the byte-identical moves in review-bundle
`evidence/core/metadata/baseline-to-reviewed.diff`.

- F-3: The legacy failing `src/gate.mjs` remains beside the Rust library, accurately reflecting a
  deferred cleanup state. The file itself is unchanged; the co-location results from moving the
  Rust enforcement files into root `src/`.
- F-4: The eleven public `CONFORMANCE_*` constants are appropriately documentation-hidden. Their
  bytes are unchanged, but the containing enforcement modules moved into the root library.
- F-6: The distinction between `node --test FILE` reporting one file unit and CI's direct
  `node FILE` reporting seven cases is accurate and verified by review-bundle
  `evidence/addendum/analyzers/node-test-file-mode.txt` and `node-direct-mode.txt`.
- F-7: The review input still lacks a reviewed-head-bound reproduction command. See review-bundle
  `evidence/core/dgr-core/reviewed/qa/prod-001-extraction-review-input.md:51-57`. This is an evidence
  limitation, not a behavioral implementation shift.

## Analyzer and diagnostic review

### Semgrep

Version 1.173.0 ran against 11 Rust rules and 22/22 tracked Rust files with zero scan errors. The
single `rust.lang.security.temp-dir.temp-dir` result at unchanged
`tests/bypass-rust/tests/consumption_store.rs:19` is confirmed in review-bundle
`evidence/core/analyzers/local-semgrep.json`. This does not touch PROD-001 changes. I recommend
founder disposition as an accepted testing boundary.

### CodeQL PR and full-branch analyses

The PR merge-ref analysis (`1719130860`) yielded zero results in changed ranges across 27 rules.
The exact-head push analysis (`1719128797`) flagged nine
`rust/hard-coded-cryptographic-value` results in unchanged T3 fixture files; see review-bundle
`evidence/addendum/analyzers/github-branch-codeql-analysis.json` and
`metadata/codeql-nine-results.json`. The log reports 22/22 tracked Rust files scanned and a
generated `bindgen.rs` extraction warning at lines 1163 and 1437-1442. The second raw diagnostic
message remains unenumerable; I conclude it may remain a bounded founder-gate evidence limitation.
The prior PROD-000 founder acceptance of the nine deterministic fixture nonces is relevant
provenance, and I recommend reaffirmation.

### cargo-deny

The root-only graph contains 53 entries, while the workspace scope contains 55. Exactly
`base64ct 1.8.3` and `dgr-core-bypass-harness 0.0.0` were added; see review-bundle
`evidence/addendum/metadata/cargo-deny-scope-diff.json:1-8`. There are two bans notes and zero
errors/warnings. Since current CI lacks `--workspace`, I recommend adding it before the final-head
review.

### Tooling and traceability diagnostics

The CodeQL Action v3 December 2026 deadline and Node 20-to-24 forcing represent dated maintenance,
not current extraction blockers. Complete run/job mappings are present in review-bundle
`evidence/addendum/analyzers/github-pr-run.json` and `github-branch-run.json`; embedded run IDs in
each text-log header are optional hygiene. Capturing the missing raw CodeQL diagnostic is a
separate founder-gate evidence question, not part of the dated maintenance classification.

## PR #90 prerequisite

Based on inspection of review-bundle `github/pr-90.json`, `pr-90-reviews.json`,
`pr-90-comments.json`, and `pr-90-merge-commit.txt`, I confirm:

1. Founder fallback approval by `sapenov` at exact head `a85e3676...`, recorded at
   `https://github.com/DGR-AI-Labs/dgr-core/pull/90#issuecomment-5518513024`.
2. Formal independent approval by `bakaevs` on that exact head, recorded at
   `https://github.com/DGR-AI-Labs/dgr-core/pull/90#pullrequestreview-5103039814`.
3. Founder merge by `sapenov` as `8318f61e...` at `2026-09-03T14:20:30Z`.
4. Merge-parent continuity from `a85e3676...` into `8318f61e...`.

This closes cross-model finding F-8. This does not imply PROD-001 approval.

## Findings

| ID | Severity | Evidence | Impact | Recommendation | Changed footprint? |
|---|---|---|---|---|---|
| F-1 | NON-BLOCKING | Exact-head full-branch CodeQL analysis closes the PR diff-range coverage gap; nine unchanged fixture results and two raw diagnostics remain for founder disposition. | Coverage gap closed; decision work remains | Reaffirm the nine fixture findings and explicitly disposition the diagnostic limitation | No source change |
| F-2 | NON-BLOCKING | cargo-deny root scope has 53 entries; workspace scope has 55 and uniquely adds `base64ct` plus the harness package. | CI SCA omits one real third-party workspace dependency | Add CI `--workspace` before final-head review | Yes — workflow wiring |
| F-3 | NON-BLOCKING | Legacy failing `src/gate.mjs` remains beside the Rust library. | Provenance/perception risk, not Rust execution risk | Founder disposition without source change, or later documented relocation | No content change; contextual effect of relocation |
| F-4 | NON-BLOCKING | Eleven `CONFORMANCE_*` constants remain public and documentation-hidden. | Conformance-only public surface remains consumer-reachable | Accept the documentation-hidden state or feature-gate later | Path move only; bytes unchanged |
| F-5 | NON-BLOCKING | Fresh `cargo test --workspace --all-targets --locked` output covers nine targets and totals 52 passed / 5 ignored. | Earlier local command-scope mismatch is resolved | No further correction | Evidence only |
| F-6 | NON-BLOCKING | `node --test FILE` reports one file unit; CI's direct `node FILE` reports seven cases. | Unexplained evidence-record mismatch | Accept the runner-specific result as submitted; mechanically clarify both invocation-specific counts in the review input before final-head review | Yes — PROD-001 review input |
| F-7 | NON-BLOCKING | Review input reproduces destination hashes from intermediate relocation commit `91589759...`, not reviewed head `f1d17087...`. | Founder may follow a command bound to the wrong commit | Add the reviewed-head-bound reproduction line before final-head review | Yes — PROD-001 review input |
| F-8 | NON-BLOCKING | PR #90 evidence establishes founder fallback approval, independent exact-head approval, founder merge, and merge-parent continuity. | Authorization prerequisite is resolved and externally evidenced | No further correction beyond retaining the permalinks | No |
| F-9 | NON-BLOCKING | Complete push/PR run, attempt, head, and job mappings are supplied; individual text logs do not embed `GITHUB_RUN_ID`. | Run traceability is resolved; only optional per-log self-attribution hygiene remains | Reviewer confirmed that the supplied JSON resolves F-9 and that embedded headers are optional non-blocking hygiene | No source change |
| A-1 | NON-BLOCKING | Semgrep `temp-dir` result in unchanged `tests/bypass-rust/tests/consumption_store.rs:19`. | Test-only temporary-directory behavior | Founder disposition without source change | No |
| A-2 | NON-BLOCKING | Nine CodeQL hard-coded nonce results in unchanged T3 fixtures. | Deterministic test data, not production secrets | Reaffirm the prior PROD-000 acceptance at the PROD-001 head | No |
| A-3 | NON-BLOCKING | CodeQL reports two raw diagnostics, but the processed artifact exposes only the generated `bindgen.rs` warning. | One diagnostic cannot be individually inspected | Human recommendation: allow as a bounded founder-gate evidence limitation; founder must expressly disposition the limitation or require recapture | No tracked source finding |
| A-4 | NON-BLOCKING | CodeQL Action v3 deprecates in December 2026; Node 20 actions were forced to Node 24. | Dated tooling maintenance | Migrate CodeQL separately before the deadline; no current extraction block | Workflow, but not caused by PROD-001 semantics |

F-1 through F-9 retain the meanings assigned by the cross-model review. A-1 through A-4 preserve
the human reviewer's separately stated analyzer judgments. The cargo-deny content originally placed
under F-9 was reassigned to F-2 mechanically. Gaziz Nugmanov subsequently confirmed, as relayed by
the founder and recorded at `2026-09-03T18:48:58Z`, that the complete push/PR run, attempt, head,
and job mappings resolve F-9 and that embedded `GITHUB_RUN_ID` values in individual text logs are
optional non-blocking hygiene.

## Scope confirmation

This review concludes that PR #91 at the reviewed head successfully separates an unpublished root library from its retained conformance harness while preserving reviewed isolation-harness behavior and provenance. It does not claim deployed hook installation, complete interception, agent non-bypassability, same-process store/key protection, deployed fail-closed behavior, or an ADR-14 runtime conclusion.

## Gate accounting

### Satisfied by this review

- [x] Independent-human review at the recorded implementation head, including the subsequent F-9
  traceability confirmation recorded above.

### Still pending

- [ ] Repository corrections and any required correction review.
- [ ] Founder byte/provenance/semantic-identity review.
- [ ] Founder disposition of every analyzer result and diagnostic.
- [ ] Required checks and analyzer legs on the actual final head.
- [ ] Independent GitHub approval of the actual final head.
- [ ] Founder exact-head approval or authorized fallback binding.
- [ ] Founder-only merge.

## Signed human attestation

I attest that I personally performed the review recorded above, that I authored or directed none of the PROD-001 implementation or evidence, that my conclusions are independent, and that I have not treated prior model or founder conclusions as substitutes for my own inspection.

- Name: Gaziz Nugmanov
- Signature/name: Gaziz Nugmanov
- Stable submitted-source reference: SHA-256
  `3c4e761b5afc04a1f72cc8ee9e8c48d6707675adacaa09d939207b972a65feaf`
- Decision time (UTC, as submitted): `2026-09-03T18:37:35Z`
- F-9 confirmation (founder-relayed, recorded UTC): `2026-09-03T18:48:58Z`

**Final verdict:** `PASS WITH NON-BLOCKING FINDINGS — INDEPENDENT-HUMAN GATE SATISFIED`
