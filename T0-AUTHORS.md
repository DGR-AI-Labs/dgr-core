# T0 authorship boundary

This file is the ownership map for CORE-002. It does not authorize an agent to
write enforcement code. The binding repository constitution classifies every
consequential authorization path as T0 and human-led.

The normative contracts consumed by the founder implementation are listed in
`specs/CORE-002-reference-contracts.md`. That file points to the pinned
`dgr-internal` reference records; it does not duplicate or redefine them.

## Founder-authored units

Only the founder authors the bodies of these functions:

| File | Founder-only function | Required responsibility |
|---|---|---|
| `tests/bypass-rust/src/founder_authored_guard.rs` | `FounderAuthoredGuard::decide` | Guard decision for the intercepted `before_tool_call` request |
| `tests/bypass-rust/src/founder_token_verification.rs` | `verify_capability_token` | Capability-token verification |
| `tests/bypass-rust/src/founder_fail_closed.rs` | `fail_closed_decision` | Deny behavior for absence, invalidity, unavailability, or internal error |
| `tests/bypass-rust/src/founder_s2_consumption_store.rs` | `S2ConsumptionStore::consume` | Durable-local, atomic single-use consumption before allow |
| `tests/bypass-rust/src/founder_consumption_store.rs` | `ConsumptionStore::consume` | Store boundary retained for S2 now and S3 later |

Each unit currently contains only a public signature, a
`{FOUNDER-AUTHORS}` marker, and
`unimplemented!("FounderImplementationRequired")`. It cannot verify, decide,
deny, consume, or return allow. An agent must not replace, complete, refactor,
or route around that default.

## Agent-authored supporting units

The following are outside the founder implementation surface:

- `tests/bypass-rust/src/before_tool_call.rs`: test-only adapter types and
  mechanical relay to a fake tool probe;
- `tests/bypass-rust/src/fixtures.rs`: opaque no-token, valid-candidate,
  expired, replayed, forged, and out-of-scope fixture bytes;
- `tests/bypass-rust/tests/adapter_harness.rs`: adapter-plumbing tests using
  scripted decisions; and
- `tests/bypass-rust/tests/attack_set.rs`: conformance expectations, including
  the deliberately red ATK-01 test.

These supporting units must not absorb token verification, decision policy,
error-to-deny logic, consumption, audit recording, or any real tool
integration.

## Required change process

Founder implementation remains subject to the T0 process: human-led authorship
and review, adversarial testing, cross-model review, and at least three SAST
tools. Agents may review the founder's code against
`specs/CORE-002-guard-review-checklist.md`; they may not edit it.
