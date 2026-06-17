# Spec: [FEATURE NAME]

> Spec ID: `NNNN-short-slug` · Status: Draft | In Review | Accepted · Owner: [FILL]
> Governed by [`.specify/memory/constitution.md`](../memory/constitution.md).

## Constitution gate (must pass before this spec is "Accepted")

- [ ] **Scope fence (P6):** This work is AI-agent execution control in financial-services
      risk, fraud, or payments — or a named paying customer demands it. Justify: [FILL]
- [ ] **Phase check (P9):** This spec does not require implementing the decision core during
      Phase 0. (If it does, it stays Draft until the phase gate opens.)
- [ ] **Tier (P8):** Consequence tier assigned: T0 | T1 | T2 | T3. Justify: [FILL]
- [ ] **Enforcement impact (P1/P5):** Does this touch a consequential authorization path?
      Yes/No. If Yes, T0 process applies.

## Problem

What problem are we solving, for whom, and why now? [FILL]

## Goals / Non-goals

- **Goals:** [FILL]
- **Non-goals:** [FILL — explicitly list scope kept out, per P6]

## Users & scenarios

[FILL: primary actors and the decisions they need governed.]

## Requirements

- **Functional:** [FILL]
- **Security / enforcement (if applicable):** must satisfy non-bypassable (P1), fail-closed
  (P2), evidence-based (P3), audit-ready (P4). [FILL specifics]
- **Latency / performance:** [FILL: numeric budgets — mark unset values as [FILL]]

## Evidence & audit

What inputs, policy references, reasoning, and provenance must each governed decision capture?
What durable record is produced? [FILL]

## Threat model (if enforcement-relevant)

[FILL: assets, adversaries, trust boundaries, and the attacks the bypass suite must cover.]

## Open questions

- [FILL]

## Acceptance criteria

- [ ] [FILL: testable outcomes; for enforcement work, name the bypass tests that must go green.]
