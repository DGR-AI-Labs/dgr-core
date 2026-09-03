# PROD-000 template 4 — module registration and dependency boundary

**Purpose:** Record the module declaration and prove dependency direction through the T0 module and
T3 compatibility facade.

## Registration record

| Field | Implementation entry |
|---|---|
| Module name | `founder_before_tool_call_floor` |
| Declaration location in `src/lib.rs` | line 13 |
| Visibility | public module; explicit public boundary symbols listed below |
| First product consumer for each public item | Direct founder-unit imports and T3 facade/adapter recorded below |
| Agent implementation commit | `40b713039a5612831df415cdd785271a7342be74` |
| Cross-model remediation source commit | `b19f33ae16698a81b993e6cc5a751360b6109577` |

## Required dependency graph

| From | To | Allowed? | Actual evidence |
|---|---|---:|---|
| Existing founder T0 units | New Amendment-B boundary module | yes | Direct imports in the eight explicitly named consumers in template 5 |
| T3 facade/adapter | New Amendment-B boundary module | yes, after handoff | private floor import and public type re-exports at `before_tool_call.rs:11-15`; delegation at lines 82–117 |
| New Amendment-B boundary module | Founder stores/domain types | yes | `founder_before_tool_call_floor.rs:8-10,55-57,86-88` |
| New Amendment-B boundary module | T3 facade/adapter | no | zero imports or references |
| New Amendment-B boundary module | fixtures/probes/observations | no | zero imports or references |
| New Amendment-B boundary module | attack registry | no | zero `attack_by_id`/`ATTACK_SET` references |

## Public-surface review

For each public symbol in the new module, record why it must cross the future library boundary.

| Public symbol | Required consumer | Why private is insufficient | Approved? |
|---|---|---|---|
| `OpaqueCapabilityToken` | `BeforeToolCallRequest`, fixtures, token verifier | Crosses request/verification boundary | PENDING founder review |
| `BeforeToolCallRequest` | guard port, founder guard, T3 adapter | Boundary input shared across T0 and adapter | PENDING founder review |
| `GuardDecision` | founder guard, timeout evaluator, fail-closed unit, floor | Consequential decision shared across T0 units | PENDING founder review |
| `GuardFault` | stores, verifier, guard, floor | Fail-closed fault type shared across T0 units | PENDING founder review |
| `GuardDecisionPort` | founder guard and reached-boundary floor | Separates guard decision from boundary containment | PENDING founder review |
| `BeforeToolCallOutcome` | T3 adapter now; future extracted consumer | Product result must cross the T0/T3 boundary | PENDING founder review |
| `before_tool_call_floor` | T3 adapter now; future runtime adapter | Reached-boundary floor must be callable without harness dependency | PENDING founder review |

Internal parsing, canonicalization, pinned key internals, SQL, and helper functions must remain
private.

## Inspection commands and results

Record the output summary rather than pasting secrets or full build logs.

| Inspection | Result |
|---|---|
| T0 module imports containing `before_tool_call` harness path | 0 |
| New T0 module imports containing `fixtures` | 0 |
| New T0 module imports containing `attack_by_id` or `ATTACK_SET` | 0 |
| New T0 module references to probe/observation counters | 0 |

## Founder disposition

Decision: **PENDING**

Unresolved public-surface or dependency issue: Founder must disposition all seven public symbols.

Founder signature/name: **PENDING**

UTC timestamp: **PENDING**

Agent product/model/session: OpenAI Codex; model and session identifiers not exposed
