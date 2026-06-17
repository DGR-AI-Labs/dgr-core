# Bypass suite

This suite is the executable form of Constitution Principle 7: **the enforcement proof is the
one un-rushable thing.** Each test models an attack against the governed boundary and **asserts
the secure outcome** (the action is blocked, or escalated where specified).

## Read this first: red is correct here

In **Phase 0** the decision core is not implemented (Constitution P9), so
[`src/gate.mjs`](../../src/gate.mjs) is a deliberate failing stub. Every test below therefore
**FAILS**. That is intentional and correct — it is fail-closed *by absence*: there is nothing
implemented to prove these attacks are stopped, so the proof is honestly absent.

> **Green = enforcement proven.** A green bypass suite means real, human-led, reviewed
> enforcement makes every one of these attacks end in the secure outcome. Green is **never** to
> be reached by weakening these tests or the stub — only by building (and reviewing) a real gate.

## The attack set

Each test calls `decide(request)` and asserts the secure outcome.

| # | File | Attack | Required secure outcome |
|---|------|--------|-------------------------|
| 1 | [`01-no-token.test.mjs`](01-no-token.test.mjs) | Direct effectful-tool call with **no token** | **Blocked** |
| 2 | [`02-expired-replayed-token.test.mjs`](02-expired-replayed-token.test.mjs) | **Expired or replayed** token | **Blocked** |
| 3 | [`03-missing-justification.test.mjs`](03-missing-justification.test.mjs) | **Missing justification** | **Blocked** |
| 4 | [`04-ambiguous-evidence.test.mjs`](04-ambiguous-evidence.test.mjs) | **Ambiguous / insufficient evidence** | **Blocked or escalated** |
| 5 | [`05-gate-throws.test.mjs`](05-gate-throws.test.mjs) | The **gate/hook itself throws** | **Fail closed (block)** |

## Out of scope at this tier

**"Operator disables the gate"** (a privileged insider/operator turning enforcement off) is
**explicitly OUT of scope** for this suite. It is a real threat handled at a different tier
(operational controls, key custody, segmentation), not something these runtime-bypass tests can
or should assert. See [`specs/0001-enforcement-spec.md`](../../specs/0001-enforcement-spec.md) §5.

## Running

```sh
node --test tests/bypass        # expected: FAIL in Phase 0 (red CI by design)
```

No third-party dependencies — these use Node's built-in `node:test` and `node:assert`.
