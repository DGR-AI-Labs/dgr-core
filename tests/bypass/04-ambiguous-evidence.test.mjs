// Bypass case 4 — ambiguous / insufficient evidence -> must be blocked or escalated.
//
// Phase 0: this FAILS because no enforcement is implemented (red by design).
// See tests/bypass/README.md. Do not weaken this test to make it pass.
import { test } from "node:test";
import assert from "node:assert/strict";
import { decide } from "../../src/gate.mjs";

test("ambiguous / insufficient evidence is blocked or escalated, never allowed", () => {
  const request = {
    action: "transfer_funds",
    token: { id: "tok_valid", expired: false, used: false },
    justification: "looks fine?", // ambiguous
    evidence: [], // insufficient: no policy refs, no provenance (P3)
  };

  const decision = decide(request);

  // SECURE OUTCOME: uncertainty must not resolve to allow (fail-closed, P2).
  assert.equal(decision.allow, false, "ambiguous evidence must not be allowed");
  assert.ok(
    decision.outcome === "block" || decision.outcome === "escalate",
    `expected block or escalate, got: ${decision.outcome}`
  );
});
