# dgr-core

An experiment in runtime decision governance for AI agents.

## Status

Early work-in-progress. **Experimental.** **Not production-ready.** No guarantees of any kind.

**Phase 1** builds and tries to *prove* inline, fail-closed enforcement as OSS packages for the
OpenClaw runtime. The enforcement core is **T0 — enforcement-critical** and is currently a
**DRAFT pending a human review gate** (human review + cross-model review + ≥3 SAST + adversarial
testing). Until that gate passes, treat the core as unvalidated and do not rely on it.

### Honest scoping

What the Phase 1 adapter does and does not do:

- **Agent-non-bypassable** — an agent driven through OpenClaw cannot reach a governed effectful
  tool without a DGR "allow" decision and the short-lived capability token it issues.
- **Operator-bypassable** — a privileged operator who controls the runtime can disable or skip
  the gate. That is out of scope at this tier (Phase ≥2 work).

This is **developer-grade**. It is deliberately **not** described as "non-bypassable", "secure",
"production", or "compliant".

## Packages

| Package | Path | What it is |
|---------|------|------------|
| [`@dgr/core`](packages/core) | `packages/core` | Framework-agnostic decision engine (schemas, policy, decision point, capability token, fail-closed). |
| [`@dgr/openclaw`](packages/adapter-openclaw) | `packages/adapter-openclaw` | Thin OpenClaw `before_tool_call` adapter. |

## Governance (read before contributing)

- [`.specify/memory/constitution.md`](.specify/memory/constitution.md) — binding source of truth.
- [`specs/0001-enforcement-spec.md`](specs/0001-enforcement-spec.md) — enforcement spec (its
  numeric values are **proposed defaults pending founder confirmation**).
- [`tests/bypass/`](tests/bypass/README.md) — the enforcement proof suite. Green means the suite
  passes; it does **not** by itself mean enforcement is proven (the T0 gate must pass first).
- Consequence tiers (T0–T3): T0 enforcement-critical code is human-led. See the constitution.

## Develop

```sh
pnpm install
pnpm build          # tsc -b (workspace)
pnpm test           # all tests
pnpm test:bypass    # the bypass suite only
```

## License

[Apache-2.0](LICENSE).
