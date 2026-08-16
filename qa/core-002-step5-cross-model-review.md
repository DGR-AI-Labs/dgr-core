# CORE-002 Step 5 cross-model review record

**Review date:** 2026-08-16  
**Review type:** independent cross-model design and correctness review  
**Disposition:** PASS WITH ERRATA  
**Reviewed dgr-core commit:** `0727e327631b475990ef8d9b7ef3b2c3554050a8`  
**Reviewed bundle:** `dgr-core-002-step5-claude-qa-2026-08-16.zip`  
**Reviewed bundle SHA-256:** `74fad508dbdfe05d142af93ad610043157bddb8b2edd1b7085eb91367511be49`

## Scope and result

The review found no Step 5 code defect. It confirmed that ATK-03 uses one
persistent store and the same request for two presentations, permits the first
presentation exactly once, denies the replay, derives the denial outcome from
the attack registry, and records zero effectful invocations on the replay. It
also accepted the signature, temporal, typed request-binding, consumption,
and persist-before-allow ordering.

The currently active CORE-002 conformance cases are
ATK-01/02/03/08/09/10/11/13. ATK-04/05/06/07/12/14 remain explicitly
deferred, and ATK-15 remains an external hosted-IAM assertion.

## Errata to the initial review narrative

The following initial statements were retracted and are superseded by
repository evidence:

- `S2ConsumptionStore::open_at(path)` is present in the reviewed commit and
  bundle. File-backed restart and concurrent same-file regressions exercise
  it, so restart durability on reopening the same SQLite file is backed by
  code and tests.
- The code/design review PASS does not mean CORE-002 or Step 5 is complete.
  Required SAST, founder/human, PR, and merge gates remain.
- Merging Step 5 would not make the complete bypass suite green. Deferred
  cases and CORE-003/004/005 remain separate work.
- Runtime integration does not activate on CORE-002 merge. Its canonical
  trigger remains CORE-005 Done with the required ATK-01..14 evidence.
- Step 5 was founder-authored but was not pushed or merged when reviewed.
- The demonstrated claim is limited to a developer-grade, operator-bypassable
  conformance harness. Live-runtime agent non-bypassability is not proven.

## Evidence locations

- `tests/bypass-rust/src/founder_s2_consumption_store.rs` — file-backed and
  in-memory constructors plus the atomic insert outcome mapping.
- `tests/bypass-rust/tests/attack_set.rs` — ATK-03 and ATK-13 observations.
- `tests/bypass-rust/tests/consumption_store.rs` — restart and concurrent
  same-file regressions.
- `qa/core-002-step5-review-readiness.md` — test and local verification record.

## Gates not closed by this review

This disposition closes only the cross-model review evidence requirement. It
does not replace:

1. results from at least three independent SAST tools;
2. founder/human review of the protected checklist;
3. PR approval and merge; or
4. final backlog completion evidence with stable PR and commit links.

`CORE-002-STEP5` must remain `In Review` until those gates are satisfied.
