# Hermes core authority reference

**Status: pointer to an adopted private authority; Hermes implementation remains gated.**

This code-governance reference implements the separately gated Hermes exception in
[Constitution v5.0.0, Principle 9](../.specify/memory/constitution.md#principle-9--current-phase-phase-1-enforcement-proof).
It exposes only source identities and authoring boundaries. Private planning, manifests,
registries and review material remain private. Existing runtime and offline issuer exceptions
remain independent.

## Exact private identities

```text
Repository: dgr-internal (canonical CodeCommit, us-west-2)
ID: HERMES-CORE-AUTHORITY-001
Source status: active, standalone authority (not an ADR)
Canonical source commit: aa4258111b8ea7f66b45e19f6239010e0c9dcc53
Authority path: qa/hermes-planning/HERMES-CORE-AUTHORITY-001.md
Authority Git blob: 3d9bdfeb797e4d4e5e1c9bc4bcba8194b5e14aea
Authority SHA-256: 25b892390354b66a640fdf6de40eb522fa88725ba81f07b6d14daa0e63be07aa
Adoption-event path: qa/hermes-planning/stage1-authority-adoption/ADOPTION-EVENT.json
Adoption-event Git blob: efdb561176e813ec72e3da40b51bfa80798d0228
Adoption-event SHA-256: f0f3d40201c716ea3e6a1faba1bbc8add0f5f811395f1bcfcb7036f6807335b4
```

The proposal was merged at `8262978b0e9916df47567ee54d294e01b52627c4`; the activation
revision was merged at `9ef2fa70a9a89a207e6c2e9dea0c889a80b12345`; the canonical source above
includes the subsequent founder-authorized exception reconciliation. Resolve the authority,
event record and actual adoption/merge evidence against canonical private history. A header,
filename or hash alone does not establish adoption or completion of downstream gates.

The private adoption/activation history contains explicit founder-scoped review exceptions.
Those exceptions record accepted limitations, not completed independent reviews. They do not
waive review of this public pointer, implementation, the final manifest or backlog activation,
and do not modify the constitution. The historical draft-source status in the constitutional
amendment log describes the earlier adoption stage; this exact pointer supplies the updated
private source identity.

## Remaining preconditions and boundaries

1. Resolve the adopted private authority and event evidence at the exact identities above,
   including the limits of the recorded exceptions and the bounded payments/risk scope.
2. Independently review and human-merge this updated public pointer. Its branch presence alone
   does not complete public adoption. The existing constitutional exception remains unchanged.
3. Independently review and freeze the Stage1 reason/action/attack registry, contracts and
   sanitized independent expected results. A code author cannot invent policy meanings or
   derive acceptance expectations from the implementation being tested.
4. Obtain founder adoption of the final exact implementation manifest: actual repository/base,
   files/symbols, behavior, provenance, dependencies/features, tests, analyzers, reviewers and
   hard stops. Pin the actual implementation base after public-governance merge, resolve all
   applicable intake obligations and explicitly defer excluded later-stage work. Complete
   analyzer suitability and applicable dependency/material/fresh advisory dispositions.
5. Human-review and human-merge canonical backlog activation of only the approved Stage1 scope,
   citing actual authority and manifest identities. Record that activation's actual identity;
   do not invent a future backlog commit in an earlier manifest. Any required catalog refresh
   remains separate.
6. Only after all prerequisites resolve may authoring begin within the exact approved regions.
   Any unresolved prerequisite, contradictory authority or unlisted consequential change stops
   dependent authorship. Neither this pointer nor private activation alone enables runtime work.

The immediate scope is the bounded pure Stage1 decision/preparation/reason/encoding core and its
exact approved wiring, generator and acceptance tests. No capability issuance, key/trust
operations, durable ledger/storage implementation, proxy-side credentials/dispatch/approval
ingress/evidence acquisition, effectful module execution, provider effects, deployment or
release is implicitly included. Existing v1 behavior and proof expectations remain unchanged.
An authority ceiling is not a final implementation manifest.

## Acceptance and provenance

The [constitution](../.specify/memory/constitution.md) remains binding. Consequential regions
remain T0, with truthful authorship by region; supervision, review or merge does not convert
agent-applied text or code into founder-authored work. The private activation exceptions do not
waive constitutional implementation requirements or extend to other stages.

Retain independent human and nonauthor cross-model review, adversarial acceptance expectations,
and at least three applicable SAST tools under Principle 8. A proposed tool list or zero findings
does not establish suitability or security-property coverage. Cargo-deny supplies additional
dependency/license/advisory analysis and does not count as SAST; conformance tests and Pylint
scores do not substitute for that requirement. Disposition findings, warnings, skips and
extraction/coverage gaps. Runtime test evidence follows authorized implementation, not before
it exists. Require the applicable checks and founder semantic/provenance approval on the actual
final implementation head, followed by human merge. Changed bytes require affected evidence to
be revisited.

This reference is documentation for code authors and reviewers, not an implementation or release
plan. No working enforcement, production readiness or non-bypassability claim follows from it.
