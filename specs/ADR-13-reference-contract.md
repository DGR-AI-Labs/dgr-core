# ADR-13 normative reference contract

The enforcement-core extraction, prerequisite PROD-000 decoupling, and RUNTIME-003/004 trigger
revision live in the private `dgr-internal` reference corpus. This repository pins ADR-13 and its
active Gate-1 amendment rather than copying either body, so the founder decisions remain one
immutable source of truth.

| Reference ID | Expected type | Expected status | Expected `last-reviewed` | dgr-internal path | Pinned source commit | Blob SHA-256 |
|---|---|---|---|---|---|---|
| `ADR-13` | `ADR` | `active` | `2026-08-28` | `specs/adr/ADR-13-productize-enforcement-core.md` | `104dbe651a869f198f2c76a58d7b2682bb82fbd6` | `0b0ff77b111c6a336e62e5c51c1c901ea1d811075c826e40f90889416dbbfb6e` |
| `ADR-13-AMENDMENT-A` | `ADR` | `active` | `2026-08-28` | `specs/adr/ADR-13-AMENDMENT-A-gate-1-resolutions.md` | `104dbe651a869f198f2c76a58d7b2682bb82fbd6` | `ccb4268ceef292a4c167883d9299ab581c1a898ed344bb0d0623a5e06ad03ec7` |

To verify the pointer, resolve the pinned file with:

```text
git show 104dbe651a869f198f2c76a58d7b2682bb82fbd6:specs/adr/ADR-13-productize-enforcement-core.md
git show 104dbe651a869f198f2c76a58d7b2682bb82fbd6:specs/adr/ADR-13-AMENDMENT-A-gate-1-resolutions.md
```

Require each row's exact `id`, `type: ADR`, `status: active`, and `last-reviewed: 2026-08-28`, then
compare each complete file SHA-256 with the table. A missing commit, path, field, or digest match is
reference drift and blocks PROD-000 and extraction.

The founder resolved the four former Gate-1 items and selected authorship option (c): the founder
authors the complete new T0 boundary module and applies import-only changes inside founder-owned
files; agent assistance begins only at the T3 facade, conversion, tests, and documentation after the
T0 surface exists. Active ADR-13 authorizes that ordered PROD-000 process, not agent-authored
enforcement, extraction, runtime work, or claim expansion. PROD-001 remains blocked until PROD-000
passes its complete exact-commit T0 gate and is merged.
