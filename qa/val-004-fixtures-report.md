# VAL-004 deterministic fixture report

**Date:** 2026-08-20  
**Status:** In Review — founder fixture gate required  
**Base:** `052a355fbe449db9590ee3259794df3d1c4fc62c`  
**Fixture code commit:** `a02ddd0f585fde49f04038852becd4976ded612b`

## Preconditions verified

- `CORE-004-DESIGN` and `CORE-004-DESIGN-ADDENDUM-A` are active in the
  merged dgr-internal reference corpus.
- Addendum A source commit
  `e2649b0387ac7984a84bb8d6b6e16718c57dde8d` resolves with SHA-256
  `ef5a19e99f0ad6753c9351f337d376542c4657c291dd00787dfa6e43d86bd514`,
  matching `specs/CORE-004-reference-contract.md`.
- dgr-internal PR #4, dgr-core PR #78, and dgr-backlog PR #15 are merged.
- The unchanged pre-authoring baseline passed format, build, all-target tests,
  and Clippy with 39 passing tests and six ignored tests.

## Authored fixture surface

- `tests/bypass-rust/src/val_004_fixtures.rs` contains deterministic scenario
  data and expected labels only.
- `tests/bypass-rust/tests/val_004_fixtures.rs` validates fixture integrity,
  the genuine signed token, two-surface shape, boundary clocks, immutable
  pending facts, registry-derived terminal outcome, and unchanged control.
- `tests/bypass-rust/src/lib.rs` mechanically exports the module.
- `tests/bypass-rust/src/val_002_fixtures.rs` exposes one fixture-only helper
  that reuses the existing private registered test signer. This avoids copying
  signing logic or exposing the signing seed; it implements no verification or
  decision behavior.

The six deterministic cases are:

1. `escalation-required` — valid token bound to amount `1000001`;
2. `timeout-deadline-minus-one` — pending surface at deadline − 1;
3. `timeout-at-deadline` — pending surface exactly at deadline;
4. `timeout-deadline-plus-one` — registry-derived terminal block label;
5. `re-presentation-does-not-extend` — identical token and original pending
   ID/deadline with `AlreadyPending` and nonce-unconsumed expectations; and
6. `below-threshold-control` — byte-identical VAL-002 valid artifact at amount
   `100000`, following the existing consume/allow path.

`requested_at` is `1_800_000_000`; the fixture window is `86_400`; the immutable
deadline is `1_800_086_400`. Timeout cases never contain a capability token or
`before_tool_call` request.

## Founder constant mirror dependency

Founder T0 has not yet created
`CONFORMANCE_APPROVAL_WINDOW_SECONDS` or
`CONFORMANCE_APPROVAL_REQUIRED_ABOVE_MINOR_UNITS`. The fixture records a typed
pending assertion naming both symbols and their expected values. It does not
create or simulate those founder constants. The future dependency remains
test → T0.

## Validation

- exact VAL-004 fixture tests: **7 passed**;
- full all-target suite: **46 passed, 6 ignored**;
- format check: **passed**;
- build: **passed**;
- Clippy with `-D warnings`: **passed**;
- `ATK-06`: **still ignored**;
- existing VAL-002 tests: **12 passed unchanged**;
- Cargo and lockfiles: **unchanged**;
- founder-owned guard, verifier, fail-closed, consumption, approval, adapter,
  store, observation, and timeout-evaluation bodies: **not authored or edited**.

## Non-claims and next gate

This change does not implement escalation, pending persistence, timeout
evaluation, nonce behavior, an `Escalated` observation, or authorization. It
does not claim ATK-06 green.

The founder must review the amount boundary, deadline arithmetic, two-surface
shape, original ID/deadline facts, pending constant-mirror dependency, and
unchanged regression suite. After founder approval and merge, only the RED
`CORE-004-T3-tests` step may begin; founder T0 remains blocked behind that
reviewed RED-test gate.
