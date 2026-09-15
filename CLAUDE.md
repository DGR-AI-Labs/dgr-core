# CLAUDE.md

Guidance for AI coding agents (and humans) working in this repo. **Short by design** — the
canonical rules live elsewhere; this file only points at them and restates the hard guardrails.

## Source of truth

[`.specify/memory/constitution.md`](.specify/memory/constitution.md) is the **binding source of
truth**. If anything here and the constitution disagree, the constitution wins. Do not duplicate
or paraphrase its principles into code or docs — link to them.

## Current-phase guardrail (read every time)

> **Phase 1 — enforcement proof.** Agent-authored T0 is prohibited by default. The only
> future-facing exception is the exact manifest-scoped RUNTIME-002 work pinned by
> [`specs/ADR-14-AMENDMENT-A-reference-contract.md`](specs/ADR-14-AMENDMENT-A-reference-contract.md)
> and tracked in [`T0-AUTHORS.md`](T0-AUTHORS.md); it is inoperative until every listed prerequisite
> is complete. Agents may not author, complete, refactor, or patch T0 outside an operative exception.
> Hold the FS-wedge scope fence and never weaken a bypass test to obtain green.

Tests, harnesses, fixtures, specs, and tooling may support the proof within their assigned
tier. A green attack case must come from real reviewed enforcement, not a bypass or reclassified
expectation.

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
