# T0 boundary — read before implementing this harness

This directory is a **T3 test-data and interface scaffold** for CORE-001 and
CORE-002. It is not the DGR gate and must never become an alternate
enforcement path. `before_tool_call.rs` can invoke only a test probe and merely
relays a supplied decision; it contains no decision or token logic.

The following remain **T0, founder-led, and review-only once implemented**:

- capability-token minting or verification;
- the policy decision point;
- the fail-closed runtime guard or native `before_tool_call` hook;
- audit/hash-chain construction or verification; and
- any implementation that can authorize or block a consequential action.

The founder-only CORE-002 units are listed exactly in the repository-root
`T0-AUTHORS.md`. Their committed bodies are `unimplemented!()` defaults marked
`{FOUNDER-AUTHORS}`. ATK-01 is active and deliberately red against those
defaults; ATK-02 through ATK-14 remain ignored, and ATK-15 remains an external
IAM case. CORE-005 may wire conformance into required CI only after the T0
human gate.

Do not make a test pass by adding enforcement behavior here. If an attack
requires gate internals, record the dependency and stop.

ATK-15 is intentionally an external IAM assertion. It belongs in the hosted
infrastructure test suite and must not be simulated as gate behavior.
