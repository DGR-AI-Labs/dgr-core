# dgr-core

An experiment in runtime decision governance for AI agents.

## Status

Early work-in-progress. **Experimental.** **Not production-ready.** No guarantees of any kind.

This repository is in **Phase 1 — enforcement proof**. Its root Rust package is the extracted,
Git-dependency-only `dgr-core` library, and `tests/bypass-rust` is its isolated conformance harness.
The active suite is expected green in CI, but this remains experimental proof code—not a deployed
runtime gate—and nothing here should be relied upon as production enforcement.

## What's here

- [`.specify/memory/constitution.md`](.specify/memory/constitution.md) — the binding governance
  document (source of truth).
- [`specs/0001-enforcement-spec.md`](specs/0001-enforcement-spec.md) — a draft specification.
- [`src/lib.rs`](src/lib.rs) — the extracted Rust enforcement library; Git dependency only and not
  published to crates.io.
- [`tests/bypass-rust/`](tests/bypass-rust/T0-BOUNDARY.md) — the retained canonical isolated Rust
  conformance harness consuming the root library.
- [`tests/bypass/`](tests/bypass/README.md) — the retained legacy Node scaffold; it is not the
  canonical CI gate.
- [`CLAUDE.md`](CLAUDE.md) / [`AGENTS.md`](AGENTS.md) — short guidance for agents working here.

## License

[Apache-2.0](LICENSE).

## Contributing and security

Public issues are limited to reproducible public-code defects, build or compatibility regressions, and public documentation corrections. See [CONTRIBUTING.md](CONTRIBUTING.md) before submitting an issue or pull request.

Do not disclose suspected vulnerabilities or bypasses publicly. Follow [SECURITY.md](SECURITY.md) and use the repository's Security tab to report them privately.

---

This project is not announced anywhere — **please do not share.**
