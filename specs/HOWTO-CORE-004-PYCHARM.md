# HOWTO: review CORE-004 in PyCharm

## Purpose and current gate

Use this workspace for CORE-004 documentation and ownership-map review. The
founder-confirmed design is pinned by `specs/CORE-004-reference-contract.md`,
but opening this project does not authorize implementation or start the
separately gated VAL/T3/T0 steps.

The current branch is documentation-only:

- worktree: `/mnt/c/Users/Khazret/Documents/.codex-worktrees/dgr-core004-design-ownership`;
- branch: `codex/core-004-design-ownership`;
- Cargo manifest: `tests/bypass-rust/Cargo.toml`.

Before any Rust, fixture, or test authoring, confirm in the canonical backlog
that `CORE-004-DESIGN` is Done with evidence and that `CORE-004-GOVERNANCE` is
Done after founder review. ATK-06 must remain ignored until the
founder-authored T0 implementation turns the reviewed RED tests green.

## Open the project

1. In PyCharm, choose **File → Open**.
2. Open the worktree directory above as a separate project. Do not open the
   historical `dgr-core-step1` worktree for CORE-004 work.
3. Enable the Rust plugin if the IDE does not recognize `Cargo.toml`.
4. In **Settings → Languages & Frameworks → Rust**, attach
   `tests/bypass-rust/Cargo.toml` as the Cargo project.
5. Select the WSL Rust toolchain. At the time this HOWTO was validated, the
   environment reported `rustc 1.94.1`, `cargo 1.94.1`, and
   `/home/khazret/.cargo/bin/rust-analyzer`.
6. Let PyCharm index the pinned dependency graph, but do not run an update or
   allow the IDE to rewrite `Cargo.lock`.

If PyCharm cannot resolve the crates, verify the configured toolchain and
`rust-analyzer` path before changing project files. Dependency installation or
upgrades are not a valid workaround for an indexing problem.

## Recommended run configurations

Create Cargo configurations with the repository worktree as the working
directory:

| Name | Cargo command |
|---|---|
| CORE-004 — all targets | `test --manifest-path tests/bypass-rust/Cargo.toml --all-targets --locked` |
| CORE-004 — build | `build --manifest-path tests/bypass-rust/Cargo.toml --all-targets --locked` |
| CORE-004 — format check | `fmt --manifest-path tests/bypass-rust/Cargo.toml --all -- --check` |
| CORE-004 — clippy | `clippy --manifest-path tests/bypass-rust/Cargo.toml --all-targets --locked -- -D warnings` |

Do not create a run configuration that un-ignores ATK-06 or changes its
expected outcome merely to obtain green. A future dedicated RED-test command
must be copied from the reviewed `CORE-004-T3-tests` change, not invented in
this setup document.

## Branch and authorship separation

Keep one worktree per review surface:

| Surface | Proposed worktree | Branch | Authoring boundary |
|---|---|---|---|
| Design pointer and ownership documentation | `.codex-worktrees/dgr-core004-design-ownership` | `codex/core-004-design-ownership` | Documentation only; founder review required |
| VAL-004 fixtures | `.codex-worktrees/dgr-core004-val004` | `codex/core-004-val004` | T3 only after the contract is frozen |
| RED conformance tests | `.codex-worktrees/dgr-core004-t3` | `codex/core-004-t3-tests` | T3 only after VAL-004 is reviewed |
| Founder implementation | `.codex-worktrees/dgr-core004-t0-founder` | `codex/core-004-t0-founder` | Founder-authored T0 only |

The last three worktrees are plans, not instructions to create or start them
early. Create each only when its canonical backlog dependency is satisfied.

## Files to review after the design is confirmed

- `specs/CORE-002-reference-contracts.md`: the established pointer-not-copy
  pattern for pinned dgr-internal contracts;
- `specs/CORE-004-reference-contract.md`: the immutable design pointer and
  drift-detection metadata for this work;
- `T0-AUTHORS.md`: the binding founder-authored unit map;
- `tests/bypass-rust/T0-BOUNDARY.md`: isolation and non-claim boundary;
- `tests/bypass-rust/src/`: future implementation/support location, subject to
  the recorded T0/T3 ownership split;
- `tests/bypass-rust/tests/`: future conformance-test location.

Do not copy the CORE-004 design body into dgr-core. Once founder-confirmed and
published in dgr-internal, dgr-core should contain only the pinned reference
metadata and the ownership/boundary documentation needed to consume it.

## No-shadow-work checklist

Before starting any follow-on activity, verify it is represented by one of the
canonical CORE-004 backlog records:

- founder design decision and publication: `CORE-004-DESIGN` (dgr-internal
  pull request #3);
- reference pointer, ownership-map/boundary documentation, workspace HOWTO,
  and runtime-item ID reconciliation: `CORE-004-GOVERNANCE` (proposed until
  its backlog PR is merged);
- deterministic scenario data: `VAL-004`;
- RED conformance scaffolding: `CORE-004-T3-tests`;
- founder-only consequential implementation: `CORE-004-T0-boundary`;
- real human delivery/wait and cross-instance behavior: `RUNTIME-006`, the
  deferred item allocated without overwriting the existing `RUNTIME-005` risk
  register.

If an activity does not fit one of those records, add or amend a backlog item
before authoring. A local IDE task, scratch branch, or review note is not a
substitute for canonical backlog coverage.

## Commit hygiene

`.idea/` is ignored by the repository `.gitignore`; this was confirmed with
`git check-ignore`. Still run `git status --short` before each commit. A
documentation/ownership change must not include `.rs`, `Cargo.toml`,
`Cargo.lock`, generated build output, or IDE metadata.
