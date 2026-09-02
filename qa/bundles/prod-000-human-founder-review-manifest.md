# PROD-000 independent-human and founder review bundle manifest

## Identity

- Bundle date: 2026-09-02.
- Repository/PR: `DGR-AI-Labs/dgr-core#90`.
- Baseline: `e9c8f585809c15d2464b3d45bc2ce26d716c8673`.
- Exact executable review input: `587585cf476431f078efe587c5dbcc052389cdad`.
- Exact executable tree: `89e5de51d23ead98d24bbbe1b4cd57db343b2dc4`.
- Evidence/instruction head: `e4fdda5969493a83e2b1b0bdffff26a837d999d1`.
- Evidence/instruction tree: `75ca0f933883457f121accc17e46358789e6cd0c`.
- Baseline-to-executable patch SHA-256:
  `c08919d86a1f060cce9a05b3143140a5f011b9349f243475dad4f4ec1b40cf99`.
- Cross-model addendum SHA-256:
  `534ce4164067aef339b9f35a176de1b39e6f1573834e52cdc3d533fad7e634db`.

## Contents

- `dgr-core/baseline/` — complete baseline snapshot.
- `dgr-core/executable/` — complete executable review snapshot.
- `dgr-core/evidence-head/` — complete evidence/instruction snapshot, including both Claude records,
  N13–N15 follow-up, raw canonical scans, and unsigned human/founder forms.
- `dgr-internal/` — ADR-13 and active Amendments A/B.
- `metadata/baseline-to-executable.diff` — complete binary/full-index review patch.
- `metadata/r5-1-timeout.diff` and `metadata/r5-1-timeout-semantic.diff` — raw and
  EOL-insensitive founder-file evidence.
- `metadata/` — lineage, inventories, drift, authority hashes, scanner hashes, and critical hashes.
- `review/` — remaining-gate instructions plus independent-human and founder input forms.
- `MANIFEST.sha256` — SHA-256 over every other bundled file.

## Gate state

- Cross-model: satisfied with non-blocking findings; original and addendum both retained.
- N13: reduced semantic diff stored.
- N14: bounded explicitly; test-name/active-state guard is not assertion-body proof.
- N15: ledger line corrected to 10.
- Independent-human: pending.
- Founder dispositions and line-by-line review: pending.
- Final-head GitHub approval and founder-only merge: pending.
- PROD-001: unauthorized.
