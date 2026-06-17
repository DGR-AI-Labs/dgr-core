// Bypass case 5 — the gate/hook itself throws -> must fail closed (block).
//
// A throwing gate must NEVER let the action through and must NOT surface an
// uncaught error in place of a decision: it must resolve to a deny.
//
// Here a signer whose mint() throws models an HSM/key failure on the allow
// path of an otherwise-valid request. The engine must catch it and BLOCK.
// Passes only if the T0 fail-closed guard is correct. Do not weaken.
import { describe, it, expect } from "vitest";
import {
  createDecisionEngine,
  V0_POLICY,
  type ActionRequest,
  type EvidencePacket,
  type TokenSigner,
} from "@dgr/core";

const throwingSigner: TokenSigner = {
  mint() {
    throw new Error("signing backend unavailable");
  },
};

describe("bypass 05 — gate throws", () => {
  it("an internally-faulting gate fails closed (blocks) and does not throw", async () => {
    const engine = createDecisionEngine({ policy: V0_POLICY, signer: throwingSigner });

    // A fully valid pay_invoice request that WOULD be allowed — forcing the
    // allow path, where mint() throws.
    const request: ActionRequest = {
      action: "pay_invoice",
      params: { invoiceId: "INV-1" },
      caller: { agentId: "agent-1" },
    };
    const evidence: EvidencePacket = {
      justification: "approved by finance",
      evidence: [{ kind: "policy", ref: "PAY-1", confidence: 0.95 }],
      provenance: { requestedBy: "agent-1", via: "test", at: "2026-01-01T00:00:00Z" },
    };

    // Must resolve (not reject) to a block.
    const decision = await engine.decide(request, evidence);

    expect(decision.allow).toBe(false);
    expect(decision.outcome).toBe("block");
  });
});
