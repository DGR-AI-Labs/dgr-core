# CORE-005 founder settings checklist

- **Repository:** `DGR-AI-Labs/dgr-core`
- **Target branch:** `main`
- **Control owner:** Founder
- **Agent limitation:** This checklist records settings for the founder to apply. The agent must not apply branch protection.

## Required target state

| Setting | Current | Target |
| --- | --- | --- |
| Require status checks before merging | Enabled | Enabled |
| Required contexts | `Structural / governance check` | `Structural / governance check`; `Rust format / build / test` |
| Require branches to be up to date (`strict`) | Yes | Yes |
| Required approving reviews | 0 | 1 |
| Independent reviewer | Not required | Gaziz Nugmanov is the eligible independent reviewer |
| Enforce for administrators | Yes | Yes |
| Allow force pushes | No | No |
| Allow deletions | No | No |

GitHub's approval-count rule does not designate one person. Gaziz Nugmanov must have repository access and is the founder-designated eligible independent reviewer. A `CODEOWNERS` rule is needed only if automatic review routing or a path-specific required code-owner review is desired; it is not needed to set the approval count to one.

## Apply in GitHub

- [ ] Open **Settings → Branches** for `DGR-AI-Labs/dgr-core` and edit the rule that protects `main`.
- [ ] Keep **Require a pull request before merging** enabled.
- [ ] Change **Required approvals** from **0** to **1**.
- [ ] Confirm Gaziz Nugmanov has sufficient repository access to submit an approving review.
- [ ] Keep **Require status checks to pass before merging** enabled.
- [ ] Keep **Require branches to be up to date before merging** enabled (`strict: true`).
- [ ] Keep `Structural / governance check` required.
- [ ] Add `Rust format / build / test` as the second required context.
- [ ] Keep administrator enforcement enabled.
- [ ] Keep force pushes disabled.
- [ ] Keep branch deletion disabled.
- [ ] Save the protection rule.

Do not add any `Informational SAST / Semgrep (non-blocking)`, `Informational SAST / CodeQL (non-blocking)`, or `Informational SCA / cargo-deny (non-blocking)` context to required checks. Those jobs are informational only; the blocking T0 control remains exact-commit, founder-dispositioned three-engine evidence under FND-7.

## Verify the setting

- [ ] Re-query the protection endpoint and retain the response as settings evidence:

  ```bash
  gh api repos/DGR-AI-Labs/dgr-core/branches/main/protection \
    --jq '{strict: .required_status_checks.strict, contexts: .required_status_checks.contexts, approvals: .required_pull_request_reviews.required_approving_review_count, admins: .enforce_admins.enabled, force_pushes: .allow_force_pushes.enabled, deletions: .allow_deletions.enabled}'
  ```

- [ ] Confirm the response reports `strict: true`, both exact required contexts, `approvals: 1`, `admins: true`, `force_pushes: false`, and `deletions: false`.
- [ ] Open or use a test pull request and confirm merge is blocked while `Rust format / build / test` is absent or failing.
- [ ] Confirm the same test pull request remains blocked until one independent approval is recorded.
- [ ] Record the protection response and test-PR result in the CORE-005 completion evidence before marking the parent Done.

## Residual control that GitHub cannot enforce here

GitHub cannot mechanically enforce “the agent may never merge” while automation authenticates as the founder/administrator. The enforceable control is one required approving review. The stronger separation-of-duties rule remains an attested procedural control backed by the pull-request review, merge identity, branch-protection response, and CORE-005 completion evidence. Do not claim it is mechanically enforced.
