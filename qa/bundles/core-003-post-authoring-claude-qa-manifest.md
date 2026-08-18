# CORE-003 post-authoring Claude QA bundle manifest

**Bundle date:** 2026-08-18

## Review identity

- Founder-authored working-tree branch:
  `codex/core-003-t0-founder-authoring`
- Baseline dgr-core commit:
  `4c7f6a33a5f0c01c42eed81b936a77450c8edd40`
- Complete binary-capable patch SHA-256:
  `d689cfe6fc092b7cac1fcfea397e09288a169aa2be14c94583cff57eedc905d9`
- dgr-backlog reference commit:
  `a92df7eedd782a666f16f2d2e2a66541841699ff`
- Locally pinned dgr-internal reference commit:
  `1ec7059bc34d0a962e569aac3acf7785aa509d69`

## Bundle layout

- `QA-PROMPT.md` — independent review instructions and verdict contract.
- `STATE.md` — authored behavior, verification summary, and non-claims.
- `SNAPSHOT.txt` — branch, baseline, status, changed-file hashes, and patch
  identity.
- `AUTHORING.patch` — complete `git diff --full-index --binary` from baseline.
- `VALIDATION.txt` — raw formatting, Clippy, test, structure, and whitespace
  command transcript.
- `INPUT-DESIGN-REVIEW.txt` — prior design review against the merged tests.
- `INPUT-PREAUTHORING-READINESS.txt` — Claude's final readiness instruction
  before founder authoring.
- `dgr-core/` — full changed files plus the necessary surrounding source,
  tests, constitution, CI, attack specification, and Cargo dependency lock.
- `dgr-backlog/` — CORE-003 parent/child source records and generated view.
- `dgr-internal/` — exact locally pinned architecture, threat, attack, and
  verifier references.
- `BUNDLE-SHA256SUMS.txt` — SHA-256 of every other regular file in the archive.

## Evidence boundary

This is a cross-model review input for an uncommitted founder-authored snapshot.
It is not final exact-commit SAST evidence or founder approval. If the snapshot
changes after this bundle, the patch digest and bundle are stale and must not be
used to approve the changed tree.
