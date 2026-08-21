# CORE-004 post-authoring state for independent QA

## Claimed outcome

Founder-authored CORE-004 code and mechanical T3 wiring implement the frozen
timeout-only two-surface contract at exact commit
`60febb08ac9c3e207d6f7a3563b6824374c5c93e`. Local tests and scanners have run.
No claim is made that independent-human review, cross-model disposition,
founder approval, PR approval, merge, or backlog closeout is complete.

## Local validation

- `cargo fmt --check`: PASS.
- `cargo check --locked --all-targets`: PASS.
- `cargo clippy --locked --all-targets -- -D warnings`: PASS.
- `cargo test --locked`: PASS — 52 passed, 5 unrelated/external ignored.
- `core_004_conformance`: 5 passed, 0 ignored.
- No ATK-06 test remains ignored.

## Scanner state

- Semgrep: 20/20, one INFO finding, zero scan errors.
- CodeQL: 20/20, nine fixture findings, zero extraction/execution errors.
- cargo-deny: exit 0, no warning/error.

All scanner findings remain for founder adjudication. Suggested text in the
founder form is not a decision.

## Current process state

The branch contains unsigned independent-human and founder forms. A human who
did not write the T0 implementation must complete the independent review.
Claude must review this bundle. The founder must then personally review source,
tests, both human/cross-model dispositions, and every raw scanner result before
selecting a final decision.

## Claim fence

The only claimed proof is the local timeout-only 6-A isolation contract. Real
human delivery, real waiting, approve-to-Allow, cross-instance state, runtime
route-around resistance, and deployed non-bypassability remain out of scope.
