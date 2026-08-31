# PROD-000 template 1 — founder boundary-module authorship

**Purpose:** Record direct founder authorship of the new T0 boundary module without supplying an
implementation skeleton.

**Target:** `tests/bypass-rust/src/founder_before_tool_call_floor.rs`

## Preconditions

- [ ] ADR-13 and Amendment A are merged and active.
- [ ] The dgr-core ADR-13 pointer is merged at the active source pin.
- [ ] The author has read `T0-AUTHORS.md`, `tests/bypass-rust/T0-BOUNDARY.md`, and the repository
      constitution.
- [ ] The author is working on the PROD-000 branch, not `main`.
- [ ] No agent-authored T0 stub or generated implementation is present.

## Authorship record

| Field | Founder entry |
|---|---|
| Founder name | `<fill>` |
| UTC start time | `<fill>` |
| Branch | `<fill>` |
| Base commit | `<fill>` |
| ADR-13 source pin | `104dbe651a869f198f2c76a58d7b2682bb82fbd6` |
| Editor/environment | `<fill>` |
| Destination file SHA-256 after authoring | `<fill>` |

## Source provenance

Author from the founder's existing source in `tests/bypass-rust/src/before_tool_call.rs`, not from
an agent-produced module body.

| Existing source surface | Baseline location | Founder disposition |
|---|---|---|
| `OpaqueCapabilityToken` | lines 13–17 | `<relocated / intentionally changed>` |
| `BeforeToolCallRequest` | lines 19–25 | `<relocated / intentionally changed>` |
| `GuardDecision` | lines 27–41 | `<relocated / intentionally changed>` |
| `GuardFault` | lines 43–49 | `<relocated / intentionally changed>` |
| `GuardDecisionPort` | lines 51–60 | `<relocated / intentionally changed>` |

Baseline complete-file SHA-256:
`5e44f9d6c4451bbe80c7821a6587663110b1144a6895a4dd2f8548d0e0de049d`.

## Product-outcome design record

Record the exact fields authored for each product outcome. This table is the review contract, not
code.

| Outcome | Required information | Forbidden information | Founder line(s) |
|---|---|---|---|
| `Blocked` | `RequiredOutcome`, denial signal | authorization/test counters, probe state | `<fill>` |
| `Escalated` | original review-request ID, original deadline | authorization/test counters, probe state | `<fill>` |
| `Authorized` | authorization reference | claim that a tool executed, probe state | `<fill>` |

## Boundary checklist

- [ ] The T0 module owns the request, guard decision, guard fault, decision port, product outcome,
      and reached-boundary floor.
- [ ] The T0 module does not define or import `EffectfulToolProbe`.
- [ ] The T0 module does not define or import `RecordingToolProbe`.
- [ ] The T0 module does not define or import `BeforeToolCallObservation`.
- [ ] The T0 module contains no `authorization_issued` or `effectful_invocations` test counters.
- [ ] `Authorized` means permission to proceed, not evidence of execution.
- [ ] The module contains no real tool invocation.
- [ ] No public item was added without a recorded first consumer.

## Founder attestation

> I authored the target T0 module directly from the existing founder-authored source and the active
> ADR invariants. I did not paste an agent-drafted enforcement implementation. Every consequential
> outcome and fault mapping in this module is my authored decision.

Founder signature/name: `<fill>`

UTC completion time: `<fill>`

