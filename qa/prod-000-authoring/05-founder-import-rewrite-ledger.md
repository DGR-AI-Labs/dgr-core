# PROD-000 template 5 — founder import-rewrite ledger

**Purpose:** Prove module-path edits inside founder-owned files are import-only and do not conceal
an enforcement change.

## Scope rule

For seven files below, the permitted change is limited to the import/module path needed to consume
the new founder boundary module. `founder_approval_timeout.rs` also contains the separately reviewed
R5.1 control-flow change; therefore this ledger must not certify that file's complete diff as
import-only.

## Per-file ledger

| Founder-owned file | Old path line(s) | New path line(s) | Before SHA-256 | After SHA-256 | Non-import diff? |
|---|---|---|---|---|---|
| `founder_approval_store.rs` | `<fill>` | `<fill>` | `<fill>` | `<fill>` | must be no |
| `founder_approval_timeout.rs` | `<fill>` | `<fill>` | `<fill>` | `<fill>` | yes — R5.1 only |
| `founder_authored_guard.rs` | `<fill>` | `<fill>` | `<fill>` | `<fill>` | must be no |
| `founder_consumption_store.rs` | `<fill>` | `<fill>` | `<fill>` | `<fill>` | must be no |
| `founder_fail_closed.rs` | `<fill>` | `<fill>` | `<fill>` | `<fill>` | must be no |
| `founder_s2_approval_store.rs` | `<fill>` | `<fill>` | `<fill>` | `<fill>` | must be no |
| `founder_s2_consumption_store.rs` | `<fill>` | `<fill>` | `<fill>` | `<fill>` | must be no |
| `founder_token_verification.rs` | `<fill>` | `<fill>` | `<fill>` | `<fill>` | must be no |

## Enforcement-body exclusion check

For every file except `founder_approval_timeout.rs`, confirm no changed line touches:

- [ ] condition or comparison
- [ ] arithmetic
- [ ] constant value
- [ ] match arm
- [ ] SQL statement
- [ ] store operation
- [ ] returned decision
- [ ] denial signal
- [ ] visibility or ownership classification

For `founder_approval_timeout.rs`, classify each non-import changed line under the separately
reviewed R5.1 disposition. No other non-import change is permitted.

## Dependency-direction check

- [ ] No founder-owned file imports the T3 `before_tool_call` module after the rewrite.
- [ ] No founder-owned file imports fixtures, observations, probes, or the attack registry.
- [ ] All imported type names remain unchanged unless Amendment A explicitly requires otherwise.
- [ ] The new module path resolves only to the founder-owned boundary module.

## Diff evidence

| Field | Founder entry |
|---|---|
| Baseline commit | `<fill>` |
| Founder commit | `<fill>` |
| Import-only diff artifact/path | `<fill>` |
| R5.1 diff artifact/path | `<fill>` |
| Unexpected changed line count | `<must be 0>` |

## Founder attestation

> I reviewed every changed line in the eight founder-owned consumers. Except for the separately
> dispositioned R5.1 timeout change, the changes are restricted to import/module paths and do not
> alter an enforcement expression, constant, SQL statement, store operation, returned decision, or
> denial signal.

Founder signature/name: `<fill>`

UTC timestamp: `<fill>`

