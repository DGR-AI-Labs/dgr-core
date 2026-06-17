// Bypass case 3 — missing justification -> must be blocked.
//
// Phase 0: this FAILS because no enforcement is implemented (red by design).
// See tests/bypass/README.md. Do not weaken this test to make it pass.
import { test } from "node:test";
import assert from "node:assert/strict";
import { decide } from "../../src/gate.mjs";

test("request with no justification is blocked", () => {
  const request = {
    action: "transfer_funds",
    token: { id: "tok_valid", expired: false, used: false },
    justification: undefined, // evidence-based decisions require justification (P3)
    evidence: ["policy:PAY-1"],
  };

  const decision = decide(request);

  assert.equal(decision.allow, false, "must not allow without justification");
  assert.equal(decision.outcome, "block");
});
