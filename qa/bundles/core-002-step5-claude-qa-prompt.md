# Claude QA prompt — CORE-002 Step 5 final evidence pass

Perform an independent, evidence-first QA pass over this bundle. Do not assume
that an analyzer exit code, a prior PASS, or this prompt proves the gate.

## Required checks

1. Confirm the founder-authored T0 implementation at
   `0727e327631b475990ef8d9b7ef3b2c3554050a8` is unchanged in Rust through the
   final scanned commit `0a54d4995d1b9d98ab8a3ec61861fe2fe7ae29c3`.
2. Re-review the five founder-owned units for non-bypassability, fail-closed
   behavior, signature/time/request binding, record-before-allow ordering,
   atomic single-use consumption, and the stated S2 durable-local boundary.
3. Verify ATK-03 and ATK-13 evidence, restart durability on the same SQLite
   file, concurrent same-file consumption, zero tool invocation on denial, and
   the accuracy of all seven ignored-case classifications.
4. Verify every Semgrep, CodeQL, and cargo-deny artifact is bound to the full
   scanned commit and records a real execution, complete target coverage where
   applicable, tool version, command, exit code, and raw output.
5. Independently assess the one Semgrep temp-directory finding, the seven
   CodeQL deterministic-nonce findings, and the cargo-deny policy/notes. Do not
   accept the suggested founder dispositions merely because they are written.
6. Review `deny.toml` for overly broad licenses, registries, git sources, bans,
   wildcard dependencies, duplicate-dependency skips, or advisory ignores.
7. Check cross-repository state: DECI-0011 resolves FND-7; VAL-001 names
   cargo-deny but remains To Do for CI wiring; CORE-002 Step 5 remains In Review
   until finding dispositions, independent human T0 review, founder sign-off,
   PR approval, and merge.
8. Confirm the protected checklist is still pending and that the bundle does
   not falsely claim an independent human review or founder approval.

## Reporting format

Lead with a verdict: `PASS TO HUMAN GATE`, `PASS WITH REQUIRED EDITS`, or
`BLOCKED`. Then list findings from highest to lowest severity with exact bundle
path and line. Distinguish code defects, evidence defects, policy defects,
documentation drift, and acceptable scoped risks. End with the exact remaining
founder and independent-reviewer actions. Do not mark Step 5 Done.

The five T0 Rust units and the protected checklist are review-only surfaces for
agents. Suggest changes precisely if needed, but do not rewrite them in the QA
response.
