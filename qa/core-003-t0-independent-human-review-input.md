# CORE-003 T0 independent human review — input for completion

**Status:** Incomplete review input; not evidence until an independent human
reviewer completes and signs it

This record satisfies a different gate from founder authorship/sign-off. Under
DECI-0011, the T0 writer cannot be the sole human code reviewer. The reviewer
completing this record must not be the author of the founder-owned
`BeforeToolCallAdapter::before_tool_call` change. Codex and Claude reviews do
not satisfy this gate.

## Review identity and artifact binding

- **Independent reviewer:** `[FULL NAME]`
- **Reviewer relationship/role:** `[ROLE; CONFIRM NOT THE T0 WRITER]`
- **Review date/time (UTC):** `[YYYY-MM-DDTHH:MM:SSZ]`
- **Reviewed code commit:**
  `6cb6826fb29ee18bd2ce5f596c620f4170f37a47`
- **Baseline commit:** `4c7f6a33a5f0c01c42eed81b936a77450c8edd40`
- **Reviewed patch SHA-256:**
  `d689cfe6fc092b7cac1fcfea397e09288a169aa2be14c94583cff57eedc905d9`
- **Stable approval/review reference:** `[PR REVIEW URL, REVIEW ID, OR OTHER
  DURABLE EVIDENCE LOCATION]`

Suggested integrity command:

```bash
git diff --no-ext-diff --full-index --binary \
  4c7f6a33a5f0c01c42eed81b936a77450c8edd40 \
  6cb6826fb29ee18bd2ce5f596c620f4170f37a47 | sha256sum
```

Expected result:

```text
d689cfe6fc092b7cac1fcfea397e09288a169aa2be14c94583cff57eedc905d9  -
```

## Required review scope

Review the complete seven-file patch, not only the new match arm:

- `AGENTS.md`
- `CLAUDE.md`
- `T0-AUTHORS.md`
- `tests/bypass-rust/T0-BOUNDARY.md`
- `tests/bypass-rust/src/before_tool_call.rs`
- `tests/bypass-rust/tests/adapter_harness.rs`
- `tests/bypass-rust/tests/attack_set.rs`

## Independent code-review checks

- [ ] I am not the writer of the founder-owned T0 boundary behavior.
- [ ] I verified the full commit and patch digest above.
- [ ] I reviewed all seven changed files.
- [ ] The complete `guard.decide` invocation is contained by
  `catch_unwind(AssertUnwindSafe(...))`.
- [ ] A returned `Err(GuardFault)` and a caught Rust unwind both produce
  `Blocked(FailClosed)` with no authorization.
- [ ] Neither failure path calls the effectful tool.
- [ ] Returned `Ok(Deny)` and `Ok(Allow)` retain their established relay
  semantics; only the Allow path invokes the tool.
- [ ] The bounded `AssertUnwindSafe` rationale is acceptable: the current
  invocation neither inspects nor reuses the store after the caught unwind,
  and the code does not claim safety for a later invocation.
- [ ] The limitation to unwind-mode panics is explicit; `panic=abort`, process
  termination, OOM abort, and hook-never-fired remain outside this proof.
- [ ] The two dedicated ATK-07 tests cover typed fault and panic independently,
  derive `FailClosed` from the attack registry, and assert zero effectful
  invocations.
- [ ] No ignored or generic ATK-07 stub weakens or duplicates the dedicated
  evidence.
- [ ] Governance files accurately separate founder-owned T0 behavior from test
  plumbing and retain the RUNTIME-003/004 scope fence.
- [ ] I found no unrelated enforcement change or weakened adversarial
  expectation.

## Review findings

Record every finding, including non-blocking observations. Use `None` only if
the review found none.

```text
[FINDINGS, OR "None"]
```

## Independent disposition

Select exactly one:

- [ ] **PASS** — no blocking defect; the exact reviewed commit is acceptable.
- [ ] **PASS WITH NON-BLOCKING FINDINGS** — all findings are recorded above.
- [ ] **CHANGES REQUIRED** — do not treat CORE-003 as complete; record defects
  above and repeat affected gates on the replacement commit.
- [ ] **REJECT** — the boundary or evidence is unacceptable.

## Reviewer attestation

> I independently reviewed the complete seven-file CORE-003 T0 patch at commit
> `6cb6826fb29ee18bd2ce5f596c620f4170f37a47`. I am not the writer of the
> founder-owned boundary behavior. My selected disposition and findings above
> are my human code-review result for the DECI-0011 reviewer-separation gate.

- **Independent reviewer name:** `[FULL NAME]`
- **Signature/approval reference:** `[REFERENCE]`
- **Decision timestamp (UTC):** `[YYYY-MM-DDTHH:MM:SSZ]`
