# @dgr/openclaw

Thin OpenClaw `before_tool_call` adapter for [`@dgr/core`](../core).

> **Experimental. Not production-ready. No guarantees.** Draft for review.
>
> **Honest threat model:** this enforces against the **agent** (an agent cannot reach a governed
> tool without a DGR "allow" + capability token) but is **operator-bypassable** (a privileged
> operator who controls the runtime can skip the hook). **Developer-grade**, not a security
> guarantee — not "non-bypassable", "secure", "production", or "compliant".

## Behavior

Fail-closed end-to-end. The guard returns one of:

- `approve` — DGR allowed the call (with a minted capability token attached);
- `narrow` — approved, with narrowed parameters (when a `narrow` function is supplied);
- `block` — anything else: block / escalate / request-evidence, a **decision timeout**
  (approve-with-deny-on-timeout), or a thrown hook error.

## Sketch

```ts
import { createOpenClawGuard } from "@dgr/openclaw";
import { createDecisionEngine, createSigner, generateSigningKeyPair, V0_POLICY } from "@dgr/core";

const { privateKey } = generateSigningKeyPair();
const guard = createOpenClawGuard({
  engine: createDecisionEngine({ policy: V0_POLICY, signer: createSigner(privateKey) }),
});

const result = await guard.beforeToolCall(
  { toolName: "pay_invoice", args: { invoiceId: "INV-1" }, metadata: { dgr: evidencePacket } },
  { agentId: "agent-1" },
);
// result.decision is "approve" | "narrow" | "block"
```

> The OpenClaw hook payload/return shapes here are a **documented assumption** pending
> confirmation against OpenClaw's actual `before_tool_call` API.
