# CORE-005 branch-protection probe

- **Date opened:** 2026-08-24
- **Scope:** T3 evidence-only probe; no T0, Rust, workflow, test, or protection-setting change
- **Target:** `DGR-AI-Labs/dgr-core` branch `main`
- **Expected required contexts:** `Structural / governance check`; `Rust format / build / test`
- **Expected approvals:** 1
- **Designated eligible independent reviewer:** Gaziz Nugmanov

## Purpose

This pull request exists only to observe the already-applied `main` branch-protection rule. It must demonstrate that:

1. merge is blocked while a required context, including `Rust format / build / test`, has not succeeded; and
2. after both required contexts succeed, merge remains blocked until one independent approval is recorded.

The probe does not weaken, bypass, or modify protection. Its observations are retained in pull-request metadata/comments and the canonical CORE-005 backlog evidence. After the required observations and approval are captured, this probe should be closed without merge; the evidence record, not this file, is the deliverable.

## Boundaries

- Do not change T0 enforcement, Rust source, tests, expectations, `#[ignore]`, CI, or branch protection from this branch.
- Do not treat informational Semgrep, CodeQL, or cargo-deny contexts as required checks.
- Do not mark CORE-005 Done until both blocked states and the eventual approved state are recorded.
