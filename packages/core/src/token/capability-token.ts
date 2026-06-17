// T0 — enforcement-critical. Human-led: requires human review + cross-model
// review + ≥3 SAST before trusted. Not validated.
//
// DRAFT. First-draft capability-token mint/sign/verify against
// specs/0001-enforcement-spec.md §1 and the bypass suite. Do NOT finalize,
// do NOT merge, do NOT trust until the T0 review gate is satisfied.

import {
  createHash,
  generateKeyPairSync,
  randomUUID,
  sign as cryptoSign,
  verify as cryptoVerify,
  type KeyObject,
} from "node:crypto";
import type { ActionRequest, CapabilityToken, VerificationResult } from "../types.js";
import { PROPOSED_DEFAULTS } from "../constants.js";

export interface SigningKeyPair {
  publicKey: KeyObject;
  privateKey: KeyObject;
}

/** Generate an Ed25519 signing key pair (proposed default — confirm; spec §1). */
export function generateSigningKeyPair(): SigningKeyPair {
  const { publicKey, privateKey } = generateKeyPairSync("ed25519");
  return { publicKey, privateKey };
}

/** Deterministic SHA-256 hash binding a token to specific action params. */
export function canonicalParamsHash(params: Record<string, unknown>): string {
  return sha256(stableStringify(params));
}

export interface MintInput {
  action: string;
  params: Record<string, unknown>;
  audience: string;
  decisionId: string;
  /** Issued-at (epoch ms); injected for testability. */
  now: number;
  ttlMs?: number;
}

export interface TokenSigner {
  mint(input: MintInput): CapabilityToken;
}

/** Create a signer that mints signed, short-lived, single-action capability tokens. */
export function createSigner(privateKey: KeyObject): TokenSigner {
  return {
    mint(input: MintInput): CapabilityToken {
      const iat = input.now;
      const exp = iat + (input.ttlMs ?? PROPOSED_DEFAULTS.TOKEN_TTL_MS);
      const claims: TokenClaims = {
        jti: randomUUID(),
        action: input.action,
        paramsHash: canonicalParamsHash(input.params),
        audience: input.audience,
        decisionId: input.decisionId,
        iat,
        exp,
      };
      const sig = cryptoSign(null, Buffer.from(canonicalClaims(claims)), privateKey).toString("base64");
      return { ...claims, sig };
    },
  };
}

/** Single-use token-id store for replay defense (spec §1, replay prevention). */
export interface ReplayStore {
  has(jti: string): boolean;
  add(jti: string, expEpochMs: number): void;
}

/** In-memory replay store for v0 (a shared store is required for multi-verifier deployments). */
export function createInMemoryReplayStore(now: () => number = () => Date.now()): ReplayStore {
  const used = new Map<string, number>();
  return {
    has(jti: string): boolean {
      const exp = used.get(jti);
      if (exp === undefined) return false;
      if (now() > exp) {
        used.delete(jti);
        return false;
      }
      return true;
    },
    add(jti: string, expEpochMs: number): void {
      used.set(jti, expEpochMs);
    },
  };
}

export interface VerifyTokenInternalOptions {
  publicKey: KeyObject;
  now: number;
  request: ActionRequest;
  replayStore: ReplayStore;
  clockSkewMs?: number;
}

/**
 * Tool-side verification (spec §1.2). FAIL-CLOSED: any failed check, missing
 * token, or thrown error resolves to { ok: false }. Returns { ok: true } only
 * when every check passes, and consumes the token id (single-use) on success.
 */
export function verifyToken(
  token: CapabilityToken | null | undefined,
  opts: VerifyTokenInternalOptions,
): VerificationResult {
  try {
    if (!token) return { ok: false, reason: "no token presented" };

    const claims: TokenClaims = {
      jti: token.jti,
      action: token.action,
      paramsHash: token.paramsHash,
      audience: token.audience,
      decisionId: token.decisionId,
      iat: token.iat,
      exp: token.exp,
    };

    let sigOk = false;
    try {
      sigOk = cryptoVerify(
        null,
        Buffer.from(canonicalClaims(claims)),
        opts.publicKey,
        Buffer.from(token.sig, "base64"),
      );
    } catch {
      sigOk = false;
    }
    if (!sigOk) return { ok: false, reason: "invalid signature" };

    const skew = opts.clockSkewMs ?? PROPOSED_DEFAULTS.CLOCK_SKEW_TOLERANCE_MS;
    if (opts.now > token.exp + skew) return { ok: false, reason: "token expired" };
    if (opts.now + skew < token.iat) return { ok: false, reason: "token not yet valid" };

    if (token.action !== opts.request.action) return { ok: false, reason: "action mismatch" };
    if (token.paramsHash !== canonicalParamsHash(opts.request.params)) {
      return { ok: false, reason: "params mismatch" };
    }
    if (token.audience !== opts.request.caller.agentId) {
      return { ok: false, reason: "audience mismatch" };
    }

    if (opts.replayStore.has(token.jti)) return { ok: false, reason: "token already used (replay)" };
    opts.replayStore.add(token.jti, token.exp);

    return { ok: true };
  } catch (err) {
    // Fail closed on any unexpected error during verification.
    return { ok: false, reason: `verification error: ${errMessage(err)}` };
  }
}

// --- internal helpers ---

interface TokenClaims {
  jti: string;
  action: string;
  paramsHash: string;
  audience: string;
  decisionId: string;
  iat: number;
  exp: number;
}

function canonicalClaims(claims: TokenClaims): string {
  return stableStringify(claims);
}

function sha256(input: string): string {
  return createHash("sha256").update(input).digest("hex");
}

/** Deterministic JSON with lexicographically sorted object keys. */
function stableStringify(value: unknown): string {
  if (value === null || typeof value !== "object") return JSON.stringify(value) ?? "null";
  if (Array.isArray(value)) return "[" + value.map(stableStringify).join(",") + "]";
  const obj = value as Record<string, unknown>;
  const keys = Object.keys(obj).sort();
  return "{" + keys.map((k) => JSON.stringify(k) + ":" + stableStringify(obj[k])).join(",") + "}";
}

function errMessage(err: unknown): string {
  return err instanceof Error ? err.message : String(err);
}
