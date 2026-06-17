// Bypass case 3 — missing justification -> must be blocked.
//
// Asserts the SECURE outcome via the decision engine. Passes only if the T0
// decision-point draft enforces the evidence requirement. Do not weaken.
import { describe, it, expect } from "vitest";
import {
  createDecisionEngine,
  createSigner,
  generateSigningKeyPair,
  V0_POLICY,
  type ActionRequest,
  type EvidencePacket,
} from "@dgr/core";

describe("bypass 03 — missing justification", () => {
  it("a payment request with no justification is blocked", async () => {
    const { privateKey } = generateSigningKeyPair();
    const engine = createDecisionEngine({ policy: V0_POLICY, signer: createSigner(privateKey) });

    const request: ActionRequest = {
      action: "pay_invoice",
      params: { invoiceId: "INV-1" },
      caller: { agentId: "agent-1" },
    };
    const evidence: EvidencePacket = {
      // justification deliberately omitted
      evidence: [{ kind: "policy", ref: "PAY-1", confidence: 0.9 }],
      provenance: { requestedBy: "agent-1", via: "test", at: "2026-01-01T00:00:00Z" },
    };

    const decision = await engine.decide(request, evidence);

    expect(decision.allow).toBe(false);
    expect(decision.outcome).toBe("block");
  });
});
