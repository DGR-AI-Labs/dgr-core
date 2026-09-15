# RUNTIME-002 known analyzer dispositions

**Status:** carried-forward baseline input — not a substitute for scanning or founder review

This ledger prevents a previously reviewed result from being misreported as new or silently
treated as resolved. Every RUNTIME-002 analyzer run must report the result if it recurs, bind it to
the reviewed head, and either carry this disposition forward explicitly or reopen it when the
source, reachability, threat context, or risk changes.

| Analyzer result | Existing founder disposition | Authority and rationale | RUNTIME-002 handling |
|---|---|---|---|
| Semgrep `rust.lang.security.temp-dir.temp-dir` at `tests/bypass-rust/tests/consumption_store.rs:19` | `ACCEPT` — test-fixture-scoped, non-blocking | `qa/prod-000-founder-review.md` §6 records that the timestamp/PID-named path is unreachable as production authorization logic, while explicitly retaining local collision, symlink, race, and cross-user residual risk. `qa/prod-001-founder-review.md` §5 carries the same result forward as unchanged T3 test source. | Report it if present; cite both records; do not call the scan clean or treat unchanged location as the risk disposition. Reopen if the file becomes production-reachable or the source/threat context changes. |

This ledger makes no disposition for a future analyzer result, diagnostic, skipped rule, extraction
gap, or coverage gap. Every such item remains subject to the exact-final-head founder disposition
required by the active RUNTIME-002 authority.
