// Bypass case 2 — expired OR replayed token -> must be blocked.
//
// Asserts the SECURE outcome. Passes only if the T0 verification draft is
// correct (expiry + skew check, single-use jti replay defense). Do not weaken.
import { describe, it, expect } from "vitest";
import {
  generateSigningKeyPair,
  createSigner,
  createInMemoryReplayStore,
  verifyToken,
  type ActionRequest,
} from "@dgr/core";

const request: ActionRequest = {
  action: "pay_invoice",
  params: { invoiceId: "INV-1" },
  caller: { agentId: "agent-1" },
};
const T0 = 1_000_000;

describe("bypass 02 — expired / replayed token", () => {
  it("an expired token is blocked", () => {
    const { publicKey, privateKey } = generateSigningKeyPair();
    const token = createSigner(privateKey).mint({
      action: request.action,
      params: request.params,
      audience: "agent-1",
      decisionId: "d1",
      now: T0,
      ttlMs: 30_000,
    });
    const replayStore = createInMemoryReplayStore();

    // Well past exp (T0 + 30s) plus the clock-skew tolerance.
    const result = verifyToken(token, {
      publicKey,
      request,
      replayStore,
      clock: () => T0 + 30_000 + 60_000,
    });

    expect(result.ok).toBe(false);
  });

  it("a replayed (single-use) token is blocked on the second use", () => {
    const { publicKey, privateKey } = generateSigningKeyPair();
    const token = createSigner(privateKey).mint({
      action: request.action,
      params: request.params,
      audience: "agent-1",
      decisionId: "d1",
      now: T0,
    });
    const replayStore = createInMemoryReplayStore(() => T0);
    const clock = () => T0 + 1; // within validity

    const first = verifyToken(token, { publicKey, request, replayStore, clock });
    const second = verifyToken(token, { publicKey, request, replayStore, clock });

    expect(first.ok).toBe(true);
    expect(second.ok).toBe(false);
  });
});
