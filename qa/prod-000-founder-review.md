# PROD-000 founder final review and disposition

**Status:** founder disposition complete — final-head GitHub approval and founder-only merge pending

This record captures the founder's personally supplied decisions and review statements. Codex
mechanically assembled and verified the record from those statements; it did not make the founder's
decisions. This review does not convert agent-authored or agent-transformed T0 into
founder-authored T0.

## 1. Founder identity and review-input binding

- Founder name: Khazretgali Sapenov
- UTC review start: `2026-09-02T20:27:00Z`
- Reviewed pre-disposition PR head SHA:
  `5cabd9bc611ff9a4c8255ed46b5984d621d2f10a`
- Reviewed pre-disposition tree SHA:
  `bfed766b97867c3222ec504a5d786cc12e6e9028`
- PR: [DGR-AI-Labs/dgr-core#90](https://github.com/DGR-AI-Labs/dgr-core/pull/90).
- `Structural / governance check`: confirmed passing on the reviewed head.
- `Rust format / build / test`: confirmed passing on the reviewed head.
- Post-executable drift: confirmed that drift from executable commit
  `587585cf476431f078efe587c5dbcc052389cdad` through the reviewed head contains only
  documentation, evidence, completed review records, and bundle artifacts. No Rust source, test,
  executable script, workflow, Cargo input, lockfile, or dependency-policy drift was found.

Evidence identities independently reverified during founder-record preparation:

- baseline commit/tree:
  `e9c8f585809c15d2464b3d45bc2ce26d716c8673` /
  `837bef46997d5c8a9db946d2a87c1d4b6ce5aced`;
- executable commit/tree:
  `587585cf476431f078efe587c5dbcc052389cdad` /
  `89e5de51d23ead98d24bbbe1b4cd57db343b2dc4`;
- safe evidence source/tree:
  `46da707b8b84dfa599c4e27e5fbb2dc005e9e0e4` /
  `16b5812cf3bdfce585a9e04da720ab99b0bcac29`;
- baseline-to-executable patch SHA-256:
  `c08919d86a1f060cce9a05b3143140a5f011b9349f243475dad4f4ec1b40cf99`;
- R2 package SHA-256:
  `b26825a72ff1603fba263c8355f32429b386ae80a98f7dee1b6648466ce319da`.

The R2 package verification passed: manifest 106/106, selected inventory 77/77, critical baseline
8/8, critical executable 12/12, cross-model records 2/2, scanner artifacts 6/6, and external
authority records 3/3 against the authorized internal checkout.

## 2. Required prior records

- [x] I read active ADR-13 Amendments A and B.
- [x] I read the original `CHANGES REQUIRED` Claude review.
- [x] I read the passing Claude addendum and its N13–N15 follow-up disposition.
- [x] I read the completed independent-human review and verified the reviewer did not author the
      implementation or remediation.
- [x] I confirmed no record describes the complete PROD-000 result as founder-authored.

## 3. Founder line-by-line source review

- [x] I reviewed every consequential line in `founder_before_tool_call_floor.rs`.
- [x] I reviewed every removed line from the historical mixed T3/founder-source floor.
- [x] I reviewed every public type, trait, function, variant, and re-export decision.
- [x] I reviewed `r5-1-timeout-semantic.diff`, including the removed missing-row fail-closed branch,
      authoritative T0 constant, T3 mirror, and final deny mapping.
- [x] I reviewed the raw EOL diff and separately accept its file-wide CRLF-to-LF normalization.
- [x] I reviewed all eight founder-owned consumers and confirmed seven are import-only.
- [x] I reviewed the T3 adapter and confirmed the probe is reachable only after `Authorized`.
- [x] I inspected the ATK-06 equality assertion body directly and understand that the CI
      enumeration guard proves only that the named test remains present and active.
- [x] I confirmed no test expectation, ignored attack, Cargo input, dependency policy, workflow
      context name, denial signal, deadline, store operation, or unrelated enforcement behavior was
      changed outside Amendment B.

### 3.1 Section 6.1 — T0 floor and provenance

I reviewed `tests/bypass-rust/src/founder_before_tool_call_floor.rs` line by line and compared the
relocation with `qa/prod-000-review-evidence/floor-semantic-identity.diff` and provenance templates
1 and 2. I confirm that the documented founder-source provenance, agent transformation, and
non-founder T0-by-consequence classifications are accurate.

The verification reproduced the baseline complete-file hash
`5e44f9d6c4451bbe80c7821a6587663110b1144a6895a4dd2f8548d0e0de049d`, destination complete-file
hash `d1c98dedbf544ab1e27d3d9e12055f96e8a5d5b76b2c63edb76e4df4ff0b542f`, both baseline-region
hashes, and destination-region hash recorded in templates 1 and 2. The T0 module contains one
bounded `catch_unwind` and no probe, observation, fixture, attack-registry, effectful-counter, or T3
reference.

### 3.2 Section 6.2 — T3 facade and probe

I confirm that the T3 adapter invokes the probe only after `BeforeToolCallOutcome::Authorized`.
Blocked and escalated paths do not issue authorization or invoke the probe. This proves ordering in
the isolation harness only. It does not prove that a deployed agent cannot bypass a hook, that every
runtime tool route is intercepted, or that the hook is installed.

### 3.3 Section 6.3 — module registration and dependency boundary

I confirm that `founder_before_tool_call_floor` is registered exactly once at
`tests/bypass-rust/src/lib.rs:13`, with no unrelated module, feature, or public-export change. The
template-4 dependency graph is accurate: founder T0 units and the T3 adapter depend on the new T0
module; the new T0 module has zero imports or references to the T3 facade, fixtures, probes,
observations, or attack registry. Each public symbol has a concrete recorded consumer.

### 3.4 Section 6.4 — R5.1

I reviewed the EOL-insensitive semantic hunk and raw diff separately. I accept the mechanical
CRLF-to-LF normalization separately from the semantic R5.1 change. The semantic hunk contains only
the module-path rewrite, authoritative T0 timeout-outcome constant, hidden T3 conformance mirror,
removal of the now-impossible T3 registry/missing-row branch, and use of the T0 constant in the
terminal deny. The denial signal and remaining pending, missing, and faulted behavior are unchanged.
Templates 2 and 5 accurately classify these changes.

### 3.5 Section 6.5 — eight consumer rewrites

I reviewed `qa/prod-000-review-evidence/eight-consumer.diff` and all eight destination files. The
focused artifact has exactly eight file headers and eight hunks. Seven files contain one removed and
one added import line only. `founder_approval_timeout.rs` contains the import rewrite plus the
separately reviewed R5.1 and EOL normalization.

All sixteen template-5 before/after SHA-256 values reproduce against the baseline and executable
commits. The filename prefix does not establish authorship; the recorded markers and provenance
remain controlling.

### 3.6 Section 6.6 — ATK-06 assertion and N14

I read the assertion body at `tests/bypass-rust/tests/core_004_conformance.rs:168-172`. It compares
the hidden T0 mirror with the registry-derived expected terminal result and then with
`RequiredOutcome::EscalateThenDenyOnTimeout`. The named test remains present, active, and passing.

The CI guard proves that the named test remains listed and is not ignored; it does not prove that
the equality-assertion body remains present. I accept and record that limitation under N14.

### 3.7 Section 6.7 — active/ignored guard

I confirm that the expected ignored set remains exactly ATK-04, ATK-05, ATK-12, ATK-14, and ATK-15.
Deletion, rename, or `#[ignore]` of the named ATK-06 test makes the active-test comparison fail.
`.github/workflows/ci.yml:39-42` invokes both the JavaScript comparison test and live enumeration
under the blocking `Rust format / build / test` job. This is not a JavaScript SAST claim and does not
establish assertion-body integrity.

The locally reverified locked suite completed with 52 passed and 5 ignored tests.

## 4. Provenance-template dispositions

The original agent-prepared templates remain unchanged and inspectable. My consolidated founder
dispositions are:

| Template | Founder disposition | Lines/hashes and authorship | Amendment-B scope and rationale |
|---|---|---|---|
| 1 — boundary module | **APPROVE** | All cited lines and complete-file hashes resolve. The five shapes are truthfully classified as pre-existing non-founder T0-by-consequence and agent-relocated/transformed; the transformed floor retains identified founder-source provenance without becoming founder-authored. | The module and its exact authority-mandated filename fall within Amendment B. `{AGENT-AUTHORS}` and `T0-AUTHORS.md` prevent the filename from becoming an authorship claim. No correction required. |
| 2 — semantic identity | **APPROVE** | Both baseline-region hashes and the destination-region hash reproduce. Every non-verbatim constructor and method-to-function transformation is identified; the reached-boundary fault/unwind behavior and explicit non-claims are preserved. | The relocation/transformation is within Amendment B and does not add policy, token verification, store semantics, or tool execution. No correction required. |
| 3 — removal and handoff | **APPROVE** | All cited old and destination ranges resolve. The old T3 file contains zero `catch_unwind` or floor denial-signal occurrences; the new T0 module contains the single active floor. | The removal, compatibility re-exports, observation surface, and adapter handoff match the authorized T0/T3 partition. No correction required. |
| 4 — registration and dependency | **APPROVE** | Registration resolves once at `lib.rs:13`; all seven public symbols have recorded consumers. The T0 module has zero references to T3, fixtures, probes, observations, or the attack registry. | Dependency direction and public surface match Amendment B. No correction required. |
| 5 — import ledger | **APPROVE** | All cited lines and all sixteen before/after hashes resolve. The corrected timeout import is line 10. Exactly seven files are import-only; the eighth contains the separately reviewed R5.1 and EOL normalization. | All eight named rewrites fall within Amendment B. Clarification: `full-implementation.diff` binds the cross-model-remediation checkpoint `b19f33a...`; the canonical final baseline-to-executable patch is `full-resubmission.diff`, SHA-256 `c08919d8...`, through `587585c...`. This clarification does not change the template's classifications. |

## 5. Cross-model and non-blocking findings

| Finding | Founder disposition and rationale |
|---|---|
| N5 — Semgrep `p/rust` is an 11-rule leg; custom boundary rules deferred | **ACCEPT.** The scan covers all 21 tracked Rust files but is a bounded 11-rule general Rust leg. Repository-specific boundary rules are not claimed and are appropriately deferred outside Amendment B. |
| N6 — CodeQL SARIF requires external commit/tree binding | **ACCEPT.** The SARIF does not self-bind to a commit or tree. The wrapper, reconstructed source, commit-to-tree verification, and matching executable tree `89e5de51...` provide an adequate external binding for this gate without describing the SARIF as self-authenticating. |
| N7 — complete CodeQL notification array | **ACCEPT.** I reviewed the complete 99-entry array: 67 entries at level `note`, 32 at level `none`, and zero warning/error notifications. I did not rely only on the nine security results or wrapper summary. |
| N8 — no JavaScript SAST claim | **ACCEPT.** JavaScript changed in the T3 guard but was not covered by Semgrep or CodeQL. Syntax checking, comparison tests, and live enumeration passed; those checks are not represented as JavaScript SAST. |
| N13 — stored EOL-insensitive R5.1 hunk | **ACCEPT.** The reduced semantic artifact accurately isolates the substantive R5.1 change, while the raw diff preserves the separate line-ending normalization record. |
| N14 — name/active-state guard does not prove assertion body | **ACCEPT.** The guard detects deletion, rename, and ignored state but not removal of the equality body while retaining the test name. The assertion body was directly inspected and the claim remains explicitly bounded. |
| N15 — ledger line corrected from 9 to 10 | **ACCEPT.** Direct inspection confirms the new import in `founder_approval_timeout.rs` is at line 10; the corrected template reference resolves. |

### Founder response to independent-human recommendations

I read and independently accept each recommendation:

1. **Semgrep temporary path — ACCEPT.** The reported construction is confined to local test code
   and grants no production authority. Timestamp-plus-process-ID naming is not a general security
   guarantee, but the residual collision/symlink/race exposure is bounded to this test environment
   and does not justify an out-of-scope T0 change in PROD-000.
2. **Nine deterministic fixture nonces — ACCEPT.** These values intentionally make conformance
   fixtures reproducible. They are not production secrets, signing keys, or reusable production
   capabilities, and no result touches a PROD-000 changed enforcement region.
3. **`founder_` filename classification — ACCEPT.** Amendment B mandates the filename, while the
   inline `{AGENT-AUTHORS}` marker and `T0-AUTHORS.md` truthfully preserve agent authorship and
   transformation provenance.
4. **ATK-06 equality and canonical ownership — ACCEPT.** T0 owns the authoritative timeout outcome,
   and the T3 assertion directly verifies registry equality. The active-name guard prevents silent
   deletion, rename, or ignoring, subject to the recorded N14 body-integrity limitation.
5. **Previously missing inputs — ACCEPT AS RESOLVED.** The sanitized R2 package, public Git object
   database, and separately authorized internal checkout now permit verification of all required
   bytes, identities, diffs, scanner artifacts, and authority hashes.

## 6. Semgrep disposition

Canonical artifact: `qa/sast/prod-000-final-input-semgrep-2026-09-02.json`.

- Finding: `rust.lang.security.temp-dir.temp-dir` at
  `tests/bypass-rust/tests/consumption_store.rs:19`.
- Founder decision: **ACCEPT**.
- Rationale: the code constructs a local test database path under `std::env::temp_dir()` from
  timestamp nanoseconds and process ID. This is test-only code and cannot be reached as production
  authorization logic. The naming is not cryptographically unpredictable and does not eliminate
  all collision, symlink, race, or cross-user risk, but that residual risk is bounded to the local
  test environment. Remediating it would be unrelated to Amendment B and should be separately
  scoped rather than folded into this T0 partition.
- Changed-region confirmation: the finding predates PROD-000 and does not touch a changed
  PROD-000 region. This locational fact supports scope analysis but is not the sole risk rationale.
- Coverage confirmation: 21/21 tracked Rust files, 11 `p/rust` rules, zero JavaScript SAST coverage.

## 7. CodeQL disposition

Canonical artifact: `qa/sast/prod-000-final-input-codeql-2026-09-02.sarif`.

- Results: seven deterministic nonce results at
  `tests/bypass-rust/src/val_002_fixtures.rs:126,152,169,186,204,222,339` and two at
  `tests/bypass-rust/src/val_004_fixtures.rs:136,143`.
- Founder decision for all nine fixture results: **ACCEPT**.
- Rationale: the fixed byte arrays are deliberate deterministic conformance-fixture nonces used to
  exercise exact token, expiry, attack, and authorization behavior. They are not production
  cryptographic secrets, signing keys, or capabilities and do not grant authority outside the
  isolation tests. Public visibility and reuse are part of fixture reproducibility, not production
  nonce generation. Their unchanged baseline provenance limits PROD-000 scope but does not erase
  the analyzer's reported risk.
- Founder decision for the complete diagnostic array: **ACCEPT**. The canonical SARIF contains 99
  notifications: 67 at level `note`, 32 at level `none`, and zero warning/error notifications.
- Founder decision for the external binding limitation: **ACCEPT**. The SARIF contains no adequate
  self-contained commit/tree provenance or artifact hashes. Its wrapper, source reconstruction,
  public commit-to-tree mapping, and executable-tree equality provide the binding used here. No
  claim of SARIF self-authentication is made.
- Changed-region confirmation: none of the nine results touches a changed PROD-000 region.

## 8. cargo-deny disposition

Canonical artifact: `qa/sast/prod-000-final-input-cargo-deny-2026-09-02.txt`.

- Verified result: exit 0; advisories 0/0/0; bans 0 errors, 0 warnings, 2 notes; licenses 0 errors,
  0 warnings, 54 notes; sources 0/0/0.
- Founder decision: **ACCEPT**.
- Rationale: the notes are compatible with the checked-in dependency policy and produce no denied
  advisory, ban, license, or source. `Cargo.toml`, `Cargo.lock`, `deny.toml`, and the pinned Rust
  toolchain did not change in the executable patch. The decision rests on the complete categorized
  result and unchanged policy inputs, not only the exit code.

## 9. Scope and non-claims

- [x] I accept that PROD-000 proves only the bounded isolation-harness partition and preservation
      of the reviewed reached-boundary floor behavior.
- [x] I make no claim of deployed interception, agent non-bypassability, missing-hook or
      route-around resistance, operator-proofing, abort/termination/OOM containment, real human
      delivery or waiting, cross-instance state, or safe post-panic store reuse.
- [x] I confirm this review does not authorize PROD-001 until PR #90 is founder-approved and merged.
- [x] I confirm founder review does not convert agent-authored or agent-transformed lines into
      founder-authored lines.

## 10. Founder disposition before final GitHub approval

- [ ] **APPROVE REVIEWED HEAD**.
- [x] **APPROVE REVIEWED HEAD WITH RECORDED NON-BLOCKING FINDINGS**.
- [ ] **CHANGES REQUIRED**.
- [ ] **REJECT**.

Founder rationale:

> I reviewed the complete bounded PROD-000 partition, its consequential lines, removed branch,
> public surface, provenance classifications, prior reviews, and analyzer evidence. The verified
> behavior and provenance conform to Amendment B. The remaining findings are understood, expressly
> bounded, and non-blocking; none supports a broader runtime-interception or non-bypassability claim.

## 11. Founder attestation

```text
I, Khazretgali Sapenov, personally reviewed and dispositioned the complete PROD-000 change and
evidence at reviewed pre-disposition PR head 5cabd9bc611ff9a4c8255ed46b5984d621d2f10a.
I reviewed every consequential changed line, removed branch, public item, provenance claim,
independent-human and cross-model record, every SAST/SCA result, and the complete CodeQL diagnostic
array. I confirm the required GitHub contexts pass on that exact reviewed head.
Decision: APPROVE REVIEWED HEAD WITH RECORDED NON-BLOCKING FINDINGS.
This is founder review of agent-authored/transformed T0; it is not a claim of founder authorship.
```

- Founder signature/name: Khazretgali Sapenov
- Stable founder identity reference: [GitHub `sapenov`](https://github.com/sapenov)
- UTC decision time: `2026-09-03T00:27:40Z`
- Toronto decision time: `2026-09-02T20:27:40-04:00`
- Final-head GitHub approval reference: pending completion of the post-commit sequence below.

## 12. Required final GitHub action after this record is committed

This record names the reviewed pre-disposition head. Committing it creates the final candidate head,
which is bound by the subsequent GitHub approval rather than by a self-referential SHA here.

- [ ] Commit and push this founder record and the two verified documentation-only corrections to
      the founder-review runbook.
- [ ] Record the resulting final PR head and tree.
- [ ] Verify drift from `5cabd9bc611ff9a4c8255ed46b5984d621d2f10a` contains only this founder
      record and those two documentation corrections.
- [ ] Wait for `Structural / governance check` and `Rust format / build / test` on that final head.
- [ ] Submit founder GitHub **Approve** on that exact final head and retain its review URL.
- [ ] Do not push after approval; if the head moves, repeat drift review, checks, and approval.
- [ ] Perform the founder-only merge and verify the merge reaches the protected default branch.
- [ ] Only after merge may PROD-001 be considered for authorization.
