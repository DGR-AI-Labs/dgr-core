// T0 — enforcement-critical. Human-led: requires human review + cross-model
// review + ≥3 SAST before trusted. Not validated.
//
// DRAFT. First-draft policy decision point (allow / block / escalate /
// request-evidence) against specs/0001-enforcement-spec.md and the bypass
// suite. Do NOT finalize, do NOT merge, do NOT trust until the T0 review gate
// is satisfied. Fail-closed bias: when in doubt, this MUST NOT allow.

import { randomUUID } from "node:crypto";
import type {
  ActionRequest,
  DecisionObject,
  DecisionOutcome,
  EvidencePacket,
  PolicyRule,
} from "../types.js";
import type { CompiledPolicy } from "../policy/bundle.js";
import type { TokenSigner } from "../token/capability-token.js";

export interface EvaluateContext {
  policy: CompiledPolicy;
  signer: TokenSigner;
  /** Decision timestamp (epoch ms); injected for testability. */
  now: number;
  tokenTtlMs?: number;
  /** Whether DGR's decision authority is currently reachable (spec §2). */
  dgrAvailable: boolean;
}

/**
 * Evaluate a single authorization request. Pure given its context except for
 * token minting on the allow path. MAY THROW (e.g., signer failure) — callers
 * MUST wrap this in the fail-closed guard so any throw resolves to a block.
 */
export function evaluateDecision(
  request: ActionRequest,
  evidence: EvidencePacket,
  ctx: EvaluateContext,
): DecisionObject {
  const reasoning: string[] = [];
  const decidedAt = new Date(ctx.now).toISOString();
  const id = randomUUID();

  const make = (
    outcome: DecisionOutcome,
    allow: boolean,
    reason: string,
    policyRefs: string[],
    degraded?: boolean,
  ): DecisionObject => ({
    id,
    outcome,
    allow,
    action: request.action,
    reason,
    policyRefs,
    reasoning: [...reasoning],
    evidence,
    caller: request.caller,
    decidedAt,
    degraded,
  });

  const rule = ctx.policy.match(request.action);
  if (!rule) {
    reasoning.push("no matching policy rule");
    return make("block", false, "no policy rule matches; fail-closed default deny", []);
  }
  reasoning.push(`matched rule ${rule.id} (effect=${rule.effect})`);

  // Degraded mode (spec §2): DGR unavailable.
  if (!ctx.dgrAvailable) {
    if (rule.deferrable) {
      reasoning.push("degraded mode: deferrable action queued for replay");
      return make("escalate", false, "DGR unavailable; deferrable action queued", [rule.id], true);
    }
    reasoning.push("degraded mode: non-deferrable action blocked");
    return make("block", false, "DGR unavailable; fail-closed block", [rule.id], true);
  }

  if (rule.effect === "deny") {
    return make("block", false, `denied by policy rule ${rule.id}`, [rule.id]);
  }
  if (rule.effect === "allow") {
    reasoning.push("rule allows unconditionally");
    return allow(rule);
  }

  // effect === "require-authorization"
  const req = rule.requires ?? {};

  if (req.justification && !hasText(evidence.justification)) {
    reasoning.push("justification required but absent/empty");
    return make("block", false, "missing justification", [rule.id]);
  }

  const items = evidence.evidence ?? [];

  if (req.evidenceKinds && req.evidenceKinds.length > 0) {
    const present = new Set(items.map((i) => i.kind));
    const missing = req.evidenceKinds.filter((k) => !present.has(k));
    if (missing.length > 0) {
      reasoning.push(`required evidence kind(s) absent: ${missing.join(", ")}`);
      return make("block", false, `missing required evidence: ${missing.join(", ")}`, [rule.id]);
    }
  }

  if (typeof req.minEvidence === "number" && items.length < req.minEvidence) {
    reasoning.push(`evidence count ${items.length} < required ${req.minEvidence}`);
    return make("escalate", false, "insufficient evidence; escalate for review", [rule.id]);
  }

  if (typeof req.minConfidence === "number") {
    const agg = aggregateConfidence(items);
    if (agg < req.minConfidence) {
      reasoning.push(`aggregate confidence ${agg} < required ${req.minConfidence}`);
      return make("escalate", false, "ambiguous/low-confidence evidence; escalate", [rule.id]);
    }
  }

  reasoning.push("all authorization requirements satisfied");
  return allow(rule);

  function allow(matched: PolicyRule): DecisionObject {
    // Mint may throw; the fail-closed guard upstream converts that to a block.
    const token = ctx.signer.mint({
      action: request.action,
      params: request.params,
      audience: request.caller.agentId,
      decisionId: id,
      now: ctx.now,
      ttlMs: ctx.tokenTtlMs,
    });
    return {
      ...make("allow", true, "authorized", [matched.id]),
      token,
    };
  }
}

function hasText(value: string | undefined): boolean {
  return typeof value === "string" && value.trim().length > 0;
}

/** Conservative aggregate confidence: the weakest evidence item (missing confidence = 0). */
function aggregateConfidence(items: ReadonlyArray<{ confidence?: number }>): number {
  if (items.length === 0) return 0;
  let min = 1;
  for (const item of items) {
    const c = typeof item.confidence === "number" ? item.confidence : 0;
    if (c < min) min = c;
  }
  return min;
}
