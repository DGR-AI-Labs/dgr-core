// T0 — enforcement-critical. Human-led: requires human review + cross-model
// review + ≥3 SAST before trusted. Not validated.
//
// DRAFT. Fail-closed guard + degraded-mode helpers against
// specs/0001-enforcement-spec.md §2. Do NOT finalize, do NOT merge, do NOT
// trust until the T0 review gate is satisfied. The contract: any error,
// timeout, or ambiguity resolves to a BLOCK — never an allow.

import { randomUUID } from "node:crypto";
import type { ActionRequest, DecisionObject, EvidencePacket } from "../types.js";

/** Build a fail-closed BLOCK decision object. */
export function failClosedBlock(
  request: ActionRequest | undefined,
  evidence: EvidencePacket,
  reason: string,
  now: number,
): DecisionObject {
  return {
    id: randomUUID(),
    outcome: "block",
    allow: false,
    action: request?.action ?? "<unknown>",
    reason,
    policyRefs: [],
    reasoning: ["fail-closed: blocked due to error, timeout, or uncertainty"],
    evidence,
    caller: request?.caller ?? { agentId: "<unknown>" },
    decidedAt: new Date(now).toISOString(),
    degraded: true,
  };
}

/**
 * Run an evaluation and guarantee a decision is returned even if it throws.
 * On any thrown error, returns the fail-closed BLOCK from `onError`.
 */
export async function guardFailClosed(
  evaluate: () => DecisionObject | Promise<DecisionObject>,
  onError: (err: unknown) => DecisionObject,
): Promise<DecisionObject> {
  try {
    return await evaluate();
  } catch (err) {
    return onError(err);
  }
}

/**
 * Race an async evaluation against the fail-closed deadline (spec §3). If the
 * deadline elapses first, resolves to the fail-closed BLOCK. Used for remote
 * DGR calls; the local v0 evaluation is synchronous and effectively immediate.
 */
export async function withDeadline(
  evaluate: () => Promise<DecisionObject>,
  deadlineMs: number,
  onTimeout: () => DecisionObject,
): Promise<DecisionObject> {
  let timer: ReturnType<typeof setTimeout> | undefined;
  const timeout = new Promise<DecisionObject>((resolve) => {
    timer = setTimeout(() => resolve(onTimeout()), deadlineMs);
  });
  try {
    return await Promise.race([evaluate(), timeout]);
  } finally {
    if (timer) clearTimeout(timer);
  }
}
