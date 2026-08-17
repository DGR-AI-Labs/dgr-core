# CLAUDE.md

Guidance for AI coding agents (and humans) working in this repo. **Short by design** — the
canonical rules live elsewhere; this file only points at them and restates the hard guardrails.

## Source of truth

[`.specify/memory/constitution.md`](.specify/memory/constitution.md) is the **binding source of
truth**. If anything here and the constitution disagree, the constitution wins. Do not duplicate
or paraphrase its principles into code or docs — link to them.

## Current-phase guardrail (read every time)

> **Phase 1 — enforcement proof.** Founder-authored T0 enforcement is permitted only in the
> five units named by [`T0-AUTHORS.md`](T0-AUTHORS.md). Agents may review those units but may
> not author, complete, refactor, or patch their enforcement logic. Hold the FS-wedge scope
> fence and never weaken a bypass test to obtain green.

Tests, harnesses, fixtures, specs, and tooling may support the proof within their assigned
tier. A green attack case must come from real founder-authored enforcement, not a bypass or
reclassified expectation.

## Consequence-tier rules (Constitution P8)

- **T0 (enforcement-critical):** human-led; requires **human review + adversarial test +
  cross-model review + ≥3 SAST tools.** No exceptions.
- **T1:** human review required; SAST in CI.
- **T2/T3 only** may run with high autonomy.
- When unsure of a tier, treat the work as the higher (more critical) tier.

## Workflow

Branch → small commits → PR. No direct pushes to `main`. A human merges. See the constitution's
"Governance of this repo" section.

See also [`AGENTS.md`](AGENTS.md) (same guidance, tool-agnostic).
