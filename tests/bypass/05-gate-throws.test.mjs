// Bypass case 5 — the gate/hook itself throws -> must fail closed (block).
//
// A throwing gate must NEVER let the action through and must NOT surface an
// uncaught error in place of a decision: it must resolve to a deny.
//
// Phase 0: this FAILS because no enforcement is implemented (the stub throws
// rather than returning a fail-closed block) — red by design.
// See tests/bypass/README.md. Do not weaken this test to make it pass.
import { test } from "node:test";
import assert from "node:assert/strict";
import { decide } from "../../src/gate.mjs";

test("an internally-faulting gate fails closed (blocks), does not throw uncaught", () => {
  const request = {
    action: "transfer_funds",
    token: { id: "tok_valid", expired: false, used: false },
    justification: "approved",
    evidence: ["policy:PAY-1"],
    // Signals the gate to simulate an internal fault on the decision path.
    // The secure contract: faults resolve to BLOCK, not to an uncaught throw
    // and never to allow.
    simulateInternalFault: true,
  };

  let decision;
  assert.doesNotThrow(() => {
    decision = decide(request);
  }, "a faulting gate must fail closed, not propagate an uncaught error");

  assert.equal(decision.allow, false, "a faulting gate must not allow");
  assert.equal(decision.outcome, "block");
});
