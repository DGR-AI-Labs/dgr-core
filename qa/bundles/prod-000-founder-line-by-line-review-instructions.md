# PROD-000 founder line-by-line review and analyzer-disposition runbook

**Purpose:** founder-operated instructions for completing the remaining PROD-000 governance gate.

**Output:** a founder-authored `qa/prod-000-founder-review.md`, followed by a GitHub approval on
the actual final PR head and a founder-only merge.

This runbook is an index and procedure, not a decision. It does not pre-approve any line, inherit
another reviewer's recommendations, or convert agent-authored/transformed T0 into founder-authored
T0. The founder must personally inspect the named source and evidence, choose every disposition,
and write the rationale in their own words.

## 1. Preconditions and hard stops

Begin only when all of the following are true:

- PR: [DGR-AI-Labs/dgr-core#90](https://github.com/DGR-AI-Labs/dgr-core/pull/90).
- Branch: `codex/prod-000-supervised-agent-t0`.
- The passing cross-model addendum is committed at
  `qa/prod-000-cross-model-review-addendum.md`.
- The independent-human review is committed at
  `qa/prod-000-independent-human-review.md` with a passing verdict.
- The founder can read the canonical ADR-13 and active Amendments A/B in `dgr-internal` directly.
  Do not copy their bodies into this public repository, GitHub comments, or public artifacts.
- The founder has the sanitized R2 package and verifies it before relying on its contents.

Stop and select `CHANGES REQUIRED` or `REJECT` if any of the following occurs:

- a digest, commit-to-tree mapping, or external-authority digest fails;
- the current PR has executable, test, script, workflow, Cargo, lockfile, deny-policy, or substantive
  claim drift after executable commit `587585cf476431f078efe587c5dbcc052389cdad` that is not already
  reviewed and authorized;
- a changed region falls outside Amendment B's bounded classes;
- any analyzer result or diagnostic cannot be inspected or cannot be dispositioned;
- either required GitHub context fails on the head being approved;
- a review record incorrectly claims founder authorship for agent-authored/transformed material; or
- the founder cannot personally support every required attestation.

Do not repair a substantive defect during this review. Record it, stop the gate, and determine
which tests, scans, cross-model review, or independent-human review must be repeated. Any change to
executable code or gate logic invalidates the docs-only final-head sequence below.

## 2. Open the exact review state

Open this project in PyCharm:

```text
C:\Users\Khazret\Documents\DGR-ADR7-AWS-Account-Vending-PlusMore\_work\prod-000-implementation\dgr-core
```

Use PyCharm's embedded terminal from the repository root. Capture the live values at the start of
the founder review; do not copy a SHA from this runbook into the signed record without verifying it:

```bash
git status --short --branch
git rev-parse HEAD
git rev-parse 'HEAD^{tree}'
git log -1 --format=fuller
```

Record those full 40-character values as the **reviewed pre-disposition head** and tree in the
founder form. At the time this runbook was prepared, the completed independent-human record was
committed at:

- human-record commit: `5020a1c7a41a950637f4060e457607fb4075d6cc`;
- human-record tree: `4839a4bcc8c07db8906095a43fc51371df72d63a`;
- human-review file SHA-256:
  `c651f0cab6240c5b3b1e21047fed02cbf3b029e610c97712a82e84b0aed0c00c`.

The live pre-disposition head may be a documentation-only descendant of that commit. Verify the
drift; do not assume it.

The executable evidence is fixed to:

- baseline commit: `e9c8f585809c15d2464b3d45bc2ce26d716c8673`;
- baseline tree: `837bef46997d5c8a9db946d2a87c1d4b6ce5aced`;
- executable commit: `587585cf476431f078efe587c5dbcc052389cdad`;
- executable tree: `89e5de51d23ead98d24bbbe1b4cd57db343b2dc4`;
- complete baseline-to-executable patch SHA-256:
  `c08919d86a1f060cce9a05b3143140a5f011b9349f243475dad4f4ec1b40cf99`;
- safe evidence source: `46da707b8b84dfa599c4e27e5fbb2dc005e9e0e4`;
- safe evidence-source tree: `16b5812cf3bdfce585a9e04da720ab99b0bcac29`.

## 3. Verify the evidence before reading conclusions

Use `qa/bundles/dgr-core-prod-000-public-review-r2-2026-09-02.zip`. Its expected SHA-256 is:

```text
b26825a72ff1603fba263c8355f32429b386ae80a98f7dee1b6648466ce319da
```

After extracting it to a clean directory, work from the package root and verify:

```bash
sha256sum -c MANIFEST.sha256
sha256sum -c metadata/selected-file-inventory.sha256
sha256sum -c metadata/critical-baseline.sha256
sha256sum -c metadata/critical-executable.sha256
sha256sum -c metadata/cross-model-records.sha256
sha256sum -c metadata/canonical-scanner-artifacts.sha256
```

All paths in those six files must resolve and all entries must report `OK`. The package contains an
eight-file baseline subset and a selected review-source set; neither directory is a complete Git
snapshot. Verify full commit-to-tree identities in the public clone:

```bash
git rev-parse 'e9c8f585809c15d2464b3d45bc2ce26d716c8673^{tree}'
git rev-parse '587585cf476431f078efe587c5dbcc052389cdad^{tree}'
git rev-parse '46da707b8b84dfa599c4e27e5fbb2dc005e9e0e4^{tree}'
sha256sum qa/prod-000-review-evidence/full-resubmission.diff
```

From the root of an authorized canonical `dgr-internal` checkout, verify the package's external
authority file:

```bash
sha256sum -c <absolute-R2-package-root>/metadata/external-authorities.sha256
```

Its `specs/adr/...` paths intentionally resolve against `dgr-internal`, not the public package.
Read the authority documents from that checkout after the digests pass.

Finally, inspect post-executable drift through the actual live review head:

```bash
git diff --name-status 587585cf476431f078efe587c5dbcc052389cdad..HEAD
git diff --stat 587585cf476431f078efe587c5dbcc052389cdad..HEAD
git diff 587585cf476431f078efe587c5dbcc052389cdad..HEAD -- . ':!qa/**' ':!specs/**'
```

The last command must not reveal an unreviewed executable or gate-behavior change. Inspect the
documentation/evidence changes too; the exclusions are only a quick drift detector, not the review.

## 4. Read the authority and prior reviews in this order

1. Canonical ADR-13 and active Amendments A/B in `dgr-internal`.
2. `specs/ADR-13-reference-contract.md`, only as the public pointer—not as a substitute for the
   internal authority.
3. `qa/prod-000-t0-decoupling-preparation.md`.
4. `qa/prod-000-cross-model-review.md`, including its original `CHANGES REQUIRED` findings.
5. `qa/prod-000-remediation-validation.md` and `qa/prod-000-post-cross-model-follow-up.md`.
6. `qa/prod-000-cross-model-review-addendum.md`, including its evidence limits.
7. `qa/prod-000-independent-human-review.md`.
8. `T0-AUTHORS.md` and `tests/bypass-rust/T0-BOUNDARY.md`.
9. All five files under `qa/prod-000-authoring/`.

The cross-model and independent-human conclusions are review inputs. Their recommendations do not
become founder dispositions by inheritance. For each item, decide independently whether the
evidence supports `ACCEPT`, `REMEDIATE`, or `REJECT`, and explain why.

## 5. Review the complete patch once

Before reviewing focused hunks, read the whole baseline-to-executable patch:

```text
qa/prod-000-review-evidence/full-resubmission.diff
```

In PyCharm, use **Git | Show History**, open baseline commit `e9c8f585...` and executable commit
`587585cf...`, and compare them. Use the stored patch to bind the reviewed bytes, and the PyCharm
diff for navigation. Confirm the complete change footprint maps to Amendment B's permitted classes:

1. new T0 reached-boundary floor module;
2. module registration;
3. Amendment-A R5.1 timeout constant/mirror/missing-row removal;
4. exactly eight consumer import rewrites;
5. removal of relocated T0 from the mixed T3 module;
6. T3 facade, re-exports, adapter, probe, and conformance mirror assertion; and
7. provenance and review evidence.

Any change to token verification, policy, canonical binding, keys, token lifetime/skew/windows,
SQL, store semantics, nonce handling, deadlines, denial signals, test expectations, Cargo inputs,
lockfiles, deny policy, CI context names, attack classification, or product claims outside those
classes is a hard stop.

## 6. Consequential line-by-line source review

Open each file with line numbers visible. For every range below, compare the baseline and executable
versions where applicable and record either acceptance or a precise concern in the founder form.

### 6.1 New T0 boundary module

File: `tests/bypass-rust/src/founder_before_tool_call_floor.rs`

| Lines | Inspect | Founder must determine |
|---|---|---|
| 1–6 | `{AGENT-AUTHORS}` marker and provenance statement | It truthfully identifies agent authorship/transformation and does not imply founder authorship. The authority-mandated filename does not override the marker. |
| 8–10 | imports | Only the dependencies needed by the relocated types and floor are imported; no T3 dependency crosses into T0. |
| 12–16 | `OpaqueCapabilityToken` | The bytes remain opaque; there is no verification or semantic expansion here. |
| 18–24 | `BeforeToolCallRequest` | All fields and lifetimes preserve the pre-partition public shape. |
| 26–40 | `GuardDecision` | Allow, escalate, and deny fields are unchanged; no outcome, deadline, or signal is synthesized here. |
| 42–48 | `GuardFault` | Typed faults remain faults and cannot become allow decisions. |
| 50–59 | `GuardDecisionPort` | The signature and store mutability remain semantically identical to the source region. |
| 61–75 | `BeforeToolCallOutcome` | Public product-level outcomes preserve the old shape and fields. |
| 77–82 | contract and non-claims | The containment claim is limited to reached-boundary Rust unwinding and expressly excludes abort, termination, OOM, missing-hook, and route-around cases. |
| 83–92 | function signature and generic bound | No new policy, token, store, or adapter behavior appears in the T0 floor. |
| 93–99 | bounded `catch_unwind` | `AssertUnwindSafe` covers only `guard.decide`; after unwind the function does not inspect or reuse either store, and it makes no later-reuse guarantee. |
| 101–108 | deny relay | `outcome` and `denial_signal` are passed through without modification. |
| 109–115 | escalation relay | `review_request_id` and `deadline` are passed through without arithmetic or rewriting. |
| 116–120 | authorization relay | Authorization occurs only in response to `GuardDecision::Allow`, and the reference is relayed unchanged. |
| 121–124 | typed-fault and panic handling | Both paths return `FailClosed` with the established `CORE-003 boundary fail-closed` signal; neither can reach an authorized outcome. |

Compare the relocation against both:

- `qa/prod-000-review-evidence/floor-semantic-identity.diff`; and
- templates 1 and 2 in `qa/prod-000-authoring/`.

The founder must decide whether the documented founder-source provenance, agent transformation,
and non-founder T0-by-consequence classifications are accurate. Do not replace those distinctions
with a blanket “founder-authored” label.

### 6.2 T3 facade and probe boundary

File: `tests/bypass-rust/src/before_tool_call.rs`

| Lines | Inspect | Founder must determine |
|---|---|---|
| 1–5 | module claim | It truthfully describes an isolation-harness adapter, not deployed interception. |
| 11–15 | T0 import/re-exports | Compatibility re-exports expose the intended public surface without redefining T0 behavior. |
| 17–21 | `EffectfulToolProbe` | It is a test probe and does not claim to be a real tool boundary. |
| 23–49 | observation enum | Observations describe harness results; the negative raw-fault sentinel is not reachable from the floor's typed result. |
| 51–58 | adapter contract/non-claims | Fail-closed language is bounded to a reached hook and explicitly excludes important process-level failures. |
| 59–69 | adapter storage/constructor | Only the guard is retained; no hidden effectful capability is introduced. |
| 71–88 | delegation | The adapter calls the T0 floor before matching any result. |
| 89–106 | blocked/escalated arms | Neither arm invokes the probe or issues authorization. |
| 107–116 | authorized arm | `tool.invoke` occurs only after `BeforeToolCallOutcome::Authorized`; the observation then records authorization and the invocation count. |

Explicitly state that this proves ordering in the isolation harness only. It does not prove that a
deployed agent cannot bypass a hook, that every runtime tool route is intercepted, or that the hook
is installed.

### 6.3 Module registration

File: `tests/bypass-rust/src/lib.rs`, line 13.

Confirm the new module is registered once at the expected crate level, with no unrelated module,
feature, or public export change. Cross-check template 4.

### 6.4 Amendment-A R5.1 timeout change

File: `tests/bypass-rust/src/founder_approval_timeout.rs`

First inspect the small semantic hunk:

```text
qa/prod-000-review-evidence/r5-1-timeout-semantic.diff
```

Then separately inspect the raw line-ending diff:

```text
qa/prod-000-review-evidence/r5-1-timeout.diff
```

| Lines | Inspect | Founder must determine |
|---|---|---|
| 10 | import rewrite | The type path moves to the T0 module and makes no semantic change. |
| 13 | authoritative timeout constant | `EscalateThenDenyOnTimeout` is now owned in T0 rather than fetched from the T3 registry. |
| 15–16 | hidden conformance mirror | This is a mirror for T3 equality checking, not a second authority. |
| 34–45 | pending arm | Request identity and deadline behavior are unchanged. |
| 46–55 | timed-out arm | The removed registry lookup and impossible `None => fail_closed` branch are replaced only by the authoritative constant; denial signal remains unchanged. |
| 56–60 | missing/faulted arms | Existing fail-closed behavior is unchanged. |

The raw diff reflects file-wide CRLF-to-LF normalization. Accept or reject that mechanical change
separately from the semantic R5.1 change. Cross-check templates 2 and 5. Do not treat a whole-file
raw replacement as proof that every line changed semantically.

### 6.5 Exactly eight founder-owned consumer imports

Inspect `qa/prod-000-review-evidence/eight-consumer.diff` and each destination file directly:

| File | New import line | Expected change class |
|---|---:|---|
| `tests/bypass-rust/src/founder_consumption_store.rs` | 6 | import only |
| `tests/bypass-rust/src/founder_approval_timeout.rs` | 10 | import plus separately reviewed R5.1 and EOL normalization |
| `tests/bypass-rust/src/founder_authored_guard.rs` | 7 | import only |
| `tests/bypass-rust/src/founder_approval_store.rs` | 6 | import only |
| `tests/bypass-rust/src/founder_s2_consumption_store.rs` | 6 | import only |
| `tests/bypass-rust/src/founder_s2_approval_store.rs` | 9 | import only |
| `tests/bypass-rust/src/founder_token_verification.rs` | 13 | import only |
| `tests/bypass-rust/src/founder_fail_closed.rs` | 4 | import only |

Confirm that the focused diff has exactly eight file headers and one hunk per file; seven files have
only the path rewrite. For every file, compare the before/after SHA-256 values in template 5 with
the actual bytes. The filename prefix does not establish authorship; preserve each file's recorded
authorship marker and provenance.

### 6.6 ATK-06 mirror assertion

File: `tests/bypass-rust/tests/core_004_conformance.rs`

- lines 14–16: widened import includes the hidden T0 mirror;
- lines 162–172: named active test reads the registry result and compares the T0 mirror with it;
- lines 206–212: observed timeout decision is compared with that same expected terminal outcome.

Read the assertion body itself. The CI guard proves that the named test remains listed and not
ignored; it does **not** prove that lines 168–172 continue to contain the equality assertion.
Record that limitation under N14.

### 6.7 T3 active-test and ignored-set guard

Files:

- `scripts/check-ignored-attacks.mjs`: lines 10–22, 43–50, 53–85, and 111–139;
- `scripts/check-ignored-attacks.test.mjs`: lines 49–71.

Confirm the expected ignored set remains exactly ATK-04, ATK-05, ATK-12, ATK-14, and ATK-15.
Confirm deletion, rename, or `#[ignore]` of the named ATK-06 test causes the blocking guard to fail.
Confirm `.github/workflows/ci.yml` invokes both the script test and live enumeration under
`Rust format / build / test`. Do not expand that conclusion into JavaScript SAST coverage or
assertion-body integrity.

## 7. Provenance-template dispositions

Review these five inputs in order:

1. `qa/prod-000-authoring/01-founder-boundary-module.md`;
2. `qa/prod-000-authoring/02-floor-relocation-semantic-identity.md`;
3. `qa/prod-000-authoring/03-old-floor-removal-and-handoff.md`;
4. `qa/prod-000-authoring/04-module-registration-and-dependency-boundary.md`;
5. `qa/prod-000-authoring/05-founder-import-rewrite-ledger.md`.

For each template, record in `qa/prod-000-founder-review.md`:

- `APPROVE`, `CORRECT`, or `REJECT`;
- whether all cited lines and hashes resolve;
- whether the authorship class is truthful;
- whether the change falls within Amendment B;
- any correction, with exact file and line; and
- the founder's rationale.

Do not overwrite the five agent-prepared templates or fill their `PENDING` fields in place. They are
provenance inputs showing what the agent presented for review. Put the founder's final decisions in
the consolidated founder review record so the original inputs remain inspectable.

Template-specific questions:

- **Template 1:** Are the five relocated public shapes accurately classified as pre-existing
  non-founder T0-by-consequence and agent-relocated/transformed? Does the module marker prevent the
  mandated `founder_` filename from being read as an authorship claim?
- **Template 2:** Do the baseline and destination region hashes reproduce? Is the reached-boundary
  floor semantically preserved, with its panic/store non-claims intact?
- **Template 3:** Was all relocated T0 behavior removed from the mixed T3 file exactly once, leaving
  only facade, observation, adapter, and probe responsibilities?
- **Template 4:** Is the dependency direction T3 to T0, without T0 importing T3? Is registration
  limited to the one expected module line?
- **Template 5:** Do all sixteen before/after hashes reproduce? Are exactly seven consumers
  import-only, with `founder_approval_timeout.rs` separately and truthfully classified? Confirm its
  corrected import line is 10, not 9.

## 8. Analyzer evidence and required founder dispositions

Use the canonical machine-readable artifact where one exists, then the wrapper for invocation and
coverage provenance. A scanner exit code is evidence, not a disposition. “Unchanged region” is
relevant scope context but is not, by itself, an adequate risk rationale.

For every subsection below, record exactly one decision: `ACCEPT`, `REMEDIATE`, or `REJECT`.

- `ACCEPT`: the evidence is understood and the bounded residual risk is compatible with this gate.
- `REMEDIATE`: do not approve or merge; state the required change and which gates must repeat.
- `REJECT`: the evidence or architecture is unacceptable; do not approve or merge.

### 8.1 Semgrep finding and coverage bound

Artifacts:

- canonical: `qa/sast/prod-000-final-input-semgrep-2026-09-02.json`;
- wrapper: `qa/sast/prod-000-final-input-semgrep-2026-09-02.txt`;
- raw console: `qa/sast/prod-000-final-input-semgrep-2026-09-02.raw.txt`.

Verified evidence to inspect:

- Semgrep `1.173.0`, `p/rust`, 11 rules;
- 21 of 21 Rust files scanned;
- zero JavaScript files scanned;
- one INFO result, with non-zero `--error` exit:
  `rust.lang.security.temp-dir.temp-dir` at
  `tests/bypass-rust/tests/consumption_store.rs:19`.

Read `consumption_store.rs:15–22`. It builds a test database path under
`std::env::temp_dir()` using timestamp nanoseconds and process ID. The founder's rationale must
address:

- test-only scope and whether production code can reach it;
- predictable-path, collision, symlink, race, or cross-user risks in the actual test environment;
- whether existing isolation and cleanup are sufficient for this gate;
- whether changing it would be within Amendment B or would require reopening scope; and
- N5/N8: the 11-rule Rust leg and absence of JavaScript SAST coverage.

Record separate statements for the single finding and the coverage limitation even if both receive
the same overall decision.

### 8.2 CodeQL results

Artifacts:

- canonical: `qa/sast/prod-000-final-input-codeql-2026-09-02.sarif`;
- wrapper: `qa/sast/prod-000-final-input-codeql-2026-09-02.txt`.

Verified evidence to inspect:

- CodeQL CLI `2.25.5`, Rust queries `0.1.35`;
- 21 of 21 Rust files extracted;
- nine hard-coded cryptographic-value results:
  - `tests/bypass-rust/tests/val_002_fixtures.rs`: lines 126, 152, 169, 186, 204, 222, 339;
  - `tests/bypass-rust/tests/val_004_fixtures.rs`: lines 136 and 143.

For each location, inspect the value, its use, and whether it is deterministic test-fixture material
or a production secret. Then disposition the nine results as a group only if the same rationale
actually applies to all nine. The rationale must address:

- why deterministic fixture nonces are or are not acceptable;
- whether any value grants authority outside the isolated tests;
- whether reuse or public visibility changes the tested security property;
- why unchanged baseline provenance affects scope but does not erase the reported risk; and
- whether remediation would alter fixtures or expected behavior outside PROD-000's authority.

### 8.3 CodeQL diagnostics and evidence binding

Inspect the complete SARIF `invocations[0].toolExecutionNotifications` array, not just its nine
results. The recorded count is 99 entries: 67 `note`, 32 without a level, and zero warning/error
notifications. Confirm those counts from the canonical file and inspect the notification messages.

Record a decision and rationale for:

1. the complete 99-entry diagnostic array; and
2. N6, the SARIF self-binding limitation.

The SARIF has no sufficient self-contained commit/tree binding. The wrapper and reconstructed source
tree provide external binding to executable tree `89e5de51...`. State whether that external chain is
adequate and what it does **not** prove. Do not describe the SARIF as self-authenticating.

### 8.4 cargo-deny results

Artifact: `qa/sast/prod-000-final-input-cargo-deny-2026-09-02.txt`.

Inspect the complete output and confirm:

- exit 0;
- advisories: 0 errors, 0 warnings, 0 notes;
- bans: 0 errors, 0 warnings, 2 notes;
- licenses: 0 errors, 0 warnings, 54 notes;
- sources: 0 errors, 0 warnings, 0 notes.

The founder's rationale must state whether the bans notes and accepted-license notes are compatible
with the repository's dependency policy, whether any dependency input changed in PROD-000, and why
the evidence is adequate. Do not use “exit 0” as the complete rationale.

## 9. Residual cross-model findings

Record an explicit founder disposition and rationale for each item:

| ID | Required founder question |
|---|---|
| N5 | Is the 11-rule `p/rust` Semgrep leg adequate for this bounded gate, with repository-specific boundary rules deferred rather than silently claimed? |
| N6 | Is external commit/tree reconstruction an adequate binding for SARIF that is not self-binding? |
| N7 | After reading all 99 notifications, is the diagnostic array acceptable rather than merely summarized? |
| N8 | Is the lack of JavaScript SAST acceptable given syntax checks, unit tests, live enumeration, and the explicit non-claim? |
| N13 | Does the stored EOL-insensitive hunk accurately expose the substantive R5.1 change while the raw EOL diff preserves the mechanical normalization record? |
| N14 | Is the bounded claim acceptable when the guard protects only the test's name and active state, not its equality-assertion body? |
| N15 | Is the corrected template-5 line reference (`founder_approval_timeout.rs:10`) accurate and fully resolved? |

Also read every independent-human finding and recommendation. State whether the founder accepts,
modifies, or rejects each recommendation, with a reason. In particular, independently decide the
temp-directory finding, deterministic fixture nonces, `founder_` filename classification, ATK-06
canonical direction, and whether missing-input concerns are now closed.

## 10. Scope and non-claim confirmation

Before approving, the founder must affirm that the evidence supports only the bounded partition and
preserved isolation-harness behavior. It does not establish:

- deployed runtime interception;
- agent non-bypassability or complete route-around coverage;
- hook installation or missing-hook resistance;
- operator-proofing or out-of-process enforcement;
- containment of `panic=abort`, process termination, or OOM abort;
- safe post-panic store reuse;
- real human notification, delivery, waiting, or decision infrastructure;
- cross-instance persistence or synchronization; or
- authorization of PROD-001 before PR #90 is merged.

Confirm also that founder supervision and approval do not convert agent-authored or
agent-transformed lines into founder-authored lines.

## 11. Create the founder record

1. Copy `qa/prod-000-founder-review-input.md` to `qa/prod-000-founder-review.md`.
2. Do not overwrite the input template.
3. The founder personally completes every blank, checkbox, template disposition, analyzer
   disposition, finding rationale, scope confirmation, decision, and attestation.
4. Use full 40-character commit and tree SHAs.
5. Name the reviewed pre-disposition head captured in section 2—not the executable commit unless
   they are actually identical.
6. Select exactly one overall decision:
   `APPROVE REVIEWED HEAD`,
   `APPROVE REVIEWED HEAD WITH RECORDED NON-BLOCKING FINDINGS`,
   `CHANGES REQUIRED`, or `REJECT`.
7. Supply a stable founder-controlled signature or identity reference and UTC timestamp.

An agent may mechanically save, commit, and push the founder-supplied record unchanged after the
founder supplies it. The agent must not invent, complete, paraphrase, or strengthen any founder
decision or rationale.

## 12. Bind approval to the actual final head

The founder review file names the pre-disposition head because committing the review file creates a
new commit; a file cannot contain the SHA of its own commit. Complete the binding as follows:

1. Commit and push only the completed founder record and any strictly mechanical review metadata.
2. Capture the resulting full final PR head SHA and tree SHA.
3. Compare final head with the founder-reviewed pre-disposition head. Only the founder review record
   and explicitly inspected mechanical review metadata may differ.
4. Wait for these exact required contexts on that final head:
   - `Structural / governance check`;
   - `Rust format / build / test`.
5. Inspect all informational analyzer statuses and ensure they match the recorded dispositions.
6. The founder submits a GitHub **Approve** review. GitHub must show that approval on the exact final
   head from step 2.
7. Do not push another commit after approval. If the head moves for any reason, repeat the drift
   inspection, required checks, and founder GitHub approval on the new head.
8. Only the founder merges PR #90.
9. Verify the merge commit is reachable from the protected default branch. Only then may PROD-001
   be considered for authorization.

The public-history purge request for the previously exposed sensitive bundle should also be tracked
to completion with GitHub Support. Treat it as a separate security obligation; do not confuse a
rewritten PR branch with deletion of unreachable objects from GitHub's backend.
