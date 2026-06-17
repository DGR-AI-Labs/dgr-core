// Bypass case 1 — direct effectful-tool call with NO token -> must be blocked.
//
// Phase 0: this FAILS because no enforcement is implemented (red by design).
// See tests/bypass/README.md. Do not weaken this test to make it pass.
import { test } from "node:test";
import assert from "node:assert/strict";
import { decide } from "../../src/gate.mjs";

test("effectful call with no capability token is blocked", () => {
  const request = {
    action: "transfer_funds", // an effectful, consequential action
    token: undefined, // attacker presents no DGR-issued token
    justification: "n/a",
    evidence: [],
  };

  const decision = decide(request);

  // SECURE OUTCOME: no token => the action must not execute.
  assert.equal(decision.allow, false, "must not allow an untokened effectful call");
  assert.equal(decision.outcome, "block");
});
