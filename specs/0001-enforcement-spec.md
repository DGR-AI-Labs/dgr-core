# Spec 0001 — Enforcement

> Spec ID: `0001-enforcement` · Status: **Draft (Phase 1 build)** · Owner: [FILL: founder/eng lead]
> Governed by [`../.specify/memory/constitution.md`](../.specify/memory/constitution.md).
>
> **Phase 1 notice:** This spec drives the Phase 1 build. The enforcement core is **T0 —
> enforcement-critical** and is being implemented as a **draft** pending the human gate
> (human review + cross-model review + ≥3 SAST + adversarial test). Nothing here is validated.
>
> **On the numbers below (READ THIS):** the former `[FILL: …]` founder decisions have been
> populated with **PROPOSED DEFAULTS** chosen by common-sense engineering judgment at the
> founder's instruction. They are **NOT yet ratified founder decisions** — each is tagged
> `(proposed default — confirm)`. They exist so the build can compile and the bypass suite can
> run; they are mirrored as named constants in `packages/core/src/constants.ts` (see
> `PROPOSED_DEFAULTS`) so a single, reviewable place changes them. **Confirm or override before
> the T0 review gate.**

## Constitution gate

- **Scope fence (P6):** AI-agent execution control for financial-services risk, fraud, and
  payments. In scope.
- **Phase:** Phase 1 — build + prove enforcement. Core is T0 (draft, human-led).
- **Tier (P8):** **T0 — enforcement-critical.** Implementation is human-led with mandatory
  human review + adversarial test + cross-model review + ≥3 SAST tools.
- **Enforcement impact:** This *is* the consequential authorization path.

---

## 1. Named enforcement mechanism

DGR authorizes a consequential action by minting a **short-lived capability token** that is
**verified tool-side**, backed by **admission control / network segmentation** so that the
effectful tool is unreachable except through the governed path.

1. **Short-lived capability token.** On an allowed decision, DGR issues a narrowly-scoped,
   single-action, short-TTL capability token bound to the specific action, parameters, caller
   identity, and decision record.
   - **TTL:** `30 s` *(proposed default — confirm)* — long enough to use immediately on the hot
     path, short enough to bound the replay window.
   - **Signing/verification scheme:** `Ed25519 asymmetric signatures` *(proposed default —
     confirm)* — DGR holds the private key; tools verify with the pinned public key. Asymmetric
     so a tool-side compromise cannot mint tokens.
   - **Replay prevention:** `single-use jti (token id) + one-time-use store` *(proposed default
     — confirm)*; store TTL = token TTL. In-memory for v0; a shared store (e.g., Redis) in
     multi-verifier deployments.
2. **Tool-side verification.** The effectful tool (or its sidecar/adapter) **verifies the
   token before acting** — checking signature, expiry, single-use, action/parameter binding,
   and audience. Verification failure ⇒ block (P2). The tool trusts the token, never the
   agent's say-so (P5).
3. **Admission control / network segmentation.** Effectful tools live in a segment that
   **only accepts calls carrying a valid DGR-issued token**, enforced below the application
   layer.
   - **Enforcement point:** `sidecar/adapter verification at the tool boundary` for Phase 1
     (developer-grade); `network policy / egress broker` is the Phase ≥2 hardening *(proposed
     default — confirm)*.

> This mechanism is the subject of Constitution P7 (the one un-rushable thing). The *proof*
> that it cannot be bypassed lives in [`tests/bypass/`](../tests/bypass/README.md).
>
> **Phase 1 honest scoping:** the shipped adapter is **agent-non-bypassable** (an agent cannot
> reach the tool without a valid token) but **operator-bypassable** (a privileged operator can
> disable the gate). This is developer-grade, not a security guarantee. See §5.

## 2. Fail-closed degraded-mode behavior

When DGR is missing, unreachable, returns invalid/ambiguous output, or cannot establish
sufficient justification, the outcome is **deny**. On DGR unavailability the system runs a
**defined** degraded mode (Constitution P2):

- **Blocks (always):** all consequential / effectful actions requiring authorization
  (payments, fund movement, secret access, bulk external messaging, etc.).
- **Queues (deferred, replayed when healthy):** effectful actions that are *safe to defer* and
  idempotent-on-replay where deferral causes no harm. *(proposed default — confirm)*:
  queue-eligible = explicitly tagged `deferrable` non-urgent actions (e.g., non-time-critical
  bulk notifications); **max queue age 24 h**, then drop + alert; replay at-most-once on
  recovery.
- **Executes (narrow allowlist only):** *(proposed default — confirm)* **empty by default** —
  only explicitly pre-authorized, non-consequential, read-only actions may be added to the
  allowlist, and none is enabled in v0. Nothing effectful executes while degraded.

There is no fail-open path. Token issuance failure, verification error, or an internally
throwing gate all resolve to **block** (covered by bypass case 5).

## 3. Latency budget

*(All proposed defaults — confirm. Mirrored in `PROPOSED_DEFAULTS` in code.)*

- **Added p99 latency (DGR authorization on the hot path):** `< 50 ms` *(proposed default — confirm)*
- **Added p50 latency:** `< 10 ms` *(proposed default — confirm)*
- **Token verification overhead (tool-side):** `< 5 ms` *(proposed default — confirm)*
- **Authorization timeout → fail-closed deadline:** `250 ms` *(proposed default — confirm)* —
  block (or, for `deferrable` actions, queue) if no decision arrives within this deadline.

## 4. Blast radius / containment (if DGR itself is compromised)

- **Token scope minimization:** single-action, single-use, ≤30 s TTL caps what any one leaked
  token authorizes — **max authority per token = exactly one already-decided action within its
  TTL** *(proposed default — confirm)*.
- **Key custody & rotation:** signing key held in `KMS/HSM`, isolated from the agent plane;
  **rotation every 90 days** *(proposed default — confirm)*; revocation via key-id denylist
  distributed to verifiers.
- **Tamper-evident audit (P4):** decision records written to an **append-only, hash-chained
  log** *(proposed default — confirm)* so compromise is detectable after the fact.
- **Segmentation holds:** verifiers hold an **independent trust anchor (pinned DGR public
  key)** *(proposed default — confirm)*; a forged-but-unverifiable token is rejected even if
  the DGR control plane is compromised.
- **Containment objective:** *(proposed default — confirm)* "No compromise of the DGR decision
  plane alone permits an unverifiable token to execute a consequential action; an operator with
  key-custody access is explicitly out of scope at this tier (§5)."

## 5. Threat model (explicit)

**Assets:** the integrity of the authorization decision; the effectful tools; the audit record.

**Trust boundaries:** agent plane (untrusted) → DGR (decision authority) → token →
tool-side verifier (trusts only valid tokens) → effectful tool (segmented).

**In-scope adversaries & attacks** (each has a corresponding bypass test asserting the secure
outcome):

| # | Attack | Required secure outcome |
|---|--------|-------------------------|
| 1 | Agent calls an effectful tool with **no token** | **Block** |
| 2 | **Expired or replayed** token | **Block** |
| 3 | Request with **missing justification** | **Block** |
| 4 | **Ambiguous / insufficient evidence** | **Block or escalate** |
| 5 | The **gate/hook itself throws** | **Fail closed (block)** |

**Explicitly out of scope at this tier:** a privileged **operator who disables the gate**
(insider/operator threat). The Phase 1 deliverable is **agent-non-bypassable, operator-
bypassable** (developer-grade). Operator-disable is addressed in **Phase ≥2** via operational
controls (network segmentation, key custody in HSM/KMS, tamper-evident audit, separation of
duties) *(proposed default — confirm)*.

**Assumptions** *(proposed defaults — confirm)*: tool-side verifier integrity; signing-key
custody in KMS/HSM; clock-skew tolerance **±5 s** between issuer and verifier.

## 6. Open questions (founder decisions to confirm)

- Confirm token TTL, signing scheme, replay store (§1).
- Confirm latency budgets (§3), especially the **250 ms fail-closed deadline** (drives code).
- Confirm degraded-mode queue/execute allowlists (§2).
- Confirm blast-radius bound, key custody, rotation (§4).
- Confirm the Phase ≥2 placement of the operator-disable threat (§5).

## 7. Acceptance criteria

- [ ] All bypass tests in [`tests/bypass/`](../tests/bypass/) go **green against real,
      reviewed enforcement** — never by weakening the tests.
- [ ] Every proposed-default value above is **confirmed or overridden** by a recorded founder
      decision.
- [ ] T0 process satisfied (human review + cross-model review + adversarial test + ≥3 SAST)
      before the core is trusted.
