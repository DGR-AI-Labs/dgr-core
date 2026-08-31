# PROD-000 template 3 — old-floor removal and T3 handoff

**Purpose:** Record the founder's removal of the old mixed-file T0 surface and define the exact
compile-red handoff to the agent-authored T3 adapter work.

## Founder removal ledger

Target: `tests/bypass-rust/src/before_tool_call.rs`.

| Surface | Required action | Founder line/diff reference |
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

The founder does not author the replacement adapter conversion. The agent will add it only after
the founder commit exists.

## Prohibited transitional states

- [ ] No `todo!()` or permissive adapter fallback was introduced.
- [ ] No temporary allow path was introduced.
- [ ] No duplicate active floor remains in the T3 module.
- [ ] No agent-authored T0 compatibility shim exists.
- [ ] No test was ignored or changed to accommodate the handoff.

## Expected intermediate status

The founder-only commit may be temporarily non-compiling because the T3 facade and adapter method
are intentionally deferred. Record the exact expected failure rather than fixing it in T0:

| Field | Founder entry |
|---|---|
| Expected compile status | `<green / intentionally red>` |
| If red, exact compiler error summary | `<fill>` |
| Why the error belongs to T3 follow-up | `<fill>` |

## Boundary inspection

Record results after founder removal:

| Inspection | Expected | Actual |
|---|---|---|
| `catch_unwind` occurrences in old T3 file | 0 | `<fill>` |
| `CORE-003 boundary fail-closed` occurrences in old T3 file | 0 | `<fill>` |
| `catch_unwind` occurrences in new T0 file | 1 | `<fill>` |
| Product outcome defined in new T0 file | yes | `<fill>` |

## Handoff record

| Field | Founder entry |
|---|---|
| Founder commit SHA | `<fill>` |
| Working tree clean after commit | `<yes/no>` |
| Files intentionally left for T3 agent | `<fill>` |
| UTC handoff time | `<fill>` |

Founder signature/name: `<fill>`

