# CORE-002 Step 5 Claude QA bundle manifest

**Bundle date:** 2026-08-17

## Canonical source points

- dgr-core evidence commit:
  `61e08b8609b9ba23ab538e30a2ac744d7a5add6f`
- dgr-core final scanned commit:
  `0a54d4995d1b9d98ab8a3ec61861fe2fe7ae29c3`
- dgr-core founder-authored T0 commit:
  `0727e327631b475990ef8d9b7ef3b2c3554050a8`
- dgr-internal FND-7 decision source commit:
  `055906424049a610839098d40e0c729b62fb1fae`
- dgr-internal generated reference commit:
  `505f3e470109bd10581b79980dc6a3ed2bb0b546`
- dgr-backlog reconciled evidence-state commit:
  `3feef3ba9e37a5b01e11799f9ebb6c63f0cec08c`

## Bundle layout

- `dgr-core/` — constitution, authorship boundary, policy, manifests, relevant
  specs, T0/support Rust, adversarial tests, readiness records, and final plus
  historical raw analyzer evidence.
- `dgr-internal/` — DECI-0011, program status, FND-7 verification report, and
  reference index.
- `dgr-backlog/` — canonical catalog source, generated items, verifier, reports,
  and pinned reference lock showing FND-7 Done and Step 5 In Review.
- `BUNDLE-SHA256SUMS.txt` — digest for every bundled file except itself.

The bundle deliberately includes the protected review checklist in its pending
state. It does not contain an independent human review or founder final
sign-off and therefore cannot close the T0 gate by itself.
