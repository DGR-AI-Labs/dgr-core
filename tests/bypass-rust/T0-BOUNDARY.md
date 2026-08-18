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

The five founder-owned CORE-002 units and the founder-owned CORE-003 boundary
method are listed in the repository-root `T0-AUTHORS.md`. They contain
founder-authored enforcement but remain pending the applicable T0 review gates.
CORE-005 may wire conformance into required CI only after those human gates.

Agents must not make a test pass by adding or changing enforcement behavior in
the founder-owned units. If an attack requires unresolved gate internals,
record the dependency and stop. Founder changes remain subject to the T0
process recorded in the repository constitution.

CORE-003 covers ATK-07 only: the boundary was reached, but its guard/verifier
returned a fault or panicked. A hook that never fires, a route around the hook,
a missing plugin, or operator bypass is runtime-integration scope retained by
RUNTIME-003/004, not simulated by this isolation harness.

ATK-01/02/03/07/08/09/10/11/13 are active; ATK-04/05/06/12/14 remain deferred,
and ATK-15 remains an external IAM case.
