# T0 authorship boundary

This file is the ownership map for CORE-002, CORE-003, and CORE-004. It does
not authorize an agent to write enforcement code. The binding repository constitution
classifies every consequential authorization path as T0 and human-led.

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

Only the founder authors the bodies of the existing functions named here:

| File | Founder-only function | Required responsibility |
|---|---|---|
| `tests/bypass-rust/src/founder_authored_guard.rs` | `FounderAuthoredGuard::decide` | Guard decision for the intercepted `before_tool_call` request |
| `tests/bypass-rust/src/founder_token_verification.rs` | `verify_capability_token` | Capability-token verification |
| `tests/bypass-rust/src/founder_fail_closed.rs` | `fail_closed_decision` | Deny behavior for absence, invalidity, unavailability, or internal error |
| `tests/bypass-rust/src/founder_s2_consumption_store.rs` | `S2ConsumptionStore::consume` | Durable-local, atomic single-use consumption before allow |
| `tests/bypass-rust/src/founder_consumption_store.rs` | `ConsumptionStore::consume` | Store boundary retained for S2 now and S3 later |
| `tests/bypass-rust/src/before_tool_call.rs` | `BeforeToolCallAdapter::before_tool_call` | Contain guard faults and unwinding panics and return a fail-closed block before tool invocation |

## CORE-004 founder-owned surfaces

The following implemented surfaces are founder-authored T0. Their names record
ownership and responsibility and do not authorize later agent changes.

| Surface/location | Founder-owned responsibility |
|---|---|
| `tests/bypass-rust/src/founder_approval_store.rs` | The `ApprovalStore` port; original-id/deadline `AlreadyPending` behavior; and every consequential pending/not-found/timed-out/fault outcome |
| `tests/bypass-rust/src/founder_s2_approval_store.rs` | Durable-local SQLite record, deduplication, lookup, and timeout-transition behavior, including deadline immutability and persist-then-observe |
| `tests/bypass-rust/src/founder_authored_guard.rs` | Canonical amount validation; founder threshold and conformance mirrors; escalation after binding and before nonce consumption; and timeout decisions |
| R-3 timeout-evaluation path (exact function/location selected by the founder during authoring) | Evaluate a pending record against the trusted injected clock without token re-presentation; return the same `Escalated` id/deadline while `now <= deadline`; persist timeout before returning the terminal block when `now > deadline` |
| `tests/bypass-rust/src/before_tool_call.rs` adapter behavior | Emit `Escalated` only after durable persistence and guarantee no authorization, nonce consumption, or effectful invocation on that path |

Any shared enum or trait encoding consequential pending, escalated, approved,
or denied semantics is T0 until the founder records a narrower classification.

## Current implementation state

The CORE-002 units named above, the CORE-003
`BeforeToolCallAdapter::before_tool_call` boundary, and the CORE-004 surfaces
contain founder-authored T0 enforcement pending the applicable T0 review gates. The default
`ConsumptionStore` implementation still returns `FounderImplementationRequired`
explicitly so an absent concrete store fails closed. The S2 unit exposes an
in-memory constructor for isolated conformance tests and a file-backed
constructor for restart-durable local consumption.

This state record does not relax the authorship boundary. An agent must not
replace, complete, refactor, or route around any founder-authored unit.

## Agent-authored supporting units

The following are outside the founder implementation surface:

- `tests/bypass-rust/src/before_tool_call.rs`, except for the founder-owned
  `BeforeToolCallAdapter::before_tool_call` body: test-only adapter types and
  mechanical probe plumbing;
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

Founder implementation remains subject to the T0 process: human-led authorship
and review, adversarial testing, cross-model review, and at least three SAST
tools. Agents may review the founder's code against
`specs/CORE-002-guard-review-checklist.md`; they may not edit it.
