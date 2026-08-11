# CORE-002 normative reference contracts

The normative authorization-schema and token-wire-format contracts live in the
private `dgr-internal` reference corpus. They are linked here, not copied, so
the founder-authored guard and the VAL-002 fixture work have one source of
truth:

| Reference ID | Expected status | Expected `last-reviewed` | dgr-internal path | Pinned source commit | Blob SHA-256 |
|---|---|---|---|---|---|
| `ARCH-005-typed-authorization-schema` | `active` | `2026-08-10` | `specs/ARCH-005-typed-authorization-schema.md` | `9dd14d757ed108037bdeda8e5bb14da22d90bbba` | `178edbfc1ff0b597c412c782000c74c2db46d2f9bbee6d8ebb38a86d7afa3f98` |
| `ARCH-006-token-wire-format` | `active` | `2026-08-10` | `specs/ARCH-006-token-wire-format.md` | `9dd14d757ed108037bdeda8e5bb14da22d90bbba` | `28fbc85064765f4374f0fbd1167d282112baac768fcef1b1d0f519415614ff6a` |

Both specifications were founder-merged and cataloged by
[dgr-internal pull request #1](https://us-west-2.console.aws.amazon.com/codesuite/codecommit/repositories/dgr-internal/pull-requests/1/details?region=us-west-2).
The merged dgr-internal tip for that placement is
`a563f388bca240ab94ddbf491582e58ac96f988f`; the catalog intentionally resolves
the immutable source content at `9dd14d757ed108037bdeda8e5bb14da22d90bbba`.

To verify the pointer rather than trusting this prose, resolve each path with
`git show <pinned-commit>:<path>`, require the frontmatter `id`, `status`, and
`last-reviewed` values shown above, and compare SHA-256 with the recorded blob
digest. Any commit, ID, status, review marker, path, or digest mismatch is
reference drift and blocks the consumer.

This file is a documentation pointer only. It defines no serialization,
verification, decision, fail-closed, or consumption behavior and authorizes no
agent to edit a founder-only T0 unit.
