# CORE-004 T0 founder review input

**Status:** T0 authoring and exact-commit review gates complete — PR review and merge pending
**Founder T0 base:** `7324cbb33be59595657a2df13c300aa388208d77`  
**Branch:** `codex/core-004-t0-founder`

This is an agent-prepared input form, not founder authorship, approval, or T0
implementation. The founder must personally write each selection and rationale
below before the first enforcement-code edit. Do not replace a selection with
an inferred default.

The controlling contracts are `CORE-004-DESIGN`, Addendum A,
`T0-AUTHORS.md`, `tests/bypass-rust/T0-BOUNDARY.md`, and the merged RED tests
from dgr-core PR #80.

## A. Pre-authoring identity and scope

- **Founder/T0 author:** `Khazretgali Sapen`
- **Decision date/time (UTC):** `2026-08-20T22:20:00Z`
- **Confirmed base commit:** `7324cbb33be59595657a2df13c300aa388208d77`
- **Confirmed scope:** timeout-only 6-A; no approve-to-allow path:
  `Confirmed — I accept timeout-only 6-A. CORE-004 implements no approve-to-allow path.`
- **Confirmed claim fence:** single guard instance and local pending store under
  a modeled clock; no real human delivery/wait, cross-instance state, live
  restart guarantee, or deployed-runtime non-bypassability:
  `Confirmed — I accept this bounded isolation claim and the listed runtime deferrals.`

## B. Decisions required before Rust authoring

### D1 — pending-record correlation key

Recommended shape: retain verified `key_id`, nonce, and action commitment in
the pending record. Treat `(key_id, nonce)` as the presentation identity and
require its stored action commitment to match on re-presentation. A mismatch
fails closed; it never creates a second request or extends a deadline.

- **Founder selection:** `Accepted. Pending identity is (key_id: [u8; 16], nonce: [u8; 16]); the record also stores action_commitment: [u8; 32]. Same identity and commitment returns the original pending record. A different commitment fails closed without creating or extending a request.`
- **Rationale and rejected alternative:** `Including key_id avoids aliasing identical nonces issued by different trusted keys. I reject nonce-only correlation and agent-supplied correlation identifiers.`

### D2 — review-request identifier

Recommended shape: a founder-owned `[u8; 32]` newtype derived deterministically
as:

```text
SHA256(
  "DGR-CORE004-REVIEW-V1\0" ||
  verified_token.key_id ||
  verified_token.nonce ||
  verified_token.action_commitment
)
```

The agent cannot supply it, and re-presentation cannot regenerate a different
identifier. The VAL-004 string is an opaque fixture label, not a required
production representation.

- **Founder selection:** `Accepted. ReviewRequestId is a founder-owned [u8; 32] newtype equal to SHA-256("DGR-CORE004-REVIEW-V1\0" || key_id || nonce || action_commitment).`
- **Rationale and collision/domain-separation disposition:**
  `The domain tag prevents cross-protocol reuse, and verified immutable fields make the ID deterministic and agent-independent. If an existing review ID maps to different stored identity fields, the store returns an internal fail-closed fault and never overwrites the row.`

### D3 — approval-store operations and atomic timeout transition

Recommended minimum port:

1. record the first pending request or return the original record as
   `AlreadyPending`;
2. atomically evaluate an existing record against trusted `now`;
3. return the unchanged pending record for `now <= deadline`;
4. transactionally persist `Requested -> DeniedOnTimeout` before returning the
   timed-out record for `now > deadline`; and
5. distinguish `NotFound` and operational `Faulted(GuardFault)` from expected
   `AlreadyPending`.

Do not expose a read-then-unchecked-update timeout sequence.

- **Founder selection and exact operation signatures:** `Accepted. ApprovalStore exposes fn record_pending(&mut self, pending: PendingApproval) -> RecordPendingOutcome and fn evaluate_pending(&mut self, review_request_id: &ReviewRequestId, now_unix_seconds: u64) -> EvaluatePendingOutcome. Outcomes are Recorded/AlreadyPending/Faulted and Pending/TimedOut/NotFound/Faulted. evaluate_pending performs and commits Requested -> DeniedOnTimeout atomically when now > deadline; an already-timed-out record returns TimedOut idempotently.`
- **Rationale:** `One store operation owns the trusted-clock comparison, transition, commit, and returned record, preventing a read/update race and ensuring persist-before-observe behavior.`

### D4 — SQLite integer and schema domain

Recommended shape: a STRICT table; fixed-length BLOB validation for identifiers
and commitments; `requested_at` and `deadline` stored as SQLite `INTEGER` only
after checked `u64 -> i64` conversion; constrained status values
`requested | denied_on_timeout`; `synchronous=FULL`; transactional insert,
deduplication, and timeout transition.

Conversion or row-shape failure returns a closed fault. TTL, row deletion, and
record absence are not timeout signals.

- **Founder selection:** `Accepted. Use a STRICT SQLite table with synchronous=FULL, checked u64-to-i64 conversion, fixed-length BLOB checks, constrained status, transactional insertion, deduplication, and timeout transition.`
- **Rationale and exact uniqueness constraints:** `review_request_id is the 32-byte primary key; UNIQUE(key_id, nonce) identifies re-presentation; key_id and nonce must be 16 bytes and action_commitment 32 bytes. requested_at and deadline are non-negative INTEGER values with deadline >= requested_at. Status is restricted to requested or denied_on_timeout. Conversion, malformed-row, or uniqueness-invariant failure fails closed.`

### D5 — explicit dependency injection into the guard/adapter

The current `GuardDecisionPort::decide` receives only a `ConsumptionStore`.
CORE-004 also needs the founder-owned approval store. Recommended shape: pass
both mutable ports explicitly through the guard and `before_tool_call` adapter;
do not hide either in a global or construct SQLite inside a decision.

The founder may instead select a small founder-owned composite port container,
provided lifetimes remain explicit and tests can inject both stores.

- **Founder selection and exact signature:** `Accepted. GuardDecisionPort::decide receives request, trusted now, &mut dyn ConsumptionStore, and &mut dyn ApprovalStore as explicit arguments. BeforeToolCallAdapter::before_tool_call receives and passes both stores explicitly. No global store or decision-time SQLite construction is permitted.`
- **Rationale and migration plan for existing tests:** `All GuardDecisionPort implementations and adapter calls will add the approval-store argument. Conformance tests will inject an in-memory S2 approval store or a test-only fake after the founder port exists. Existing below-threshold tests must prove the approval store is not used and retain the current consume/allow result.`

### D6 — fault and missing-record outcomes

Recommended shape: pending-store operational errors, checked-arithmetic
failure, malformed rows, ID mismatch, commitment mismatch, and missing record
all produce an explicit fail-closed observation. None may produce Allow,
`Escalated`, a fresh deadline, or a tool invocation.

- **Founder selection and denial signals:** `Malformed canonical amount returns Deny with "CORE-004 non-canonical amount". Approval-store Unavailable/InternalError and checked-arithmetic or row-invariant failures use the existing fail_closed_decision mapping. Missing pending state returns FailClosed with "CORE-004 pending approval not found". A committed timeout returns the registry-derived EscalateThenDenyOnTimeout outcome with "ATK-06 approval timed out".`
- **Rationale:** `Expected malformed input is denied distinctly; operational or invariant uncertainty fails closed; only a durably committed timeout receives the ATK-06 terminal outcome. No failure creates Escalated, resets a deadline, consumes a nonce, authorizes, or invokes the tool.`

## C. Frozen implementation invariants

The founder confirms each invariant before authoring:

- [x] Signature, lifetime/expiry, and action binding remain ahead of the
  escalation check.
- [x] Amount canonical validation occurs before comparison.
- [x] Escalation is strictly `amount > 1_000_000` minor units.
- [x] Comparison handles arbitrarily long canonical integers without overflow
  into Allow.
- [x] Deadline uses checked `requested_at + 86_400` from the trusted injected
  clock and is written once.
- [x] Durable pending commit precedes the `Escalated` observation.
- [x] Escalation never consumes the capability nonce.
- [x] Re-presentation returns the original review ID and deadline.
- [x] `now <= deadline` re-observes that same escalation.
- [x] Only `now > deadline` durably transitions to the registry-derived
  `EscalateThenDenyOnTimeout` block.
- [x] No escalation or timeout branch issues authorization or invokes the
  effectful probe.
- [x] The existing amount `100000` path remains consume-then-Allow.

## D. Planned founder-authored files

Record the final file/function ownership before editing:

| Planned file | Founder-owned responsibility | Founder confirmation |
|---|---|---|
| `tests/bypass-rust/src/founder_approval_store.rs` | Approval port and consequential outcome types | `Confirmed — founder-owned T0` |
| `tests/bypass-rust/src/founder_s2_approval_store.rs` | Durable-local SQLite pending state and atomic timeout transition | `Confirmed — founder-owned T0` |
| `tests/bypass-rust/src/founder_authored_guard.rs` | Canonical amount rule, threshold, escalation ordering, deadline, pending write | `Confirmed — founder-owned T0` |
| `tests/bypass-rust/src/before_tool_call.rs` | Shared T0 decision/observation variants and adapter relay | `Confirmed — founder-owned T0` |
| `tests/bypass-rust/src/founder_approval_timeout.rs` | Token-free R-3 evaluation and fail-closed mapping | `Confirmed — founder-owned T0` |
| `tests/bypass-rust/src/lib.rs` | Mechanical module exports only | `Confirmed — mechanical exports only; no independent behavior` |

## E. Post-authoring exact-commit gate

- **Baseline commit:** `7324cbb33be59595657a2df13c300aa388208d77`
- **Reviewed T0 commit:** `60febb08ac9c3e207d6f7a3563b6824374c5c93e`
- **Patch SHA-256:** `71f051e24055cb0febd620d84a2703ea43d7277f5af4266feef8d03d0fbb9f1f`
  (`git diff --binary --full-index 7324cbb33be59595657a2df13c300aa388208d77..60febb08ac9c3e207d6f7a3563b6824374c5c93e`)
- **Adversarial/regression evidence:** `qa/core-004-t3-green-report.md` — local
  Rust gates pass; independent-human, cross-model, three-engine SAST/SCA, and
  founder dispositions are recorded below.
- **Independent human reviewer and stable reference:** Gaziz Nugmanov — PASS;
  `qa/core-004-t0-independent-human-review-input.md`; [dgr-core PR #81](https://github.com/DGR-AI-Labs/dgr-core/pull/81)
- **Cross-model bundle/disposition:** bundle SHA-256
  `e23527b6adbdd7c8431cacfdb3ffc0ed682b33ee765f04f6febce380986b2bd0`;
  `qa/core-004-post-authoring-claude-qa-disposition.md` — CONFIRMED, ready for
  human gates.
- **Semgrep raw evidence and founder disposition:**
  `qa/sast/core-004-t0-semgrep-2026-08-21.txt` and `.json`; ACCEPT TEST-ONLY in
  `qa/core-004-t0-founder-review-draft.md`.
- **CodeQL raw evidence and founder disposition:**
  `qa/sast/core-004-t0-codeql-2026-08-21.txt` and `.sarif`; ACCEPT TEST FIXTURES
  WITH DIAGNOSTIC LIMITATION in `qa/core-004-t0-founder-review-draft.md`.
- **cargo-deny raw evidence and founder disposition:**
  `qa/sast/core-004-t0-cargo-deny-2026-08-21.txt`; ACCEPT in
  `qa/core-004-t0-founder-review-draft.md`.
- **Founder final decision:** APPROVE at `2026-08-21T17:19:00Z`;
  `qa/core-004-t0-founder-review-draft.md`; [dgr-core PR #81](https://github.com/DGR-AI-Labs/dgr-core/pull/81).

ATK-06 is active in the isolated suite at the reviewed commit: the dedicated
two-surface target has five passing tests and no ignored cases. That local green
state did not satisfy the human, cross-model, or SAST/SCA gates by itself; those
gates and their dispositions are now recorded above. PR approval and merge
remain pending.
