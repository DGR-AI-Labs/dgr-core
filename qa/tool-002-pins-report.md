# TOOL-002 dependency pin report

**Date:** 2026-08-11

**Branch:** `feat/tool-002-pin-crates`

**Base:** GitHub `main` at `a68befb601505410193584622ae036f56aff686a`

## Approved pins

| Dependency | Manifest pin | Resolved version | Features |
|---|---|---|---|
| `ed25519-dalek` | `=2.2.0` | 2.2.0 | crate defaults |
| `rusqlite` | `=0.40.2` | 0.40.2 | `bundled` plus crate defaults |

FND-15 fixes `ed25519-dalek` to the 2.x line. The crates.io index reported
2.2.0 as the latest 2.x release (with 3.0.0 available outside the approved
line). The current stable `rusqlite` release was 0.40.2. Its `bundled` feature
is enabled so SQLite compiles self-contained rather than depending on a system
SQLite installation.

Both manifest requirements use an exact `=` pin. `Cargo.lock` records the full
resolved dependency graph.

## Compile-only validation

```text
$ cargo fmt --manifest-path tests/bypass-rust/Cargo.toml -- --check
PASS
$ cargo build --manifest-path tests/bypass-rust/Cargo.toml
PASS
$ cargo test --manifest-path tests/bypass-rust/Cargo.toml --no-run
PASS
```

The new crates are intentionally unused. No import, signature-verification
call, database connection, schema, insert, consume operation, or other proof
code was added.

## RustSec audit

Tool: `cargo-audit 0.22.2`.

```text
$ cargo audit --file tests/bypass-rust/Cargo.lock
Loaded 1211 security advisories
Scanning tests/bypass-rust/Cargo.lock for vulnerabilities (64 crate dependencies)
PASS (exit 0; no advisories reported)
```

No RustSec vulnerability advisory was reported for the locked dependency
graph at audit time.

## Deliberately red conformance state

```text
$ cargo test --manifest-path tests/bypass-rust/Cargo.toml
EXPECTED RED (exit 101)
```

- Adapter harness: 3 passed.
- Attack harness: 3 passed, 1 failed, 14 ignored.
- ATK-01 is still the sole active red conformance test and stops at
  `FounderImplementationRequired`.
- ATK-02 through ATK-15 remain ignored.
- The zero-effectful-invocation scaffold check passes.

## T0 boundary attestation

- Only `tests/bypass-rust/Cargo.toml`, `tests/bypass-rust/Cargo.lock`, and this
  report change in TOOL-002.
- All five `tests/bypass-rust/src/founder_*.rs` files are byte-unchanged from
  `main`.
- Exactly five founder-unit files and five `unimplemented!()` defaults remain.
- No signature verification, action binding, guard decision, fail-closed
  mapping, SQLite consumption, replay prevention, or other enforcement logic
  was authored.
- ATK-01 was not weakened or ignored.

Using these dependencies inside the five T0 units remains founder-authored work
under the constitution and `T0-AUTHORS.md`.
