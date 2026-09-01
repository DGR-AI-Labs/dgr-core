# PROD-000 template 3 — old-floor removal and T3 boundary completion

**Purpose:** Record the agent's removal of the old mixed-file T0 surface and completion of the exact
T3 facade/adapter boundary authorized by Amendment B.

## Agent removal ledger

Target: `tests/bypass-rust/src/before_tool_call.rs`.

| Surface | Required action | Agent line/diff reference |
|---|---|---|
| `OpaqueCapabilityToken` | remove after founder-owned relocation | `<fill>` |
| `BeforeToolCallRequest` | remove after founder-owned relocation | `<fill>` |
| `GuardDecision` | remove after founder-owned relocation | `<fill>` |
| `GuardFault` | remove after founder-owned relocation | `<fill>` |
| `GuardDecisionPort` | remove after founder-owned relocation | `<fill>` |
| Old founder floor/method body | remove after founder-owned relocation | `<fill>` |

## T3 surfaces intentionally retained

- [ ] `EffectfulToolProbe`
- [ ] `BeforeToolCallObservation`
- [ ] `BeforeToolCallAdapter`
- [ ] Adapter constructor
- [ ] `RecordingToolProbe` remains in fixtures

The agent authors the replacement T3 adapter conversion in the same dedicated PROD-000 branch,
after the T0 product outcome exists. T3 authorship must remain separately classified from T0.

## Prohibited transitional states

- [ ] No `todo!()` or permissive adapter fallback was introduced.
- [ ] No temporary allow path was introduced.
- [ ] No duplicate active floor remains in the T3 module.
- [ ] No agent-authored T0 compatibility shim exists.
- [ ] No test was ignored or changed to accommodate the handoff.

## Intermediate and final status

An intermediate working tree may be temporarily non-compiling while the bounded partition is in
progress, but no review commit may contain a permissive stub or unexplained red state:

| Field | Implementation entry |
|---|---|
| Expected compile status | `<green / intentionally red>` |
| If red, exact compiler error summary | `<fill>` |
| Why the error belongs to the bounded T3 completion | `<fill>` |
| Final exact-commit status | `<must be green>` |

## Boundary inspection

Record results after founder removal:

| Inspection | Expected | Actual |
|---|---|---|
| `catch_unwind` occurrences in old T3 file | 0 | `<fill>` |
| `CORE-003 boundary fail-closed` occurrences in old T3 file | 0 | `<fill>` |
| `catch_unwind` occurrences in new T0 file | 1 | `<fill>` |
| Product outcome defined in new T0 file | yes | `<fill>` |

## Exact-commit record

| Field | Entry |
|---|---|
| Agent implementation commit SHA | `<fill>` |
| Exact final review commit SHA | `<fill>` |
| Working tree clean after final commit | `<yes/no>` |
| T0 regions classified | `<fill>` |
| T3 regions classified | `<fill>` |
| UTC completion time | `<fill>` |

Agent product/model/session: `<fill>`

Founder line-by-line disposition: `<PASS / FAIL; findings>`

Founder signature/name: `<fill>`
