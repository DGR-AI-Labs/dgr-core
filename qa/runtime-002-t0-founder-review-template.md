# RUNTIME-002 T0 founder exact-final-head review template

**Status:** unsigned template — founder disposition pending

This record must be completed by the founder after the non-author cross-model and independent-human
reviews are available. It records founder decisions and review; it does not convert agent-authored
or agent-transformed work into founder-authored work.

## 1. Founder and exact review binding

- Founder name and GitHub identity: `[REQUIRED]`
- Repository and PR: `[REQUIRED]`
- Founder-approved pre-authoring manifest commit/blob/SHA-256: `[REQUIRED]`
- Baseline commit and tree: `[REQUIRED]`
- Reviewed pre-disposition head commit and tree: `[REQUIRED]`
- Baseline-to-head binary-capable patch SHA-256: `[REQUIRED]`
- Complete changed-path inventory SHA-256: `[REQUIRED]`
- Non-author cross-model record commit/blob/SHA-256: `[REQUIRED]`
- Independent-human record commit/blob/SHA-256: `[REQUIRED]`
- UTC review start: `[REQUIRED]`

## 2. Authority, scope, and provenance

- [ ] I resolved the active private authority and every prerequisite against canonical sources.
- [ ] I verified each changed path, symbol, dependency, generated artifact, and consequential region
      is named by the exact pre-authoring manifest.
- [ ] I reviewed every consequential changed and removed line and every public interface.
- [ ] I approve or correct every region-level provenance class.
- [ ] I confirm my supervision, review, approval, or merge does not change agent authorship.
- [ ] I confirm no excluded adapter, Hermes, out-of-process, policy, deployment, runtime-evidence,
      or claim work is present.

## 3. Founder-owned semantic decisions

For every manifest-frozen design and trust decision, record `APPROVE` or `CORRECT`, cite the exact
source location, and explain the founder's rationale.

| Decision/region | Disposition | Exact evidence | Founder rationale or correction |
|---|---|---|---|
| `[REQUIRED — one row per frozen decision and consequential region]` |  |  |  |

## 4. Adversarial-test disposition

- [ ] I reviewed every required adversarial test and its implementation path line by line.
- [ ] I confirmed no expected outcome, ignored set, fixture, assertion, or guard was weakened.
- [ ] I confirmed test and check outputs bind to the reviewed head.

**Founder test disposition and rationale:** `[REQUIRED]`

## 5. Analyzer and diagnostic dispositions

Record a founder disposition for every finding, warning, note, error, skipped rule, extraction
diagnostic, and coverage gap from Semgrep, CodeQL, cargo-deny, and all required host-language and
dependency analyzers. `No code finding` does not dispose of diagnostic or coverage evidence.

Start from `qa/runtime-002-known-analyzer-dispositions.md`. Carry a still-applicable prior founder
disposition forward explicitly; do not silently treat it as a new finding or as resolved. Reopen it
if its source, reachability, threat context, or risk changes.

| Analyzer item | Disposition | Founder rationale | Required remediation or claim bound |
|---|---|---|---|
| `[REQUIRED — one row per item, or an exact indexed disposition attachment]` |  |  |  |

## 6. Cross-model and independent-human findings

| Finding/recommendation | Founder disposition | Founder rationale |
|---|---|---|
| `[REQUIRED — every item from both reviews]` |  |  |

## 7. Scope and non-claims

- [ ] I approve only the manifest-bounded RUNTIME-002 T0 change.
- [ ] I make no claim beyond the exact evidence-supported boundary recorded by the manifest.
- [ ] I do not treat isolation-harness behavior as deployed hook installation, complete route
      coverage, non-bypassability, protected state/key integrity, or operator resistance.
- [ ] I confirm this review does not authorize excluded adapter, runtime-evidence, deployment, or
      product-claim work.

## 8. Overall founder disposition

Select exactly one:

- [ ] `APPROVE REVIEWED HEAD`
- [ ] `APPROVE REVIEWED HEAD WITH RECORDED NON-BLOCKING FINDINGS`
- [ ] `CHANGES REQUIRED`
- [ ] `REJECT`

**Founder rationale:** `[REQUIRED]`

**Founder signature/name:** `[REQUIRED]`

**UTC decision time:** `[REQUIRED]`

## 9. Required final-head sequence

Committing this record creates a new PR head that the record cannot self-name. After this completed
record is committed:

1. require the drift from the reviewed pre-disposition head to contain only completed review/evidence
   records and other explicitly approved non-executable mechanical updates;
2. rerun every required check and analyzer leg on the actual final head;
3. reopen affected reviews, tests, and scans if any executable input changed;
4. submit founder GitHub approval bound to the actual final head; and
5. perform the founder-only merge only after all required contexts pass.
