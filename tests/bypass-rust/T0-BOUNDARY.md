# T0 boundary — read before implementing this harness

This directory combines the **T3 test-data and conformance harness** with the
founder-owned T0 units listed in the repository-root `T0-AUTHORS.md`. It is not
the deployed DGR gate and must never become an alternate production enforcement
path. `before_tool_call.rs` can invoke only a test probe, never a real tool.

`founder_before_tool_call_floor.rs` now contains the Amendment-B agent-authored CORE-003 T0 floor,
with identified founder-source provenance for the transformed fault/unwind logic. The T3
`BeforeToolCallAdapter` converts the product outcome to a harness observation and invokes the probe
only after `Authorized`. Returned `Deny`, `Escalate`, and `Allow` decisions retain their established
relay behavior. `Err(GuardFault)` and an unwinding panic from `decide` produce a product-level
`Blocked` with `RequiredOutcome::FailClosed`; the adapter records no authorization and no effectful
invocation.

`ATK_06_TIMEOUT_OUTCOME` is authoritative for T0 enforcement after PROD-000. The CORE-001 attack
registry remains the T3 conformance representation. The equality assertion added to the existing
`atk_06_sequence_is_escalated_then_registry_derived_timeout_block` test detects drift from T0 to
the registry; it is not a second source of enforcement policy.

The unwind guarantee is bounded to Rust unwinding panics after the boundary is
reached. It does not claim protection from `panic=abort`, process termination,
OOM abort, a hook that never fires, or a route around the hook. It also does not
certify a store's post-panic invariants for later reuse.

The following remain **T0, human-led, and review-gated once implemented**:

- capability-token minting or verification;
- the policy decision point;
- the fail-closed runtime guard or native `before_tool_call` hook;
- audit/hash-chain construction or verification; and
- any implementation that can authorize or block a consequential action.

The founder-owned CORE-002 units, historical CORE-003 boundary source, and CORE-004
surfaces are listed in the repository-root `T0-AUTHORS.md`. Their applicable T0
human gates are complete: CORE-002 Step 5 was accepted through implementation
PR #68, review-record PR #70, and
`qa/core-002-step5-governance-disposition.md`; CORE-003 was accepted through PR
#73 and the signed records indexed by `qa/core-003-t0-review-readiness.md`; and
CORE-004 was accepted through implementation PR #81, evidence PR #82, and the
signed founder and independent-human dispositions under `qa/`. This satisfies
the T0-review precondition for CORE-005 to wire the currently active
conformance suite into required CI. It does not activate deferred attacks or
broaden the isolation claim.

Agents must not make a test pass by weakening or changing an expectation. Outside ADR-13
Amendment B's exact PROD-000 scope, they also must not add or change enforcement behavior in the
founder-owned units. If an attack requires unresolved gate internals or a broader T0 change, record
the dependency and stop. All consequential changes remain subject to the T0 process recorded in the
repository constitution.

## PROD-000 supervised-agent exception

ADR-13 Amendment B supersedes only Amendment A R5.3 authorship option (c). The pointer, templates,
and backlog prerequisites are merged, and the founder explicitly discarded the zero-byte draft.
OpenAI Codex authored the bounded PROD-000 T0/T3 partition at implementation checkpoint
`40b713039a5612831df415cdd785271a7342be74` under founder design authority. Line-by-line review,
exact-commit finding disposition, independent-human review, and founder-only merge remain pending.

After the first Claude review returned `CHANGES REQUIRED`, replacement source commit
`b19f33ae16698a81b993e6cc5a751360b6109577` corrected the bounded source/document findings. T3-only
commit `587585cf476431f078efe587c5dbcc052389cdad` makes the ATK-06 T0/registry equality test
non-droppable through the required libtest-enumeration guard. These are review inputs, not approval.

The resulting evidence must distinguish existing founder source, agent-relocated founder source,
agent-authored T0, T3, and founder review. The complete result must not be described as
founder-authored. This exception does not authorize a permissive stub, a changed conformance
expectation, a second active floor, real tool integration, PROD-001 extraction, or any T0 change
outside Amendment B's exact file and symbol scope.

CORE-003 covers ATK-07 only: the boundary was reached, but its guard/verifier
returned a fault or panicked. A hook that never fires, a route around the hook,
a missing plugin, or operator bypass is runtime-integration scope retained by
RUNTIME-003/004, not simulated by this isolation harness.

`src/gate.mjs` is a deliberately failing Phase-0 scaffold, not an active or authoritative
enforcement floor. PROD-000 and its three-engine evidence cover the Rust T0/T3 partition. The
JavaScript scaffold neither competes with nor bypasses the Rust isolation harness, and the
Rust-only SAST evidence does not claim security coverage for unrelated JavaScript.

## CORE-004 ownership and bounded claim

CORE-004 covers ATK-06's timeout-only isolation contract. Its `Escalated` observation, durable
pending-store behavior, escalation and timeout decisions, R-3 timeout-evaluation path, and adapter
behavior that emits `Escalated` retain founder-authored T0 provenance. PROD-000's R5.1
constant/dependency reversal, module-path rewrites, and outcome conversion are agent-authored
changes pending founder review. `Escalated` is not an authorization and must not invoke the
effectful probe.

The R-3 timeout moment is distinct from token-bearing `before_tool_call`: it
evaluates an existing pending record against the trusted injected clock
without re-presenting or re-verifying an expired token. The existing
verify-then-decide ordering for token-bearing requests remains unchanged.

Addendum A freezes the token-bearing order as signature → lifetime/expiry →
binding plus canonical validation → escalation check → consume → allow. The
founder threshold is `1_000_000` minor units and escalation occurs only when
the bound canonical amount is greater than it. Escalation must not consume the
nonce. Re-presentation and pre-deadline timeout evaluation return the original
review-request id and deadline; only `now > deadline` produces the ATK-06
terminal block.

An escalation predicate over agent-supplied action data may use only committed,
verified fields. Future ATK-05 evidence handling must reuse the same approval
path, store, and observations, but it remains deferred until its own evidence
provenance/binding contract is founder-approved.

The CORE-004 claim is bounded exactly as follows:

> CORE-004 proves the escalate → deny-on-timeout contract for a single guard
> instance and its local pending store under a modeled clock. It does not prove
> real human delivery, real waiting across restarts, cross-instance pending
> state, or live non-bypassability.

### Deadline and TTL standing rule

The founder-owned deadline is computed once with checked arithmetic from the
trusted injected clock, stored explicitly, and never extended. Timeout is
evaluated by comparing the stored deadline at read time. SQLite cleanup,
DynamoDB TTL, or another reaper may reclaim storage only; record expiry or
absence must never be the enforcement signal.

ATK-06 became active in the isolated conformance suite through the pre-PROD-000 founder-authored T0
behavior. PROD-000's agent-authored R5.1 constant/dependency reversal preserves the expected value
and adds a T3 equality assertion; its exact-commit review is pending. The original CORE-004 T0
human, cross-model,
adversarial-test, and three-engine SAST/SCA review was accepted through PR #81,
evidence PR #82, `qa/core-004-t0-founder-review-draft.md`, and
`qa/core-004-t0-independent-human-review-input.md`. Real human approval
delivery and wait, live restart/retry,
cross-instance pending state, and approval-path route-around/non-bypassability
remain in the deferred runtime-integration epic; their exact runtime item is
recorded by the canonical backlog and does not activate before CORE-005 Done
with ATK-01..14 green.

ATK-01/02/03/06/07/08/09/10/11/13 are active; ATK-04/05/12/14 remain deferred,
and ATK-15 remains an external IAM case.
