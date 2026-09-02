# PROD-000 sanitized public review bundle manifest

## Identity

- Bundle date: 2026-09-02.
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

- `dgr-core/review-source/` — public source, tests, workflow inputs, scanner outputs,
  provenance templates, review records, and unsigned human/founder forms selected from the safe
  evidence source head.
- `metadata/baseline-to-executable.diff` — complete binary/full-index review patch.
- `metadata/r5-1-timeout.diff` and `metadata/r5-1-timeout-semantic.diff` — raw and
  EOL-insensitive founder-file evidence.
- `metadata/` — commit/tree identities, selected-file inventory, drift, authority hashes, scanner
  hashes, and critical hashes.
- `review/` — remaining-gate instructions plus independent-human and founder input forms.
- `MANIFEST.sha256` — SHA-256 over every other bundled file.

## Deliberate exclusions

This public package contains no `dgr-internal` repository snapshot, no internal ADR body, and no
nested archive. The complete bundles containing ADR-13 and active Amendments A/B are retained only
in the internal governance repository. An authorized reviewer who must compare implementation to
the full authority text must obtain that text from `dgr-internal`; it must not be copied back into
this repository, a GitHub comment, or a public review artifact.

The public authority pointer and recorded hashes remain available for identity checking. Their
presence does not substitute for the authorized reviewer's direct access to the governed internal
source.

## Gate state

- Cross-model: satisfied with non-blocking findings; original and addendum both retained.
- N13: reduced semantic diff stored.
- N14: bounded explicitly; test-name/active-state guard is not assertion-body proof.
- N15: ledger line corrected to 10.
- Independent-human: pending.
- Founder dispositions and line-by-line review: pending.
- Final-head GitHub approval and founder-only merge: pending.
- PROD-001: unauthorized.
