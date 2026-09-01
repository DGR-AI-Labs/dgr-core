# PROD-000 template 3 — old-floor removal and T3 boundary completion

**Purpose:** Record the agent's removal of the old mixed-file T0 surface and completion of the exact
T3 facade/adapter boundary authorized by Amendment B.

## Agent removal ledger

Target: `tests/bypass-rust/src/before_tool_call.rs`.

| Surface | Required action | Agent line/diff reference |
|---|---|---|
| `OpaqueCapabilityToken` | remove after agent-authored T0 relocation | `40b7130...`; old lines 13–17 removed, T3 re-export at lines 11–14 |
| `BeforeToolCallRequest` | remove after agent-authored T0 relocation | `40b7130...`; old lines 19–25 removed, T3 re-export at lines 11–14 |
| `GuardDecision` | remove after agent-authored T0 relocation | `40b7130...`; old lines 27–41 removed, T3 re-export at lines 11–14 |
| `GuardFault` | remove after agent-authored T0 relocation | `40b7130...`; old lines 43–49 removed, T3 re-export at lines 11–14 |
| `GuardDecisionPort` | remove after agent-authored T0 relocation | `40b7130...`; old lines 51–60 removed, T3 re-export at lines 11–14 |
| Old founder floor/method body | remove after agent-authored transformation | `40b7130...`; old lines 126–170 replaced by T3 delegation at lines 80–115 |

## T3 surfaces intentionally retained

- [x] `EffectfulToolProbe`
- [x] `BeforeToolCallObservation`
- [x] `BeforeToolCallAdapter`
- [x] Adapter constructor
- [x] `RecordingToolProbe` remains in fixtures

The agent authors the replacement T3 adapter conversion in the same dedicated PROD-000 branch,
after the T0 product outcome exists. T3 authorship must remain separately classified from T0.

## Prohibited transitional states

- [x] No `todo!()` or permissive adapter fallback was introduced.
- [x] No temporary allow path was introduced.
- [x] No duplicate active floor remains in the T3 module.
- [x] No agent-authored T0 compatibility shim exists.
- [x] No test was ignored or changed to accommodate the handoff.

## Intermediate and final status

An intermediate working tree may be temporarily non-compiling while the bounded partition is in
progress, but no review commit may contain a permissive stub or unexplained red state:

| Field | Implementation entry |
|---|---|
| Expected compile status | Green at committed implementation checkpoint |
| If red, exact compiler error summary | Not applicable; no red state was committed |
| Why the error belongs to the bounded T3 completion | Not applicable |
| Final exact-input status | Green at scanned descendant `425d771...`; final PR-head founder review pending |

## Boundary inspection

Record results after founder removal:

| Inspection | Expected | Actual |
|---|---|---|
| `catch_unwind` occurrences in old T3 file | 0 | 0 |
| `CORE-003 boundary fail-closed` occurrences in old T3 file | 0 | 0 |
| `catch_unwind` occurrences in new T0 file | 1 | 1 |
| Product outcome defined in new T0 file | yes | yes, lines 61–75 |

## Exact-commit record

| Field | Entry |
|---|---|
| Agent implementation commit SHA | `40b713039a5612831df415cdd785271a7342be74` |
| Exact final review commit SHA | **PENDING final PR head; founder must name it in disposition** |
| Working tree clean after final commit | **PENDING evidence commit** |
| T0 regions classified | New module; R5.1 timeout change; eight direct import rewrites; `lib.rs` registration |
| T3 regions classified | Compatibility re-exports; outcome conversion/probe invocation; registry-mirror assertion |
| UTC completion time | `2026-09-01T21:12:17Z` |

Agent product/model/session: OpenAI Codex; model and session identifiers not exposed

Founder line-by-line disposition: **PENDING**

Founder signature/name: **PENDING**
