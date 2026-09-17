# ADR-14 Amendment B reference contract

**Status:** active private authority reference — signer implementation not authorized.

This public-safe summary pins the proposed Constitution 4.0.0 offline simulation issuer exception
to exact private authority without copying private ADR/review bodies or custody information.
ADR-14 Amendment A remains unchanged; ADR-13 Amendment B remains PROD-000-only.

## Resolving private identities

Authority ID: ADR-14-AMENDMENT-B. Repository: dgr-internal.

```text
Active authority
Commit: 4cbd92727bee9cf233374c02958fe983253eeac7
Path: specs/adr/ADR-14-AMENDMENT-B-supervised-local-issuer-authorship.md
Git blob: d41ba50d09c78fb80f543572d27d7e475488e243
SHA-256: ee3cff7effe20f3662de68236f10f1f0f3bc2f13b8b1cfa84113d5f68befacd5

Canonical draft adoption
Commit: 2e86ea4d72ca3a9070e7a8cb9b0996f5d665002f
Path: specs/adr/ADR-14-AMENDMENT-B-supervised-local-issuer-authorship.md
Git blob: 56324b7f69055b97157bf9deb735f4fc87162d66
SHA-256: d840dddd046d9a94ed0af03e2790cbb79c5d6a5b05d381644b21c68a49b14ed6

Founder reported review/adoption record
Commit: b71969fad6fcc7cd82b319bc9417ed4188790763
Path: qa/runtime-002-planning/local-issuer-activation/FOUNDER-REVIEW-RECORD.md
Git blob: ef41da30862718becfd1112430297e825c64b633
SHA-256: f6a45dc6f9531e9c721de03b287939f42132daefd8b46cbba373b43990dcacb8

Mechanical activation verification
Commit: 4cbd92727bee9cf233374c02958fe983253eeac7
Path: qa/runtime-002-planning/local-issuer-activation/mechanical-verification.json
Git blob: 826b103e2054cf92c44327d2cd6800d983dd7bc1
SHA-256: 6249f03c07500fdbaa0f911381eb1ada8e8926d8e7cf4bc550b5991f1c249a5d
```

Resolve every identity against canonical private Git history. A moving branch, filename or digest
alone is insufficient. The private amendment controls if this summary conflicts with it. The
review/adoption record explicitly distinguishes reported review and verified merge evidence from
an unavailable detailed draft-review transcript or exact review time. The mechanical verification
was generated on the activation candidate; it does not itself assert a founder merge. The
activation commit above was subsequently verified as founder-merged. Do not turn either record
into an invented signed review or runtime proof.

## Bounded responsibilities

Only a founder-approved, exact-manifest, operator-invoked offline simulation issuer may become
eligible for agent authorship: strict request/profile acceptance, committed-field preview and
operator confirmation, approved encrypted-key unlock/custody, approved time/nonce/key selection,
unchanged capability signature/wire encoding and strictly necessary enumerated wiring/evidence.
All consequential regions remain T0. No path, symbol, dependency, generated artifact or policy
choice is authorized by this summary.

No daemon, network listener, agent-callable command, container signing secret, live runtime callback,
second runtime decision service, arbitrary-preimage signer, batch/noninteractive approval or
production issuer is permitted. Rust remains the runtime decision authority. Preserve the existing
token/commitment, nonce-consumption, approval and failure semantics; no weakened tests, operational
test-key fallback, extra scope or policy exceptions may be inferred. Key-management functionality
is not implicitly included and must be expressly scoped where applicable.

## Ordered prerequisites and hard stops

1. The exact canonical amendment, review/adoption record and separate founder-merged private
   activation resolve as above.
2. Constitution 4.0.0, this pointer and ownership/boundary guidance receive independent human
   review and founder merge. Private authority does not override an unamended public constitution.
3. The canonical backlog explicitly records bounded signer activation without activating deferred
   runtime/deployment items.
4. The founder approves one exact signer manifest freezing repository/base/branch, paths/symbols,
   provenance, request/profile/commitment and confirmation rules, encryption/KDF/unlock/secret
   handling, key/time/nonce/restart policies, output/cancellation/uncertainty, custody and recovery,
   dependencies/features, build, tests, scanners, reviewers and evidence. Unresolved decisions
   or unlisted consequential changes prohibit dependent authoring.
5. Author only that signer scope on its dedicated branch, separate from runtime/adapter work,
   after the approved runtime commitment contract is available. Complete the final-head gate below.
6. Founder approval/merge of implementation does not authorize execution or provisioning. Those
   require separate applicable authorization and actual backup/custody readiness.

This PR completes no future manifest, signer activation, custody or backup requirement. Existing
runtime authorization and signer authorization are separate; neither silently supplies the other.

## Provenance and complete final-head gate

Record unchanged founder source, agent transformations with retained source provenance, existing
exception-governed code, new agent-authored signer T0 and passive T3 support truthfully by region.
Review, supervision or merge does not change authorship. An author may not independently review,
approve or merge their own change.

Require actual adversarial tests of request/profile/key/commitment substitution, bounds and malformed
inputs, confirmation mutation/cancellation, custody/entropy/clock/nonce failures and output/uncertain
retry/recovery behavior as applicable. Preserve all existing attack expectations and required checks.
Retain Semgrep, CodeQL and cargo-deny plus applicable language/dependency coverage on the final head,
and every finding, note, warning, error, skipped target and extraction/coverage gap with founder
disposition. Include nonauthor cross-model review, independent-human review, founder line-by-line
semantic/provenance review, all required checks passing, founder GitHub approval bound to the exact
final head and founder-only merge. Changed bytes invalidate affected evidence/reviews.

No implementation, key provisioning, token issuance, runtime integration, deployment, operator
resistance or non-bypassability claim follows from this pointer. Public deterministic fixtures
are not operational credentials. Private evidence and signing material must not be copied here.
