# Original private-bundle to sanitized R2 path map

The immutable cross-model addendum accurately records the complete private
bundle Claude reviewed. Do not edit that audit record to make its paths look
like this later sanitized package.

| Original private-bundle path | Sanitized R2 location or disposition |
|---|---|
| `dgr-core/baseline/` | `dgr-core/baseline-critical/`, eight critical pre-images only |
| `dgr-core/reviewed/` | Relevant selected bytes under `dgr-core/review-source/` |
| `dgr-core/executable/` | Relevant selected bytes under `dgr-core/review-source/` |
| `dgr-core/evidence-head/` | Relevant selected bytes under `dgr-core/review-source/` |
| `dgr-internal/` | Deliberately excluded; consult the canonical internal repository directly |

The sanitized selections are not complete snapshot replacements and cannot be
used to reconstruct the original commits' complete Git trees. Whole-tree
identity is verified from the public repository object database using
`metadata/commit-tree-identities.txt`. Bundle-local byte identity is verified
with the selected inventory and critical/evidence sidecars.
