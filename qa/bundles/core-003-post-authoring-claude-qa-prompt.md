# Claude QA prompt — CORE-003 founder-authored T0 boundary

Perform an independent, review-only QA pass over the exact snapshot in this
bundle. Do not author, complete, refactor, or silently repair the T0 logic.

## Integrity first

1. Verify `BUNDLE-SHA256SUMS.txt`.
2. Confirm the baseline commit and authoring branch in `SNAPSHOT.txt`.
3. Compute the SHA-256 of `AUTHORING.patch` and require it to equal
   `d689cfe6fc092b7cac1fcfea397e09288a169aa2be14c94583cff57eedc905d9`.
4. Treat the bundled working-tree files as the review subject. They are an
   uncommitted founder-authored snapshot based on
   `4c7f6a33a5f0c01c42eed81b936a77450c8edd40`.

## Required code review

Review the complete diff and the full surrounding files. At minimum determine:

1. Whether an unwind from `guard.decide` can escape
   `BeforeToolCallAdapter::before_tool_call`.
2. Whether `tool.invoke` is reachable from any typed-fault or panic path.
3. Whether returned `Ok(Deny)` and `Ok(Allow)` retain their prior semantics.
4. Whether `Ok(Err(GuardFault))` and a caught unwind both return an explicit
   `Blocked` observation with `RequiredOutcome::FailClosed`, no authorization,
   and zero effectful invocation in the tested call.
5. Whether the panic payload is ignored and never inspected, exposed, or
   resumed.
6. Whether the `AssertUnwindSafe` explanation is properly bounded to the
   current invocation and avoids claiming that the store is safe for later
   reuse.
7. Whether the guarantee is correctly limited to Rust unwinding panics after
   the hook is reached, leaving hook-never-fired and route-around threats to
   RUNTIME-003/004.
8. Whether retaining the raw `BeforeToolCallObservation::GuardFault` variant as
   a negative-test sentinel creates any reachable fail-open or ambiguity.
9. Whether the fixed denial signal is appropriate and avoids leaking the panic
   payload or internal fault details.

## Required test and governance review

1. Trace both dedicated ATK-07 fake guards and tests end to end.
2. Confirm both tests are active, registry-derived, and reject `Proceeded`, a
   raw `GuardFault`, and an escaping panic.
3. Confirm the obsolete generic `atk_07_hook_error` macro invocation was
   correctly removed because it did not inject a hook failure.
4. Confirm the adapter regression test expects the new fail-closed floor while
   existing returned-Deny and returned-Allow tests remain unchanged.
5. Review `T0-AUTHORS.md`, `AGENTS.md`, `CLAUDE.md`, and `T0-BOUNDARY.md` for
   ownership, current-state, isolation-harness, and scope-fence consistency.
6. Reconcile the implementation with ATK-07, DECI-0006, ADR-11, and SRS-07.
7. Confirm that six remaining ignored attack tests are unrelated deferred or
   hosted cases and that no ATK-07 case remains ignored.

## Required output

Lead with exactly one verdict:

- `PASS FOR COMMIT AND T0 GATES`
- `PASS WITH NON-BLOCKING NOTES`
- `CHANGES REQUIRED`
- `BLOCKED — INTEGRITY OR EVIDENCE FAILURE`

Then provide findings from highest to lowest severity with exact bundle paths
and line numbers. Separate code/security defects from documentation, test, and
pre-commit evidence issues. Explicitly state whether each of the nine code
questions and seven test/governance questions passed.

End with:

- the reviewed baseline and patch SHA-256;
- the exact corrections, if any, that the founder must author;
- whether the snapshot is safe to commit; and
- the remaining exact-commit gates: human review, cross-model disposition,
  Semgrep, CodeQL, cargo-deny, founder approval, PR review, and human merge.
