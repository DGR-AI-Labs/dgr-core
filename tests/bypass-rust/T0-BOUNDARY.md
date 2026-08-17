# T0 boundary — read before implementing this harness

This directory combines the **T3 test-data and conformance harness** for
CORE-001/CORE-002 with the five explicitly marked, founder-owned T0 units. It
is not the deployed DGR gate and must never become an alternate production
enforcement path. `before_tool_call.rs` can invoke only a test probe and merely
relays a supplied decision; it contains no decision or token logic.

The following remain **T0, founder-led, and review-only once implemented**:

- capability-token minting or verification;
- the policy decision point;
- the fail-closed runtime guard or native `before_tool_call` hook;
- audit/hash-chain construction or verification; and
- any implementation that can authorize or block a consequential action.

The founder-only CORE-002 units and their current state are listed exactly in
the repository-root `T0-AUTHORS.md`. All five contain founder-authored
enforcement but remain pending the full T0 gate. ATK-01/02/03/08/09/10/11/13
are active, ATK-04/05/06/07/12/14 remain deferred, and ATK-15 remains an
external IAM case. CORE-005 may wire conformance into required CI only after
the T0 human gate.

Agents must not make a test pass by adding or changing enforcement behavior in
the founder-owned units. If an attack requires unresolved gate internals,
record the dependency and stop. Founder changes remain subject to the T0
process recorded in the repository constitution.

ATK-15 is intentionally an external IAM assertion. It belongs in the hosted
infrastructure test suite and must not be simulated as gate behavior.
