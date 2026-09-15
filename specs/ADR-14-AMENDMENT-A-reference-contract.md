# ADR-14 Amendment A reference contract

**Status:** active authority reference — RUNTIME-002 implementation not yet authorized

**Purpose:** pin the public Constitution 3.0.0 exception to exact private authority without copying
the internal ADR or planning records into this public repository.

## Immutable private authority

```text
Authority ID: ADR-14-AMENDMENT-A
Authority status: active
dgr-internal activation commit: 1313a33b7bccb3abf2d8c6d58deae8327c0fb8df
ADR path: specs/adr/ADR-14-AMENDMENT-A-supervised-agent-runtime-002-authorship.md
ADR git blob: b4f758628b5f1567d2c35a56f3dd17a6ba0b9a0d
ADR SHA-256: 0bdd72b28f389acd35be73464fe72cb7dee52482c41297949388169d2e7d771d

Founder review commit: f45596bfbf3ac42e2cc7f75ce9adf38c600cb29e
Founder review path: qa/runtime-002-planning/ADR-14-AMENDMENT-A-FOUNDER-REVIEW.md
Founder review git blob: dee7590916d7116583c2b764712842adf47d2d91
Founder review SHA-256: d78640311b81f5346a295537964865e750feef95ce05b88afb3025723ec0284b

Activation-verification commit: 750f4f1a3d7d0a7f51bd0784edab68e4677413af
Activation-verification path: qa/runtime-002-planning/ADR-14-AMENDMENT-A-ACTIVATION-VERIFICATION.md
Activation-verification git blob: 78660e58fbaf34cb3da36bb9cb749d8baa3b8b1d
Activation-verification SHA-256: 39bd885f0c8a2c80ed3a1255f0f484b0f9eb48d45a7431f14c97571a90fdade6
```

An authorized reviewer must resolve these identities against canonical `dgr-internal`. A branch
name, moving `main` reference, copied private body, or digest without its resolving commit and blob
is insufficient. The private source remains authoritative if this summary conflicts with it.

## Allowed responsibility summary

The exception may cover only founder-approved, manifest-enumerated RUNTIME-002 T0 regions required
for:

1. stable binding of one namespaced OpenClaw tool identifier and authorization-relevant arguments;
2. the minimal owned-input/owned-output same-process Rust host-language contract;
3. founder-selected initialization, compatibility, timeout, panic, shutdown, and failure behavior;
4. founder-selected trusted ports and state lifecycle;
5. frozen-profile startup enforcement; and
6. strictly necessary Cargo, module/export, build, CI, ownership, boundary, and evidence wiring.

This responsibility summary does not authorize a path, symbol, dependency, generated artifact, or
implementation choice. Those become eligible for authoring only if the founder later approves them
in the exact pre-authoring manifest.

## Load-bearing prerequisites

Agent-authored RUNTIME-002 T0 remains prohibited until all of the following are complete in order:

1. this active authority and its founder review and activation verification resolve exactly;
2. Constitution 3.0.0 and this pointer are independently reviewed and founder-merged;
3. the canonical internal backlog explicitly activates RUNTIME-002 under the same authority;
4. a preparation record freezes every founder-owned design/trust decision, repository, path,
   symbol, dependency, generated artifact, branch/base identity, expected change, test, analyzer,
   reviewer, provenance class, and hard stop; and
5. the founder approves that exact preparation record before any T0 authoring begins.

Any unresolved decision, contradictory authority, missing prerequisite, or unlisted consequential
path is a hard stop. The agent may not infer or silently broaden the manifest.

## Provenance classes

Every later changed region must be classified truthfully as exactly one of:

- unchanged founder-authored source;
- agent-modified founder source, retaining the founder-source provenance record;
- existing ADR-13 Amendment-B agent-authored T0;
- new RUNTIME-002 agent-authored T0; or
- T3.

Founder supervision, review, approval, or merge does not convert agent-authored or
agent-transformed lines into founder-authored lines. A filename or path prefix is not an authorship
claim.

## Complete exact-final-head gate

Every later manifest-scoped T0 branch requires:

- adversarial tests for the selected boundary and failure semantics;
- Semgrep, CodeQL, and cargo-deny, plus language-appropriate host-code and dependency coverage;
- capture and founder disposition of every finding, warning, note, error, skipped rule, extraction
  diagnostic, and coverage gap;
- non-author cross-model review;
- independent-human review by a reviewer who authored none of the implementation or remediation;
- founder line-by-line provenance and semantic review and disposition of every finding;
- required checks passing on the actual final head;
- founder GitHub approval bound to that exact final head; and
- founder-only merge.

The authoring agent may not satisfy an independent review role, approve its own work, or merge it.
Any executable change after a review input invalidates affected reviews, tests, and scans.

## Exclusions and non-claims

This exception does not authorize Hermes, unrestricted tool routes, out-of-process or second
authorization boundaries, permissive fallback, policy changes, weakened tests, later mechanical
adapter/packaging work, runtime evidence, deployment, or product claims. It does not activate
RUNTIME-002 and does not authorize RUNTIME-003, RUNTIME-004, RUNTIME-005, or RUNTIME-006.

This exclusion list is a public summary; the active private Amendment A §A3 remains controlling.
In particular, restrictive same-user permissions are not isolation from the process that owns
them, and no finite route-coverage matrix supports a universal or unqualified non-bypassability
claim.

This reference contract and Constitution 3.0.0 do not prove live OpenClaw hook installation,
exactly-once interception, protected-tool coverage, route-around resistance, store/key/clock
integrity, deployed ATK-03/06/10 survival, operator resistance, or any other runtime property.
