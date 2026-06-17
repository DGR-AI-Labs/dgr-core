// PROPOSED DEFAULTS — NOT ratified founder decisions.
//
// These mirror the values written into specs/0001-enforcement-spec.md, where
// each is tagged "(proposed default — confirm)". They were chosen by common-
// sense engineering judgment at the founder's explicit instruction to fill the
// spec's former [FILL] placeholders. They exist so the build compiles and the
// bypass suite can run with named, single-source-of-truth constants rather than
// magic numbers.
//
// TODO(founder): confirm or override every value below before the T0 review
// gate. Until confirmed, treat them as provisional.

export const PROPOSED_DEFAULTS = {
  /** Capability-token time-to-live (ms). Spec §1. (proposed default — confirm) */
  TOKEN_TTL_MS: 30_000,
  /** Fail-closed deadline: block/queue if no decision within this window (ms). Spec §3. (proposed default — confirm) */
  AUTH_TIMEOUT_MS: 250,
  /** Allowed clock skew between issuer and verifier (ms). Spec §5. (proposed default — confirm) */
  CLOCK_SKEW_TOLERANCE_MS: 5_000,
  /** Hot-path latency budget (ms). Spec §3. (proposed default — confirm) */
  LATENCY_BUDGET: {
    addedP99Ms: 50,
    addedP50Ms: 10,
    verifyOverheadMs: 5,
  },
  /** Signing-key rotation interval (days). Spec §4. (proposed default — confirm) */
  KEY_ROTATION_DAYS: 90,
  /** Token signing algorithm. Spec §1. (proposed default — confirm) */
  SIGNING_ALGORITHM: "ed25519",
} as const;

export type ProposedDefaults = typeof PROPOSED_DEFAULTS;
