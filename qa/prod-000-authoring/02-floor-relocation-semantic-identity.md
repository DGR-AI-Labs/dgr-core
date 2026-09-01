# PROD-000 template 2 — floor relocation and semantic identity

**Purpose:** Prove the supervised agent-authored reached-boundary floor preserves the approved
semantics while changing from a harness observation constructor to a product outcome constructor.

## Baseline evidence

| Baseline region | Location | SHA-256 |
|---|---|---|
| Complete old `before_tool_call` method | `before_tool_call.rs:115-171` | `35639394ea9d089f86b31d103027353e99e3e55522db8bf57112526d3ccfacc7` |
| Floor comment, `catch_unwind`, and match | `before_tool_call.rs:126-170` | `d9f2b23fb9a44422d38d8231b530ea0d739244897a9f147234e65608a1793c3f` |

## Destination evidence

| Field | Implementation entry |
|---|---|
| New floor function name | `<fill>` |
| Destination line range | `<fill>` |
| Destination region SHA-256 | `<fill>` |
| Agent implementation commit | `<fill after commit>` |

## Semantic-identity ledger

For every row, record the destination line and whether it is verbatim, structurally relocated, or
an Amendment-A-authorized constructor change.

| Proof obligation | Baseline fact | Destination line(s) | Disposition |
|---|---|---|---|
| Unwind boundary | `catch_unwind` surrounds only guard decision invocation | `<fill>` | `<fill>` |
| Unwind-safety scope | `AssertUnwindSafe` is bounded to one invocation | `<fill>` | `<fill>` |
| Store reasoning | stores are not inspected or reused after unwind | `<fill>` | `<fill>` |
| Typed-fault payload | payload is dropped | `<fill>` | `<fill>` |
| Panic payload | payload is dropped | `<fill>` | `<fill>` |
| Returned deny | original outcome and signal relayed | `<fill>` | `<fill>` |
| Returned escalation | original ID and deadline relayed | `<fill>` | `<fill>` |
| Returned allow | original authorization reference relayed | `<fill>` | `<fill>` |
| Fault/unwind outcome | `RequiredOutcome::FailClosed` | `<fill>` | `<fill>` |
| Fault/unwind signal | `CORE-003 boundary fail-closed` | `<fill>` | `<fill>` |
| Constructor | harness `Blocked` becomes product `Blocked` | `<fill>` | authorized semantic change |
| Tool behavior | floor never invokes a tool | `<fill>` | `<fill>` |

## Non-claim check

Confirm the new documentation still excludes all four conditions:

- [ ] `panic=abort`
- [ ] process termination
- [ ] OOM abort
- [ ] hook never invoked

## Behavioral evidence reservation

These checks run only after the T3 adapter is added:

- [ ] Existing ATK-07 typed-fault test passes unchanged.
- [ ] Existing ATK-07 panic test passes unchanged.
- [ ] Active attack set is unchanged.
- [ ] Ignored set remains exactly ATK-04/05/12/14/15.
- [ ] No test expectation was weakened.

## Founder semantic-identity disposition

Decision: `<PASS / FAIL>`

Rationale, including every non-verbatim line: `<fill>`

Founder signature/name: `<fill>`

UTC timestamp: `<fill>`

## Agent completeness attestation

> I recorded every non-verbatim line, constructor change, and semantic equivalence in this ledger.
> I did not infer byte identity where only semantic or behavioral evidence exists.

Agent product/model/session: `<fill>`

UTC timestamp: `<fill>`
