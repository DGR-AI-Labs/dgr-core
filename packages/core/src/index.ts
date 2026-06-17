// T1 — review required.
//
// Public API surface for @dgr/core. Composes the T0 enforcement-critical
// modules (token, decision point, fail-closed) behind a small, reviewable
// facade. The T0 internals are DRAFT and unvalidated until the human gate is
// satisfied; this facade is intentionally thin so the trusted boundary is easy
// to audit.

import type { KeyObject } from "node:crypto";
import type {
  ActionRequest,
  CapabilityToken,
  DecisionObject,
  EvidencePacket,
  PolicyBundle,
  VerificationResult,
} from "./types.js";
import { compilePolicy, type CompiledPolicy } from "./policy/bundle.js";
import { evaluateDecision } from "./decision/decision-point.js";
import { failClosedBlock, guardFailClosed } from "./decision/fail-closed.js";
import {
  verifyToken as verifyTokenInternal,
  type ReplayStore,
  type TokenSigner,
} from "./token/capability-token.js";

// --- Re-exports: types, schemas, constants, policy, token utilities ---
export * from "./types.js";
export { PROPOSED_DEFAULTS } from "./constants.js";
export type { ProposedDefaults } from "./constants.js";
export { V0_POLICY } from "./policy/v0-policy.js";
export {
  compilePolicy,
  parsePolicy,
  validateBundle,
  PolicyBundleError,
  type CompiledPolicy,
} from "./policy/bundle.js";
export {
  generateSigningKeyPair,
  createSigner,
  createInMemoryReplayStore,
  canonicalParamsHash,
  type SigningKeyPair,
  type TokenSigner,
  type ReplayStore,
  type MintInput,
} from "./token/capability-token.js";

// --- Decision engine ---

export interface DecisionEngineOptions {
  policy: PolicyBundle | CompiledPolicy;
  signer: TokenSigner;
  /** Clock source (epoch ms). Defaults to Date.now. Injectable for tests. */
  clock?: () => number;
  tokenTtlMs?: number;
  /** Returns whether DGR's decision authority is reachable (spec §2 degraded mode). */
  dgrAvailable?: () => boolean;
}

export interface DecisionEngine {
  /**
   * Decide an action request. ALWAYS fail-closed: any internal error resolves
   * to a BLOCK decision and never throws. Returns an allow only when policy and
   * evidence requirements are fully satisfied (with a minted capability token).
   */
  decide(request: ActionRequest, evidence: EvidencePacket): Promise<DecisionObject>;
}

export function createDecisionEngine(opts: DecisionEngineOptions): DecisionEngine {
  const policy: CompiledPolicy = isCompiled(opts.policy) ? opts.policy : compilePolicy(opts.policy);
  const clock = opts.clock ?? (() => Date.now());
  const dgrAvailable = opts.dgrAvailable ?? (() => true);

  return {
    decide(request: ActionRequest, evidence: EvidencePacket): Promise<DecisionObject> {
      const now = clock();
      return guardFailClosed(
        () =>
          evaluateDecision(request, evidence, {
            policy,
            signer: opts.signer,
            now,
            tokenTtlMs: opts.tokenTtlMs,
            dgrAvailable: dgrAvailable(),
          }),
        () => failClosedBlock(request, evidence, "internal error during decision; fail-closed", now),
      );
    },
  };
}

// --- Tool-side verification ---

export interface VerifyTokenOptions {
  publicKey: KeyObject;
  request: ActionRequest;
  replayStore: ReplayStore;
  clock?: () => number;
  clockSkewMs?: number;
}

/**
 * Tool-side capability-token verification. FAIL-CLOSED: returns { ok: false }
 * for a missing/invalid/expired/replayed/mis-bound token or on any error.
 */
export function verifyToken(
  token: CapabilityToken | null | undefined,
  opts: VerifyTokenOptions,
): VerificationResult {
  const now = (opts.clock ?? (() => Date.now()))();
  return verifyTokenInternal(token, {
    publicKey: opts.publicKey,
    now,
    request: opts.request,
    replayStore: opts.replayStore,
    clockSkewMs: opts.clockSkewMs,
  });
}

function isCompiled(policy: PolicyBundle | CompiledPolicy): policy is CompiledPolicy {
  return typeof (policy as CompiledPolicy).match === "function";
}
