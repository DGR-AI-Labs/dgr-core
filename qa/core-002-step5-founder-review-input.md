# CORE-002 Step 5 founder review input

This file is review input, not founder approval. It does not modify or satisfy
the protected checklist. The founder must author the final dispositions and
sign-off in `specs/CORE-002-guard-review-checklist.md`.

## Suggested analyzer dispositions

### Semgrep: test temporary directory

**Finding:** `rust.lang.security.temp-dir.temp-dir` at
`tests/bypass-rust/tests/consumption_store.rs:19`.

Suggested founder-authored disposition if accepting the current test code:

> Accepted for this developer-only conformance test. The path contains process
> ID plus nanosecond time, stores no secrets, and is not used by production
> construction. A malicious same-host process could race or pre-create the
> predictable path, so this exception does not support a production-security
> claim. Replace it with securely created temporary storage before reusing this
> helper outside isolated tests. No T0 enforcement path is affected.

If that residual local race is unacceptable, replace the helper with a secure
temporary-file/directory primitive and rerun Semgrep, CodeQL, cargo-deny, tests,
Clippy, and formatting against the new full commit. Do not mark the current raw
evidence as covering a remediated commit.

### CodeQL: seven deterministic fixture nonces

**Finding:** `rust/hard-coded-cryptographic-value` at
`tests/bypass-rust/src/val_002_fixtures.rs:126,152,169,186,204,222,339`.

Suggested grouped founder-authored disposition:

> Accepted as intentional deterministic test fixtures. The seven values are
> distinct, fixed 16-byte nonce inputs used to make VAL-002 vectors stable and
> reviewable. They are not production keys, passwords, salts, initialization
> vectors, or runtime nonce-generation logic; the file is fixture support and
> cannot authorize by label. Randomizing them would reduce reproducibility
> without improving the production enforcement path. This exception covers
> only the seven listed locations and no future hard-coded cryptographic value.

### cargo-deny: passing policy with visible notes

Suggested founder-authored disposition:

> Reviewed and accepted. cargo-deny 0.20.2 completed against
> `0a54d4995d1b9d98ab8a3ec61861fe2fe7ae29c3` with zero advisory, license,
> ban, or source errors and zero warnings. The two duplicate-dependency notes
> are narrow, version-pinned transitive exceptions documented in `deny.toml`;
> the 54 license notes resolve to the committed allow-list. This approval does
> not waive future advisories, new sources, new licenses, or unpinned duplicate
> exceptions.

## Protected checklist edits for the founder

In `specs/CORE-002-guard-review-checklist.md`:

1. Lines 83–90: check an item only after reviewing its cited raw evidence.
   Line 87 may be checked after the three dispositions above are authored.
   Line 88 requires the independent human reviewer; founder-only self-review
   does not satisfy the reviewer/writer separation recorded by FND-7.
2. Lines 96–102: replace each `{FOUNDER-SUPPLY}` and `Pending` value with the
   actual reviewer, date, stable evidence location, and result. Suggested row
   structure:

```markdown
| Founder authorship confirmation | Founder — `T0-AUTHORS.md`; reviewed YYYY-MM-DD | PASS — five T0 units founder-authored; no agent-authored T0 changes |
| Human code review | NAME — file/line review, YYYY-MM-DD | PASS / CHANGES REQUIRED — include finding links |
| Adversarial test | `qa/core-002-step5-review-readiness.md` | PASS — 37 passed, 7 explicitly deferred, 0 failed |
| Cross-model review | `qa/core-002-step5-cross-model-review.md` | PASS WITH ERRATA — findings resolved or accepted |
| SAST 1 | `qa/sast/core-002-step5-semgrep-2026-08-17.txt`; founder disposition YYYY-MM-DD | PASS WITH ACCEPTED TEST-ONLY FINDING / CHANGES REQUIRED |
| SAST 2 | `qa/sast/core-002-step5-codeql-2026-08-17.sarif`; founder disposition YYYY-MM-DD | PASS WITH 7 ACCEPTED FIXTURE FINDINGS / CHANGES REQUIRED |
| SAST 3 | `qa/sast/core-002-step5-cargo-deny-2026-08-17.txt`; founder review YYYY-MM-DD | PASS — 0 blocking diagnostics; notes reviewed |
```

3. After the table, add the founder's dated final decision, for example:

```markdown
**Founder final decision (YYYY-MM-DD):** APPROVED FOR PR / CHANGES REQUIRED.
I reviewed the independent human findings, adversarial evidence, cross-model
errata, all three raw analyzer outputs, and the recorded dispositions. The
approved artifact is commit `FULL_COMMIT`; no stronger claim than S2
durable-local developer-grade enforcement is made.
```

Use the actual final commit and reviewer identity. Do not use the example's
`APPROVED FOR PR` wording until every required row is complete.
