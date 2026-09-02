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
| New floor function name | `before_tool_call_floor` |
| Destination line range | `founder_before_tool_call_floor.rs:77-126` |
| Destination region SHA-256 after cross-model remediation | `b46374c66dd4acbe3210546f40ccc73d8b3ca8b894f24045258745ed36319bd5` |
| Agent implementation commit | `40b713039a5612831df415cdd785271a7342be74` |
| Cross-model remediation source commit | `b19f33ae16698a81b993e6cc5a751360b6109577` |

## Semantic-identity ledger

For every row, record the destination line and whether it is verbatim, structurally relocated, or
an Amendment-A-authorized constructor change.

| Proof obligation | Baseline fact | Destination line(s) | Disposition |
|---|---|---|---|
| Unwind boundary | `catch_unwind` surrounds only guard decision invocation | 97–99 | Structurally relocated; `self.guard` becomes the explicit `guard` argument |
| Unwind-safety scope | `AssertUnwindSafe` is bounded to one invocation | 93–99 | Preserved; method wording becomes function wording |
| Store reasoning | stores are not inspected or reused after unwind | 93–96 | Preserved with method→function wording only |
| Typed-fault payload | payload is dropped | 121 | Preserved by combined `Ok(Err(_))` arm |
| Panic payload | payload is dropped | 121 | Preserved by combined `Err(_)` arm |
| Returned deny | original outcome and signal relayed | 101–108 | Preserved values; product constructor replaces harness constructor |
| Returned escalation | original ID and deadline relayed | 109–115 | Preserved values; product constructor replaces harness constructor |
| Returned allow | original authorization reference relayed | 116–120 | Preserved value; `Authorized` defers probe invocation to T3 |
| Fault/unwind outcome | `RequiredOutcome::FailClosed` | 121–123 | Preserved exactly |
| Fault/unwind signal | `CORE-003 boundary fail-closed` | 123 | Preserved exactly |
| Constructor | harness `Blocked` becomes product `Blocked` | 104–108, 121–124 | Amendment-A-authorized semantic change |
| Tool behavior | floor never invokes a tool | entire module; adapter 107–116 | Probe type absent from T0; T3 invokes only after `Authorized` |

## Non-claim check

Confirm the new documentation still excludes all four conditions:

- [x] `panic=abort`
- [x] process termination
- [x] OOM abort
- [x] hook never invoked

## Behavioral evidence reservation

These checks run only after the T3 adapter is added:

- [x] Existing ATK-07 typed-fault test passes unchanged.
- [x] Existing ATK-07 panic test passes unchanged.
- [x] Active attack set is unchanged.
- [x] Ignored set remains exactly ATK-04/05/12/14/15.
- [x] No test expectation was weakened.

## Founder semantic-identity disposition

Decision: **PENDING FOUNDER REVIEW**

Rationale, including every non-verbatim line: Agent ledger above; founder disposition pending.

Founder signature/name: **PENDING**

UTC timestamp: **PENDING**

## Agent completeness attestation

> I recorded every non-verbatim line, constructor change, and semantic equivalence in this ledger.
> I did not infer byte identity where only semantic or behavioral evidence exists.

Agent product/model/session: OpenAI Codex; model and session identifiers not exposed

UTC timestamp: `2026-09-01T21:12:17Z`
