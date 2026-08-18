# CORE-003 T0 cross-model review disposition

- **Reviewer:** Claude (independent cross-model review)
- **Review date:** 2026-08-18
- **Reviewed code commit:** `6cb6826fb29ee18bd2ce5f596c620f4170f37a47`
- **Baseline commit:** `4c7f6a33a5f0c01c42eed81b936a77450c8edd40`
- **Reviewed patch SHA-256:**
  `d689cfe6fc092b7cac1fcfea397e09288a169aa2be14c94583cff57eedc905d9`
- **Reviewed bundle SHA-256:**
  `b457be430192665cb9a180423c9ab0eafba6e428f7b1529a9a1ff41b816df8c8`
- **Verdict:** **PASS — source verified; no code defect found**

## Integrity binding

Claude reviewed the founder-authored source contained in the post-authoring QA
bundle, not the earlier pre-authoring adapter. Before this record was created,
the exact seven-file snapshot was committed as `6cb6826f...`. The complete
baseline-to-commit binary-capable patch hashes to the same
`d689cfe6...` digest reviewed by Claude. The cross-model review therefore binds
to the code commit above.

## Confirmed properties

1. `BeforeToolCallAdapter::before_tool_call` contains the complete
   `GuardDecisionPort::decide` call inside
   `catch_unwind(AssertUnwindSafe(...))`.
2. An unwind from `guard.decide` is contained and becomes a normal
   `BeforeToolCallObservation::Blocked` result.
3. The bounded `AssertUnwindSafe` comment correctly states that the store is
   not inspected or reused within the failing invocation and does not certify
   later reuse.
4. `Ok(Err(GuardFault))` and a caught unwind converge on the same fixed
   fail-closed observation: `RequiredOutcome::FailClosed`, no authorization,
   and no effectful invocation in the tested call.
5. The panic payload is discarded; it is not inspected, exposed, logged, or
   resumed.
6. Returned `Ok(Deny)` and `Ok(Allow)` relay behavior remains unchanged.
7. The source and boundary documentation state the unwind-only and
   hook-reached scope limits.
8. Both dedicated ATK-07 tests are active and green.

## Precision corrections recorded with the PASS

- The proven panic-containment statement is limited to an unwind from
  `guard.decide`. The generic test probe methods are outside the containment
  closure; this does not weaken the ATK-07 guard/verifier result.
- `effectful_invocations` is zero in the adversarial tests because each uses a
  fresh probe. The general source property is that the fault/panic branch makes
  no additional invocation.
- The obsolete generic `atk_07_hook_error` macro test must not be restored or
  un-ignored. It did not inject a hook failure and was correctly removed in
  favor of the two dedicated tests.

## Scope limit

This PASS covers the reached isolation boundary when `decide` returns a typed
fault or raises a Rust unwinding panic. It does not claim coverage of
`panic=abort`, process termination, OOM abort, hook-never-fired, route-around,
plugin-missing, operator-bypass, or deployed runtime non-bypassability. Those
remain RUNTIME-003/004 integration scope.

## Remaining gate effect

This cross-model PASS satisfies the cross-model-review component only. It does
not replace founder/human review, scanner adjudication, founder approval, PR
approval, or human merge.
