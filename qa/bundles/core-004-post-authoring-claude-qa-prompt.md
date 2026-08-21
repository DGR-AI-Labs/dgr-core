# Claude QA prompt — CORE-004 post-authoring T0 review

Perform an independent, source-grounded QA of the CORE-004 timeout-only 6-A
implementation using only this bundle. This is review work: do not author or
silently repair founder-owned enforcement, infer a human approval, or broaden
the bounded isolation claim.

## Integrity first

1. Verify `MANIFEST.sha256` from the bundle root before relying on any file.
2. Confirm `metadata/commit-lineage.txt` identifies:
   - baseline `7324cbb33be59595657a2df13c300aa388208d77`;
   - reviewed implementation/test commit
     `60febb08ac9c3e207d6f7a3563b6824374c5c93e`;
   - evidence/template commit
     `0807e34d91bd853620afc63879c44c15df8425ea`.
3. Recompute the baseline-to-reviewed binary full-index patch digest and
   require
   `71f051e24055cb0febd620d84a2703ea43d7277f5af4266feef8d03d0fbb9f1f`.
4. Confirm `metadata/post-review-drift.txt` shows no Rust, Cargo manifest,
   lockfile, or `deny.toml` change after the reviewed commit.
5. Verify the two pinned private design blobs match the SHA-256 values in
   `specs/CORE-004-reference-contract.md`.

## Required source review

Review the complete eleven-file implementation patch and determine whether:

1. signature, lifetime/expiry, and action binding precede canonical amount
   validation and the escalation trigger;
2. canonical amount comparison is strictly above `1_000_000` and cannot
   overflow an arbitrarily long unsigned decimal into Allow;
3. review ID derivation is agent-independent and domain-separated over the
   verified key ID, nonce, and action commitment;
4. checked deadline creation, record-before-observe ordering, and return before
   nonce consumption are correct;
5. first-write and `AlreadyPending` store results cannot substitute identity,
   commitment, or extend the original deadline without failing closed;
6. the STRICT SQLite schema and transaction boundaries preserve fixed shapes,
   deduplication, committed observation, and atomic timeout transition;
7. the token-free R-3 evaluator returns the unchanged escalation for
   `now <= deadline`, and only a committed `now > deadline` transition returns
   the registry-derived ATK-06 terminal denial;
8. NotFound, operational fault, arithmetic/conversion failure, malformed row,
   and identity mismatch have no Allow or fresh-escalation route;
9. `Escalate` never issues authorization or reaches the effectful probe; and
10. the implementation contains no approve-to-Allow path.

## Tests and regression review

Confirm all five dedicated CORE-004 tests are active and actually exercise the
real founder guard, SQLite approval store, and token-free evaluator. Confirm
they prove the ordered two-surface sequence, strict timeout boundary,
re-presentation ID/deadline stability, unconsumed nonce, zero effects, and the
unchanged below-threshold consume/Allow path. Reject any generic terminal
no-token block as a substitute for the two-surface proof.

## SAST/SCA review

Inspect raw evidence, not only the summaries:

- Semgrep 1.173.0: 20/20 files, one INFO temp-dir result.
- CodeQL 2.25.5 / Rust queries 0.1.35: 20/20 files, nine deterministic fixture
  nonce findings, zero extraction/execution errors, and seven path-resolution
  inconsistency diagnostics.
- cargo-deny 0.20.2: exit 0, no warning/error; two documented duplicate-version
  skips and an empty advisory-ignore list.

State whether each suggested founder disposition in
`qa/core-004-t0-founder-review-draft.md` is adequately bounded. Flag any
finding that touches founder-owned CORE-004 code or requires remediation.

## Human/process boundary

The independent-human and founder forms are deliberately unsigned. Do not
mark either gate complete or treat suggested wording as an actual decision.
Assess whether the forms ask the right review questions and identify any
missing human confirmation.

The canonical backlog snapshot may still show CORE-004 work in progress. Do
not recommend Done until implementation, evidence, independent-human review,
cross-model disposition, founder sign-off, PR approval, and merge are all
recorded.

## Scope guardrail

CORE-004 proves only the timeout-only 6-A contract for one guard instance and
its durable-local pending store under an injected modeled clock. It does not
prove real human delivery/waiting, approve-to-Allow, cross-instance state,
deployed-runtime route-around resistance, or live non-bypassability. Those
claims remain deferred, including RUNTIME-006 and the future ATK-05 reuse.

## Required response

Return:

1. **Verdict:** exactly one of `CONFIRMED — READY FOR HUMAN GATES`,
   `CONDITIONAL — ACTIONS REQUIRED`, or `BLOCKED`.
2. **Integrity:** manifest, commits, patch digest, drift, and design hashes.
3. **Source correctness:** every required source-review item above.
4. **Tests:** whether the active conformance proves the frozen contract without
   weakening expectations.
5. **SAST/SCA:** every finding and diagnostic, with disposition adequacy.
6. **Human/process:** whether the unsigned forms are complete enough for real
   human review and founder decision.
7. **Backlog/scope:** accurate current state and preserved deferrals.
8. **Findings/actions:** every defect, ambiguity, or confirmation required;
   write `None` only if there truly are none beyond the intentionally pending
   human gates.

Do not claim CORE-004 Done or authorize merge. A confirmed verdict means only
that the exact source/evidence package is ready for the required human gates.
