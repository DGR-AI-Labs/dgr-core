# ADR-13 normative reference contract

The proposed enforcement-core extraction and RUNTIME-003/004 trigger revision live in the private
`dgr-internal` reference corpus. This repository pins that record rather than copying it, so its
status, founder decisions, and future activation remain visible as one immutable source of truth.

| Reference ID | Expected type | Expected status | Expected `last-reviewed` | dgr-internal path | Pinned source commit | Blob SHA-256 |
|---|---|---|---|---|---|---|
| `ADR-13` | `ADR` | `draft` | `2026-08-27` | `specs/adr/ADR-13-productize-enforcement-core.md` | `66b5ab9f315ec10a7bd5a721250e91c8f6865891` | `e058046131fa46d18a38ca590460a365015e2d2985ee7dac79f6bceb005273f6` |

To verify the pointer, resolve the pinned file with:

```text
git show 66b5ab9f315ec10a7bd5a721250e91c8f6865891:specs/adr/ADR-13-productize-enforcement-core.md
```

Require front matter `id: ADR-13`, `type: ADR`, `status: draft`, and
`last-reviewed: 2026-08-27`, then compare the complete file SHA-256 with the table. A missing commit,
path, field, or digest match is reference drift and blocks extraction.

The expected status is deliberately `draft`. ADR-13 still requires founder resolution of the crate
name/publication posture, versioning policy, workspace-versus-path-dependency choice, and revised
runtime-trigger wording. This pointer authorizes no extraction, T0 edit, runtime work, enforcement
change, test change, or claim expansion. After the founder activates ADR-13, this pointer must be
repinned to the active source commit before extraction begins.
