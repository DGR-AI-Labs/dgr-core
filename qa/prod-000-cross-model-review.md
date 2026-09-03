# PROD-000 non-author cross-model review

**Destination path:** `qa/prod-000-cross-model-review.md`
**Gate:** ADR-13 Amendment B → non-author cross-model review (one of four open review gates)
**Verdict:** `CHANGES REQUIRED`

---

## 1. Reviewer and review binding

| Field | Entry |
|---|---|
| Reviewer product | Anthropic Claude (claude.ai chat surface, with container tooling used for digest and line-range verification) |
| Reviewer model | Claude Opus 5 |
| Non-author attestation | The implementation at `40b713039a5612831df415cdd785271a7342be74` was authored by OpenAI Codex. This reviewer is a different vendor and model family and authored none of the reviewed source, templates, or evidence records. |
| UTC review time | `2026-09-02T16:19:30Z` (container clock at start of inspection) |
| Base commit | `e9c8f585809c15d2464b3d45bc2ce26d716c8673` — as quoted by template 1; **not independently verified** (no repository access) |
| Agent implementation commit | `40b713039a5612831df415cdd785271a7342be74` — as quoted by templates 1–5; **not independently verified** |
| Scanned validation commit | `425d7718ecf83086776de8fc09caec26c728df92` — as quoted by `prod-000-exact-commit-validation.md`; **not independently verified** |
| Reviewed commit / PR #90 head SHA | **NOT SUPPLIED** — see finding B2 |
| Patch digest | **NOT SUPPLIED** — see finding B1 |
| Repository / diff access | **NONE.** This review had no git access, no `git diff`, and no PR #90 metadata. It inspected only the artifact set in §2. |

### Scope of source actually inspected

**2 of the 21 tracked Rust files were read at source.** No diff was available. Findings therefore split into
*verified*, *not established*, and *blocking-input* categories, and question answers use
`YES` / `NO` / `NOT ESTABLISHED` per atomic question.

**Asymmetry clause.** Absence of a defect inside the verified scope is not evidence of correctness
outside it. Nothing in §5 should be read as a positive assurance about the 19 tracked Rust files,
the eight founder-owned consumers, the conformance suite, or `founder_approval_timeout.rs`.

---

## 2. Files and evidence inspected, with digests

Digests computed by this reviewer over the exact bytes supplied.

| Artifact | SHA-256 |
|---|---|
| `T0-AUTHORS.md` | `8f25ea8ca0a08478871bc9c1d8c3691f9328e0c63ed86cd1cad0412d2296edf3` |
| `tests/bypass-rust/T0-BOUNDARY.md` | `7b4d0fe9e23e61a7f4909fc0cb28bf71d7a6f572fa542f26c5074bc55b3a84f8` |
| `qa/prod-000-authoring/01-founder-boundary-module.md` | `c6a5caaa8b4df7b4f1e0162c9d17424398dcdf05731336713d21c60a9fe9ea55` |
| `qa/prod-000-authoring/02-floor-relocation-semantic-identity.md` | `c207910918519bd23570cb0afe2936dbe5eb133777556fd8f13608f154d90446` |
| `qa/prod-000-authoring/03-old-floor-removal-and-handoff.md` | `ca9ca8d9957c410ace79182ada1b84de1d6efadc62ffeb3f59fb648c6ad367fd` |
| `qa/prod-000-authoring/04-module-registration-and-dependency-boundary.md` | `94ff1913b787b86674c361ad0dd9baf2e653cc32c3ddc7e0b2147b8fddc1722c` |
| `qa/prod-000-authoring/05-founder-import-rewrite-ledger.md` | `b22b11ac0c93ea2eb7b62aff1d9104a713fe6a1da82e98df3647ff391dedc505` |
| `qa/prod-000-exact-commit-validation.md` | `305588f081786e8b7700c6dd101ca7867f023ada2a9dc634b3e1fbcd44b443c9` |
| `qa/sast/prod-000-semgrep-2026-09-01.txt` | `8b7eb9e3b62066c8dfe6d82bde0fec634ca5577f3a26657fd4ba6ba4d318fcbe` |
| `qa/sast/prod-000-semgrep-2026-09-01.json` | `cc84acaf9645f48605fd84ad48558930f9276082d72649c106260ecaf44950c7` |
| `qa/sast/prod-000-codeql-2026-09-01.txt` | `3c6b73865eee28839dea7ac3e7681a767c2365665e36dc087b2e8d21ed2e9c20` |
| `qa/sast/prod-000-codeql-2026-09-01.sarif` | `4c22554bbda1637966303d722bfc450aa651f1ded79eaec7539f2161797c76b7` |
| `qa/sast/prod-000-cargo-deny-2026-09-01.txt` | `13acf718133ce67aa84c0780632fe0f444eee6f20063256012485a272ad7ef8a` |
| `tests/bypass-rust/src/founder_before_tool_call_floor.rs` | `edc342844cabc11ba399bb80c20e59b785cd7ce6909895c059ac83148cd1d122` |
| `tests/bypass-rust/src/before_tool_call.rs` | `70baf99e181b61bb3a4c5049e86351855871cf7757f8e5bb74fcd3f5ca60255b` |

**Content binding — positive.** The supplied `founder_before_tool_call_floor.rs` is byte-identical to the
digest recorded in template 1 (`edc3428…`), and `sed -n '77,126p' | sha256sum` reproduces template 2's
destination-region digest `8509f437a338edfa215f82f78aaa7656ec146dba4b7167eb941e573ef54f8720` exactly. The
reviewed T0 module is therefore the same artifact the authoring templates describe.

**Not supplied** (each drives a blocking finding): PR #90 metadata and head SHA; patch digest; any `git diff`;
baseline `before_tool_call.rs` at `e9c8f58…`; `founder_approval_timeout.rs` (before or after);
the other seven founder-owned consumers; `tests/attack_set.rs`; `tests/core_004_conformance.rs`;
`src/lib.rs`; `specs/CORE-001-bypass-attack-set.md`; the ADR-13 Amendment A/B text.

**Manifest note.** The cargo-deny artifact was supplied twice, byte-identical. One run, not two.

---

## 3. Verdict

`CHANGES REQUIRED`

**Basis.** The T0 module itself reviews clean at source on every question answerable from it (Q2, Q3
destination behavior, Q4, Q5 within the crate, Q9 in-source). The verdict is driven not by a defect in
the floor but by four classes of defect in the evidence chain and governance record:

1. The review inputs required by this gate were not supplied, leaving Q1, Q6, Q7, and Q8 unverifiable (B1, B7, B8).
2. All functional and SAST evidence is bound to `425d7718…`, not to the PR #90 head, and the head SHA is
   absent from every artifact (B2).
3. The new agent-authored T0 module is named `founder_*`, which corrupts the very pathspec template 5
   nominates as its import-only diff evidence (B3).
4. Four governance-record contradictions must be dispositioned before founder line-by-line review, two of
   which change what a reviewer is supposed to be checking (B4, B5, B6, B9).

Under the repository's own fail-closed principle, an ambiguous gate result does not advance. This review does
not assert that PROD-000 is defective; it asserts that the cross-model gate cannot be certified on the
supplied inputs.

---

## 4. Per-question findings

| # | Question | Answer |
|:--:|---|---|
| 1 | Every changed region truthfully classified | **NOT ESTABLISHED** |
| 2 | T0 floor free of probes / observations / counters / registry / real tool invocation | **YES** |
| 3 | `before_tool_call_floor` preserves the five behaviors | **YES** as to destination behavior; **NOT ESTABLISHED** as to identity with founder source |
| 4 | Probe invoked only after `Authorized` | **YES** |
| 5 | Exactly one active floor, none in T3 | **YES** within the Rust crate; **NOT ESTABLISHED** repo-wide |
| 6 | R5.1 change correct | **NOT ESTABLISHED** |
| 7 | Seven founder files import-only; `founder_approval_timeout.rs` R5.1-only | **NOT ESTABLISHED** |
| 8 | Tests or ignored-attack expectations weakened | **NOT ESTABLISHED** (asserted, not evidenced) |
| 9 | Stated non-claims intact | **YES**, with N3 |
| 10 | Raw SAST/SCA records support their summaries | **YES**, with N6–N8 |

### Q1 — classification truthfulness — NOT ESTABLISHED

Verifiable within the two supplied files, the stated classifications are consistent with the source: the T0
module's content matches "agent-authored T0 with relocated shapes," and the T3 file's content matches
"compatibility re-exports; outcome conversion and probe invocation." Every line reference in templates 1–4
resolves correctly against the actual file (see the line-range audit below) — the ledgers are precise, which
is the main reason the remaining questions are answerable at all.

Line-range audit, `founder_before_tool_call_floor.rs` (126 lines):

| Claim (template) | Claimed | Actual | Result |
|---|---|---|---|
| `OpaqueCapabilityToken` (T1) | 12–16 | 12–16 | match |
| `BeforeToolCallRequest` (T1) | 18–24 | 18–24 | match |
| `GuardDecision` (T1) | 26–40 | 26–40 | match |
| `GuardFault` (T1) | 42–48 | 42–48 | match |
| `GuardDecisionPort` (T1) | 50–59 | 50–59 | match |
| `BeforeToolCallOutcome` (T3) | 61–75 | 61–75 | match |
| `Blocked` variant (T1) | 63–67 | 64–67 (63 is the enum header) | match, see N10 |
| `Escalated` / `Authorized` variants (T1) | 68–71 / 72–74 | 68–71 / 72–74 | match |
| Floor function (T2) | 77–126 | 77–126 | match |
| `AssertUnwindSafe` scope / `catch_unwind` (T2) | 93–99 / 97–99 | 93–99 / 97–99 | match |
| Deny / escalation / allow relay arms (T2) | 101–108 / 109–115 / 116–120 | same | match |
| Fault-unwind outcome and signal (T2) | 121–123 / 123 | same | match |
| Store/domain-type dependency points (T4) | 8–10, 55–57, 86–88 | same | match |

Line-range audit, `before_tool_call.rs` (117 lines): T3 re-export at 11–14 (claimed 11–14, match); floor
delegation at 80–115 (claimed 80–115, match); probe-invocation region at 105–114 (claimed 105–114, match).

**Why still NOT ESTABLISHED.** (a) No diff exists in the evidence set, so "every changed region" cannot be
enumerated by this reviewer — only the two supplied end-state files can be read. (b) Finding B9 identifies a
direct tension in the classification of the five relocated shapes that requires founder disposition before
the classification can be called truthful.

### Q2 — T0 module isolation — YES

Mechanical checks over `founder_before_tool_call_floor.rs`:

| Symbol / pattern | Occurrences in T0 module |
|---|---|
| `EffectfulToolProbe` | 0 |
| `RecordingToolProbe` | 0 |
| `BeforeToolCallObservation` | 0 |
| `authorization_issued` | 0 |
| `effectful_invocations` | 0 |
| `attack_by_id` / `ATTACK_SET` | 0 / 0 |
| `fixtures` | 0 |
| Real or probe tool invocation | 0 |

Imports are exactly three (lines 8–10): `founder_approval_store::{ApprovalStore, ReviewRequestId}`,
`founder_consumption_store::ConsumptionStore`, and `crate::{DecisionContext, ProposedAction,
RequiredOutcome}`. Dependency direction is inward to founder stores and crate-root domain types only, with
zero references to the harness facade, fixtures, probes, observations, or the attack registry. This matches
template 4's dependency table exactly. `Authorized` carries only an authorization reference and makes no
execution claim.

### Q3 — floor behavior — YES (destination) / NOT ESTABLISHED (identity with founder source)

Verified at source:

- **Bounded `catch_unwind`.** Exactly one occurrence (line 97). `AssertUnwindSafe` wraps a closure whose only
  statement is `guard.decide(...)` (98). Nothing else is inside the unwind boundary.
- **Store handling after unwind.** No match arm reads either store; on `Err(_)` the function returns
  immediately (121–124). The inline comment (93–96) correctly declines to certify store invariants for reuse.
- **Typed fault and panic both fail closed.** The combined arm `Ok(Err(_)) | Err(_)` yields
  `Blocked { outcome: RequiredOutcome::FailClosed, denial_signal: "CORE-003 boundary fail-closed" }` — the
  signal string is byte-identical to the one recorded in T0-BOUNDARY and appears exactly once in the module.
- **Denial relay.** `GuardDecision::Deny { outcome, denial_signal }` → `Blocked { outcome, denial_signal }`;
  both fields are moved through unmodified, no substitution or defaulting (102–108).
- **Escalation relay.** `Escalate { review_request_id, deadline }` → `Escalated { review_request_id,
  deadline }` unmodified (109–115). No arithmetic on `deadline` anywhere in the module, consistent with the
  standing "deadline computed once, never extended" rule.
- **Authorization relay.** `Allow { authorization_reference }` → `Authorized { authorization_reference }`
  unmodified (116–120).
- **Ordering.** No policy, threshold, canonicalization, or consumption logic exists in the module; it is a
  pure containment-and-conversion layer, so it cannot perturb the Addendum-A token-bearing order.

**Why identity is NOT ESTABLISHED.** Template 2's ledger asserts semantic identity against baseline regions
`before_tool_call.rs:115-171` (`35639394…`) and `:126-170` (`d9f2b23f…`), and against the baseline complete
file (`5e44f9d6…`). None of those bytes were supplied. This reviewer can confirm the destination behavior and
the internal consistency of the ledger, but cannot confirm that it matches the founder source it claims to
preserve — which is precisely the dependency this gate exists to remove (B8).

### Q4 — probe invocation ordering — YES

`before_tool_call.rs` contains exactly two occurrences of `invoke(`: the trait declaration (line 18) and one
call site (line 108), which sits inside the `BeforeToolCallOutcome::Authorized` arm and executes before
constructing `Proceeded`. The `Blocked` (87–95) and `Escalated` (96–104) arms call only
`tool.invocation_count()`, a read, and hard-code `authorization_issued: false`. There is no fallthrough,
default, or catch-all arm that could reach `invoke`.

### Q5 — single active floor — YES within the crate, NOT ESTABLISHED repo-wide

`catch_unwind`: 1 in the T0 module, 0 in the T3 adapter. `"CORE-003 boundary fail-closed"`: 1 in T0, 0 in T3.
The T3 adapter's only path to an outcome is delegation to `before_tool_call_floor` (80–86). No duplicate
floor, no `todo!()`, no permissive fallback, no compatibility shim in the supplied files.

Repo-wide is not established for two reasons: 19 tracked Rust files were not inspected, and the CodeQL SARIF
artifact table reveals a separate JavaScript enforcement surface in the same repository (`src/gate.mjs`,
`tests/bypass/05-gate-throws.test.mjs`) that no supplied artifact classifies — see N9.

### Q6 — R5.1 correctness — NOT ESTABLISHED

None of the four sub-claims can be checked. `founder_approval_timeout.rs` was not supplied in either
revision; neither was the conformance test holding the registry-mirror equality assertion, nor
`specs/CORE-001-bypass-attack-set.md`. The only evidence is template 5's before/after digests plus prose
attestations in T0-AUTHORS and T0-BOUNDARY. Two substantive concerns attach and are recorded as B5 and B6.

### Q7 — import-only changes in the founder-owned consumers — NOT ESTABLISHED

Template 5's internal arithmetic is coherent: eight rows, seven flagged `Non-import diff? no`, and
`founder_approval_timeout.rs` flagged `yes — R5.1 only`, with the scope rule explicitly declining to certify
that file as import-only. That is the correct structure. But the diff itself was not supplied, the nominated
evidence artifact is a command rather than a stored object (N11), the pathspec that command uses is defective
(B3), and T0-AUTHORS contradicts the ledger on whether all eight are module-path-only (B4).

### Q8 — test and ignored-set integrity — NOT ESTABLISHED (asserted, not evidenced)

`prod-000-exact-commit-validation.md` reports `cargo test` at 52 passed / 5 ignored, `check-ignored-attacks.mjs`
PASS on the exact `ATK-04/05/12/14/15` set, `check-ignored-attacks.test.mjs` 4 of 4, and that the CI workflow,
`Cargo.toml`, `Cargo.lock`, `deny.toml`, and `attack_set.rs` "remained byte-identical to the recorded baseline
hashes." Five ignored tests is internally consistent with the five-member deferred set.

The claim is not evidenced within the supplied set: the record does not print the recorded baseline hashes or
the observed hashes it compared them to, and `attack_set.rs` was not supplied. A byte-identity assertion
without both digests is a re-run instruction, not evidence. Compounding this, the run is bound to
`425d7718…` rather than the reviewed head (B2). Note also the interaction with B5: if the registry-mirror
equality assertion was *added* by PROD-000 while `attack_set.rs` is byte-identical, the added assertion lives
in another test file whose diff was never produced.

### Q9 — non-claims intact — YES, with N3

`founder_before_tool_call_floor.rs:80-82` and `before_tool_call.rs:54-56` both carry the four exclusions:
`panic=abort`, process termination, OOM abort, and a hook that is never invoked. The T0 module additionally
declines to certify store invariants for later reuse (95–96). T0-BOUNDARY continues to reserve route-around,
missing plugin, and operator bypass to RUNTIME-003/004, retains the bounded CORE-004 claim (no real human
delivery, no cross-restart waiting, no cross-instance pending state, no live non-bypassability), and states
the isolation harness "must never become an alternate production enforcement path." Nothing in the supplied
source or documents upgrades any of these claims. `Escalated` is not an authorization and cannot reach the
probe — confirmed at source, not merely asserted.

N3 records the one omission: the in-source non-claim list drops "a route around the hook," which matters
because this module is the PROD-001 extraction candidate.

### Q10 — raw records support their summaries — YES, with N6–N8

Independently re-derived from the raw artifacts:

| Check | Summary claim | Raw artifact | Result |
|---|---|---|---|
| Semgrep version | 1.173.0 | JSON `version` = 1.173.0 | match |
| Semgrep findings | 1 | `results` length 1 | match |
| Semgrep finding identity | `rust.lang.security.temp-dir.temp-dir`, `tests/consumption_store.rs:19` | identical rule / path / line | match |
| Semgrep errors | 0 | `errors` = `[]` | match |
| Semgrep coverage | 21 tracked files | `paths.scanned` length 21 | match |
| Semgrep severity honesty | "INFO in machine-readable output; blocking under `--error`" | JSON `severity` = INFO | accurate, and correctly disclosed rather than reported as PASS |
| CodeQL version / pack | 2.25.5 / `rust-queries@0.1.35` | driver "CodeQL 2.25.5"; extension `codeql/rust-queries 0.1.35+b551e89…` | match |
| CodeQL findings | 9 | 9 results, all `rust/hard-coded-cryptographic-value` | match |
| CodeQL finding locations | 7 in `val_002_fixtures.rs` (126/152/169/186/204/222/339), 2 in `val_004_fixtures.rs` (136/143) | identical | match |
| CodeQL coverage | 21/21 tracked | 22 `.rs` artifacts = 21 tracked + generated `bindgen.rs` | match |
| CodeQL execution | 0 execution errors | `executionSuccessful: true`; 0 `toolConfigurationNotifications`; no error-level notifications | match |
| Generated-file warning | 1, `libsqlite3-sys` `bindgen.rs` | 1 extraction warning, attributed to that path | match |
| Engine coverage agreement | — | Semgrep's 21-file set and CodeQL's 21 tracked `.rs` set are **identical** (set difference empty both ways) | corroborated |
| cargo-deny | 0.20.2, exit 0, 2 bans notes, 54 license notes, 0 errors/warnings across advisories, bans, licenses, sources | raw output identical | match |
| Correct engine taxonomy | cargo-deny labelled SCA, not code-SAST | consistent with the FND-7 amendment distinction | correct |

No misreported count, path, or line was found in any of the three summaries, and no finding was suppressed or
self-dispositioned by the authoring agent. The three qualifications are N6 (the SARIF does not self-bind to a
commit), N7 (the diagnostic array is far larger than the finding list), and N8 (the JavaScript surface has no
coverage from any engine).

---

## 5. Findings

### Blocking

**B1 — Required review inputs were not supplied; four questions are unverifiable.**
No PR #90 metadata, no head SHA, no patch digest, and no diff of any kind. Q1 (in full), Q6, Q7, and Q8
cannot be answered. A cross-model gate whose reviewer inspected 2 of 21 tracked Rust files and zero diff
hunks cannot certify "every changed region." Re-submission manifest in §6.

**B2 — Evidence is bound to `425d7718…`, not to the reviewed head; the head SHA appears nowhere.**
Templates 1, 3, and 5 all defer to "final PR head," and `prod-000-exact-commit-validation.md` states the
scanned commit is a documentation-only descendant of the implementation commit, with the evidence commit a
further non-Rust descendant. That chain is asserted in prose only. Under the "evidence bound to exact
commits" discipline, produce: the PR #90 head SHA; `git diff --name-only 425d7718..<head>` demonstrating zero
changes to `*.rs`, `Cargo.toml`, `Cargo.lock`, `deny.toml`, `rust-toolchain.toml`, `.github/workflows/**`,
and `scripts/**`; and a digest over that output. Absent this, every PASS in the validation table is a PASS
for a commit nobody is being asked to merge.

**B3 — The new agent-authored T0 module is named `founder_*`, and this corrupts template 5's own evidence artifact.**
`founder_before_tool_call_floor.rs` is agent-authored T0 (T0-AUTHORS says so explicitly), yet it adopts the
`founder_*` prefix that every other file in that directory uses to denote founder authorship. This is the
exact hazard T0-AUTHORS warns against — "must never be presented as founder authorship of agent-written or
agent-transformed lines" — encoded into the filename, where it will survive every future grep, review, and
extraction. Concretely: template 5 nominates
`git diff e9c8f58...40b7130 -- tests/bypass-rust/src/founder_*.rs` as the import-only diff artifact. That
pathspec now matches the new 126-line agent-authored module, so the artifact attested to contain only
import-path changes will contain a whole new T0 file. Remedy: rename the module (e.g.
`t0_before_tool_call_floor.rs` or `boundary_floor.rs`), or keep the name and (a) record the naming exception
explicitly in T0-AUTHORS and (b) correct the pathspec to exclude it, storing the resulting diff as a
digest-bound artifact.

**B4 — T0-AUTHORS contradicts template 5 on the eight consumers.**
T0-AUTHORS' Amendment-B scope bullet reads "module-path-only rewrites in the eight founder-owned consumers."
Template 5's scope rule and T0-AUTHORS' own later paragraph correctly state seven import-only files plus
`founder_approval_timeout.rs` carrying the separately classified R5.1 change. Correct the scope bullet before
founder line-by-line review; a founder reading the bullet would not know to look for a non-import change.

**B5 — Unresolved contradiction: was the registry-mirror equality assertion added by PROD-000 or pre-existing?**
T0-BOUNDARY says PROD-000 "adds a T3 equality assertion." T0-AUTHORS lists "the registry-mirror assertion"
among the T3 regions classified by this change. The review question as posed says "the *existing* conformance
test verifies equality." Meanwhile the validation record says `attack_set.rs` is byte-identical to baseline.
These cannot all be true as written. If the assertion was added, it lives in another test file
(`core_004_conformance.rs` is the likely home) whose diff was never produced or classified. Resolve, and
supply the file.

**B6 — R5.1 reverses the canonical-ownership direction for an expected outcome, and no record says which side is canonical now.**
The question's premise is that T0 owns `EscalateThenDenyOnTimeout` and the registry lookup plus its
missing-row branch are gone. Removing the registry dependency from T0 is architecturally right — template 4
requires zero `attack_by_id`/`ATTACK_SET` references in the T0 module, and this reviewer confirmed zero. But
the consequence is that a hard-coded outcome constant now lives in T0 while CORE-001's registry is described
elsewhere as canonical, and the only thing preventing silent divergence is a single equality assertion in one
test. That is the ATK-02 failure class re-created one level up: 27 green tests did not catch a `Block`/`Deny`
mismatch, and the standing remedy was to derive expected outcomes from `attack_by_id(...).expected` rather
than hard-code them. Required before merge: (a) record in T0-AUTHORS or the ADR which side is canonical
post-PROD-000 and which is the mirror; (b) make the equality assertion non-droppable — enumerate it in the
bidirectional libtest-enumeration guard that currently polices only the ignored set, so deleting or
`#[ignore]`-ing it fails CI rather than passing quietly.

**B7 — `founder_approval_timeout.rs` was not supplied; Q6 cannot be reviewed at source.**
This is the one file in the change set carrying a non-import edit to a founder-owned enforcement unit, and it
is the file most in need of a non-author read. Supply both revisions plus the isolated R5.1 hunk.

**B8 — The baseline `before_tool_call.rs` was not supplied; the semantic-identity claim rests on the author's own ledger.**
Template 2's entire purpose is to prove the transformed floor preserves approved semantics. Without the
baseline bytes at `e9c8f58…`, a non-author reviewer can verify the destination and the ledger's internal
coherence but not the equivalence being claimed. Supply the baseline file and, ideally, the two baseline
region extracts so the recorded digests (`5e44f9d6…`, `35639394…`, `d9f2b23f…`) can be recomputed
independently.

**B9 — Classification tension on the five relocated shapes.**
Template 1 states the relocated shapes "were previously T3 supporting code, not founder-authored enforcement,"
and classifies all five as agent-authored T0 relocations. T0-AUTHORS states: "Any shared enum or trait
encoding consequential pending, escalated, approved, or denied semantics is T0 until the founder records a
narrower classification." `GuardDecision` (Allow / Escalate / Deny), `GuardFault`, and `GuardDecisionPort`
encode exactly those semantics. Either they were T0-by-rule before PROD-000 — in which case template 1's
"previously T3" understates the transformation and the correct classification is agent-transformed T0 — or
the founder must record the narrower classification the catch-all contemplates. This bears directly on Q1's
truthfulness and should be dispositioned in writing, not inferred.

### Non-blocking

**N1 — `BeforeToolCallObservation::GuardFault` is no longer constructible from a product outcome.**
`before_tool_call.rs:42-46` retains the variant, but the adapter matches only `Blocked`, `Escalated`, and
`Authorized`; nothing in the file constructs it. Either an active test still constructs it — in which case it
asserts a shape the product can no longer produce — or it is dead public surface that survived `-D warnings`
only because public enum variants are exempt from dead-code analysis. Related: collapsing
`Ok(Err(_)) | Err(_)` discards the `GuardFault` discriminant, so `Unavailable`, `InternalError`, and
`FounderImplementationRequired` are indistinguishable at the boundary. That is correct for enforcement (all
deny) but removes a diagnostic distinction a future audit record may want. Recommend removing the variant or
recording the intent, and confirming no active test constructs it.

**N2 — T3 publicly re-exports `before_tool_call_floor`.**
Line 13 re-exports the floor function itself, creating a second public path to the single floor through the
harness module. Not a second floor, but it lets a future consumer bind to the T3 path and drift. Recommend
narrowing the re-export to the types the harness actually needs.

**N3 — In-source non-claim list omits "a route around the hook."**
T0-BOUNDARY lists five exclusions; the module doc (80–82) lists four. Since this module is the PROD-001
extraction candidate, the doc comment is the non-claim that travels with the distributed crate once
T0-BOUNDARY.md no longer accompanies it. Add the route-around exclusion.

**N4 — Literal `{AGENT-AUTHORS}` on line 1.**
Reads as an unsubstituted template token. Resolve it or document the convention.

**N5 — The Semgrep leg is thin: `p/rust` is 11 rules.**
Adequate as one of three engines, but 11 rules is weak coverage for the code-SAST leg. Higher-leverage
suggestion: add repo-specific Semgrep rules that mechanize the boundary invariants this review checked by
hand — forbid `EffectfulToolProbe`, `BeforeToolCallObservation`, `attack_by_id`, `ATTACK_SET`, and
`effectful_invocations` in the T0 floor module, and forbid a second `catch_unwind` outside it. That converts
the T0/T3 partition from a documented rule into a CI-enforced one, which is the same non-bypassability
argument the product makes.

**N6 — The CodeQL SARIF does not self-bind to a commit or to file content.**
`versionControlProvenance` is null, `invocations[0]` has no `commandLine`, `workingDirectory`, or timestamps,
and no artifact carries a `hashes` entry despite `--sarif-add-baseline-file-info`. The binding to
`425d7718…` exists only in the agent-authored `.txt` wrapper. Recommend recording `git rev-parse HEAD` and
`git status --porcelain` inside the wrapper (partly done) and regenerating with provenance so the
machine-readable artifact stands alone.

**N7 — The SARIF diagnostic array is much larger than the finding list.**
`invocations[0].toolExecutionNotifications` holds 101 entries (67 `note`, 33 unlevelled, 1 unset): 22 "File
successfully extracted," the 1 `bindgen.rs` extraction warning, and the remainder CodeQL consistency and
diagnostic-query registrations. `execution_error_count: 0` is supported and no error-level notification
exists. Flagged only because the founder's own open gate is "disposition of every SAST/SCA finding *and
diagnostic*" — that must be read against the 101-entry notification array, not the 9-finding list, or the
gate's wording overstates what was reviewed.

**N8 — No engine covers the repository's JavaScript surface.**
The SARIF artifact table lists nine non-Rust files: `src/gate.mjs`, `scripts/check-structure.mjs`,
`scripts/check-ignored-attacks.mjs`, and `tests/bypass/01…05-*.test.mjs`. None is in any engine's scope —
Semgrep ran `p/rust` over an explicit 21-file `.rs` list, CodeQL used `--language=rust`, cargo-deny is Rust
SCA. So the three-engine claim covers zero of the JavaScript surface, including a file named `src/gate.mjs`
and the two guard scripts that PROD-000's own validation table depends on
(`check:structure`, `check-ignored-attacks`). Either record this as a founder-accepted scope boundary or add
a JS leg.

**N9 — Q5 qualified: a second enforcement-shaped surface exists in the repository.**
`src/gate.mjs` and `tests/bypass/05-gate-throws.test.mjs` indicate a JavaScript gate with its own
throw/fail-closed test, alongside the Rust floor. Neither is PROD-000 scope and neither is classified by any
supplied document. Given T0-BOUNDARY's rule that the harness "must never become an alternate production
enforcement path," the founder should record which surface is authoritative and whether the JS gate is a
superseded Phase-0 artifact.

**N10 — Template 1 attributes `Blocked` to lines 63–67; line 63 is the enum header.**
Trivial; noted only so the founder's line-by-line pass is not tripped by an apparent mismatch.

**N11 — Template 5 records a diff command, not a stored artifact.**
The "Import-only diff artifact/path" field is a `git diff` invocation. Re-running it depends on repo state
and, per B3, on a defective pathspec. Store the eight-file diff as a file and record its digest.

**N12 — The cargo-deny artifact was supplied twice, byte-identical.**
One run. Noted so the artifact count is not misread as two independent SCA executions.

---

## 6. Re-submission manifest to convert this verdict

Supplying the following would let a second pass answer Q1, Q6, Q7, and Q8 at source:

1. PR #90 head SHA and patch digest, plus `git diff --name-only 425d7718..<head>` and a digest over it (B2).
2. Full patch: `git diff e9c8f585…40b7130…` (and head), stored as a file with a digest (B1).
3. `founder_approval_timeout.rs` — both revisions and the isolated R5.1 hunk (B7).
4. Baseline `tests/bypass-rust/src/before_tool_call.rs` at `e9c8f585…` (B8).
5. The remaining seven founder-owned consumers, after-state (Q7).
6. `tests/bypass-rust/tests/attack_set.rs` and `tests/core_004_conformance.rs`, plus the recorded and observed
   baseline digests for `attack_set.rs`, `Cargo.toml`, `Cargo.lock`, `deny.toml`, and the CI workflow (Q8, B5).
7. `src/lib.rs` (module registration at line 13) and the relevant section of
   `specs/CORE-001-bypass-attack-set.md` (Q6, B6).
8. Written dispositions on B3, B4, B5, B6, and B9.

---

## 7. Scope of this record

This review satisfies **only** the non-author cross-model review gate for PROD-000 under ADR-13 Amendment B,
and it does so partially: four of ten questions are `NOT ESTABLISHED` on the supplied inputs, and two of
twenty-one tracked Rust files were read at source.

It is **not** human review, **not** independent-human review, and **not** founder review, approval, or merge
authorization. It does not disposition any Semgrep, CodeQL, or cargo-deny finding — all remain open. It does
not classify any region's authorship on the founder's behalf, does not certify PROD-000 as merge-ready, and
does not authorize PROD-001 extraction. Founder line-by-line review and disposition of the exact final PR
head remain required and are unaffected by this record.

Nothing in this review speaks to live non-bypassability, real runtime integration, missing-hook or
route-around behavior, `panic=abort`, process termination, OOM abort, operator bypass, real human approval
delivery, cross-restart durability, or cross-instance pending state. Those remain outside the isolation
harness and reserved to the deferred runtime-integration epic.

---

**Reviewer:** Anthropic Claude Opus 5 (non-author; implementation authored by OpenAI Codex)
**UTC:** `2026-09-02T16:19:30Z`
**Verdict:** `CHANGES REQUIRED`
