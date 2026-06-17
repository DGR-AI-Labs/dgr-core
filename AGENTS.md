# AGENTS.md

Tool-agnostic guidance for AI agents working in this repo (companion to
[`CLAUDE.md`](CLAUDE.md)). **Short by design.**

## Source of truth

[`.specify/memory/constitution.md`](.specify/memory/constitution.md) is the **binding source of
truth.** It wins over anything here. Reference it; do not duplicate it.

## Current-phase guardrail

> **Phase 0 — spine only.** Do **NOT** implement the decision core. Hold the **FS-wedge scope
> fence** (financial-services risk, fraud, payments; reject scope creep — generation speed is
> not a reason to broaden). **The enforcement proof is the one un-rushable thing.**

The repo holds **specs, governance, and red (failing) scaffolding only.** Red CI and the failing
bypass suite are intentional (fail-closed by absence) — never make them green by weakening the
tests or the [`src/gate.mjs`](src/gate.mjs) stub.

## Consequence-tier rules (Constitution P8)

- **T0 (enforcement-critical):** human-led; needs **human review + cross-model review + ≥3 SAST
  tools + adversarial tests.**
- **T1:** human review + SAST. **T2/T3 only** get high autonomy.
- When unsure, assume the higher tier.

## Workflow

Branch → small commits → PR; no direct pushes to `main`; a human merges.
