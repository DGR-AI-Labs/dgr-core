# @dgr/core

Framework-agnostic decision engine for DGR (Decision-Grade Reasoning).

> **Experimental. Not production-ready. No guarantees.** The enforcement-critical internals
> (`token/`, `decision/`) are **T0 — DRAFT, unvalidated** pending a human review gate (human
> review + cross-model review + ≥3 SAST + adversarial test). **Developer-grade**: it enforces
> against the agent, not against a privileged operator. Not "non-bypassable", "secure",
> "production", or "compliant".

## What's here

- **Schemas** (T1): `DecisionObject`, `EvidencePacket`, `PolicyBundle`, decision result
  (`allow` / `block` / `escalate` / `request-evidence`).
- **Policy** (T1): `compilePolicy` / `parsePolicy`; `V0_POLICY` spans payment **and**
  non-payment actions.
- **Decision point** (T0 draft): `createDecisionEngine(...).decide(...)` — always fail-closed.
- **Capability token** (T0 draft): `createSigner` / `verifyToken` — Ed25519, single-use,
  short-TTL, tool-side verification.

## Sketch

```ts
import {
  createDecisionEngine, createSigner, generateSigningKeyPair,
  verifyToken, createInMemoryReplayStore, V0_POLICY,
} from "@dgr/core";

const { publicKey, privateKey } = generateSigningKeyPair();
const engine = createDecisionEngine({ policy: V0_POLICY, signer: createSigner(privateKey) });

const decision = await engine.decide(
  { action: "pay_invoice", params: { invoiceId: "INV-1" }, caller: { agentId: "agent-1" } },
  { justification: "approved", evidence: [{ kind: "policy", ref: "PAY-1", confidence: 0.9 }],
    provenance: { requestedBy: "agent-1", via: "app", at: new Date().toISOString() } },
);

if (decision.outcome === "allow") {
  // tool-side: verify the minted capability token before acting
  const ok = verifyToken(decision.token, {
    publicKey, request: { action: "pay_invoice", params: { invoiceId: "INV-1" }, caller: { agentId: "agent-1" } },
    replayStore: createInMemoryReplayStore(),
  });
}
```

The numeric defaults (TTL, timeout, clock skew, …) live in `PROPOSED_DEFAULTS` and are
**proposed values pending founder confirmation** — see `specs/0001-enforcement-spec.md`.
