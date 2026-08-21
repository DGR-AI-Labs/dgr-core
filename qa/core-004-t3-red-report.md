# CORE-004 T3 RED conformance report

> Historical RED evidence. The reviewed founder implementation and mechanical
> T3 wiring turned this contract green at `60febb08ac9c3e207d6f7a3563b6824374c5c93e`;
> see `qa/core-004-t3-green-report.md`. The original RED facts below are retained
> as the pre-authoring audit record.

**Date:** 2026-08-20  
**Status:** In Review — founder approval required before T0 authoring  
**Base:** `0e462eb9dbfa8937aa64a80c978e560e5c0d2f32`  
**RED-test commit:** `a321397ec1cc79ba72db4efb053ad4be2f7c3d72`

## Preconditions

- dgr-internal PR #4, dgr-core PRs #78 and #79, and dgr-backlog PRs #15 and
  #16 are merged.
- `CORE-004-DESIGN` and Addendum A remain pinned by
  `specs/CORE-004-reference-contract.md`.
- Merged VAL-004 fixtures encode the founder-approved two-surface R-3 contract.
- ATK-06 remains registered as
  `RequiredOutcome::EscalateThenDenyOnTimeout` and remains ignored.

## Authored T3 surface

`tests/bypass-rust/tests/core_004_conformance.rs` adds:

1. an active below-threshold regression proving the existing `100000`
   consume/allow path remains unchanged;
2. an ignored RED above-threshold test that presents the valid `1000001`
   token to the real current adapter and requires zero effectful invocations;
3. an ignored RED re-presentation test requiring the same token to preserve
   the original pending facts and remain unconsumed;
4. an ignored RED ordered-sequence test pinning
   `[Escalated, BlockedFromRegistry { ATK-06 }]`; and
5. an ignored RED deadline test pinning `deadline - 1`, `deadline`, and
   `deadline + 1`, with the original review-request ID and deadline.

The test-local `FakeApprovalState` records only deterministic scenario
preconditions (`NoPendingRecord` or `Existing`) and the fact that approval is
unanswered. It does not implement a port, lookup, write, transition, timeout
decision, observation, or persistence behavior.

`tests/bypass-rust/tests/attack_set.rs` replaces the generic ATK-06 macro case
with an explicit ignored RED guardrail. A terminal no-token block can no longer
be mistaken for the required two-surface proof.

## Honest missing-surface boundary

The founder-owned `Escalated` observation variant, `ApprovalStore` port,
durable pending store, and R-3 timeout evaluator do not exist. T3 therefore
does not invent signatures for them.

- The first surface invokes the current real adapter. It fails RED because the
  current guard proceeds and invokes the probe once for the above-threshold
  token.
- The timeout tests validate every frozen input and expected observation, then
  stop at `founder_timeout_surface_pending`. That sentinel always panics and
  performs no decision. Once the founder freezes and authors the actual T0
  timeout API, T3 wiring must replace only this sentinel call with the real
  surface and make the typed observation assertions direct before activation.

This limitation is deliberate: any shared enum or trait encoding pending,
escalated, approved, or denied semantics is founder-owned T0.

## Validation

- format check: **PASS**;
- full all-target suite: **PASS — 47 passed, 10 ignored**;
- dedicated T3 target under normal CI semantics: **PASS — 1 passed, 4 ignored**;
- Clippy with `-D warnings`: **PASS**;
- Cargo manifest and lockfile: **unchanged**;
- founder-owned guard, adapter body, verifier, fail-closed mapping,
  consumption stores, and planned approval surfaces: **not edited**.

Explicit RED command:

```text
cargo test --manifest-path tests/bypass-rust/Cargo.toml \
  --test core_004_conformance --locked -- --ignored
```

Result: **exit 101; 0 passed, 4 failed**.

- escalation and re-presentation fail because the current path invokes the
  effectful probe once instead of escalating with zero invocations;
- sequence and boundary cases fail at the explicit absent-founder-timeout-
  evaluator sentinel.

## Founder review gate

Confirm that the reviewed RED contract requires all of the following:

- amount `1000001` escalates only after verification/binding/canonical
  validation and before nonce consumption;
- amount `100000` retains the existing consume/allow behavior;
- escalation and timeout are different surfaces;
- escalation and re-presentation issue no authorization and make zero
  effectful invocations;
- re-presentation returns the original review-request ID and deadline without
  consuming the nonce;
- `now <= deadline` re-observes the same escalation;
- only `now > deadline` produces the registry-derived terminal block; and
- no generic no-token denial is accepted as ATK-06 conformance.

After founder approval and merge, the next step is founder-authored
`CORE-004-T0-boundary`. ATK-06 must remain ignored until the founder behavior,
mechanical T3-to-T0 wiring, and full T0 review gate turn the reviewed contract
green without changing its registry expectation.
