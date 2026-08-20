# CORE-004 normative reference contract

The founder-confirmed ATK-06 approval-timeout design lives in the private
`dgr-internal` reference corpus. It is pinned here rather than copied into
dgr-core, so the future VAL-004, T3 conformance, and founder-authored T0 work
consume one immutable source of truth.

| Reference ID | Expected type | Expected status | Expected `last-reviewed` | dgr-internal path | Pinned source commit | Blob SHA-256 |
|---|---|---|---|---|---|---|
| `CORE-004-DESIGN` | `DECI` | `active` | `2026-08-19` | `decisions/CORE-004-DESIGN.md` | `fd0329d44ee174eb5a2171c1ba14d67304a22aa6` | `a6b883eafa471bf797617e1716a65aafb2689ad993cabac42dd2d1aa3db85dab` |
| `CORE-004-DESIGN-ADDENDUM-A` | `DECI` | `active` | `2026-08-19` | `decisions/CORE-004-DESIGN-ADDENDUM-A.md` | `e2649b0387ac7984a84bb8d6b6e16718c57dde8d` | `ef5a19e99f0ad6753c9351f337d376542c4657c291dd00787dfa6e43d86bd514` |

The record is published for founder review through
[dgr-internal pull request #3](https://us-west-2.console.aws.amazon.com/codesuite/codecommit/repositories/dgr-internal/pull-requests/3/details?region=us-west-2).
Its generated reference catalog resolves the immutable source content at the
commit above; generation occurs in a separate follow-up commit.

Addendum A is published for founder review through
[dgr-internal pull request #4](https://us-west-2.console.aws.amazon.com/codesuite/codecommit/repositories/dgr-internal/pull-requests/4/details?region=us-west-2).
It freezes the pre-deadline observation, bound-amount threshold, canonical and
overflow-safe comparison semantics, placement before nonce consumption, and
the ATK-05 reuse/deferral boundary. PR #4 must merge before VAL-004 fixture
authoring begins.

To verify the pointer, resolve
`git show <pinned-commit>:<dgr-internal-path>` for each row, require the
frontmatter `id`, `type`, `status`, and `last-reviewed` values shown above, and
compare the content SHA-256. Any commit, ID, type, status, review marker, path,
or digest mismatch is reference drift and blocks CORE-004 authoring.

This pointer defines no observation variant, port, store, deadline
calculation, decision, or enforcement behavior. The referenced design remains
subject to the T0/T3 ownership map; this file does not authorize an agent to
author a founder-owned surface.
