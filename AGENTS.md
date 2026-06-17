# AGENTS.md

Tool-agnostic guidance for AI agents working in this repo (companion to
[`CLAUDE.md`](CLAUDE.md)). **Short by design.**

## Source of truth

[`.specify/memory/constitution.md`](.specify/memory/constitution.md) is the **binding source of
truth.** It wins over anything here. Reference it; do not duplicate it.

## Current-phase guardrail

> **Phase 1 — build + prove enforcement.** The enforcement core (token mint/verify, fail-closed/
> degraded-mode, decision point) is **T0 — enforcement-critical, human-led, DRAFT and
> unvalidated** until the human gate (review + cross-model review + ≥3 SAST + adversarial test).
> Hold the **FS-wedge scope fence** (financial-services risk, fraud, payments; reject scope
> creep — generation speed is not a reason to broaden). **The enforcement proof is the one
> un-rushable thing.** Honest scoping: developer-grade — agent-non-bypassable, operator-bypassable.

The bypass suite ([`tests/bypass/`](tests/bypass/README.md)) is real and runnable; it passes only
if the T0 draft is correct, and green still does not mean enforcement is proven until the T0 gate
passes. Never make it green by weakening tests or the core.

## Consequence-tier rules (Constitution P8)

- **T0 (enforcement-critical):** human-led; needs **human review + cross-model review + ≥3 SAST
  tools + adversarial tests.**
- **T1:** human review + SAST. **T2/T3 only** get high autonomy.
- When unsure, assume the higher tier.

## Workflow

Branch → small commits → PR; no direct pushes to `main`; a human merges.
