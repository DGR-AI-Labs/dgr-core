# DGR Constitution

> Canonical source of truth for the `dgr-core` repository.
> Internal project name: **DGR — Decision-Grade Reasoning**: a runtime decision-governance
> layer that sits between AI agents and effectful tools and authorizes consequential actions
> before they execute.
>
> This document is **binding and non-negotiable**. Where a principle and any other file in this
> repo disagree, this constitution wins. Agent-context files (`CLAUDE.md`, `AGENTS.md`) and the
> README reference this document; they do not override it.

- **Ratified:** 2026-06-16
- **Version:** 1.0.0 (semver: MAJOR = principle removed/redefined, MINOR = principle added,
  PATCH = wording/clarification)
- **Current phase:** Phase 0 — spine only (see Principle 9)

---

## Principle 1 — Non-bypassable

Agents **cannot** reach effectful tools without an explicit DGR authorization for the specific
action. There is no side door, no "fast path," no debug flag, and no privileged caller that is
exempt. If a path can reach an effectful tool, that path is governed — or it does not ship.

A "non-bypassable" claim that breaks in a consequential path is a project-ending failure
(see Principle 7). The absence of a bypass must be *demonstrated by tests* (see the bypass
suite), not asserted in prose.

## Principle 2 — Fail-closed

Missing, invalid, ambiguous, or insufficiently justified requests **block execution**. The
default outcome of any uncertainty is *deny*, never *allow*.

When DGR itself is **unavailable**, the system enters a **defined degraded mode** with an
explicit, written partition of behavior:

- **Blocks:** every consequential / effectful action that requires authorization.
- **Queues:** actions that are safe to defer and replay once DGR is healthy, where deferral
  does not itself cause harm. `[FILL: which action classes are queue-eligible]`.
- **Executes:** only a narrowly enumerated allowlist of non-consequential, read-only, or
  pre-authorized actions. `[FILL: the explicit executes-while-degraded allowlist]`.

"Fail-open while we figure it out" is prohibited. The degraded mode is part of the contract,
not an incident-time improvisation.

## Principle 3 — Evidence-based

A governed decision requires, at minimum:

- **Inputs** — the action being requested and its parameters.
- **Policy references** — the specific rules/policies evaluated.
- **Reasoning** — why the decision follows from the inputs and policy.
- **Provenance** — who/what asked, under what identity/capability, and the chain of custody
  for the supporting evidence.

A decision that cannot cite these is, by definition, *insufficiently justified* and is blocked
under Principle 2.

## Principle 4 — Audit-ready

Every governed decision produces a **durable, tamper-evident record** sufficient to reconstruct
*what was decided, on what evidence, under which policy, and why* — after the fact, by someone
who was not present. If it was not recorded, it did not happen in a way we can stand behind.

## Principle 5 — Runtime-enforced and decision-centered

DGR governs **the decision, not the prompt**. Enforcement happens at runtime, at the boundary
where an action would take effect — not by inspecting or massaging model inputs, and not by
trusting the agent to police itself. Prompt-level "guardrails" are not enforcement and never
count as satisfying Principle 1.

## Principle 6 — Scope fence (HARD)

The wedge is **AI-agent execution control in financial-services risk, fraud, and payments.**

- Reject scope creep toward generic "AI governance," generic "agent safety," or any adjacent
  market **unless a paying customer demands it**.
- **Generation speed is not a reason to broaden.** That a thing is easy or fast to build is
  irrelevant to whether it is in scope.
- New surface area requires an explicit, written justification tied to the FS wedge (or a
  named paying customer) before any spec, plan, or code is created for it.

## Principle 7 — The enforcement proof is the one un-rushable thing

The proof that DGR is non-bypassable is the single artifact that may never be rushed,
hand-waved, or marked "good enough to ship." Everything else can iterate. This cannot.

A bypass that reaches a consequential action ends the project's credibility and, per this
constitution, the project. Treat the enforcement proof accordingly.

## Principle 8 — Consequence-tiered build (T0–T3)

Work is classified by blast radius. The tier sets the **minimum** process — higher autonomy is
never permitted to lower these bars.

| Tier | Scope | Required process |
|------|-------|------------------|
| **T0** | Enforcement-critical code (anything on a consequential authorization path) | **Human-led.** Mandatory human review + adversarial test + cross-model review + **≥3 SAST tools**. No exceptions. |
| **T1** | Code adjacent to enforcement (evidence handling, audit records, token issuance/verification) | Human review required; adversarial test where applicable; SAST in CI. |
| **T2** | Supporting code with limited blast radius (adapters, non-critical services) | May run with higher autonomy; standard review + CI. |
| **T3** | Non-consequential (docs, tooling, examples, scaffolding) | High autonomy permitted. |

Only **T2/T3** run with high autonomy. **T0/T1 are human-led.** When in doubt about a tier,
treat the work as the *higher* (more critical) tier.

## Principle 9 — Current phase: Phase 0 (spine only)

This repository is currently in **Phase 0**: signal-testing demand. The decision core is
**explicitly gated and forbidden** until a later phase.

- **Do NOT implement** the decision core, enforcement logic, token issuance/verification,
  admission control, or any product logic.
- This repo currently holds **specs, governance, and red (failing) scaffolding only.**
- The failing bypass suite and red CI are **intentional and correct** (fail-closed by
  absence): there is nothing implemented to make them pass, and that is the truthful state.
- Green CI for the bypass suite is a **future milestone** that means *real, reviewed
  enforcement exists* — it is never to be achieved by weakening the tests.

---

## Amendment process

Changes to this constitution require a pull request that (a) states the principle affected,
(b) bumps the version per the semver rule above, and (c) is merged by a human. No agent may
amend this document and merge its own change.

## Governance of this repo (Phase 0)

- All work lands via pull request on a branch; no direct pushes to `main`.
- The bootstrap/scaffold PR is the one allowed exception to "CI must be green to merge,"
  because CI is intentionally red until enforcement exists. A human merges it.
- The bypass suite is **not** wired as a required status check yet. Flipping it to required is
  a deliberate, later, human step taken only once a real gate exists.
