# PROD-001 Founder Review Document

**Reviewer:** Khazretgali Sapenov (`@sapenov`)
**Status:** signed founder review and dispositions complete for pre-disposition head `60bc229dedbc72d8d3f8b262f44d897a8a0342ac`; final-head checks, approvals, and merge pending.

**Submitted source SHA-256:** `03e03b56bbd7d63dd570fcfe22107779bac3a1e9a253d4baa7826bd073258d00`

**Mechanical normalization:** OpenAI Codex saved the founder-supplied record under the canonical QA filename, normalized Markdown and line endings, and expanded the abbreviated status head. No founder judgment, disposition, attestation, timestamp, or signature was changed.

**Project Scope:** PROD-001 core extraction and bounded T0/T3 partition

**Target PR:** `DGR-AI-Labs/dgr-core#91`

**Final Candidate Commit:** `60bc229dedbc72d8d3f8b262f44d897a8a0342ac`

**Final Candidate Tree:** `797514f13b9eb6d5759635cae525bddf81818aa6`

**Review-Start Timestamp (UTC):** `2026-09-03T19:50:00Z`

**Decision Timestamp (UTC):** `2026-09-03T20:10:00Z`

---

## 1. Provenance, Integrity, and Verification Attestation

- **Base Commit:** `8318f61eadf689f9b8a72f673cc68cd083dc7831` — *Verified personally via git log and object inspection.*

- **Cross-Model Reviewed Commit:** `f1d17087d140b41750c1aeca032916bb4d2d90ae` — *Verified personally against review interchange artifacts.*

- **Manifest validations:** Top-level manifest: 565/565 verified. Core nested manifest: 122/122 verified. Addendum nested manifest: 39/39 verified. All have been personally and fully re-verified across all entries against the local storage state.

- **ZIP Integrity:** SHA-256 hash `bc858fadcf55fda0f15e3ba29b31ada9d084d2c8b7ca1fa24b6d90d168207b6e` — *Confirmed via local recomputation completed before the decision.*

- **Line-by-Line Attestation:** I personally reviewed `metadata/baseline-to-final.diff` and `metadata/reviewed-to-final.diff` line by line. The latter contains only the six recorded workflow/review-document changes, and `metadata/implementation-drift.empty` confirms no reviewed implementation drift.

---

## 2. Evidence, Change Manifests, and Scope Narrowing

- **Required Entry Points:** `README-FIRST.md`, `review/PROMPT.md`, `review/FOUNDER-REVIEW-TEMPLATE.md`, `metadata/final-evidence-summary.md`, and `metadata/reproduction-commands.txt` are verified present.

- **Snapshots:** Baseline snapshot (`source/baseline/`, 168 files) and final candidate snapshot (`source/final/`, 175 files).

- **Diffs:** Full binary-capable base diff (`metadata/baseline-to-final.diff`) and six-path post-review correction diff (`metadata/reviewed-to-final.diff`).

- **Prerequisite Evidence:** Retained under `github/pr-90*`.

### Required Non-Claims & Scope Clarification

- No proof of hook installation.
- No proof every runtime tool route is intercepted.
- No proof of agent non-bypassability.
- No proof of same-process store/key protection.
- No proof of deployed fail-closed behavior.
- The listing guard does not prove assertion-body integrity.
- **Authorship Boundary:** Founder review does not relabel agent-authored, agent-transformed, or agent-relocated bytes as founder-authored.

---

## 3. Semantic Relocation and T0 Boundary Check

- **Drift Attestation:** The `metadata/implementation-drift.empty` file proves zero implementation drift *only* from commit `f1d17087…` to `60bc229…`. The base-to-final change intentionally contains the PROD-001 relocation and wiring.
- **T0 Relocation Ledger:** 18 before/after whole-file entries in `metadata/t0-relocation.sha256` verified successfully (18/18).

- **Shared Definition Regions Ledger:** 6 before/after region entries in `metadata/shared-definition-regions.sha256` verified successfully (6/6).

---

## 4. Comprehensive Founder Decisions

### A. Relocated T0 Files (9 Files)

1. **`founder_approval_store.rs`**: **APPROVE**. Hashes resolve; before and after bytes are identical. Existing authorship and bounded isolation semantics are preserved. No deployed-runtime guarantee is inferred.
2. **`founder_approval_timeout.rs`**: **APPROVE**. Hashes resolve; before and after bytes are identical. Existing authorship and bounded isolation semantics are preserved. No deployed-runtime guarantee is inferred.
3. **`founder_authored_guard.rs`**: **APPROVE**. Hashes resolve; before and after bytes are identical. Existing authorship and bounded isolation semantics are preserved. No deployed-runtime guarantee is inferred.
4. **`founder_before_tool_call_floor.rs`**: **APPROVE**. Hashes resolve; before and after bytes are identical. Existing authorship and bounded isolation semantics are preserved. No deployed-runtime guarantee is inferred.
5. **`founder_consumption_store.rs`**: **APPROVE**. Hashes resolve; before and after bytes are identical. Existing authorship and bounded isolation semantics are preserved. No deployed-runtime guarantee is inferred.
6. **`founder_fail_closed.rs`**: **APPROVE**. Hashes resolve; before and after bytes are identical. Existing authorship and bounded isolation semantics are preserved. No deployed-runtime guarantee is inferred.
7. **`founder_s2_approval_store.rs`**: **APPROVE**. Hashes resolve; before and after bytes are identical. Existing authorship and bounded isolation semantics are preserved. No deployed-runtime guarantee is inferred.
8. **`founder_s2_consumption_store.rs`**: **APPROVE**. Hashes resolve; before and after bytes are identical. Existing authorship and bounded isolation semantics are preserved. No deployed-runtime guarantee is inferred.
9. **`founder_token_verification.rs`**: **APPROVE**. Hashes resolve; before and after bytes are identical. Existing authorship and bounded isolation semantics are preserved. No deployed-runtime guarantee is inferred.

### B. Project Metadata & Governance Fields

- **RequiredOutcome:** **APPROVE**. Complete source region is byte-identical.
- **ProposedAction:** **APPROVE**. Complete source region is byte-identical; it remains opaque input and does not define an execution path.
- **DecisionContext:** **APPROVE**. Complete source region is byte-identical; it remains scenario/test context, not a complete operational-risk model.

### C. Wiring, Exports, CI, and Documentation

- **Cargo/Workspace Wiring:** **APPROVE**. Evidence resolves; workspace members synchronized.
- **Lockfile Relocation:** **APPROVE**. Evidence resolves; dependency tree locked.
- **Module Exports:** **APPROVE**. Evidence resolves; surface bounded.
- **Harness Re-exports:** **APPROVE**. Evidence resolves; test utilities segregated.
- **CI Workflows:** **APPROVE**. Evidence resolves; required job names are preserved and workspace checks execute from the root.
- **Documentation:** **APPROVE**. Evidence resolves; repository documentation reflects the new location and bounded non-claims.

### D. Authorship Categories

- **Category 1 (Existing founder-authored source moved byte-identically):** **APPROVE**.
- **Category 2 (PROD-000 agent-authored T0):** **APPROVE**.
- **Category 3 (PROD-000 agent-transformed T0):** **APPROVE**.
- **Category 4 (PROD-001 agent-authored relocation and wiring):** **APPROVE**.

### E. F-1 through F-9 Decisions

- **F-1 (CodeQL PR diff-range scope, full-branch results, diagnostic limitation):** **ACCEPT**. Evidence resolves; boundaries noted.
- **F-2 (cargo-deny workspace scope):** **ACCEPT**. Evidence resolves; policy checked against `deny.toml`.
- **F-3 (legacy failing `src/gate.mjs` placement):** **ACCEPT**. Evidence resolves; uncompiled JS scaffold remains in place. I accept the residual provenance/perception risk of leaving `gate.mjs` under `src/`.
- **F-4 (documentation-hidden public conformance surface):** **ACCEPT**. Actual inventory is eight `CONFORMANCE_*` constants and three `conformance_*` functions. All are `#[doc(hidden)]` but remain reachable by Git-dependency consumers. I accept that bounded public surface at this stage; documentation hiding is not access control. *(Note: This corrects the earlier records’ inaccurate “eleven constants” description).*
- **F-5 (workspace test evidence):** **ACCEPT**. Evidence resolves; 52 passed, five ignored (`ATK-04`, `ATK-05`, `ATK-12`, `ATK-14`, `ATK-15`), named `ATK-06` test active.
- **F-6 (Node runner presentation):** **ACCEPT**. `node --test FILE` reports one top-level file unit, while CI’s direct `node FILE` reports seven cases; both execute the same seven-case suite.
- **F-7 (reviewed-head-bound hash reproduction):** **ACCEPT**. Evidence resolves.
- **F-8 (PR #90 prerequisite evidence):** **ACCEPT**. Evidence resolves.
- **F-9 (run/job traceability):** **ACCEPT**. Push run `33793355575` (attempt 1) and PR run `33793360491` (attempt 1) both bind to `60bc229…`. Required structural and Rust jobs passed. Semgrep completed with one finding and exited `1` under `--error`; the job was non-blocking and both overall runs succeeded.

### F. A-1 through A-4 Decisions

- **A-1 (Semgrep temporary-directory result):** **ACCEPT**. Located in unchanged T3 test source.
- **A-2 (nine deterministic CodeQL fixture nonces):** **ACCEPT**. The nine values are unchanged deterministic T3 fixture nonces, not production secrets.
- **A-3 (two raw CodeQL diagnostics, one unenumerable):** **ACCEPT**. Limitation explicitly accepted.
- **A-4 (CodeQL v3 deadline and Node 20→24 forcing):** **ACCEPT**. Tooling constraints acknowledged; must be tracked separately before December 2026.

---

## 5. Explicit Disposition of Analyzer Results & Codebase Findings

- **Semgrep temp-dir finding:** **ACCEPT**. Located in unchanged T3 test source; non-blocking.
- **Nine deterministic CodeQL fixture nonces:** **ACCEPT**. Unchanged deterministic T3 fixture nonces, not production secrets; non-blocking.
- **Generated `bindgen.rs` warning:** **ACCEPT**. Generated dependency file; warning reports an unavailable semantic analyzer; non-blocking.
- **Unenumerable diagnostic limitation:** **ACCEPT**. Limitation explicitly accepted without characterizing its content.
- **Two cargo-deny bans notes:** **ACCEPT**. Raw notes reviewed against `deny.toml` policy; non-blocking.
- **All 55 licence entries:** **ACCEPT**. Personally reviewed against `deny.toml` policy; all entries accepted as compliant.
- **CodeQL Action v3 December 2026 deadline & Node 20→24 forcing:** **ACCEPT**. Acknowledged; must be tracked separately before December 2026.
- **Legacy failing `src/gate.mjs`:** **ACCEPT**. Unchanged deliberately failing Phase-0 JavaScript scaffold that Rust does not compile; remains under `src/`. Residual provenance/perception risk accepted.
- **Eleven documentation-hidden public conformance items:** **ACCEPT**. Corrected from earlier records; actual inventory contains eight `CONFORMANCE_*` constants and three `conformance_*` functions.
- **ATK-06 assertion-body inspection:** **ACCEPT**. Lines 168–172 verify equality between the T0 timeout outcome, registry-derived expected outcome, and `EscalateThenDenyOnTimeout`.
- **Guard limitation:** **ACCEPT**. Proves test presence, name, and not-ignored status, but does not prove assertion-body integrity.
- **CI Traceability:** Push run `33793355575` (attempt 1) and PR run `33793360491` (attempt 1) both bind to `60bc229…`. Required structural and Rust jobs passed. Semgrep completed with one finding and exited `1` under `--error`; the job was non-blocking and both overall runs succeeded.

---

## 6. Exact-Head Binding Statement, Disposition, and Post-Signature Checklist

**Binding Statement:**

> I bind this review only to commit `60bc229dedbc72d8d3f8b262f44d897a8a0342ac` and tree `797514f13b9eb6d5759635cae525bddf81818aa6`.

**Determination:** APPROVE REVIEWED FINAL CANDIDATE WITH RECORDED NON-BLOCKING FINDINGS

### Post-Signature Checklist

- [ ] Signed founder record committed without implementation drift.
- [ ] Required checks pass and all three analyzer legs run on the resulting head.
- [ ] Independent GitHub approval applies to that exact head.
- [ ] Founder GitHub approval applies to that exact head.
- [ ] Founder-only merge completed.

*Signed,*
**Khazretgali Sapenov** (`@sapenov`)
