# T0 authorship boundary

This file is the ownership map for CORE-002, CORE-003, and CORE-004. The binding repository
constitution classifies every consequential authorization path as T0 and human-led. The sole
current agent-authorship exception is the bounded PROD-000 change authorized by ADR-13 Amendment B
and described below; it does not reclassify T0 or alter historical authorship.

The CORE-003 boundary contract is recorded in
`tests/bypass-rust/T0-BOUNDARY.md` and the ATK-07 section of
`specs/CORE-001-bypass-attack-set.md`.

The founder-confirmed CORE-004 contract and Addendum A are pinned by
`specs/CORE-004-reference-contract.md`. Together they select the R-3
two-surface model: token-bearing escalation at `before_tool_call`, followed by
a distinct trusted-clock evaluation of the durable pending record. Addendum A
also freezes the bound-amount trigger, pre-deadline re-observation, placement
before nonce consumption, and ATK-05 path-reuse boundary.

The normative contracts consumed by the founder implementation are listed in
`specs/CORE-002-reference-contracts.md`. That file points to the pinned
`dgr-internal` reference records; it does not duplicate or redefine them.

## Founder-authored units

The founder authored the existing function bodies named here. That historical fact remains true
until a reviewed PROD-000 commit relocates or transforms an identified region:

| File | Founder-only function | Required responsibility |
|---|---|---|
| `src/founder_authored_guard.rs` | `FounderAuthoredGuard::decide` | Guard decision for the intercepted `before_tool_call` request |
| `src/founder_token_verification.rs` | `verify_capability_token` | Capability-token verification |
| `src/founder_fail_closed.rs` | `fail_closed_decision` | Deny behavior for absence, invalidity, unavailability, or internal error |
| `src/founder_s2_consumption_store.rs` | `S2ConsumptionStore::consume` | Durable-local, atomic single-use consumption before allow |
| `src/founder_consumption_store.rs` | `ConsumptionStore::consume` | Store boundary retained for S2 now and S3 later |
| Historical source at `tests/bypass-rust/src/before_tool_call.rs` | Original `BeforeToolCallAdapter::before_tool_call` body | Founder-authored source provenance for the reached-boundary fault/unwind floor transformed by PROD-000 |

## CORE-004 founder-owned surfaces

The following implemented surfaces retain founder-authored T0 provenance. Their names record
ownership and responsibility and do not authorize later agent changes outside Amendment B. The
R5.1 timeout constant/dependency reversal and the eight module-path rewrites at `40b7130...` are
separately classified as agent-authored changes pending founder review.

| Surface/location | Founder-owned responsibility |
|---|---|
| `src/founder_approval_store.rs` | The `ApprovalStore` port; original-id/deadline `AlreadyPending` behavior; and every consequential pending/not-found/timed-out/fault outcome |
| `src/founder_s2_approval_store.rs` | Durable-local SQLite record, deduplication, lookup, and timeout-transition behavior, including deadline immutability and persist-then-observe |
| `src/founder_authored_guard.rs` | Canonical amount validation; founder threshold and conformance mirrors; escalation after binding and before nonce consumption; and timeout decisions |
| R-3 timeout-evaluation path (exact function/location selected by the founder during authoring) | Evaluate a pending record against the trusted injected clock without token re-presentation; return the same `Escalated` id/deadline while `now <= deadline`; persist timeout before returning the terminal block when `now > deadline` |
| `tests/bypass-rust/src/before_tool_call.rs` adapter behavior | Emit `Escalated` only after durable persistence and guarantee no authorization, nonce consumption, or effectful invocation on that path |

Any shared enum or trait encoding consequential pending, escalated, approved,
or denied semantics is T0 until the founder records a narrower classification.

## Current implementation state

The CORE-002 units named above, the historical CORE-003
`BeforeToolCallAdapter::before_tool_call` floor source, and the CORE-004 surfaces
contain founder-authored T0 enforcement provenance. Their applicable pre-PROD-000 T0 gates are
complete: CORE-002 Step 5 was accepted through PRs #68/#70 and
`qa/core-002-step5-governance-disposition.md`; CORE-003 through PR #73 and the
signed records indexed by `qa/core-003-t0-review-readiness.md`; and CORE-004
through PRs #81/#82 and the signed founder and independent-human dispositions
under `qa/`. This completed state does not relax the ownership boundary or
claim that deferred attacks are implemented. The default `ConsumptionStore`
implementation still returns `FounderImplementationRequired` explicitly so an
absent concrete store fails closed. The S2 unit exposes an in-memory constructor
for isolated conformance tests and a file-backed constructor for
restart-durable local consumption.

This state record does not relax the authorship boundary. PROD-001 may relocate the reviewed T0
files byte-identically and change only approved crate/module/Cargo wiring. An agent must not replace,
complete, refactor, or route around any enforcement body.

## PROD-000 Amendment-B exception — implementation checkpoint

The Amendment-B pointer/templates and canonical backlog update are merged. The founder explicitly
authorized discarding the pre-existing zero-byte placeholder (SHA-256
`e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`). OpenAI Codex authored the
bounded PROD-000 implementation checkpoint at
`40b713039a5612831df415cdd785271a7342be74`. Its scope is:

- the new `founder_before_tool_call_floor.rs` boundary module and its `lib.rs` registration;
- Amendment A R5.1's timeout constant, conformance mirror, and now-impossible missing-row removal;
- module-path rewrites in exactly eight founder-owned consumers named by Amendment B; seven files
  contain no other semantic change, while `founder_approval_timeout.rs` separately contains the
  Amendment-A R5.1 change already listed above;
- removal of the relocated T0 definitions and old floor from the mixed T3 file;
- the T3 facade, adapter conversion/probe invocation, and registry-mirror assertion; and
- ownership, boundary, preparation, review, and evidence documentation.

Every changed region must be classified as existing founder source, agent-relocated founder source,
agent-authored T0, or T3. Founder review, approval, or merge must be recorded separately and must
never be presented as founder authorship of agent-written or agent-transformed lines. Any change
outside the enumerated scope is a hard stop requiring a new founder decision.

At the checkpoint, `founder_before_tool_call_floor.rs` is agent-authored T0. Amendment B requires
this exact filename; the `{AGENT-AUTHORS}` marker and this ownership record are authoritative and
prevent the `founder_` path prefix from being misread as an authorship claim. Its transformed
fault/unwind floor retains identified founder-source provenance. The five request/decision/fault/
port shapes pre-date PROD-000 and were non-founder T0-by-consequence even while physically located
in the mixed-tier adapter; their relocation and documentation changes are agent-authored T0
transformations. `BeforeToolCallOutcome`, the R5.1 constant/control-flow change, and all module-path
rewrites were also authored by the agent. This checkpoint is not founder-approved or merge-ready
until the exact-commit review stack is complete and every finding is founder-dispositioned.

The first non-author cross-model review at `qa/prod-000-cross-model-review.md` returned
`CHANGES REQUIRED`. Replacement commit `b19f33ae16698a81b993e6cc5a751360b6109577` addresses its
source, classification, and evidence findings without expanding the Amendment-B T0 scope. Commit
`587585cf476431f078efe587c5dbcc052389cdad` then changes only the T3 enumeration guard and its unit
tests so deletion, rename, or ignoring of the named ATK-06 T0/registry equality test fails required
CI. This guard proves test presence and active status; it does not prove the assertion body remains
unchanged, which stays in source/human review scope. The full PROD-000 gate was completed at final
head `a85e3676367978d5964f0be29e802e8d51f4ed24` and founder-merged through PR #90 as merge commit
`8318f61eadf689f9b8a72f673cc68cd083dc7831`.

## PROD-001 extraction candidate

PROD-001 moves the nine reviewed T0 files from `tests/bypass-rust/src/` to the root `src/` library
without changing their bytes. The shared `RequiredOutcome`, `ProposedAction`, and
`DecisionContext` definitions are relocated from the former mixed harness crate root to the root
library; the harness re-exports the library definitions for unchanged conformance imports. The
attack registry, fixtures, adapter, observations, and tests remain T3 under `tests/bypass-rust/`.

The extraction commit is agent-assisted structural work, not authorship of the moved enforcement
bodies. Existing founder-authored source retains founder provenance; PROD-000 agent-authored and
agent-transformed T0 retains that provenance; the PROD-001 commit is a relocation/wiring commit
pending byte-identity review and founder merge. Extraction changes distributability and does not
expand the bounded isolation claim.

## Agent-authored supporting units

The following are outside the founder implementation surface:

- `tests/bypass-rust/src/before_tool_call.rs`: the T3 compatibility facade, observation types,
  adapter conversion, and mechanical probe plumbing;
- `tests/bypass-rust/src/fixtures.rs`: opaque no-token, valid-candidate,
  expired, replayed, forged, and out-of-scope fixture bytes;
- `tests/bypass-rust/tests/adapter_harness.rs`: adapter-plumbing tests using
  scripted decisions; and
- `tests/bypass-rust/tests/attack_set.rs`: conformance expectations, including
  active CORE-002 checks for ATK-01/02/03/08/09/10/11/13, active CORE-003
  checks for ATK-07, and explicitly deferred cases.

The following supporting surfaces are T3 and were authored in their recorded
backlog order:

- VAL-004 fixture data: valid above-threshold and below-threshold actions,
  deterministic review-request IDs, requested/deadline facts, clocks at
  `deadline - 1`, `deadline`, and `deadline + 1`, re-presentation facts, and
  the no-approval scenario;
- a deterministic or fake approval store used solely by tests; and
- conformance tests, initially reviewed RED and activated only after founder
  implementation, asserting the ordered
  `[Escalated, Blocked { ... }]` sequence with the registry-derived ATK-06
  outcome and zero effectful invocations.

Those support units may represent frozen facts and expectations. They must not
calculate policy, extend a deadline, implement persistence semantics, emit a
consequential observation, authorize, deny, or make ATK-06 green by changing
the expected outcome.

The supporting portions of these units must not absorb token verification,
decision policy, error-to-deny logic, consumption, audit recording, or any real
tool integration.

## Required change process

T0 implementation remains subject to the complete T0 process: founder design authority and final
disposition, human review, adversarial testing, cross-model review, and at least three SAST/SCA
tools. Agents may review existing founder code against
`specs/CORE-002-guard-review-checklist.md`; they may edit T0 only within an explicit active
authorship exception such as Amendment B and only after every prerequisite gate is satisfied.
