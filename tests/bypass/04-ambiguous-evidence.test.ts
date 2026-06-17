// Bypass case 4 — ambiguous / insufficient evidence -> must be blocked or escalated.
//
// Asserts the SECURE outcome: uncertainty must never resolve to allow (fail-
// closed). Passes only if the T0 decision-point draft is correct. Do not weaken.
import { describe, it, expect } from "vitest";
import {
  createDecisionEngine,
  createSigner,
  generateSigningKeyPair,
  V0_POLICY,
  type ActionRequest,
  type EvidencePacket,
} from "@dgr/core";

describe("bypass 04 — ambiguous / insufficient evidence", () => {
  it("low-confidence evidence is escalated or blocked, never allowed", async () => {
    const { privateKey } = generateSigningKeyPair();
    const engine = createDecisionEngine({ policy: V0_POLICY, signer: createSigner(privateKey) });

    const request: ActionRequest = {
      action: "pay_invoice",
      params: { invoiceId: "INV-1" },
      caller: { agentId: "agent-1" },
    };
    const evidence: EvidencePacket = {
      justification: "vendor says it is due",
      // required kind present, but confidence is below the rule's threshold → ambiguous
      evidence: [{ kind: "policy", ref: "PAY-1", confidence: 0.2 }],
      provenance: { requestedBy: "agent-1", via: "test", at: "2026-01-01T00:00:00Z" },
    };

    const decision = await engine.decide(request, evidence);

    expect(decision.allow).toBe(false);
    expect(["block", "escalate", "request-evidence"]).toContain(decision.outcome);
  });
});
