# CORE-004 T3 green conformance report

**Date:** 2026-08-21

**Status:** Local conformance green — T0 review gates pending

**Baseline:** `7324cbb33be59595657a2df13c300aa388208d77`

**Reviewed implementation/test commit:** `60febb08ac9c3e207d6f7a3563b6824374c5c93e`

**Reviewed tree:** `71ed21dcbd2f940c55b0e400f1f2071e628b074b`

**Binary full-index patch SHA-256:**
`71f051e24055cb0febd620d84a2703ea43d7277f5af4266feef8d03d0fbb9f1f`

## Exact patch digest command

```text
git diff --binary --full-index \
  7324cbb33be59595657a2df13c300aa388208d77..60febb08ac9c3e207d6f7a3563b6824374c5c93e \
  | sha256sum
```

## Activated conformance

The previously reviewed RED contract is now wired to the founder-authored
surfaces without changing ATK-06's registry outcome:

1. amount `1000001` records pending state and returns `Escalated` with no
   authorization or effectful invocation;
2. amount `100000` retains the existing consume-and-allow behavior and does not
   consult approval state;
3. re-presentation returns the original review-request ID and deadline;
4. the escalation path leaves the capability nonce unconsumed;
5. token-free evaluation at `deadline - 1` and `deadline` returns the same
   escalation; and
6. token-free evaluation at `deadline + 1` returns the registry-derived
   `EscalateThenDenyOnTimeout` denial after the SQLite transition.

The generic ATK-06 registry test is active and points behavioral proof to the
dedicated two-surface conformance target. No ATK-06 test remains ignored.

## Local validation

Run from `tests/bypass-rust` with the reviewed commit checked out:

```text
cargo fmt --check
cargo check --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
```

Observed results:

- formatting: PASS;
- all-target check: PASS;
- Clippy with warnings denied: PASS;
- full Rust suite: PASS — 52 passed, 0 failed, 5 intentionally ignored;
- dedicated CORE-004 target: PASS — 5 passed, 0 failed, 0 ignored; and
- diff whitespace validation: PASS.

The five remaining ignored attacks are outside CORE-004's completed isolated
ATK-06 surface: ATK-04, ATK-05, ATK-12, ATK-14, and external-IAM ATK-15.

## Claim boundary and outstanding gates

This evidence proves only the frozen single-guard, local-store, modeled-clock
isolation contract. It does not prove real human delivery or waiting,
cross-instance state, deployed runtime non-bypassability, or the later
approve-to-allow path.

This report is not founder approval. Merge remains blocked until the exact
commit receives independent human review, cross-model review, three-engine
SAST/SCA evidence with founder dispositions, and final founder sign-off.
