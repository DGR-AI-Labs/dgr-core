// Bypass case 2 — expired OR replayed token -> must be blocked.
//
// Phase 0: this FAILS because no enforcement is implemented (red by design).
// See tests/bypass/README.md. Do not weaken this test to make it pass.
import { test } from "node:test";
import assert from "node:assert/strict";
import { decide } from "../../src/gate.mjs";

test("expired capability token is blocked", () => {
  const request = {
    action: "transfer_funds",
    token: { id: "tok_expired", expired: true, used: false },
    justification: "approved earlier",
    evidence: ["policy:PAY-1"],
  };

  const decision = decide(request);

  assert.equal(decision.allow, false, "must not allow an expired token");
  assert.equal(decision.outcome, "block");
});

test("replayed (already-used) capability token is blocked", () => {
  const request = {
    action: "transfer_funds",
    token: { id: "tok_once", expired: false, used: true }, // single-use token, replayed
    justification: "approved earlier",
    evidence: ["policy:PAY-1"],
  };

  const decision = decide(request);

  assert.equal(decision.allow, false, "must not allow a replayed token");
  assert.equal(decision.outcome, "block");
});
