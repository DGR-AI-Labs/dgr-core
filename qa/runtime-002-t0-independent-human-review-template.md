# RUNTIME-002 T0 independent-human review template

**Status:** unsigned template — independent-human gate pending

The reviewer must be a human who authored none of the implementation or remediation. The reviewer
must inspect the complete patch, source, tests, scanner artifacts, and authorized private authority
inputs personally. Agent-prepared checks may reduce mechanical work but cannot supply the human's
semantic conclusions or signature.

## 1. Reviewer and exact input

- Reviewer name and role/relationship: `[REQUIRED]`
- Confirmation of no implementation/remediation authorship: `[REQUIRED]`
- Repository and PR: `[REQUIRED]`
- Founder-approved pre-authoring manifest commit/blob/SHA-256: `[REQUIRED]`
- Baseline commit and tree: `[REQUIRED]`
- Reviewed head commit and tree: `[REQUIRED]`
- Baseline-to-head binary-capable patch SHA-256: `[REQUIRED]`
- Complete changed-path inventory SHA-256: `[REQUIRED]`
- UTC review start: `[REQUIRED]`
- UTC decision time: `[REQUIRED]`

## 2. Integrity and authority

- [ ] I verified all manifests and hashes before relying on their contents.
- [ ] I resolved the active private authority, founder review, activation verification, backlog
      activation, and exact pre-authoring manifest from authorized canonical sources.
- [ ] I confirmed the reviewed base/head identities and complete changed-path inventory.
- [ ] I confirmed every consequential changed region is manifest-authorized and every unlisted
      consequential path is absent.
- [ ] I confirmed no private ADR body, credential, key, internal-only plan, or sensitive bundle is
      present in the public repository or review artifact.

## 3. Line-by-line enforcement and provenance review

- [ ] I reviewed every changed and removed consequential line and every public interface.
- [ ] I verified the founder-owned design/trust decisions against the approved manifest.
- [ ] I verified every region's provenance classification and found no false founder-authorship
      claim for agent-authored or agent-transformed work.
- [ ] I verified the Rust/host-language authority boundary and every failure/startup-refusal path.
- [ ] I verified binding, retry, nested-call, consumption, approval-continuation, lifecycle, store,
      key, clock, permission, symlink, recovery, and shutdown behavior in scope.
- [ ] I confirmed the change contains none of the exception's excluded work.

## 4. Adversarial tests and analyzers

- [ ] I inspected every manifest-required adversarial test and confirmed no expectation, fixture,
      ignored set, or guard was weakened to obtain green.
- [ ] I verified required checks and test logs bind to the reviewed head.
- [ ] I inspected canonical Semgrep, CodeQL, cargo-deny, host-language, and dependency artifacts,
      including every finding, warning, note, error, skipped rule, extraction diagnostic, and
      coverage gap.
- [ ] I recorded all findings and recommendations below; locational non-impact is not treated as a
      risk disposition.

## 5. Human findings and recommendations

| ID | Severity | File/line or artifact | Finding | Recommendation |
|---|---|---|---|---|
| `[REQUIRED or NONE]` |  |  |  |  |

## 6. Independent-human disposition

Select exactly one:

- [ ] `PASS`
- [ ] `PASS WITH NON-BLOCKING FINDINGS`
- [ ] `CHANGES REQUIRED`
- [ ] `REJECT`

**Rationale:** `[REQUIRED]`

**Reviewer signature/name:** `[REQUIRED]`

**Stable approval reference:** `[REQUIRED]`

**UTC decision time:** `[REQUIRED]`
