# CORE-005 required-CI prerequisites and scoping report

- **Status:** Reconnaissance complete; founder decisions required
- **Prepared:** 2026-08-21
- **Repository:** `DGR-AI-Labs/dgr-core`
- **Reviewed canonical commit:**
  `74e0f179441478356aa53818d1100b4b7a8dbf4a`
- **Branch:** `codex/core-005-scoping`
- **Scope:** documentation and draft backlog structure only

No workflow, branch-protection setting, code, Rust test, attack expectation,
suppression/baseline, Cargo file, or founder-owned T0 unit was changed by this
reconnaissance.

## Executive finding

The implementation precondition is **substantively satisfied** for the T0
surfaces that the current suite exercises:

- CORE-002 Step 5's completed human, cross-model, adversarial, and
  three-engine review was formally accepted as post-merge remediation
  (`qa/core-002-step5-governance-disposition.md:3-18`).
- CORE-003 records its review gate as complete, including founder APPROVE,
  independent-human PASS, merged implementation PR #73, and merged evidence PR
  #74 (`qa/core-003-t0-review-readiness.md:3-24,38-57`).
- CORE-004 records independent-human PASS and founder APPROVE against exact
  commit `60febb08...` (`qa/core-004-t0-founder-review-input.md:152-177`;
  `qa/core-004-t0-independent-human-review-input.md:108-134`). Implementation
  PR #81 and evidence PR #82 are both merged.

There is, however, a **documentation inconsistency to reconcile before CI
wiring**: `tests/bypass-rust/T0-BOUNDARY.md:27-30,83-90` and
`T0-AUTHORS.md:51-59` still say the applicable gates are pending. Those summary
sentences predate the completed evidence and merges. They do not invalidate the
detailed signed records, but leaving them stale would make the CORE-005
precondition ambiguous to the next reviewer.

The central CI finding is narrower: the Rust conformance job already runs and
passes on every push and pull request, but GitHub currently requires only
`Structural / governance check`. `Rust format / build / test` is green but is
**not a required status check**. CORE-005 is therefore primarily a durability
and repository-settings change, plus any founder-selected additions such as
Clippy or an exact ignored-set guard.

CORE-005 remains **To Do**. This report does not claim ATK-01..14 green and does
not activate RUNTIME-003, RUNTIME-004, or RUNTIME-006.

## 1. Current CI inventory

### 1.1 Definitions and triggers

GitHub reports one active Actions workflow:
`.github/workflows/ci.yml`. No CircleCI, GitLab CI, Jenkins, Azure Pipelines, or
Buildkite configuration is present in the tracked tree.

The workflow is named `CI` (`.github/workflows/ci.yml:1`) and runs on:

- every pushed branch (`.github/workflows/ci.yml:5-8`); and
- every pull request, without a base-branch filter
  (`.github/workflows/ci.yml:8-9`).

It grants read-only repository contents permission
(`.github/workflows/ci.yml:10-11`).

### 1.2 Jobs and exact commands

| Job/check context | Exact command(s) | Current `main` result |
|---|---|---|
| `Structural / governance check` | `node scripts/check-structure.mjs` (`.github/workflows/ci.yml:15-24`) | PASS |
| `Rust format / build / test` | `rustup component add rustfmt`; `cargo fmt --manifest-path tests/bypass-rust/Cargo.toml --all -- --check`; `cargo build --manifest-path tests/bypass-rust/Cargo.toml --all-targets --locked`; `cargo test --manifest-path tests/bypass-rust/Cargo.toml --all-targets --locked` (`.github/workflows/ci.yml:26-38`) | PASS |

The latest canonical run is GitHub Actions run
[`32515796879`](https://github.com/DGR-AI-Labs/dgr-core/actions/runs/32515796879),
bound to `74e0f179441478356aa53818d1100b4b7a8dbf4a`. Both jobs passed. The complete
run took about 22 seconds; the Rust job took about 17 seconds and the structural
job about 7 seconds. The ten most recent `main` runs inspected were all green.

The current workflow does **not** run Clippy. A cold local feasibility run of
the proposed command passed:

```text
cargo clippy --manifest-path tests/bypass-rust/Cargo.toml \
  --all-targets --locked -- -D warnings
result: PASS
cold elapsed time: 12.57 seconds
```

The current workflow does **not** run Semgrep, CodeQL, or cargo-deny. Its only
SAST section is a commented-out placeholder and executes nothing
(`.github/workflows/ci.yml:40-60`). GitHub CodeQL default setup reports
`not-configured`, and the canonical commit has exactly two check runs—the two
jobs above.

### 1.3 What the structural check actually enforces

`scripts/check-structure.mjs`:

1. requires 18 named files to exist (`scripts/check-structure.mjs:13-36`); and
2. requires four disclaimer phrases in `README.md`
   (`scripts/check-structure.mjs:38-53`).

It does **not**:

- inspect a diff or author identity;
- protect any founder-owned path or function named in `T0-AUTHORS.md:22-49`;
- compare the ignored attack set;
- validate attack outcomes or the registry;
- run Rust formatting, build, Clippy, tests, or SAST; or
- enforce reviewer/writer separation.

The script's comments still describe Phase 0 and claim the bypass suite is
expected red (`scripts/check-structure.mjs:1-5`). `README.md:9-18` and
`package.json:15` also retain Phase-0/red-scaffold language. That stale prose is
outside the mechanical CORE-005 gate itself, but should be corrected in a
tracked documentation child rather than allowed to remain misleading.

Consequently, the currently required structural context does **not** make the
T0 authorship boundary mechanical. GitHub cannot protect function bodies by
itself; a path/diff ownership rule would require additional design and a
reviewable implementation.

### 1.4 Legacy TypeScript job

The permanently red Node bypass job was removed in commit
`8d5ee32bb0a770b32897692e8bf64ec61792bc37` (`ci: retire stale TypeScript
check (superseded by FND-14 Rust-canonical)`). That commit replaced
`node --test tests/bypass` with the Rust job now at
`.github/workflows/ci.yml:26-38`.

The legacy command remains available as a local package script
(`package.json:11-15`), and the old scaffold files remain required by the
structural script (`scripts/check-structure.mjs:19-25`), but it is not invoked
by any active workflow and is not attached to canonical `main` as a check run.
There is therefore no permanently red legacy check today.

### 1.5 Reproduced local results

All commands used temporary Cargo target directories where compilation was
needed and left the worktree clean.

| Command | Result | Cold/local elapsed |
|---|---:|---:|
| `node scripts/check-structure.mjs` | PASS; 18 paths present | 0.35 s |
| `cargo fmt --manifest-path tests/bypass-rust/Cargo.toml --all -- --check` | PASS | 0.85 s |
| proposed locked all-target Clippy command | PASS | 12.57 s |
| `cargo test --manifest-path tests/bypass-rust/Cargo.toml --all-targets --locked` | PASS; 52 passed, 5 ignored, 0 failed | 13.31 s |

The current jobs are deterministic and fast in this small crate. The principal
CI variability is tool/download infrastructure: `ubuntu-latest` and the Rust
toolchain are not pinned by a tracked `rust-toolchain.toml`, `rustup component
add rustfmt` may access the network, and a cold Cargo build may download the
locked crates. `--locked` prevents version drift but does not eliminate
registry or runner availability risk.

## 2. What “required” means in `dgr-core`

### 2.1 Readable GitHub enforcement state

The settings are readable from this environment. Evidence was queried on
2026-08-21 from:

- `GET /repos/DGR-AI-Labs/dgr-core/branches/main/protection`;
- `GET /repos/DGR-AI-Labs/dgr-core/rulesets?includes_parents=true`; and
- GitHub UI: **Settings → Branches → Branch protection rules → `main`**.

Current `main` protection is:

| Setting | Current value |
|---|---|
| Required status checks | `Structural / governance check` only |
| Require branches to be up to date | Yes (`strict: true`) |
| Required approving reviews | 0 |
| Dismiss stale reviews | No |
| Require code-owner review | No |
| Require approval of most recent push | No |
| Enforce for administrators | Yes |
| Require linear history | No |
| Require conversation resolution | No |
| Allow force pushes / deletions | No / No |
| Additional repository/inherited rulesets | None |

Thus “required” has a precise current meaning: GitHub blocks even an
administrator from merging a non-up-to-date PR unless the structural context
passes. It does **not** presently require the green Rust context or a submitted
human review.

### 2.2 Convention versus mechanical enforcement

Repository policy says branch → PR → human merge and forbids direct pushes to
`main` (`AGENTS.md:29-31`; `.specify/memory/constitution.md:135-142`). The
observed T0 sequence follows that convention: PRs #68, #70, #73, #81, and #82
were merged by the founder account `sapenov`.

The convention is **not mechanically equivalent to required review**:

- branch protection requires zero approvals;
- none of those five PRs has a submitted GitHub review event; and
- GitHub protection does not express “an agent may never merge” when the agent
  operates through the same authenticated founder/admin identity.

A generic one-approval rule is possible, but a sole PR author cannot satisfy
their own required approval. A CODEOWNERS rule is also path-based, not
function-body-based, and still requires an eligible reviewer distinct from the
author. The founder must decide whether CORE-005 adds a mechanical review rule,
who can supply the independent GitHub approval, and whether T0 path ownership
is enforced with CODEOWNERS or remains a signed-evidence plus human-merge
process.

### 2.3 CodeCommit interaction and scope

CORE-005 is scoped to GitHub repository `dgr-core` only.

Read-only AWS inspection found:

- no CodeCommit approval-rule templates in `us-east-1` or `us-west-2`;
- no repository triggers on `dgr-backlog` or `dgr-internal`; and
- one post-`main` autopublish pipeline per repository. The backlog pipeline
  sources `dgr-backlog/main` and then runs `ValidateGateAndPublishBacklog`; the
  reference pipeline sources `dgr-internal/main` and then runs
  `GateAndPublishReference`.

Those pipelines validate/publish commits **after they reach `main`**. They are
not pre-merge required checks equivalent to GitHub branch protection. Changing
their behavior is separate backlog/reference publishing scope and is not part
of CORE-005.

## 3. Founder decisions—unresolved

No option below is selected by this report.

### D1 — Required build/test/governance set

**Current evidence:** format, build, and locked all-target tests already share
one green check context; Clippy is absent; the structural context is already
required.

**Options:**

1. Require the existing `Rust format / build / test` context plus the existing
   structural context. This is the smallest settings-only change and makes the
   current conformance command blocking, but does not add Clippy.
2. Add locked all-target Clippy with `-D warnings` to the Rust job, then require
   both job contexts. This adds useful compiler lint coverage at low current
   cost, but unpinned Rust releases can introduce new warnings and block merges.
3. Split format, Clippy, conformance, and structure into separate job contexts.
   This improves failure attribution and granular required settings, at the
   cost of more runner startup/download work and more settings to keep in sync.

The existing standalone `cargo build` is partially redundant once Clippy and
all-target tests both compile the crate, but retaining it keeps the current
explicit build gate. Removing it would be a separate founder-approved workflow
choice.

`{FOUNDER-SUPPLY: select the exact required status-check contexts, whether
Clippy is added, whether jobs are split, and whether the explicit build step is
retained}`

**Mechanical human-review subdecision:**

`{FOUNDER-SUPPLY: retain zero GitHub approvals with signed repository evidence
and founder merge, or require N approvals / code-owner review and name an
eligible independent GitHub reviewer}`

### D2 — SAST/SCA in CI: informational or blocking

The constitutional T0 bar is human review, adversarial testing, cross-model
review, and at least three SAST tools
(`.specify/memory/constitution.md:92-105`). The established project label is
“three-engine SAST/SCA gate: Semgrep, CodeQL, and cargo-deny”
(`qa/core-002-step5-review-readiness.md:60-72`). Existing evidence is bound to
an exact reviewed commit and explicitly adjudicated by the founder
(`qa/sast/README.md:11-15,27-40,93-100`). A generic green CI invocation is not a
substitute for that human disposition.

Known live results on the latest reviewed T0 surface are:

- Semgrep: one INFO `temp-dir` result in a test
  (`qa/core-004-post-authoring-claude-qa-disposition.md:41-46`);
- CodeQL: nine deterministic fixture-nonce results plus seven diagnostic-only
  path-resolution inconsistencies at full 20/20 extraction
  (`qa/core-004-post-authoring-claude-qa-disposition.md:41-46`); and
- cargo-deny: clean, with two reviewed temporary duplicate-version skips and an
  empty advisory ignore list (`deny.toml:1-2,18-28`).

**Options:**

1. Run SAST/SCA informationally in CI and retain exact-commit raw evidence plus
   founder dispositions as the blocking T0 review gate. This avoids a
   permanently red merge check while still surfacing drift on every PR.
2. Make all three tools blocking only after an explicit reviewed
   baseline/suppression policy is designed. That policy must distinguish
   accepted test fixtures from new findings and must itself be reviewed; a
   broad suppression would weaken the gate.
3. Make only cargo-deny blocking now and keep Semgrep/CodeQL informational.
   This is operationally feasible because cargo-deny is clean, but it creates a
   mixed policy and must not be described as satisfying the full three-engine
   human T0 gate by itself.

Making Semgrep or CodeQL required without a reviewed baseline would recreate a
permanently red check and invite per-merge overrides—the anti-pattern removed
with the legacy Node job.

`{FOUNDER-SUPPLY: select informational versus blocking behavior for each of
Semgrep, CodeQL, and cargo-deny; if blocking, approve the baseline ownership and
review process}`

### D3 — Reproducibility and `--locked`

`Cargo.lock` is committed (`tests/bypass-rust/Cargo.lock:1-9`) and both current
build and test commands use `--locked` (`.github/workflows/ci.yml:35-38`). The
proposed Clippy command also passed with `--locked`.

The test gate should retain the exact form:

```text
cargo test --manifest-path tests/bypass-rust/Cargo.toml --all-targets --locked
```

`{FOUNDER-SUPPLY: confirm --locked is mandatory for every required Cargo
build/check/clippy/test invocation, and decide whether the Rust toolchain must
also be pinned}`

### D4 — Exact ignored-set guard

The structural check does not inspect ignored Rust tests. The exact current
ignored set is:

- ATK-04, ATK-05, ATK-12, and ATK-14 through the macro at
  `tests/bypass-rust/tests/attack_set.rs:310-317,591-594`; and
- ATK-15 as an external IAM case at
  `tests/bypass-rust/tests/attack_set.rs:607-610`.

Feasible placements:

1. **Recommended shape if adopted:** a T3 script runs or consumes
   `cargo test ... --test attack_set -- --list --ignored`, normalizes the five
   test names, and compares them to one explicit expected set. It fails for
   either an added ignore or a missing ignore. This observes libtest's actual
   metadata rather than maintaining a loose source-text grep.
2. Add a static rule to `scripts/check-structure.mjs`. This is fast, but parsing
   Rust macro invocations as text is more brittle and still does not prove what
   libtest registered.
3. Add a Rust test that reads sibling source. A normal Rust test cannot directly
   introspect another test's `#[ignore]` attribute, so this also devolves into
   source parsing and is the least attractive option.

The guard is T3 only if it **observes** the exact ignored set and never changes
registry outcomes, enforcement, or the attack expectations. `attack_set.rs` is
listed as an agent-authored conformance surface (`T0-AUTHORS.md:64-77`), but the
constitution forbids weakening a test to obtain green
(`.specify/memory/constitution.md:117-123`). Any change that reclassifies an
attack, alters a required outcome, or makes founder-owned enforcement pass is
outside this T3 child and must stop for founder ownership review.

`{FOUNDER-SUPPLY: adopt or reject the ignored-set guard; if adopted, select the
libtest-enumeration script, structural source check, or another reviewed design}`

### D5 — Gate claim while attacks remain unimplemented

The registry contains ATK-01 through ATK-15
(`tests/bypass-rust/src/lib.rs:108-261`). Current isolation state is:

- **active gate cases:** ATK-01, ATK-02, ATK-03, ATK-06, ATK-07, ATK-08,
  ATK-09, ATK-10, ATK-11, and ATK-13;
- **ignored/deferred:** ATK-04, ATK-05, ATK-12, and ATK-14; and
- **ignored external-IAM case:** ATK-15.

The exact locked all-target command passes with 52 tests passed, 5 ignored, and
0 failed. `--all-targets` compiles and runs all test targets; it does not run
tests carrying `#[ignore]` unless `--ignored` or `--include-ignored` is supplied.
Therefore making the current job required does not force deferred attacks to
pass.

The truthful CORE-005 claim at this stage is: **all currently active
conformance tests must pass on every merge, and the exact deferred/external set
must remain visible and unchanged without review.** It is not “ATK-01..14 are
green.”

Canonical runtime items intentionally require the conjunction **CORE-005 Done
and ATK-01..14 green**. CORE-005 alone cannot activate them while
ATK-04/05/12/14 are deferred.

`{FOUNDER-SUPPLY: approve the bounded active-suite claim and confirm that
CORE-005 completion alone does not satisfy the ATK-01..14 runtime trigger}`

## 4. Ownership and tier pre-check

| Surface | Proposed tier/owner | Boundary |
|---|---|---|
| Workflow YAML and non-consequential helper scripts | T2/T3, Engineering | May run existing checks; must not alter T0 behavior or expectations. |
| GitHub required-check/review settings | Founder-executed external governance action | Agent may document and verify; founder approves and applies the durability decision. |
| Ignored-set observer | T3, Engineering, if D4 adopted | Compare metadata only; never add/remove ignores or redefine outcomes. |
| SAST CI plumbing | T2 tooling with founder human gate | Any suppression/baseline affecting T0 findings receives explicit human review; no silent baseline. |
| `T0-BOUNDARY.md`, `T0-AUTHORS.md`, README/Phase language reconciliation | T3 docs, Engineering with founder review | Historical statements only; no T0 source change. |
| Founder-owned Rust units and consequential enums/traits | T0, Founder | Review-only for agents; unchanged by CORE-005 wiring. |
| Attack outcomes or enforcement-consumed registry semantics | Treat as T0 until founder narrows | Not part of the ignored-set observer or CI plumbing. |

The material contradiction to settle before implementation is not an
authorship conflict in the proposed T3 observer; it is the stale “gates
pending” language in the two ownership summaries. The signed QA records and
merged PRs say the gates are complete, while `T0-BOUNDARY.md` and
`T0-AUTHORS.md` have not yet been reconciled. Track that correction explicitly
and obtain founder review before flipping the Rust job to required.

## 5. Draft backlog structure—do not finalize

These are draft records for a later `dgr-backlog` PR. All remain **To Do** and
retain unresolved founder supply.

### `CORE-005` — Wire the bypass suite as a permanent CI gate

- **Status:** To Do
- **Owner:** Engineering
- **Dependencies:** CORE-002, CORE-003, CORE-004 plus the children below
- **Context:** The applicable CORE-002/003/004 T0 evidence gates are complete,
  subject to reconciliation of stale ownership summaries. Require the active
  suite without claiming deferred attacks green.
- **Acceptance:** selected required checks block merges; a seeded bypass makes
  the conformance context fail; the exact ignored set cannot drift silently;
  settings and ownership evidence are durable.
- **Definition of Done:** `{FOUNDER-SUPPLY: exact required contexts, review
  settings, SAST mode, ignored-set policy, toolchain pinning, and bounded claim}`
- **Human gate:** Founder approves D1-D5 and verifies final GitHub settings.

### `CORE-005-PRECONDITION-RECONCILIATION` — Reconcile completed T0 gate summaries

- **Tier/owner:** T3 documentation / Engineering; founder review
- **Status:** To Do
- **Acceptance:** `T0-BOUNDARY.md`, `T0-AUTHORS.md`, README, package metadata,
  and structural comments no longer state obsolete Phase-0 or pending-gate
  facts; history and bounded claims remain accurate.
- **No T0 Rust change.**

### `CORE-005-CI-REQUIRED` — Make selected Rust and governance checks blocking

- **Tier/owner:** T2/T3 CI / Engineering; founder applies repository settings
- **Status:** To Do
- **Dependency:** `CORE-005-PRECONDITION-RECONCILIATION`
- **Acceptance:** workflow implements the D1/D3 selections; branch protection
  requires the exact approved contexts with strict up-to-date behavior; a
  temporary seeded-bypass validation demonstrates red without committing a
  weakened test; green is restored only by removing the seed.
- **Definition of Done:** `{FOUNDER-SUPPLY: required contexts and mechanical
  review policy}`

### `CORE-005-IGNORED-SET-GUARD` — Pin the exact deferred/external attack set

- **Tier/owner:** T3 observer / Engineering
- **Status:** To Do if D4 is adopted; otherwise record “Not adopted” with the
  founder rationale rather than silently dropping it
- **Acceptance:** CI fails on an added or removed ignored attack; expected set
  is exactly ATK-04/05/12/14 plus external ATK-15; the observer cannot alter
  expectations or enforcement.
- **Definition of Done:** `{FOUNDER-SUPPLY: selected mechanism and evidence}`

### `CORE-005-SAST-CI` — Implement the selected SAST/SCA CI posture

- **Tier/owner:** T2 tooling / Engineering; founder human gate
- **Status:** Conditional To Do after D2
- **Acceptance:** informational versus blocking behavior is explicit per
  engine; exact-commit founder disposition remains mandatory for T0; any
  baseline is narrow, reviewed, versioned, and fails on new findings.
- **Definition of Done:** `{FOUNDER-SUPPLY: per-engine mode, baseline policy,
  and ownership}`

### Runtime trigger fence

RUNTIME-003, RUNTIME-004, and RUNTIME-006 remain Deferred. Their activation
condition is the conjunction:

```text
CORE-005 == Done AND ATK-01..14 == green
```

Because ATK-04/05/12/14 remain unimplemented, completion of CI wiring alone
does not satisfy that condition.

## 6. Founder decision checklist

- [ ] D1: exact required checks/jobs selected.
- [ ] D1 review subdecision: GitHub approval/CODEOWNERS policy selected and an
      eligible reviewer named if approvals become required.
- [ ] D2: Semgrep, CodeQL, and cargo-deny each classified informational or
      blocking; baseline policy approved if needed.
- [ ] D3: `--locked` and optional Rust-toolchain pinning confirmed.
- [ ] D4: ignored-set guard adopted/rejected and mechanism selected.
- [ ] D5: bounded active-suite claim and runtime-trigger fence approved.
- [ ] Ownership split approved before any workflow, test, or settings change.

## 7. Reconnaissance command evidence

Commands used for externally held facts:

```text
gh api repos/DGR-AI-Labs/dgr-core/branches/main/protection
gh api 'repos/DGR-AI-Labs/dgr-core/rulesets?includes_parents=true'
gh api repos/DGR-AI-Labs/dgr-core/actions/workflows
gh api repos/DGR-AI-Labs/dgr-core/code-scanning/default-setup
gh api repos/DGR-AI-Labs/dgr-core/commits/<main>/check-runs
gh run list --repo DGR-AI-Labs/dgr-core --workflow ci.yml --branch main
gh run view 32515796879 --repo DGR-AI-Labs/dgr-core
aws codecommit list-approval-rule-templates --region us-east-1/us-west-2
aws codecommit get-repository-triggers --repository-name dgr-backlog/dgr-internal
aws codepipeline get-pipeline --name <autopublish pipeline>
```

No external setting was modified.
