# CLAUDE.md

Guidance for AI coding agents (and humans) working in this repo. **Short by design** — the
canonical rules live elsewhere; this file only points at them and restates the hard guardrails.

## Source of truth

[`.specify/memory/constitution.md`](.specify/memory/constitution.md) is the **binding source of
truth**. If anything here and the constitution disagree, the constitution wins. Do not duplicate
or paraphrase its principles into code or docs — link to them.

## Current-phase guardrail (read every time)

> **Phase 0 — spine only.** Do **NOT** implement the decision core (no enforcement logic, no
> token issuance/verification, no admission control, no product logic). Hold the **FS-wedge
> scope fence** (AI-agent execution control in financial-services risk, fraud, payments — reject
> scope creep; generation speed is not a reason to broaden). **The enforcement proof is the one
> un-rushable thing.**

This repo currently holds **specs, governance, and red (failing) scaffolding only**. The failing
bypass suite and red CI are intentional and correct (fail-closed by absence). Do not "fix" them
by weakening tests or the [`src/gate.mjs`](src/gate.mjs) stub.

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
