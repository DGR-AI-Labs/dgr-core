# PROD-001 post-review follow-up

**Status:** reviewed mechanical corrections applied; founder and final-head gates pending

This record connects the completed non-author cross-model and independent-human reviews to the
mechanical correction commit that follows reviewed implementation head
`f1d17087d140b41750c1aeca032916bb4d2d90ae`. It is evidence, not founder disposition or merge
authorization.

## Review records

- `qa/prod-001-cross-model-review.md` — SHA-256
  `e05a1e3864d181a723eaf9769f51904b90825b2681bcc065216c477c917b6a06`.
- `qa/prod-001-cross-model-review-addendum.md` — SHA-256
  `607716840944eb2507c64874a16e0e6773b00131d25ad2e68dd1d71fa67d7825`.
- `qa/prod-001-independent-human-review.md` — SHA-256
  `7183229f62355a4d06d7a2177654d74b62542e9b3bfccdd5ce324522b6a83c54`.

The independent-human record preserves Gaziz Nugmanov's submitted source digest and identifies all
Codex mechanical normalization. Gaziz subsequently confirmed F-9: complete run/job JSON mappings
resolve traceability, and embedded `GITHUB_RUN_ID` text-log headers are optional non-blocking
hygiene.

## Applied mechanical corrections

1. `.github/workflows/ci.yml` adds `--workspace` to cargo-deny so the informational SCA leg includes
   both workspace members and the previously omitted `base64ct 1.8.3` dependency.
2. `qa/prod-001-extraction-review-input.md` records both truthful Node presentations: local
   `node --test FILE` reports one top-level file unit, while CI's direct `node FILE` reports all
   seven test cases.
3. The byte-identity reproduction block adds the cross-model-reviewed head `f1d17087...` alongside
   the baseline and intermediate relocation commit.
4. The public PR #90 prerequisite section now cites the founder exact-head fallback approval,
   independent exact-head approval, founder merge, and merge-parent continuity.
5. The review checklist now records the completed cross-model and independent-human gates.

These changes modify workflow/evidence wiring and review records only. They do not modify any Rust
enforcement body, shared data type, harness source, test expectation, ignored set, Cargo manifest,
lockfile, dependency version, or package public item.

## Founder decisions still required

The founder must independently review and disposition:

- the single unchanged Semgrep test-only temporary-directory result;
- the nine unchanged CodeQL deterministic fixture-nonce results at the PROD-001 head;
- both CodeQL raw diagnostics, including whether the unenumerable second message may be accepted as
  a bounded evidence limitation or requires recapture;
- cargo-deny's two bans notes and the complete 55-entry workspace license set after the corrected
  CI run;
- the legacy failing `src/gate.mjs` location;
- the eleven documentation-hidden public `CONFORMANCE_*` items;
- CodeQL Action v3's December 2026 migration deadline and Node 20-to-24 forcing; and
- every cross-model F-1 through F-9 and human A-1 through A-4 recommendation.

## Final-head rule

The correction commit will differ from `f1d17087...` only by the recorded QA files and reviewed
workflow correction. Required checks and all three analyzer legs must run again on that actual
final candidate head. The founder must inspect this drift, bind the review and disposition to the
exact final head, and only then request independent GitHub approval and apply the authorized
founder approval mechanism. Founder-only merge remains last.
