# Claude QA prompt — final CORE-003 closeout

Perform an independent, source-grounded final QA of CORE-003 using only this
bundle. This is a review task: do not author, silently repair, or propose a
broader runtime guarantee than the included evidence supports.

## Integrity first

1. Verify `MANIFEST.sha256` from the bundle root before relying on any file.
2. Confirm the commit lineage in `metadata/commit-lineage.txt`:
   - reviewed/founder-authored implementation:
     `6cb6826fb29ee18bd2ce5f596c620f4170f37a47`;
   - PR #73 merge:
     `50347fe169e1207146ffe7a111669cddcd22c664`;
   - review-evidence PR #74 merge:
     `6db4761f42c79ccd757bcef9726466aef6610776`;
   - final evidence-status PR #75 merge / bundled dgr-core main:
     `4aeeceba24a353c399b09054bc84ea4ab84a55ba`;
   - bundled dgr-backlog main:
     `7f5771108abe6540f208a6e89799d0258dfb1eb4`.
3. Verify the baseline-to-implementation patch SHA-256 remains
   `d689cfe6fc092b7cac1fcfea397e09288a169aa2be14c94583cff57eedc905d9`.
4. Use `metadata/code-drift-after-reviewed-commit.txt` to confirm that no Rust,
   enforcement, adversarial-test, manifest, lockfile, or policy file changed
   after the reviewed implementation commit.

## Required review

Review the actual source, tests, raw evidence, signed dispositions, governing
references, and backlog records. Determine whether all of the following hold:

1. `BeforeToolCallAdapter::before_tool_call` contains the complete
   `guard.decide` invocation with `catch_unwind(AssertUnwindSafe(...))`.
2. Both a returned `Err(GuardFault)` and a caught Rust unwind produce
   `Blocked(FailClosed)`, issue no authorization, and cannot invoke the
   effectful probe.
3. Returned `Ok(Deny)` and `Ok(Allow)` retain their established relay
   semantics, and only Allow invokes the probe.
4. The bounded `AssertUnwindSafe` rationale is honest: it covers the current
   invocation only and does not certify later reuse after a panic.
5. Both dedicated ATK-07 tests are active, derive the expected outcome from the
   registry, reject raw faults/escaped panics, and assert zero effectful
   invocations. Confirm no obsolete generic ATK-07 stub was restored.
6. Fresh final-main validation and the three exact-commit SAST/SCA artifacts
   are complete and internally consistent. Review the founder's explicit
   dispositions of the one Semgrep test-only temp-dir finding, seven CodeQL
   deterministic fixture-nonce findings, and the clean cargo-deny result.
7. The DECI-0011 reviewer-separation model is satisfied by founder sign-off
   plus Gaziz Nugmanov's signed independent-human PASS. Do not represent this
   as a GitHub review approval: GitHub exposes no submitted review event for
   PR #73, and the repository records that timing/evidence distinction.
8. CORE-003, CORE-003-T3-tests, and CORE-003-T0-boundary are accurately Done in
   the canonical backlog, with links to the implementation, merges, and signed
   evidence.

## Scope guardrail

CORE-003 proves fail-closed behavior only after this Rust isolation boundary is
reached and only for typed guard faults and Rust unwind-mode panics. It does not
prove behavior for `panic=abort`, process termination, OOM abort, a hook that is
never invoked, route-around access, a missing plugin, operator bypass, or
deployed-runtime non-bypassability. Those runtime-integration claims remain in
RUNTIME-003/004. Treat any broader claim as a defect.

## Required response

Return:

1. **Verdict:** exactly one of `CONFIRMED — READY FOR NEXT STEP`,
   `CONDITIONAL — ACTIONS REQUIRED`, or `BLOCKED`.
2. **Integrity:** manifest result, exact commits, patch digest, and drift result.
3. **Source correctness:** fault path, panic path, Allow/Deny relay, and unwind
   scope.
4. **Tests and validation:** active ATK-07 evidence and final-main results.
5. **SAST/SCA:** coverage, findings, and whether the dispositions are adequate.
6. **Human/process evidence:** founder and independent-human records, including
   the post-merge publication timing.
7. **Backlog and scope:** whether Done is accurate and runtime deferrals remain
   explicit.
8. **Findings/actions:** every defect, ambiguity, or confirmation still needed;
   write `None` only if there truly are none.

Do not infer that the next step is authorized merely because code is merged.
Your verdict should answer whether CORE-003 itself is complete enough for the
founder to select and begin the next separately scoped backlog item.
