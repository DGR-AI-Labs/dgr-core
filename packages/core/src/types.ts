// T1 — review required.
//
// Schemas / interfaces for the DGR decision engine: DecisionObject,
// EvidencePacket, PolicyBundle, the decision result, and supporting types.
// These shape the enforcement contract, so they require human review before
// being trusted (Constitution P8). They are not, by themselves, the T0
// enforcement-critical code — that lives in token/, decision/, and fail-closed.

/** A consequential action an agent is attempting to take. */
export interface ActionRequest {
  /** Action identifier, e.g. "pay_invoice", "make_purchase", "send_bulk_email", "read_secret". */
  action: string;
  /** Action parameters. The decision binds a granted token to a hash of these. */
  params: Record<string, unknown>;
  /** Who/what is requesting the action. */
  caller: CallerIdentity;
}

/** Identity of the requesting principal. */
export interface CallerIdentity {
  /** Stable agent identifier (token audience). */
  agentId: string;
  /** Optional session/run identifier for provenance. */
  sessionId?: string;
}

// --- Evidence (Constitution P3: inputs, policy references, reasoning, provenance) ---

/** Evidence supporting an authorization request. */
export interface EvidencePacket {
  /** Free-text rationale for why the action should be permitted. Required by most rules. */
  justification?: string;
  /** Structured supporting evidence. */
  evidence: EvidenceItem[];
  /** Chain-of-custody for the request. */
  provenance: Provenance;
}

export interface EvidenceItem {
  /** e.g. "policy", "approval", "document", "risk-score". */
  kind: string;
  /** Pointer to the evidence (policy id, approval id, URL, hash, …). */
  ref: string;
  /** Optional confidence 0..1; used to detect ambiguous/insufficient evidence. */
  confidence?: number;
}

export interface Provenance {
  /** Principal that originated the request. */
  requestedBy: string;
  /** Identity/capability used to make the request. */
  via: string;
  /** ISO-8601 timestamp asserted by the caller (engine may re-stamp on decision). */
  at: string;
}

// --- Policy ---

export interface PolicyBundle {
  /** Bundle version, for audit/provenance. */
  version: string;
  rules: PolicyRule[];
}

export type PolicyEffect = "require-authorization" | "allow" | "deny";

export interface PolicyRule {
  /** Stable rule id, recorded in the decision's policyRefs. */
  id: string;
  /** Action this rule matches, or "*" for a catch-all. */
  action: string;
  /** What the rule does when matched. */
  effect: PolicyEffect;
  /** Requirements that must hold for a "require-authorization" rule to allow. */
  requires?: PolicyRequirements;
  /** If true, this action may be QUEUED (not executed) in degraded mode. Spec §2. */
  deferrable?: boolean;
  description?: string;
}

export interface PolicyRequirements {
  /** A non-empty justification is required. */
  justification?: boolean;
  /** Minimum number of evidence items. */
  minEvidence?: number;
  /** Evidence kinds that must all be present. */
  evidenceKinds?: string[];
  /** Minimum aggregate evidence confidence (0..1); below this is "ambiguous". */
  minConfidence?: number;
}

// --- Decision result ---

/** The four possible outcomes of a governed decision. */
export type DecisionOutcome = "allow" | "block" | "escalate" | "request-evidence";

/** The durable decision record (Constitution P4: audit-ready). */
export interface DecisionObject {
  /** Unique decision id. */
  id: string;
  /** Decision outcome. */
  outcome: DecisionOutcome;
  /** Convenience flag; always equal to (outcome === "allow"). */
  allow: boolean;
  /** Action that was evaluated. */
  action: string;
  /** Human-readable reason for the outcome. */
  reason: string;
  /** Ids of policy rules consulted. */
  policyRefs: string[];
  /** Ordered reasoning notes that produced the outcome. */
  reasoning: string[];
  /** The evidence the decision was made on. */
  evidence: EvidencePacket;
  /** Who requested it. */
  caller: CallerIdentity;
  /** ISO-8601 decision timestamp. */
  decidedAt: string;
  /** Capability token — present ONLY when outcome === "allow". */
  token?: CapabilityToken;
  /** True when decided under degraded (DGR-unavailable) mode. Spec §2. */
  degraded?: boolean;
}

/** A short-lived capability token (Spec §1). Minting/verification is T0. */
export interface CapabilityToken {
  /** Unique token id (jti) — single-use, defends against replay. */
  jti: string;
  /** Bound action. */
  action: string;
  /** Hash binding the token to specific action params. */
  paramsHash: string;
  /** Token audience (caller agentId). */
  audience: string;
  /** Decision this token authorizes. */
  decisionId: string;
  /** Issued-at (epoch ms). */
  iat: number;
  /** Expiry (epoch ms). */
  exp: number;
  /** Detached signature over the canonical claims (base64). */
  sig: string;
}

/** Result of tool-side token verification (Spec §1.2). */
export interface VerificationResult {
  ok: boolean;
  /** Reason for rejection when ok === false. */
  reason?: string;
}
