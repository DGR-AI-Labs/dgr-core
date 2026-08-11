# CORE-002 normative reference contracts

The normative authorization-schema and token-wire-format contracts live in the
private `dgr-internal` reference corpus. They are linked here, not copied, so
the founder-authored guard and the VAL-002 fixture work have one source of
truth:

| Reference ID | dgr-internal path | Pinned source commit |
|---|---|---|
| `ARCH-005-typed-authorization-schema` | `specs/ARCH-005-typed-authorization-schema.md` | `9dd14d757ed108037bdeda8e5bb14da22d90bbba` |
| `ARCH-006-token-wire-format` | `specs/ARCH-006-token-wire-format.md` | `9dd14d757ed108037bdeda8e5bb14da22d90bbba` |

Placement and cataloging are under founder review in
[dgr-internal pull request #1](https://us-west-2.console.aws.amazon.com/codesuite/codecommit/repositories/dgr-internal/pull-requests/1/details?region=us-west-2).
Until that PR is merged to `dgr-internal/main`, downstream work must treat the
spec-placement prerequisite as open.

This file is a documentation pointer only. It defines no serialization,
verification, decision, fail-closed, or consumption behavior and authorizes no
agent to edit a founder-only T0 unit.
