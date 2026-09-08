# RUNTIME-002 T0 non-author cross-model review template

**Status:** unsigned template — does not authorize implementation or merge

This record is for a model that authored none of the reviewed implementation or remediation. The
review must bind to the actual candidate head and independently assess the complete change and
evidence, not merely a summary prepared by the authoring model.

## 1. Exact review binding

- Repository and PR: `[REQUIRED]`
- Founder-approved pre-authoring manifest commit/blob/SHA-256: `[REQUIRED]`
- Baseline commit and tree: `[REQUIRED]`
- Reviewed head commit and tree: `[REQUIRED]`
- Baseline-to-head binary-capable patch SHA-256: `[REQUIRED]`
- Complete changed-path inventory SHA-256: `[REQUIRED]`
- Review model/version and provider: `[REQUIRED]`
- UTC review time: `[REQUIRED]`

## 2. Mandatory checks

Record `YES`, `NO`, or `NOT ESTABLISHED` for each item. `NOT ESTABLISHED` requires the exact evidence
gap and the run-only experiment or artifact that would close it.

| Question | Verdict | Evidence and rationale |
|---|---|---|
| Every changed path, symbol, dependency, generated artifact, and consequential region is authorized by the exact founder-approved manifest | `[REQUIRED]` | `[REQUIRED]` |
| All founder-owned design and trust decisions match the frozen manifest without implementation-selected policy | `[REQUIRED]` | `[REQUIRED]` |
| Region-level provenance classifications are complete and truthful | `[REQUIRED]` | `[REQUIRED]` |
| The Rust boundary remains the sole decision authority and host-language code cannot authorize independently | `[REQUIRED]` | `[REQUIRED]` |
| Missing, incompatible, unloadable, panicking, timed-out, malformed, or unavailable states follow the founder-approved fail-closed/startup-refusal contract | `[REQUIRED]` | `[REQUIRED]` |
| Tool and argument binding, retry, nested-call, and single-use behavior match the frozen contract | `[REQUIRED]` | `[REQUIRED]` |
| Startup, store, key, clock, path, ownership, permission, symlink, recovery, and shutdown checks match the frozen profile | `[REQUIRED]` | `[REQUIRED]` |
| Adversarial tests cover every manifest-named attack and no expectation was weakened | `[REQUIRED]` | `[REQUIRED]` |
| Semgrep, CodeQL, cargo-deny, and required host-language/dependency coverage bind to the reviewed head and disclose all diagnostics and gaps | `[REQUIRED]` | `[REQUIRED]` |
| The diff contains no excluded adapter, Hermes, out-of-process, policy, deployment, runtime-evidence, or claim work | `[REQUIRED]` | `[REQUIRED]` |
| The evidence supports only the bounded claim authorized by the manifest | `[REQUIRED]` | `[REQUIRED]` |

## 3. Findings and dispositions required from the founder

| ID | Severity | File/line or artifact | Finding | Required action or claim narrowing |
|---|---|---|---|---|
| `[REQUIRED or NONE]` |  |  |  |  |

## 4. Verdict

Select exactly one:

- [ ] `PASS`
- [ ] `PASS WITH NON-BLOCKING FINDINGS`
- [ ] `CHANGES REQUIRED`
- [ ] `REJECT`

**Rationale:** `[REQUIRED]`

**Reviewer/model identity:** `[REQUIRED]`

**UTC decision time:** `[REQUIRED]`
