# Bypass suite

This suite is the executable form of Constitution Principle 7: **the enforcement proof is the
one un-rushable thing.** Each test models an attack against the governed boundary and **asserts
the secure outcome** (blocked, or escalated where specified), exercising the real `@dgr/core`
decision engine and tool-side token verification.

## Status (Phase 1)

The tests are now **real and runnable** (Vitest, against `@dgr/core`). They pass **only if the
T0 enforcement core is correct.** The T0 core is currently a **DRAFT** (see the `T0 —
enforcement-critical … Not validated` headers in `packages/core/src/token/`,
`packages/core/src/decision/`).

> **Green here means the suite passes — it does NOT by itself mean enforcement is *proven*.**
> Proof requires the T0 human gate: human review + cross-model review + ≥3 SAST + adversarial
> testing on the T0 files. Only after that gate should `bypass-suite` be made a required check.
> Never make the suite green by weakening a test or the core.

## The attack set

Each test asserts the secure outcome.

| # | File | Attack | Required secure outcome |
|---|------|--------|-------------------------|
| 1 | [`01-no-token.test.ts`](01-no-token.test.ts) | Direct effectful-tool call with **no token** | **Blocked** |
| 2 | [`02-expired-replayed-token.test.ts`](02-expired-replayed-token.test.ts) | **Expired or replayed** token | **Blocked** |
| 3 | [`03-missing-justification.test.ts`](03-missing-justification.test.ts) | **Missing justification** | **Blocked** |
| 4 | [`04-ambiguous-evidence.test.ts`](04-ambiguous-evidence.test.ts) | **Ambiguous / insufficient evidence** | **Blocked or escalated** |
| 5 | [`05-gate-throws.test.ts`](05-gate-throws.test.ts) | The **gate/hook itself throws** | **Fail closed (block)** |

## Out of scope at this tier

**"Operator disables the gate"** (a privileged insider/operator turning enforcement off) is
**explicitly OUT of scope** for this suite. The Phase 1 deliverable is **agent-non-bypassable,
operator-bypassable** (developer-grade). Operator-disable is Phase ≥2 work. See
[`specs/0001-enforcement-spec.md`](../../specs/0001-enforcement-spec.md) §5.

## Running

```sh
pnpm install
pnpm test:bypass        # vitest run tests/bypass
```

Tests run against TypeScript source via the workspace alias — no build step required.
