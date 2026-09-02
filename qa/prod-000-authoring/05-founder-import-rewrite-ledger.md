# PROD-000 template 5 — supervised-agent import-rewrite ledger

**Purpose:** Prove agent-authored module-path edits inside founder-owned files are import-only and
do not conceal an enforcement change.

## Scope rule

For seven files below, the permitted change is limited to the import/module path needed to consume
the new founder boundary module. `founder_approval_timeout.rs` also contains the separately reviewed
R5.1 control-flow change; therefore this ledger must not certify that file's complete diff as
import-only.

## Per-file ledger

| Founder-owned file | Old path line(s) | New path line(s) | Before SHA-256 | After SHA-256 | Non-import diff? |
|---|---|---|---|---|---|
| `founder_approval_store.rs` | 6 | 6 | `35277b1e836c505f7a8a6c7a85752428ba57dd01e8b1572aa0d9cc35b8c212a0` | `92b4bc4716725569e9dbf3834d9fd6d72128918f0937a31a9e7bfd5282dde7ec` | no |
| `founder_approval_timeout.rs` | 8 | 9 | `989ae1df102c1a78638b28354e1cbcbd37e7cc3565292703840b2e7222f2c9b7` | `14cb21fbb11ac0a0216ae31150f292e9a9e1995ad332acf75b661116c8b6d0c2` | yes — R5.1 plus file-wide CRLF→LF normalization |
| `founder_authored_guard.rs` | 7–9 | 7–9 | `f2f1351d387c9a4fca1d76f9d171b7ddf47322e9e88bb8880c4788726ee866cf` | `cf6f32d5d37ad990dfa04cf6ef18c86661c61e0d6236be48d287261e3ff808e1` | no |
| `founder_consumption_store.rs` | 6 | 6 | `ebf568985ecb17e460268d2b15a2ac33ee908c00fb354f3527a9ae8d64d5c1f9` | `813ca93068f9d81ae84d79b7b52ecd4aa15fd7e894b7ca599b30472375ac157c` | no |
| `founder_fail_closed.rs` | 4 | 4 | `5cbd452b9db4a3d8b1799cf0605e3391898e5960e8ed44b7d45800f07b8e08d1` | `fba6071de88417e1c551fc1793f7d6a77a6547cce438c34648d88dfd9bc8c3fc` | no |
| `founder_s2_approval_store.rs` | 6 | 9 | `6f027615304f87b78f56f7853fad7f3e5382cabb2c9796d3746b9016a8ca1c2c` | `1e524445376f64158974ccfddfa9676199c1930659391353e0a51f0af800221a` | no |
| `founder_s2_consumption_store.rs` | 6 | 6 | `c27936ce033eb4d340689739888a0ecffdf7777d8168ddcea62d51b147a105e5` | `3da870fc853939f8fcac4a657fdcaf41f85e12680e76d6e369d754895d5f923d` | no |
| `founder_token_verification.rs` | 13 | 13 | `b1532ea33ff6c2a9f65ba0ecc4d613d7b4edb54740e23195a800b15da8eb0384` | `ba86403c02e6d3714a0cb9bc2abdbe04cfd6ad83c619a789a8251147a51e73e5` | no |

## Enforcement-body exclusion check

For every file except `founder_approval_timeout.rs`, confirm no changed line touches:

- [x] condition or comparison
- [x] arithmetic
- [x] constant value
- [x] match arm
- [x] SQL statement
- [x] store operation
- [x] returned decision
- [x] denial signal
- [x] visibility or ownership classification

For `founder_approval_timeout.rs`, classify each non-import changed line under the separately
reviewed R5.1 disposition. The replacement review commit also normalizes the complete file from
CRLF/mixed line endings to consistent LF; `--ignore-space-at-eol` isolates the same R5.1 semantic
diff, and no other semantic change is permitted.

## Dependency-direction check

- [x] No founder-owned file imports the T3 `before_tool_call` module after the rewrite.
- [x] No founder-owned file imports fixtures, observations, probes, or the attack registry.
- [x] All imported type names remain unchanged unless Amendment A explicitly requires otherwise.
- [x] The new module path resolves only to the Amendment-B T0 boundary module.

## Diff evidence

| Field | Entry |
|---|---|
| Baseline commit | `e9c8f585809c15d2464b3d45bc2ce26d716c8673` |
| Agent implementation commit | `40b713039a5612831df415cdd785271a7342be74` |
| Cross-model remediation source commit | `b19f33ae16698a81b993e6cc5a751360b6109577` |
| Non-droppable T3 assertion guard commit | `587585cf476431f078efe587c5dbcc052389cdad` |
| Eight-consumer diff artifact/path | `qa/prod-000-review-evidence/eight-consumer.diff` with an explicit eight-file path list; no wildcard |
| R5.1 diff artifact/path | `qa/prod-000-review-evidence/r5-1-timeout.diff` plus template 2/preparation |
| Complete implementation diff artifact/path | `qa/prod-000-review-evidence/full-implementation.diff` |
| Unexpected changed line count | 0 |

## Agent attestation

> I authored and classified every changed line in the eight founder-owned consumers. Except for the
> separately dispositioned R5.1 timeout change, the changes are restricted to import/module paths
> and do not alter an enforcement expression, constant, SQL statement, store operation, returned
> decision, or denial signal.

Agent product/model/session: OpenAI Codex; model and session identifiers not exposed

UTC timestamp: `2026-09-01T21:12:17Z`

## Founder disposition

Exact reviewed commit: **PENDING final PR head**

Decision and finding disposition: **PENDING**

Founder signature/name: **PENDING**

UTC timestamp: **PENDING**
