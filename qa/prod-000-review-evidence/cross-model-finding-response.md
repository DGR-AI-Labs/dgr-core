# PROD-000 response to the first non-author cross-model review

The original review at `qa/prod-000-cross-model-review.md` is preserved byte-for-byte as the audit
record for implementation commit `40b713039a5612831df415cdd785271a7342be74`. Its verdict remains
`CHANGES REQUIRED`; this response does not overwrite or reinterpret that verdict. Claude must issue
a separate addendum after reviewing the remediation bundle.

## Binding

- Baseline: `e9c8f585809c15d2464b3d45bc2ce26d716c8673`.
- First reviewed implementation: `40b713039a5612831df415cdd785271a7342be74`.
- Original evidence head: `61906108a5c2b2aaafd4bd8c1ed7d62dc903a4a5`.
- Original Claude review record: `9a8082256fcff28babec07751762b33388bc4fe5`.
- Replacement/remediation source commit: `b19f33ae16698a81b993e6cc5a751360b6109577`.
- T3 active-assertion guard commit: `587585cf476431f078efe587c5dbcc052389cdad`.
- PR: `DGR-AI-Labs/dgr-core#90`.

## Blocking findings

| Finding | Disposition and evidence |
|---|---|
| B1 — incomplete review inputs | Remediated. The bundle contains complete baseline and current repository snapshots, the full binary/full-index patch, four focused stored diffs, PR/commit metadata, and SHA-256 manifests. |
| B2 — evidence not bound to head | Remediated for the replacement source. Fresh tests and three-engine evidence bind to `b19f33a...`. The only subsequent executable changes are the two T3 JavaScript guard files at `587585c...`; a stored drift report proves no Rust, Cargo, lockfile, deny policy, workflow, or other script changed after the scan. The bundle carrier and future review record are documentation/artifact-only descendants and must carry equivalent drift proof. |
| B3 — `founder_*` filename and wildcard | The rename request is rejected because active ADR-13 Amendment B section B2 mandates the exact filename `founder_before_tool_call_floor.rs`. The repository's `{AGENT-AUTHORS}` marker and `T0-AUTHORS.md` explicitly classify it as agent-authored T0. The defective wildcard command was removed: `eight-consumer.diff` names exactly the eight existing consumers and excludes the new module. |
| B4 — eight-consumer wording | Remediated. `T0-AUTHORS.md` and the ledgers now say all eight have module-path rewrites, seven have no other semantic change, and `founder_approval_timeout.rs` also contains the separately classified R5.1 change. |
| B5 — assertion provenance | Remediated. The equality assertion was added inside the pre-existing `atk_06_sequence_is_escalated_then_registry_derived_timeout_block` test function. The test file and its baseline/current versions are in the bundle. |
| B6 — canonical direction / droppable assertion | Remediated. `ATK_06_TIMEOUT_OUTCOME` is explicitly authoritative T0 policy and CORE-001 is the T3 conformance mirror. Commit `587585c...` makes deletion or `#[ignore]` of the equality test fail the required enumeration guard; the guard unit tests exercise present, missing, and ignored states. |
| B7 — timeout source absent | Remediated. Baseline/current repository snapshots and `r5-1-timeout.diff` contain both revisions and the isolated semantic hunk. The apparent larger raw diff is LF normalization; the focused diff uses `--ignore-space-at-eol`. |
| B8 — baseline floor absent | Remediated. Both snapshots and `floor-semantic-identity.diff` contain the baseline/current files and exact region evidence. |
| B9 — type classification tension | Remediated. The five shapes are recorded as pre-existing non-founder T0-by-consequence, then agent-relocated/transformed under Amendment B. No narrower founder classification is inferred. |

## Non-blocking findings

| Finding | Disposition and evidence |
|---|---|
| N1 — `GuardFault` observation | Retained and documented as the ATK-07 negative-conformance sentinel. Active tests reject observing the raw-fault shape; the product floor still collapses every fault to fail-closed. |
| N2 — public floor re-export | Remediated. The T3 facade imports the floor privately and publicly re-exports only the shared types. |
| N3 — route-around non-claim | Remediated in the extractable T0 source documentation. |
| N4 — `{AGENT-AUTHORS}` | Retained as the repository's literal authorship convention; `T0-AUTHORS.md` makes its authority and the filename exception explicit. |
| N5 — thin Semgrep ruleset | Recorded, not treated as a false negative or silently expanded. `p/rust` remains one leg of the required three-engine gate; custom repository rules are a separate hardening proposal and not an Amendment-B prerequisite. |
| N6 — SARIF self-binding | The SARIF format still lacks complete commit provenance. The adjacent wrapper, exact commit/tree, stored diff, and bundle-wide SHA-256 manifest supply the binding; no claim is made that SARIF alone is self-authenticating. |
| N7 — diagnostic array | Preserved and called out explicitly. Human and founder instructions require review of the complete SARIF notification array, including the generated dependency-output warning. |
| N8 — JavaScript scanner coverage | Explicitly bounded. The three-engine evidence is Rust-only. The changed T3 JavaScript guard receives syntax, unit, and live libtest-enumeration checks, but no SAST coverage is claimed for JavaScript. |
| N9 — `src/gate.mjs` | Recorded in `T0-BOUNDARY.md` as a deliberately failing Phase-0 scaffold, not an active or authoritative enforcement floor. |
| N10 — Blocked line range | Corrected from 63–67 to 64–67. |
| N11 — unstored diff command | Remediated with `eight-consumer.diff` and three additional digest-bound focused/full diffs. |
| N12 — duplicate cargo-deny artifact | Clarified as one earlier run. The remediation bundle contains one fresh cargo-deny record. |

## Required reviewer behavior

Claude must verify these dispositions against the supplied bytes. A claim marked “remediated” here
is an author response, not a closed finding. The addendum must independently accept, reject, or
reopen every B1–B9 and N1–N12 item and must not substitute for independent-human or founder review.
