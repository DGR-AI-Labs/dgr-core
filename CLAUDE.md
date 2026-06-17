# CLAUDE.md

Guidance for AI coding agents (and humans) working in this repo. **Short by design** — the
canonical rules live elsewhere; this file only points at them and restates the hard guardrails.

## Source of truth

[`.specify/memory/constitution.md`](.specify/memory/constitution.md) is the **binding source of
truth**. If anything here and the constitution disagree, the constitution wins. Do not duplicate
or paraphrase its principles into code or docs — link to them.

## Current-phase guardrail (read every time)

> **Phase 1 — build + prove enforcement.** The enforcement core is **T0 — enforcement-critical
> and human-led**: capability-token mint/verify, fail-closed/degraded-mode, and the decision
> point are **DRAFT, unvalidated** until the human gate (review + cross-model review + ≥3 SAST +
> adversarial test). Hold the **FS-wedge scope fence** (AI-agent execution control in financial-
> services risk, fraud, payments — reject scope creep; generation speed is not a reason to
> broaden). **The enforcement proof is the one un-rushable thing.** Honest scoping: developer-
> grade — agent-non-bypassable, operator-bypassable.

The bypass suite is **real and runnable** ([`tests/bypass/`](tests/bypass/README.md)) and passes
only if the T0 draft is correct — a green run still does **not** mean enforcement is proven until
the T0 gate passes. Never make it green by weakening a test or the core, and do not mark
`bypass-suite` a required check until a human verifies it passes legitimately.

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
