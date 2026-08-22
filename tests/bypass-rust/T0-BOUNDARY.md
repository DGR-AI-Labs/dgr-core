# T0 boundary — read before implementing this harness

This directory combines the **T3 test-data and conformance harness** with the
founder-owned T0 units listed in the repository-root `T0-AUTHORS.md`. It is not
the deployed DGR gate and must never become an alternate production enforcement
path. `before_tool_call.rs` can invoke only a test probe, never a real tool.

`BeforeToolCallAdapter::before_tool_call` now contains the founder-authored
CORE-003 T0 floor. Returned `Ok(Deny | Allow)` decisions retain their established
relay behavior. `Err(GuardFault)` and an unwinding panic from `decide` produce
`BeforeToolCallObservation::Blocked` with `RequiredOutcome::FailClosed`, no
authorization, and zero effectful invocations.

The unwind guarantee is bounded to Rust unwinding panics after the boundary is
reached. It does not claim protection from `panic=abort`, process termination,
OOM abort, a hook that never fires, or a route around the hook. It also does not
certify a store's post-panic invariants for later reuse.

The following remain **T0, founder-led, and review-only once implemented**:

- capability-token minting or verification;
- the policy decision point;
- the fail-closed runtime guard or native `before_tool_call` hook;
- audit/hash-chain construction or verification; and
- any implementation that can authorize or block a consequential action.

The founder-owned CORE-002 units, CORE-003 boundary method, and CORE-004
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

Agents must not make a test pass by adding or changing enforcement behavior in
the founder-owned units. If an attack requires unresolved gate internals,
record the dependency and stop. Founder changes remain subject to the T0
process recorded in the repository constitution.

CORE-003 covers ATK-07 only: the boundary was reached, but its guard/verifier
returned a fault or panicked. A hook that never fires, a route around the hook,
a missing plugin, or operator bypass is runtime-integration scope retained by
RUNTIME-003/004, not simulated by this isolation harness.

## CORE-004 ownership and bounded claim

CORE-004 covers ATK-06's timeout-only isolation contract. Its
`Escalated` observation, durable pending-store behavior, escalation and
timeout decisions, R-3 timeout-evaluation path, and adapter behavior that
emits `Escalated` are founder-authored T0. In particular, `Escalated` is not an
authorization and must not invoke the effectful probe.

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

ATK-06 is active in the isolated conformance suite after founder-authored T0
behavior turned the reviewed RED tests green without changing the
registry-derived expectation. Its complete T0 human, cross-model,
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
