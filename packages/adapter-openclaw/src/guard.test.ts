// Unit tests for the OpenClaw before_tool_call guard: approve on allow,
// block on missing justification, deny on timeout (fail-closed).
import { describe, it, expect } from "vitest";
import { createOpenClawGuard } from "@dgr/openclaw";
import {
  createDecisionEngine,
  createSigner,
  generateSigningKeyPair,
  V0_POLICY,
  type DecisionEngine,
} from "@dgr/core";

function realEngine(): DecisionEngine {
  const { privateKey } = generateSigningKeyPair();
  return createDecisionEngine({ policy: V0_POLICY, signer: createSigner(privateKey) });
}

const approvedEvidence = {
  justification: "approved",
  evidence: [{ kind: "policy", ref: "P", confidence: 0.9 }],
  provenance: { requestedBy: "a", via: "openclaw", at: "2026-01-01T00:00:00Z" },
};

describe("openclaw guard", () => {
  it("approves an allowed call and attaches a capability token", async () => {
    const guard = createOpenClawGuard({ engine: realEngine() });

    const res = await guard.beforeToolCall(
      { toolName: "pay_invoice", args: { invoiceId: "INV-1" }, metadata: { dgr: approvedEvidence } },
      { agentId: "agent-1" },
    );

    expect(res.decision).toBe("approve");
    if (res.decision === "approve") expect(res.dgrToken).toBeDefined();
  });

  it("blocks when justification is missing (fail-closed)", async () => {
    const guard = createOpenClawGuard({ engine: realEngine() });

    const res = await guard.beforeToolCall(
      {
        toolName: "pay_invoice",
        args: { invoiceId: "INV-1" },
        metadata: {
          dgr: {
            evidence: [{ kind: "policy", ref: "P", confidence: 0.9 }],
            provenance: { requestedBy: "a", via: "openclaw", at: "2026-01-01T00:00:00Z" },
          },
        },
      },
      { agentId: "agent-1" },
    );

    expect(res.decision).toBe("block");
  });

  it("denies on timeout (decision never arrives)", async () => {
    const hangingEngine = { decide: () => new Promise<never>(() => {}) } as unknown as DecisionEngine;
    const guard = createOpenClawGuard({ engine: hangingEngine, timeoutMs: 10 });

    const res = await guard.beforeToolCall(
      { toolName: "pay_invoice", args: {}, metadata: {} },
      { agentId: "a" },
    );

    expect(res.decision).toBe("block");
  });
});
