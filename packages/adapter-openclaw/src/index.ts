// T1 — review required. (Draft for human review.)
//
// Thin OpenClaw `before_tool_call` adapter for @dgr/core.
//
// HONEST THREAT MODEL (read before trusting this):
//   • This enforces against the AGENT: an agent driven through OpenClaw cannot
//     reach a governed effectful tool without a DGR "allow" decision (and the
//     capability token that comes with it).
//   • It is OPERATOR-BYPASSABLE: a privileged operator who controls the runtime
//     can simply not register this hook, edit the runtime, or call tools out of
//     band. That is out of scope at this tier (spec §5) and is Phase ≥2 work.
//   • Therefore this is DEVELOPER-GRADE, not a security guarantee. Do NOT
//     describe it as "non-bypassable", "secure", "production", or "compliant".
//
// The OpenClaw hook payload/return shapes below are a MINIMAL MODEL and an
// ASSUMPTION pending confirmation against OpenClaw's actual before_tool_call
// API. TODO(adapter): pin these to the real OpenClaw types.

import {
  type ActionRequest,
  type CapabilityToken,
  type DecisionObject,
  type DecisionEngine,
  type EvidencePacket,
} from "@dgr/core";

/** A tool call intercepted before execution. (ASSUMPTION — confirm vs OpenClaw.) */
export interface OpenClawToolCall {
  toolName: string;
  args: Record<string, unknown>;
  metadata?: Record<string, unknown>;
}

/** Runtime context for the hook. (ASSUMPTION — confirm vs OpenClaw.) */
export interface OpenClawHookContext {
  agentId: string;
  sessionId?: string;
}

/** What the adapter tells OpenClaw to do with the call. */
export type BeforeToolCallResult =
  | { decision: "approve"; args: Record<string, unknown>; dgrToken: CapabilityToken }
  | { decision: "narrow"; args: Record<string, unknown>; reason: string; dgrToken: CapabilityToken }
  | { decision: "block"; reason: string };

export interface OpenClawGuardOptions {
  engine: DecisionEngine;
  /** Map a tool call to a DGR action name. Default: the tool name verbatim. */
  actionOf?: (call: OpenClawToolCall) => string;
  /** Extract the evidence packet. Default: reads call.metadata.dgr as an EvidencePacket. */
  evidenceOf?: (call: OpenClawToolCall, ctx: OpenClawHookContext) => EvidencePacket;
  /** Optional parameter narrowing applied on approve (e.g., clamp an amount). */
  narrow?: (call: OpenClawToolCall, decision: DecisionObject) => Record<string, unknown> | undefined;
  /** Hard deadline for the whole hook (ms). On timeout the call is DENIED (fail closed). */
  timeoutMs?: number;
}

export interface OpenClawGuard {
  beforeToolCall(call: OpenClawToolCall, ctx: OpenClawHookContext): Promise<BeforeToolCallResult>;
}

const DEFAULT_HOOK_TIMEOUT_MS = 1_000;

/**
 * Build the before_tool_call guard. Fail-closed end-to-end: anything other than
 * a clean "allow" (block, escalate, request-evidence, timeout, or a thrown
 * error) results in the tool call being DENIED.
 */
export function createOpenClawGuard(opts: OpenClawGuardOptions): OpenClawGuard {
  const actionOf = opts.actionOf ?? ((call) => call.toolName);
  const evidenceOf = opts.evidenceOf ?? defaultEvidenceOf;
  const timeoutMs = opts.timeoutMs ?? DEFAULT_HOOK_TIMEOUT_MS;

  return {
    async beforeToolCall(call, ctx): Promise<BeforeToolCallResult> {
      try {
        const request: ActionRequest = {
          action: actionOf(call),
          params: call.args,
          caller: { agentId: ctx.agentId, sessionId: ctx.sessionId },
        };
        const evidence = evidenceOf(call, ctx);

        const decision = await withTimeout(
          () => opts.engine.decide(request, evidence),
          timeoutMs,
        );

        // Approve-with-deny-on-timeout: a null here means the deadline elapsed.
        if (!decision) {
          return { decision: "block", reason: "DGR decision timed out; fail-closed deny" };
        }
        if (decision.outcome !== "allow" || !decision.token) {
          return { decision: "block", reason: decision.reason || `not authorized (${decision.outcome})` };
        }

        const narrowed = opts.narrow?.(call, decision);
        if (narrowed) {
          return {
            decision: "narrow",
            args: narrowed,
            reason: "approved with narrowed parameters",
            dgrToken: decision.token,
          };
        }
        return { decision: "approve", args: call.args, dgrToken: decision.token };
      } catch (err) {
        // The hook itself threw — deny (fail closed), never let the call through.
        return { decision: "block", reason: `DGR hook error; fail-closed deny: ${errMessage(err)}` };
      }
    },
  };
}

/** Default evidence extraction: expects an EvidencePacket at call.metadata.dgr. */
function defaultEvidenceOf(call: OpenClawToolCall): EvidencePacket {
  const raw = call.metadata?.["dgr"];
  if (raw && typeof raw === "object") return raw as EvidencePacket;
  // No evidence supplied → an empty packet → the engine will block/escalate.
  return { evidence: [], provenance: { requestedBy: "unknown", via: "openclaw", at: new Date().toISOString() } };
}

/** Resolve to null if the work does not finish within `ms`. */
async function withTimeout<T>(work: () => Promise<T>, ms: number): Promise<T | null> {
  let timer: ReturnType<typeof setTimeout> | undefined;
  const timeout = new Promise<null>((resolve) => {
    timer = setTimeout(() => resolve(null), ms);
  });
  try {
    return await Promise.race([work(), timeout]);
  } finally {
    if (timer) clearTimeout(timer);
  }
}

function errMessage(err: unknown): string {
  return err instanceof Error ? err.message : String(err);
}
