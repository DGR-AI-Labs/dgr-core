# PROD-000 template 1 — supervised-agent boundary-module authorship

**Purpose:** Record agent authorship, source provenance, and founder review of the new T0 boundary
module without supplying an implementation skeleton.

**Target:** `tests/bypass-rust/src/founder_before_tool_call_floor.rs`

## Preconditions

- [x] ADR-13 and Amendments A and B are merged and active.
- [x] The dgr-core ADR-13 pointer is merged at the active source pin.
- [x] The Amendment-B versions of all five evidence templates are merged.
- [x] The canonical PROD-000 backlog update is merged.
- [x] The pre-existing founder draft has a recorded checkpoint-or-discard disposition.
- [x] The author has read `T0-AUTHORS.md`, `tests/bypass-rust/T0-BOUNDARY.md`, and the repository
      constitution.
- [x] The author is working on the PROD-000 branch, not `main`.
- [x] The baseline hashes in the preparation have been recomputed and dispositioned.
- [x] No permissive stub or unrelated implementation is present.

## Authorship record

| Field | Implementation entry |
|---|---|
| Implementing agent product/model | OpenAI Codex; model identifier not exposed to the task |
| Agent session/task identifier, if available | Current Codex task; identifier not exposed |
| Founder authorization reference | `ADR-13-AMENDMENT-B` |
| UTC start time | Not exposed; implementation checkpoint recorded at `2026-09-01T21:12:17Z` |
| Branch | `codex/prod-000-supervised-agent-t0` |
| Base commit | `e9c8f585809c15d2464b3d45bc2ce26d716c8673` |
| ADR-13 source pin | `891607c20ba65c31b024c59f29f09744f8a62b26` |
| Editor/environment | Codex desktop shared workspace; WSL/Linux Rust toolchain |
| Destination file SHA-256 after cross-model remediation | `d1c98dedbf544ab1e27d3d9e12055f96e8a5d5b76b2c63edb76e4df4ff0b542f` |
| Agent implementation commit | `40b713039a5612831df415cdd785271a7342be74` |
| Cross-model remediation source commit | `b19f33ae16698a81b993e6cc5a751360b6109577` |

## Pre-existing founder-draft disposition

Exactly one disposition must be completed before implementation:

| Field | Entry |
|---|---|
| Disposition | Discard |
| Founder authorization | Explicit instruction in the current Codex task on 2026-09-01: “I authorize discarding the zero-byte founder draft.” |
| If checkpointed: founder commit | Not applicable |
| If checkpointed: complete-file SHA-256 | Not applicable |
| If checkpointed: exact founder-authored ranges | Not applicable |
| If discarded: exact discarded path and hash | `tests/bypass-rust/src/founder_before_tool_call_floor.rs`; zero bytes; `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| Implementing agent verified clean baseline | Yes; new worktree at `e9c8f585809c15d2464b3d45bc2ce26d716c8673` |

## Source provenance

Relocate the existing type shapes from the former mixed-tier
`tests/bypass-rust/src/before_tool_call.rs`. These shared request/decision/fault/port shapes were
not founder-authored, but they encoded consequential authorization semantics and therefore were
T0 by consequence even while physically located in the mixed-tier adapter. Their relocation and
documentation changes are agent-authored T0 transformations. The floor transformation is recorded
separately in template 2 with its founder-source provenance.

| Existing source surface | Baseline location | Implementation classification |
|---|---|---|
| `OpaqueCapabilityToken` | lines 13–17 | Pre-existing non-founder T0-by-consequence type; agent-relocated/transformed; shape preserved at destination lines 12–16, doc wording changed |
| `BeforeToolCallRequest` | lines 19–25 | Pre-existing non-founder T0-by-consequence type; agent-relocated/transformed; shape preserved at destination lines 18–24, doc wording changed |
| `GuardDecision` | lines 27–41 | Pre-existing non-founder T0-by-consequence type; agent-relocated/transformed; shape preserved at destination lines 26–40, doc wording changed |
| `GuardFault` | lines 43–49 | Pre-existing non-founder T0-by-consequence type; agent-relocated/transformed; shape preserved at destination lines 42–48 |
| `GuardDecisionPort` | lines 51–60 | Pre-existing non-founder T0-by-consequence type; agent-relocated/transformed; signature preserved at destination lines 50–59, doc wording changed |

Baseline complete-file SHA-256:
`5e44f9d6c4451bbe80c7821a6587663110b1144a6895a4dd2f8548d0e0de049d`.

## Product-outcome design record

Record the exact fields authored for each product outcome. This table is the review contract, not
code.

| Outcome | Required information | Forbidden information | Agent-authored line(s) |
|---|---|---|---|
| `Blocked` | `RequiredOutcome`, denial signal | authorization/test counters, probe state | lines 64–67 |
| `Escalated` | original review-request ID, original deadline | authorization/test counters, probe state | lines 68–71 |
| `Authorized` | authorization reference | claim that a tool executed, probe state | lines 72–74 |

## Boundary checklist

- [x] The T0 module owns the request, guard decision, guard fault, decision port, product outcome,
      and reached-boundary floor.
- [x] The T0 module does not define or import `EffectfulToolProbe`.
- [x] The T0 module does not define or import `RecordingToolProbe`.
- [x] The T0 module does not define or import `BeforeToolCallObservation`.
- [x] The T0 module contains no `authorization_issued` or `effectful_invocations` test counters.
- [x] `Authorized` means permission to proceed, not evidence of execution.
- [x] The module contains no real tool invocation.
- [x] No public item was added without a recorded first consumer.

## Agent authorship attestation

> I authored or relocated the target T0 module under ADR-13 Amendment B. I classified every changed
> region truthfully, identified every transformation of existing founder source, and introduced no
> implementation outside the authorized PROD-000 scope.

Agent product/model/session: OpenAI Codex; model and session identifiers not exposed

UTC completion time: `2026-09-01T21:12:17Z`

## Founder review disposition

Exact reviewed commit: **PENDING** — founder must review the final PR head, not only `40b7130...`

Non-author cross-model review record: **PENDING**

Independent-human review record: **PENDING**

Three-engine SAST/SCA evidence record: **AVAILABLE** —
`qa/prod-000-exact-commit-validation.md`; findings remain pending independent-human and founder
disposition

Decision: **PENDING**

Finding disposition and line-level provenance corrections: **PENDING**

Founder signature/name: **PENDING**

UTC timestamp: **PENDING**
