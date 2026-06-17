// Structural / governance check — Phase 1.
//
// The CI job expected to PASS. Verifies the governance spine + workspace layout
// are present, the README carries its honest-scoping disclaimers, and every
// T0 enforcement-critical file carries the mandated T0 header.
import { readFileSync, existsSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const problems = [];

const requiredPaths = [
  ".specify/memory/constitution.md",
  "specs/0001-enforcement-spec.md",
  "pnpm-workspace.yaml",
  "package.json",
  "tsconfig.base.json",
  "packages/core/package.json",
  "packages/core/src/index.ts",
  "packages/adapter-openclaw/package.json",
  "packages/adapter-openclaw/src/index.ts",
  "tests/bypass/README.md",
  "tests/bypass/01-no-token.test.ts",
  "tests/bypass/02-expired-replayed-token.test.ts",
  "tests/bypass/03-missing-justification.test.ts",
  "tests/bypass/04-ambiguous-evidence.test.ts",
  "tests/bypass/05-gate-throws.test.ts",
  ".github/workflows/ci.yml",
  ".github/workflows/release.yml",
  ".github/ISSUE_TEMPLATE/bug.yml",
  "CLAUDE.md",
  "AGENTS.md",
  "README.md",
  "LICENSE",
];

for (const rel of requiredPaths) {
  if (!existsSync(join(root, rel))) problems.push(`missing required file: ${rel}`);
}

// README must carry the honest-scoping disclaimers (matched case-insensitively).
const readmePath = join(root, "README.md");
if (existsSync(readmePath)) {
  const readme = readFileSync(readmePath, "utf8").toLowerCase();
  for (const phrase of ["experimental", "not production-ready", "no guarantees"]) {
    if (!readme.includes(phrase)) {
      problems.push(`README.md must contain the disclaimer phrase: "${phrase}"`);
    }
  }
}

// Every T0 enforcement-critical file must carry the mandated header.
const t0Files = [
  "packages/core/src/token/capability-token.ts",
  "packages/core/src/decision/decision-point.ts",
  "packages/core/src/decision/fail-closed.ts",
];
const T0_HEADER = "T0 — enforcement-critical";
for (const rel of t0Files) {
  const p = join(root, rel);
  if (!existsSync(p)) {
    problems.push(`missing T0 file: ${rel}`);
    continue;
  }
  if (!readFileSync(p, "utf8").includes(T0_HEADER)) {
    problems.push(`T0 file missing required "${T0_HEADER}" header: ${rel}`);
  }
}

if (problems.length > 0) {
  console.error("Structural check FAILED:");
  for (const p of problems) console.error(`  - ${p}`);
  process.exit(1);
}

console.log(
  `Structural check passed: ${requiredPaths.length} spine files present, T0 headers verified.`,
);
