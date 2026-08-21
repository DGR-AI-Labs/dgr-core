# CORE-004 post-authoring Claude QA bundle manifest

## Artifact identity

- **Bundle date:** 2026-08-21
- **Baseline commit:** `7324cbb33be59595657a2df13c300aa388208d77`
- **Reviewed implementation/test commit:**
  `60febb08ac9c3e207d6f7a3563b6824374c5c93e`
- **Reviewed tree:** `71ed21dcbd2f940c55b0e400f1f2071e628b074b`
- **Evidence/template commit:**
  `0807e34d91bd853620afc63879c44c15df8425ea`
- **Reviewed patch SHA-256:**
  `71f051e24055cb0febd620d84a2703ea43d7277f5af4266feef8d03d0fbb9f1f`
- **Pinned CORE-004-DESIGN SHA-256:**
  `a6b883eafa471bf797617e1716a65aafb2689ad993cabac42dd2d1aa3db85dab`
- **Pinned Addendum A SHA-256:**
  `ef5a19e99f0ad6753c9351f337d376542c4657c291dd00787dfa6e43d86bd514`

## Contents

- `dgr-core/` — the 20 reviewed Rust files, Cargo inputs, ownership/scope
  documents, frozen conformance/fixture evidence, unsigned human/founder forms,
  and raw three-engine SAST/SCA artifacts.
- `dgr-internal/` — pinned CORE-004 design and Addendum A plus the governing
  SRS/ADR/decision records available at the pinned reference lineage.
- `dgr-backlog/` — canonical CORE-004-family items from the recorded backlog
  main snapshot; in-progress states are intentional and must not be inferred
  complete.
- `metadata/` — exact commit lineage, reviewed patch, drift check, scanner
  hashes, validation summary, and reference hashes.
- `review/` — prompt, state, and this manifest.
- `MANIFEST.sha256` — SHA-256 for every other file in the bundle.

## Known findings requiring review

- Semgrep: one INFO temporary-directory construction in the existing
  consumption-store test helper.
- CodeQL: nine deterministic fixture-nonce results: seven VAL-002 and two
  VAL-004; plus seven Rust path-resolution inconsistency diagnostics with
  complete 20/20 extraction and zero extraction/execution errors.
- cargo-deny: exit 0; no error/warning; two documented duplicate-version notes
  and 54 accepted-license notes.

## Intentionally incomplete gates

- independent-human review is unsigned;
- founder SAST dispositions and final sign-off are unsigned;
- Claude's disposition does not exist until this bundle is reviewed;
- the branch is not human-approved or merged; and
- CORE-004 and its T0 child must not be marked Done.
