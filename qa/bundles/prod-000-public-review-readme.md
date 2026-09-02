# PROD-000 sanitized public evidence package

This package supports the remaining independent-human and founder reviews for
`DGR-AI-Labs/dgr-core#90` without publishing internal ADR texts.

Start with `MANIFEST.sha256`, then read:

1. `review/prod-000-human-founder-review-instructions.md`;
2. `review/prod-000-human-founder-review-manifest.md`;
3. the appropriate unsigned form in `review/`; and
4. the complete `metadata/baseline-to-executable.diff` and relevant source under
   `dgr-core/review-source/`.

The exact executable review input remains
`587585cf476431f078efe587c5dbcc052389cdad`, with tree
`89e5de51d23ead98d24bbbe1b4cd57db343b2dc4`. Later public files are review,
evidence, or instruction records and do not change that executable input.

## Authority handling

The package deliberately excludes the `dgr-internal` repository and the bodies
of ADR-13 and Amendments A/B. Authorized reviewers must consult the canonical
internal repository directly. `metadata/authorities.sha256` provides identity
digests only; it is not a substitute for the authority text.

Do not add internal ADR copies to this package, PR #90, GitHub comments, or any
other public artifact.
