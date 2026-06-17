// T1 — review required.
//
// Policy-bundle parsing and compilation. Validates structure and builds a fast
// action→rule lookup. Shapes the enforcement contract, so human review is
// required before trusted (Constitution P8).

import type { PolicyBundle, PolicyRule } from "../types";

export class PolicyBundleError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "PolicyBundleError";
  }
}

export interface CompiledPolicy {
  readonly version: string;
  readonly rules: readonly PolicyRule[];
  /** Return the rule matching an action (exact match preferred, else wildcard "*"). */
  match(action: string): PolicyRule | undefined;
}

const VALID_EFFECTS = new Set(["require-authorization", "allow", "deny"]);

/** Validate a policy bundle's structure. Throws PolicyBundleError on malformed input. */
export function validateBundle(bundle: PolicyBundle): void {
  if (!bundle || typeof bundle !== "object") {
    throw new PolicyBundleError("policy bundle must be an object");
  }
  if (typeof bundle.version !== "string" || bundle.version.length === 0) {
    throw new PolicyBundleError("policy bundle requires a non-empty string version");
  }
  if (!Array.isArray(bundle.rules)) {
    throw new PolicyBundleError("policy bundle requires a rules array");
  }
  const seen = new Set<string>();
  for (const rule of bundle.rules) {
    if (!rule || typeof rule.id !== "string" || rule.id.length === 0) {
      throw new PolicyBundleError("each rule requires a non-empty string id");
    }
    if (seen.has(rule.id)) throw new PolicyBundleError(`duplicate rule id: ${rule.id}`);
    seen.add(rule.id);
    if (typeof rule.action !== "string" || rule.action.length === 0) {
      throw new PolicyBundleError(`rule ${rule.id} requires a non-empty action`);
    }
    if (!VALID_EFFECTS.has(rule.effect)) {
      throw new PolicyBundleError(`rule ${rule.id} has invalid effect: ${String(rule.effect)}`);
    }
  }
}

/** Compile a validated bundle into a fast-lookup CompiledPolicy. */
export function compilePolicy(bundle: PolicyBundle): CompiledPolicy {
  validateBundle(bundle);
  const exact = new Map<string, PolicyRule>();
  let wildcard: PolicyRule | undefined;
  for (const rule of bundle.rules) {
    if (rule.action === "*") wildcard = rule;
    else exact.set(rule.action, rule);
  }
  return {
    version: bundle.version,
    rules: bundle.rules,
    match(action: string): PolicyRule | undefined {
      return exact.get(action) ?? wildcard;
    },
  };
}

/** Parse a JSON string into a CompiledPolicy. Throws PolicyBundleError on bad input. */
export function parsePolicy(json: string): CompiledPolicy {
  let data: unknown;
  try {
    data = JSON.parse(json);
  } catch {
    throw new PolicyBundleError("policy bundle is not valid JSON");
  }
  return compilePolicy(data as PolicyBundle);
}
