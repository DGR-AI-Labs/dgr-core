# Spec 0001 — Enforcement

> Spec ID: `0001-enforcement` · Status: **Draft (Phase 0)** · Owner: [FILL: founder/eng lead]
> Governed by [`../.specify/memory/constitution.md`](../.specify/memory/constitution.md).
>
> **Phase 0 notice:** This spec describes the *target* enforcement design so the bypass suite
> can assert the secure outcomes. It is **not** an implementation directive. No decision-core
> code is to be written while the repo is in Phase 0 (Constitution P9). Every unset number is
> marked `[FILL]` and must be a founder decision, not invented here.

## Constitution gate

- **Scope fence (P6):** AI-agent execution control for financial-services risk, fraud, and
  payments. In scope.
- **Phase (P9):** Phase 0 — specification only; do not implement.
- **Tier (P8):** **T0 — enforcement-critical.** Any implementation is human-led with mandatory
  human review + adversarial test + cross-model review + ≥3 SAST tools.
- **Enforcement impact:** This *is* the consequential authorization path.

---

## 1. Named enforcement mechanism

DGR authorizes a consequential action by minting a **short-lived capability token** that is
**verified tool-side**, backed by **admission control / network segmentation** so that the
effectful tool is unreachable except through the governed path.

1. **Short-lived capability token.** On an allowed decision, DGR issues a narrowly-scoped,
   single-action, short-TTL capability token bound to the specific action, parameters, caller
   identity, and decision record. `[FILL: token TTL]` · `[FILL: signing/verification scheme,
   e.g., asymmetric signature / MAC]` · `[FILL: replay-prevention scheme, e.g., nonce/jti +
   one-time-use store]`.
2. **Tool-side verification.** The effectful tool (or its sidecar/adapter) **verifies the
   token before acting** — checking signature, expiry, single-use, action/parameter binding,
   and audience. Verification failure ⇒ block (P2). The tool trusts the token, never the
   agent's say-so (P5).
3. **Admission control / network segmentation.** Effectful tools live in a segment that
   **only accepts calls carrying a valid DGR-issued token**, enforced below the application
   layer (network policy / admission controller / broker) so the control cannot be skipped by
   calling the tool "directly." `[FILL: enforcement point — service mesh / sidecar / egress
   broker / network policy]`.

> This mechanism is the subject of Constitution P7 (the one un-rushable thing). The *proof*
> that it cannot be bypassed lives in [`tests/bypass/`](../tests/bypass/README.md).

## 2. Fail-closed degraded-mode behavior

When DGR is missing, unreachable, returns invalid/ambiguous output, or cannot establish
sufficient justification, the outcome is **deny**. On DGR unavailability the system runs a
**defined** degraded mode (Constitution P2):

- **Blocks (always):** all consequential / effectful actions requiring authorization.
- **Queues (deferred, replayed when healthy):** safe-to-defer actions where deferral causes
  no harm. `[FILL: queue-eligible action classes + max queue age + replay policy]`.
- **Executes (narrow allowlist only):** non-consequential / read-only / pre-authorized
  actions. `[FILL: the explicit executes-while-degraded allowlist]`.

There is no fail-open path. Token issuance failure, verification error, or an internally
throwing gate all resolve to **block** (covered by bypass case 5).

## 3. Latency budget

- **Added p99 latency (DGR authorization on the hot path):** `[FILL: e.g., added p99 < 50 ms]`
- **Added p50 latency:** `[FILL]`
- **Token verification overhead (tool-side):** `[FILL]`
- **Authorization timeout → fail-closed deadline:** `[FILL: e.g., block if no decision within N ms]`

All four are **founder decisions**. Do not infer them from generation convenience.

## 4. Blast radius / containment (if DGR itself is compromised)

Assume DGR is breached and reason about limiting damage:

- **Token scope minimization:** single-action, single-use, short-TTL tokens cap what any one
  leaked token authorizes. `[FILL: max authority per token]`.
- **Key custody & rotation:** signing keys isolated from the agent plane; compromise is
  contained and revocable. `[FILL: key custody (HSM/KMS) + rotation interval + revocation path]`.
- **Tamper-evident audit (P4):** records are durable and tamper-evident so a compromise is
  *detectable* after the fact. `[FILL: integrity mechanism]`.
- **Segmentation holds:** even with DGR control-plane compromise, tools still independently
  verify tokens; a forged-but-unverifiable token is rejected. `[FILL: independent trust anchor]`.
- **Containment objective:** `[FILL: stated blast-radius bound, e.g., "no compromise of DGR
  alone permits an unverifiable token to execute a consequential action"]`.

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
(insider/operator threat). This is acknowledged and deferred — see
[`tests/bypass/README.md`](../tests/bypass/README.md). `[FILL: when/where operator-disable is
addressed]`.

**Assumptions:** `[FILL: e.g., tool-side verifier integrity, key custody, clock-skew bounds]`.

## 6. Open questions (founder decisions)

- `[FILL: token TTL, signing scheme, replay store]`
- `[FILL: latency budgets (§3)]`
- `[FILL: degraded-mode queue/execute allowlists (§2)]`
- `[FILL: blast-radius bound and key custody (§4)]`
- `[FILL: where operator-disable threat is handled (§5)]`

## 7. Acceptance criteria

- [ ] All bypass tests in [`tests/bypass/`](../tests/bypass/) go **green against real,
      reviewed enforcement** — never by weakening the tests.
- [ ] Every `[FILL]` above is replaced by a recorded founder decision.
- [ ] T0 process satisfied (human review + adversarial test + cross-model review + ≥3 SAST).
