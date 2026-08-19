# HOWTO: author VAL-004 fixtures in PyCharm

## Gate and workspace

VAL-004 is T3 fixture-data work, but it consumes founder-owned contracts. Do
not author fixtures until both of these are merged and resolvable:

1. dgr-internal PR #4 publishes `CORE-004-DESIGN-ADDENDUM-A`; and
2. the dgr-core pointer/governance PR containing this HOWTO pins its immutable
   source commit and digest.

The dedicated fixture workspace is:

- WSL path: `/mnt/c/Users/Khazret/Documents/.codex-worktrees/dgr-core004-val004`;
- Windows path: `C:\Users\Khazret\Documents\.codex-worktrees\dgr-core004-val004`;
- branch: `codex/core-004-val004`;
- Cargo project: `tests/bypass-rust/Cargo.toml`.

The worktree may be created in advance, but it remains blocked until the two
publication gates above merge. Refresh it from `origin/main` before authoring.

## Open and configure PyCharm

1. Choose **File → Open** and select the Windows path above as a separate
   project.
2. Enable the Rust plugin and attach `tests/bypass-rust/Cargo.toml`.
3. Select the WSL Rust toolchain and its `rust-analyzer`.
4. Do not update dependencies or permit the IDE to rewrite `Cargo.lock`.
5. Keep `.idea/` untracked; verify with `git status --short` before commits.

Create these Cargo run configurations with the worktree root as the working
directory:

| Name | Cargo command |
|---|---|
| Baseline all targets | `test --manifest-path tests/bypass-rust/Cargo.toml --all-targets --locked` |
| Fixture unit only | `test --manifest-path tests/bypass-rust/Cargo.toml --test val_004_fixtures --locked` |
| Build | `build --manifest-path tests/bypass-rust/Cargo.toml --all-targets --locked` |
| Format | `fmt --manifest-path tests/bypass-rust/Cargo.toml --all -- --check` |
| Clippy | `clippy --manifest-path tests/bypass-rust/Cargo.toml --all-targets --locked -- -D warnings` |

## Allowed VAL-004 files

Expected T3 changes are limited to:

- new `tests/bypass-rust/src/val_004_fixtures.rs` — deterministic data and
  fixture-only expected labels;
- new `tests/bypass-rust/tests/val_004_fixtures.rs` — data-shape, arithmetic,
  token-integrity, attack-tag, and regression assertions; and
- `tests/bypass-rust/src/lib.rs` — the mechanical module export only.

Do not edit or create any founder-owned approval port, SQLite store, guard,
adapter decision, observation variant, timeout evaluator, or conformance
constant. Do not edit `Cargo.toml` or `Cargo.lock`. ATK-06 remains ignored.

## Fixture shape to author after the gate opens

Use the existing VAL-002 signer and canonical-action helpers rather than
copying cryptography. The VAL-004 module should represent, but never evaluate:

- one valid token committed to amount `"1000001"`;
- one below-threshold control at the existing `"100000"` baseline;
- `requested_at = 1_800_000_000` and `deadline = 1_800_086_400`;
- explicit timeout-surface clocks `1_800_086_399`, `1_800_086_400`, and
  `1_800_086_401`;
- a fixed review-request id and the same original id/deadline on
  re-presentation; and
- fixture-only expected labels for `Escalated`, `Blocked`, and
  `ProceedNormally`, with terminal ATK-06 denial resolved from the registry by
  the later conformance test.

The escalation and timeout moments must be separate fixture surfaces. Do not
model them as two identical `before_tool_call` calls. The fixture may add
`requested_at + 86_400` because it is constructing frozen test data; it must
not decide whether a runtime request is timed out.

If the founder conformance mirrors do not yet exist, mark mirror equality as a
pending founder dependency. Do not create the mirrors in T3 code.

## Required evidence before the VAL-004 PR

- pointer drift check for both CORE-004 records;
- baseline all-target test before and after authoring;
- exact fixture test, format, build, and Clippy results;
- proof that no pre-existing fixture changes behavior;
- proof that ATK-06 remains ignored; and
- a name-only diff showing no founder/T0, Cargo, or lockfile changes.

The founder reviews the amount boundary, arithmetic, two-surface shape,
original-id/deadline immutability facts, and unchanged baseline before merge.
