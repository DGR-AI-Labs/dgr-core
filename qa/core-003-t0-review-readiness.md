# CORE-003 T0 review readiness

**Status:** Ready for founder/human disposition; not yet founder-approved
**Code commit:** `6cb6826fb29ee18bd2ce5f596c620f4170f37a47`
**Baseline:** `4c7f6a33a5f0c01c42eed81b936a77450c8edd40`
**Reviewed patch SHA-256:**
`d689cfe6fc092b7cac1fcfea397e09288a169aa2be14c94583cff57eedc905d9`

## Completed evidence

- Exact founder-authored code snapshot committed without byte changes.
- Baseline-to-commit patch digest matches the cross-model-reviewed digest.
- Formatting, Clippy with warnings denied, all-target tests, governance
  structure, and whitespace checks pass.
- Test result: 39 passed; 6 unrelated deferred/hosted cases ignored; no ATK-07
  test ignored.
- Claude source-verified cross-model review: PASS, no code defect.
- Semgrep: complete 14-of-14 Rust coverage; one known test-only finding; no scan
  errors.
- CodeQL: complete 14-of-14 Rust extraction; seven known deterministic fixture
  findings; no extraction or execution errors.
- cargo-deny: exit 0; no advisory, license, ban, or source error/warning.

## Evidence files

- `qa/core-003-t0-cross-model-review.md`
- `qa/core-003-t0-founder-review-draft.md`
- `qa/sast/core-003-t0-semgrep-2026-08-18.txt`
- `qa/sast/core-003-t0-semgrep-2026-08-18.json`
- `qa/sast/core-003-t0-codeql-2026-08-18.txt`
- `qa/sast/core-003-t0-codeql-2026-08-18.sarif`
- `qa/sast/core-003-t0-cargo-deny-2026-08-18.txt`

## Pending human actions

1. Founder completes every applicable checkbox and all disposition fields in
   `qa/core-003-t0-founder-review-draft.md`.
2. Founder records an explicit review decision for each scanner's raw output.
3. Founder selects the final T0 disposition and signs it.
4. PR receives the required human approval.
5. Founder/human merges only if the PR code commit remains
   `6cb6826fb29ee18bd2ce5f596c620f4170f37a47` and no T0 code changes are added.

## Change invalidation rule

Any change to the founder-owned boundary or its adversarial expectations after
commit `6cb6826f...` invalidates the affected review and scanner binding. Repeat
the relevant tests, scanners, cross-model review, and founder disposition on
the replacement exact commit before merge.
