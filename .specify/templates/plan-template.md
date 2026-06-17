# Implementation Plan: [FEATURE NAME]

> For spec: `NNNN-short-slug` · Status: Draft | In Review | Accepted
> Governed by [`.specify/memory/constitution.md`](../memory/constitution.md).

## Constitution re-check (before planning work)

- [ ] Scope fence (P6) still holds.
- [ ] Phase gate (P9): nothing here implements the decision core during Phase 0.
- [ ] Tier (P8) confirmed; if T0/T1, the plan is **human-led** and lists the required
      human review + adversarial test + cross-model review + ≥3 SAST tools.

## Approach

[FILL: the chosen design, at a high level. Link to an ADR if one exists.]

## Architecture & boundaries

[FILL: components, trust boundaries, where enforcement sits at runtime (P5).]

## Enforcement mechanism (if applicable)

[FILL: how non-bypassability (P1) and fail-closed behavior (P2) are achieved and proven.]

## Risks & mitigations

| Risk | Tier | Mitigation |
|------|------|------------|
| [FILL] | [FILL] | [FILL] |

## Verification strategy

- Bypass suite coverage required: [FILL list]
- Adversarial tests (T0/T1): [FILL]
- SAST tools (≥3 for T0): [FILL]
- Cross-model review (T0): [FILL]

## Rollout & degraded mode

[FILL: how this ships; what the defined fail-closed degraded mode is for this feature (P2).]

## Out of scope

[FILL]
