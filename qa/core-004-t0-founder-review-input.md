# CORE-004 T0 founder review input

**Status:** Awaiting founder pre-authoring decisions  
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

- **Founder/T0 author:** `[FOUNDER TO AUTHOR]`
- **Decision date/time (UTC):** `[FOUNDER TO AUTHOR]`
- **Confirmed base commit:** `[FOUNDER TO AUTHOR]`
- **Confirmed scope:** timeout-only 6-A; no approve-to-allow path:
  `[FOUNDER TO AUTHOR]`
- **Confirmed claim fence:** single guard instance and local pending store under
  a modeled clock; no real human delivery/wait, cross-instance state, live
  restart guarantee, or deployed-runtime non-bypassability:
  `[FOUNDER TO AUTHOR]`

## B. Decisions required before Rust authoring

### D1 — pending-record correlation key

Recommended shape: retain verified `key_id`, nonce, and action commitment in
the pending record. Treat `(key_id, nonce)` as the presentation identity and
require its stored action commitment to match on re-presentation. A mismatch
fails closed; it never creates a second request or extends a deadline.

- **Founder selection:** `[FOUNDER TO AUTHOR]`
- **Rationale and rejected alternative:** `[FOUNDER TO AUTHOR]`

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

- **Founder selection:** `[FOUNDER TO AUTHOR]`
- **Rationale and collision/domain-separation disposition:**
  `[FOUNDER TO AUTHOR]`

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

- **Founder selection and exact operation signatures:** `[FOUNDER TO AUTHOR]`
- **Rationale:** `[FOUNDER TO AUTHOR]`

### D4 — SQLite integer and schema domain

Recommended shape: a STRICT table; fixed-length BLOB validation for identifiers
and commitments; `requested_at` and `deadline` stored as SQLite `INTEGER` only
after checked `u64 -> i64` conversion; constrained status values
`requested | denied_on_timeout`; `synchronous=FULL`; transactional insert,
deduplication, and timeout transition.

Conversion or row-shape failure returns a closed fault. TTL, row deletion, and
record absence are not timeout signals.

- **Founder selection:** `[FOUNDER TO AUTHOR]`
- **Rationale and exact uniqueness constraints:** `[FOUNDER TO AUTHOR]`

### D5 — explicit dependency injection into the guard/adapter

The current `GuardDecisionPort::decide` receives only a `ConsumptionStore`.
CORE-004 also needs the founder-owned approval store. Recommended shape: pass
both mutable ports explicitly through the guard and `before_tool_call` adapter;
do not hide either in a global or construct SQLite inside a decision.

The founder may instead select a small founder-owned composite port container,
provided lifetimes remain explicit and tests can inject both stores.

- **Founder selection and exact signature:** `[FOUNDER TO AUTHOR]`
- **Rationale and migration plan for existing tests:** `[FOUNDER TO AUTHOR]`

### D6 — fault and missing-record outcomes

Recommended shape: pending-store operational errors, checked-arithmetic
failure, malformed rows, ID mismatch, commitment mismatch, and missing record
all produce an explicit fail-closed observation. None may produce Allow,
`Escalated`, a fresh deadline, or a tool invocation.

- **Founder selection and denial signals:** `[FOUNDER TO AUTHOR]`
- **Rationale:** `[FOUNDER TO AUTHOR]`

## C. Frozen implementation invariants

The founder confirms each invariant before authoring:

- [ ] Signature, lifetime/expiry, and action binding remain ahead of the
  escalation check.
- [ ] Amount canonical validation occurs before comparison.
- [ ] Escalation is strictly `amount > 1_000_000` minor units.
- [ ] Comparison handles arbitrarily long canonical integers without overflow
  into Allow.
- [ ] Deadline uses checked `requested_at + 86_400` from the trusted injected
  clock and is written once.
- [ ] Durable pending commit precedes the `Escalated` observation.
- [ ] Escalation never consumes the capability nonce.
- [ ] Re-presentation returns the original review ID and deadline.
- [ ] `now <= deadline` re-observes that same escalation.
- [ ] Only `now > deadline` durably transitions to the registry-derived
  `EscalateThenDenyOnTimeout` block.
- [ ] No escalation or timeout branch issues authorization or invokes the
  effectful probe.
- [ ] The existing amount `100000` path remains consume-then-Allow.

## D. Planned founder-authored files

Record the final file/function ownership before editing:

| Planned file | Founder-owned responsibility | Founder confirmation |
|---|---|---|
| `tests/bypass-rust/src/founder_approval_store.rs` | Approval port and consequential outcome types | `[FOUNDER TO AUTHOR]` |
| `tests/bypass-rust/src/founder_s2_approval_store.rs` | Durable-local SQLite pending state and atomic timeout transition | `[FOUNDER TO AUTHOR]` |
| `tests/bypass-rust/src/founder_authored_guard.rs` | Canonical amount rule, threshold, escalation ordering, deadline, pending write | `[FOUNDER TO AUTHOR]` |
| `tests/bypass-rust/src/before_tool_call.rs` | Shared T0 decision/observation variants and adapter relay | `[FOUNDER TO AUTHOR]` |
| `[FOUNDER-SELECTED TIMEOUT FILE]` | Token-free R-3 evaluation and fail-closed mapping | `[FOUNDER TO AUTHOR]` |
| `tests/bypass-rust/src/lib.rs` | Mechanical module exports only | `[FOUNDER TO AUTHOR]` |

## E. Post-authoring exact-commit gate — complete later

- **Baseline commit:** `7324cbb33be59595657a2df13c300aa388208d77`
- **Reviewed T0 commit:** `[AFTER AUTHORING]`
- **Patch SHA-256:** `[AFTER AUTHORING]`
- **Adversarial/regression evidence:** `[AFTER AUTHORING]`
- **Independent human reviewer and stable reference:** `[AFTER AUTHORING]`
- **Cross-model bundle/disposition:** `[AFTER AUTHORING]`
- **Semgrep raw evidence and founder disposition:** `[AFTER AUTHORING]`
- **CodeQL raw evidence and founder disposition:** `[AFTER AUTHORING]`
- **cargo-deny raw evidence and founder disposition:** `[AFTER AUTHORING]`
- **Founder final decision:** `[AFTER AUTHORING]`

ATK-06 stays ignored until the reviewed T0 implementation and mechanical
test wiring make the dedicated two-surface contract green. The generic
terminal no-token placeholder must never be used as completion evidence.
