# CORE-004 Post-Authoring T0 Review — Claude Independent Disposition

## 1. Verdict

**CONFIRMED — READY FOR HUMAN GATES.**

No defect found. The timeout-only 6-A contract is implemented faithfully to the pinned design + Addendum A, the two-surface proof is real, and every failure mode I probed routes to deny/fault. This confirms only that the exact source/evidence package is ready for the required human gates — it does not mark CORE-004 Done or authorize merge.

## 2. Integrity

- **Manifest:** `MANIFEST.sha256` — **63/63 OK, 0 FAILED.**
- **Lineage:** confirms baseline `7324cbb…` (T3 RED merge), reviewed implementation `60febb0…`, evidence/template `0807e34…`. The intermediate history reads as a coherent founder-authored progression (port → durable record → atomic transition → non-effectful escalation → token-free evaluation → canonical amounts → deterministic facts → explicit injection → persist-before-observe → conformance activation).
- **Patch digest:** recomputed `71f051e24055cb0febd620d84a2703ea43d7277f5af4266feef8d03d0fbb9f1f` — **exact match** to the required value.
- **Drift:** `post-review-drift.txt` shows **no Rust, Cargo manifest, lockfile, or `deny.toml` change** after the reviewed commit. Post-review additions are docs/QA/SAST only (`T0-AUTHORS.md`, review forms, T3 reports, SAST artifacts, `T0-BOUNDARY.md`). Clean.
- **Pinned design blobs:** computed `a6b883ea…` (CORE-004-DESIGN) and `ef5a19e9…` (ADDENDUM-A) — **both match** the reference-contract table exactly, at pinned commits `fd0329d4…` / `e2649b03…`.

## 3. Source correctness — all ten items pass

1. **Ordering ✓** — signature → lifetime/expiry → binding (commitment recompute) → canonical amount validation → escalation trigger. The amount check sits *after* the commitment comparison, so escalation is only ever evaluated on a verified, hash-committed amount.
2. **Amount comparison ✓ — and notably well done.** `canonical_amount_requires_approval` never parses to an integer: it validates ASCII-digits/non-empty/no-leading-zero, then compares **length first, lexicographic second**. That is numerically correct for canonical decimals and **structurally immune to overflow** — a 100-digit amount cannot wrap into a below-threshold Allow. Strictly above (`amount > threshold`), so exactly `1_000_000` does not escalate, per Addendum A. Non-canonical → `None` → Deny (`"CORE-004 non-canonical amount"`), never Allow.
3. **Review-ID derivation ✓** — SHA-256 over a domain tag (`DGR-CORE004-REVIEW-V1\0`) plus the **verified** key_id, nonce, and action_commitment. Agent-independent (derived from verified token fields, not request-supplied), domain-separated, 32 bytes.
4. **Deadline / ordering ✓** — `requested_at.checked_add(APPROVAL_WINDOW_SECONDS)`, fault on `None`. `record_pending` commits **before** `Escalate` is returned (persist-then-observe), and the `Escalate` return occurs **before** `consumption_store.consume(...)` — the nonce is not consumed on the escalation path.
5. **First-write / AlreadyPending ✓** — `same_committed_request` requires review_request_id **and** key_id **and** nonce **and** action_commitment to match; any mismatch → `Faulted(InternalError)`, not a substituted record. `AlreadyPending` returns the **existing stored** `PendingApproval` (original deadline), so the deadline cannot be extended. A review_request_id collision with a different identity → fault.
6. **Schema / transactions ✓** — `STRICT` table, `PRIMARY KEY` on review_request_id, `UNIQUE(key_id, nonce)`, and `CHECK` constraints pinning all blob lengths (32/16/16/32), `requested_at >= 0`, `deadline >= requested_at`, and status ∈ {`requested`,`denied_on_timeout`}. Both operations use `TransactionBehavior::Immediate` and commit explicitly; observation is returned only after a successful commit (`commit_evaluation`). The timeout transition is a single atomic `UPDATE … WHERE status='requested' AND deadline < ?now`, and a non-1 row count → fault.
7. **R-3 evaluator ✓** — token-free (`evaluate_approval_timeout` takes store + review id + clock only; no token parameter exists). `now <= deadline` → `Escalate{same id, same deadline}`; only a **committed** `now > deadline` transition returns the terminal denial, whose outcome is **registry-derived** via `attack_by_id("ATK-06").expected`. **Boundary double-checked:** the Rust guard (`now_unix_seconds <= deadline` → Pending) and the SQL predicate (`deadline < ?now`) agree, both encoding *timed out iff now > deadline*.
8. **Failure modes ✓** — `NotFound` → `Deny{FailClosed}`; `Faulted` → `fail_closed_decision`; identity mismatch → fault; `i64`/`u64` conversion failures → `InternalError`; malformed status/blob → `InternalError`; SQLite errors classified (busy/locked/OOM/IO/disk-full/etc → `Unavailable`, else `InternalError`). **No Allow or fresh-escalation route from any of them.**
9. **`Escalate` non-effectful ✓** — new `GuardDecision::Escalate` and `BeforeToolCallObservation::Escalated` variants; the adapter relays it without invoking the probe (invocation occurs only on the Allow arm), and the CORE-003 fail-closed floor and its bound remain intact in the amended contract comment.
10. **No approve-to-Allow path ✓** — grep of the implementation finds no approval-grant transition; `Allow` remains reachable only via the unchanged consume path. 6-A holds.

## 4. Tests

All five dedicated tests are **active** (no `#[ignore]`) in `core_004_conformance.rs`, and they exercise the real founder guard, the real `S2ApprovalStore`, and the real token-free evaluator (not mocks):
- `below_threshold_control_retains_the_existing_consume_and_allow_path`
- `atk_06_above_threshold_before_tool_call_requires_escalation`
- `atk_06_sequence_is_escalated_then_registry_derived_timeout_block`
- `atk_06_timeout_boundary_preserves_id_and_deadline_until_strictly_after`
- `atk_06_re_presentation_keeps_original_pending_facts_and_unconsumed_nonce`

Together these prove the ordered two-surface sequence, the strict boundary, id/deadline stability, the unconsumed nonce, zero effects, and the unchanged below-threshold path. **No generic terminal no-token block is used as a substitute** — the registry macro case at `attack_set.rs:597` is a registration/target assertion, not a stand-in for the two-surface proof. Expectations are registry-derived, not weakened.

## 5. SAST / SCA

- **Semgrep 1.173.0** — 20/20 files, **one INFO**: `temp-dir` in a test (restart-durability helper). Test-only; disposition adequate. Touches no founder-owned CORE-004 code.
- **CodeQL 2.25.5 / Rust 0.1.35** — 20/20 files, **nine** `hard-coded-cryptographic-value` findings, all deterministic **fixture nonces** in test data; zero extraction/execution errors. **Seven path-resolution-inconsistency diagnostics** are tool-side (analyzer path normalization), not code defects — adequately bounded, but worth one line in the founder disposition noting they were reviewed and are diagnostic-only.
- **cargo-deny 0.20.2** — exit 0, no warnings/errors; two documented duplicate-version skips; **advisory-ignore list empty** (nothing vulnerable suppressed).
- **Net:** every finding is test-only or tool-diagnostic. **None touches founder-owned CORE-004 enforcement; none requires remediation.** The draft dispositions are adequately bounded.

## 6. Human / process boundary

The independent-human and founder forms are **deliberately unsigned** — I treat neither gate as complete and none of the suggested wording as a decision. Assessing whether they ask the right questions: yes, and they cover the items that matter here (ordering, amount canonicalization, id derivation, persist-before-observe, nonce non-consumption, deadline immutability, atomic transition, no-approve-path, bounded claim).

**One addition I'd suggest to the human form** (not a defect): an explicit question on the **enforcement-reads-registry** point — `evaluate_approval_timeout` derives the ATK-06 terminal outcome from `attack_by_id("ATK-06").expected` at runtime. This is *sanctioned* (the registry's own doc states enforcement may consume but not redefine these cases) and it prevents code/test drift, but it does mean the terminal outcome depends on the integrity of a table that lives in an agent-authorable file. Worth a conscious human confirmation rather than passing silently — either affirm the sanction or pin the outcome T0-side with a conformance assertion, as the temporal constants do.

## 7. Backlog / scope

Backlog correctly still shows CORE-004 work in progress; I make **no Done recommendation** — implementation, evidence, independent-human review, cross-model disposition, founder sign-off, PR approval, and merge must all be recorded first. Deferrals preserved and unbroadened: real human delivery/waiting, approve-to-Allow (6-B, requiring its own approval-authenticity design), cross-instance state, deployed-runtime route-around, live non-bypassability — including **RUNTIME-006** and the future **ATK-05 reuse** of this same escalation path/port/store/observation.

The constants carry `CONFORMANCE_*` mirrors (`APPROVAL_REQUIRED_ABOVE_MINOR_UNITS`, `APPROVAL_WINDOW_SECONDS`, `REVIEW_REQUEST_ID_DOMAIN`) with dependency pointing test → T0, and the threshold sits above the VAL-002 baseline so the existing suite is unaffected.

## 8. Findings / actions

**No defects. No required actions.** Three non-blocking items for the founder record:

- **(non-blocking, recommend confirming)** Enforcement derives the ATK-06 outcome from the CORE-001 registry at runtime (§6 above). Sanctioned and drift-preventing, but deserves an explicit human confirmation given the registry file's tier.
- **(non-blocking)** CodeQL's seven path-resolution diagnostics should be named in the founder disposition as reviewed tool-side diagnostics, so the evidence record shows they weren't overlooked.
- **(non-blocking)** Semgrep's test-only `temp-dir` finding persists from CORE-002/003; if you ever want it cleared, `tempfile` would do it — not required.

**Reviewed identity:** bundle manifest 63/63 OK · patch SHA-256 `71f051e24055cb0febd620d84a2703ea43d7277f5af4266feef8d03d0fbb9f1f` · reviewed commit `60febb08ac9c3e207d6f7a3563b6824374c5c93e` · design blobs `a6b883ea…` / `ef5a19e9…`. **This disposition is valid only for that exact patch digest**; any change to the tree invalidates it and requires re-review.

---

Two things worth saying outside the formal disposition. First, the **amount comparison** is the best single piece of engineering in this step — refusing to parse into `u64` and instead doing length-then-lexicographic on validated canonical digits eliminates an entire overflow attack class by construction rather than by bounds-checking. That's the kind of choice that makes a control durable. Second, the **atomic timeout transition** (single `UPDATE` guarded by both `status='requested'` and `deadline < now`, with row-count verification and commit-before-observe) means two concurrent evaluations cannot both transition, and a crash cannot leave a half-transitioned record — the same discipline as the consumption store, correctly applied to a harder shape.
