# Changesets

This folder holds [Changesets](https://github.com/changesets/changesets) for versioning the
DGR packages (`@dgr/core`, `@dgr/openclaw`).

Add a changeset with `pnpm changeset`. Releases publish to npm only via the tagged release
workflow, **gated on the bypass suite being green** — never from a feature PR, and never while
the T0 enforcement core is unvalidated.
