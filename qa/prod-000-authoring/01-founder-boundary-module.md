# PROD-000 template 1 — supervised-agent boundary-module authorship

**Purpose:** Record agent authorship, source provenance, and founder review of the new T0 boundary
module without supplying an implementation skeleton.

**Target:** `tests/bypass-rust/src/founder_before_tool_call_floor.rs`

## Preconditions

- [ ] ADR-13 and Amendments A and B are merged and active.
- [ ] The dgr-core ADR-13 pointer is merged at the active source pin.
- [ ] The Amendment-B versions of all five evidence templates are merged.
- [ ] The canonical PROD-000 backlog update is merged.
- [ ] The pre-existing founder draft has a recorded checkpoint-or-discard disposition.
- [ ] The author has read `T0-AUTHORS.md`, `tests/bypass-rust/T0-BOUNDARY.md`, and the repository
      constitution.
- [ ] The author is working on the PROD-000 branch, not `main`.
- [ ] The baseline hashes in the preparation have been recomputed and dispositioned.
- [ ] No permissive stub or unrelated implementation is present.

## Authorship record

| Field | Implementation entry |
|---|---|
| Implementing agent product/model | `<fill>` |
| Agent session/task identifier, if available | `<fill>` |
| Founder authorization reference | `ADR-13-AMENDMENT-B` |
| UTC start time | `<fill>` |
| Branch | `<fill>` |
| Base commit | `<fill>` |
| ADR-13 source pin | `891607c20ba65c31b024c59f29f09744f8a62b26` |
| Editor/environment | `<fill>` |
| Destination file SHA-256 after authoring | `<fill>` |
| Agent implementation commit | `<fill after commit>` |

## Pre-existing founder-draft disposition

Exactly one disposition must be completed before implementation:

| Field | Entry |
|---|---|
| Disposition | `<checkpoint / discard>` |
| Founder authorization | `<commit, signed record, or explicit review reference>` |
| If checkpointed: founder commit | `<fill / not applicable>` |
| If checkpointed: complete-file SHA-256 | `<fill / not applicable>` |
| If checkpointed: exact founder-authored ranges | `<fill / not applicable>` |
| If discarded: exact discarded path and hash | `<fill / not applicable>` |
| Implementing agent verified clean baseline | `<yes/no; commit>` |

## Source provenance

Relocate from the existing founder source in `tests/bypass-rust/src/before_tool_call.rs`. Classify
verbatim relocation separately from transformed or new agent-authored T0.

| Existing source surface | Baseline location | Implementation classification |
|---|---|---|
| `OpaqueCapabilityToken` | lines 13–17 | `<verbatim agent relocation / agent-authored transformation>` |
| `BeforeToolCallRequest` | lines 19–25 | `<verbatim agent relocation / agent-authored transformation>` |
| `GuardDecision` | lines 27–41 | `<verbatim agent relocation / agent-authored transformation>` |
| `GuardFault` | lines 43–49 | `<verbatim agent relocation / agent-authored transformation>` |
| `GuardDecisionPort` | lines 51–60 | `<verbatim agent relocation / agent-authored transformation>` |

Baseline complete-file SHA-256:
`5e44f9d6c4451bbe80c7821a6587663110b1144a6895a4dd2f8548d0e0de049d`.

## Product-outcome design record

Record the exact fields authored for each product outcome. This table is the review contract, not
code.

| Outcome | Required information | Forbidden information | Agent-authored line(s) |
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

## Agent authorship attestation

> I authored or relocated the target T0 module under ADR-13 Amendment B. I classified every changed
> region truthfully, identified every transformation of existing founder source, and introduced no
> implementation outside the authorized PROD-000 scope.

Agent product/model/session: `<fill>`

UTC completion time: `<fill>`

## Founder review disposition

Exact reviewed commit: `<fill>`

Non-author cross-model review record: `<fill path and reviewed commit>`

Independent-human review record: `<fill path and reviewed commit>`

Three-engine SAST/SCA evidence record: `<fill path and reviewed commit>`

Decision: `<PASS / FAIL>`

Finding disposition and line-level provenance corrections: `<fill>`

Founder signature/name: `<fill>`

UTC timestamp: `<fill>`
