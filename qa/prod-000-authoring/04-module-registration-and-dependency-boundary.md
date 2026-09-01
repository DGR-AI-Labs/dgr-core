# PROD-000 template 4 — module registration and dependency boundary

**Purpose:** Record the module declaration and prove dependency direction before the T3 facade is
added.

## Registration record

| Field | Implementation entry |
|---|---|
| Module name | `<fill>` |
| Declaration location in `src/lib.rs` | `<fill>` |
| Visibility | `<fill>` |
| First product consumer for each public item | `<fill or link>` |
| Agent implementation commit | `<fill after commit>` |

## Required dependency graph

| From | To | Allowed? | Actual evidence |
|---|---|---:|---|
| Founder T0 units | New founder boundary module | yes | `<fill>` |
| T3 facade/adapter | New founder boundary module | yes, after handoff | `<fill later>` |
| New founder boundary module | Founder stores/domain types | yes | `<fill>` |
| New founder boundary module | T3 facade/adapter | no | `<fill>` |
| New founder boundary module | fixtures/probes/observations | no | `<fill>` |
| New founder boundary module | attack registry | no | `<fill>` |

## Public-surface review

For each public symbol in the new module, record why it must cross the future library boundary.

| Public symbol | Required consumer | Why private is insufficient | Approved? |
|---|---|---|---|
| `<fill>` | `<fill>` | `<fill>` | `<yes/no>` |

Internal parsing, canonicalization, pinned key internals, SQL, and helper functions must remain
private.

## Inspection commands and results

Record the output summary rather than pasting secrets or full build logs.

| Inspection | Result |
|---|---|
| T0 module imports containing `before_tool_call` harness path | `<fill>` |
| New T0 module imports containing `fixtures` | `<fill>` |
| New T0 module imports containing `attack_by_id` or `ATTACK_SET` | `<fill>` |
| New T0 module references to probe/observation counters | `<fill>` |

## Founder disposition

Decision: `<PASS / FAIL>`

Unresolved public-surface or dependency issue: `<none / fill>`

Founder signature/name: `<fill>`

UTC timestamp: `<fill>`

Agent product/model/session: `<fill>`
