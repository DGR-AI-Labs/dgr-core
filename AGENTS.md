# AGENTS.md

Tool-agnostic guidance for AI agents working in this repo (companion to
[`CLAUDE.md`](CLAUDE.md)). **Short by design.**

## Source of truth

[`.specify/memory/constitution.md`](.specify/memory/constitution.md) is the **binding source of
truth.** It wins over anything here. Reference it; do not duplicate it.

## Current-phase guardrail

> **Phase 1 — enforcement proof.** Founder-authored T0 enforcement is permitted only in the
> five units named by [`T0-AUTHORS.md`](T0-AUTHORS.md). Agents may review those units but may
> not author, complete, refactor, or patch their enforcement logic. Hold the FS-wedge scope
> fence and never weaken a bypass test to obtain green.

Tests, harnesses, fixtures, specs, and tooling may support the proof within their assigned
tier. A green attack case must come from real founder-authored enforcement, not a bypass or
reclassified expectation.

## Consequence-tier rules (Constitution P8)

- **T0 (enforcement-critical):** human-led; needs **human review + cross-model review + ≥3 SAST
  tools + adversarial tests.**
- **T1:** human review + SAST. **T2/T3 only** get high autonomy.
- When unsure, assume the higher tier.

## Workflow

Branch → small commits → PR; no direct pushes to `main`; a human merges.
