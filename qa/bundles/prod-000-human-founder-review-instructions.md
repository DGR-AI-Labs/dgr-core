# PROD-000 remaining-gate instructions

The non-author cross-model gate is satisfied by
`qa/prod-000-cross-model-review-addendum.md`. Do not ask Claude to review the same executable commit
again unless a later change touches executable code, test expectations, scripts, Cargo inputs,
dependency policy, workflow behavior, or a claim that Claude relied on. N13–N15 were explicitly
non-blocking and have been addressed or bounded without executable changes.

## Step 1 — independent-human review

Reviewer eligibility: a human who did not author the PROD-000 implementation or remediation. The
founder may not substitute Codex or Claude for this person.

1. Give the reviewer this complete ZIP and ask them to verify `MANIFEST.sha256` first.
2. Direct them to `review/prod-000-independent-human-review-input.md`.
3. Require review of the entire `metadata/baseline-to-executable.diff`, not only the new floor.
4. Require line-by-line inspection of `metadata/r5-1-timeout-semantic.diff` and direct inspection of
   the ATK-06 equality assertion body; the name guard does not prove body integrity.
5. Require inspection of the raw Semgrep JSON, CodeQL SARIF including the complete diagnostic
   array, and cargo-deny output.
6. Require the reviewer to write their own identity, findings, verdict, rationale, attestation, and
   UTC timestamp. Prewritten suggested language is not a review.
7. Save the response unchanged as `qa/prod-000-independent-human-review.md`; do not overwrite the
   input template.
8. Commit that record and push it to PR #90. An agent may perform the mechanical commit/push only
   after the human supplies the completed record; the human remains its author/reviewer.

If the verdict is `CHANGES REQUIRED` or `REJECT`, stop. Do not begin founder approval. Any
executable remediation must repeat affected tests/scans and both independent reviews.

## Step 2 — founder line-by-line review and dispositions

Begin only after a passing independent-human record is committed.

1. Note the resulting pre-disposition PR head SHA.
2. Give the founder this same bundle plus the completed independent-human record.
3. Use `review/prod-000-founder-review-input.md` as the checklist.
4. The founder personally reviews every consequential line, removed branch, public item, five
   provenance templates, original Claude review, passing addendum, and human findings.
5. The founder explicitly dispositions:
   - the Semgrep temp-directory result;
   - all nine CodeQL deterministic-nonce results;
   - the complete CodeQL diagnostic array and external SARIF-binding limitation;
   - cargo-deny's two bans and 54 license notes;
   - N5–N8 and N13–N15; and
   - every line-level provenance classification.
6. Save the founder-completed record unchanged as `qa/prod-000-founder-review.md`; do not overwrite
   the input template.
7. Commit and push the founder record.

If the founder selects `CHANGES REQUIRED` or `REJECT`, stop and repeat affected gates.

## Step 3 — bind approval to the actual final head

A repository review record cannot contain its own commit SHA. Therefore:

1. after the founder record is committed, note the resulting final PR head;
2. wait for `Structural / governance check` and `Rust format / build / test` to pass on that head;
3. compare it with the founder-reviewed pre-disposition head and require only the completed review
   record and mechanical bundle metadata to differ;
4. the founder submits a GitHub **Approve** review, which GitHub binds to the actual final head;
5. record the GitHub review URL as the final-head approval reference; and
6. only the founder merges PR #90.

Any executable or policy change after independent-human review invalidates the docs-only drift
assumption and reopens affected gates. PROD-001 remains unauthorized until PR #90 is merged.

## Exact executable binding

- Baseline: `e9c8f585809c15d2464b3d45bc2ce26d716c8673`.
- Executable review input: `587585cf476431f078efe587c5dbcc052389cdad`.
- Executable tree: `89e5de51d23ead98d24bbbe1b4cd57db343b2dc4`.
- Patch SHA-256: `c08919d86a1f060cce9a05b3143140a5f011b9349f243475dad4f4ec1b40cf99`.
- Cross-model addendum SHA-256:
  `534ce4164067aef339b9f35a176de1b39e6f1573834e52cdc3d533fad7e634db`.
- Evidence/instruction head before this bundle: `e4fdda5969493a83e2b1b0bdffff26a837d999d1`.
