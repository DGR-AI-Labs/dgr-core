# CORE-002 Step 5 — Claude independent QA disposition (addendum)

- **Reviewer:** Claude (cross-model independent QA)
- **Date:** 2026-08-17
- **Reviewed artifact:** bundle `dgr-core-002-step5-claude-qa-2026-08-17.zip`
- **Bundle SHA-256:** `bdfd24e667cd9c54a6eeb36818722c949c33da32bde43b6e1ec6fe16c9e9ae67`
  (computed == provided; integrity confirmed)
- **T0 implementation commit:** `0727e327631b475990ef8d9b7ef3b2c3554050a8`
- **SAST-scanned commit:** `0a54d4995d1b9d98ab8a3ec61861fe2fe7ae29c3` (documented non-Rust
  descendant of the T0 commit)
- **Relates to / supersedes-scope-of:** `qa/core-002-step5-cross-model-review.md`

## Disposition

**PASS (independent cross-model review), no code defect found**, subject to the founder
confirmations below. This addendum records the confirmations-required list alongside the PASS,
so the bundle carries the reviewer's outstanding items rather than an unqualified pass.

## Code re-verification (from bundle source)

- Guard enforcement chain intact and correct: absent-token → signature/K2 → lifetime+expiry →
  typed-hash binding → single-use consumption → single `Allow`. `Allow` reachable only after all
  five checks pass. Outcomes registry-derived. ATK-03 single-use proven by the one-store /
  two-call test (first: `Proceeded`, 1 invocation; second: `Blocked`, 0 invocations).
- Consumption core sound: UNIQUE/PRIMARY-KEY-violation → `AlreadyConsumed` (replay → Deny);
  all other DB errors → `Faulted` (fail closed); one persistent connection per store;
  `Consumed` only after durable commit (persist-then-allow).

## Founder confirmations

### 1. `fail_closed_decision` fails closed — RESOLVED (verified at source)

The `Faulted` consume arm now routes through `fail_closed_decision(fault)`
(`founder_fail_closed.rs`), which returns `Ok(GuardDecision::Deny { outcome:
RequiredOutcome::FailClosed, .. })` for every `GuardFault` variant. A consumption fault becomes
an explicit fail-closed **Deny**, never an `Allow`. This is a correct improvement over the prior
`Err(f)` propagation. No action required; recorded as verified.

### 2. `0727e327` → `0a54d499` touches no `.rs` file — FOUNDER MUST VERIFY

The SAST evidence was scanned against `0a54d499`, asserted to be a non-Rust descendant of the T0
commit `0727e327` (intervening changes: QA records, `deny.toml`, Apache-2.0 metadata). The
reviewer has only the bundle snapshot, not both commits, and therefore **cannot** confirm this
from the bundle alone. Founder must verify the claim before relying on the SAST evidence as
covering the actual T0 code:
`git diff --stat 0727e327 0a54d499 -- '*.rs'` must be empty. If any `.rs` differs, the SAST
evidence does not cover the reviewed T0 code and must be re-run against the code commit.

### 3. cargo-deny `bans.skip` entries — FOUNDER MUST ADJUDICATE

`deny.toml` skips two duplicate-version bans:
`hashbrown@0.16.1` and `syn@2.0.119`, each rationaled as a temporary transitive/build-time
version split with "no advisory present." `advisories.ignore = []` (no known vulnerability is
suppressed). Founder to confirm: (a) each "no advisory present" claim is true against current
RustSec state (advisory status can change post-lockfile); (b) both are tracked as **temporary**
with a revisit trigger (dependency convergence), not permanent silent skips. Duplicate-version
skips are policy exceptions, not advisory waivers — acceptable when adjudicated and time-bounded.

### 4. Two SAST findings accepted as test-only — FOUNDER TO RECORD

Both are true-positive pattern matches confined to test/fixture code; correct in context:

- **CodeQL `rust/hard-coded-cryptographic-value`** ×7 in `val_002_fixtures.rs`
  (lines 126,152,169,186,204,222,339): deterministic fixture nonces (`[0x01;16]`, `[0x02;16]`,
  …), required for deterministic tests; not production keys/salts/nonces. Confirmed all in the
  fixture file, none in non-test T0 code. One grouped disposition may cover all seven.
- **Semgrep `rust.lang.security.temp-dir.temp-dir`** at `consumption_store.rs:19`: predictable
  temp-path (`temp_dir()` + pid + nanos) in the restart-durability test for the file-backed
  store. Local test-only race risk. Accept with rationale, or switch to the `tempfile` crate and
  re-run all three scans.

## Gate-composition note

SAST evidence is category-accurate per amended FND-7: **2 code SAST (Semgrep, CodeQL) + 1 SCA
(cargo-deny)**, with cargo-geiger not required. The disposition must not be described as
"3 SAST." cargo-deny final run: exit 0, 0 blocking diagnostics (54 license notes, 2 ban notes
for founder review).

## Scope limits of this disposition

This is a design/correctness + SAST-evidence review. It does **not** substitute for: the
independent human T0 review, the founder analyzer/checklist sign-off, PR approval, or merge.
It does not assert "CORE-002 complete" (CORE-003/004/005 and deferred attacks remain) nor
"bypass suite green," and does not trigger runtime integration (whose trigger is CORE-005 Done +
ATK-01..14 green). `CORE-002-STEP5` remains `In Review` until the human gates and merge complete.

---

## Repository verification appendix

**Verifier:** Codex repository check

**Date:** 2026-08-17

**Purpose:** resolve bundle-external facts without converting Claude's review
or this appendix into founder adjudication.

### Confirmation 1 — independently reproduced

`tests/bypass-rust/src/founder_authored_guard.rs:101-110` maps
`ConsumeOutcome::Faulted(fault)` through `fail_closed_decision(fault)`.
`tests/bypass-rust/src/founder_fail_closed.rs:6-16` maps every current
`GuardFault` variant to `Ok(GuardDecision::Deny)` with
`RequiredOutcome::FailClosed`. No `Allow` is reachable from that arm.

### Confirmation 2 — resolved from repository history

The full commits are present locally, and the ancestry and Rust-diff checks
pass:

```text
git merge-base --is-ancestor \
  0727e327631b475990ef8d9b7ef3b2c3554050a8 \
  0a54d4995d1b9d98ab8a3ec61861fe2fe7ae29c3
exit_code: 0

git diff --name-only \
  0727e327631b475990ef8d9b7ef3b2c3554050a8 \
  0a54d4995d1b9d98ab8a3ec61861fe2fe7ae29c3 -- '*.rs'
output: <empty>
```

The complete non-Rust diff contains only:

```text
deny.toml
qa/core-002-step5-cross-model-review.md
qa/core-002-step5-review-readiness.md
tests/bypass-rust/Cargo.toml
```

Therefore, the final three analyzer runs cover the exact Rust source reviewed
at the T0 implementation commit. No rerun is required for this confirmation.
The founder may cite this command evidence but still owns the final checklist
statement.

### Confirmation 3 — technical facts verified; founder judgment remains

The recorded final cargo-deny run reports zero advisory errors and
`advisories.ignore = []`; the two `bans.skip` entries do not suppress RustSec
advisories. They are version-pinned duplicate-version policy exceptions with
reasons and dependency-convergence revisit triggers. Whether to approve those
exceptions at the T0 gate remains a founder judgment, and advisory status must
be rechecked whenever the lockfile or advisory database changes.

### Confirmation 4 — founder record still required

The raw results and bundled source support Claude's test-only classification.
The founder must still author the scoped dispositions in the protected review
record. Suggested wording is in
`qa/core-002-step5-founder-review-input.md`; this appendix does not apply it.

### Governance terminology reconciliation

Claude's taxonomy—two first-party code SAST engines plus one SCA engine—is
technically precise. The binding constitution still says “≥3 SAST tools,” and
founder-approved FND-7/DECI-0011 defines cargo-deny as engine three for the
dependency/supply-chain surface. This review does not amend either record.

For gate reporting, prefer **“three-engine SAST/SCA gate: Semgrep, CodeQL, and
cargo-deny”** or name all three engines explicitly. Avoid implying that
cargo-deny is a third first-party code scanner, but do not treat Claude's
terminology preference as overriding the recorded founder decision.

**Founder terminology approval (2026-08-17):** Approved as the canonical
label: **“three-engine SAST/SCA gate: Semgrep, CodeQL, and cargo-deny.”** This
approval resolves the terminology question only. It is not a disposition of
the analyzer findings or cargo-deny exceptions, protected-checklist sign-off,
or independent human T0 review.
