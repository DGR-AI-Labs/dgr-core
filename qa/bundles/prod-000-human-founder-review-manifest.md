# PROD-000 sanitized public review bundle manifest — R2

## Identity

- Bundle date: 2026-09-02.
- Bundle revision: R2; supersedes the first sanitized public package.
- Repository/PR: `DGR-AI-Labs/dgr-core#90`.
- Classification: public; internal ADR authority texts are deliberately omitted.
- Baseline: `e9c8f585809c15d2464b3d45bc2ce26d716c8673`.
- Exact executable review input: `587585cf476431f078efe587c5dbcc052389cdad`.
- Exact executable tree: `89e5de51d23ead98d24bbbe1b4cd57db343b2dc4`.
- Safe evidence source head: `46da707b8b84dfa599c4e27e5fbb2dc005e9e0e4`.
- Safe evidence source tree: `16b5812cf3bdfce585a9e04da720ab99b0bcac29`.
- Baseline-to-executable patch SHA-256:
  `c08919d86a1f060cce9a05b3143140a5f011b9349f243475dad4f4ec1b40cf99`.
- Cross-model addendum SHA-256:
  `534ce4164067aef339b9f35a176de1b39e6f1573834e52cdc3d533fad7e634db`.

## Public bundle contents

- `dgr-core/baseline-critical/` — eight exact critical-file pre-images selected from the baseline
  commit. This is not a complete baseline snapshot.
- `dgr-core/review-source/` — public source, tests, workflow inputs, scanner outputs,
  provenance templates, and review records selected from the safe evidence source head. This is
  not a complete Git-tree reconstruction. The current unsigned forms ship only under `review/` to
  avoid stale duplicates.
- `metadata/baseline-to-executable.diff` — complete binary/full-index review patch.
- `metadata/r5-1-timeout.diff` and `metadata/r5-1-timeout-semantic.diff` — raw and
  EOL-insensitive founder-file evidence.
- `metadata/commit-tree-identities.txt` — public commit-to-tree bindings and selection limits.
- `metadata/selected-file-inventory.sha256` — digest inventory for every selected review-source
  file.
- `metadata/post-executable-drift.name-status` and
  `metadata/post-executable-drift-assessment.md` — stored drift through the safe evidence source and
  its scope assessment.
- `metadata/original-bundle-path-map.md` — mapping from the immutable Claude record's private-bundle
  paths to this sanitized layout.
- `metadata/critical-baseline.sha256`, `metadata/critical-executable.sha256`,
  `metadata/cross-model-records.sha256`, and `metadata/canonical-scanner-artifacts.sha256` —
  bundle-local verifiers whose paths must all resolve from the package root.
- `metadata/external-authorities.sha256` — identity digests that intentionally resolve only against
  an authorized canonical `dgr-internal` checkout.
- `review/` — remaining-gate instructions plus independent-human and founder input forms.
- `MANIFEST.sha256` — SHA-256 over every other bundled file.

## Deliberate exclusions

This public package contains no `dgr-internal` repository snapshot, no internal ADR body, and no
nested archive. The complete bundles containing ADR-13 and active Amendments A/B are retained only
in the internal governance repository. An authorized reviewer who must compare implementation to
the full authority text must obtain that text from `dgr-internal`; it must not be copied back into
this repository, a GitHub comment, or a public review artifact.

The public authority pointer and external hashes remain available for identity checking. Their
presence does not substitute for the authorized reviewer's direct access to the governed internal
source. The external authority sidecar is the only intentionally non-bundle-local verifier.

## Gate state

- Cross-model: satisfied with non-blocking findings; original and addendum both retained.
- N13: reduced semantic diff stored.
- N14: bounded explicitly; test-name/active-state guard is not assertion-body proof.
- N15: ledger line corrected to 10.
- Independent-human: pending.
- Founder dispositions and line-by-line review: pending.
- Final-head GitHub approval and founder-only merge: pending.
- PROD-001: unauthorized.
