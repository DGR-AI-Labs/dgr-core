// Bypass case 1 — direct effectful-tool call with NO token -> must be blocked.
//
// Asserts the SECURE outcome (tool-side verification rejects an absent token).
// Passes only if the T0 verification draft is correct. Do not weaken.
import { describe, it, expect } from "vitest";
import {
  generateSigningKeyPair,
  createInMemoryReplayStore,
  verifyToken,
  type ActionRequest,
} from "@dgr/core";

const request: ActionRequest = {
  action: "pay_invoice",
  params: { invoiceId: "INV-1", amount: 100 },
  caller: { agentId: "agent-1" },
};

describe("bypass 01 — no token", () => {
  it("a call presenting no capability token is blocked", () => {
    const { publicKey } = generateSigningKeyPair();
    const replayStore = createInMemoryReplayStore();

    const result = verifyToken(undefined, { publicKey, request, replayStore });

    expect(result.ok).toBe(false);
  });
});
