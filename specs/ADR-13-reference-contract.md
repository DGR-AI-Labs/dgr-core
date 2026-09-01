# ADR-13 normative reference contract

The enforcement-core extraction, prerequisite PROD-000 decoupling, and RUNTIME-003/004 trigger
revision live in the private `dgr-internal` reference corpus. This repository pins ADR-13 and its
two active amendments rather than copying their bodies, so the founder decisions remain one
immutable source of truth.

| Reference ID | Expected type | Expected status | Expected `last-reviewed` | dgr-internal path | Pinned source commit | Blob SHA-256 |
|---|---|---|---|---|---|---|
| `ADR-13` | `ADR` | `active` | `2026-08-31` | `specs/adr/ADR-13-productize-enforcement-core.md` | `891607c20ba65c31b024c59f29f09744f8a62b26` | `ef97fc6eb1c3f353d3b54d195c6ba8a118e5bb8d8ca34aa365ecb2a08e023c7c` |
| `ADR-13-AMENDMENT-A` | `ADR` | `active` | `2026-08-28` | `specs/adr/ADR-13-AMENDMENT-A-gate-1-resolutions.md` | `891607c20ba65c31b024c59f29f09744f8a62b26` | `ccb4268ceef292a4c167883d9299ab581c1a898ed344bb0d0623a5e06ad03ec7` |
| `ADR-13-AMENDMENT-B` | `ADR` | `active` | `2026-08-31` | `specs/adr/ADR-13-AMENDMENT-B-supervised-agent-t0-authorship.md` | `891607c20ba65c31b024c59f29f09744f8a62b26` | `d27a10c83f27e63ad0fe4ffeb9dc19b6bd2a4bd2461171b32b64ca30816d13e6` |

To verify the pointer, resolve the pinned file with:

```text
git show 891607c20ba65c31b024c59f29f09744f8a62b26:specs/adr/ADR-13-productize-enforcement-core.md
git show 891607c20ba65c31b024c59f29f09744f8a62b26:specs/adr/ADR-13-AMENDMENT-A-gate-1-resolutions.md
git show 891607c20ba65c31b024c59f29f09744f8a62b26:specs/adr/ADR-13-AMENDMENT-B-supervised-agent-t0-authorship.md
```

Require each row's exact `id`, `type: ADR`, `status: active`, and row-specific `last-reviewed`, then
compare each complete file SHA-256 with the table. A missing commit, path, field, or digest match is
reference drift and blocks PROD-000 and extraction.

Amendment A resolves the four former Gate-1 items and fixes the PROD-000 design and proof
obligations. Amendment B supersedes only Amendment A R5.3 authorship option (c): an agent may author
the exact bounded PROD-000 T0 diff under founder design authority, line-by-line review, finding
disposition, and founder-only merge. Existing founder source retains its historical provenance;
agent relocation or transformation does not become founder authorship.

Implementation remains blocked until this pointer and the five supervised-agent evidence templates
are merged, the canonical backlog contract is updated and merged, and any pre-existing uncommitted
founder draft is explicitly checkpointed or discarded. Neither amendment authorizes PROD-001,
runtime work, unrelated T0 changes, or claim expansion. PROD-001 remains blocked until PROD-000
passes its complete exact-commit T0 gate and is founder-merged.
