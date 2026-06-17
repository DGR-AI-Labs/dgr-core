// Unit tests for the decision engine: the allow path works end-to-end, DGR
// governs non-payment actions (not payment-only), default-deny is fail-closed,
// and degraded mode blocks/queues per spec §2. These are NOT bypass tests; they
// document expected behavior and confirm a legitimate green is achievable.
import { describe, it, expect } from "vitest";
import {
  createDecisionEngine,
  createSigner,
  generateSigningKeyPair,
  createInMemoryReplayStore,
  verifyToken,
  V0_POLICY,
  type ActionRequest,
  type EvidencePacket,
} from "@dgr/core";

function fullEvidence(kind = "policy"): EvidencePacket {
  return {
    justification: "approved",
    evidence: [{ kind, ref: "REF-1", confidence: 0.9 }],
    provenance: { requestedBy: "agent-1", via: "test", at: "2026-01-01T00:00:00Z" },
  };
}

describe("decision engine", () => {
  it("allows a fully-justified payment and mints a verifiable token", async () => {
    const { publicKey, privateKey } = generateSigningKeyPair();
    const engine = createDecisionEngine({
      policy: V0_POLICY,
      signer: createSigner(privateKey),
      clock: () => 1000,
    });
    const request: ActionRequest = {
      action: "pay_invoice",
      params: { invoiceId: "INV-1" },
      caller: { agentId: "agent-1" },
    };

    const decision = await engine.decide(request, fullEvidence());

    expect(decision.outcome).toBe("allow");
    expect(decision.token).toBeDefined();
    const verify = verifyToken(decision.token, {
      publicKey,
      request,
      replayStore: createInMemoryReplayStore(() => 1000),
      clock: () => 1000,
    });
    expect(verify.ok).toBe(true);
  });

  it("governs a NON-payment action (send_bulk_email), proving DGR is not payment-only", async () => {
    const { privateKey } = generateSigningKeyPair();
    const engine = createDecisionEngine({ policy: V0_POLICY, signer: createSigner(privateKey) });
    const request: ActionRequest = {
      action: "send_bulk_email",
      params: { listId: "all" },
      caller: { agentId: "agent-1" },
    };

    const blocked = await engine.decide(request, {
      evidence: [],
      provenance: { requestedBy: "a", via: "test", at: "2026-01-01T00:00:00Z" },
    });
    expect(blocked.allow).toBe(false);

    const allowed = await engine.decide(request, fullEvidence("approval"));
    expect(allowed.outcome).toBe("allow");
  });

  it("denies an ungoverned action by default (fail-closed)", async () => {
    const { privateKey } = generateSigningKeyPair();
    const engine = createDecisionEngine({ policy: V0_POLICY, signer: createSigner(privateKey) });

    const decision = await engine.decide(
      { action: "format_disk", params: {}, caller: { agentId: "x" } },
      { evidence: [], provenance: { requestedBy: "x", via: "test", at: "2026-01-01T00:00:00Z" } },
    );

    expect(decision.outcome).toBe("block");
  });

  it("degraded mode: blocks non-deferrable, queues (escalates) deferrable", async () => {
    const { privateKey } = generateSigningKeyPair();
    const engine = createDecisionEngine({
      policy: V0_POLICY,
      signer: createSigner(privateKey),
      dgrAvailable: () => false,
    });

    const pay = await engine.decide(
      { action: "pay_invoice", params: {}, caller: { agentId: "a" } },
      fullEvidence(),
    );
    expect(pay.outcome).toBe("block");
    expect(pay.degraded).toBe(true);

    const email = await engine.decide(
      { action: "send_bulk_email", params: {}, caller: { agentId: "a" } },
      fullEvidence("approval"),
    );
    expect(email.outcome).toBe("escalate");
    expect(email.degraded).toBe(true);
  });
});
