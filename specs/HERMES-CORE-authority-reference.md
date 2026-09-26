# Hermes core authority reference

**Status: proposed authority pointer; private source is draft; implementation prohibited.**

This code-governance reference accompanies proposed Constitution 5.0.0. It identifies private
review material without copying the planning packet, manifest or registry into this repository.
It is separate from the existing runtime and offline issuer exceptions.

## Exact private identity

```text
Repository: dgr-internal (canonical CodeCommit, us-west-2)
ID: HERMES-CORE-AUTHORITY-001
Status at this pin: draft, not registered as an ADR, not active
Commit: ba27c8ff3943baefb4f3fef77c27623a767aaecc
Path: qa/hermes-planning/HERMES-CORE-AUTHORITY-001.md
Git blob: 17892c7a3b1e30de3a9968f78c6e26a5761e8de3
SHA-256: 0ab5d826171e84cc33d5bc9d69cc365386825198e0bc6a14f42d5befdefa7d84
```

Resolve all identities against canonical private Git history. An updated filename, branch or
hash alone is not an adoption record. A later active authority needs its own reviewed exact pin;
merging the current draft cannot silently change this pointer's status.

## Preconditions and boundaries

1. Founder adopts active private authority, resolves its registration requirements and explicitly
   dispositions the Principle 6 scope justification. The current draft satisfies none of these.
2. Human review and merge adopt the public constitution and updated authority reference.
3. Founder-owned reason/action/attack registry and complete technical contracts are independently
   reviewed and frozen. A code author cannot invent policy meanings or acceptance expectations.
4. Founder approves an exact implementation manifest with repository/base, files/symbols, behavior,
   provenance, dependencies/features, tests, analyzers, reviewers and hard stops.
5. Canonical CodeCommit backlog explicitly activates only the approved scope, citing exact authority
   and manifest identities. Any required reference-catalog refresh remains a separate founder action.
6. Only then may authoring begin within the exact approved core regions. Any unresolved prerequisite,
   contradictory authority or unlisted consequential change stops dependent authorship.

All consequential regions remain T0. No proxy-side credential, dispatch, approval-ingress or evidence
acquisition code, production provisioning, operational token issuance, arbitrary module execution in
the proxy, multi-instance service, deployment or release claim is implicitly included. Existing v1
semantics and proof expectations remain unchanged. Private planning and full manifests stay private.

## Acceptance and provenance

Preserve truthful authorship by region; supervision/review/merge does not make agent-written code
founder-authored. Retain independent human and nonauthor cross-model review, adversarial tests from
the reviewed registry, at least three applicable analysis tools (Semgrep, CodeQL and cargo-deny plus
required language/dependency coverage), and disposition of findings, warnings, skips and extraction
gaps. Require all checks and founder semantic/provenance approval on the actual final head, followed
by human-only merge. Changed bytes invalidate affected evidence. Pylint cannot replace these gates.

This reference is documentation for code authors and reviewers, not an implementation or release
plan. No working enforcement, production readiness or non-bypassability claim follows from it.
