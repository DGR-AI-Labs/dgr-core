# HOWTO: founder-author the CORE-004 T0 boundary in PyCharm

## Status and non-authorization

This is a reviewable authoring map, not implementation and not authorization
for an agent to write T0 Rust. The founder opens the dedicated T0 worktree only
after Addendum A, VAL-004, and the reviewed RED conformance tests have merged.

- Windows project path:
  `C:\Users\Khazret\Documents\.codex-worktrees\dgr-core004-t0-founder`;
- WSL path:
  `/mnt/c/Users/Khazret/Documents/.codex-worktrees/dgr-core004-t0-founder`;
- branch: `codex/core-004-t0-founder`;
- Cargo project: `tests/bypass-rust/Cargo.toml`.

Use the same WSL Rust toolchain and locked run configurations documented in
`specs/HOWTO-VAL-004-PYCHARM.md`. Never let PyCharm update `Cargo.lock`.

## Baseline insertion map

Line numbers below refer to dgr-core main commit
`21b85d9febbaf3090f30026c0cd5ee33c44f92e3`; re-resolve them after later
fixture/test merges.

| Piece | File and current anchor | Founder-authored change |
|---|---|---|
| T0-1 approval port | new `tests/bypass-rust/src/founder_approval_store.rs` | Pending identity/record types, record/evaluate outcomes, and fail-closed default trait methods |
| T0-2 durable store | new `tests/bypass-rust/src/founder_s2_approval_store.rs` | STRICT SQLite schema, atomic deduplication, immutable deadline, atomic timeout transition, persist-then-observe |
| T0-3 guard constants/trigger | `founder_authored_guard.rs`, after current lines 27–34 and between current binding lines 88–99 and consume line 101 | Window/threshold constants and mirrors; canonical positive-decimal validation; escalation after binding and before consume |
| T0-4 guard/adapter contract | `before_tool_call.rs`, current `GuardDecision` lines 27–35, port lines 46–53, observation lines 63–80, match lines 120–146 | Consequential `Escalate` decision, approval-store dependency, `Escalated` observation, no-invoke relay arm |
| T0-5 R-3 evaluator | founder-selected new module or explicitly named function | Evaluate an existing pending record by trusted clock without token presentation; re-observe pending or persist timeout then block |
| T0-6 module wiring | `tests/bypass-rust/src/lib.rs`, current lines 8–15 | Export only the founder-authored modules selected above |

No dependency change is expected: `rusqlite`, `sha2`, and the existing token
types are already present.

## Founder decisions to record before the first Rust edit

The observable contract is frozen, but the original design intentionally left
exact Rust signatures and schema shape to the founder. Record these choices in
the T0 review input before implementation:

1. **Pending correlation.** Recommended isolation shape: use the signed token
   nonce as the unique authorization reference, store the action commitment,
   and keep `review_request_id` as the primary timeout/evaluation identifier.
   A duplicate nonce must return the original record; a duplicate nonce paired
   with a different commitment must fail closed.
2. **Review-request id.** Choose either a deterministic domain-separated digest
   over verified token identity or an injected founder-owned id source. Never
   accept an agent-supplied id and never generate a new id on re-presentation.
3. **Atomic timeout transition.** Recommended store operation performs
   `Requested → DeniedOnTimeout` transactionally when `now > deadline` and
   returns the persisted record. Avoid a separate unchecked read then update.
4. **SQLite integer conversion.** Every `u64` timestamp conversion to SQLite's
   signed integer domain must be checked; conversion failure is a typed fault
   and cannot wrap, panic, or authorize.

These are T0 choices, not fixture decisions. If the founder selects a different
shape, update the ownership map and review checklist before coding.

## Suggested T0-1 port contract

Keep expected rejections distinct from operational faults. A useful review
shape is:

```rust
PendingKey { authorization_reference, action_commitment }
PendingApproval { review_request_id, requested_at, deadline, action_commitment }

RecordPendingOutcome = Recorded(PendingApproval)
                     | AlreadyPending(PendingApproval)
                     | Faulted(GuardFault)

EvaluatePendingOutcome = Pending(PendingApproval)
                       | TimedOut(PendingApproval)
                       | NotFound
                       | Faulted(GuardFault)
```

Default trait methods must return
`Faulted(GuardFault::FounderImplementationRequired)`. `AlreadyPending` must
carry the original id and deadline. `NotFound` is never authorization.

## Suggested T0-2 SQLite invariants

Use one persistent connection per store instance, `synchronous=FULL`, and a
STRICT table containing at least:

- immutable `review_request_id` primary key;
- unique signed authorization reference;
- action commitment;
- `requested_at` and immutable `deadline`;
- closed status domain containing `requested` and `denied_on_timeout`.

Insert/deduplicate and timeout transition must be transactional. Commit before
returning `Recorded`, `AlreadyPending`, or `TimedOut`. At `now <= deadline`,
return the stored pending record unchanged. At `now > deadline`, persist
`denied_on_timeout` before returning the terminal result. Re-evaluation of an
already timed-out row is idempotently timed out. Any SQL, constraint,
conversion, or commit error returns a fault.

## Suggested T0-3 guard ordering

Add private founder constants and read-only mirrors:

```rust
const APPROVAL_WINDOW_SECONDS: u64 = 86_400;
const APPROVAL_REQUIRED_ABOVE_MINOR_UNITS: u64 = 1_000_000;
```

The exact order is:

```text
signature → lifetime/expiry → binding + canonical amount validation
→ escalation check → consume nonce → allow
```

Validate `amount` as non-empty positive ASCII digits with no sign, separator,
decimal point, Unicode digit, or leading zero. Compare the mathematical
decimal value without overflow; a canonical value larger than machine range is
above threshold and escalates. Malformed values deny. Compute the deadline
with `checked_add`; failure faults closed.

For `amount > threshold`, record pending and return escalation without calling
`ConsumptionStore::consume`. `Recorded` and `AlreadyPending` both return the
stored id/deadline. Store fault returns fail closed. At/below threshold, retain
the existing consume/allow behavior exactly.

## Suggested T0-4 and T0-5 observations

Add one consequential guard decision/observation path carrying
`review_request_id` and `deadline`. The adapter branch must not call
`tool.invoke`; it emits `Escalated` only after the guard/store reports durable
success and records `authorization_issued: false` and the unchanged invocation
count.

The R-3 evaluator accepts an existing review-request id, trusted
`now_unix_seconds`, and the approval store—never a capability token. Map:

- `Pending` at `now <= deadline` → `Escalated` with the same id/deadline;
- durably `TimedOut` at `now > deadline` → `Blocked` with
  `EscalateThenDenyOnTimeout`;
- `NotFound`, store fault, arithmetic/conversion fault, or unwind → explicit
  fail-closed block;
- no branch issues authorization, consumes a nonce, or invokes a tool.

## T0 review sequence

Author and review in small pieces: port → durable store → guard trigger →
adapter observation → R-3 evaluator → conformance activation. After the exact
code commit is frozen, run adversarial tests, independent-human review,
cross-model review, Semgrep, CodeQL, cargo-deny, founder disposition, and human
merge. ATK-06 is un-ignored only after the reviewed RED tests turn green
without changing their registry-derived outcome.
